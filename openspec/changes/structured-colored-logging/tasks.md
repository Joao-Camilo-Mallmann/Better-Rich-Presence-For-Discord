## 1. Platform & Crate Configuration

- [x] 1.1 Add `"Win32_System_Console"` to `[target.'cfg(windows)'.dependencies.windows].features` in `src-tauri/Cargo.toml`

## 2. Structured Colored Logger Service

- [x] 2.1 Create `src-tauri/src/services/logger.rs` with `LogFilter` (internal `DEBUG`, external `WARN`, `RUST_LOG` parsing), `should_use_color`, Windows VT initialization, `component_tag`, `truncate_for_log`, `format_terminal_line`, and plain-text `"app-log"` emission
- [x] 2.2 Export `pub mod logger;` in `src-tauri/src/services/mod.rs` and replace inline logger boilerplate in `src-tauri/src/lib.rs` with `services::logger::init`
- [x] 2.3 Add unit tests in `src-tauri/src/services/logger.rs` for filtering, `RUST_LOG`, color toggles, component tags, terminal line formatting, and UTF-8 truncation

## 3. Service Transition & App Switch Logging

- [x] 3.1 Update `src-tauri/src/services/watcher.rs` to log window changes and priority overrides at `DEBUG` with UTF-8-safe truncated titles
- [x] 3.2 Update `src-tauri/src/services/engine.rs` with `presence_app_label`, `format_presence_summary`, `has_committed_presence`, clear `Active:` / `Switch:` / `Update:` / rate-limit / Claude override / idle transition logs, and unit tests for helpers
- [x] 3.3 Update `src-tauri/src/services/discord.rs` with `short_icon_label`, `on_ready` username logging, concise `DEBUG` `set_activity` logging, and remove duplicate `[Discord]` prefixes
- [x] 3.4 Remove redundant `[ClaudeCode]` and `[Linux]` prefixes from `src-tauri/src/services/claude_code_server.rs` and `src-tauri/src/services/linux/mod.rs`

## 4. Verification

- [x] 4.1 Run `cargo test --manifest-path src-tauri/Cargo.toml` and verify all tests pass with no warnings in touched files
- [x] 4.2 Run `cargo check --manifest-path src-tauri/Cargo.toml` to confirm both library and binary targets build cleanly

## 5. Documentation

- [x] 5.1 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise — N/A: no architectural conventions, workflows, or reference docs changed)
