# Spec Delta

## MODIFIED Requirements

### Requirement: VS Code Universal Parser
The system SHALL parse active window titles to detect VS Code derived IDEs and extract relevant data for Discord Rich Presence, cleaning dirty file indicators and remote workspace suffixes and supporting both three-part and two-part workspace titles.

#### Scenario: Valid VS Code Window
- **WHEN** the active window title matches the pattern `<filename> - <workspace> - Visual Studio Code`
- **THEN** the system extracts filename and workspace accurately, and identifies the IDE as Visual Studio Code.

#### Scenario: Valid Forked IDE Window
- **WHEN** the active window title matches the pattern `<filename> - <workspace> - Cursor`
- **THEN** the system extracts filename and workspace accurately, and identifies the IDE as Cursor.

#### Scenario: Complex Filename
- **WHEN** the filename contains ` - ` (e.g., `my-file - test.rs - workspace - VSCodium`)
- **THEN** the system parses the IDE and workspace from the right side, grouping the remainder as the filename (`my-file - test.rs`).

#### Scenario: Dirty indicator and remote suffix cleanup
- **WHEN** the active window title is `● main.rs - my-app [WSL: Ubuntu] - Visual Studio Code`
- **THEN** the extracted filename is `main.rs` and the extracted workspace is `my-app`.

#### Scenario: Two-part workspace-only title
- **WHEN** the active window title is `my-app - Cursor`
- **THEN** the extracted workspace is `my-app` and filename is `None`.

#### Scenario: VS Code Insiders
- **WHEN** the active window title ends with ` - Visual Studio Code - Insiders`
- **THEN** the system identifies the IDE as Visual Studio Code.

### Requirement: IDE Activity Layout
The system SHALL construct the Discord activity for a parsed IDE window with `<IDE> · <workspace>` (or `<IDE>`) as `details`, `Editing <file> (<Language>)` (or `Editing <file>` / `Working on <workspace>` / `Active`) as `state`, the IDE's catalog icon as `large_image` with the IDE name as `large_text`, and the detected programming language icon as `small_image` with the language name as `small_text` (no `small_image` when no language is detected).

#### Scenario: Setting Activity Fields
- **WHEN** a valid VS Code window `main.rs - my-project - Visual Studio Code` is detected
- **THEN** the system sets `Details` to `Visual Studio Code · my-project`, `State` to `Editing main.rs (Rust)`, `LargeImage` to the VS Code catalog icon, `LargeText` to `Visual Studio Code`, `SmallImage` to the Rust language icon URL, and `SmallText` to `Rust`

#### Scenario: Unknown language
- **WHEN** a valid window of an IDE that has a catalog entry with `icon_url` is detected (for example Cursor) and the file has an unknown language (`test.unknown`)
- **THEN** the system sets `LargeImage` to that IDE's catalog icon
- **AND** sets no `SmallImage` and no `SmallText`

#### Scenario: VSCodium icon
- **WHEN** a valid `VSCodium` window is detected
- **THEN** the system sets `LargeImage` to the VSCodium catalog icon URL

#### Scenario: Workspace-only window
- **WHEN** the window title is `my-app - Visual Studio Code` (no file open)
- **THEN** `State` is `Working on my-app`, `LargeImage` is the VS Code icon, and there is no `SmallImage`
