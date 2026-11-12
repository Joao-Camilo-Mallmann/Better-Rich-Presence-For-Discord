# Spec Delta

## ADDED Requirements

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

## REMOVED Requirements

### Requirement: Discord Activity Construction
**Reason**: It set `Editing <file>` / `Workspace: <workspace>` and asset keys (`rust-logo`, `file-icon`, IDE keys) that do not exist on the shared Discord application, so Discord rendered blank images and never named the IDE. The repository has no language icons to replace them.
**Migration**: Replaced by "IDE Activity Layout": the IDE name moves to `Details`, file and workspace move to `State`, and the large image is the IDE's catalog icon URL.
