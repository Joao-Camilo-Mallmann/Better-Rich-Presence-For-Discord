# main-window Specification

## Purpose
Defines how the main window is organized so a user can tell at a glance whether Discord shows their activity, manage which apps are shown and how, and change the app's behavior.

## Requirements

### Requirement: Single-row title bar
The main window SHALL show a title bar on one row at every window width from the minimum width up. It shows the app logo, the name "Better RPC", and the minimize, maximize/restore and close buttons. The title bar is a drag region for moving the window.

#### Scenario: Default window size
- **WHEN** the window is 420 px wide
- **THEN** the logo, name and the three window buttons appear on a single row

#### Scenario: Name does not follow the detected app
- **WHEN** the detected app changes from one app to another
- **THEN** the title bar still reads "Better RPC"

### Requirement: Maximize button reflects window state
The maximize button SHALL show a maximize icon and the label "Maximize" while the window is not maximized, and a restore icon and the label "Restore" while it is maximized, whichever way the window state changed.

#### Scenario: Maximized from the window manager
- **WHEN** the user maximizes the window with a window-manager shortcut or by double-clicking the title bar
- **THEN** the button shows the restore icon and its label reads "Restore"

### Requirement: Status band
Below the title bar the window SHALL show a full-width status band whose color and label state whether Discord shows the user's activity. The labels are "Live on Discord", "Sharing off", "Connecting to Discord", "Not connected to Discord" and "Paused for a game".

#### Scenario: Live
- **WHEN** sharing is on and the app is connected to Discord
- **THEN** the band uses the primary (Blurple) color and reads "Live on Discord"

#### Scenario: Sharing off
- **WHEN** the user has turned sharing off
- **THEN** the band uses the yellow color and reads "Sharing off", whatever the connection state

#### Scenario: Discord not reachable
- **WHEN** sharing is on and the app is not connected to Discord
- **THEN** the band uses a dark color with a red indicator and reads "Not connected to Discord"

#### Scenario: Waiting for Discord
- **WHEN** sharing is on and the connection is waiting for Discord to become ready
- **THEN** the band reads "Connecting to Discord"

#### Scenario: Paused for a game
- **WHEN** sharing is on and presence is paused because a game shows its own status
- **THEN** the band reads "Paused for a game"

### Requirement: Global share switch
The status band SHALL hold a switch labelled "Share" that turns sharing on and off, and the change SHALL be saved immediately.

#### Scenario: Turn sharing off
- **WHEN** the user turns the Share switch off
- **THEN** the setting is saved and the band changes to "Sharing off"

### Requirement: Sharing off keeps the window editable
When sharing is off, the Apps and Settings views SHALL stay fully usable. Only the Now preview is dimmed, with a note that Discord shows nothing until sharing is turned on.

#### Scenario: Edit an app while sharing is off
- **WHEN** sharing is off and the user opens the Apps view
- **THEN** the user can add, edit, reorder, enable and disable apps

### Requirement: Three views
The window SHALL organize its content into three views, "Now", "Apps" and "Settings", selected with tabs below the status band. One view is shown at a time, and "Now" is shown when the window opens.

#### Scenario: Switch views
- **WHEN** the user selects the "Settings" tab
- **THEN** the Settings view is shown, the tab is marked as selected, and the other views are hidden

#### Scenario: Keyboard use
- **WHEN** a tab has keyboard focus and the user presses the left or right arrow key
- **THEN** focus and selection move to the previous or next tab

### Requirement: Now view
The Now view SHALL show a preview of the Discord profile card with the current activity, labelled "What friends see". When Max priority mode shows an app other than the one in focus, it SHALL also show which app is shown and which app is in focus.

#### Scenario: Activity detected
- **WHEN** an app with a rule is detected
- **THEN** the preview shows the app's icon, name, both presence lines and the elapsed time

#### Scenario: Priority override
- **WHEN** Max priority mode shows Visual Studio Code while Firefox is in focus
- **THEN** the Now view names Visual Studio Code as shown and Firefox as in focus

### Requirement: Apps view
The Apps view SHALL list the configured apps in priority order. Each row shows the position, icon, name, a one-line preview of both presence lines, move up and down controls, an enable switch and an edit control. The view SHALL state that list order sets priority when Max priority mode is on.

#### Scenario: Add an app
- **WHEN** the user selects "Add app" and picks a running app
- **THEN** the app is added to the end of the list with default presence lines

#### Scenario: Search
- **WHEN** the user types in the search field
- **THEN** only apps whose name or process name contains the text are listed, and reordering is disabled until the search is cleared

#### Scenario: No apps yet
- **WHEN** no apps are configured
- **THEN** the view explains that apps added here are shown on Discord while in use, and offers "Add app"

### Requirement: App editing
Editing an app SHALL offer the fields Name, First line and Second line. Image and Discord application ID SHALL be under "More options", and the application ID SHALL be marked optional. Save, Cancel and Delete actions SHALL be available, and all labels SHALL be in English.

#### Scenario: Save an edit
- **WHEN** the user changes the first line and selects Save
- **THEN** the change is saved and the row's preview line shows the new text

#### Scenario: Cancel an edit
- **WHEN** the user changes a field and selects Cancel
- **THEN** the change is discarded and the edit form closes

### Requirement: App picker
The app picker SHALL list running apps that can be added, mark apps that are already configured, and close when the user presses Escape or clicks outside it.

#### Scenario: Close with Escape
- **WHEN** the picker is open and the user presses Escape
- **THEN** the picker closes without adding an app

### Requirement: Settings view
The Settings view SHALL group settings under Startup, Priority, When you're away, Timing and Activity detection, with a plain-language label and help text for each setting. Idle time and idle message are shown only while idle detection is on.

#### Scenario: Idle detection off
- **WHEN** idle detection is off
- **THEN** the idle time and idle message fields are hidden

### Requirement: Saving settings
The Settings view SHALL keep a "Save changes" button visible at the bottom. The button is enabled only when a setting differs from the saved value, and saving SHALL NOT change the Share switch.

#### Scenario: Nothing changed
- **WHEN** the user opens Settings without changing anything
- **THEN** the "Save changes" button is disabled

#### Scenario: Save after a change
- **WHEN** the user changes the idle time and selects "Save changes"
- **THEN** the settings are saved, the Share state is left as it is, and the button is disabled again

### Requirement: Keyboard-operable switches
Every on/off control in the window SHALL be a switch that can be focused with the keyboard, toggled with Space or Enter, and announces its on/off state to assistive technology.

#### Scenario: Toggle an app with the keyboard
- **WHEN** an app's enable switch has focus and the user presses Space
- **THEN** the app is enabled or disabled

### Requirement: Readable width when maximized
When the window is wider than the content needs, the views SHALL keep a centered column with a fixed maximum width instead of stretching to the window edges.

#### Scenario: Maximized on a wide screen
- **WHEN** the window is maximized on a 1920 px wide screen
- **THEN** the views are centered and no wider than their maximum column width
