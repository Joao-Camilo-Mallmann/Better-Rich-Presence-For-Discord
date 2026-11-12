# presence-display Specification

## Purpose
Defines what the Discord activity contains for each presence source (text layout, status text, images, activity name) and how the in-app Status Preview mirrors what Discord shows, so users see the same presence in both places.

## Requirements

### Requirement: Activity name comes from the Discord application
The system SHALL NOT send an activity name override. The bold title of the activity is the registered name of the Discord application whose client ID is in use. Every presence SHALL carry that name as its `activity_name`.

#### Scenario: Catalog app on the shared client
- **WHEN** a presence is published for any app that uses the shared default client ID
- **THEN** the activity payload has no `name` field
- **AND** the presence's `activity_name` is `Better Rich Presence`

#### Scenario: App with its own Discord application
- **WHEN** a presence is published with a client ID other than the default and Discord's application lookup returns a name
- **THEN** the presence's `activity_name` is that returned name

### Requirement: App-first text layout for generated presences
For presences the system generates (catalog apps, apps not in the catalog, and IDE windows), `details` SHALL be the app name and `state` SHALL describe the activity.

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

### Requirement: Rule presences keep the user's text
For a matching enabled rule, `details` and `state` SHALL be the rule's templates with `{file}` and `{title}` filled in. On the shared client, when `details` does not already contain the rule's display name, `details` SHALL be prefixed with `<display name> - `. When the rendered `details` is empty, `details` SHALL be the display name.

#### Scenario: Rule with templates
- **WHEN** a rule with display name `Notes`, details `Writing {title}` and state `Focus mode` matches a window titled `Plan`
- **THEN** `details` is `Notes - Writing Plan` and `state` is `Focus mode`

#### Scenario: Rule with empty details
- **WHEN** a matching rule with display name `Notes` has empty details and state `Focus mode`
- **THEN** `details` is `Notes` and `state` is `Focus mode`

#### Scenario: Rule with a legacy placeholder
- **WHEN** a matching rule with display name `Notes` has details `Using the computer` or `Usando o computador`
- **THEN** `details` is `Notes`

### Requirement: Member-list status shows the details line
Every activity the system sends SHALL set the status display type to `details`, so Discord's member and friends lists show the `details` text after the activity verb instead of the application name.

#### Scenario: Status text for a catalog app
- **WHEN** the presence for `Firefox` is published on the shared client
- **THEN** the activity payload sets `status_display_type` to the value for `details`
- **AND** member lists read `Playing Firefox`

### Requirement: Only images Discord can render
On the shared client, every large image SHALL be an HTTPS URL, because that Discord application has no uploaded art assets. A non-URL value SHALL resolve to the icon of the catalog app whose id or executable matches it, else to the Better Rich Presence logo URL. Empty, `auto` and `default` SHALL resolve to the detected app's icon, else the logo URL.

#### Scenario: Catalog app with an icon
- **WHEN** the detected app has an `icon_url`
- **THEN** the large image is that URL with the jsDelivr rewrite applied

#### Scenario: Catalog app without an icon
- **WHEN** the detected app has no `icon_url`
- **THEN** the large image is the Better Rich Presence logo URL

#### Scenario: Rule with a catalog id as image
- **WHEN** a matching rule's image is the plain string `firefox`
- **THEN** the large image is the Firefox catalog icon URL

#### Scenario: Idle presence
- **WHEN** the idle presence is published
- **THEN** the large image is the Better Rich Presence logo URL

#### Scenario: Asset key on an app's own Discord application
- **WHEN** a presence uses a client ID other than the default and its image is a key that matches no catalog app
- **THEN** the key is sent unchanged, because that application may have uploaded the asset

### Requirement: Large image tooltip names the app
The large image text SHALL be the app name for generated and rule presences, and `Idle` for the idle presence.

#### Scenario: Tooltip for a catalog app
- **WHEN** the presence for `Firefox` is published
- **THEN** the large image text is `Firefox`

### Requirement: Presence updates include every visible field
The system SHALL skip a Discord update only when every presence field except the timestamp is unchanged.

#### Scenario: Only the image text changes
- **WHEN** a new presence differs from the current one only in its large image text
- **THEN** the system sends the new presence to Discord

### Requirement: Default and idle presence text is English
The presence shown before any app is detected SHALL read `Browsing the PC` / `Just chilling...`. The idle presence SHALL use `Idle` as its state and image text, and the user's idle message as `details`.

#### Scenario: Idle presence text
- **WHEN** the user goes idle with idle message `Away`
- **THEN** `details` is `Away`, `state` is `Idle` and the large image text is `Idle`

### Requirement: Status Preview mirrors the Discord activity
The Status Preview SHALL render the presence the way Discord does: the large image from the same URL, `activity_name` as the bold title, then `details`, `state`, and the elapsed timer. Above the card body it SHALL show the member-list line `Playing <details>`.

#### Scenario: Preview for a catalog app
- **WHEN** the current presence is `Firefox` / `Browsing GitHub` on the shared client
- **THEN** the preview's bold title is `Better Rich Presence`
- **AND** its lines are `Firefox`, `Browsing GitHub` and the elapsed timer
- **AND** its member-list line reads `Playing Firefox`

### Requirement: Presence stays current while Rich Presence is off
While the global switch is off, the system SHALL keep building the presence for the foreground app and publish it to the frontend without sending it to Discord. Turning the switch on SHALL send the latest presence.

#### Scenario: App changes while switched off
- **WHEN** the global switch is off and the foreground app changes to `Firefox`
- **THEN** the frontend receives the `Firefox` presence and Discord receives nothing
- **AND** when the switch is turned on, Discord receives the `Firefox` presence
