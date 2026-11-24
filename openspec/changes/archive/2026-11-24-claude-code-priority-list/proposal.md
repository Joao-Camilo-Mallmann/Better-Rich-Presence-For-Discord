# Proposal

## Why

An active Claude Code session always replaces the window presence, ignoring the Apps priority list. With Visual Studio Code ranked first and open, Discord still shows Claude Code. The priority list is the user's single source of truth for "what wins", so Claude Code must take a place in it like any other app.

## What Changes

- Claude Code becomes an entry in the Apps priority list: a reserved rule with `process_name` `claude_code` (the catalog id), display name `Claude Code`. It never matches a running process.
- On startup, an existing rule for the Claude Code CLI process (`claude`, `claude-code`, with or without `.exe`) is converted in place into the Claude Code entry, keeping its position and enabled state. When there is neither, the entry is inserted at the top (position 1, enabled), so current behavior is kept until the user reorders it.
- While a Claude Code session is active, the engine compares positions: when the app the window detection selected (the foreground app, or the highest-ranked running app in Max priority mode) has an enabled rule ranked **above** the Claude Code entry, that app's presence is shown; otherwise the Claude Code presence is shown.
- A disabled Claude Code entry means Claude Code sessions never replace the window presence.
- The Apps view shows the Claude Code entry with the Claude Code icon, move controls and the enable switch, without edit or delete controls; its preview line shows the live Claude Code activity (from the existing Claude Code status the frontend already receives), or `Waiting for a session`.

## Capabilities

### Modified Capabilities

- `claude-code-integration`: session precedence follows the Apps priority list instead of always overriding window detection.
- `main-window`: the Apps view lists the Claude Code entry.

## Impact

- Backend: new pure rule in `src-tauri/src/domain/claude_code.rs`; `src-tauri/src/models/presets.rs` / `src-tauri/src/lib.rs` (insert the entry on load); `src-tauri/src/services/engine.rs` (every place that checks "Claude session active" before showing the window presence); rule scanners must ignore the `claude_code` entry.
- Frontend: Apps view row for the Claude Code entry (`src/pages/` / `src/components/` Apps list), no edit/delete.
