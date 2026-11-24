# Spec Delta

## ADDED Requirements

### Requirement: Small image resolution on the shared client
On the shared client, `small_image` SHALL resolve HTTP(S) URLs via `discord_image_url`, resolve catalog IDs or executables to their catalog icon URL, and resolve empty, placeholder, or unknown values to an empty string (never falling back to the Better Rich Presence logo).

#### Scenario: Language icon URL in small_image
- **WHEN** an IDE presence sets `small_image` to the Rust language icon URL (GitHub Raw form) on the shared client
- **THEN** `small_image` resolves to the jsDelivr `rust.png` URL

#### Scenario: Unknown small_image on the shared client
- **WHEN** a presence sets `small_image` to `"unknown-key"` on the shared client
- **THEN** `small_image` resolves to an empty string

## MODIFIED Requirements

### Requirement: App-first text layout for generated presences
For presences the system generates (catalog apps, apps not in the catalog, IDE windows, and Claude Code live sessions), `details` SHALL be the app name (optionally paired with workspace/project) and `state` SHALL describe the activity.

#### Scenario: Browser with a page title
- **WHEN** the foreground app is the catalog app `Firefox` with window title `GitHub - Mozilla Firefox`
- **THEN** `details` is `Firefox` and `state` is `Browsing GitHub`

#### Scenario: App without a useful window title
- **WHEN** the foreground app is a catalog productivity app whose window title is empty or equals the app name
- **THEN** `details` is the app name and `state` is the category's generic activity (for example `Working`)

#### Scenario: App not in the catalog
- **WHEN** the foreground process `my-tool` is not in the catalog, matches no rule, and has window title `Report Q3`
- **THEN** `details` is `my-tool` and `state` is `Report Q3`
- **AND** when the window title is empty, `state` is `Active`

#### Scenario: IDE window
- **WHEN** the foreground window title is `main.rs - my-project - Visual Studio Code` and no rule matches
- **THEN** `details` is `Visual Studio Code · my-project` and `state` is `Editing main.rs (Rust)`

#### Scenario: Claude Code live session
- **WHEN** a Claude Code session is active in project `my-project` editing `main.rs`
- **THEN** `details` is `Claude Code`, `state` is `Editing main.rs in my-project`, `large_text` is `Claude Code`, and `large_image` resolves to the `claude_code` catalog icon URL (`claude.png`)
