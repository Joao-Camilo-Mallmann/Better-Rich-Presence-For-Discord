# Spec Delta

## Purpose

Defines how the app learns which application the user is focused on, which applications are running, and how long the user has been idle on Linux, across X11 and Wayland desktops.

## ADDED Requirements

### Requirement: Focused window detection on X11
On an X11 session the system SHALL report the focused window's owning process, title and window class, using the window manager's active-window hints.

#### Scenario: Focused editor on X11
- **WHEN** the session is X11 and the user focuses a VS Code window titled `main.rs - app - Visual Studio Code`
- **THEN** within one polling interval the engine receives process `code` and title `main.rs - app - Visual Studio Code`

#### Scenario: No focused client
- **WHEN** the session is X11 and the active-window hint is empty or points to a window without a class
- **THEN** the system reports no focused window

### Requirement: Focused window detection on GNOME Wayland
On a GNOME Wayland session the system SHALL read the focused window from a supported GNOME Shell extension over the session bus. Supported extensions are "Focused Window D-Bus" (preferred) and "Window Calls", including Window Calls versions that report focus only per window.

#### Scenario: Focused Window D-Bus extension installed
- **WHEN** the session is GNOME on Wayland and the Focused Window D-Bus extension is enabled
- **THEN** the system reports the focused window's process, title and window class

#### Scenario: Current Window Calls version installed
- **WHEN** the session is GNOME on Wayland, only Window Calls is enabled, and its window list carries no focus or title fields
- **THEN** the system finds the focused window through the per-window details and reports its process, title and window class

#### Scenario: No supported extension
- **WHEN** the session is GNOME on Wayland and neither supported extension answers on the session bus
- **THEN** the system does not use X11 active-window hints and switches to process-scan detection

### Requirement: Focused window detection on wlroots compositors
On Hyprland and Sway sessions the system SHALL read the focused window from the compositor's IPC.

#### Scenario: Hyprland session
- **WHEN** the session runs Hyprland and its IPC tool answers
- **THEN** the system reports the active window's process, title and class

#### Scenario: Sway session
- **WHEN** the session runs Sway and its IPC tool answers
- **THEN** the system reports the focused node's process, title and app id

### Requirement: Untrusted X11 hints on Wayland
On a Wayland session the system SHALL NOT use X11 active-window hints, because they only describe XWayland windows and go stale when a native Wayland window is focused.

#### Scenario: Stale XWayland hint
- **WHEN** the session is Wayland, no Wayland backend answers, and the X11 root window still names an XWayland window as active
- **THEN** the system ignores that hint and uses process-scan detection

### Requirement: Linux process identity
On Linux the system SHALL identify a window's application by the lowercase file name of the owning process's executable, falling back to the kernel process name when the executable cannot be read. The window class SHALL be offered as an alias for matching.

#### Scenario: Executable name is used
- **WHEN** the focused window belongs to a process whose executable is `/usr/share/code/code`
- **THEN** the reported process name is `code`

#### Scenario: Long process name
- **WHEN** the owning process's kernel name is truncated to 15 characters but its executable file name is longer
- **THEN** the reported process name is the full executable file name

#### Scenario: Alias match through window class
- **WHEN** the executable name matches no rule or catalog entry, but the window class `discord` matches a catalog entry
- **THEN** the engine resolves the application through the `discord` alias

### Requirement: Self exclusion
The system SHALL NOT report its own windows as the focused application.

#### Scenario: App window focused
- **WHEN** the user focuses the Better Rich Presence window on Linux
- **THEN** the system reports no focused window

### Requirement: Process-scan fallback
When no focused-window backend is available, the system SHALL select the highest-priority enabled rule whose process is running and report it without a window title.

#### Scenario: Rule app running without focus information
- **WHEN** no focused-window backend is available and a running process matches the enabled rule `code`
- **THEN** the engine receives process `code` with the process name as the title

#### Scenario: No matching rule
- **WHEN** no focused-window backend is available and no running process matches an enabled rule
- **THEN** the system reports no focused window

### Requirement: Backend re-detection
The system SHALL retry backend detection periodically while it is using the process-scan fallback, and SHALL re-detect when the active backend stops answering.

#### Scenario: Extension enabled after start
- **WHEN** the app started in process-scan mode on GNOME Wayland and the user then enables a supported extension
- **THEN** within one minute the system switches to the extension backend without a restart

### Requirement: Max Priority Mode on Linux
When Max Priority Mode is enabled on Linux, the system SHALL report the highest-priority running rule application, with its window title when a backend can list windows, and SHALL flag it as prioritized when it is not the focused application.

#### Scenario: Background rule app outranks focused app
- **WHEN** Max Priority Mode is enabled, rule `code` is running in the background, and the user focuses Firefox
- **THEN** the engine receives process `code`, a prioritized flag, and `firefox` as the foreground app

#### Scenario: No running rule app
- **WHEN** Max Priority Mode is enabled and no running process matches an enabled rule
- **THEN** the system reports the focused window as in normal mode

### Requirement: Idle time on Linux
On Linux the system SHALL measure user idle time through the GNOME Mutter idle monitor when available, and through `xprintidle` on X11 otherwise. When neither is available, the user SHALL be treated as never idle.

#### Scenario: GNOME session
- **WHEN** the session is GNOME (X11 or Wayland) and the user has not touched input for 6 minutes
- **THEN** the measured idle time is at least 6 minutes

#### Scenario: No idle source
- **WHEN** no idle source is available
- **THEN** the system never switches to the idle presence

### Requirement: Idle threshold reporting
The system SHALL report idle time to the engine at least once per idle minute, so any configured idle threshold can trigger the idle presence, and SHALL report activity as soon as input resumes.

#### Scenario: Threshold above one minute
- **WHEN** the idle threshold is 5 minutes and the user stays idle for 5 minutes
- **THEN** the idle presence is shown

#### Scenario: User returns
- **WHEN** the idle presence is shown and the user moves the mouse
- **THEN** the previous presence is restored within one polling interval

### Requirement: Running process list on Linux
On Linux the running-process list offered to the user SHALL contain only the current user's processes that have an executable, named by executable file name, excluding kernel threads, the app itself, and common desktop daemons.

#### Scenario: Kernel threads hidden
- **WHEN** the user opens the process picker on Linux
- **THEN** kernel threads such as `kworker/0:1` and other users' processes are not listed

#### Scenario: User application listed
- **WHEN** Firefox is running for the current user
- **THEN** the picker lists `firefox`
