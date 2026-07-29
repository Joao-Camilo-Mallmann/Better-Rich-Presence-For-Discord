## MODIFIED Requirements

### Requirement: Local Icon Asset Retrieval
The asset localization tool MUST download all remote icons defined in `apps.json` and store them locally inside `public/assets/icons/`. Icons SHALL be sourced as colored SVGs from the Simple Icons CDN (primary) or Iconify API (fallback), and converted to 1024×1024 PNG format before saving.

#### Scenario: Download and convert colored icons
- **WHEN** the download asset script is executed
- **THEN** it extracts the icon slug from the `icon` field in each `apps.json` entry, fetches the colored SVG from the appropriate CDN, converts it to a 1024×1024 PNG via `sharp`, and writes the result to `public/assets/icons/{slug}.png`
