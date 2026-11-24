## RENAMED Requirements

- FROM: `### Requirement: Max Priority Mode on Linux`
- TO: `### Requirement: Priority list mode on Linux`

## MODIFIED Requirements

### Requirement: Priority list mode on Linux
When Priority list mode is selected on Linux, the system SHALL report the highest-priority running rule application, with its window title when a backend can list windows, and SHALL flag it as prioritized when it is not the focused application.

#### Scenario: Background rule app outranks focused app
- **WHEN** Priority list mode is selected, rule `code` is running in the background, and the user focuses Firefox
- **THEN** the engine receives process `code`, a prioritized flag, and `firefox` as the foreground app

#### Scenario: No running rule app
- **WHEN** Priority list mode is selected and no running process matches an enabled rule
- **THEN** the system reports the focused window as in Dynamic mode
