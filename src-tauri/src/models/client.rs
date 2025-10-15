use std::net::ToSocketAddrs;
use std::sync::Arc;

use anyhow::Result;
use thrussh::client;
use thrussh::{ChannelMsg, Disconnect};
use thrussh_keys::key;
use thrussh_keys::load_secret_key;

pub struct Client {}

impl client::Handler for Client {
    type Error = thrussh::Error;
    type FutureUnit = futures::future::Ready<Result<(Self, client::Session), Self::Error>>;
    type FutureBool = futures::future::Ready<Result<(Self, bool), Self::Error>>;

    fn finished_bool(self, b: bool) -> Self::FutureBool {
        futures::future::ready(Ok((self, b)))
    }

    fn finished(self, session: client::Session) -> Self::FutureUnit {
        futures::future::ready(Ok((self, session)))
    }

    fn check_server_key(self, _server_public_key: &key::PublicKey) -> Self::FutureBool {
        // In a production environment, you should verify the server's key
        self.finished_bool(true)
    }
}

pub struct Session {
    session: client::Handle<Client>,
}

impl Session {
    pub async fn connect<A: ToSocketAddrs>(user: String, addrs: A) -> Result<Self> {
        let config = Arc::new(client::Config::default());
        let client_handler = Client {};

        let mut session = client::connect(config, addrs, client_handler).await?;

        let key_pair =
            match load_secret_key("/home/daniel/Workspace/tsumari/src-tauri/privatekey", None) {
                Ok(key) => key,
                Err(e) => {
                    println!("Error loading private key: {}", e);
                    return Err(e.into());
                }
            };

        session
            .authenticate_publickey(user, Arc::new(key_pair))
            .await?;

        Ok(Self { session })
    }

    pub async fn channel_open_session(&mut self) -> Result<client::Channel> {
        // Request an interactive shell once
        let mut channel = self.session.channel_open_session().await?;
        channel.request_shell(true).await?;

        //Send a test command and wait for output to see if we are ready to process commands
        channel.data(&b"echo shell ready\n"[..]).await?;
        let tries = 10;
        let mut i = 0;
        loop {
            i += 1;
            if i == tries {
                break;
            }
            match channel.wait().await {
                Some(ChannelMsg::Data { data }) => {
                    let output_data = String::from_utf8_lossy(&data);
                    if output_data.contains("shell ready") {
                        break;
                    }
                }
                _ => {}
            }
        }

        Ok(channel)
    }

    pub async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}

pub struct ShellChannel {
    channel: client::Channel,
}

impl ShellChannel {
    pub fn new(channel: client::Channel) -> Self {
        Self { channel }
    }

    pub async fn send_command(&mut self, command: &str) -> Result<String> {
        //Add a newline to the command
        self.channel
            .data(format!("{}\n", command).as_bytes())
            .await?;

        let mut output = String::new();

        loop {
            match self.channel.wait().await {
                Some(ChannelMsg::Data { data }) => {
                    let output_data = String::from_utf8_lossy(&data);
                    output.push_str(&output_data);
                    break;
                }
                Some(ChannelMsg::ExitStatus { exit_status }) => {
                    println!("Command exited with status: {}", exit_status);
                    break;
                }
                Some(ChannelMsg::Close) | Some(ChannelMsg::Eof) => {
                    println!("Channel closed unexpectedly.");
                    break;
                }
                _ => {}
            }
        }
        Ok(output.trim_end_matches('\n').to_string())
    }
}
