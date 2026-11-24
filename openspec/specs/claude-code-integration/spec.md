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
The system SHALL format Claude Code session state into concise, privacy-safe Discord `details`, `state`, and `large_text` strings exposing only filenames, project folder names, sanitized chat titles, formatted model names, and git branch names (never full paths, shell commands, or raw multi-line prompts).

#### Scenario: Editing a file in a project
- **WHEN** a `PreToolUse` event arrives with `tool_name: "Edit"`, `tool_input.file_path: "/home/dev/my-app/src/main.rs"`, and `cwd: "/home/dev/my-app"` without a chat title
- **THEN** `details` is `Claude Code · my-app` and `state` is `Editing main.rs`

#### Scenario: Editing a file in a project with a chat title
- **WHEN** a `PreToolUse` event arrives with `tool_name: "Edit"`, `tool_input.file_path: "/home/dev/my-app/src/main.rs"`, `cwd: "/home/dev/my-app"`, and chat title `"Fix auth flow"` (`claude_code_show_chat_title: true`)
- **THEN** `details` is `Claude Code · my-app` and `state` is `Editing main.rs · Fix auth flow`

#### Scenario: Windows paths are handled identically
- **WHEN** a `PreToolUse` event arrives with `tool_name: "Read"`, `tool_input.file_path: "C:\\Users\\dev\\repo\\Cargo.toml"`, and `cwd: "C:\\Users\\dev\\repo"` without a chat title
- **THEN** `details` is `Claude Code · repo` and `state` is `Reading Cargo.toml`

#### Scenario: Shell commands do not leak arguments
- **WHEN** a `PreToolUse` event arrives with `tool_name: "Bash"` (or `"PowerShell"`) and `cwd: "/work/my-app"`
- **THEN** `details` is `Claude Code · my-app` and `state` starts with `Running command`

#### Scenario: PostToolUse preserves visible tool activity
- **WHEN** a `PostToolUse` event arrives after `PreToolUse` for `Edit` on `main.rs`
- **THEN** the session's heartbeat timestamp refreshes
- **AND** the visible action remains `Editing main.rs` until the next prompt, tool call, notification, or stop event

#### Scenario: Turn states map clearly
- **WHEN** `UserPromptSubmit` arrives, the action becomes `Thinking`
- **AND** when `Stop` or `StopFailure` arrives, the action becomes `Waiting for input`
- **AND** when `Notification` arrives with `notification_type: "permission_prompt"`, the action becomes `Waiting for approval`

#### Scenario: Large image tooltip includes model, branch, and chat count
- **WHEN** an active session has model `Claude Opus 4.6`, git branch `main`, and 2 active sessions exist
- **THEN** `large_text` is `Claude Opus 4.6 · main · 2 chats`

### Requirement: Session tracking, precedence, and auto-expiry
The system SHALL track concurrent Claude Code sessions by `session_id` and restore normal app detection when sessions end, expire, or integration is disabled. Whether the most recently active session replaces the window presence SHALL follow the position of the Claude Code entry (`process_name` `claude_code`) in the Apps priority list.

#### Scenario: Active session overrides window detection
- **WHEN** `claude_code_enabled` is `true`, a Claude Code session is active, and the app selected by window detection has no enabled rule ranked above the Claude Code entry
- **THEN** the engine publishes the Claude Code presence (`details: "Claude Code"`, `large_image` resolved to `claude.png`, `large_text: "Claude Code"`, `timestamp` set to session start)

#### Scenario: Higher-ranked app wins over Claude Code
- **WHEN** a Claude Code session is active and the selected window app is `code` whose enabled rule is ranked above the Claude Code entry
- **THEN** the engine publishes the Visual Studio Code window presence
- **AND** when the selected window app changes to an app ranked below the Claude Code entry, the Claude Code presence is published again

#### Scenario: Disabled Claude Code entry
- **WHEN** the Claude Code entry in the Apps list is disabled
- **THEN** Claude Code sessions never replace the window presence

