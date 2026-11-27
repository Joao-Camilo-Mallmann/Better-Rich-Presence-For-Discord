## ADDED Requirements

### Requirement: Console-free Windows startup and silent minimized launch
On Windows, launching the application—including at user login with `--minimized`—SHALL NOT spawn a console (`cmd`) window and SHALL NOT flash the main window before hiding it.

#### Scenario: Launched at login with minimized flag
- **WHEN** the application starts on Windows with `--minimized`
- **THEN** no console window is opened
- **AND** the main window remains hidden while the system tray icon and presence engine run

#### Scenario: Launched normally without minimized flag
- **WHEN** the application starts without `--minimized`
- **THEN** window decorations are disabled before the main window is shown and focused

### Requirement: Windows startup registry entry formatting and MSConfig respect
When Windows login autostart is enabled, the `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run` entry named `"Better Rich Presence"` SHALL store a double-quoted executable path followed by `--minimized`, prefer an available release or installed executable over a debug binary, and preserve any user-disabled state recorded in `StartupApproved\Run` (MSConfig / Task Manager Startup Apps).

#### Scenario: Path with spaces is quoted
- **WHEN** autostart is synchronized for `C:\Users\Example User\AppData\Local\Better Rich Presence\better-rich-presence.exe`
- **THEN** the `Run` value equals `"C:\Users\Example User\AppData\Local\Better Rich Presence\better-rich-presence.exe" --minimized`

#### Scenario: Disabled in MSConfig or Task Manager
- **WHEN** the user has disabled `"Better Rich Presence"` in Task Manager Startup Apps or MSConfig (`StartupApproved\Run`)
- **THEN** synchronizing autostart does not overwrite or re-enable `StartupApproved\Run`

### Requirement: Clean Windows executable metadata
The compiled Windows executable SHALL embed `"Better Rich Presence"` as `ProductName`, `FileDescription`, and `InternalName`, and `"João Camilo Mallmann"` as `CompanyName` / bundle publisher, while preserving `"better-rich-presence"` as the Tauri application identifier.

#### Scenario: Inspecting Windows version metadata
- **WHEN** Windows or Task Manager reads `VERSIONINFO` from the built executable
- **THEN** `ProductName` and `FileDescription` read `"Better Rich Presence"` and `CompanyName` reads `"João Camilo Mallmann"`

### Requirement: Crisp application and system tray icons
The repository SHALL include a script (`scripts/generate-app-icons.mjs`) that produces trimmed, balanced app icons, a dedicated high-contrast tray icon (`src-tauri/icons/tray.png` and `32x32.png`), and a multi-frame `src-tauri/icons/icon.ico` (`16`–`256` px) readable by Windows resource compilers.

#### Scenario: Generating icons
- **WHEN** `bun run icons:app` is executed
- **THEN** `src-tauri/icons/tray.png`, `src-tauri/icons/32x32.png`, `src-tauri/icons/128x128.png`, `src-tauri/icons/icon.ico`, and `public/logo-256.png` are regenerated without error

### Requirement: Interactive system tray with live status and Share toggle
The system tray icon SHALL display a tooltip reflecting the current sharing/connection state, restore and unminimize the main window on left-click without opening the context menu, and provide a right-click menu with a disabled status row, a synchronized `"Share on Discord"` check item, `"Open Better Rich Presence"`, and `"Quit"`.

#### Scenario: Left-clicking the tray icon
- **WHEN** the user left-clicks the tray icon while the main window is hidden or minimized
- **THEN** the main window is unminimized, shown, and focused
- **AND** the context menu does not pop up

#### Scenario: Toggling Share on Discord from the tray
- **WHEN** the user clicks `"Share on Discord"` in the tray menu
- **THEN** `global_enabled` is toggled and persisted
- **AND** the tray status, tooltip, and open UI window update immediately
