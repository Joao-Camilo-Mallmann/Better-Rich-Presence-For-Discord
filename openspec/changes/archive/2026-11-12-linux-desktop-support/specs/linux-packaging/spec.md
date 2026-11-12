# Spec Delta

## Purpose

Covers how the app is built and installed on Linux, and how it integrates with the Linux desktop session (login autostart and the system tray).

## ADDED Requirements

### Requirement: Linux bundles
A release build on Linux SHALL produce a `.deb` package, an `.rpm` package and an AppImage, without changing the Windows NSIS installer produced on Windows.

#### Scenario: Build on Linux
- **WHEN** a maintainer runs the Tauri build on Linux
- **THEN** the bundle output contains a `.deb`, an `.rpm` and an `.AppImage`

#### Scenario: Build on Windows
- **WHEN** a maintainer runs the Tauri build on Windows
- **THEN** the bundle output is the per-user NSIS installer, as before

### Requirement: Desktop entry
Linux bundles SHALL install a desktop entry with the app name, icon and a utility category, so the app appears in the desktop's application launcher.

#### Scenario: Installed from .deb
- **WHEN** the user installs the `.deb` package
- **THEN** "Better Rich Presence" appears in the application launcher with its icon

### Requirement: Login autostart on Linux
When the user enables "Start on login" on Linux, the system SHALL register an XDG autostart entry that launches the app with the minimized flag, and SHALL remove it when the setting is disabled.

#### Scenario: Enable autostart
- **WHEN** the user enables "Start on login" on Linux
- **THEN** an autostart entry for the app exists in the user's XDG autostart directory

### Requirement: Start minimized
When launched with the minimized flag, the system SHALL start with its main window hidden and keep running in the tray.

#### Scenario: Launched at login
- **WHEN** the app starts with `--minimized`
- **THEN** the main window is not shown and presence detection runs

### Requirement: Tray access on Linux
On Linux the tray menu SHALL offer actions to open the main window and to quit, since tray click events are not delivered on Linux.

#### Scenario: Reopen from tray
- **WHEN** the main window is hidden and the user chooses the open action in the tray menu
- **THEN** the main window is shown and focused