#### Scenario: Claude Code entry is created
- **WHEN** the app starts and the saved Apps list has no Claude Code entry and no rule for the Claude Code CLI process (`claude`, `claude.exe`, `claude-code`, `claude-code.exe`)
- **THEN** an enabled Claude Code entry is inserted at position 1

#### Scenario: Existing Claude rule becomes the Claude Code entry
- **WHEN** the app starts and the saved Apps list has a rule for process `claude` at position 2, disabled or enabled
- **THEN** that rule is replaced in place by the Claude Code entry, keeping position 2 and its enabled state

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
The system SHALL register `claude_code` in `apps.json` using `public/assets/icons/claude.png` and expose a **Claude Code** section in Settings for connection management, status display, precedence toggling, chat title visibility toggling, and timeout configuration.

#### Scenario: Settings shows status and one-click connect/disconnect
- **WHEN** the user opens the Settings view
- **THEN** the Claude Code section displays hook installation status, local listener status, active session summary (including chat count when multiple sessions are live), the resolved config path, a Connect/Reconnect/Disconnect action button, toggles for `claude_code_enabled` and `claude_code_show_chat_title`, and a numeric input for `claude_code_timeout_minutes`

### Requirement: Local session metadata enrichment
The system SHALL enrich tracked Claude Code sessions with chat title, model name, and git branch read locally from the session's `transcript_path` and repository `.git/HEAD`, without sending any data externally or blocking hook responses.

#### Scenario: Extracting explicit chat title, model, and branch from transcript
- **WHEN** a hook event includes `transcript_path` pointing to a JSONL file containing `custom-title`, `assistant` (`message.model`), and `gitBranch` entries
- **THEN** the session metadata is updated with the sanitized chat title, formatted model name (e.g. `Claude Opus 4.6`), and git branch

#### Scenario: Falling back to prompt and .git/HEAD when transcript lacks metadata
- **WHEN** a session has no explicit `custom-title`, `summary`, or `agent-name` in its transcript
- **THEN** the system falls back to a sanitized `last-prompt` or `UserPromptSubmit` `prompt` text, and reads `.git/HEAD` from `cwd` (or up to 5 parent directories) to resolve the git branch

#### Scenario: Explicit title upgrades a prompt fallback
- **WHEN** a session initially has a prompt-derived fallback title and later receives an explicit `custom-title`, `summary`, or `agent-name`
- **THEN** the explicit title replaces the prompt fallback, and subsequent fallback prompts never overwrite an explicit title

#### Scenario: Large transcript files are read with a bounded cap
- **WHEN** a `transcript_path` JSONL file exceeds 192 KiB
- **THEN** the metadata reader reads only a bounded head and tail slice instead of loading the entire file into memory

### Requirement: Multi-chat tracking and session pinning
The system SHALL expose all active unexpired Claude Code sessions ordered by most recent activity and allow the user to pin a specific session or return to auto-follow via `select_claude_code_session`.

#### Scenario: Pinning a specific chat session
- **WHEN** multiple Claude Code sessions are active and the user selects a specific session via `select_claude_code_session`
- **THEN** that session becomes the active presence published to Discord and previewed in the app, even when another session receives newer hook events

#### Scenario: Auto-unpinning when a pinned session ends or expires
- **WHEN** a pinned Claude Code session receives `SessionEnd` or expires due to inactivity
- **THEN** the pin is cleared automatically and the system falls back to the most recently active remaining session (or restores desktop window detection if none remain)

#### Scenario: Returning to auto-follow mode
- **WHEN** `select_claude_code_session` is invoked with `null`
- **THEN** the pin is cleared and the system tracks the most recently active session

### Requirement: Chat title visibility setting
The system SHALL provide a `claude_code_show_chat_title` setting (defaulting to `true`) that controls whether the active chat title is included in the Discord `state` line.

#### Scenario: Hiding chat title on Discord
- **WHEN** `claude_code_show_chat_title` is `false`
- **THEN** Discord `state` shows only the sanitized action without the chat title, while the in-app chats list continues displaying chat titles locally
