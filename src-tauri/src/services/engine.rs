//! # Presence Engine Module
//!
//! The core brain of the application. Processes incoming events, applies priority rules,
//! handles debounce and anti-flicker, and sends updates to the Discord RPC thread.

use crate::models::types::{
    AppState, EngineCommand, EngineEvent, PresenceData, PresenceState,
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

                            if process_name.is_empty() {
                                if let Some(task) = settle_task.take() {
                                    task.abort();
                                }
                                self.app_state.discord_handle.send(EngineCommand::ClearActivity);
                            } else {
                                // 1. Resolve presence data in Rust
                                let (client_id, data) = self.resolve_presence(&process_name, &window_title).await;

                                // 2. Trigger settle task or update pre-idle data
                                if !self.is_idle {
                                    if let Some(task) = settle_task.take() {
                                        task.abort();
                                    }

                                    let settings = self.app_state.get_settings().await;
                                    let delay = settings.settle_delay_seconds;
                                    let tx = settle_tx.clone();
                                    let data_clone = data.clone();

                                    settle_task = Some(tauri::async_runtime::spawn(async move {
                                        sleep(Duration::from_secs(delay)).await;
                                        let _ = tx.send((client_id, data_clone)).await;
                                    }));
                                } else {
                                    self.pre_idle_data = Some(data);
                                }
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

    async fn resolve_presence(&self, process_name: &str, window_title: &str) -> (u64, PresenceData) {
        use crate::services::parser::{detect_catalog_app, parse_file_name, resolve_iconify_url, resolve_auto_image};
        use crate::services::presence_db::get_client_id_or_default;

        let rules = self.app_state.get_app_rules().await;
        
        // Find rule
        let matched_rule = rules.iter().find(|r| {
            if !r.enabled {
                return false;
            }
            let rule_proc = r.process_name.replace(".exe", "").replace(".EXE", "").to_lowercase();
            let proc = process_name.replace(".exe", "").replace(".EXE", "").to_lowercase();
            rule_proc == proc
        });

        // Resolve client ID
        let client_id = if let Some(rule) = matched_rule {
            if let Some(ref override_id) = rule.client_id {
                override_id.parse::<u64>().unwrap_or_else(|_| get_client_id_or_default(process_name))
            } else {
                get_client_id_or_default(process_name)
            }
        } else {
            get_client_id_or_default(process_name)
        };

        let mut details = String::new();
        let mut state = String::new();
        let mut large_image = String::new();
        let mut large_text = String::new();

        let app = detect_catalog_app(process_name);

        let default_client_id = 1517170930764480552;

        if let Some(rule) = matched_rule {
            let clean_file_name = parse_file_name(window_title, process_name);
            details = rule.details
                .replace("{file}", &clean_file_name)
                .replace("{title}", window_title);
            state = rule.state
                .replace("{file}", &clean_file_name)
                .replace("{title}", window_title);
            large_image = rule.large_image.clone();
            large_text = rule.display_name.clone();
        } else if let Some(catalog_app) = app {
            details = format!("Using {}", catalog_app.name);
            state = if window_title.is_empty() { "Active".to_string() } else { window_title.to_string() };
            large_image = "auto".to_string();
            large_text = catalog_app.name.to_string();
        } else if process_name.to_lowercase() == "explorer.exe" || process_name.to_lowercase() == "explorer" {
            let is_desktop = window_title.is_empty()
                || window_title == "Área de Trabalho"
                || window_title == "Desktop"
                || window_title == "Program Manager";

            if is_desktop {
                let phrases = [
                    ("Admirando o papel de parede", "Só de boa..."),
                    ("Organizando a Área de Trabalho", "Faxina digital"),
                    ("Refletindo sobre o próximo código", "Pensando..."),
                    ("Navegando pela Área de Trabalho", "Decidindo o que abrir"),
                    ("Explorando o ciberespaço", "No controle"),
                    ("Customizando o sistema", "Deixando no estilo"),
                ];
                // Select a pseudo-random phrase using timestamp to avoid rand dependency
                let idx = (chrono::Local::now().timestamp_millis().abs() as usize) % phrases.len();
                let phrase = phrases[idx];
                details = phrase.0.to_string();
                state = phrase.1.to_string();
            } else {
                details = "Organizando pastas".to_string();
                state = format!("Explorando: {}", window_title);
            }
            large_image = "default".to_string();
            large_text = "Windows Explorer".to_string();
        } else {
            let pretty_name = crate::services::presence_db::display_name_from_process_name(process_name);
            details = format!("Using {}", pretty_name);
            state = if window_title.is_empty() { "Active".to_string() } else { window_title.to_string() };
            large_image = "default".to_string();
            large_text = pretty_name;
        }

        // Image formatting/mapping
        if large_image == "auto" || large_image.is_empty() {
            if client_id == default_client_id && app.is_some() {
                let catalog_app = app.unwrap();
                large_image = resolve_iconify_url(catalog_app.icon);
            } else if let Some(catalog_app) = app {
                large_image = catalog_app.discord_asset.to_string();
            } else {
                large_image = "default".to_string();
            }
        } else if large_image.contains(':') {
            large_image = resolve_iconify_url(&large_image);
        } else if client_id == default_client_id 
            && large_image != "default" 
            && large_image != "idle" 
            && !large_image.starts_with("http") 
        {
            if let Some(catalog_app) = app {
                large_image = resolve_iconify_url(catalog_app.icon);
            } else {
                large_image = resolve_auto_image(process_name, &large_text);
            }
        }

        let presence_data = PresenceData {
            details,
            state,
            large_image,
            large_text,
            timestamp: 0, // Will be set by apply_presence
        };

        (client_id, presence_data)
    }
}

/// Helper function to spawn the engine task
pub fn start_engine(rx: mpsc::Receiver<EngineEvent>, app_state: AppState) {
    let engine = PresenceEngine::new(app_state);
    tauri::async_runtime::spawn(async move {
        engine.run(rx).await;
    });
}
