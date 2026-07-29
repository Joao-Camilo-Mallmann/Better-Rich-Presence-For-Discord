## ADDED Requirements

### Requirement: Single-source icon resolution via icon_url
The system SHALL resolve app icons exclusively through the `icon_url` field in `apps.json`. The `icon` field (Iconify key format) SHALL NOT exist in the app catalog schema.

#### Scenario: App with icon_url set
- **WHEN** the system resolves an icon for an app that has `icon_url` defined in `apps.json`
- **THEN** the system SHALL use that URL as the icon source (applying jsDelivr CDN rewrite for Discord consumption)

#### Scenario: App without icon_url
- **WHEN** the system resolves an icon for an app that does NOT have `icon_url` defined
- **THEN** the Rust backend SHALL use `"default"` as the Discord asset key
- **AND** the React frontend SHALL render the generic `<Monitor/>` fallback icon

#### Scenario: icon_url points to non-existent file
- **WHEN** the system resolves an icon whose `icon_url` points to a file that does not exist (HTTP 404)
- **THEN** the React frontend SHALL render the generic `<Monitor/>` fallback icon via `onError`

### Requirement: No external icon API dependencies
The system SHALL NOT make requests to any external icon API (including Iconify) for icon resolution. All icon URLs MUST point to the project's own hosted assets (GitHub Raw / jsDelivr CDN).

#### Scenario: Colon-formatted icon string received
- **WHEN** `get_discord_asset_key()` receives a non-URL string containing a colon (e.g. `simple-icons:vscode`)
- **THEN** the function SHALL treat it as a plain string passthrough (no Iconify URL construction)

#### Scenario: React AppIcon with non-URL non-catalog name
- **WHEN** the `AppIcon` component receives a name that is not a URL and does not match any app in the catalog
- **THEN** the component SHALL render the generic `<Monitor/>` fallback icon
- **AND** SHALL NOT construct an Iconify API URL

### Requirement: apps.json schema excludes icon field
The `apps.json` catalog schema SHALL NOT include an `icon` field. Each app entry SHALL have only: `id`, `name`, `executables`, `client_id`, `category`, and optionally `icon_url`.

#### Scenario: App entry structure
- **WHEN** a new app is added to `apps.json`
- **THEN** it SHALL include `id`, `name`, `executables`, `client_id`, `category`
- **AND** it MAY include `icon_url` pointing to a GitHub Raw PNG in `public/assets/icons/`
- **AND** it SHALL NOT include an `icon` field
