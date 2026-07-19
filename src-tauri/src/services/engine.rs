//! # Presence Engine Module
//!
//! The core brain of the application. Processes incoming events, applies priority rules,
//! handles debounce and anti-flicker, and sends updates to the Discord RPC thread.

use crate::models::types::{
    AppState, EngineCommand, EngineEvent, PresenceData, PresenceState,
};
use crate::services::presence_db::{
    display_name_from_process_name, get_client_id_or_default, DEFAULT_CLIENT_ID,
};
use log::{debug, info};
use tauri::Emitter;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration, Instant};

pub struct PresenceEngine {
    app_state: AppState,

    current_state: PresenceState,
    current_data: PresenceData,
    is_idle: bool,
    current_client_id: u64,
    last_update_time: Option<Instant>,

    // For storing the last non-idle presence
    pre_idle_data: Option<PresenceData>,
}

impl PresenceEngine {
    pub fn new(app_state: AppState) -> Self {
        Self {
            app_state,
            current_state: PresenceState::Disconnected,
            current_data: PresenceData::default(),
            is_idle: false,
            current_client_id: 1517170930764480552,
            last_update_time: None,
            pre_idle_data: None,
        }
    }

    pub async fn run(mut self, mut rx: mpsc::Receiver<EngineEvent>) {
        info!("Presence Engine started");

        let mut settle_task: Option<tauri::async_runtime::JoinHandle<()>> = None;
        let (settle_tx, mut settle_rx) = mpsc::channel::<(u64, PresenceData)>(10);
        let mut connection_check_interval = tokio::time::interval(Duration::from_secs(1));

        // Initial sync of state to frontend
        self.sync_state().await;

        loop {
            tokio::select! {
                _ = connection_check_interval.tick() => {
                    self.sync_state().await;
                }
                Some(event) = rx.recv() => {
                    match event {
                        EngineEvent::WindowChanged { process_name, window_title, is_prioritized, foreground_app } => {
                            // Emit priority info to frontend
                            let priority_info = crate::models::types::PriorityInfo {
                                active: is_prioritized,
                                prioritized_app: if is_prioritized { Some(process_name.clone()) } else { None },
                                foreground_app,
                            };
                            let _ = self.app_state.app_handle.emit("priority-info", &priority_info);

                            // Emit event to TS so it can resolve the presence
                            let _ = self.app_state.app_handle.emit(
                                "active-process-changed",
                                serde_json::json!({
                                    "process_name": process_name,
                                    "window_title": window_title,
                                }),
                            );
                        }
                        EngineEvent::ResolvedPresence { client_id, data } => {
                            let settings = self.app_state.get_settings().await;

                            if !self.is_idle {
                                // Start anti-flicker settle timer
                                if let Some(task) = settle_task.take() {
                                    task.abort();
                                }

                                let delay = settings.settle_delay_seconds;
                                let tx = settle_tx.clone();
                                let data_clone = data.clone();

                                settle_task = Some(tauri::async_runtime::spawn(async move {
                                    sleep(Duration::from_secs(delay)).await;
                                    let _ = tx.send((client_id, data_clone)).await;
                                }));
                            } else {
                                self.pre_idle_data = Some(data.clone());
                            }
                        }
                        EngineEvent::ClearPresence => {
                            if let Some(task) = settle_task.take() {
                                task.abort();
                            }
                            self.app_state.discord_handle.send(EngineCommand::ClearActivity);
                        }
                        EngineEvent::IdleChanged { idle, idle_minutes } => {
                            let settings = self.app_state.get_settings().await;
                            if !settings.idle_enabled {
                                continue;
                            }

                            if idle && idle_minutes >= settings.idle_threshold_minutes {
                                if !self.is_idle {
                                    info!("User is idle, switching presence");
                                    self.pre_idle_data = Some(self.current_data.clone());
                                    self.is_idle = true;

                                    let idle_data = PresenceData {
                                        details: settings.idle_message,
                                        state: "Inativo".to_string(),
                                        large_image: "idle".to_string(),
                                        large_text: "Inativo".to_string(),
                                        timestamp: chrono::Utc::now().timestamp(),
                                    };

                                    self.apply_presence(1517170930764480552, idle_data).await;
                                }
                            } else if !idle && self.is_idle {
                                info!("User is active again, restoring presence");
                                self.is_idle = false;
                                if let Some(pre_idle) = self.pre_idle_data.take() {
                                    // Update timestamp to now to reset timer
                                    let mut restored = pre_idle;
                                    restored.timestamp = chrono::Utc::now().timestamp();
                                    self.apply_presence(self.current_client_id, restored).await;
                                }
                            }
                        }
                        EngineEvent::ManualProfile(data) => {
                            if let Some(task) = settle_task.take() {
                                task.abort();
                            }
                            self.apply_presence(1517170930764480552, data).await;
                        }
                        EngineEvent::Shutdown => {
                            info!("Engine received shutdown signal");
                            self.app_state.discord_handle.send(EngineCommand::Disconnect);
                            break;
                        }
                    }
                }

                Some((client_id, settled_data)) = settle_rx.recv() => {
                    // This data has survived the anti-flicker delay
                    self.apply_presence(client_id, settled_data).await;
                }
            }
        }
    }

