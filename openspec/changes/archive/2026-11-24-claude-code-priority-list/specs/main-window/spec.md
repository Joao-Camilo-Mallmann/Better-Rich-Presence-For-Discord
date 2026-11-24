# Spec Delta

## MODIFIED Requirements

### Requirement: Apps view
The Apps view SHALL list the configured apps in priority order, including the Claude Code entry. Each row shows the position, icon, name, a one-line preview of both presence lines, move up and down controls, an enable switch and an edit control. The Claude Code entry has no edit or delete control. The view SHALL state that list order sets priority, including over Claude Code sessions.

#### Scenario: Add an app
- **WHEN** the user selects "Add app" and picks a running app
- **THEN** the app is added to the end of the list with default presence lines

#### Scenario: Search
- **WHEN** the user types in the search field
- **THEN** only apps whose name or process name contains the text are listed, and reordering is disabled until the search is cleared

#### Scenario: No apps yet
- **WHEN** no apps are configured
- **THEN** the view explains that apps added here are shown on Discord while in use, and offers "Add app"

#### Scenario: Claude Code entry
- **WHEN** the Apps view lists the Claude Code entry
- **THEN** it shows the Claude Code icon (`claude.png`), the name `Claude Code`, move controls and the enable switch, and no edit or delete control

#### Scenario: Claude Code entry shows live activity
- **WHEN** a Claude Code session is active
- **THEN** the entry's preview line shows the live Claude Code presence lines (for example `Claude Code · Editing main.rs in my-project`) and updates as hook events arrive
- **AND** with no active session it shows `Claude Code · Waiting for a session`
