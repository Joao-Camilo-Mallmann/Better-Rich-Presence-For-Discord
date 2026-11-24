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

/// Maximum length in bytes for a sanitized chat title.
const MAX_CHAT_TITLE_BYTES: usize = 64;

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
    pub transcript_path: Option<String>,
    #[serde(default)]
    pub prompt: Option<String>,
    #[serde(default)]
    pub permission_mode: Option<String>,
    #[serde(default)]
    pub tool_name: Option<String>,
    #[serde(default)]
    pub tool_input: Option<ClaudeToolInput>,
    #[serde(default)]
    pub notification_type: Option<String>,
    #[serde(default)]
    pub agent_type: Option<String>,
}

/// Local metadata extracted from a session's transcript JSONL and `.git/HEAD`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClaudeSessionMetadata {
    pub chat_title: Option<String>,
    pub fallback_prompt: Option<String>,
    pub model: Option<String>,
    pub git_branch: Option<String>,
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
    pub cwd: Option<String>,
    pub transcript_path: Option<String>,
    pub chat_title: Option<String>,
    pub has_explicit_title: bool,
    pub model: Option<String>,
    pub git_branch: Option<String>,
    pub permission_mode: Option<String>,
    pub activity: ClaudeActivity,
    pub started_at: i64,
    pub last_event_at: i64,
}

impl ClaudeSession {
    /// Returns just the human-readable action for this session.
    pub fn action_text(&self) -> String {
        format_action(&self.activity)
    }

    /// Formats the `state` line for Discord Rich Presence (legacy action + project).
    pub fn state_text(&self) -> String {
        format_claude_activity(&self.activity, self.project_name.as_deref())
    }

    /// Formats the `details` line: `Claude Code · <project>` or `Claude Code`.
    pub fn details_text(&self) -> String {
        let project = self
            .project_name
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let full = match project {
            Some(project) => format!("{CLAUDE_CODE_DISPLAY_NAME} · {project}"),
            None => CLAUDE_CODE_DISPLAY_NAME.to_string(),
        };
        truncate_discord_text(&full, MAX_DISCORD_TEXT_BYTES)
    }

    /// Formats the `state` line: `<action> · <chat_title>` (when enabled and present) or `<action>`.
    pub fn state_text_rich(&self, show_chat_title: bool) -> String {
        let action = self.action_text();
        let title = if show_chat_title {
            self.chat_title
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
        } else {
            None
        };
        let full = match title {
            Some(title) => format!("{action} · {title}"),
            None => action,
        };
        truncate_discord_text(&full, MAX_DISCORD_TEXT_BYTES)
    }

    /// Formats the `large_text` tooltip: `<model> · <git_branch> · <N> chats`.
    pub fn large_text(&self, active_count: usize) -> String {
        let mut parts: Vec<String> = Vec::new();
        let model = self
            .model
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or(CLAUDE_CODE_DISPLAY_NAME);
        parts.push(model.to_string());

        if let Some(branch) = self
            .git_branch
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            parts.push(branch.to_string());
        }

        if active_count > 1 {
            parts.push(format!("{active_count} chats"));
        }

        truncate_discord_text(&parts.join(" · "), MAX_DISCORD_TEXT_BYTES)
    }
}

