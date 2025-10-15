import { writable, get } from 'svelte/store';

/**
 * TODO: Heartbeat Implementation Plan
 * The heartbeat should be implemented as a Tauri command in Rust that:
 * 1. Executes a lightweight command like `echo "heartbeat"` on the SSH connection
 * 2. Verifies the response to confirm connection is alive
 * 3. This approach minimizes load on the SSH channel compared to running system info commands
 * 
 * Implementation steps:
 * 1. Create a new Rust command in src-tauri/src/commands.rs: check_ssh_heartbeat
 * 2. Command should use existing SSH session to send echo command
 * 3. Return success/failure status to frontend
 */

interface ConnectionState {
    isConnected: boolean;
    config: {
        host: string;
        port: number;
        username: string;
    };
}

const createConnectionStore = () => {
    const config = {
        host: '100.98.93.96',
        port: 22,
        username: 'opc',
    }
    return writable<ConnectionState>({
        isConnected: false,
        config,
    });
    
}

export const connectionStore = createConnectionStore();
