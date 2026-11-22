//! Pure domain rules for Claude Code Rich Presence integration.
//!
//! Parses Claude Code HTTP hook payloads, tracks concurrent sessions, computes
//! privacy-safe activity labels, and enforces session auto-expiry.
//! Depends on `std` and `serde` only.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Display name shown as `details` and `large_text` on Discord.
pub const CLAUDE_CODE_DISPLAY_NAME: &str = "Claude Code";

/// Catalog id in `apps.json` whose `icon_url` resolves to `claude.png`.
pub const CLAUDE_CODE_CATALOG_ID: &str = "claude_code";

/// Default localhost port for the Claude Code HTTP hook receiver.
pub const DEFAULT_CLAUDE_CODE_PORT: u16 = 47892;

/// HTTP path accepting Claude Code hook POST payloads.
pub const CLAUDE_CODE_HOOK_PATH: &str = "/hooks/claude-code";

/// Default inactivity timeout (minutes) before a waiting session auto-expires.
pub const DEFAULT_CLAUDE_CODE_TIMEOUT_MINUTES: u32 = 5;

/// Minimum safety timeout (seconds) for an actively executing turn (e.g. long builds).
const MIN_ACTIVE_TIMEOUT_SECS: i64 = 900;

/// Discord's maximum length in bytes for `details` and `state` strings.
const MAX_DISCORD_TEXT_BYTES: usize = 128;

/// Claude Code lifecycle events subscribed via HTTP hooks.
///
/// `SessionStart` is intentionally omitted because Claude Code does not dispatch
/// `type: "http"` hooks for `SessionStart`. `PermissionRequest` is intentionally
/// omitted so Better RPC can never gate or fail-close permission prompts.
pub const CLAUDE_CODE_HOOK_EVENTS: &[&str] = &[
    "UserPromptSubmit",
    "PreToolUse",
    "PostToolUse",
    "PostToolUseFailure",
    "Notification",
    "Stop",
    "StopFailure",
    "SubagentStart",
    "SubagentStop",
    "CwdChanged",
    "SessionEnd",
];

/// Hook installation state of `~/.claude/settings.json`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaudeHookInstallState {
    NotInstalled,
    Installed,
    Outdated,
}

/// Subset of tool arguments inspected to build a privacy-safe activity line.
///
/// Deliberately omits raw shell commands, file contents, and prompt text.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct ClaudeToolInput {
    #[serde(default)]
    pub file_path: Option<String>,
    #[serde(default)]
    pub notebook_path: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
}

/// JSON payload POSTed by Claude Code HTTP hooks.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct ClaudeHookPayload {
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub hook_event_name: String,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub tool_name: Option<String>,
    #[serde(default)]
    pub tool_input: Option<ClaudeToolInput>,
    #[serde(default)]
    pub notification_type: Option<String>,
    #[serde(default)]
    pub agent_type: Option<String>,
}

/// High-level activity state of a Claude Code session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaudeActivity {
    Ready,
    Thinking,
    EditingFile(Option<String>),
    ReadingFile(Option<String>),
    RunningCommand,
    Searching,
    Researching,
    Planning,
    RunningSubagent(Option<String>),
    UsingMcp(String),
    UsingTool(String),
    WaitingForApproval,
    WaitingForInput,
}

impl ClaudeActivity {
    /// Whether the session is waiting on the user rather than actively executing.
    pub fn is_waiting(&self) -> bool {
        matches!(
            self,
            Self::Ready | Self::WaitingForInput | Self::WaitingForApproval
        )
    }
}

/// Live state tracked for a single Claude Code session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaudeSession {
    pub session_id: String,
    pub project_name: Option<String>,
    pub activity: ClaudeActivity,
    pub started_at: i64,
    pub last_event_at: i64,
}

impl ClaudeSession {
    /// Formats the `state` line for Discord Rich Presence.
    pub fn state_text(&self) -> String {
        format_claude_activity(&self.activity, self.project_name.as_deref())
    }
}

/// Tracks concurrent Claude Code sessions and determines which session is active.
#[derive(Debug, Default)]
pub struct ClaudeSessionTracker {
    sessions: BTreeMap<String, ClaudeSession>,
}

