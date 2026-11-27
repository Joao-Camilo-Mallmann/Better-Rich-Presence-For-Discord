## 1. Icons & Asset Generation

- [x] 1.1 Create `scripts/generate-app-icons.mjs` and add `"icons:app"` script in `package.json` to generate trimmed app icons, dedicated crisp `tray.png` / `32x32.png`, and multi-frame `icon.ico` (`16`–`256` px)
- [x] 1.2 Run `bun run icons:app` and visually verify `tray.png`, `32x32.png`, and `128x128.png`

## 2. Windows Binary, Metadata & Silent Window Launch

- [x] 2.1 Set `#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]` in `src-tauri/src/main.rs` so no console window opens
- [x] 2.2 Update `src-tauri/tauri.conf.json` (`visible: false`, `bundle.publisher`, descriptions) and `src-tauri/Cargo.toml` (`winreg` on Windows, `[package.metadata.tauri-winres]`)
- [x] 2.3 Ignore `"better rich presence.exe"` alongside existing executable names in `src-tauri/src/services/watcher.rs` and `src-tauri/src/commands/system.rs`

## 3. Clean Architecture Startup & System Tray

- [x] 3.1 Add `src-tauri/src/domain/startup.rs` and `src-tauri/src/domain/tray.rs` with unit tests and export them from `src-tauri/src/domain/mod.rs`
- [x] 3.2 Add `src-tauri/src/infrastructure/autostart.rs` (`sync_autostart_entry`) and `src-tauri/src/infrastructure/mod.rs`
- [x] 3.3 Add `src-tauri/src/interface/tray.rs` (`setup_tray`, `refresh_tray_from_state`) and `src-tauri/src/interface/mod.rs`
- [x] 3.4 Wire startup visibility, autostart sync, tray setup/refresh, and `"settings-updated"` emission into `src-tauri/src/lib.rs`, `src-tauri/src/commands/settings.rs`, `src-tauri/src/models/types.rs`, and `src-tauri/src/services/engine.rs`

## 4. Frontend Title & Settings Sync

- [x] 4.1 Keep `document.title` fixed to `"Better Rich Presence"` in `src/App.tsx`
- [x] 4.2 Listen for `"settings-updated"` in `src/hooks/useSettings.ts` while remaining React Compiler compliant

## 5. Verification

- [x] 5.1 Run `bun run compiler:check` and `bun run build`
- [x] 5.2 Run `cargo test --manifest-path src-tauri/Cargo.toml` and `cargo build --release --manifest-path src-tauri/Cargo.toml`
- [x] 5.3 Verify release `.exe` PE subsystem is `2` (Windows GUI) and `VersionInfo` reports `"Better Rich Presence"` / `"João Camilo Mallmann"`
- [x] 5.4 Run `openspec validate "windows-startup-and-tray-polish"`
- [x] 5.5 Repair active `HKCU\...\Run` entry to point to the quoted release `.exe --minimized` and clear stale `MuiCache` entries for `better-rich-presence.exe`

## 6. Documentation

- [x] 6.1 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise)
