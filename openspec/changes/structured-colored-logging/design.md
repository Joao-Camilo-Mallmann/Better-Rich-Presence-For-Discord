## Context

`src-tauri/src/lib.rs` currently registers an inline `AppLogger` with `log::set_max_level(LevelFilter::Trace)` and prints `[LEVEL] message` directly to stdout without target filtering or ANSI colors. Because `discord-presence` uses the `log` crate at `TRACE` level for every IPC header poll (`Receiving from connection` / `Reading header` / `Finished send and receive loop iteration`), the terminal is overwhelmed with noise while the actual application switches, rate-limit coalescing, and active app names are hard to spot.

## Goals / Non-Goals

**Goals:**
- Suppress third-party `INFO`/`DEBUG`/`TRACE` noise by default while keeping internal app logs at `DEBUG` and external crate `WARN`/`ERROR` visible.
- Allow overriding filter levels at runtime via `RUST_LOG` without adding external dependencies.
- Render colorized, column-aligned terminal output with timestamps, severity badges, component tags (`[Watcher]`, `[Engine]`, `[Discord]`, `[Claude]`, `[Linux]`, `[App]`), and highlighted app switch/active/update lines.
- Keep `"app-log"` Tauri event payloads plain text (no ANSI escape sequences).
- Log clear, human-readable app names and transition reasons across `watcher.rs`, `engine.rs`, and `discord.rs`.

**Non-Goals:**
- Adding a frontend log viewer UI or file-based log rotation.
- Adding new external crates (`env_logger`, `tracing`, `owo-colors`) to `Cargo.toml`.
- Changing Discord Rich Presence payloads or frontend IPC contracts.

## Decisions

- **Zero-dependency `services/logger.rs` over `env_logger`/`tracing`**: The project already uses `log = "0.4"` and `chrono = "0.4"`, and only needs `Win32_System_Console` from the already-present `windows` crate to enable Windows VT processing. A compact `services/logger.rs` module keeps dependencies minimal, stays unit-testable via pure functions (`LogFilter`, `component_tag`, `format_terminal_line`, `should_use_color`, `truncate_for_log`), and continues emitting Tauri `"app-log"` events.
- **Keep ANSI enabled even when stdout is piped unless `NO_COLOR`/`CLICOLOR=0`/`TERM=dumb`**: `tauri dev` and IDE runners often pipe stdout to an ANSI-capable terminal. Relying on `std::io::IsTerminal` would strip colors during `tauri dev`. Instead, enable Windows VT mode when attached to a console and respect standard `NO_COLOR`, `CLICOLOR=0`, `TERM=dumb`, and `CLICOLOR_FORCE` conventions.
- **Explicit transition logs in `engine.rs`**: Track `has_committed_presence: bool` in `PresenceEngine` and derive friendly labels (`presence_app_label`) so `commit_presence` logs `Active:`, `Switch: <from> → <to>`, or `Update:` cleanly with `details · state`, while `discord.rs` keeps low-level RPC details (`details="..."`, short icon filename) at `DEBUG`.

## Risks / Trade-offs

- **ANSI codes in non-VT log files**: If a user redirects stdout to a file without setting `NO_COLOR=1`, escape codes appear unless `NO_COLOR=1` is set. → Mitigation: honor `NO_COLOR`, `CLICOLOR=0`, and `TERM=dumb`, and keep `"app-log"` Tauri payloads strictly ANSI-free.
- **Hiding `discord_presence` trace logs by default**: Diagnosing raw Discord IPC frames is hidden at default settings. → Mitigation: support `RUST_LOG=discord_presence=trace` (or `RUST_LOG=trace`) so full frames remain accessible on demand.
