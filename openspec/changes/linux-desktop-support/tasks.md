# Tasks

## 1. Shared engine and watcher plumbing

- [x] 1.1 Add `aliases: Vec<String>` to `EngineEvent::WindowChanged` in `models/types.rs`, send an empty list from the Windows watcher path, and verify the crate still type-checks (`cargo check` in `src-tauri/`)
- [x] 1.2 Resolve an effective process name in `services/engine.rs` (first of `[process_name, aliases…]` matching an enabled rule, else the catalog, else `process_name`) in a pure helper, and verify with unit tests for rule match, alias catalog match and no match (`cargo test` in `src-tauri/`)
- [ ] 1.3 Fix idle reporting in `services/watcher.rs` to send `IdleChanged` on every idle-minute change and on return to activity, update the `EngineEvent::IdleChanged` doc comment, and verify a threshold of 5 minutes triggers in the running app
- [x] 1.4 Add a cross-platform `DetectionStatus` store (window backend, idle backend, hint) written by the watcher, with the Windows path reporting `win32` backends, and verify with a unit test that a write is read back

## 2. Linux parsers (pure, unit-tested)

- [x] 2.1 Create `services/linux/` (`mod.rs`, `parsers.rs`, `process.rs`, `backends.rs`) replacing the inline `linux` module in `watcher.rs`, and verify `watcher.rs` no longer contains Linux-specific code (`cargo check`)
- [x] 2.2 Implement parsers for `busctl --json=short` replies (string payload, `t` payload), Window Calls `List` JSON (focused entry, title optional), Focused Window D-Bus `Get` JSON, `hyprctl activewindow -j` / `clients -j`, the Sway tree (focused node, `app_id` or X11 class), and `xprop` output (`_NET_ACTIVE_WINDOW`, `_NET_CLIENT_LIST`, `_NET_WM_PID`, `_NET_WM_NAME`, `WM_CLASS`), and verify each with unit tests using captured sample outputs (`cargo test`)
- [x] 2.3 Implement D-Bus error classification (`ServiceUnknown`, `UnknownObject`, `UnknownMethod`, `UnknownInterface` → unavailable; anything else → no window) and verify with unit tests

## 3. Linux detection backends

- [x] 3.1 Implement process identity (`/proc/<pid>/exe` file name, `comm` fallback, lowercase) and alias building (window class/app id, `comm`, deduplicated), and verify with unit tests for name extraction and alias dedup
- [ ] 3.2 Implement window backends (Hyprland, Sway, Window Calls with `GetTitle` fallback, Focused Window D-Bus, X11) returning window / no window / unavailable, with self exclusion by PID, and verify on the GNOME Wayland desktop that the Focused Window D-Bus and Window Calls backends each report the focused app
- [x] 3.3 Implement environment-based backend selection (decision 2: never X11 on Wayland), caching, re-detection after 3 unavailable results and every 60 s in process-scan mode, and verify with unit tests for the candidate order per environment
- [x] 3.4 Implement the process scan (PID name cache, `.exe`-insensitive rule match, lowest rule index wins) used by the fallback and Max Priority Mode, with titles from the backend window list when available, and verify with unit tests for rule selection
- [x] 3.5 Implement idle backends (Mutter IdleMonitor via `busctl`, `xprintidle` on X11, none otherwise) and verify on GNOME that idle time grows while the input is untouched
- [ ] 3.6 Wire the Linux detector into `watcher.rs` (focused window, priority mode, idle, detection status and hints from decision 7), and verify in the running app on GNOME Wayland with and without the Focused Window D-Bus extension

## 4. Commands and frontend

- [ ] 4.1 Add the `get_platform_status` command (OS, session type, desktop, backends, hint), register it in `lib.rs`, and verify it returns the expected values from the running app's devtools (`invoke("get_platform_status")`)
- [x] 4.2 Make `get_running_processes` Linux-aware (current user only, with an executable, named by executable file name, daemon filter from decision 8), keep the Windows branch unchanged, and verify the picker on Linux lists `firefox` and no `kworker/*` entries
- [ ] 4.3 Add the `PlatformStatus` type to `src/types.ts`, show the detection backends and the hint notice on the Settings page (fetched on mount and every 10 s), and verify `npm run build` passes and the notice renders when a hint is present
- [ ] 4.4 Rename the autostart toggle to "Start on login", and verify the label in the running app

## 5. Packaging and desktop integration

- [x] 5.1 Add `src-tauri/tauri.linux.conf.json` (targets `deb`, `rpm`, `appimage`; category `Utility`; short description; tray library dependencies), and verify `tauri.conf.json` still targets only `nsis`
- [ ] 5.2 Hide the main window at startup when launched with `--minimized`, and verify by running the built binary with `--minimized`
- [ ] 5.3 Verify the tray menu "Abrir" action reopens the hidden window on GNOME (AppIndicator extension enabled)

## 6. Verification

- [ ] 6.1 Install the Linux build prerequisites (Rust toolchain; `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `build-essential`, `libssl-dev`, `curl`, `wget`, `file`, `libxdo-dev`) and verify `cargo --version` and `pkg-config --modversion webkit2gtk-4.1` succeed
- [x] 6.2 Run `cargo test` in `src-tauri/` and verify all tests pass
- [x] 6.3 Run `npm run build` and verify it succeeds
- [ ] 6.4 Run `npm run tauri build` on Linux and verify `.deb`, `.rpm` and `.AppImage` files are produced under `src-tauri/target/release/bundle/`
- [ ] 6.5 Manual check on GNOME Wayland with Discord running: focusing VS Code shows the IDE presence, idle after the threshold shows the idle presence, and the Settings page shows the Focused Window D-Bus backend with no hint
- [ ] 6.6 Manual check on GNOME Wayland without the extension: Settings shows the install hint, and an enabled rule app that is running is shown in Discord

## 7. Documentation

- [ ] 7.1 Add a Linux section to `README.md` / `README.en.md` / `README.pt.md` (supported desktops, the GNOME extension, optional tools, build prerequisites), and verify the documented build commands match `package.json` scripts
- [x] 7.2 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise)

## Workflow follow-up

- Archive the change with `/opsx:archive` once all tasks are done, so the three new specs merge into `openspec/specs/`.
