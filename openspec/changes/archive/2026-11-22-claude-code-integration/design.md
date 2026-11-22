# Design: Claude Code Integration

## Context

Better Rich Presence detects foreground and prioritized desktop apps via OS window/process APIs (`watcher.rs`), maps them to `PresenceData` in `engine.rs`, and publishes them to Discord over local IPC (`discord.rs`). Because Claude Code runs inside host terminals, IDE panels, or headless subagents, window detection cannot observe its live project or tool activity.

Claude Code supports native HTTP hooks configured in `~/.claude/settings.json` (or `$CLAUDE_CONFIG_DIR/settings.json`). When a subscribed lifecycle event fires, Claude Code POSTs a JSON payload containing `session_id`, `hook_event_name`, `cwd`, and event-specific fields (`tool_name`, `tool_input`, `notification_type`, `agent_type`) to the configured URL and treats HTTP connection failures or non-2xx responses as non-blocking errors.

## Goals / Non-Goals

**Goals:**

- Cross-platform (Windows and Linux) Claude Code integration with zero external runtime dependencies (no Node, `jq`, or shell wrappers required).
- Safe, idempotent, non-destructive hook installation and removal in `~/.claude/settings.json` via one-click UI controls in Settings.
- Live, privacy-safe Discord Rich Presence showing `"Claude Code"` as `details`, human-readable project + action as `state`, the official `claude.png` icon, and the session elapsed timer.
- Clean Architecture compliance: pure domain rules in `src-tauri/src/domain/claude_code.rs` (`std` and `serde` only), pure parsers/mergers next to their adapters, and full unit test coverage without requiring Tauri, Discord, or Claude Code to run.
- Non-blocking rate-limit coalescing in the presence engine so bursts of tool events never stall the engine loop.

**Non-Goals:**

- Blocking, modifying, or approving Claude Code tool calls or permission prompts (Better RPC is strictly observational and always responds `200 OK` with `{}`).
- Reading or uploading prompt text, shell command arguments, file contents, or `~/.claude/projects/` JSONL transcripts.
- Modifying `statusLine` or `allowedHttpHookUrls` in the user's Claude Code configuration.

## Decisions

### 1. Native HTTP Hooks on Loopback (`127.0.0.1:47892`)

- **Choice**: Embed a lightweight HTTP/1.1 listener using `tokio::net::TcpListener` bound strictly to `127.0.0.1:<port>` (default `47892`), enabled by adding `net` and `io-util` features to the existing `tokio` dependency.
- **Why**: Avoids adding HTTP framework dependencies (`axum`/`hyper`) and keeps the binary lean, never exposes a port outside loopback, works identically on Windows and Linux regardless of shell (`cmd`, PowerShell, Git Bash, zsh, bash), and responds in under 1 ms with `200 OK` + `{}`.
- **Protocol handling**: Supports `POST /hooks/claude-code` and `GET /health`, case-insensitive headers, `Content-Length`, `Transfer-Encoding: chunked`, `Expect: 100-continue`, a 16 KiB header cap, an 8 MiB body cap (to accommodate large `Write`/`PostToolUse` payloads), and a 2-second per-connection read timeout.

### 2. Hook Event Selection & Safety

