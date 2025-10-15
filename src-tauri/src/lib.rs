use crate::models::client::{Session, ShellChannel};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::Mutex;

pub mod models;

type SharedSession = Mutex<Option<Session>>;
type SharedChannel = Mutex<Option<ShellChannel>>;

#[derive(Deserialize)]
pub enum ProcessSort {
    #[serde(rename = "cpu")]
    Cpu,
    #[serde(rename = "ram")]
    Ram,
}

#[derive(Serialize)]
pub struct SystemInfo {
    cpu_usage: f32,
    memory: MemoryInfo,
    processes: Vec<ProcessInfo>,
}

#[derive(Serialize)]
pub struct MemoryInfo {
    used: f32,
    total: f32,
    percentage: f32,
}

#[derive(Serialize)]
pub struct ProcessInfo {
    name: String,
    cpu: f32,
    memory: f32,
}

#[derive(Serialize)]
pub struct StorageInfo {
    used: f32,
    total: f32,
    percentage: f32,
}

#[tauri::command]
async fn connect_ssh(
    username: String,
    host: String,
    port: u16,
    shared_session: State<'_, SharedSession>,
    shared_channel: State<'_, SharedChannel>,
) -> Result<String, String> {
    let mut session = Session::connect(username, (host.as_str(), port))
        .await
        .map_err(|e| e.to_string())?;

    let channel = session
        .channel_open_session()
        .await
        .map_err(|e| e.to_string())?;

    *shared_session.lock().await = Some(session);
    *shared_channel.lock().await = Some(ShellChannel::new(channel));

    Ok("Connected".to_string())
}

async fn execute_ssh_command(
    shell_channel: &mut ShellChannel,
    command: &str,
) -> Result<String, String> {
    let output = shell_channel
        .send_command(command)
        .await
        .map_err(|e| e.to_string())?;
    Ok(output)
}

#[tauri::command]
async fn execute_command(
    command: String,
    state: State<'_, SharedChannel>,
) -> Result<String, String> {
    let mut channel_guard = state.lock().await;
    let channel = channel_guard
        .as_mut()
        .ok_or_else(|| "SSH session not connected".to_string())?;

    let output = channel
        .send_command(&command)
        .await
        .map_err(|e| e.to_string())?;
    Ok(output)
}

#[tauri::command]
async fn get_system_info(
    sort_by: ProcessSort,
    state: State<'_, SharedChannel>
) -> Result<SystemInfo, String> {
    let mut channel_guard = state.lock().await;
    let channel = channel_guard
        .as_mut()
        .ok_or_else(|| "SSH session not connected".to_string())?;

    // Get system info using top in batch mode with a better formatted output
    let output = execute_ssh_command(
        channel,
        &format!(
            r#"top -bn1 -w 150 -o {} | awk '
                /Cpu/ {{
                    cpu=$2+$4
                    print "CPU " cpu
                }}
                /MiB Mem :/ {{
                    total=$4
                    free=$6
                    used=$8
                    print "MEM " total " " used " " free
                }}
                NR>7 && NR<18 {{
                    print "PROC " $12 " " $9 " " $10
                }}'"#,
            match sort_by {
                ProcessSort::Cpu => "%CPU",
                ProcessSort::Ram => "%MEM",
            }
        ),
    )
    .await?;

    let mut cpu_usage = 0.0;
    let mut mem_total = 0.0;
    let mut mem_used = 0.0;
    let mut processes = Vec::new();

    // Parse the output line by line
    for line in output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.get(0) {
            Some(&"CPU") => {
                if let Some(value) = parts.get(1).and_then(|v| v.parse().ok()) {
                    cpu_usage = value;
                }
            }
            Some(&"MEM") => {
                if let (Some(total), Some(used)) = (
                    parts.get(1).and_then(|v| v.parse::<f32>().ok()),
                    parts.get(2).and_then(|v| v.parse::<f32>().ok())
                ) {
                    mem_total = total;
                    mem_used = used;
                }
            }
            Some(&"PROC") => {
                if let (Some(name), Some(cpu), Some(mem)) = (
                    parts.get(1),
                    parts.get(2).and_then(|v| v.parse().ok()),
                    parts.get(3).and_then(|v| v.parse().ok())
                ) {
                    processes.push(ProcessInfo {
                        name: name.to_string(),
                        cpu,
                        memory: mem,
                    });
                }
            }
            _ => {}
        }
    }

    // Calculate memory percentage
    let mem_percentage = if mem_total > 0.0 {
        (mem_used / mem_total) * 100.0
    } else {
        0.0
    };

    Ok(SystemInfo {
        cpu_usage,
        memory: MemoryInfo {
            used: mem_used / 1024.0,  // Convert to GB
            total: mem_total / 1024.0, // Convert to GB
            percentage: mem_percentage,
        },
        processes,
    })
}

#[tauri::command]
async fn get_disk_usage(state: State<'_, SharedChannel>) -> Result<StorageInfo, String> {
    let mut channel_guard = state.lock().await;
    let channel = channel_guard
        .as_mut()
        .ok_or_else(|| "SSH session not connected".to_string())?;

    let output = execute_ssh_command(
        channel,
        "df -h / --output=size,used,pcent | awk 'NR==2 { printf \"%s,%s,%s\", $2, $1, $3 }' | tr -d 'G%'",
    )
    .await?;

    let mut values = output.split(',');
    let used = values.next()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0.0);
    let total = values.next()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0.0);
    let percentage = values.next()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0.0);

    Ok(StorageInfo {
        used,
        total,
        percentage,
    })
}

#[tauri::command]
async fn disconnect_ssh(state: State<'_, SharedSession>) -> Result<String, String> {
    let mut session_guard = state.lock().await;
    if let Some(mut session) = session_guard.take() {
        session.close().await.map_err(|e| e.to_string())?;
    }
    Ok("Disconnected".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(SharedSession::default())
        .manage(SharedChannel::default())
        .invoke_handler(tauri::generate_handler![
            connect_ssh,
            execute_command,
            get_system_info,
            get_disk_usage,
            disconnect_ssh
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
