# presence-display Delta Specification

## ADDED Requirements

### Requirement: Explicit session timestamps are preserved on the shared client
When a presence source supplies a non-zero start timestamp (such as a Claude Code session start time or an IDE workspace start time), the engine SHALL preserve that timestamp instead of replacing it with the previous app's timestamp on the shared client.

#### Scenario: Switching from a browser to a Claude Code session
- **WHEN** the current presence is `Firefox` on the shared client with an earlier timestamp
- **AND** a Claude Code session starts at timestamp `T`
- **THEN** the published Claude Code presence uses timestamp `T`

#### Scenario: Tool changes within the same Claude Code session
- **WHEN** the same Claude Code session moves from `Thinking` to `Editing main.rs`
- **THEN** the session's start timestamp `T` is preserved

## MODIFIED Requirements

### Requirement: App-first text layout for generated presences
For presences the system generates (catalog apps, apps not in the catalog, IDE windows, and Claude Code live sessions), `details` SHALL be the app name and `state` SHALL describe the activity.

#### Scenario: Claude Code live session
- **WHEN** a Claude Code session is active in project `my-project` editing `main.rs`
- **THEN** `details` is `Claude Code`, `state` is `Editing main.rs in my-project`, `large_text` is `Claude Code`, and `large_image` resolves to the `claude_code` catalog icon URL (`claude.png`)