    async fn sync_state(&mut self) {
        let is_connected = self.app_state.discord_handle.is_connected();

        // Use AppState connection status helper
        let (conn_changed, state_changed, new_state) =
            self.app_state.update_connection_status(is_connected).await;

        if conn_changed {
            if is_connected {
                let settings = self.app_state.get_settings().await;
                if settings.global_enabled {
                    info!("[Engine] Discord reconnected, sending current activity");
                    self.app_state
                        .discord_handle
                        .send(EngineCommand::SetActivity {
                            client_id: self.current_client_id,
                            data: self.current_data.clone(),
                        });
                }
            }
            let conn_payload = self.app_state.get_connection_status().await;
            let _ = self
                .app_state
                .app_handle
                .emit("connection-changed", &conn_payload);
        }

        if state_changed {
            self.current_state = new_state;
            let _ = self.app_state.app_handle.emit("state-changed", &new_state);
        }

        // Sync presence data and source
        let presence_changed = self
            .app_state
            .update_presence_data(self.current_client_id, self.current_data.clone())
            .await;
        if presence_changed {
            let _ = self
                .app_state
                .app_handle
                .emit("presence-updated", &self.current_data);
        }
    }

    async fn apply_presence(&mut self, client_id: u64, mut new_data: PresenceData) {
        // Diff check
        if new_data.details == self.current_data.details
            && new_data.state == self.current_data.state
            && new_data.large_image == self.current_data.large_image
            && client_id == self.current_client_id
        {
            return; // Nothing changed meaningfully
        }

        let settings = self.app_state.get_settings().await;

        // If global Rich Presence is disabled, do nothing
        if !settings.global_enabled {
            return;
        }

        // Rate limit (Debounce)
        if let Some(last_time) = self.last_update_time {
            let elapsed = last_time.elapsed();
            let debounce = Duration::from_secs(settings.debounce_seconds.max(15)); // Discord requires ~15s
            if elapsed < debounce {
                let wait_time = debounce - elapsed;
                debug!("Rate limiting update, waiting {:?}", wait_time);
                sleep(wait_time).await;
            }
        }

        // Keep timestamp if client_id is the same to avoid timer reset
        if client_id == self.current_client_id && self.current_data.timestamp > 0 {
            new_data.timestamp = self.current_data.timestamp;
        } else if new_data.timestamp == 0 {
            new_data.timestamp = chrono::Utc::now().timestamp();
        }

        self.current_data = new_data.clone();
        self.current_client_id = client_id;
        self.last_update_time = Some(Instant::now());

        // Update Discord
        self.app_state
            .discord_handle
            .send(EngineCommand::SetActivity {
                client_id: self.current_client_id,
                data: self.current_data.clone(),
            });

        // Sync to shared state and emit events
        self.sync_state().await;
    }
}

/// Helper function to spawn the engine task
pub fn start_engine(rx: mpsc::Receiver<EngineEvent>, app_state: AppState) {
    let engine = PresenceEngine::new(app_state);
    tauri::async_runtime::spawn(async move {
        engine.run(rx).await;
    });
}

fn fallback_app_display_name(process_name: &str, window_title: &str) -> String {
    let process_display_name = display_name_from_process_name(process_name);
    if !process_display_name.is_empty() {
        return process_display_name;
    }

    let title = window_title.trim();
    if !title.is_empty() {
        return title.to_string();
    }

    "Unknown app".to_string()
}

fn fallback_window_state(app_name: &str, window_title: &str) -> String {
    let title = window_title.trim();
    if title.is_empty() || title.eq_ignore_ascii_case(app_name) {
        return "Active".to_string();
    }

    title.to_string()
}

fn ensure_visible_app_name(data: &mut PresenceData, app_name: &str) {
    let app_name = app_name.trim();
    if app_name.is_empty() {
        return;
    }

    let details = data.details.trim();
    if details.is_empty()
        || details.eq_ignore_ascii_case("Using the computer")
        || details.eq_ignore_ascii_case("Usando o computador")
    {
        data.details = format!("Using {}", app_name);
    } else if !details.to_lowercase().contains(&app_name.to_lowercase()) {
        data.details = format!("{} - {}", app_name, details);
    }

    if data.large_text.trim().is_empty()
        || data.large_text.eq_ignore_ascii_case("Better Rich Presence")
    {
        data.large_text = app_name.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_presence_uses_process_name_as_visible_app_name() {
        let app_name = fallback_app_display_name("custom_tool.exe", "Project - Custom Tool");
        let mut data = PresenceData {
            details: format!("Using {}", app_name),
            state: fallback_window_state(&app_name, "Project - Custom Tool"),
            large_image: String::new(),
            large_text: app_name.clone(),
            timestamp: 0,
        };

        ensure_visible_app_name(&mut data, &app_name);

        assert_eq!(app_name, "Custom Tool");
        assert_eq!(data.details, "Using Custom Tool");
        assert_eq!(data.state, "Project - Custom Tool");
        assert_eq!(data.large_text, "Custom Tool");
    }

    #[test]
    fn generic_client_presence_prefixes_rule_details_with_app_name() {
        let mut data = PresenceData {
            details: "Editing main.rs".to_string(),
            state: "Developing".to_string(),
            large_image: "auto".to_string(),
            large_text: "Cursor".to_string(),
            timestamp: 0,
        };

        ensure_visible_app_name(&mut data, "Cursor");

        assert_eq!(data.details, "Cursor - Editing main.rs");
    }
}