/// Tracks concurrent Claude Code sessions and determines which session is active.
#[derive(Debug, Default)]
pub struct ClaudeSessionTracker {
    sessions: BTreeMap<String, ClaudeSession>,
    pinned_session_id: Option<String>,
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
            let removed = self.sessions.remove(&session_id).is_some();
            if removed && self.pinned_session_id.as_deref() == Some(session_id.as_str()) {
                self.pinned_session_id = None;
            }
            return removed;
        }

        let project = payload.cwd.as_deref().and_then(extract_project_name);

        let applied = match event {
            "SessionStart" => {
                self.upsert(&session_id, project, Some(ClaudeActivity::Ready), now);
                true
            }
            "UserPromptSubmit" => {
                self.upsert(&session_id, project, Some(ClaudeActivity::Thinking), now);
                if let Some(prompt_title) = payload.prompt.as_deref().and_then(sanitize_chat_title) {
                    if let Some(session) = self.sessions.get_mut(&session_id) {
                        if !session.has_explicit_title {
                            session.chat_title = Some(prompt_title);
                        }
                    }
                }
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
        };

        if applied {
            if let Some(existing) = self.sessions.get_mut(&session_id) {
                if let Some(cwd) = payload.cwd.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
                    existing.cwd = Some(cwd.to_string());
                }
                if let Some(path) = payload
                    .transcript_path
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                {
                    existing.transcript_path = Some(path.to_string());
                }
                if let Some(mode) = payload
                    .permission_mode
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                {
                    existing.permission_mode = Some(mode.to_string());
                }
            }
        }

        applied
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
                    cwd: None,
                    transcript_path: None,
                    chat_title: None,
                    has_explicit_title: false,
                    model: None,
                    git_branch: None,
                    permission_mode: None,
                    activity: activity.unwrap_or(ClaudeActivity::Ready),
                    started_at: now,
                    last_event_at: now,
                },
            );
        }
    }

    /// Applies locally extracted metadata to an existing session.
    /// Returns `true` if any visible field changed.
    pub fn apply_metadata(&mut self, session_id: &str, meta: &ClaudeSessionMetadata) -> bool {
        let key = normalized_session_id(session_id);
        let Some(session) = self.sessions.get_mut(&key) else {
            return false;
        };

        let mut changed = false;

        if let Some(title) = meta.chat_title.as_deref().and_then(sanitize_chat_title) {
            if !session.has_explicit_title || session.chat_title.as_deref() != Some(title.as_str())
            {
                session.chat_title = Some(title);
                session.has_explicit_title = true;
                changed = true;
            }
        } else if !session.has_explicit_title && session.chat_title.is_none() {
            if let Some(fallback) = meta
                .fallback_prompt
                .as_deref()
                .and_then(sanitize_chat_title)
            {
                session.chat_title = Some(fallback);
                changed = true;
            }
        }

        if let Some(model) = meta.model.as_deref().and_then(format_model_name) {
            if session.model.as_deref() != Some(model.as_str()) {
                session.model = Some(model);
                changed = true;
            }
        }

        if let Some(branch) = meta
            .git_branch
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            if session.git_branch.as_deref() != Some(branch) {
                session.git_branch = Some(branch.to_string());
                changed = true;
            }
        }

        changed
    }

    /// Pins a specific session by ID, or clears the pin when `None` is passed.
    /// Returns `true` if the pin state changed.
    pub fn select_session(&mut self, session_id: Option<String>) -> bool {
        match session_id {
            Some(raw) => {
                let id = normalized_session_id(&raw);
                if !self.sessions.contains_key(&id) {
                    return false;
                }
                if self.pinned_session_id.as_deref() == Some(id.as_str()) {
                    return false;
                }
                self.pinned_session_id = Some(id);
                true
            }
            None => {
                if self.pinned_session_id.is_some() {
                    self.pinned_session_id = None;
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Returns the currently pinned session ID, if any.
    pub fn pinned_session_id(&self) -> Option<&str> {
        self.pinned_session_id.as_deref()
    }

    /// Looks up a tracked session by ID.
    pub fn get_session(&self, session_id: &str) -> Option<&ClaudeSession> {
        let key = normalized_session_id(session_id);
        self.sessions.get(&key)
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

        if let Some(ref pinned) = self.pinned_session_id {
            if !self.sessions.contains_key(pinned) {
                self.pinned_session_id = None;
            }
        }

        self.sessions.len() != before
    }

    /// Returns the pinned session if active, otherwise the most recently active session.
    pub fn active_session(&self) -> Option<&ClaudeSession> {
        if let Some(ref pinned) = self.pinned_session_id {
            if let Some(session) = self.sessions.get(pinned) {
                return Some(session);
            }
        }
        self.sessions.values().max_by(|a, b| {
            a.last_event_at
                .cmp(&b.last_event_at)
                .then_with(|| a.session_id.cmp(&b.session_id))
        })
    }

    /// Returns all active sessions ordered by most recently active first.
    pub fn sessions_ordered(&self) -> Vec<&ClaudeSession> {
        let mut list: Vec<&ClaudeSession> = self.sessions.values().collect();
        list.sort_by(|a, b| {
            b.last_event_at
                .cmp(&a.last_event_at)
                .then_with(|| a.session_id.cmp(&b.session_id))
        });
        list
    }

    /// Number of currently tracked unexpired sessions.
    pub fn active_count(&self) -> usize {
        self.sessions.len()
    }

    /// Clears all tracked sessions (for example when disconnecting hooks).
    pub fn clear(&mut self) -> bool {
        let had = !self.sessions.is_empty() || self.pinned_session_id.is_some();
        self.sessions.clear();
        self.pinned_session_id = None;
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

/// Formats a [`ClaudeActivity`] into a concise human-readable action string.
pub fn format_action(activity: &ClaudeActivity) -> String {
    match activity {
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
    }
}

/// Normalizes an Anthropic model ID (e.g. `claude-opus-4-6`, `claude-haiku-4-5-20251001`)
/// into a readable display label (e.g. `Claude Opus 4.6`, `Claude Haiku 4.5`).
pub fn format_model_name(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    let lower = trimmed.to_ascii_lowercase();
    let Some(rest) = lower.strip_prefix("claude-") else {
        return Some(trimmed.to_string());
    };

    let mut tokens: Vec<&str> = rest.split('-').filter(|t| !t.is_empty()).collect();
    if let Some(last) = tokens.last() {
        if last.len() == 8 && last.bytes().all(|b| b.is_ascii_digit()) {
            tokens.pop();
        }
    }

    if tokens.is_empty() {
        return Some("Claude".to_string());
    }

    let mut family: Option<&str> = None;
    let mut version_parts: Vec<&str> = Vec::new();
    let mut extras: Vec<&str> = Vec::new();

    for token in tokens {
        match token {
            "opus" => family = Some("Opus"),
            "sonnet" => family = Some("Sonnet"),
            "haiku" => family = Some("Haiku"),
            _ if token.bytes().all(|b| b.is_ascii_digit()) => version_parts.push(token),
            _ => extras.push(token),
        }
    }

    let mut out = vec!["Claude".to_string()];
    if let Some(fam) = family {
        out.push(fam.to_string());
    }
    if !version_parts.is_empty() {
        out.push(version_parts.join("."));
    }
    for extra in extras {
        out.push(extra.to_string());
    }

    Some(out.join(" "))
}

/// Extracts a single-line, markdown-stripped, length-capped chat title from raw text.
pub fn sanitize_chat_title(raw: &str) -> Option<String> {
    for line in raw.lines() {
        let stripped = line
            .trim()
            .trim_start_matches(['#', '>', '-', '*', ' '])
            .trim();
        if stripped.is_empty() {
            continue;
        }
        let collapsed: String = stripped.split_whitespace().collect::<Vec<_>>().join(" ");
        if collapsed.is_empty() {
            continue;
        }
        return Some(truncate_discord_text(&collapsed, MAX_CHAT_TITLE_BYTES));
    }
    None
}

/// Formats a Claude Code activity and optional project into a Discord `state` line,
/// capped safely at Discord's 128-byte limit.
pub fn format_claude_activity(activity: &ClaudeActivity, project: Option<&str>) -> String {
    let action = format_action(activity);

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

/// `process_name` of the reserved Apps-list entry that ranks Claude Code sessions
/// against other apps. It never matches a running process.
pub const CLAUDE_CODE_RULE_ID: &str = CLAUDE_CODE_CATALOG_ID;

fn same_rule_name(a: &str, b: &str) -> bool {
    let a = a.trim().to_ascii_lowercase();
    let b = b.trim().to_ascii_lowercase();
    a.trim_end_matches(".exe") == b.trim_end_matches(".exe")
}

/// Whether an active Claude Code session replaces the window presence, given the
/// Apps list as `(process_name, enabled)` in priority order and the app window
/// detection selected. Without a Claude Code entry it always does (legacy); a
/// disabled entry never does; otherwise it does unless `window_app` is an
/// enabled rule placed above the entry.
pub fn claude_code_takes_precedence(rules: &[(&str, bool)], window_app: Option<&str>) -> bool {
    let Some(claude_idx) = rules
        .iter()
        .position(|(name, _)| same_rule_name(name, CLAUDE_CODE_RULE_ID))
    else {
        return true;
    };
    if !rules[claude_idx].1 {
        return false;
    }
    let Some(app) = window_app.map(str::trim).filter(|a| !a.is_empty()) else {
        return true;
    };
    !rules[..claude_idx]
        .iter()
        .any(|(name, enabled)| *enabled && same_rule_name(name, app))
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

    #[test]
    fn formats_model_names_cleanly() {
        assert_eq!(
            format_model_name("claude-opus-4-6"),
            Some("Claude Opus 4.6".to_string())
        );
        assert_eq!(
            format_model_name("claude-sonnet-4-6"),
            Some("Claude Sonnet 4.6".to_string())
        );
        assert_eq!(
            format_model_name("claude-haiku-4-5-20251001"),
            Some("Claude Haiku 4.5".to_string())
        );
        assert_eq!(
            format_model_name("claude-3-5-sonnet-20241022"),
            Some("Claude Sonnet 3.5".to_string())
        );
        assert_eq!(
            format_model_name("custom-model"),
            Some("custom-model".to_string())
        );
        assert_eq!(format_model_name("   "), None);
    }

    #[test]
    fn sanitizes_chat_titles_safely() {
        assert_eq!(
            sanitize_chat_title("\n  # Fix   auth   flow \nsecond line"),
            Some("Fix auth flow".to_string())
        );
        assert_eq!(
            sanitize_chat_title("> - * Refactor engine"),
            Some("Refactor engine".to_string())
        );
        assert_eq!(sanitize_chat_title("   \n  #  "), None);

        let long_unicode = "é".repeat(50);
        let truncated = sanitize_chat_title(&long_unicode).unwrap();
        assert!(truncated.len() <= MAX_CHAT_TITLE_BYTES);
        assert!(truncated.ends_with("..."));
    }

    #[test]
    fn formats_rich_presence_fields_and_respects_show_chat_title() {
        let mut tracker = ClaudeSessionTracker::new();
        let mut prompt_ev = payload("UserPromptSubmit", "s1", Some("/work/Better-RPC"));
        prompt_ev.prompt = Some("Add chat list preview".to_string());
        tracker.apply_event(&prompt_ev, 100);

        let mut edit_ev = payload("PreToolUse", "s1", Some("/work/Better-RPC"));
        edit_ev.tool_name = Some("Edit".to_string());
        edit_ev.tool_input = Some(ClaudeToolInput {
            file_path: Some("/work/Better-RPC/src/engine.rs".to_string()),
            ..Default::default()
        });
        tracker.apply_event(&edit_ev, 110);

        tracker.apply_metadata(
            "s1",
            &ClaudeSessionMetadata {
                chat_title: Some("Claude Code Rich Details".to_string()),
                fallback_prompt: None,
                model: Some("claude-opus-4-6".to_string()),
                git_branch: Some("feat/rich-details".to_string()),
            },
        );

        let s = tracker.active_session().unwrap();
        assert_eq!(s.details_text(), "Claude Code · Better-RPC");
        assert_eq!(
            s.state_text_rich(true),
            "Editing engine.rs · Claude Code Rich Details"
        );
        assert_eq!(s.state_text_rich(false), "Editing engine.rs");
        assert_eq!(s.large_text(1), "Claude Opus 4.6 · feat/rich-details");
        assert_eq!(
            s.large_text(3),
            "Claude Opus 4.6 · feat/rich-details · 3 chats"
        );
    }

    #[test]
    fn metadata_explicit_title_upgrades_fallback_and_is_not_overwritten_by_prompt() {
        let mut tracker = ClaudeSessionTracker::new();
        let mut p1 = payload("UserPromptSubmit", "s1", Some("/work/app"));
        p1.prompt = Some("Initial prompt fallback".to_string());
        tracker.apply_event(&p1, 10);
        assert_eq!(
            tracker.active_session().unwrap().chat_title.as_deref(),
            Some("Initial prompt fallback")
        );

        // Explicit title replaces prompt fallback.
        assert!(tracker.apply_metadata(
            "s1",
            &ClaudeSessionMetadata {
                chat_title: Some("Renamed Chat".to_string()),
                fallback_prompt: Some("Other fallback".to_string()),
                model: None,
                git_branch: None,
            }
        ));
        assert_eq!(
            tracker.active_session().unwrap().chat_title.as_deref(),
            Some("Renamed Chat")
        );

        // Subsequent UserPromptSubmit does not overwrite explicit title.
        let mut p2 = payload("UserPromptSubmit", "s1", Some("/work/app"));
        p2.prompt = Some("Follow up prompt".to_string());
        tracker.apply_event(&p2, 20);
        assert_eq!(
            tracker.active_session().unwrap().chat_title.as_deref(),
            Some("Renamed Chat")
        );
    }

    #[test]
    fn tracker_pinning_and_auto_unpin_on_end_and_expiry() {
        let mut tracker = ClaudeSessionTracker::new();
        tracker.apply_event(&payload("UserPromptSubmit", "s1", Some("/work/a")), 100);
        tracker.apply_event(&payload("UserPromptSubmit", "s2", Some("/work/b")), 200);

        // Auto-follow picks s2 (most recent).
        assert_eq!(tracker.active_session().unwrap().session_id, "s2");
        assert_eq!(
            tracker
                .sessions_ordered()
                .iter()
                .map(|s| s.session_id.as_str())
                .collect::<Vec<_>>(),
            vec!["s2", "s1"]
        );

        // Pin s1; even newer events on s2 keep s1 active.
        assert!(tracker.select_session(Some("s1".to_string())));
        assert_eq!(tracker.pinned_session_id(), Some("s1"));
        assert_eq!(tracker.active_session().unwrap().session_id, "s1");

        tracker.apply_event(&payload("UserPromptSubmit", "s2", Some("/work/b")), 300);
        assert_eq!(tracker.active_session().unwrap().session_id, "s1");

        // Ending pinned s1 clears the pin and falls back to s2.
        tracker.apply_event(&payload("SessionEnd", "s1", None), 310);
        assert_eq!(tracker.pinned_session_id(), None);
        assert_eq!(tracker.active_session().unwrap().session_id, "s2");

        // Pin s2 and let it expire -> pin clears.
        assert!(tracker.select_session(Some("s2".to_string())));
        tracker.apply_event(&payload("Stop", "s2", Some("/work/b")), 320);
        assert!(tracker.prune_expired(1000, 300));
        assert_eq!(tracker.pinned_session_id(), None);
        assert!(tracker.active_session().is_none());
    }

    #[test]
    fn claude_code_precedence_follows_the_apps_list() {
        // No entry: legacy behavior, Claude Code always wins.
        assert!(claude_code_takes_precedence(&[("code", true)], Some("code")));
        // Disabled entry: never wins.
        assert!(!claude_code_takes_precedence(&[("claude_code", false)], Some("firefox")));
        // App above the entry wins.
        let rules = [("code", true), ("claude_code", true), ("firefox", true)];
        assert!(!claude_code_takes_precedence(&rules, Some("code")));
        assert!(!claude_code_takes_precedence(&rules, Some("Code.exe")));
        // App below the entry, unknown app, or no window: Claude Code wins.
        assert!(claude_code_takes_precedence(&rules, Some("firefox")));
        assert!(claude_code_takes_precedence(&rules, Some("gimp")));
        assert!(claude_code_takes_precedence(&rules, None));
        // A disabled rule above the entry does not outrank it.
        let rules = [("code", false), ("claude_code", true)];
        assert!(claude_code_takes_precedence(&rules, Some("code")));
    }
}
