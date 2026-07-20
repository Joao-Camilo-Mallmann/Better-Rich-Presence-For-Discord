//! # Core Types Module
//!
//! Defines all shared data structures used throughout the Better Rich Presence app.
//! Every type that crosses the IPC boundary derives `Serialize` and `Deserialize`.
//! The priority system ensures that higher-priority sources (lower number) always
//! take precedence when multiple presence sources compete.

use serde::{Deserialize, Serialize};

/// The default Discord Application ID for Better Rich Presence.
pub const DEFAULT_CLIENT_ID: u64 = 1517170930764480552;

// ---------------------------------------------------------------------------
// PresenceState — lifecycle state of the connection & engine
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// PresenceState — lifecycle state of the connection & engine
// ---------------------------------------------------------------------------

/// Lifecycle state of the Discord connection and presence engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresenceState {
    /// Not connected to Discord.
    Disconnected,
    /// Connected to IPC but waiting for Discord Ready event.
    WaitingDiscord,
    /// Fully connected and operational.
    Connected,
    /// Presence paused because a game with its own Rich Presence was detected.
    PausedByGame,
    /// Currently pushing an activity update.
    Updating,
}

impl Default for PresenceState {
    fn default() -> Self {
        Self::Disconnected
    }
}

// ---------------------------------------------------------------------------
// AppRule — a user-defined or preset rule that maps a process to a presence
// ---------------------------------------------------------------------------

/// A rule that maps a running process to a Discord Rich Presence configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRule {
    /// The executable file name to match (e.g., `"code.exe"`).
    pub process_name: String,
    /// Human-readable name shown in the UI (e.g., `"Visual Studio Code"`).
    pub display_name: String,
    /// The "details" line shown on Discord.
    pub details: String,
    /// The "state" line shown on Discord.
    pub state: String,
    /// Asset key for the large image (must match a Discord app asset).
    pub large_image: String,
    /// Whether this rule is currently active.
    pub enabled: bool,
    /// Custom Discord Client ID override for this rule.
    #[serde(default)]
    pub client_id: Option<String>,
}

impl AppRule {
    /// Convenience constructor.
    pub fn new(
        process_name: impl Into<String>,
        display_name: impl Into<String>,
        details: impl Into<String>,
        state: impl Into<String>,
        large_image: impl Into<String>,
    ) -> Self {
        Self {
            process_name: process_name.into(),
            display_name: display_name.into(),
            details: details.into(),
            state: state.into(),
            large_image: large_image.into(),
            enabled: true,
            client_id: None,
        }
    }
}

// ---------------------------------------------------------------------------
// PresenceData — the payload sent to Discord
// ---------------------------------------------------------------------------

/// The data payload that describes the current Rich Presence shown on Discord.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresenceData {
    /// First line of the presence (e.g., "Programando no VSCode").
    pub details: String,
    /// Second line of the presence (e.g., "Desenvolvendo").
    pub state: String,
    /// Asset key for the large image.
    pub large_image: String,
    /// Tooltip text for the large image.
    pub large_text: String,
    /// Unix epoch timestamp (seconds) for the elapsed timer.
    pub timestamp: i64,
}

