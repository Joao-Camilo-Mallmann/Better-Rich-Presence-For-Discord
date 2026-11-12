# Spec Delta

## Purpose

Lets the user see how the app is detecting their activity on the current platform, and tells them what to install or change when detection is degraded.

## ADDED Requirements

### Requirement: Platform status query
The system SHALL expose the current platform status to the frontend: operating system, session type, desktop environment, the active window-detection backend, the active idle backend, and an optional setup hint.

#### Scenario: Linux GNOME Wayland with extension
- **WHEN** the frontend requests the platform status on GNOME Wayland with the Focused Window D-Bus extension enabled
- **THEN** the response names `linux`, `wayland`, `GNOME`, the Focused Window D-Bus backend, the GNOME idle backend, and no hint

#### Scenario: Windows
- **WHEN** the frontend requests the platform status on Windows
- **THEN** the response names `windows`, a Win32 window backend, a Win32 idle backend, and no hint

### Requirement: Degraded detection hint
When window detection on Linux falls back to process scan, or idle detection is unavailable, the platform status SHALL include a hint that names the missing component and how to enable it.

#### Scenario: GNOME Wayland without extension
- **WHEN** the session is GNOME Wayland and no supported extension answers
- **THEN** the hint tells the user to install and enable the "Focused Window D-Bus" GNOME Shell extension, and that until then only rule apps are detected

#### Scenario: Unsupported Wayland compositor
- **WHEN** the session is Wayland on a compositor without a supported backend, such as KDE Plasma
- **THEN** the hint says focused-window detection is not supported on that desktop and that only rule apps are detected

### Requirement: Settings notice
The Settings page SHALL show the detection backends and, when a hint is present, a visible notice with the hint text.

#### Scenario: Hint shown
- **WHEN** the platform status contains a hint
- **THEN** the Settings page shows a notice with that hint above the settings list

#### Scenario: Healthy detection
- **WHEN** the platform status contains no hint
- **THEN** the Settings page shows the active backends without a warning notice

### Requirement: Platform-neutral autostart label
The autostart setting SHALL be labelled without naming a specific operating system.

#### Scenario: Label on Linux
- **WHEN** the user opens Settings on Linux
- **THEN** the autostart toggle reads "Start on login"
