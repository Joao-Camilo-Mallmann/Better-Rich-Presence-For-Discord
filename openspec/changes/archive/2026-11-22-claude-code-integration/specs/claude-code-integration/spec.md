# claude-code-integration Specification

## Purpose

Connects Better Rich Presence to Claude Code via native HTTP lifecycle hooks so live Claude Code coding sessions appear accurately in Discord Rich Presence across Windows and Linux, with one-click setup and status visibility in Settings.

## Requirements

### Requirement: Local HTTP hook receiver
The system SHALL run an HTTP/1.1 listener bound strictly to `127.0.0.1` on the configured Claude Code port (default `47892`) that accepts Claude Code hook POST requests at `/hooks/claude-code` and health checks at `/health`.

#### Scenario: Receiving a valid hook event
- **WHEN** Claude Code sends `POST /hooks/claude-code` with a valid JSON hook payload
- **THEN** the server responds immediately with HTTP `200 OK` and JSON body `{}`
- **AND** forwards the parsed event to the presence engine without blocking Claude Code

#### Scenario: Supporting chunked transfer encoding
- **WHEN** a client sends `POST /hooks/claude-code` using `Transfer-Encoding: chunked`
- **THEN** the server decodes the chunked body and processes the JSON payload identically to a `Content-Length` request

#### Scenario: Rejecting malformed payloads safely
- **WHEN** a client sends invalid JSON to `POST /hooks/claude-code`
- **THEN** the server responds with HTTP `400 Bad Request` and does not crash or alter active sessions

#### Scenario: Listener bind failure does not crash the app
- **WHEN** the configured localhost port is already in use at startup
- **THEN** the app continues running normally
- **AND** `get_claude_code_status` reports `server_listening: false` with a human-readable `server_error`

### Requirement: Non-destructive Claude Code hook configuration
The system SHALL install, inspect, and remove Better RPC's HTTP hooks in `$CLAUDE_CONFIG_DIR/settings.json` (when `CLAUDE_CONFIG_DIR` is set) or `<home>/.claude/settings.json` without modifying or deleting any other settings or user-defined hooks.

#### Scenario: Connecting hooks to an existing settings file
- **WHEN** the user invokes `connect_claude_code` and `~/.claude/settings.json` already contains other settings and custom hooks
- **THEN** the system preserves every existing key and custom hook
- **AND** adds Better RPC HTTP hooks (`type: "http"`, `url: "http://127.0.0.1:<port>/hooks/claude-code"`, `timeout: 2`) for `UserPromptSubmit`, `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `Notification`, `Stop`, `StopFailure`, `SubagentStart`, `SubagentStop`, `CwdChanged`, and `SessionEnd`
- **AND** does not add hooks to `SessionStart` or `PermissionRequest`

#### Scenario: Connecting is idempotent
- **WHEN** `connect_claude_code` is invoked when Better RPC hooks are already installed
- **THEN** no duplicate Better RPC hook entries are created

#### Scenario: Disconnecting removes only Better RPC hooks
- **WHEN** the user invokes `disconnect_claude_code`
- **THEN** only Better RPC's `/hooks/claude-code` HTTP hook entries are removed
- **AND** user-defined hooks on the same events and all other settings remain intact
- **AND** active Claude Code sessions in the engine are cleared

#### Scenario: Invalid settings JSON is never overwritten
- **WHEN** `~/.claude/settings.json` contains malformed JSON
- **THEN** `connect_claude_code` and `disconnect_claude_code` return an error and leave the file untouched

### Requirement: Privacy-safe activity formatting
The system SHALL format Claude Code session state into a concise, privacy-safe Discord `state` string containing the action and project name (derived from `cwd`), exposing only filenames (never full paths, shell commands, or prompt text).

#### Scenario: Editing a file in a project
- **WHEN** a `PreToolUse` event arrives with `tool_name: "Edit"`, `tool_input.file_path: "/home/dev/my-app/src/main.rs"`, and `cwd: "/home/dev/my-app"`
- **THEN** `details` is `Claude Code` and `state` is `Editing main.rs in my-app`

#### Scenario: Windows paths are handled identically
- **WHEN** a `PreToolUse` event arrives with `tool_name: "Read"`, `tool_input.file_path: "C:\\Users\\dev\\repo\\Cargo.toml"`, and `cwd: "C:\\Users\\dev\\repo"`
- **THEN** `state` is `Reading Cargo.toml in repo`

#### Scenario: Shell commands do not leak arguments
- **WHEN** a `PreToolUse` event arrives with `tool_name: "Bash"` (or `"PowerShell"`) and `cwd: "/work/my-app"`
- **THEN** `state` is `Running command in my-app`

#### Scenario: PostToolUse preserves visible tool activity
- **WHEN** a `PostToolUse` event arrives after `PreToolUse` for `Edit` on `main.rs`
- **THEN** the session's heartbeat timestamp refreshes
- **AND** the visible activity remains `Editing main.rs in my-app` until the next prompt, tool call, notification, or stop event

#### Scenario: Turn states map clearly
- **WHEN** `UserPromptSubmit` arrives, `state` becomes `Thinking in <project>`
- **AND** when `Stop` or `StopFailure` arrives, `state` becomes `Waiting for input in <project>`
- **AND** when `Notification` arrives with `notification_type: "permission_prompt"`, `state` becomes `Waiting for approval in <project>`

### Requirement: Session tracking, precedence, and auto-expiry
The system SHALL track concurrent Claude Code sessions by `session_id`, prioritize the most recently active session over desktop window detection while enabled, and restore normal app detection when sessions end, expire, or integration is disabled.

#### Scenario: Active session overrides window detection
- **WHEN** `claude_code_enabled` is `true` and a Claude Code session is active
- **THEN** the engine publishes the Claude Code presence (`details: "Claude Code"`, `large_image` resolved to `claude.png`, `large_text: "Claude Code"`, `timestamp` set to session start)
- **AND** subsequent `WindowChanged` events are cached without replacing the Claude Code presence

#### Scenario: Session end restores desktop window presence
- **WHEN** a `SessionEnd` event closes the last active Claude Code session
- **THEN** the engine immediately restores presence from the latest cached window snapshot

#### Scenario: Inactivity auto-expiry
- **WHEN** a waiting Claude Code session receives no hook events for `claude_code_timeout_minutes` (default 5 minutes), or an in-flight session receives no events for the safety timeout (`max(timeout * 2, 15)` minutes)
- **THEN** the engine prunes the expired session and restores the latest cached window presence

#### Scenario: Disabling in Settings restores window detection
- **WHEN** the user turns off `claude_code_enabled` while a Claude Code session is active
- **THEN** the engine immediately restores normal window-based presence

#### Scenario: Rapid hook bursts do not stall the engine
- **WHEN** multiple hook events arrive within Discord's 15-second rate-limit window
- **THEN** the engine coalesces pending updates without blocking its event loop and publishes the latest state once the rate-limit window elapses

### Requirement: Claude Code Settings UI and catalog support
The system SHALL register `claude_code` in `apps.json` using `public/assets/icons/claude.png` and expose a **Claude Code** section in Settings for connection management, status display, precedence toggling, and timeout configuration.

#### Scenario: Settings shows status and one-click connect/disconnect
- **WHEN** the user opens the Settings view
- **THEN** the Claude Code section displays hook installation status, local listener status, active session summary (when live), the resolved config path, a Connect/Reconnect/Disconnect action button, a toggle for `claude_code_enabled`, and a numeric input for `claude_code_timeout_minutes`
