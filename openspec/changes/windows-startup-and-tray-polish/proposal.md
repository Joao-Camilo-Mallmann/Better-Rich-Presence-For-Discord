## Why

On Windows, launching Better Rich Presence at login currently flashes a console (`cmd`) window, can leave an unquoted debug executable in the `Run` registry key, flashes the main window before hiding it, shows `better-rich-presence` as the publisher in MSConfig / Task Manager Startup Apps, and overwrites the native OS window title with whichever third-party app is detected. In addition, the system tray icon is small and blurry on dark taskbars, left-clicking it triggers both the window and the context menu, and the menu lacks live status or a quick Share toggle.

## What Changes

- Compile the Windows binary with `windows_subsystem = "windows"` across all build profiles so launching the app never spawns a console window.
- Start the main window hidden (`visible: false`) and show it only when launched without `--minimized`, eliminating startup window flashes.
- Add clean Windows executable `VERSIONINFO` metadata (`publisher`, `InternalName`, `OriginalFilename`) while preserving the `better-rich-presence` identifier and store directory.
- Add a Windows autostart synchronization helper that quotes the executable path with `--minimized`, prefers an available release/installed executable over a debug binary when launched from dev, and respects user disabled state in `StartupApproved\Run` (MSConfig / Task Manager).
- Keep the native OS window title fixed to `"Better Rich Presence"` while preserving `"Better RPC"` in the custom in-app header.
- Generate crisp app icons with reduced dead-space padding, a dedicated high-contrast system tray icon (`tray.png` / `32x32.png`), and a multi-frame `icon.ico` (`16`–`256` px).
- Upgrade the system tray with `.show_menu_on_left_click(false)`, window unminimize/restore on left-click, live tooltip and disabled status row, and a `"Share on Discord"` check menu item synced with the UI.

## Capabilities

### New Capabilities
- `system-tray-and-autostart`: Console-free Windows startup, quoted `Run` registry synchronization respecting MSConfig / Task Manager approval, clean PE metadata, crisp tray icon, and interactive tray menu with live status and Share toggle.

### Modified Capabilities
- `main-window`: Keep the native OS window title fixed to `"Better Rich Presence"` instead of tracking the detected third-party app.

## Impact

- Backend (`src-tauri/`): `main.rs`, `Cargo.toml`, `tauri.conf.json`, `lib.rs`, `domain/startup.rs`, `domain/tray.rs`, `infrastructure/autostart.rs`, `interface/tray.rs`, `commands/settings.rs`, `commands/system.rs`, `models/types.rs`, `services/engine.rs`, `services/watcher.rs`.
- Frontend (`src/`): `App.tsx`, `hooks/useSettings.ts`.
- Scripts & Assets: `scripts/generate-app-icons.mjs`, `package.json`, `src-tauri/icons/*`, `public/logo-256.png`.
