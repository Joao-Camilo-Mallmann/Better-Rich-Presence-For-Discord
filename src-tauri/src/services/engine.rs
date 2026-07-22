//! # Presence Engine Module
//!
//! The core brain of the application. Processes incoming events, applies priority rules,
//! handles debounce and anti-flicker, and sends updates to the Discord RPC thread.

use crate::models::types::{
    AppState, EngineCommand, EngineEvent, PresenceData, PresenceState, DEFAULT_CLIENT_ID,
};
use crate::services::app_registry::{AppRegistry, format_app_presence};
use log::{debug, info};
use std::collections::HashMap;
use tauri::Emitter;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration, Instant};

async fn fetch_discord_app_details(client_id: u64) -> Option<(String, String)> {
    let url = format!("https://discord.com/api/v9/oauth2/applications/{}/rpc", client_id);
    let client = reqwest::Client::builder()
        .user_agent("Better-Rich-Presence/0.1.0")
        .timeout(Duration::from_secs(3))
        .build()
        .ok()?;
    let response = client.get(&url).send().await.ok()?;
    if response.status().is_success() {
        #[derive(serde::Deserialize)]
        struct DiscordAppResponse {
            name: String,
            icon: Option<String>,
        }
        let app: DiscordAppResponse = response.json().await.ok()?;
        Some((app.name, app.icon.unwrap_or_default()))
    } else {
        None
    }
}

pub struct PresenceEngine {
    app_state: AppState,

    current_state: PresenceState,
    current_data: PresenceData,
    is_idle: bool,
    current_client_id: u64,
    last_update_time: Option<Instant>,

    // For storing the last non-idle presence
    pre_idle_data: Option<PresenceData>,

    // Cache for Discord application details (client_id -> (name, icon_hash))
    client_cache: HashMap<u64, (String, String)>,
    
    app_registry: AppRegistry,
}

