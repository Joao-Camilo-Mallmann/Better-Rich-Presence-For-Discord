# Proposal

## Why

Better Rich Presence only works on Windows today. The Linux code path is a stub: it shells out to `xprop` and `xprintidle`, which see nothing on Wayland (the default session on Ubuntu, Fedora and most current distros), so on a GNOME Wayland desktop no window is ever detected and idle never fires. Linux also has no installable bundle, because the Tauri config only builds an NSIS installer.

## What Changes

- Replace the Linux watcher stub with a backend chain that detects the focused window (process, title, window class) on:
  - X11 sessions (any window manager) through EWMH properties, resolving the owning PID.
  - GNOME Wayland through the "Window Calls" or "Focused Window D-Bus" Shell extensions over D-Bus.
  - Hyprland (`hyprctl`) and Sway (`swaymsg`).
- Resolve the Linux process identity from the PID (`/proc/<pid>/exe`), with the window class as an alias, so `apps.json` entries (`code`, `cursor`, `firefox`, …) and user rules match on Linux.
- Add a process-scan fallback when no focused-window backend is available, so enabled rules still produce a presence (without the window title).
- Implement Max Priority Mode on Linux (it currently falls back to the foreground window).
- Detect idle time on Linux through the GNOME Mutter IdleMonitor D-Bus API, with `xprintidle` as the X11 fallback.
- Fix idle detection on every OS: the watcher only reported the first idle minute, so a threshold above 1 minute never triggered.
- Report which detection backends are active through a new `get_platform_status` command, and show setup guidance in Settings when Linux detection is degraded (for example, the GNOME extension is missing).
- Filter kernel threads, other users' processes and desktop daemons from the process picker on Linux, and list processes by executable name.
- Build `.deb`, `.rpm` and AppImage bundles on Linux through a platform config file, keeping the Windows NSIS build unchanged.
- Rename the "Start with Windows" setting to a platform-neutral "Start on login".

Non-goals: KDE Plasma Wayland focused-window detection (it falls back to process scan), shipping our own GNOME Shell extension, and macOS support.

## Capabilities

### New Capabilities
- `linux-activity-detection`: How the app learns the focused window, running processes and idle time on Linux, across X11 and Wayland compositors, including fallbacks.
- `detection-status`: Reporting which window and idle detection backends are active, and guiding the user when detection is degraded.
- `linux-packaging`: Producing installable Linux bundles and integrating with the Linux desktop (autostart, tray).

### Modified Capabilities
<!-- None: existing specs (IDE title parsing, assets, icons) are unchanged; they receive the same inputs on Linux. -->

## Impact

- **Rust backend**: `src-tauri/src/services/watcher.rs` (Linux module moved out, idle reporting fix), new `src-tauri/src/services/linux/` module, new `src-tauri/src/services/detection_status.rs` (`DetectionStatus`, `PlatformStatus`), `src-tauri/src/services/engine.rs` (alias matching), `src-tauri/src/models/types.rs` (`EngineEvent::WindowChanged` gains `aliases`), `src-tauri/src/commands/system.rs` (Linux process listing, `get_platform_status`), `src-tauri/src/lib.rs` (command registration, `--minimized`).
- **Frontend**: `src/pages/Settings.tsx` (detection status, autostart label), new `src/components/DetectionStatus.tsx` and `src/hooks/usePlatformStatus.ts`, `src/types.ts`.
- **Config**: new `src-tauri/tauri.linux.conf.json`.
- **Dependencies**: no new crates. At runtime the Linux build uses optional system tools when present (`busctl` from systemd, `xprop`, `xprintidle`, `hyprctl`, `swaymsg`); missing tools disable only the backend that needs them.
- **Build requirements (Linux)**: Rust toolchain plus the Tauri Linux system packages (`libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `build-essential`, `libssl-dev`).
- **Windows**: behavior is unchanged except the idle-threshold fix.
