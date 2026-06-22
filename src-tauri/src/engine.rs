//! # Presence Engine Module
//!
//! The core brain of the application. Processes incoming events, applies priority rules,
//! handles debounce and anti-flicker, and sends updates to the Discord RPC thread.

use crate::discord::DiscordHandle;
use crate::types::{
    AppRule, EngineCommand, EngineEvent, PresenceData, PresenceSource, PresenceState, Settings
};
use crate::commands::AppState;
use log::{debug, info};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
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
                        EngineEvent::WindowChanged { process_name, window_title } => {
                            let rules = self.app_rules.read().await;
                            let settings = self.settings.read().await;
                            
                            // Find matching rule
                            let matched_rule = rules.iter().find(|r| r.enabled && r.process_name.to_lowercase() == process_name.to_lowercase());
                            
                            let mut new_data = PresenceData::default();
                            
                            if let Some(rule) = matched_rule {
                                let file_name = parse_file_name(&window_title, &process_name);
                                
                                let mut details = rule.details.clone();
                                let mut state = rule.state.clone();
                                
                                // Automatic upgrade of old default rules
                                let lower_proc = process_name.to_lowercase();
                                if lower_proc == "cursor.exe" {
                                    if details == "Programando no Cursor" {
                                        details = "Editando {file}".to_string();
                                    }
                                    if state == "Desenvolvendo" {
                                        state = "No Cursor".to_string();
                                    }
                                } else if lower_proc == "code.exe" {
                                    if details == "Programando no VSCode" {
                                        details = "Editando {file}".to_string();
                                    }
                                    if state == "Desenvolvendo" {
                                        state = "No VS Code".to_string();
                                    }
                                } else if lower_proc == "antigravity ide.exe" || lower_proc == "antigravity-ide.exe" {
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
                                    resolve_auto_image(&rule.process_name, &rule.display_name)
                                } else {
                                    rule.large_image.clone()
                                };
                                new_data.large_text = rule.display_name.clone();
                                new_data.source = rule.source.clone();
                            } else {
                                // Default "Using computer" fallback if priority allows
                                new_data.details = "Usando o computador".to_string();
                                new_data.state = window_title.clone();
                                new_data.large_image = resolve_auto_image(&process_name, &window_title);
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
                            let (idle_enabled, idle_threshold_minutes, idle_message) = {
                                let settings = self.settings.read().await;
                                (settings.idle_enabled, settings.idle_threshold_minutes, settings.idle_message.clone())
                            };
                            if !idle_enabled {
                                continue;
                            }
                            
                            if idle && idle_minutes >= idle_threshold_minutes {
                                if self.current_source != PresenceSource::Game {
                                    info!("User is idle, switching presence");
                                    self.pre_idle_data = Some(self.current_data.clone());
                                    
                                    let idle_data = PresenceData {
                                        details: idle_message,
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

    async fn sync_state(&mut self) {
        if let Some(state) = self.app_handle.try_state::<AppState>() {
            let is_connected = self.discord_handle.is_connected();
            
            // 1. Check if connection status changed
            let mut conn_changed = false;
            let mut conn_info = state.connection_info.write().await;
            if conn_info.connected != is_connected {
                conn_info.connected = is_connected;
                conn_info.state = if is_connected {
                    PresenceState::Connected
                } else {
                    PresenceState::Disconnected
                };
                conn_changed = true;
                
                // If we just transitioned to connected, make sure we push the current activity
                if is_connected {
                    info!("[Engine] Discord reconnected, sending current activity");
                    self.discord_handle.send(EngineCommand::SetActivity(self.current_data.clone()));
                }
            }
            
            // Also keep engine's local current_state in sync
            let new_state = conn_info.state;
            let state_changed = self.current_state != new_state;
            if state_changed {
                self.current_state = new_state;
                *state.presence_state.write().await = new_state;
            }
            
            // Drop connection info lock before emitting to avoid deadlocks
            let conn_payload = conn_info.clone();
            drop(conn_info);
            
            if conn_changed {
                let _ = self.app_handle.emit("connection-changed", &conn_payload);
            }
            if state_changed {
                let _ = self.app_handle.emit("state-changed", &new_state);
            }

            // 2. Sync presence data
            let mut presence_changed = false;
            let mut current_presence = state.current_presence.write().await;
            if current_presence.as_ref() != Some(&self.current_data) {
                *current_presence = Some(self.current_data.clone());
                presence_changed = true;
            }
            drop(current_presence);

            // 3. Sync source
            let mut current_source = state.current_source.write().await;
            if *current_source != self.current_source {
                *current_source = self.current_source;
            }
            drop(current_source);

            // Emit events if changed
            if presence_changed {
                let _ = self.app_handle.emit("presence-updated", &self.current_data);
            }
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
        drop(settings);

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
        self.discord_handle.send(EngineCommand::SetActivity(self.current_data.clone()));

        // Sync to shared state and emit events
        self.sync_state().await;
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
    tauri::async_runtime::spawn(async move {
        engine.run(rx).await;
    });
}

/// Resolve a process or display name to a favicon URL using a lookup table and guessing.
fn resolve_auto_image(process_name: &str, display_name: &str) -> String {
    let name_to_search = if !display_name.is_empty() {
        display_name.to_lowercase()
    } else {
        process_name.to_lowercase()
    };

    let guessed_domain: String;
    let domain = match name_to_search.as_str() {
        // Dev / Editors
        n if n.contains("vscode") || n.contains("visual studio code") => "visualstudio.com",
        n if n.contains("cursor") => "cursor.com",
        n if n.contains("antigravity") => "deepmind.com",
        n if n.contains("intellij") => "jetbrains.com",
        n if n.contains("android studio") => "developer.android.com",
        n if n.contains("visual studio") => "visualstudio.com",
        n if n.contains("sublime") => "sublimetext.com",
        n if n.contains("webstorm") => "jetbrains.com",
        n if n.contains("rider") => "jetbrains.com",
        n if n.contains("clion") => "jetbrains.com",
        n if n.contains("datagrip") => "jetbrains.com",
        n if n.contains("postman") => "postman.com",
        n if n.contains("insomnia") => "insomnia.rest",
        n if n.contains("pgadmin") => "pgadmin.org",
        n if n.contains("dbeaver") => "dbeaver.io",
        
        // Browsers
        n if n.contains("chrome") => "google.com",
        n if n.contains("firefox") => "mozilla.org",
        n if n.contains("edge") => "microsoft.com",
        n if n.contains("opera") => "opera.com",
        n if n.contains("brave") => "brave.com",
        n if n.contains("safari") => "apple.com",

        // Office / Productivity
        n if n.contains("notion") => "notion.so",
        n if n.contains("obsidian") => "obsidian.md",
        n if n.contains("excel") => "microsoft.com",
        n if n.contains("word") => "microsoft.com",
        n if n.contains("powerpoint") => "microsoft.com",
        n if n.contains("teams") => "microsoft.com",
        n if n.contains("trello") => "trello.com",
        n if n.contains("asana") => "asana.com",
        n if n.contains("jira") => "atlassian.com",
        n if n.contains("confluence") => "atlassian.com",
        n if n.contains("slack") => "slack.com",
        n if n.contains("discord") => "discord.com",
        n if n.contains("telegram") => "telegram.org",
        n if n.contains("whatsapp") => "whatsapp.com",
        n if n.contains("zoom") => "zoom.us",
        n if n.contains("skype") => "skype.com",

        // Design / Media
        n if n.contains("figma") => "figma.com",
        n if n.contains("photoshop") => "adobe.com",
        n if n.contains("illustrator") => "adobe.com",
        n if n.contains("premiere") => "adobe.com",
        n if n.contains("after effects") => "adobe.com",
        n if n.contains("canva") => "canva.com",
        n if n.contains("blender") => "blender.org",
        n if n.contains("unity") => "unity.com",
        n if n.contains("unreal") => "unrealengine.com",
        n if n.contains("vlc") => "videolan.org",
        n if n.contains("obs") || n.contains("obs64") => "obsproject.com",

        // Entertainment / Gaming
        n if n.contains("spotify") => "spotify.com",
        n if n.contains("steam") => "steampowered.com",
        n if n.contains("github") => "github.com",
        n if n.contains("docker") => "docker.com",
        n if n.contains("netflix") => "netflix.com",
        n if n.contains("youtube") => "youtube.com",
        n if n.contains("twitch") => "twitch.tv",
        n if n.contains("minecraft") => "minecraft.net",
        n if n.contains("roblox") => "roblox.com",
        n if n.contains("league of legends") => "leagueoflegends.com",
        n if n.contains("valorant") => "playvalorant.com",
        n if n.contains("counter-strike") || n.contains("cs2") || n.contains("csgo") => "counter-strike.net",
        
        // Default guessing
        _ => {
            // Remove ".exe" and sanitize
            let clean_name = name_to_search
                .replace(".exe", "")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
                .collect::<String>();
            
            // Try to extract the first word or join words
            let first_word = clean_name.split_whitespace().next().unwrap_or("").trim().to_string();
            guessed_domain = if !first_word.is_empty() {
                format!("{}.com", first_word)
            } else {
                format!("{}.com", clean_name.replace(' ', ""))
            };
            &guessed_domain
        }
    };

    format!("https://www.google.com/s2/favicons?sz=128&domain={}", domain)
}

/// Helper to extract the active file name from an editor window title.
fn parse_file_name(window_title: &str, process_name: &str) -> String {
    // If it's Cursor or VS Code, the title is usually: "● filename.ext - folder - Cursor"
    let parts: Vec<&str> = window_title.split(" - ").collect();
    if parts.len() >= 2 {
        let first_part = parts[0].trim();
        // Remove unsaved indicator dot or asterisk if present
        let clean_part = first_part
            .trim_start_matches('●')
            .trim_start_matches('*')
            .trim();
        
        if clean_part.is_empty() {
            "Sem arquivo aberto".to_string()
        } else {
            clean_part.to_string()
        }
    } else {
        // Fallback: clean the window title a bit by removing the process suffix
        let suffix = match process_name.to_lowercase().as_str() {
            "cursor.exe" => " - Cursor",
            "code.exe" => " - Visual Studio Code",
            "antigravity ide.exe" | "antigravity-ide.exe" => " - Antigravity IDE",
            _ => "",
        };
        if !suffix.is_empty() && window_title.ends_with(suffix) {
            let clean = window_title[..window_title.len() - suffix.len()].trim();
            if clean.is_empty() {
                "Sem arquivo aberto".to_string()
            } else {
                clean.to_string()
            }
        } else {
            let clean = window_title.trim();
            if clean.is_empty() {
                "Sem arquivo aberto".to_string()
            } else {
                clean.to_string()
            }
        }
    }
}
