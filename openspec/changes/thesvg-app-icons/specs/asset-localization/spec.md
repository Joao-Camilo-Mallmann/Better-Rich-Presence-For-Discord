# Spec Delta

## REMOVED Requirements

### Requirement: Local Icon Asset Retrieval
**Reason**: It reads the `icon` field that `icon-url-only` removed from `apps.json`, and it names Simple Icons, Iconify and 1024×1024 output, none of which apply any more. `colored-icon-pipeline` now owns icon retrieval and conversion.
**Migration**: Run `npm run assets:generate`, which follows `colored-icon-pipeline` (theSVG source, explicit icon source map, 256×256 PNG).

### Requirement: App Registry Schema Update
**Reason**: `icon_url` points to GitHub Raw PNGs, as `icon-url-only` requires, not to local `.svg` paths, and no script rewrites `icon_url` automatically.
**Migration**: When adding an icon to the catalog, set `icon_url` by hand to the GitHub Raw URL of the PNG, as `docs/Icons.md` describes.