- **Subscribed events**: `UserPromptSubmit`, `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `Notification`, `Stop`, `StopFailure`, `SubagentStart`, `SubagentStop`, `CwdChanged`, `SessionEnd`.
- **Excluded intentionally**:
  - `SessionStart`: Claude Code does not dispatch `type: "http"` hooks for `SessionStart`. Registering it would add dead config; instead, the first live event (`UserPromptSubmit`, `PreToolUse`, etc.) automatically initializes the session, and the parser still handles `SessionStart` if ever sent.
  - `PermissionRequest`: Acts as a permission gate and can fail closed when unreachable in some Claude Code versions. `Notification` (`permission_prompt`) provides observational waiting-for-approval status without ever risking blocked tool approvals.
- **Timeout**: Each installed hook sets `"timeout": 2` (2 seconds) and omits `"onFailure"`, so if Better RPC is closed, localhost connection refusal fails fast and non-blockingly.

### 3. Privacy-Safe Activity Mapping & Flicker Prevention

- `ClaudeToolInput` deserializes only `file_path`, `notebook_path`, and `path`, reducing them to their final filename component (`extract_file_name`) across both `/` and `\` separators. Raw shell commands (`Bash`/`PowerShell`) map to `"Running command"` without reading command text.
- `PostToolUse` and `PostToolUseFailure` refresh `last_event_at` (heartbeat) without overwriting the preceding tool activity, preventing 10 ms edits from immediately reverting to `"Thinking"` before Discord's 15-second update cadence can display them.
- `CwdChanged` updates `project_name` without resetting `started_at` or clearing activity.
- `format_claude_activity` builds `<action> in <project>` (or `<action>` when `cwd` is absent/root) and truncates safely on UTF-8 char boundaries at 128 bytes.

### 4. Multi-Session Tracking & Auto-Expiry

- `ClaudeSessionTracker` keeps sessions keyed by `session_id` (falling back to `"default"` if empty) and selects the session with the most recent `last_event_at`.
- Because `SessionEnd` does not fire on hard kills (`SIGKILL`, closed terminal window), the engine prunes sessions every tick:
  - Waiting states (`WaitingForInput`, `WaitingForApproval`, `Ready`) expire after `claude_code_timeout_minutes * 60` seconds (default 5 minutes).
  - Actively running states (`Thinking`, tool use, subagents) use a safety floor of `max(waiting_timeout * 2, 900)` seconds (at least 15 minutes) so long builds or test suites do not drop mid-turn.
- When the last session ends or expires, the engine immediately restores the most recent cached `WindowChanged` snapshot.

### 5. Non-Blocking Rate-Limit Coalescing & Timestamp Preservation

- Previously, `PresenceEngine::apply_presence` slept inline inside the main `tokio::select!` loop during Discord's 15-second debounce window. Rapid Claude Code tool hooks would queue and stall the engine.
- Instead, when `last_update_time` is still within the debounce window, `apply_presence` stores the latest `(client_id, PresenceData)` in a `pending_presence` slot and arms a single coalesced flush timer for the remaining delay. Subsequent updates simply overwrite `pending_presence` (keeping preview & Discord synced to the newest state once the rate limit opens without blocking events or status ticks).
- When `new_data.timestamp > 0` (used by Claude Code sessions and IDE workspaces), `apply_presence` preserves `new_data.timestamp` rather than overwriting it with `self.current_data.timestamp`. Catalog/rule presences continue passing `0` and inheriting the existing timer as before.

### 6. Non-Destructive Settings Merger

- Better RPC identifies its own hook entries via `type == "http"` and a loopback URL ending in `/hooks/claude-code`.
- `install_hooks_in_json` strips any prior Better RPC entries (so reconnects are idempotent and clean up old ports) and appends one matcher group per event without modifying sibling user hooks or other top-level keys.
- `remove_hooks_from_json` removes only Better RPC entries and prunes empty wrapper groups/arrays/objects it emptied.
- Files are written atomically via a sibling temp file + rename, and malformed user JSON returns an error instead of clobbering the file.

### 7. UI & Design System Alignment

- A new **Claude Code** group is placed in `Settings` using the existing `Group`, `SettingRow`, `Toggle`, `NumberInput`, and `AppIcon` primitives.
- Buttons follow `DESIGN.md`: Blurple (`bg-primary`) for Connect/Reconnect, neutral dark (`bg-canvas`) for Disconnect (no loud green CTA on Settings), keeping the Settings view calm.
- All frontend hooks and components avoid `try/finally` and conditional/logical expressions inside `try` blocks so `babel-plugin-react-compiler` compiles 100% of functions with 0 skipped.

## Risks / Trade-offs

- **Port conflict on `47892`**: If another local process occupies `47892`, the listener logs the bind error and exposes `server_error` in `ClaudeCodeStatus` so Settings surfaces the issue clearly without crashing the app.
- **Sessions already open before connecting**: Claude Code's file watcher usually reloads `~/.claude/settings.json` automatically, though older versions pick up new hooks on the next session; the UI notes this succinctly.
- **No `SessionStart` HTTP hook**: A newly opened idle Claude Code prompt won't appear on Discord until the user submits their first prompt (`UserPromptSubmit`). This is an upstream Claude Code limitation and avoids writing a non-functional hook entry.
