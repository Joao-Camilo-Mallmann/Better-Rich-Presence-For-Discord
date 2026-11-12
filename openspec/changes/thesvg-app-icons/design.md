## Context

See proposal.md for the motivation. Constraints:

- `apps.json` `icon_url` values point to `public/assets/icons/{name}.png` on GitHub Raw. The Rust engine rewrites them to jsDelivr for Discord, and `AppIcon` renders them in the UI. Filenames must not change.
- The app UI is dark (`surface-indigo`, `surface-onyx`, `surface-black` in `docs/DESIGN.md`), and most Discord users run the dark theme.
- `sharp` is already a dev dependency and is used by the current script.
- theSVG exposes a manifest at `src/data/icons.json` (slug, title, aliases, `variants` map, `license`, `collection`) and SVGs at `public/icons/{slug}/{variant}.svg`, both served by jsDelivr from `gh/glincker/thesvg@main`.

## Goals / Non-Goals

**Goals:**
- One command (`npm run assets:generate`) regenerates every icon from a reviewed, versioned map.
- Generated PNGs are committed; nothing fetches theSVG at runtime.

**Non-Goals:**
- Adding catalog entries for the 59 extra apps (executables and client IDs need their own review).
- Changing `icon_url`, the jsDelivr rewrite, or `AppIcon`.
- Light-theme-specific icon variants.

## Decisions

### Decision 1: Explicit source map instead of name matching at run time

`scripts/icon-sources.json` maps each output name to a theSVG slug (plus an optional pinned variant), and lists the names to keep as they are. Automatic matching by normalized name got 219 of 233 right but produced wrong hits (`terraform` → OpenTofu, `files` → Azure Files, `powershell` → Azure PowerShell). A committed map makes every choice reviewable and the output reproducible. Alternative considered: fuzzy matching with an override list. It was rejected because a manifest update could silently change which logo an app gets.

### Decision 2: Pin theSVG to a commit

The script resolves the manifest and SVGs from `gh/glincker/thesvg@<sha>`, with the SHA recorded in the map file, instead of `@main`. That keeps regeneration deterministic and leaves an audit trail when the pin is bumped. Alternative considered: `@main`, rejected because logos and slugs change upstream.

### Decision 3: 256×256 output

The previous 1024×1024 PNGs were later shrunk to 192×192 to save space. 256×256 stays sharp in Discord's large image (shown at up to about 120 CSS pixels, so about 240 device pixels on HiDPI screens) and in the UI, at about 10–30 KB per icon. Alternative considered: 512×512, which roughly triples the folder size for no visible gain.

### Decision 4: Dark-background variant selection

The script renders the default variant and measures the alpha-weighted share of its opaque area that is "dark neutral" (linear luminance below 0.08 and chroma below 0.25). At 60% or more the logo counts as near-black: the script uses `dark` when the manifest lists it, otherwise `mono` with every pixel painted white (`mono-white`). A first version used plain mean luminance below 0.18. It also flagged saturated dark brand colors (Make's purple, FileZilla's red) and logos on dark tiles, so it was replaced. A `variant` field in the map overrides the choice. The first full run needed pins for Premiere Pro (`default`), and for DaVinci Resolve, Monday.com, SQLite, Roblox, TeamViewer and Cinema 4D (`mono-white`), where the default artwork was a wordmark, dim navy, or near-invisible gray. Alternative considered: always placing logos on a tile, which reads well on any background. It was not chosen because the request was for theSVG's logos as published.

### Decision 5: Only `brands` and `community` collection entries

Slugs come from theSVG's `brands` collection, plus `community` where a product lives only there (Microsoft Azure, Microsoft Edge). The `auth-badges` and cloud-architecture (`aws`, `azure`, `gcp`) collections have a different shape (circular badges, service diagrams). Standard Notes and pCloud exist only as badges, so they stay without an icon.

### Decision 7: Kept icons get a one-time touch-up

The four icons theSVG lacks keep their Simple Icons artwork. Three of them (Ableton Live, Shotcut, Windows Terminal) are black silhouettes, so they were repainted white once, by hand, and all four were re-centered on a 256×256 canvas to match the generated set. The generator still never writes them.

### Decision 6: Extra icons are named after the theSVG slug

Names for the 59 extra apps are the theSVG slug with non-alphanumerics removed (`epic-games` → `epicgames`, `battledotnet` stays as is). This matches the existing Simple-Icons-style naming (`nextdotjs`, `visualstudiocode`). The 8 catalog entries that gain an icon use their `apps.json` `id`.

## Risks / Trade-offs

- [Some theSVG logos are wordmarks or have a large empty margin and look small at 256 px] → Trim transparent borders before resizing, and review the contact sheet. Pin a different variant or slug in the map where needed.
- [Trademarks: theSVG licenses the SVG files, but the marks still belong to their owners] → This is the same nominative use as before (identifying the running app). `docs/Icons.md` records the source and points to theSVG's per-icon license and trademark notes.
- [White logos vanish on Discord's light theme] → Accepted. The dark theme is the default and most common, and the app UI is dark-only.
- [jsDelivr caches `main` for up to 7 days, so Discord can show old icons for a while] → Purge the jsDelivr cache for changed files after merging, or wait for it to expire.

## Migration Plan

Merge to `main`. Old and new icons share filenames, so a rollback is a revert of the PNGs and the 8 `apps.json` lines.