impl Default for PresenceData {
    fn default() -> Self {
        Self {
            details: String::from("Navegando no PC"),
            state: String::from("Só de boa..."),
            large_image: String::from("default"),
            large_text: String::from("Better Rich Presence"),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

// ---------------------------------------------------------------------------
// DiscordUser — Discord user profile info
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub avatar_url: String,
}

// ---------------------------------------------------------------------------
// EngineEvent — events flowing from watchers into the engine
// ---------------------------------------------------------------------------

/// Events produced by watchers and consumed by the presence engine.
#[derive(Debug, Clone)]
pub enum EngineEvent {
    /// The foreground window changed to a new process.
    WindowChanged {
        process_name: String,
        window_title: String,
        is_prioritized: bool,
        foreground_app: Option<String>,
    },
    /// Command to clear the presence.
    ClearPresence,
    /// The idle state changed (transition only, not periodic).
    IdleChanged { idle: bool, idle_minutes: u32 },
    /// A manual profile was set by the user.
    ManualProfile(PresenceData),
    /// The application is shutting down.
    Shutdown,
}

// ---------------------------------------------------------------------------
// EngineCommand — commands sent to the Discord RPC thread
// ---------------------------------------------------------------------------

/// Commands sent from the engine to the dedicated Discord RPC thread.
#[derive(Debug, Clone)]
pub enum EngineCommand {
    /// Set/update the current activity on Discord.
    SetActivity { client_id: u64, data: PresenceData },
    /// Clear the current activity.
    ClearActivity,
    /// Gracefully disconnect from Discord IPC.
    Disconnect,
}

// ---------------------------------------------------------------------------
// ConnectionInfo — snapshot of Discord connection health
// ---------------------------------------------------------------------------

/// Snapshot of the Discord IPC connection health, emitted to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    /// Whether the IPC pipe is connected.
    pub connected: bool,
    /// Current lifecycle state.
    pub state: PresenceState,
    /// Last error message, if any.
    pub last_error: Option<String>,
    /// How many reconnect attempts have been made since last success.
    pub reconnect_attempts: u32,
}

impl Default for ConnectionInfo {
    fn default() -> Self {
        Self {
            connected: false,
            state: PresenceState::Disconnected,
            last_error: None,
            reconnect_attempts: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// PriorityInfo — priority mode feedback sent to UI
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityInfo {
    pub active: bool,
    pub prioritized_app: Option<String>,
    pub foreground_app: Option<String>,
}

impl Default for PriorityInfo {
    fn default() -> Self {
        Self {
            active: false,
            prioritized_app: None,
            foreground_app: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Settings — user-configurable application settings
// ---------------------------------------------------------------------------

/// User-configurable application settings, persisted via `tauri-plugin-store`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Whether idle detection is enabled.
    pub idle_enabled: bool,
    /// Minutes of inactivity before switching to Idle presence.
    pub idle_threshold_minutes: u32,
    /// Custom message shown when idle.
    pub idle_message: String,
    /// Whether the app should start with the OS.
    pub autostart_enabled: bool,
    /// Minimum seconds between processing window-change events (debounce).
    pub debounce_seconds: u64,
    /// Seconds to wait before committing a window change (anti-flicker).
    pub settle_delay_seconds: u64,
    /// Whether priority mode is enabled (scans background tasks for higher priority rules).
    pub priority_mode_enabled: bool,
    /// Master switch to enable/disable the rich presence entirely.
    pub global_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            idle_enabled: true,
            idle_threshold_minutes: 5,
            idle_message: String::from("Ausente"),
            autostart_enabled: false,
            debounce_seconds: 2,
            settle_delay_seconds: 3,
            priority_mode_enabled: true,
            global_enabled: true,
        }
    }
}

// ---------------------------------------------------------------------------
// AppError — custom error type returned to the frontend
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    Store(String),
    Serialization(String),
    System(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Store(msg) => write!(f, "Erro de armazenamento: {}", msg),
            Self::Serialization(msg) => write!(f, "Erro de serialização: {}", msg),
            Self::System(msg) => write!(f, "Erro do sistema: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::System(err.to_string())
    }
}

// ---------------------------------------------------------------------------
// AppStateInner — raw shared state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct AppStateInner {
    pub app_rules: Vec<AppRule>,
    pub current_presence: Option<PresenceData>,
    pub presence_state: PresenceState,
    pub connection_info: ConnectionInfo,
    pub settings: Settings,
    pub current_client_id: u64,
    pub discord_user: Option<DiscordUser>,
}

// ---------------------------------------------------------------------------
// AppState — unified application state manager
// ---------------------------------------------------------------------------

use crate::services::discord::DiscordHandle;
use std::sync::Arc;
use tauri_plugin_store::StoreExt;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<RwLock<AppStateInner>>,
    pub app_handle: tauri::AppHandle,
    pub discord_handle: DiscordHandle,
}

impl AppState {
    pub fn new(
        app_handle: tauri::AppHandle,
        discord_handle: DiscordHandle,
        app_rules: Vec<AppRule>,
        settings: Settings,
    ) -> Self {
        let inner = AppStateInner {
            app_rules,
            current_presence: None,
            presence_state: PresenceState::Disconnected,
            connection_info: ConnectionInfo::default(),
            settings,
            current_client_id: DEFAULT_CLIENT_ID,
            discord_user: None,
        };
        Self {
            inner: Arc::new(RwLock::new(inner)),
            app_handle,
            discord_handle,
        }
    }

    // App Rules Methods
    pub async fn get_app_rules(&self) -> Vec<AppRule> {
        self.inner.read().await.app_rules.clone()
    }

    pub async fn update_app_rule(&self, rule: AppRule) -> Result<(), AppError> {
        let mut inner = self.inner.write().await;
        if let Some(pos) = inner
            .app_rules
            .iter()
            .position(|r| r.process_name == rule.process_name)
        {
            inner.app_rules[pos] = rule;
            self.save_rules_to_store(&inner.app_rules)?;
        }
        Ok(())
    }

    pub async fn add_app_rule(&self, rule: AppRule) -> Result<(), AppError> {
        let mut inner = self.inner.write().await;
        if !inner
            .app_rules
            .iter()
            .any(|r| r.process_name == rule.process_name)
        {
            inner.app_rules.push(rule);
            self.save_rules_to_store(&inner.app_rules)?;
        }
        Ok(())
    }