impl ClaudeSessionTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Applies an incoming hook event at Unix timestamp `now` (seconds).
    /// Returns `true` if session state changed (or a known event updated a session).
    pub fn apply_event(&mut self, payload: &ClaudeHookPayload, now: i64) -> bool {
        let event = payload.hook_event_name.trim();
        if event.is_empty() {
            return false;
        }

        let session_id = normalized_session_id(&payload.session_id);

        if event == "SessionEnd" {
            return self.sessions.remove(&session_id).is_some();
        }

        let project = payload.cwd.as_deref().and_then(extract_project_name);

        match event {
            "SessionStart" => {
                self.upsert(&session_id, project, Some(ClaudeActivity::Ready), now);
                true
            }
            "UserPromptSubmit" => {
                self.upsert(&session_id, project, Some(ClaudeActivity::Thinking), now);
                true
            }
            "PreToolUse" => {
                let activity = activity_from_tool(
                    payload.tool_name.as_deref(),
                    payload.tool_input.as_ref(),
                );
                self.upsert(&session_id, project, Some(activity), now);
                true
            }
            "PostToolUse" | "PostToolUseFailure" => {
                // Keep the visible tool label while refreshing the session heartbeat
                // so fast edits are not overwritten before Discord's 15s update cadence.
                if let Some(existing) = self.sessions.get_mut(&session_id) {
                    if project.is_some() {
                        existing.project_name = project;
                    }
                    existing.last_event_at = now;
                } else {
                    let activity = activity_from_tool(
                        payload.tool_name.as_deref(),
                        payload.tool_input.as_ref(),
                    );
                    self.upsert(&session_id, project, Some(activity), now);
                }
                true
            }
            "Notification" => {
                let activity = match payload.notification_type.as_deref().map(str::trim) {
                    Some("permission_prompt")
                    | Some("elicitation_dialog")
                    | Some("elicitation_url_dialog") => Some(ClaudeActivity::WaitingForApproval),
                    Some("idle_prompt") | Some("agent_needs_input") => {
                        Some(ClaudeActivity::WaitingForInput)
                    }
                    _ => None,
                };
                if let Some(act) = activity {
                    self.upsert(&session_id, project, Some(act), now);
                } else if let Some(existing) = self.sessions.get_mut(&session_id) {
                    if project.is_some() {
                        existing.project_name = project;
                    }
                    existing.last_event_at = now;
                } else {
                    self.upsert(&session_id, project, Some(ClaudeActivity::Ready), now);
                }
                true
            }
            "Stop" | "StopFailure" => {
                self.upsert(
                    &session_id,
                    project,
                    Some(ClaudeActivity::WaitingForInput),
                    now,
                );
                true
            }
            "SubagentStart" => {
                let agent = payload
                    .agent_type
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(ToString::to_string);
                self.upsert(
                    &session_id,
                    project,
                    Some(ClaudeActivity::RunningSubagent(agent)),
                    now,
                );
                true
            }
            "SubagentStop" => {
                self.upsert(&session_id, project, Some(ClaudeActivity::Thinking), now);
                true
            }
            "CwdChanged" => {
                if let Some(existing) = self.sessions.get_mut(&session_id) {
                    if project.is_some() {
                        existing.project_name = project;
                    }
                    existing.last_event_at = now;
                } else {
                    self.upsert(&session_id, project, Some(ClaudeActivity::Ready), now);
                }
                true
            }
            _ => false,
        }
    }

    fn upsert(
        &mut self,
        session_id: &str,
        project: Option<String>,
        activity: Option<ClaudeActivity>,
        now: i64,
    ) {
        if let Some(existing) = self.sessions.get_mut(session_id) {
            if project.is_some() {
                existing.project_name = project;
            }
            if let Some(act) = activity {
                existing.activity = act;
            }
            existing.last_event_at = now;
        } else {
            self.sessions.insert(
                session_id.to_string(),
                ClaudeSession {
                    session_id: session_id.to_string(),
                    project_name: project,
                    activity: activity.unwrap_or(ClaudeActivity::Ready),
                    started_at: now,
                    last_event_at: now,
                },
            );
        }
    }

    /// Removes sessions that have been inactive longer than their timeout.
    /// Returns `true` if at least one session was removed.
    pub fn prune_expired(&mut self, now: i64, waiting_timeout_secs: i64) -> bool {
        let waiting_ttl = waiting_timeout_secs.max(60);
        let active_ttl = waiting_ttl.saturating_mul(2).max(MIN_ACTIVE_TIMEOUT_SECS);
        let before = self.sessions.len();

        self.sessions.retain(|_, session| {
            let elapsed = now.saturating_sub(session.last_event_at);
            let ttl = if session.activity.is_waiting() {
                waiting_ttl
            } else {
                active_ttl
            };
            elapsed < ttl
        });

        self.sessions.len() != before
    }

    /// Returns the most recently active Claude Code session, if any.
    pub fn active_session(&self) -> Option<&ClaudeSession> {
        self.sessions
            .values()
            .max_by(|a, b| {
                a.last_event_at
                    .cmp(&b.last_event_at)
                    .then_with(|| a.session_id.cmp(&b.session_id))
            })
    }

    /// Number of currently tracked unexpired sessions.
    pub fn active_count(&self) -> usize {
        self.sessions.len()
    }

    /// Clears all tracked sessions (for example when disconnecting hooks).
    pub fn clear(&mut self) -> bool {
        let had = !self.sessions.is_empty();
        self.sessions.clear();
        had
    }
}

