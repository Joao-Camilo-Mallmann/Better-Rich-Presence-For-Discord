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

### Requirement: Session Timestamp Tracking
The system SHALL track elapsed time per workspace and reset it when the workspace changes.

#### Scenario: Workspace Change
- **WHEN** the user switches to a window with a different parsed workspace name
- **THEN** the timestamp is reset to the current time.

#### Scenario: File Change within Workspace
- **WHEN** the user switches to a window with a different file name but the same workspace name
- **THEN** the timestamp is preserved.

### Requirement: IDE Activity Layout
The system SHALL construct the Discord activity for a parsed IDE window with the IDE as the app: `details` is the IDE name, `state` describes the file and workspace, and the large image is the IDE's catalog icon URL. No small image is set.

#### Scenario: Setting Activity Fields
- **WHEN** a valid VS Code window `main.rs - my-project - Visual Studio Code` is detected
- **THEN** the system sets `Details` to `Visual Studio Code`, `State` to `Editing main.rs in my-project`, and the large image text to `Visual Studio Code`

#### Scenario: IDE Icon
- **WHEN** a valid window of an IDE that has a catalog entry with `icon_url` is detected (for example Cursor)
- **THEN** the system sets the `LargeImage` to that catalog icon URL
- **AND** sets no `SmallImage`

#### Scenario: IDE Without Catalog Icon
- **WHEN** a valid window of an IDE without a catalog icon is detected (for example VSCodium)
- **THEN** the system sets the `LargeImage` to the Better Rich Presence logo URL
