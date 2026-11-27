## 1. Icons and App Catalog

- [x] 1.1 Add 256×256 transparent `public/assets/icons/helium.png` and update `public/assets/icons/wondersharefilmora.png` from the user-provided logos
- [x] 1.2 Update `scripts/icon-sources.json` to include `helium` and `wondersharefilmora` under `keep`
- [x] 1.3 Add `helium`, `chromium`, `librewolf`, `operagx`, `torbrowser`, and `safari` to `apps.json`, and expand executable aliases for existing browsers and `wondersharefilmora`

## 2. Browser Disambiguation Domain Logic

- [x] 2.1 Create `src-tauri/src/domain/browser.rs` with pure browser disambiguation (window title suffix + executable path) and multi-dash browser title subject extraction (` - `, ` — `, ` – `)
- [x] 2.2 Register `pub mod browser;` in `src-tauri/src/domain/mod.rs` and add unit tests covering Chromium forks, Firefox forks, non-browser guards, and title parsing

## 3. Watcher, Engine, and Registry Integration

- [x] 3.1 Add `exe_path: Option<String>` to `EngineEvent::WindowChanged`, Windows watcher (`watcher.rs`), and Linux detector (`services/linux/mod.rs`, `services/linux/process.rs`)
- [x] 3.2 Apply `domain::browser::disambiguate_browser` in `PresenceEngine` (`resolve_effective_name` / `resolve_window_presence`) and Windows priority scan, and update `app_registry::format_app_presence` to use `domain::browser::extract_browser_subject`
- [x] 3.3 Add engine and registry unit tests verifying end-to-end browser disambiguation and catalog icon resolution

## 4. Verification

- [x] 4.1 Run `cargo test` in `src-tauri/` and verify all tests pass
- [x] 4.2 Run `npm run build` and `bun run compiler:check` (or `npm run compiler:check`) and verify zero errors/bailouts
- [x] 4.3 Validate that every `icon_url` in `apps.json` maps to an existing PNG in `public/assets/icons/` and is listed in `scripts/icon-sources.json`

## 5. Documentation

- [x] 5.1 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (N/A — no conventions, workflows, or reference docs changed)