impl PresenceEngine {
    pub fn new(app_state: AppState) -> Self {
        Self {
            app_state,
            current_state: PresenceState::Disconnected,
            current_data: PresenceData::default(),
            is_idle: false,
            current_client_id: DEFAULT_CLIENT_ID,
            last_update_time: None,
            pre_idle_data: None,
            client_cache: HashMap::new(),
            app_registry: AppRegistry::new(),
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
                            let rules = self.app_state.get_app_rules().await;
                            let settings = self.app_state.get_settings().await;

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
                                // Find matching rule (cross-platform, stripping .exe)
                                let matched_rule = rules.iter().find(|r| {
                                    if !r.enabled {
                                        return false;
                                    }
                                    let rule_proc = r.process_name.strip_suffix(".exe").unwrap_or(&r.process_name).to_lowercase();
                                    let proc = process_name.strip_suffix(".exe").unwrap_or(&process_name).to_lowercase();
                                    rule_proc == proc
                                });

                                // Resolve Client ID based on process name
                                let app_info = self.app_registry.find_app(&process_name);
                                let client_id = app_info
                                    .map(|app| app.client_id.parse::<u64>().unwrap_or(DEFAULT_CLIENT_ID))
                                    .unwrap_or(DEFAULT_CLIENT_ID);
                                let default_app_name = app_info.map(|app| app.name.clone());

                                // Fetch application details from Discord if custom client ID and not cached
                                let (discord_name, discord_icon) = self.resolve_discord_details(client_id).await;

                                let (mut new_data, activity_app_name) = match matched_rule {
                                    Some(rule) => {
                                        let data = self.build_presence_from_rule(rule, &process_name, &window_title, discord_icon);
                                        (data, rule.display_name.clone())
                                    }
                                    None => {
                                        self.build_fallback_presence(&process_name, &window_title, discord_name, discord_icon, default_app_name)
                                    }
                                };

                                if client_id == DEFAULT_CLIENT_ID {
                                    self.format_presence_for_default_client(&mut new_data, &activity_app_name);
                                }

                                if !self.is_idle {
                                    // Start anti-flicker settle timer
                                    if let Some(task) = settle_task.take() {
                                        task.abort();
                                    }

                                    let delay = settings.settle_delay_seconds;
                                    let tx = settle_tx.clone();
                                    let data_clone = new_data.clone();

                                    settle_task = Some(tauri::async_runtime::spawn(async move {
                                        sleep(Duration::from_secs(delay)).await;
                                        let _ = tx.send((client_id, data_clone)).await;
                                    }));
                                } else {
                                    self.pre_idle_data = Some(new_data.clone());
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

                                    self.apply_presence(DEFAULT_CLIENT_ID, idle_data).await;
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
                            self.apply_presence(DEFAULT_CLIENT_ID, data).await;
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

    async fn resolve_discord_details(&mut self, client_id: u64) -> (Option<String>, Option<String>) {
        if client_id == DEFAULT_CLIENT_ID {
            return (None, None);
        }

        if !self.client_cache.contains_key(&client_id) {
            if let Some((name, icon_hash)) = fetch_discord_app_details(client_id).await {
                self.client_cache.insert(client_id, (name, icon_hash));
            } else {
                self.client_cache.insert(client_id, (String::new(), String::new()));
            }
        }

        let cached_details = self.client_cache.get(&client_id);
        let mut discord_icon = None;
        let mut discord_name = None;

        if let Some((name, icon_hash)) = cached_details {
            if !name.is_empty() {
                discord_name = Some(name.clone());
            }
            if !icon_hash.is_empty() {
                discord_icon = Some(format!("https://cdn.discordapp.com/app-icons/{}/{}.png?size=128", client_id, icon_hash));
            }
        }

        (discord_name, discord_icon)
    }

    fn build_presence_from_rule(
        &self,
        rule: &crate::models::types::AppRule,
        process_name: &str,
        window_title: &str,
        discord_icon: Option<String>,
    ) -> PresenceData {
        let (fmt_details, _) = if let Some(app) = self.app_registry.find_app(process_name) {
            format_app_presence(&app.category, &app.name, window_title)
        } else {
            format_app_presence("generic", &rule.display_name, window_title)
        };

        let file_name = fmt_details.strip_prefix("Editing ").unwrap_or(&fmt_details);

        let details = rule.details
            .replace("{file}", file_name)
            .replace("{title}", window_title);

        let state = rule.state
            .replace("{file}", file_name)
            .replace("{title}", window_title);

        let large_image = if rule.large_image == "auto" || rule.large_image == "default" || rule.large_image.trim().is_empty() {
            if let Some(app) = self.app_registry.find_app(process_name) {
                if let Some(ref icon_url) = app.icon_url {
                    get_discord_asset_key(icon_url)
                } else if let Some(ref icon_str) = app.icon {
                    get_discord_asset_key(icon_str)
                } else {
                    discord_icon.unwrap_or_else(|| "default".to_string())
                }
            } else {
                discord_icon.unwrap_or_else(|| "default".to_string())
            }
        } else {
            if rule.large_image.contains(':') || rule.large_image.starts_with("http") {
                get_discord_asset_key(&rule.large_image)
            } else {
                rule.large_image.clone()
            }
        };

        PresenceData {
            details,
            state,
            large_image,
            large_text: rule.display_name.clone(),
            timestamp: 0,
        }
    }

    fn build_fallback_presence(
        &self,
        process_name: &str,
        window_title: &str,
        discord_name: Option<String>,
        discord_icon: Option<String>,
        default_app_name: Option<String>,
    ) -> (PresenceData, String) {
        let activity_app_name = discord_name.or(default_app_name).unwrap_or_else(|| {
            let title = window_title.trim();
            if !title.is_empty() { title.to_string() } else { "Unknown app".to_string() }
        });

        let large_image = if let Some(app) = self.app_registry.find_app(process_name) {
            if let Some(ref icon_url) = app.icon_url {
                get_discord_asset_key(icon_url)
            } else if let Some(ref icon_str) = app.icon {
                get_discord_asset_key(icon_str)
            } else {
                discord_icon.unwrap_or_else(|| "default".to_string())
            }
        } else {
            discord_icon.unwrap_or_else(|| "default".to_string())
        };

        let (details, state) = if let Some(app) = self.app_registry.find_app(process_name) {
            format_app_presence(&app.category, &app.name, window_title)
        } else {
            format_app_presence("generic", &activity_app_name, window_title)
        };

        (
            PresenceData {
                details,
                state,
                large_image,
                large_text: activity_app_name.clone(),
                timestamp: 0,
            },
            activity_app_name,
        )
    }

    fn format_presence_for_default_client(&self, data: &mut PresenceData, app_name: &str) {
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

pub fn get_discord_asset_key(icon: &str) -> String {
    let icon_trimmed = icon.trim();
    let final_url = if icon_trimmed.starts_with("http://") || icon_trimmed.starts_with("https://") {
        let url = if icon_trimmed.ends_with(".svg") {
            format!("{}.png", icon_trimmed.trim_end_matches(".svg"))
        } else {
            icon_trimmed.to_string()
        };
        let url = url.replace(
            "raw.githubusercontent.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/refs/heads/main/",
            "cdn.jsdelivr.net/gh/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord@main/",
        );
        url.replace(
            "raw.githubusercontent.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/main/",
            "cdn.jsdelivr.net/gh/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord@main/",
        )
    } else if icon_trimmed.contains(':') {
        let parts: Vec<&str> = icon_trimmed.split(':').collect();
        if parts.len() == 2 {
            let collection = parts[0];
            let name = parts[1];
            let filename = format!("{}-{}.png", collection, name);
            format!(
                "https://cdn.jsdelivr.net/gh/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord@main/public/assets/icons/{}",
                filename
            )
        } else {
            parts.last().unwrap_or(&icon_trimmed).to_string()
        }
    } else {
        icon_trimmed.to_string()
    };

    println!("Original URL: {}", icon);
    println!("Final URL: {}", final_url);

    final_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_discord_asset_key_raw_github_to_jsdelivr() {
        let input = "https://raw.githubusercontent.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/main/public/assets/icons/simple-icons-zenbrowser.png";
        let expected = "https://cdn.jsdelivr.net/gh/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord@main/public/assets/icons/simple-icons-zenbrowser.png";
        assert_eq!(get_discord_asset_key(input), expected);
    }

    #[test]
    fn test_get_discord_asset_key_iconify_colon_format() {
        let input = "simple-icons:zenbrowser";
        let expected = "https://cdn.jsdelivr.net/gh/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord@main/public/assets/icons/simple-icons-zenbrowser.png";
        assert_eq!(get_discord_asset_key(input), expected);
    }
}

/// Helper function to spawn the engine task
pub fn start_engine(rx: mpsc::Receiver<EngineEvent>, app_state: AppState) {
    let engine = PresenceEngine::new(app_state);
    tauri::async_runtime::spawn(async move {
        engine.run(rx).await;
    });
}
