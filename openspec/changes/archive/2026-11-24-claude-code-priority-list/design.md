# Design: Claude Code in the Apps priority list

## Context

`PresenceEngine` sets `claude_override_active` and skips the window presence whenever `claude_tracker.active_session().is_some()`. Max priority mode (`watcher.rs`, `linux/mod.rs`) already picks the highest-ranked running rule app and reports it in the `WindowChanged` snapshot.

## Decisions

### 1. Reserved rule, not a new setting
- **Choice**: Represent Claude Code as an `AppRule` with `process_name == "claude_code"` (constant `CLAUDE_CODE_RULE_ID`), reusing ordering, enable switch, persistence and the existing move commands.
- **Rationale**: No new storage or contract; the list already is the priority model. Its `details`/`state`/`large_image` fields are unused for presence.

### 2. Pure precedence rule in the domain
- **Choice**: `claude_code_takes_precedence(rules: &[(&str, bool)], window_app: Option<&str>) -> bool` in `domain/claude_code.rs` (names and enabled flags only, `std` only). Returns `true` when there is no Claude Code entry (legacy behavior), `false` when the entry is disabled, and otherwise `true` unless `window_app` matches an enabled rule placed before the Claude Code entry. Names compare case-insensitively without a `.exe` suffix.
- **Rationale**: Unit-testable without Tauri; the engine calls it with the effective process name of `last_window_snapshot` (resolved the same way `resolve_window_presence` does).

### 3. Engine re-evaluates on both kinds of events
- **Choice**: Replace each `settings.claude_code_enabled && claude_tracker.active_session().is_some()` check with one helper that also applies the precedence rule. On `WindowChanged`, when the window app outranks Claude Code, the window presence is shown (and `claude_override_active` cleared); when focus returns to a lower-ranked app, the Claude Code presence comes back.
- **Rationale**: Both window changes and hook events can flip the winner.

### 4. Scanners skip the reserved entry
- **Choice**: The Windows and Linux priority scanners ignore `claude_code` rules.

## Risks / Trade-offs

- An IDE without a rule (detected only by window title) cannot outrank Claude Code; the user adds it to the list to rank it. → Matches "the list decides".
