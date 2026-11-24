# Spec Delta

## MODIFIED Requirements

### Requirement: Now view
The Now view SHALL show a preview of the Discord profile card with the current activity, labelled "What friends see". When Max priority mode shows an app other than the one in focus, it SHALL also show which app is shown and which app is in focus. When one or more Claude Code sessions are active, the Now view SHALL display a live Claude Code chats list showing each chat's title, project, git branch, model, and current activity, allowing the user to pin a chat or return to auto-follow.

#### Scenario: Activity detected
- **WHEN** an app with a rule is detected
- **THEN** the preview shows the app's icon, name, both presence lines and the elapsed time

#### Scenario: Priority override
- **WHEN** Max priority mode shows Visual Studio Code while Firefox is in focus
- **THEN** the Now view names Visual Studio Code as shown and Firefox as in focus

#### Scenario: Active Claude Code chats shown in Now view
- **WHEN** two Claude Code sessions are active
- **THEN** the Now view displays both chats with their metadata and activity below the preview card
- **AND** clicking a chat pins it as the active Discord presence, with an "Auto-follow" control to unpin
