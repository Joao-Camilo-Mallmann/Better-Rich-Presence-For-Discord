# Spec Delta

## ADDED Requirements

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

## MODIFIED Requirements

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

### Requirement: Claude Code Settings UI and catalog support
The system SHALL register `claude_code` in `apps.json` using `public/assets/icons/claude.png` and expose a **Claude Code** section in Settings for connection management, status display, precedence toggling, chat title visibility toggling, and timeout configuration.

#### Scenario: Settings shows status and one-click connect/disconnect
- **WHEN** the user opens the Settings view
- **THEN** the Claude Code section displays hook installation status, local listener status, active session summary (including chat count when multiple sessions are live), the resolved config path, a Connect/Reconnect/Disconnect action button, toggles for `claude_code_enabled` and `claude_code_show_chat_title`, and a numeric input for `claude_code_timeout_minutes`
