# Spec Delta

## MODIFIED Requirements

### Requirement: App-first text layout for generated presences
For presences the system generates (catalog apps, apps not in the catalog, IDE windows, and Claude Code live sessions), `details` SHALL be the app name (optionally paired with the project name for Claude Code) and `state` SHALL describe the activity.

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
- **THEN** `details` is `Visual Studio Code` and `state` is `Editing main.rs in my-project`

#### Scenario: Claude Code live session
- **WHEN** a Claude Code session is active in project `my-project` editing `main.rs` with chat title `Fix bug`, model `Claude Opus 4.6`, and branch `main`
- **THEN** `details` is `Claude Code · my-project`, `state` is `Editing main.rs · Fix bug`, `large_text` is `Claude Opus 4.6 · main`, and `large_image` resolves to the `claude_code` catalog icon URL (`claude.png`)

### Requirement: Status Preview mirrors the Discord activity
The Status Preview SHALL render the presence the way Discord does: the large image from the same URL (with an optional small-image corner badge and tooltip when present), `activity_name` as the bold title, then `details`, `state`, and the elapsed timer. Above the card body it SHALL show the member-list line `Playing <details>`.

#### Scenario: Preview for a catalog app
- **WHEN** the current presence is `Firefox` / `Browsing GitHub` on the shared client
- **THEN** the preview's bold title is `Better Rich Presence`
- **AND** its lines are `Firefox`, `Browsing GitHub` and the elapsed timer
- **AND** its member-list line reads `Playing Firefox`

#### Scenario: Preview with a small image badge
- **WHEN** the current presence includes a non-empty `small_image` and `small_text`
- **THEN** the preview renders the small image as a circular corner badge on the large image with `small_text` as its tooltip
