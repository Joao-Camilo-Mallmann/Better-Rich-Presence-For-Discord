## Why

The backend terminal log is currently flooded with raw `TRACE` output from third-party crates (such as `discord_presence` reading IPC headers every second), prints everything in plain monochrome text without timestamps or component alignment, and omits clear app-name and transition context when switching or rate-limiting presences. Improving log filtering, color formatting, and transition messages makes it immediately obvious which app is active, queued, or switched to.

## What Changes

- Add a dedicated `services/logger.rs` module that filters logs by target (`better_rich_presence*` at `DEBUG`, external crates at `WARN`, overridable via `RUST_LOG`).
- Format terminal logs with aligned timestamps, color-coded levels, color-coded component tags (`[Watcher]`, `[Engine]`, `[Discord]`, `[Claude]`, `[Linux]`, `[App]`), and highlighted transition keywords (`Active:`, `Switch:`, `Update:`), with Windows Virtual Terminal enablement and `NO_COLOR` / `CLICOLOR` support.
- Preserve plain-text (ANSI-free) `message` payloads on the `"app-log"` Tauri event.
- Enhance log messages across `watcher.rs`, `engine.rs`, and `discord.rs` to show friendly app names, `from → to` switches, rate-limit coalescing targets, Claude Code overrides, idle transitions, and short icon filenames, while removing duplicate manual `[Discord]` / `[ClaudeCode]` / `[Linux]` / `[Engine]` prefixes.

## Capabilities

### New Capabilities
- `backend-logging`: Target-aware log filtering, `RUST_LOG` overrides, ANSI color formatting with `NO_COLOR` support, plain-text `app-log` event emission, and human-readable presence transition logging.

### Modified Capabilities
<!-- None: existing spec requirements are unchanged. -->

## Impact

- `src-tauri/Cargo.toml`: Add `Win32_System_Console` feature to the Windows `windows` dependency.
- `src-tauri/src/services/logger.rs` & `src-tauri/src/services/mod.rs`: New logger module and export.
- `src-tauri/src/lib.rs`: Delegate logger initialization to `services::logger::init`.
- `src-tauri/src/services/watcher.rs`, `engine.rs`, `discord.rs`, `claude_code_server.rs`, `linux/mod.rs`: Clearer, deduplicated log calls.
