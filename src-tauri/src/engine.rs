//! # Presence Engine Module
//!
//! The core brain of the application. Processes incoming events, applies priority rules,
//! handles debounce and anti-flicker, and sends updates to the Discord RPC thread.

use crate::discord::DiscordHandle;
use crate::types::{
    AppRule, EngineCommand, EngineEvent, PresenceData, PresenceSource, PresenceState, Settings, ConnectionInfo
};
use log::{debug, info};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, RwLock};
use tokio::time::{sleep, Duration, Instant};

pub struct PresenceEngine {
    app_handle: AppHandle,
    discord_handle: DiscordHandle,
    app_rules: Arc<RwLock<Vec<AppRule>>>,
    settings: Arc<RwLock<Settings>>,

    current_state: PresenceState,
    current_data: PresenceData,
    current_source: PresenceSource,
    last_update_time: Option<Instant>,
    
    // For storing the last non-idle presence
    pre_idle_data: Option<PresenceData>,
}

impl PresenceEngine {
    pub fn new(
        app_handle: AppHandle,
        discord_handle: DiscordHandle,
        app_rules: Arc<RwLock<Vec<AppRule>>>,
        settings: Arc<RwLock<Settings>>,
    ) -> Self {
        Self {
            app_handle,
            discord_handle,
            app_rules,
            settings,
            current_state: PresenceState::Disconnected,
            current_data: PresenceData::default(),
            current_source: PresenceSource::Idle,
            last_update_time: None,
            pre_idle_data: None,
        }
    }

    pub async fn run(mut self, mut rx: mpsc::Receiver<EngineEvent>) {
        info!("Presence Engine started");
        
        let mut settle_task: Option<tokio::task::JoinHandle<()>> = None;
        let (settle_tx, mut settle_rx) = mpsc::channel::<PresenceData>(10);

        loop {
            tokio::select! {
                Some(event) = rx.recv() => {
                    match event {
                        EngineEvent::WindowChanged { process_name, window_title } => {
                            let rules = self.app_rules.read().await;
                            let settings = self.settings.read().await;
                            
                            // Find matching rule
                            let matched_rule = rules.iter().find(|r| r.enabled && r.process_name.to_lowercase() == process_name.to_lowercase());
                            
                            let mut new_data = PresenceData::default();
                            
                            if let Some(rule) = matched_rule {
                                new_data.details = rule.details.clone();
                                new_data.state = rule.state.clone();
                                new_data.large_image = rule.large_image.clone();
                                new_data.large_text = rule.display_name.clone();
                                new_data.source = rule.source.clone();
                            } else {
                                // Default "Using computer" fallback if priority allows
                                new_data.details = "Usando o computador".to_string();
                                new_data.state = window_title;
                                new_data.large_image = "default".to_string();
                                new_data.large_text = "Better Rich Presence".to_string();
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
                                
                                settle_task = Some(tokio::spawn(async move {
                                    sleep(Duration::from_secs(delay)).await;
                                    let _ = tx.send(data_clone).await;
                                }));
                            }
                        }
                        EngineEvent::IdleChanged { idle, idle_minutes } => {
                            let settings = self.settings.read().await;
                            if !settings.idle_enabled {
                                continue;
                            }
                            
                            if idle && idle_minutes >= settings.idle_threshold_minutes {
                                if self.current_source != PresenceSource::Game {
                                    info!("User is idle, switching presence");
                                    self.pre_idle_data = Some(self.current_data.clone());
                                    
                                    let idle_data = PresenceData {
                                        details: settings.idle_message.clone(),
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
                            self.discord_handle.send(EngineCommand::Disconnect);
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

    async fn apply_presence(&mut self, mut new_data: PresenceData) {
        // Diff check
        if new_data.details == self.current_data.details &&
           new_data.state == self.current_data.state &&
           new_data.large_image == self.current_data.large_image &&
           new_data.source == self.current_data.source {
               return; // Nothing changed meaningfully
        }
        
        let settings = self.settings.read().await;

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
        self.current_state = PresenceState::Connected; // Assuming connected if we send updates
        self.last_update_time = Some(Instant::now());

        // Update Discord
        self.discord_handle.send(EngineCommand::SetActivity(self.current_data.clone()));

        // Emit to frontend
        let _ = self.app_handle.emit("presence-updated", &self.current_data);
        let _ = self.app_handle.emit("state-changed", &self.current_state);
    }
}

/// Helper function to spawn the engine task
pub fn start_engine(
    rx: mpsc::Receiver<EngineEvent>,
    discord_handle: DiscordHandle,
    app_handle: AppHandle,
    app_rules: Arc<RwLock<Vec<AppRule>>>,
    settings: Arc<RwLock<Settings>>,
) {
    let engine = PresenceEngine::new(app_handle, discord_handle, app_rules, settings);
    tokio::spawn(async move {
        engine.run(rx).await;
    });
}
