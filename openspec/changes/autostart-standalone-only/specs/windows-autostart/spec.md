## ADDED Requirements

### Requirement: Autostart targets a standalone build
The Windows `Run` entry SHALL only be written or normalized to an executable that embeds the frontend: the installed executable under `%LOCALAPPDATA%\Better Rich Presence\`, or the running executable when it is not a dev build. An executable inside a Cargo `target` directory SHALL NOT be chosen as a candidate.

#### Scenario: Installed app exists
- **WHEN** autostart is enabled and the installed executable exists
- **THEN** the `Run` entry points to the installed executable with `--minimized`

#### Scenario: Release build in target directory
- **WHEN** `target\release\better-rich-presence.exe` exists and the app is not installed
- **THEN** the `target\release` executable is not selected

#### Scenario: Running a dev build
- **WHEN** the running executable is a dev build and no installed executable exists
- **THEN** the `Run` entry is not written or changed

### Requirement: Warn about a non-standalone autostart entry
When the existing `Run` entry points into a Cargo `target` directory and no standalone executable can be chosen, the app SHALL leave the entry unchanged and log a warning that names the path.

#### Scenario: Entry points to target directory
- **WHEN** the `Run` entry points to `...\target\release\better-rich-presence.exe` and the app is not installed
- **THEN** the entry stays unchanged and a warning is logged

### Requirement: Respect disabled autostart
The app SHALL NOT change a `Run` entry that the user disabled in Task Manager or MSConfig.

#### Scenario: Disabled entry
- **WHEN** the `StartupApproved` value marks the entry as disabled
- **THEN** the `Run` entry is not changed
