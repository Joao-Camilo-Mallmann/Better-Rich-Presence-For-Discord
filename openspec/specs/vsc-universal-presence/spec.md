# Capability: VSC Universal Presence

## Purpose
TBD

## Requirements

### Requirement: VS Code Universal Parser
The system SHALL parse active window titles to detect VS Code derived IDEs and extract relevant data for Discord Rich Presence.

#### Scenario: Valid VS Code Window
- **WHEN** the active window title matches the pattern `<filename> - <workspace> - Visual Studio Code`
- **THEN** the system extracts filename and workspace accurately, and identifies the IDE as Visual Studio Code.

#### Scenario: Valid Forked IDE Window
- **WHEN** the active window title matches the pattern `<filename> - <workspace> - Cursor`
- **THEN** the system extracts filename and workspace accurately, and identifies the IDE as Cursor.

#### Scenario: Complex Filename
- **WHEN** the filename contains ` - ` (e.g., `my-file - test.rs - workspace - VSCodium`)
- **THEN** the system parses the IDE and workspace from the right side, grouping the remainder as the filename (`my-file - test.rs`).

### Requirement: Discord Activity Construction
The system SHALL construct a Discord Activity payload using the parsed window title data and set appropriate assets based on file extensions.

#### Scenario: Setting Activity Fields
- **WHEN** a valid VS Code window is detected
- **THEN** the system sets `Details` to `Editing <filename>`, `State` to `Workspace: <workspace>`, and updates the `SmallImage` to the corresponding IDE icon.

#### Scenario: File Extension Asset Mapping
- **WHEN** the parsed filename is `main.rs`
- **THEN** the system sets the `LargeImage` asset key to `rust-logo`.

#### Scenario: Unknown File Extension
- **WHEN** the parsed filename has an unknown extension (e.g., `.unknown`)
- **THEN** the system sets the `LargeImage` asset key to `file-icon`.

### Requirement: Session Timestamp Tracking
The system SHALL track elapsed time per workspace and reset it when the workspace changes.

#### Scenario: Workspace Change
- **WHEN** the user switches to a window with a different parsed workspace name
- **THEN** the timestamp is reset to the current time.

#### Scenario: File Change within Workspace
- **WHEN** the user switches to a window with a different file name but the same workspace name
- **THEN** the timestamp is preserved.
