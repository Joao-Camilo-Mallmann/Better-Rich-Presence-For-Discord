use crate::services::presence_db::get_client_id;
use discord_presence::Client as DiscordIpcClient;
use log::{error, info, warn};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::task;

// Fallback application ID if the active process isn't in our DB
const DEFAULT_CLIENT_ID: &str = "1517170930764480552";

pub struct PresenceManager {
    pub current_client_id: Option<String>,
    pub rpc: Option<DiscordIpcClient>,
    start_time: Option<u64>,
}

impl Default for PresenceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PresenceManager {
    pub fn new() -> Self {
        Self {
            current_client_id: None,
            rpc: None,
            start_time: None,
        }
    }

    /// Async method to switch the current Discord RPC presence based on the process name and window title.
    pub async fn switch_to(&mut self, process_name: &str, window_title: &str) {
        let client_id_str = get_client_id(process_name).unwrap_or(DEFAULT_CLIENT_ID);
        let client_id = client_id_str.to_string();

        let client_id_changed = match &self.current_client_id {
            Some(current) => current != &client_id,
            None => true,
        };

        if client_id_changed {
            info!(
                "[PresenceManager] Switching Discord Client ID to {}",
                client_id
            );
            self.disconnect().await;
            self.current_client_id = Some(client_id.clone());
            self.start_time = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            );
            self.reconnect(client_id_str.parse::<u64>().unwrap_or(0))
                .await;
        }

        self.update_activity(process_name, window_title).await;
    }

    /// Safely disconnect the current RPC client.
    async fn disconnect(&mut self) {
        if let Some(mut client) = self.rpc.take() {
            info!("[PresenceManager] Disconnecting current RPC client...");
            let _ = task::spawn_blocking(move || {
                let _ = client.clear_activity();
                // Client is dropped here, closing the connection.
            })
            .await;
        }
    }

    /// Reconnect the RPC client with a new Client ID.
    async fn reconnect(&mut self, app_id: u64) {
        info!("[PresenceManager] Reconnecting with App ID: {}", app_id);

        let mut client = DiscordIpcClient::new(app_id);

        client
            .on_ready(|_ctx| {
                info!("[PresenceManager] Discord RPC Ready.");
            })
            .persist();

        client
            .on_error(|_ctx| {
                error!("[PresenceManager] Discord RPC Error.");
            })
            .persist();

        // start() does IPC handshake which can block, run in blocking task
        match task::spawn_blocking(move || {
            client.start();
            client
        })
        .await
        {
            Ok(started_client) => {
                self.rpc = Some(started_client);
            }
            Err(e) => {
                error!("[PresenceManager] Failed to start RPC client: {}", e);
            }
        }
    }

    /// Update the activity for the current process and window title.
    pub async fn update_activity(&mut self, process_name: &str, window_title: &str) {
        if let Some(client) = self.rpc.as_mut() {
            let details = window_title.to_string();
            // Using a simple process name parsing to look nicer (e.g., Code.exe -> Code)
            let pretty_name = process_name.strip_suffix(".exe").unwrap_or(process_name);
            let state = format!("Using {}", pretty_name);
            let start_time = self.start_time.unwrap_or(0);

            // Discord API requires strings to be at least 2 chars and max 128
            let mut safe_details = details.clone();
            if safe_details.is_empty() {
                safe_details.push_str("Browsing...");
            }
            if safe_details.len() < 2 {
                safe_details.push_str("  ");
            }
            if safe_details.len() > 128 {
                let mut end_idx = 125;
                while !safe_details.is_char_boundary(end_idx) {
                    end_idx -= 1;
                }
                safe_details.truncate(end_idx);
                safe_details.push_str("...");
            }

            let mut safe_state = state.clone();
            if safe_state.len() < 2 {
                safe_state.push_str("  ");
            }

            let res = client.set_activity(|mut act| {
                act = act.details(&safe_details).state(&safe_state);
                if start_time > 0 {
                    act = act.timestamps(|ts| ts.start(start_time));
                }
                act
            });

            if let Err(e) = res {
                error!("[PresenceManager] Failed to update activity: {}", e);
            }
        } else {
            warn!("[PresenceManager] Cannot update activity: RPC not connected.");
        }
    }
}