fn normalized_session_id(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        "default".to_string()
    } else {
        trimmed.to_string()
    }
}

/// Maps a Claude Code tool call into a privacy-safe [`ClaudeActivity`].
pub fn activity_from_tool(
    tool_name: Option<&str>,
    tool_input: Option<&ClaudeToolInput>,
) -> ClaudeActivity {
    let name = tool_name.map(str::trim).unwrap_or("");
    let file_name = tool_input.and_then(|input| {
        input
            .file_path
            .as_deref()
            .or(input.notebook_path.as_deref())
            .or(input.path.as_deref())
            .and_then(extract_file_name)
    });

    match name {
        "Edit" | "MultiEdit" | "Write" | "NotebookEdit" => ClaudeActivity::EditingFile(file_name),
        "Read" => ClaudeActivity::ReadingFile(file_name),
        "Bash" | "PowerShell" => ClaudeActivity::RunningCommand,
        "Grep" | "Glob" | "LS" | "ToolSearch" => ClaudeActivity::Searching,
        "WebFetch" | "WebSearch" => ClaudeActivity::Researching,
        "TodoWrite" | "TaskCreate" | "TaskUpdate" | "EnterPlanMode" | "ExitPlanMode" => {
            ClaudeActivity::Planning
        }
        "Agent" | "Task" => ClaudeActivity::RunningSubagent(None),
        "AskUserQuestion" => ClaudeActivity::WaitingForInput,
        "" => ClaudeActivity::Thinking,
        other => {
            if let Some(server) = parse_mcp_server_name(other) {
                ClaudeActivity::UsingMcp(server)
            } else {
                ClaudeActivity::UsingTool(other.to_string())
            }
        }
    }
}

/// Extracts the MCP server name from `mcp__<server>__<tool>` or
/// `mcp__plugin_<plugin>_<server>__<tool>`.
fn parse_mcp_server_name(tool_name: &str) -> Option<String> {
    let rest = tool_name.strip_prefix("mcp__")?;
    let (server, _tool) = rest.split_once("__")?;
    let server = server.trim();
    if server.is_empty() {
        return None;
    }
    if let Some(plugin_scoped) = server.strip_prefix("plugin_") {
        if let Some((_, short_server)) = plugin_scoped.rsplit_once('_') {
            if !short_server.is_empty() {
                return Some(short_server.to_string());
            }
        }
    }
    Some(server.to_string())
}

/// Extracts the project folder name from a Unix or Windows `cwd` path.
/// Returns `None` for empty strings or filesystem roots (`/` or `C:\`).
pub fn extract_project_name(cwd: &str) -> Option<String> {
    let trimmed = cwd.trim();
    if trimmed.is_empty() {
        return None;
    }

    let last = trimmed
        .split(['/', '\\'])
        .filter(|seg| !seg.trim().is_empty())
        .next_back()?
        .trim();

    // Ignore bare Windows drive designators like "C:".
    if last.len() == 2 && last.as_bytes()[0].is_ascii_alphabetic() && last.as_bytes()[1] == b':' {
        return None;
    }

    Some(last.to_string())
}

/// Extracts only the final filename from a Unix or Windows file path.
pub fn extract_file_name(path: &str) -> Option<String> {
    extract_project_name(path)
}

