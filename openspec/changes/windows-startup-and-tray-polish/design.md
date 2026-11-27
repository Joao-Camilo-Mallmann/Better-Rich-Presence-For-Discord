## Context

Better Rich Presence for Discord runs as a Tauri 2 desktop application (`src-tauri/`) with a React 19 + React Compiler frontend (`src/`). On Windows, startup, taskbar identity, and the notification-area tray icon interact directly with OS metadata and registry state:

- `src-tauri/src/main.rs` previously gated `#![windows_subsystem = "windows"]` behind `not(debug_assertions)`, so any debug build launched outside a terminal spawned a console (`cmd` / `conhost`) window.
- `tauri-plugin-autostart` (backed by `auto-launch 0.5.0`) writes `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run` using `std::env::current_exe()` without surrounding quotes. Running from `cargo tauri dev` registers `target\debug\better-rich-presence.exe --minimized`, which both opens a console window and cannot load the frontend when Vite is not running. Even installed paths contain spaces (`Better Rich Presence`).
- `src-tauri/tauri.conf.json` lacked `bundle.publisher`, causing `tauri-build` to populate `CompanyName` in PE `VERSIONINFO` with `"better-rich-presence"` (from `identifier`), which Windows Task Manager Startup Apps / MSConfig displays alongside the app name.
- `src-tauri/tauri.conf.json` created the main window with `"visible": true`, causing a visible frame flash before `setup` hid it on `--minimized`.
- `src/App.tsx` overwrote `document.title` and `<link rel="icon">` with the currently detected third-party app's name/icon, making the app unrecognizable in the Windows Taskbar and Alt-Tab.
- The system tray used a downscaled high-padding PNG, did not set `.show_menu_on_left_click(false)`, lacked a tooltip, and offered only `Open` and `Quit`.

## Goals / Non-Goals

**Goals:**
- Ensure zero console (`cmd`) window ever appears on Windows startup or launch.
- Ensure `--minimized` launches start completely hidden in the system tray without flashing a window.
- Ensure Windows `Run` registry entries are properly quoted, point to an appropriate release/installed executable when available, and preserve user-disabled state in `StartupApproved\Run` (MSConfig / Task Manager).
- Embed clean Windows PE `VERSIONINFO` (`ProductName`, `FileDescription`, `CompanyName`, `InternalName`, `OriginalFilename`) without changing the `better-rich-presence` Tauri `identifier` (so existing `%APPDATA%\better-rich-presence` settings and rules remain intact).
- Keep the native OS window title fixed to `"Better Rich Presence"`.
- Generate crisp application icons and a dedicated high-contrast small-size system tray icon (`tray.png`, `32x32.png`, multi-size `icon.ico`).
- Provide a polished tray UX: left-click restores/unminimizes the window without popping up the menu, right-click shows live status, a synced `"Share on Discord"` check item, `"Open Better Rich Presence"`, and `"Quit"`.

**Non-Goals:**
- Replacing `tauri-plugin-autostart` on Linux or macOS (Linux XDG autostart continues working as before).
- Changing the in-app header title (`"Better RPC"`).
- Modifying `identifier` or migrating Tauri store paths.

## Decisions

1. **Unconditional Windows GUI subsystem (`#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]`)**
   - *Rationale*: Prevents `conhost.exe` / Windows Terminal from opening even when a developer binary is launched directly or via autostart. Developers running `cargo run` or `bun run tauri dev` still see logs in their terminal and via the `"app-log"` event stream.
   - *Alternative considered*: Keeping `not(debug_assertions)` and relying solely on release builds — rejected because any debug launch from Explorer or Run still flashes a console window.

2. **Hidden initial window (`visible: false` in `tauri.conf.json`) + explicit show when not `--minimized`**
   - *Rationale*: Hiding inside `setup` after window creation still allows Windows to paint the initial unstyled frame briefly. Starting hidden and calling `window.show()` only when `!should_start_minimized(args)` removes the flash completely.

3. **Windows `Run` sync adapter in `src-tauri/src/infrastructure/autostart.rs` + pure rules in `domain/startup.rs`**
   - *Rationale*: Follows Clean Architecture rules (`domain/` depends only on `std`, OS registry access lives in `infrastructure/` under `#[cfg(target_os = "windows")]`). `tauri-plugin-autostart` handles initial enable/disable and Linux/macOS, while `sync_autostart_entry()` normalizes any existing Windows `Run` entry (`"C:\...\better-rich-presence.exe" --minimized`), resolves a release/installed executable when invoked from a dev binary, and checks `StartupApproved\Run` so user overrides in MSConfig / Task Manager are never clobbered.

4. **Dedicated SVG-rendered small tray icon + trimmed master icon pipeline (`scripts/generate-app-icons.mjs`)**
   - *Rationale*: Downscaling a complex dark-outlined 512px PNG to 16–24px produces muddy silhouettes on dark taskbars. Rendering a crisp, high-contrast vector mark for 16–32px frames (`tray.png`, `32x32.png`, and the small frames of `icon.ico`) while trimming excess padding on `40–256px` frames keeps both the notification area and Windows Startup Apps sharp.
   - *Format detail*: Windows `rc.exe` / `tauri-winres` requires `< 256` px `.ico` frames to be uncompressed 32-bpp BGRA DIBs with AND masks, reserving PNG compression for the `256` px frame.

5. **Fixed native OS window title + `"settings-updated"` event for tray/UI sync**
   - *Rationale*: Keeping `document.title = "Better Rich Presence"` ensures Windows Taskbar / Alt-Tab / Task Manager never masquerade as another app. Emitting `"settings-updated"` from `AppState::update_settings` keeps the frontend `useSettings` hook immediately synchronized when `"Share on Discord"` is toggled from the tray.

## Risks / Trade-offs

- [Debug builds with `windows_subsystem = "windows"` detach stdout when launched from Explorer] → Acceptable; `tauri dev` inherits the parent console / frontend logs, and `AppLogger` continues emitting `"app-log"` events.
- [Windows `MuiCache` caches old `FriendlyAppName` / `ApplicationCompany` per `.exe` path] → Fresh builds or clearing stale `MuiCache` entries on the machine ensures Task Manager / MSConfig immediately reflects the updated PE `VERSIONINFO`.
- [Toggling `enable()` from the frontend writes an unquoted path via `auto-launch`] → Calling `sync_autostart_entry()` both on startup and when `update_settings` saves with `autostart_enabled: true` guarantees the `Run` value is immediately normalized and quoted.
