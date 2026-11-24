## ADDED Requirements

### Requirement: Presence mode selector
The Apps view SHALL offer a "Presence mode" selector above the app list with exactly two options, "Dynamic" and "Priority list", of which one is always selected. Dynamic means Discord shows the app in focus and follows the user when they switch windows. Priority list means Discord shows the highest running app in the Apps list even when another app is in focus. Each option SHALL have a tooltip, shown on pointer hover and on keyboard focus, with one plain-language sentence explaining what the mode does. Selecting an option SHALL save it immediately.

#### Scenario: Dynamic mode follows focus
- **WHEN** Dynamic is selected, Visual Studio Code and Firefox both have enabled rules, and the user switches from Visual Studio Code to Firefox
- **THEN** the presence changes to Firefox

#### Scenario: Priority list mode keeps the highest app
- **WHEN** Priority list is selected, Visual Studio Code is ranked above Firefox and both are running, and the user focuses Firefox
- **THEN** the presence keeps showing Visual Studio Code

#### Scenario: Selection is saved immediately
- **WHEN** the user selects Dynamic while Priority list was selected
- **THEN** the setting is saved without any further action and Dynamic is still selected after the app restarts

#### Scenario: Existing installation
- **WHEN** a user who had Max priority mode on opens the Apps view after updating
- **THEN** Priority list is selected

#### Scenario: Tooltip explains an option
- **WHEN** the pointer rests on the "Priority list" option, or the option has keyboard focus
- **THEN** a tooltip explains that Discord keeps showing the highest running app in the list even while another app is in use

#### Scenario: Keyboard operation
- **WHEN** the selector has focus and the user presses the Left or Right arrow key
- **THEN** the other option becomes selected and focused

## MODIFIED Requirements

### Requirement: Now view
The Now view SHALL show a preview of the Discord profile card with the current activity, labelled "What friends see". When Priority list mode shows an app other than the one in focus, it SHALL also show which app is shown and which app is in focus. When one or more Claude Code sessions are active, the Now view SHALL display a live Claude Code chats list showing each chat's title, project, git branch, model, and current activity, allowing the user to pin a chat or return to auto-follow.

#### Scenario: Activity detected
- **WHEN** an app with a rule is detected
- **THEN** the preview shows the app's icon, name, both presence lines and the elapsed time

#### Scenario: Priority override
- **WHEN** Priority list mode shows Visual Studio Code while Firefox is in focus
- **THEN** the Now view names Visual Studio Code as shown and Firefox as in focus

#### Scenario: Active Claude Code chats shown in Now view
- **WHEN** two Claude Code sessions are active
- **THEN** the Now view displays both chats with their metadata and activity below the preview card
- **AND** clicking a chat pins it as the active Discord presence, with an "Auto-follow" control to unpin

### Requirement: Settings view
The Settings view SHALL group settings under Startup, Claude Code, When you're away, Timing and Activity detection, with a plain-language label and help text for each setting. Idle time and idle message are shown only while idle detection is on. The presence mode is not set here.

#### Scenario: Idle detection off
- **WHEN** idle detection is off
- **THEN** the idle time and idle message fields are hidden

### Requirement: Saving settings
The Settings view SHALL keep a "Save changes" button visible at the bottom. The button is enabled only when a setting differs from the saved value, and saving SHALL NOT change the Share switch or the presence mode.

#### Scenario: Nothing changed
- **WHEN** the user opens Settings without changing anything
- **THEN** the "Save changes" button is disabled

#### Scenario: Save after a change
- **WHEN** the user changes the idle time and selects "Save changes"
- **THEN** the settings are saved, the Share state is left as it is, and the button is disabled again

#### Scenario: Presence mode changed while Settings has unsaved edits
- **WHEN** the user edits the idle time, switches the presence mode in the Apps view, then selects "Save changes"
- **THEN** the idle time is saved and the presence mode chosen in the Apps view is kept