/// Formats a Claude Code activity and optional project into a Discord `state` line,
/// capped safely at Discord's 128-byte limit.
pub fn format_claude_activity(activity: &ClaudeActivity, project: Option<&str>) -> String {
    let action = match activity {
        ClaudeActivity::Ready => "Working".to_string(),
        ClaudeActivity::Thinking => "Thinking".to_string(),
        ClaudeActivity::EditingFile(Some(file)) => format!("Editing {file}"),
        ClaudeActivity::EditingFile(None) => "Editing a file".to_string(),
        ClaudeActivity::ReadingFile(Some(file)) => format!("Reading {file}"),
        ClaudeActivity::ReadingFile(None) => "Reading a file".to_string(),
        ClaudeActivity::RunningCommand => "Running command".to_string(),
        ClaudeActivity::Searching => "Searching".to_string(),
        ClaudeActivity::Researching => "Researching".to_string(),
        ClaudeActivity::Planning => "Planning".to_string(),
        ClaudeActivity::RunningSubagent(Some(agent)) => format!("Running {agent} agent"),
        ClaudeActivity::RunningSubagent(None) => "Running subagent".to_string(),
        ClaudeActivity::UsingMcp(server) => format!("Using {server}"),
        ClaudeActivity::UsingTool(tool) => format!("Using {tool}"),
        ClaudeActivity::WaitingForApproval => "Waiting for approval".to_string(),
        ClaudeActivity::WaitingForInput => "Waiting for input".to_string(),
    };

    let project = project.map(str::trim).filter(|s| !s.is_empty());
    let full = match project {
        Some(project) => format!("{action} in {project}"),
        None => action,
    };

    truncate_discord_text(&full, MAX_DISCORD_TEXT_BYTES)
}

