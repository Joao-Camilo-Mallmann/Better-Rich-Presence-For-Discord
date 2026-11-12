# Spec Delta

## Purpose

Keeps every piece of text the app shows to people (tray menu, Discord presence, default settings) in English, consistent with the project's English-only policy.

## ADDED Requirements

### Requirement: English tray menu
The system tray menu SHALL label its actions in English.

#### Scenario: Tray menu labels
- **WHEN** the user opens the tray menu
- **THEN** the actions read "Open" and "Quit"

### Requirement: English idle presence
When the user becomes idle, the presence the app publishes SHALL use English text for its state and image tooltip.

#### Scenario: Idle presence text
- **WHEN** the idle threshold is reached
- **THEN** the published presence has state "Idle" and large image text "Idle", and its details line is the configured idle message

### Requirement: English default idle message
New installs SHALL default the idle message to "Away". An idle message the user already saved SHALL be kept unchanged.

#### Scenario: New install
- **WHEN** the app starts with no saved settings
- **THEN** the idle message setting is "Away"

#### Scenario: Existing saved message
- **WHEN** the app starts with a saved idle message "Ausente"
- **THEN** the idle message setting stays "Ausente"

### Requirement: Legacy placeholder detection
The engine SHALL treat both the English placeholder "Using the computer" and the legacy Portuguese placeholder "Usando o computador" as empty details when formatting a presence for the default client, so rules saved by older versions keep working.

#### Scenario: Legacy rule details
- **WHEN** a saved rule's details are "Usando o computador" and its app is shown with the default client
- **THEN** the published details read "Using <app name>"
