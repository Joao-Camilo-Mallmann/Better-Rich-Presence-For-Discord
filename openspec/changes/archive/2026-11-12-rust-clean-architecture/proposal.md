# Proposal

## Why

The Rust backend mixes layers: `models/types.rs` holds both pure entities and the Tauri-bound `AppState`; the presence engine calls Tauri, the Discord thread and `reqwest` directly; OS detection, persistence and business rules share the `services/` folder. Business rules are hard to test without a running Tauri app, and every new platform (Linux just landed) adds more coupling. `.agents/AGENTS.md` now sets Clean Architecture as the target for `src-tauri/src/`; this change performs that migration. It also finishes the English-only rule: the backend still shows Portuguese text in the tray menu and the idle presence.

## What Changes

- Reorganize `src-tauri/src/` into `domain/`, `application/`, `infrastructure/`, `interface/` and a `lib.rs` composition root, with dependencies pointing inward.
- Move pure entities and rules into `domain/`: rules and settings, presence data, the app catalog lookup and presence formatting, IDE title parsing, extension mapping, activity building, and name matching.
- Introduce application ports (traits) for activity detection, presence publishing, Discord app lookup, rule and settings persistence, and frontend events. Move the presence engine and the watcher loop into `application/`, generic over those ports.
- Move adapters into `infrastructure/`: Discord IPC, the Discord HTTP lookup, Win32 and Linux activity detection, `tauri-plugin-store` repositories, Tauri event emission, and detection status.
- Move Tauri commands and the tray menu into `interface/`. Command names, argument names, payloads and event names stay the same.
- Add an automated layering check, run by `cargo test`, that fails when `domain/` or `application/` import forbidden crates or modules.
- Translate the remaining non-English user-facing text: tray menu (`Open`, `Quit`), idle presence text (`Idle`), and the default idle message for new installs (`Away`).

Non-goals: no change to detection, presence or Discord behavior beyond the text above; no frontend changes; no new features.

## Capabilities

### New Capabilities
- `user-facing-text`: The language of text the app shows in the tray, in Discord presence, and in default settings.

### Modified Capabilities
<!-- None: the refactor keeps every existing requirement; only the text covered by `user-facing-text` changes. -->

## Impact

- **Rust backend**: every file under `src-tauri/src/` moves or is split; `models/`, `services/` and `commands/` are removed at the end.
- **Frontend**: none. The Tauri command and event contract is unchanged, so `npm run build` and the running app must behave as before.
- **Persisted data**: none. `settings.json` and `rules.json` keep their shapes; an existing saved idle message is kept as is.
- **Dependencies**: no new crates.
- **Sequencing**: starts after `linux-desktop-support` is archived, because it moves the Linux detection module that change adds.
