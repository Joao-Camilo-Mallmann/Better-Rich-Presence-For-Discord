//! # Presence Engine Module
//!
//! The core brain of the application. Processes incoming events, applies priority rules,
//! handles debounce and anti-flicker, and sends updates to the Discord RPC thread.

use crate::models::types::{
    EngineCommand, EngineEvent, PresenceData, PresenceSource, PresenceState, AppState
};
use log::{debug, info};
use tauri::Emitter;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration, Instant};

pub struct PresenceEngine {
    app_state: AppState,

    current_state: PresenceState,
    current_data: PresenceData,
    current_source: PresenceSource,
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
            current_source: PresenceSource::Idle,
            last_update_time: None,
            pre_idle_data: None,
        }
    }

    pub async fn run(mut self, mut rx: mpsc::Receiver<EngineEvent>) {
        info!("Presence Engine started");
        
        let mut settle_task: Option<tauri::async_runtime::JoinHandle<()>> = None;
        let (settle_tx, mut settle_rx) = mpsc::channel::<PresenceData>(10);
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
                            let rules = self.app_state.get_app_rules().await;
                            let settings = self.app_state.get_settings().await;
                            
                            // Emit priority info to frontend
                            let priority_info = crate::models::types::PriorityInfo {
                                active: is_prioritized,
                                prioritized_app: if is_prioritized { Some(process_name.clone()) } else { None },
                                foreground_app,
                            };
                            let _ = self.app_state.app_handle.emit("priority-info", &priority_info);
                            
                            // Find matching rule (cross-platform, stripping .exe)
                            let matched_rule = rules.iter().find(|r| {
                                if !r.enabled {
                                    return false;
                                }
                                let rule_proc = r.process_name.strip_suffix(".exe").unwrap_or(&r.process_name).to_lowercase();
                                let proc = process_name.strip_suffix(".exe").unwrap_or(&process_name).to_lowercase();
                                rule_proc == proc
                            });
                            
                            let mut new_data = PresenceData::default();
                            
                            if let Some(rule) = matched_rule {
                                let file_name = crate::services::parser::parse_file_name(&window_title, &process_name);
                                
                                let mut details = rule.details.clone();
                                let mut state = rule.state.clone();
                                
                                // Automatic upgrade of old default rules
                                let lower_proc = process_name.strip_suffix(".exe").unwrap_or(&process_name).to_lowercase();
                                if lower_proc == "cursor" {
                                    if details == "Programando no Cursor" {
                                        details = "Editando {file}".to_string();
                                    }
                                    if state == "Desenvolvendo" {
                                        state = "No Cursor".to_string();
                                    }
                                } else if lower_proc == "code" {
                                    if details == "Programando no VSCode" {
                                        details = "Editando {file}".to_string();
                                    }
                                    if state == "Desenvolvendo" {
                                        state = "No VS Code".to_string();
                                    }
                                } else if lower_proc == "antigravity ide" || lower_proc == "antigravity-ide" {
                                    if details == "Programando no Antigravity IDE" {
                                        details = "Editando {file}".to_string();
                                    }
                                    if state == "Desenvolvendo" {
                                        state = "No Antigravity IDE".to_string();
                                    }
                                }
                                
                                new_data.details = details
                                    .replace("{file}", &file_name)
                                    .replace("{title}", &window_title);
                                    
                                new_data.state = state
                                    .replace("{file}", &file_name)
                                    .replace("{title}", &window_title);
                                
                                new_data.large_image = if rule.large_image == "auto" {
                                    crate::services::parser::resolve_auto_image(&rule.process_name, &rule.display_name)
                                } else {
                                    rule.large_image.clone()
                                };
                                new_data.large_text = rule.display_name.clone();
                                new_data.source = rule.source.clone();
                            } else {
                                // Default "Using computer" fallback if priority allows
                                new_data.details = "Usando o computador".to_string();
                                new_data.state = window_title.clone();
                                new_data.large_image = crate::services::parser::resolve_auto_image(&process_name, &window_title);
                                new_data.large_text = window_title;
                                new_data.source = PresenceSource::Browser; // Give it a low priority
                            }
                            
                            // Check priority
                            if new_data.source.priority() <= self.current_source.priority() || self.current_source == PresenceSource::Idle {
                                // Start anti-flicker settle timer
                                if let Some(task) = settle_task.take() {
                                    task.abort();
                                }
                                
                                let delay = settings.settle_delay_seconds;
                                let tx = settle_tx.clone();
                                let data_clone = new_data.clone();
                                
                                settle_task = Some(tauri::async_runtime::spawn(async move {
                                    sleep(Duration::from_secs(delay)).await;
                                    let _ = tx.send(data_clone).await;
                                }));
                            }
                        }
                        EngineEvent::IdleChanged { idle, idle_minutes } => {
                            let settings = self.app_state.get_settings().await;
                            if !settings.idle_enabled {
                                continue;
                            }
                            
                            if idle && idle_minutes >= settings.idle_threshold_minutes {
                                if self.current_source != PresenceSource::Game {
                                    info!("User is idle, switching presence");
                                    self.pre_idle_data = Some(self.current_data.clone());
                                    
                                    let idle_data = PresenceData {
                                        details: settings.idle_message,
                                        state: "Inativo".to_string(),
                                        large_image: "idle".to_string(),
                                        large_text: "Inativo".to_string(),
                                        source: PresenceSource::Idle,
                                        timestamp: chrono::Utc::now().timestamp(),
                                    };
                                    
                                    self.apply_presence(idle_data).await;
                                }
                            } else if !idle && self.current_source == PresenceSource::Idle {
                                info!("User is active again, restoring presence");
                                if let Some(pre_idle) = self.pre_idle_data.take() {
                                    // Update timestamp to now to reset timer
                                    let mut restored = pre_idle;
                                    restored.timestamp = chrono::Utc::now().timestamp();
                                    self.apply_presence(restored).await;
                                }
                            }
                        }
                        EngineEvent::ManualProfile(data) => {
                            if let Some(task) = settle_task.take() {
                                task.abort();
                            }
                            self.apply_presence(data).await;
                        }
                        EngineEvent::Shutdown => {
                            info!("Engine received shutdown signal");
                            self.app_state.discord_handle.send(EngineCommand::Disconnect);
                            break;
                        }
                    }
                }
                
                Some(settled_data) = settle_rx.recv() => {
                    // This data has survived the anti-flicker delay
                    self.apply_presence(settled_data).await;
                }
            }
        }
    }

    async fn sync_state(&mut self) {
        let is_connected = self.app_state.discord_handle.is_connected();
        
        // Use AppState connection status helper
        let (conn_changed, state_changed, new_state) = self.app_state.update_connection_status(is_connected).await;
        
        if conn_changed {
            if is_connected {
                info!("[Engine] Discord reconnected, sending current activity");
                self.app_state.discord_handle.send(EngineCommand::SetActivity(self.current_data.clone()));
            }
            let conn_payload = self.app_state.get_connection_status().await;
            let _ = self.app_state.app_handle.emit("connection-changed", &conn_payload);
        }
        
        if state_changed {
            self.current_state = new_state;
            let _ = self.app_state.app_handle.emit("state-changed", &new_state);
        }

        // Sync presence data and source
        let presence_changed = self.app_state.update_presence_data(self.current_data.clone()).await;
        if presence_changed {
            let _ = self.app_state.app_handle.emit("presence-updated", &self.current_data);
        }
    }

    async fn apply_presence(&mut self, mut new_data: PresenceData) {
        // Diff check
        if new_data.details == self.current_data.details &&
           new_data.state == self.current_data.state &&
           new_data.large_image == self.current_data.large_image &&
           new_data.source == self.current_source {
               return; // Nothing changed meaningfully
        }
        
        let settings = self.app_state.get_settings().await;

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

        // Keep timestamp if source is the same to avoid timer reset
        if new_data.source == self.current_source && self.current_data.timestamp > 0 {
            new_data.timestamp = self.current_data.timestamp;
        } else if new_data.timestamp == 0 {
             new_data.timestamp = chrono::Utc::now().timestamp();
        }

        self.current_data = new_data.clone();
        self.current_source = new_data.source.clone();
        self.last_update_time = Some(Instant::now());

        // Update Discord
        self.app_state.discord_handle.send(EngineCommand::SetActivity(self.current_data.clone()));

        // Sync to shared state and emit events
        self.sync_state().await;
    }
}

/// Helper function to spawn the engine task
pub fn start_engine(
    rx: mpsc::Receiver<EngineEvent>,
    app_state: AppState,
) {
    let engine = PresenceEngine::new(app_state);
    tauri::async_runtime::spawn(async move {
        engine.run(rx).await;
    });
}
