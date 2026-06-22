//! # Core Types Module
//!
//! Defines all shared data structures used throughout the Better Rich Presence app.
//! Every type that crosses the IPC boundary derives `Serialize` and `Deserialize`.
//! The priority system ensures that higher-priority sources (lower number) always
//! take precedence when multiple presence sources compete.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// PresenceSource — categorizes the origin of a presence update
// ---------------------------------------------------------------------------

/// Categorizes the origin of a presence update.
///
/// Priority is fixed: Game(0) > Manual(1) > Work(2) > Browser(3) > Idle(4).
/// A source with a **lower** priority number takes precedence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresenceSource {
    Game,
    Manual,
    Work,
    Browser,
    Idle,
}

impl PresenceSource {
    /// Returns the numeric priority. Lower number = higher precedence.
    pub fn priority(&self) -> u32 {
        match self {
            Self::Game => 0,
            Self::Manual => 1,
            Self::Work => 2,
            Self::Browser => 3,
            Self::Idle => 4,
        }
    }
}

impl Default for PresenceSource {
    fn default() -> Self {
        Self::Idle
    }
}

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
    /// What type of activity this represents.
    pub source: PresenceSource,
    /// Numeric priority override (defaults to `source.priority()`).
    pub priority: u32,
    /// Whether this rule is currently active.
    pub enabled: bool,
}

impl AppRule {
    /// Convenience constructor that derives priority from the source.
    pub fn new(
        process_name: impl Into<String>,
        display_name: impl Into<String>,
        details: impl Into<String>,
        state: impl Into<String>,
        large_image: impl Into<String>,
        source: PresenceSource,
    ) -> Self {
        let source_priority = source.priority();
        Self {
            process_name: process_name.into(),
            display_name: display_name.into(),
            details: details.into(),
            state: state.into(),
            large_image: large_image.into(),
            source,
            priority: source_priority,
            enabled: true,
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
    /// The source category that produced this presence.
    pub source: PresenceSource,
    /// Unix epoch timestamp (seconds) for the elapsed timer.
    pub timestamp: i64,
}

impl Default for PresenceData {
    fn default() -> Self {
        Self {
            details: String::from("Usando o computador"),
            state: String::from("Idle"),
            large_image: String::from("default"),
            large_text: String::from("Better Rich Presence"),
            source: PresenceSource::Idle,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
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
    },
    /// The idle state changed (transition only, not periodic).
    IdleChanged {
        idle: bool,
        idle_minutes: u32,
    },
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
    SetActivity(PresenceData),
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
        }
    }
}
