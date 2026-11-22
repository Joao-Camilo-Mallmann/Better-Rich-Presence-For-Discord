# Tasks

## 1. Domain model & catalog

- [x] 1.1 Add `claude_code` entry to `apps.json` using the committed `public/assets/icons/claude.png` icon (without colliding with `claude_desktop` executables)
- [x] 1.2 Create `src-tauri/src/domain/claude_code.rs` (pure `std` + `serde` only) with `ClaudeHookPayload`, `ClaudeActivity`, `ClaudeSession`, `ClaudeSessionTracker`, `extract_project_name`, `extract_file_name`, `format_claude_activity`, and unit tests covering all hook events, Unix/Windows paths, privacy rules, multi-session selection, truncation, and expiry; export from `src-tauri/src/domain/mod.rs`

## 2. Claude Code hook config & local HTTP receiver

- [x] 2.1 Add `net` and `io-util` to `tokio` features in `src-tauri/Cargo.toml`
- [x] 2.2 Create `src-tauri/src/services/claude_code_config.rs` with cross-platform settings path resolution (`CLAUDE_CONFIG_DIR` / home `.claude/settings.json`), pure `install_hooks_in_json`, `remove_hooks_from_json`, `inspect_hooks_in_json`, atomic file read/write, and comprehensive unit tests
- [x] 2.3 Create `src-tauri/src/services/claude_code_server.rs` with a loopback-only (`127.0.0.1`) HTTP/1.1 server supporting `POST /hooks/claude-code`, `GET /health`, `Content-Length`, `Transfer-Encoding: chunked`, and `Expect: 100-continue`, plus unit and localhost integration tests; register both services in `src-tauri/src/services/mod.rs`

## 3. Settings, engine integration & Tauri commands

- [x] 3.1 Extend `Settings` in `src-tauri/src/models/types.rs` with serde-defaulted `claude_code_enabled`, `claude_code_port`, and `claude_code_timeout_minutes`; add `ClaudeCodeStatus`, `ClaudeHookInstallState`, `ClaudeCodeEvent`, `ClaudeCodeDisconnected`, and `ClaudeCodeConfigChanged` variants to `EngineEvent`, and store live Claude Code status in `AppState`
- [x] 3.2 Update `src-tauri/src/services/engine.rs` to track Claude Code sessions, prioritize active sessions over window detection when enabled, cache the latest `WindowChanged` snapshot for instant restoration on end/expiry/disable, coalesce Discord updates without blocking the `select!` loop, preserve explicit non-zero timestamps, and add engine unit tests
- [x] 3.3 Create `src-tauri/src/commands/claude_code.rs` (`get_claude_code_status`, `connect_claude_code`, `disconnect_claude_code`), notify the engine when `update_settings` changes Claude Code settings, and wire server startup and command registration in `src-tauri/src/lib.rs`

## 4. Frontend Settings integration

- [x] 4.1 Update `src/types.ts` with the new `Settings` fields and `ClaudeCodeStatus` / `ClaudeHookInstallState` types
- [x] 4.2 Create `src/hooks/useClaudeCode.ts` and `src/components/ClaudeCodeSection.tsx` (React Compiler clean: no `try/finally`, no conditionals/logical operators inside `try`), and integrate the Claude Code section into `src/pages/Settings.tsx`

## 5. Verification

- [x] 5.1 Run `cargo test` in `src-tauri/` and verify all tests pass
- [x] 5.2 Run `bun run compiler:check` and `bun run build` and verify 0 skipped functions and a clean build
- [x] 5.3 Verify end-to-end hook HTTP request handling and JSON config install/remove via automated tests

## 6. Documentation

- [x] 6.1 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise)