    pub async fn delete_app_rule(&self, process_name: &str) -> Result<(), AppError> {
        let mut inner = self.inner.write().await;
        inner.app_rules.retain(|r| r.process_name != process_name);
        self.save_rules_to_store(&inner.app_rules)?;
        Ok(())
    }

    pub async fn reset_app_rules_to_defaults(&self) -> Result<(), AppError> {
        let mut inner = self.inner.write().await;
        inner.app_rules = crate::models::presets::default_app_rules();
        self.save_rules_to_store(&inner.app_rules)?;
        Ok(())
    }

    pub async fn reorder_app_rules(
        &self,
        process_names_order: Vec<String>,
    ) -> Result<(), AppError> {
        let mut inner = self.inner.write().await;

        let mut new_rules = Vec::new();
        // First add rules in the new order
        for name in &process_names_order {
            if let Some(rule) = inner.app_rules.iter().find(|r| &r.process_name == name) {
                new_rules.push(rule.clone());
            }
        }

        // Then append any rules that were missing from the order array
        for rule in &inner.app_rules {
            if !process_names_order.contains(&rule.process_name) {
                new_rules.push(rule.clone());
            }
        }

        inner.app_rules = new_rules;
        self.save_rules_to_store(&inner.app_rules)?;
        Ok(())
    }

    fn save_rules_to_store(&self, rules: &[AppRule]) -> Result<(), AppError> {
        let store = self
            .app_handle
            .store("rules.json")
            .map_err(|e| AppError::Store(e.to_string()))?;
        store.set("app_rules", serde_json::to_value(rules)?);
        store.save().map_err(|e| AppError::Store(e.to_string()))?;
        Ok(())
    }

    // Settings Methods
    pub async fn get_settings(&self) -> Settings {
        self.inner.read().await.settings.clone()
    }

    pub async fn update_settings(&self, settings: Settings) -> Result<(), AppError> {
        let mut inner = self.inner.write().await;
        let was_enabled = inner.settings.global_enabled;
        inner.settings = settings.clone();

        let store = self
            .app_handle
            .store("settings.json")
            .map_err(|e| AppError::Store(e.to_string()))?;
        store.set("config", serde_json::to_value(&settings)?);
        store.save().map_err(|e| AppError::Store(e.to_string()))?;

        if was_enabled && !settings.global_enabled {
            self.discord_handle.send(EngineCommand::ClearActivity);
        } else if !was_enabled && settings.global_enabled {
            if let Some(presence) = &inner.current_presence {
                self.discord_handle.send(EngineCommand::SetActivity {
                    client_id: inner.current_client_id,
                    data: presence.clone(),
                });
            }
        }

        Ok(())
    }

    // State Queries
    pub async fn get_current_presence(&self) -> Option<PresenceData> {
        self.inner.read().await.current_presence.clone()
    }

    pub async fn get_presence_state(&self) -> PresenceState {
        self.inner.read().await.presence_state
    }

    pub async fn get_discord_user(&self) -> Option<DiscordUser> {
        self.inner.read().await.discord_user.clone()
    }

    pub async fn get_connection_status(&self) -> ConnectionInfo {
        self.inner.read().await.connection_info.clone()
    }

    // Engine Helpers
    pub async fn update_connection_status(
        &self,
        is_connected: bool,
    ) -> (bool, bool, PresenceState) {
        let mut inner = self.inner.write().await;
        let mut conn_changed = false;
        let mut state_changed = false;

        if inner.connection_info.connected != is_connected {
            inner.connection_info.connected = is_connected;
            inner.connection_info.state = if is_connected {
                PresenceState::Connected
            } else {
                PresenceState::Disconnected
            };
            conn_changed = true;
        }

        let new_state = inner.connection_info.state;
        if inner.presence_state != new_state {
            inner.presence_state = new_state;
            state_changed = true;
        }

        (conn_changed, state_changed, new_state)
    }

    pub async fn set_presence_state(&self, presence_state: PresenceState) -> bool {
        let mut inner = self.inner.write().await;
        if inner.presence_state != presence_state {
            inner.presence_state = presence_state;
            inner.connection_info.state = presence_state;
            true
        } else {
            false
        }
    }

    pub async fn update_presence_data(&self, client_id: u64, new_data: PresenceData) -> bool {
        let mut inner = self.inner.write().await;
        let mut changed = false;

        if inner.current_presence.as_ref() != Some(&new_data)
            || inner.current_client_id != client_id
        {
            inner.current_presence = Some(new_data.clone());
            inner.current_client_id = client_id;
            changed = true;
        }

        changed
    }
}
