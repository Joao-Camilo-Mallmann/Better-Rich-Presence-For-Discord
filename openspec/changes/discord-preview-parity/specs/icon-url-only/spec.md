# Spec Delta

## MODIFIED Requirements

### Requirement: Single-source icon resolution via icon_url
The system SHALL resolve app icons exclusively through the `icon_url` field in `apps.json`. The `icon` field (Iconify key format) SHALL NOT exist in the app catalog schema.

#### Scenario: App with icon_url set
- **WHEN** the system resolves an icon for an app that has `icon_url` defined in `apps.json`
- **THEN** the system SHALL use that URL as the icon source (applying jsDelivr CDN rewrite for Discord consumption)

#### Scenario: App without icon_url
- **WHEN** the system resolves an icon for an app that does NOT have `icon_url` defined
- **THEN** the Rust backend SHALL use the Better Rich Presence logo URL (`public/logo-256.png` through jsDelivr) as the Discord large image, because the shared Discord application has no uploaded art assets
- **AND** the Status Preview SHALL render that same logo URL, while other React icon lists keep the generic `<Monitor/>` fallback

#### Scenario: icon_url points to non-existent file
- **WHEN** the system resolves an icon whose `icon_url` points to a file that does not exist (HTTP 404)
- **THEN** the React frontend SHALL render the generic `<Monitor/>` fallback icon via `onError`
