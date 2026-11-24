# Proposal

## Why

The VS Code and derived IDE integration currently displays only the IDE name as `details` and `Editing <file> in <workspace>` as `state`, with no small image badge and no programming language detection or language icons. Additionally, window titles with dirty indicators (`● `), remote suffixes (`[WSL: ...]`, `[SSH: ...]`), two-part `<workspace> - <IDE>` titles, `Visual Studio Code - Insiders`, and `VSCodium` icon support are missing or incomplete.

## What Changes

- Add 36 programming language icons and the `vscodium` icon to `scripts/icon-sources.json` and generate 256×256 dark-background-optimized PNGs in `public/assets/icons/`.
- Add a `vscodium` entry to `apps.json` and register it in `IdeRegistry`.
- Add pure language detection (`src-tauri/src/domain/language.rs`) mapping file extensions and special filenames (e.g., `Dockerfile`, `Cargo.toml`, `.gitignore`) to language display names and icon URLs.
- Update IDE Rich Presence layout so `details` shows `<IDE> · <workspace>` (or `<IDE>`), `state` shows `Editing <file> (<Language>)` / `Working on <workspace>` / `Active`, `large_image` shows the IDE icon (e.g. Visual Studio Code) with the IDE name as `large_text`, and `small_image` shows the detected language icon with the language name as `small_text` (no small image when no language matches).
- Improve `TitleParser` to handle `Visual Studio Code - Insiders`, two-part `<workspace> - <IDE>` titles, dirty file indicators (`● `, `• `), and remote workspace suffixes (`[WSL: ...]`, `[SSH: ...]`, `[Dev Container: ...]`, `[Codespaces: ...]`, `(Workspace)`).
- Resolve `small_image` in `PresenceEngine::prepare_for_discord` and format `small_text` safely in `discord.rs`.
- Add local `/assets/icons/<name>.png` fallback in `AppIcon.tsx` when remote raw GitHub icon URLs have not yet been pushed to `main`.

## Capabilities

### New Capabilities

(None)

### Modified Capabilities

- `vsc-universal-presence`: Extend title parsing (dirty markers, remote suffixes, 2-part titles, Insiders), add `VSCodium` icon, and update IDE activity layout to include project in `details` and IDE large image + language small-image badge.
- `presence-display`: Update IDE window scenario and add `small_image` resolution rules on the shared client.

## Impact

- Assets & Catalog: `scripts/icon-sources.json`, `public/assets/icons/*.png`, `apps.json`.
- Backend (`src-tauri/src/`): new `domain/language.rs`, `domain/mod.rs`, `domain/presence.rs`, `services/ide_registry.rs`, `services/title_parser.rs`, `services/activity_builder.rs`, `services/engine.rs`, `services/discord.rs`.
- Frontend (`src/`): `components/AppIcon.tsx`.
