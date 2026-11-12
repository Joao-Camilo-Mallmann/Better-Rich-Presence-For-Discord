# Design

## Context

See `proposal.md` for motivation. Requirements are in `specs/linux-activity-detection`, `specs/detection-status` and `specs/linux-packaging`.

Current state:

- `services/watcher.rs` polls every 2 s. Windows has a `win32` module (foreground window, `EnumWindows` for priority mode, `GetLastInputInfo` for idle). Linux has a `linux` module that shells out to `xprop` and `xprintidle`, and its priority mode just falls back to the foreground window.
- On GNOME Wayland (the maintainer's desktop, GNOME Shell 50), `xprop -root _NET_ACTIVE_WINDOW` returns an XWayland window with no `WM_CLASS`, so nothing is ever detected. `org.gnome.Shell.Eval` is disabled. `org.gnome.Mutter.IdleMonitor` works.
- The engine matches `process_name` against user rules and `apps.json` after stripping `.exe`. `apps.json` already lists Linux executable names (`code`, `cursor`, `firefox`, …).
- Discord IPC needs no work: `discord-presence` 3.2 already searches `$XDG_RUNTIME_DIR` (then `TMPDIR`, `/tmp`) for `discord-ipc-0..9`, including the Flatpak (`app/com.discordapp.Discord/`) and Snap (`snap.discord/`, `snap.discord-canary/`) subdirectories.
- The watcher sends `IdleChanged` only when the idle boolean flips, which happens after the first idle minute, so a threshold above 1 minute never fires on any OS.

## Goals / Non-Goals

**Goals:**
- One Linux build that picks the right detection backend at runtime, with no per-desktop build flags.
- Pure, unit-tested parsers for every external tool's output.
- No new crates.

**Non-Goals:**
- KDE Plasma Wayland focused-window detection (needs a KWin script; falls back to process scan).
- Shipping or auto-installing a GNOME Shell extension.
- Non-systemd distros on GNOME Wayland (no `busctl`; they fall back to process scan).
- Changing Windows detection beyond the shared idle-reporting fix and start-minimized handling.

## Decisions

### 1. External tools with JSON output instead of D-Bus/X11 crates

Each backend shells out to a tool and parses its output:

| Backend | Tool and call |
|---|---|
| Hyprland | `hyprctl activewindow -j`, `hyprctl clients -j` |
| Sway | `swaymsg -t get_tree -r` |
| GNOME Focused Window D-Bus | `busctl --user --json=short call org.gnome.Shell /org/gnome/shell/extensions/FocusedWindow org.gnome.shell.extensions.FocusedWindow Get` (full window object, one call) |
| GNOME Window Calls | `busctl --user --json=short call org.gnome.Shell /org/gnome/Shell/Extensions/Windows org.gnome.Shell.Extensions.Windows List`; current versions omit `focus` and `title` from `List`, so the focused window is found with `Details u <id>` (last focused window first) and its title with `GetTitle u <id>` |
| X11 | `xprop -root _NET_ACTIVE_WINDOW _NET_CLIENT_LIST`, `xprop -id <id> _NET_WM_PID _NET_WM_NAME WM_CLASS` |
| Idle (GNOME) | `busctl --user call org.gnome.Mutter.IdleMonitor /org/gnome/Mutter/IdleMonitor/Core org.gnome.Mutter.IdleMonitor GetIdletime --json=short` |
| Idle (X11) | `xprintidle` |

`busctl --json=short` returns `{"type":"s","data":["<json>"]}`, so the GNOME path is JSON inside JSON and needs no GVariant text parsing.

Alternatives considered:
- `zbus` for D-Bus and `x11rb` for X11: typed and spawn-free, but adds a large dependency tree for an optional path, and its blocking/async APIs need care inside the Tokio watcher. Revisit if spawn cost shows up in profiling.
- `gdbus call`: always present on GNOME, but prints GVariant text that needs a custom unescaper.

Cost: at most three short-lived processes per 2 s tick (focused window, optional title, idle).

### 2. Backend selection from the session environment

At watcher start, and again on re-detection, candidates are tried in this order; the first whose call succeeds is cached:

1. `HYPRLAND_INSTANCE_SIGNATURE` set → Hyprland.
2. `SWAYSOCK` set → Sway.
3. Wayland session and `XDG_CURRENT_DESKTOP` contains `GNOME` → Focused Window D-Bus (one call per tick), then Window Calls (one `List` plus `Details` calls).
4. X11 session (`XDG_SESSION_TYPE=x11`, or `DISPLAY` set without `WAYLAND_DISPLAY`) → X11.
5. Otherwise → process scan.

X11 is never a candidate on Wayland (spec: "Untrusted X11 hints on Wayland").

A backend call returns one of: a window, no window (nothing focused, lock screen), or *unavailable* (tool missing, or a D-Bus `ServiceUnknown` / `UnknownObject` / `UnknownMethod` / `UnknownInterface` error). Only *unavailable* counts toward re-detection: after 3 consecutive unavailable results the watcher re-detects. In process-scan mode it re-detects every 30 ticks (60 s), which satisfies "Extension enabled after start".

### 3. Process identity and aliases

For a window PID, the process name is the lowercase file name of `readlink /proc/<pid>/exe`, falling back to `/proc/<pid>/comm`. This avoids the 15-character `comm` truncation and works for Snap and Flatpak apps (the link resolves inside the app's mount namespace, e.g. `/app/discord/Discord` → `discord`).

`EngineEvent::WindowChanged` gains `aliases: Vec<String>`: the lowercase window class / app id and the `comm` name, deduplicated and excluding the primary name. The engine resolves an *effective name*: the first of `[process_name, aliases…]` that matches an enabled rule, else the first that matches an `apps.json` entry, else `process_name`. Everything after that point in the engine uses the effective name, so rule matching, catalog lookup and title formatting are unchanged. Windows sends an empty list.

Alternative considered: resolve the name inside the watcher. Rejected because matching against rules and the catalog already lives in the engine; duplicating it would let them drift.

### 4. Process scan shared by fallback and Max Priority Mode

One Linux scan function refreshes the process table (`sysinfo`, names only), resolves each PID's name as in decision 3 (cached by PID, pruned each scan), and returns the enabled rule with the lowest index whose name matches. Rule names are compared after stripping `.exe`, so a rule created on Windows (`code.exe`) also matches on Linux.

- Fallback mode: the selected app is reported with its process name as title.
- Max Priority Mode: the title comes from the backend's window list (X11 `_NET_CLIENT_LIST`, Window Calls `List`, `hyprctl clients`, Sway tree) by PID, falling back to the process name; the prioritized flag and foreground app follow the Windows semantics.

### 5. Self exclusion by PID

On Linux the watcher drops any window whose PID equals `std::process::id()`, instead of comparing names. This works regardless of the installed binary name.

### 6. Idle reporting fix

The watcher sends `IdleChanged { idle: true, idle_minutes }` whenever the idle minute count changes while idle, and `IdleChanged { idle: false, 0 }` on the transition back to active. The engine already ignores repeated idle events once idle, so it needs no change. The `EngineEvent` doc comment is updated to match.

### 7. Detection status

A new `services/detection_status.rs` holds `DetectionStatus { window_backend, idle_backend, hint }` in a module-level `std::sync::RwLock`; the watcher (Windows) and the Linux detector write it on every (re)detection. A new `get_platform_status` command combines it with `std::env::consts::OS`, `XDG_SESSION_TYPE` and `XDG_CURRENT_DESKTOP`. The Settings page calls it on mount and every 10 s while open; no event is needed because the status changes rarely.

Hint text is chosen in Rust, so the frontend only renders it:

- GNOME Wayland, no extension → install and enable "Focused Window D-Bus" (extensions.gnome.org/extension/5592/focused-window-d-bus); until then only rule apps are detected. Window Calls (extension 4724) is also supported.
- Other Wayland compositor → focused-window detection is not supported on this desktop; only rule apps are detected.
- X11 without `xprop` → install `x11-utils` (`xprop`).
- No idle backend → idle detection is unavailable on this desktop (on X11, install `xprintidle`).

### 8. Linux process picker

On Linux, `get_running_processes` reads `/proc` directly: it keeps only entries owned by the current user (the owner of `/proc/<pid>`) whose `/proc/<pid>/exe` can be read (kernel threads have none), names them by executable file name, and drops desktop daemons by exact name (`systemd`, shells, `wireplumber`, `gnome-shell`, `gnome-session-binary`, `gnome-keyring-daemon`, `better-rich-presence`, …) or prefix (`dbus-`, `pipewire`, `gsd-`, `gvfs`, `xdg-`, `at-spi`, `ibus`, `evolution-`, `tracker-`, `snapd`, `flatpak-`, …). GNOME daemons are listed by exact name, not a `gnome-` prefix, so apps such as `gnome-text-editor` stay pickable. The Windows branch is unchanged.

### 9. Packaging through a platform config

`src-tauri/tauri.linux.conf.json` is merged by the Tauri CLI on Linux only (JSON merge patch; arrays are replaced), so `tauri.conf.json` keeps `"targets": ["nsis"]` for Windows. The Linux file sets `bundle.targets` to `deb`, `rpm`, `appimage`, `bundle.category` to `Utility`, a short description, and the tray runtime library as a dependency (`libayatana-appindicator3-1` for deb, `libayatana-appindicator-gtk3` for rpm).

### 10. Start minimized

`setup` hides the main window when the process arguments contain `--minimized` (the flag the autostart plugin already passes). This applies to every OS and makes the existing "Open minimized on boot" description true.

## Risks / Trade-offs

- [GNOME Wayland needs a third-party extension] → Clear hint in Settings with the extension name and URL; process-scan fallback keeps rule apps working without it.
- [Window Calls API drift: current versions drop `focus` and `title` from `List`] → Use `focus`/`title` when present (older versions), otherwise `Details(id)` per window starting with the last focused one, and `GetTitle(id)`. With nothing focused this costs one `Details` call per window per tick, which is why Focused Window D-Bus is tried first and recommended.
- [Tray icon needs the AppIndicator extension on GNOME; without it, closing the window leaves no way to reopen it] → Ubuntu enables it by default; deb/rpm declare the library; a single-instance "show window on relaunch" is a follow-up.
- [No compile or runtime check is possible on a machine without the Rust toolchain and Tauri system packages] → Verification tasks list the exact packages; parsers are pure functions covered by `cargo test`.
- [Process spawn cost] → Backend cached, at most three spawns per tick, PID name cache in scans.
- [Process scan reports rule apps that are merely running] → Only used when focus is unknown, and only for rules the user enabled.

## Migration Plan

No data migration: settings and rules are unchanged, and Windows-created rules (`code.exe`) match Linux processes because `.exe` is stripped on both sides. Rollback is a revert of the change.

## Open Questions

- Whether to ship a first-party GNOME Shell extension later, to drop the third-party dependency.
- KDE Plasma Wayland support through a KWin script loaded over D-Bus.
