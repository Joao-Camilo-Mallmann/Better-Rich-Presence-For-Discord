# Proposal: Claude Code Integration

## Why

Claude Code runs inside terminals, IDE extensions, and background subagents, so desktop window-title detection alone only sees the host terminal (for example `WindowsTerminal.exe`, `alacritty`, or `code`) and cannot tell whether a Claude Code session is running, which project it is working in, or what Claude is doing.

Claude Code natively supports HTTP lifecycle hooks (`"type": "http"`) that POST structured JSON event payloads to a local endpoint across both Windows and Linux with zero shell-script or Node dependencies. Connecting Better Rich Presence directly to Claude Code via a localhost HTTP hook receiver lets users display live, accurate Discord Rich Presence for their Claude Code sessions with one click from Settings.

## What Changes

- **Claude Code domain model and activity formatter (`src-tauri/src/domain/claude_code.rs`)**:
  - Parse Claude Code hook payloads (`UserPromptSubmit`, `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `Notification`, `Stop`, `StopFailure`, `SubagentStart`, `SubagentStop`, `CwdChanged`, `SessionEnd`, plus `SessionStart` for compatibility).
  - Track concurrent sessions by `session_id`, preserving each session's start timestamp and project name (derived from `cwd`) while extracting only basenames for edited/read files so full paths, shell commands, and prompts are never exposed to Discord.
  - Format human-readable activity lines (for example `Editing engine.rs in Better-RPC`, `Reading Cargo.toml in Better-RPC`, `Running command in Better-RPC`, `Thinking in Better-RPC`, `Waiting for approval in Better-RPC`, `Waiting for input in Better-RPC`) capped to Discord's 128-character limit.
  - Auto-expire stale sessions when `SessionEnd` does not fire (for example when a terminal is force-closed), using a shorter timeout for waiting-for-input sessions and a longer safety timeout for actively running turns.
- **Local HTTP hook receiver (`src-tauri/src/services/claude_code_server.rs`)**:
  - Bind strictly to `127.0.0.1:<port>` (default `47892`) using Tokio TCP networking with zero new external crates.
  - Accept `POST /hooks/claude-code` (supporting `Content-Length`, `Transfer-Encoding: chunked`, and `Expect: 100-continue`), forward parsed events to the presence engine without blocking, and return `200 OK` with `{}` so Claude Code is never blocked or steered.
  - Serve `GET /health` for diagnostics.
- **Non-destructive `~/.claude/settings.json` hook installer (`src-tauri/src/services/claude_code_config.rs`)**:
  - Resolve `$CLAUDE_CONFIG_DIR/settings.json` or `<home>/.claude/settings.json` cross-platform (Windows and Linux).
  - Connect installs idempotent HTTP hooks with a 2-second timeout on supported observational events (excluding `SessionStart`, which does not dispatch HTTP hooks in Claude Code, and `PermissionRequest`, which can gate permissions) while preserving all existing user settings and user-defined hooks.
  - Disconnect removes only Better RPC's hook entries and cleans up empty containers it created, writing atomically.
- **Presence engine precedence and coalescing (`src-tauri/src/services/engine.rs`)**:
  - While Claude Code integration is enabled and an unexpired session exists, Claude Code presence (`details: "Claude Code"`, `state: "<activity>"`, `large_image: "claude_code"` resolved to `claude.png`, `timestamp: session.started_at`) takes priority over foreground/priority window detection.
  - Coalesce rapid hook events during Discord's 15-second rate-limit window without blocking the engine loop so Discord always receives the latest activity once the window opens.
  - Restore the latest tracked desktop window presence immediately when a session ends, expires, disconnects, or when the Settings toggle is disabled.
  - Preserve explicit session/workspace timestamps (`timestamp > 0`) on the shared Discord client instead of overwriting them with the previous app's timestamp.
- **Catalog entry (`apps.json`)**:
  - Add `claude_code` ("Claude Code", category `developer`, referencing the existing committed `public/assets/icons/claude.png`).
- **Tauri interface & Settings UI (`src-tauri/src/commands/claude_code.rs`, `src/pages/Settings.tsx`, `src/components/ClaudeCodeSection.tsx`, `src/hooks/useClaudeCode.ts`)**:
  - Extend `Settings` with serde-defaulted `claude_code_enabled` (default `true`), `claude_code_port` (default `47892`), and `claude_code_timeout_minutes` (default `5`).
  - Add `get_claude_code_status`, `connect_claude_code`, and `disconnect_claude_code` commands plus the `claude-code-status-changed` event.
  - Add a **Claude Code** group to Settings with live connection/session status, one-click Connect/Reconnect/Disconnect, an enable/disable switch, and session timeout control matching the Neo-Brutalist Arcade design system.

## Capabilities

### New Capabilities

- `claude-code-integration`: localhost HTTP hook receiver, non-destructive `~/.claude/settings.json` hook management, privacy-safe activity formatting, multi-session tracking with auto-expiry, and Settings UI controls for Claude Code Rich Presence.

### Modified Capabilities

- `presence-display`: preserve explicit session/workspace start timestamps on the shared client and define Claude Code live session presence precedence over window detection.

## Impact

- Rust: `src-tauri/Cargo.toml` (add Tokio `net` and `io-util` features), `src-tauri/src/domain/{mod.rs,claude_code.rs}`, `src-tauri/src/services/{mod.rs,claude_code_config.rs,claude_code_server.rs,engine.rs}`, `src-tauri/src/commands/{mod.rs,claude_code.rs}`, `src-tauri/src/models/types.rs`, `src-tauri/src/lib.rs`.
- Catalog: `apps.json` (add `claude_code` entry using existing `claude.png`).
- Frontend: `src/types.ts`, `src/hooks/useClaudeCode.ts`, `src/components/ClaudeCodeSection.tsx`, `src/pages/Settings.tsx`.
- Frontend contract: adds Tauri commands `get_claude_code_status`, `connect_claude_code`, `disconnect_claude_code`, event `claude-code-status-changed`, and serde-defaulted fields on `Settings` (`claude_code_enabled`, `claude_code_port`, `claude_code_timeout_minutes`). Existing commands and payloads remain backward-compatible.
