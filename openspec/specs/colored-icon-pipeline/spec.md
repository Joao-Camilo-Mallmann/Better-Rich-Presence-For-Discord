# colored-icon-pipeline Spec

## Purpose
TBD

## Requirements

### Requirement: Colored SVG Download
The icon generation script SHALL download SVGs with official brand colors from the Simple Icons CDN (`cdn.simpleicons.org/{slug}`) for all entries in `apps.json` that use the `simple-icons:` prefix.

#### Scenario: Download colored SVG for a Simple Icons entry
- **WHEN** the script processes an app with `"icon": "simple-icons:discord"`
- **THEN** it fetches `https://cdn.simpleicons.org/discord` and receives an SVG with `fill="#5865F2"` (the brand color)

#### Scenario: Download colored SVG for a non-Simple-Icons entry
- **WHEN** the script processes an app with `"icon": "material-symbols:antigravity"` or any non-`simple-icons` prefix
- **THEN** it fetches the SVG from the Iconify API at `https://api.iconify.design/{prefix}/{name}.svg`

### Requirement: SVG to PNG Conversion
The script SHALL convert each downloaded SVG to a 1024×1024 PNG with a transparent background using `sharp`.

#### Scenario: Convert SVG to PNG
- **WHEN** a colored SVG is successfully downloaded
- **THEN** it is converted to a 1024×1024 PNG (using `fit: contain` with transparent background) and saved to `public/assets/icons/{slug}.png`

### Requirement: Failure Handling
The script SHALL log warnings for failed downloads without stopping execution, and SHALL preserve the existing PNG file for any icon that fails to download or convert.

#### Scenario: CDN returns an error
- **WHEN** a fetch to the Simple Icons CDN returns a non-200 status
- **THEN** the script logs a warning with the app ID and HTTP status, and skips to the next icon without overwriting the existing PNG

#### Scenario: Slug does not exist on any source
- **WHEN** both the primary CDN and the Iconify fallback fail for a given icon
- **THEN** the script logs a warning and the existing PNG remains untouched

### Requirement: Summary Report
The script SHALL print a summary report at the end showing total icons processed, successful conversions, failures, and skipped entries.

#### Scenario: Script completes
- **WHEN** all entries in `apps.json` have been processed
- **THEN** the script prints a summary with counts for: total processed, successfully converted, failed, and skipped
