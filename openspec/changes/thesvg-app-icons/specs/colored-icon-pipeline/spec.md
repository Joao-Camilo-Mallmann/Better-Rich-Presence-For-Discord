# Spec Delta

## REMOVED Requirements

### Requirement: Colored SVG Download
**Reason**: Simple Icons and Iconify serve single-color silhouettes, and many of them are black, which disappears on dark backgrounds. Icons now come from theSVG's full-color logos.
**Migration**: Replaced by "theSVG Logo Download". Run `npm run assets:generate`; icons are selected by `scripts/icon-sources.json` instead of the removed `icon` field.

## MODIFIED Requirements

### Requirement: SVG to PNG Conversion
The script SHALL convert each downloaded SVG to a 256×256 PNG with a transparent background, the logo scaled to fit and centered, and save it as `public/assets/icons/{name}.png` using the map entry's output name.

#### Scenario: Convert SVG to PNG
- **WHEN** a full-color SVG is downloaded for the map entry `discord`
- **THEN** `public/assets/icons/discord.png` is written as a 256×256 RGBA PNG with a transparent background

## ADDED Requirements

### Requirement: theSVG Logo Download
The icon generation script SHALL download full-color SVG logos from theSVG (`glincker/thesvg`, served through jsDelivr) for every entry in the project's icon source map. The map links each output filename to a theSVG slug.

#### Scenario: Download a mapped icon
- **WHEN** the script processes the map entry `visualstudiocode` → `visual-studio-code`
- **THEN** it fetches `https://cdn.jsdelivr.net/gh/glincker/thesvg@main/public/icons/visual-studio-code/default.svg` and receives a full-color SVG

#### Scenario: Slug missing from theSVG
- **WHEN** a map entry names a slug that is not in theSVG's manifest
- **THEN** the script logs a warning naming the file and slug and does not request any SVG for it

### Requirement: Explicit Icon Source Map
The script SHALL read the list of icons to generate from a versioned source map file, not from `apps.json`. Every `icon_url` filename referenced by `apps.json` SHALL either appear in the map or be listed as kept, so a catalog icon is never silently skipped.

#### Scenario: Catalog icon not covered by the map
- **WHEN** `apps.json` references `public/assets/icons/foo.png` and `foo` is neither mapped nor listed as kept
- **THEN** the script reports `foo` as unmapped in its summary and exits with a non-zero status

#### Scenario: Icon kept from a previous source
- **WHEN** `shotcut` is listed as kept because theSVG has no Shotcut logo
- **THEN** the script leaves `public/assets/icons/shotcut.png` untouched and counts it as skipped

### Requirement: Dark-Background Readability
For a logo whose default variant is near-black, the script SHALL use theSVG's `dark` variant when one exists, otherwise its `mono` variant filled white, so every generated icon stays visible on a dark background. A per-entry variant in the source map SHALL override this choice.

#### Scenario: Near-black logo with a dark variant
- **WHEN** the default GitHub logo is near-black and theSVG offers a `dark` variant for `github`
- **THEN** `github.png` is rendered from the `dark` variant

#### Scenario: Colorful logo
- **WHEN** the default Discord logo is not near-black
- **THEN** `discord.png` is rendered from the `default` variant

#### Scenario: Variant pinned in the map
- **WHEN** a map entry pins `"variant": "default"`
- **THEN** the script uses the default variant whatever its brightness