fn truncate_discord_text(text: &str, max_bytes: usize) -> String {
    if text.len() <= max_bytes {
        return text.to_string();
    }
    let mut end = max_bytes.saturating_sub(3);
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &text[..end])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payload(event: &str, session_id: &str, cwd: Option<&str>) -> ClaudeHookPayload {
        ClaudeHookPayload {
            session_id: session_id.to_string(),
            hook_event_name: event.to_string(),
            cwd: cwd.map(ToString::to_string),
            ..ClaudeHookPayload::default()
        }
    }

    #[test]
    fn extracts_project_and_file_names_across_platforms() {
        assert_eq!(
            extract_project_name("/home/dev/Better-Rich-Presence-For-Discord/"),
            Some("Better-Rich-Presence-For-Discord".to_string())
        );
        assert_eq!(
            extract_project_name(r"C:\Users\Joao\Projects\my-app\"),
            Some("my-app".to_string())
        );
        assert_eq!(extract_project_name("/"), None);
        assert_eq!(extract_project_name(r"C:\"), None);
        assert_eq!(extract_project_name("   "), None);

        assert_eq!(
            extract_file_name(r"C:\Users\Joao\repo\src\engine.rs"),
            Some("engine.rs".to_string())
        );
        assert_eq!(
            extract_file_name("/home/dev/app/Cargo.toml"),
            Some("Cargo.toml".to_string())
        );
    }

    #[test]
    fn formats_tool_activities_safely_without_leaking_paths_or_commands() {
        let edit = activity_from_tool(
            Some("Edit"),
            Some(&ClaudeToolInput {
                file_path: Some("/secret/user/project/src/engine.rs".to_string()),
                ..Default::default()
            }),
        );
        assert_eq!(
            format_claude_activity(&edit, Some("Better-RPC")),
            "Editing engine.rs in Better-RPC"
        );

        let read = activity_from_tool(
            Some("Read"),
            Some(&ClaudeToolInput {
                file_path: Some(r"C:\Users\alice\repo\Cargo.toml".to_string()),
                ..Default::default()
            }),
        );
        assert_eq!(
            format_claude_activity(&read, Some("repo")),
            "Reading Cargo.toml in repo"
        );

        let bash = activity_from_tool(Some("Bash"), None);
        assert_eq!(
            format_claude_activity(&bash, Some("repo")),
            "Running command in repo"
        );

        let ps = activity_from_tool(Some("PowerShell"), None);
        assert_eq!(
            format_claude_activity(&ps, Some("repo")),
            "Running command in repo"
        );

        let mcp = activity_from_tool(Some("mcp__github__search_repositories"), None);
        assert_eq!(
            format_claude_activity(&mcp, Some("repo")),
            "Using github in repo"
        );

        let plugin_mcp = activity_from_tool(Some("mcp__plugin_my-plugin_db__query"), None);
        assert_eq!(
            format_claude_activity(&plugin_mcp, Some("repo")),
            "Using db in repo"
        );
    }

    #[test]
    fn tracker_handles_full_turn_lifecycle_and_preserves_edit_across_post_tool() {
        let mut tracker = ClaudeSessionTracker::new();

        assert!(tracker.apply_event(&payload("UserPromptSubmit", "s1", Some("/work/app")), 100));
        let s = tracker.active_session().unwrap();
        assert_eq!(s.started_at, 100);
        assert_eq!(s.state_text(), "Thinking in app");

        let mut pre_edit = payload("PreToolUse", "s1", Some("/work/app"));
        pre_edit.tool_name = Some("Edit".to_string());
        pre_edit.tool_input = Some(ClaudeToolInput {
            file_path: Some("/work/app/src/main.rs".to_string()),
            ..Default::default()
        });
        assert!(tracker.apply_event(&pre_edit, 105));
        assert_eq!(
            tracker.active_session().unwrap().state_text(),
            "Editing main.rs in app"
        );

        // PostToolUse refreshes last_event_at without erasing the edited filename.
        let mut post_edit = payload("PostToolUse", "s1", Some("/work/app"));
        post_edit.tool_name = Some("Edit".to_string());
        assert!(tracker.apply_event(&post_edit, 106));
        let s = tracker.active_session().unwrap();
        assert_eq!(s.started_at, 100);
        assert_eq!(s.last_event_at, 106);
        assert_eq!(s.state_text(), "Editing main.rs in app");

        // Permission notification switches to waiting for approval.
        let mut notif = payload("Notification", "s1", Some("/work/app"));
        notif.notification_type = Some("permission_prompt".to_string());
        assert!(tracker.apply_event(&notif, 112));
        assert_eq!(
            tracker.active_session().unwrap().state_text(),
            "Waiting for approval in app"
        );

        // Stop transitions to waiting for input while keeping started_at.
        assert!(tracker.apply_event(&payload("Stop", "s1", Some("/work/app")), 120));
        let s = tracker.active_session().unwrap();
        assert_eq!(s.started_at, 100);
        assert_eq!(s.state_text(), "Waiting for input in app");

        // CwdChanged updates project name without losing start time or state.
        assert!(tracker.apply_event(&payload("CwdChanged", "s1", Some("/work/other")), 125));
        let s = tracker.active_session().unwrap();
        assert_eq!(s.started_at, 100);
        assert_eq!(s.state_text(), "Waiting for input in other");

        // SessionEnd clears the session.
        assert!(tracker.apply_event(&payload("SessionEnd", "s1", None), 130));
        assert!(tracker.active_session().is_none());
    }

    #[test]
    fn tracker_switches_between_concurrent_sessions_and_restores_remaining() {
        let mut tracker = ClaudeSessionTracker::new();
        tracker.apply_event(&payload("UserPromptSubmit", "s1", Some("/work/proj-a")), 10);
        tracker.apply_event(&payload("UserPromptSubmit", "s2", Some("/work/proj-b")), 20);

        assert_eq!(tracker.active_count(), 2);
        assert_eq!(tracker.active_session().unwrap().session_id, "s2");

        // Ending s2 falls back to s1.
        tracker.apply_event(&payload("SessionEnd", "s2", None), 25);
        assert_eq!(tracker.active_count(), 1);
        assert_eq!(tracker.active_session().unwrap().session_id, "s1");
    }

    #[test]
    fn tracker_prunes_waiting_sessions_faster_than_active_turns() {
        let mut tracker = ClaudeSessionTracker::new();
        tracker.apply_event(&payload("Stop", "waiting", Some("/work/a")), 1000);
        tracker.apply_event(&payload("UserPromptSubmit", "running", Some("/work/b")), 1000);

        // After 5 minutes (300s), the waiting session expires, active turn survives.
        assert!(tracker.prune_expired(1300, 300));
        assert_eq!(tracker.active_count(), 1);
        assert_eq!(tracker.active_session().unwrap().session_id, "running");

        // Still alive at 10 minutes (600s) for long builds/tests.
        assert!(!tracker.prune_expired(1600, 300));

        // Expires after 15 minutes (900s) safety floor.
        assert!(tracker.prune_expired(1900, 300));
        assert_eq!(tracker.active_count(), 0);
    }

    #[test]
    fn formats_long_state_within_discord_limit_on_utf8_boundary() {
        let long_project = "é".repeat(100);
        let formatted = format_claude_activity(&ClaudeActivity::Thinking, Some(&long_project));
        assert!(formatted.len() <= MAX_DISCORD_TEXT_BYTES);
        assert!(formatted.ends_with("..."));
    }

    #[test]
    fn unknown_events_are_ignored() {
        let mut tracker = ClaudeSessionTracker::new();
        assert!(!tracker.apply_event(&payload("UnknownEvent", "s1", Some("/work/a")), 100));
        assert_eq!(tracker.active_count(), 0);
    }
}
