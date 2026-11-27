# Spec Delta

## MODIFIED Requirements

### Requirement: SVG to PNG Conversion
The script SHALL convert each downloaded SVG to a 512×512 PNG with a transparent background, the logo scaled to fit and centered with a 16px margin, and save it as `public/assets/icons/{name}.png` using the map entry's output name.

#### Scenario: Convert SVG to PNG
- **WHEN** a full-color SVG is downloaded for the map entry `discord`
- **THEN** `public/assets/icons/discord.png` is written as a 512×512 RGBA PNG with a transparent background

### Requirement: theSVG Logo Download
The icon generation script SHALL download full-color SVG logos from theSVG (`glincker/thesvg`, served through jsDelivr) for every entry in the project's icon source map from the `brands`, `community`, or `auth-badges` collections, retrying transient HTTP or network failures. The map links each output filename to a theSVG slug.

#### Scenario: Download a mapped icon
- **WHEN** the script processes the map entry `googlechrome` → `chrome`
- **THEN** it fetches the `chrome` SVG from the pinned `glincker/thesvg` commit and receives the full-color SVG

#### Scenario: Download a mapped auth-badge icon
- **WHEN** the script processes a map entry whose theSVG slug belongs to the `auth-badges` collection (such as `standardnotes` → `standard-notes-badge`)
- **THEN** it accepts the `auth-badges` entry and converts its SVG to a 512×512 PNG

#### Scenario: Slug missing from theSVG
- **WHEN** a map entry names a slug that is not in theSVG's manifest
- **THEN** the script logs a warning naming the file and slug and does not request any SVG for it

### Requirement: Dark-Background Readability
Before rasterizing a downloaded SVG, the script SHALL resolve `currentColor` to `#ffffff` so dual-color SVGs retain their brand accent colors alongside legible white foreground strokes on dark backgrounds. For a logo whose default variant is still near-black, the script SHALL use theSVG's `dark` variant when one exists, otherwise its `mono` variant filled white, so every generated icon stays visible on a dark background. A per-entry variant in the source map SHALL override this choice.

#### Scenario: Dual-color SVG using currentColor
- **WHEN** the `aws` SVG uses `currentColor` for the wordmark and `#F90` for the smile
- **THEN** `currentColor` is rendered as `#ffffff`, preserving the `#F90` smile without falling back to monochrome white

#### Scenario: Near-black logo with a dark variant
- **WHEN** the default GitHub logo is near-black and theSVG offers a `dark` variant for `github`
- **THEN** `github.png` is rendered from the `dark` variant

#### Scenario: Colorful logo
- **WHEN** the default Discord logo is not near-black
- **THEN** `discord.png` is rendered from the `default` variant

#### Scenario: Variant pinned in the map
- **WHEN** a map entry pins `"variant": "light"` or `"variant": "default"`
- **THEN** the script uses the pinned variant whatever its brightness
