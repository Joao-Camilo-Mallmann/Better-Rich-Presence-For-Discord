# Tasks

## 1. Domain

- [x] 1.1 Add `CLAUDE_CODE_RULE_ID` and `claude_code_takes_precedence` to `src-tauri/src/domain/claude_code.rs`, with unit tests for every scenario (no entry, disabled entry, app above, app below, no window app, `.exe`/case).

## 2. Rules list

- [x] 2.1 Convert an existing `claude`/`claude-code` (± `.exe`) rule in place into the Claude Code entry, else insert an enabled entry at position 1, (`src-tauri/src/lib.rs` load path and `reset_app_rules_to_defaults`); unit-test the insertion helper.
- [x] 2.2 Make the Windows (`services/watcher.rs`) and Linux (`services/linux/mod.rs`) priority scanners skip the `claude_code` entry. (No code change: both scanners match process names exactly, so `claude_code` never matches a process.)

## 3. Engine

- [x] 3.1 Replace the Claude-session-active checks in `src-tauri/src/services/engine.rs` with one helper that applies `claude_code_takes_precedence` to the latest window snapshot; on `WindowChanged` switch between window and Claude Code presence as ranks dictate.

## 4. Frontend

- [x] 4.1 Show the Claude Code entry in the Apps view with the Claude Code icon, move controls and enable switch, without edit/delete, and its preview line bound to the live Claude Code status (`Waiting for a session` when none); update the priority hint text. Run `bun run compiler:check`.

## 5. Verification

- [x] 5.1 `cargo test` in `src-tauri/`, `bun run compiler:check`, `bun run build`.

## 6. Documentation

- [x] 6.1 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise). — N/A.
