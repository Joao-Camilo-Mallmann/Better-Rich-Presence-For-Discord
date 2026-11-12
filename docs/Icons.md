# Better Rich Presence: Icons Architecture

This document explains how application icons are sourced, generated, and rendered in **Better Rich Presence for Discord**.

---

## 1. Overview

The project uses a **single-source icon architecture** based on the `icon_url` field in `apps.json`. Each app entry optionally specifies an `icon_url` pointing to a PNG committed to the repository in `public/assets/icons/`.

- **Frontend (UI):** The React `AppIcon` component renders the `icon_url` directly. If the URL fails or is absent, it falls back to a generic `<Monitor/>` icon.
- **Backend (Discord RPC):** The Rust engine rewrites GitHub Raw URLs to jsDelivr CDN URLs for Discord consumption. If no `icon_url` is set, it sends `"default"` as the asset key.

Icons are served from the `main` branch, so a new or changed icon appears in the app and in Discord only after it is merged to `main` (and, for Discord, after jsDelivr's cache refreshes, which can take up to 7 days unless purged).

---

## 2. Where the icons come from

The PNGs are generated from **[theSVG](https://thesvg.org)** (`glincker/thesvg`), a collection of official full-color brand logos with a license recorded for each icon.

| File | Role |
| --- | --- |
| `scripts/icon-sources.json` | The icon map: which PNG is built from which theSVG logo |
| `scripts/generate-colored-icons.mjs` | The generator, run with `npm run assets:generate` |
| `public/assets/icons/*.png` | The committed output: 256×256 PNGs with a transparent background |

`scripts/icon-sources.json` has three parts:

```json
{
  "thesvg": { "repo": "glincker/thesvg", "commit": "985e12a0…" },
  "keep": ["abletonlive", "files", "shotcut", "windowsterminal"],
  "icons": {
    "discord": "discord",
    "visualstudiocode": "visual-studio-code",
    "roblox": { "slug": "roblox", "variant": "mono-white" }
  }
}
```

- **`thesvg.commit`** pins the theSVG commit, so regenerating gives the same result. Bump it on purpose to pick up upstream logo updates, then review the output.
- **`icons`** maps a PNG name (the filename used in `icon_url`) to a theSVG slug, as shown in the icon's URL on thesvg.org (`thesvg.org/icon/<slug>`). Use the object form to pin a variant.
- **`keep`** lists PNGs that theSVG has no logo for. The generator never touches them.

The folder holds 300 icons: every icon used by `apps.json`, plus icons for popular apps that are not in the catalog yet (Spotify, Steam, Telegram, WhatsApp, Claude, Gemini, …), ready for future catalog entries.

### Readability on dark backgrounds

The app UI is dark and most Discord users use the dark theme, so a near-black logo would disappear. For each icon the generator renders theSVG's `default` variant. If at least 60% of the logo is dark and uncolored, it switches to theSVG's `dark` variant (the logo drawn for dark backgrounds) when one exists, and otherwise to the `mono` variant painted white (`mono-white`). Saturated brand colors such as purple or red count as visible, so those logos keep their colors.

When the automatic choice looks wrong, pin a variant in the map: `default`, `dark`, `light`, `mono`, or `mono-white`.

---

## 3. How to add or change an icon

1. **Find the logo** on [thesvg.org](https://thesvg.org) and note its slug. Prefer the brand logo over the circular "badge" entries.
2. **Add it to `scripts/icon-sources.json`** under `icons`, keyed by the PNG name (lowercase letters and digits, e.g. `epicgames`).
3. **Generate it:**
   ```bash
   npm run assets:generate -- --only epicgames
   ```
   Without `--only`, every icon in the map is regenerated.
4. **Check it** on a dark background. If it is hard to see or uses the wrong artwork, pin a `variant` or choose a different slug, then regenerate.
5. **Reference it** from the app entry in `apps.json`:
   ```json
   "icon_url": "https://raw.githubusercontent.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/refs/heads/main/public/assets/icons/epicgames.png"
   ```

If theSVG has no logo for an app, add a 256×256 transparent PNG to `public/assets/icons/` by hand and list its name under `keep`.

The generator exits with an error, and lists the problem, when a slug is not on theSVG, a download or conversion fails (the previous PNG is kept), or `apps.json` references a PNG that is neither in `icons` nor in `keep`.

---

## 4. Licenses and trademarks

theSVG records a license for every logo (most are CC0-1.0 or MIT; some are marked as brand-use or fair-use). The generator prints a license summary after each run, and each icon's license is shown on its page at `thesvg.org/icon/<slug>`.

A license covers the SVG file, not the brand. Every logo remains a trademark of its owner, and this project uses them only to identify which application is running. See theSVG's [TRADEMARK.md](https://github.com/glincker/thesvg/blob/main/TRADEMARK.md) and [LICENSING.md](https://github.com/glincker/thesvg/blob/main/LICENSING.md).

---

## 5. Icon resolution flow

```
icon_url present?
  ├─ YES → GitHub Raw URL → jsDelivr CDN rewrite → Discord
  │                        → Direct render → React UI
  └─ NO  → "default" → Discord
           → <Monitor/> → React UI
```
