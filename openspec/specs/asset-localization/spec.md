# asset-localization Spec

## Requirements

### Requirement: Local Icon Asset Retrieval
The asset localization tool MUST download all remote icons defined in `apps.json` and store them locally inside `public/assets/icons/`.

#### Scenario: Download remote SVG icons
- **WHEN** the download asset script is executed
- **THEN** it fetches all `icon_url` remote HTTP endpoints listed in `apps.json` and writes the resulting SVG files to `public/assets/icons/`

### Requirement: App Registry Schema Update
The asset localization tool MUST update the `icon_url` property in `apps.json` for each application entry to point to the local path (`/assets/icons/<icon-name>.svg`).

#### Scenario: Update icon_url references
- **WHEN** an icon SVG file is successfully saved locally
- **THEN** `apps.json` entry `icon_url` is updated with the relative local path
