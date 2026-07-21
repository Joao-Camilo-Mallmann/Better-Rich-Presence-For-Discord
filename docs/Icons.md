# Better Rich Presence: Icons Architecture

This document explains the standard pattern for assigning and rendering application icons in the **Better Rich Presence for Discord** project.

Due to technical limitations within Discord's Rich Presence API, the project adopts a **dual-icon architecture**, defining two distinct icon properties for each application in `apps.json`.

---

## 1. The Core Problem

- **Frontend (UI):** The React frontend renders perfectly with modern `.svg` icons. SVG provides the sharpest quality and scales infinitely.
- **Backend (Discord RPC):** The local Discord Desktop client **strictly rejects SVG files** for Rich Presence `large_image` payloads. It requires rasterized images (PNG, JPG) without complex URL query parameters.

To solve this without sacrificing frontend visual quality or breaking the Discord integration, every app entry requires two separate properties.

---

## 2. The `apps.json` Pattern

When adding a new application to `apps.json`, you must define both `icon` and `icon_url`:

```json
{
  "id": "zenbrowser",
  "name": "Zen Browser",
  "executables": ["zen.exe", "zen"],
  "client_id": "1517170930764480552",
  "category": "browser",
  "icon": "simple-icons:zenbrowser",
  "icon_url": "https://api.iconify.design/simple-icons:zenbrowser.svg"
}
```

### Property 1: `icon` (Used by the Backend for Discord)

- **Format:** Iconify collection string (e.g., `simple-icons:zenbrowser`, `simple-icons:visualstudiocode`).
- **How it works:** The Rust backend (`engine.rs`) reads this string and maps it to a high-quality `.png` asset hosted on the `walkxcode/dashboard-icons` GitHub CDN.
- **Mapping Logic:** Prefixes like `bx:` or `bxl-` are stripped, and hardcoded overrides map specific names (e.g., `visualstudiocode` maps to `vscode.png`).
- **Why?** Discord natively caches and accepts this raw GitHub JSdelivr CDN PNG link flawlessly.

### Property 2: `icon_url` (Used by the Frontend UI)

- **Format:** Full HTTP URL pointing to an SVG asset, preferably from the Iconify API (`https://api.iconify.design/...`).
- **How it works:** The React component (`AppIcon.tsx`) directly fetches and renders this SVG URL. It ignores the backend CDN mapping.
- **Why?** SVGs look incredibly crisp on the user interface, scale properly in the dashboard, and avoid pixelation issues associated with loading PNGs on high-DPI displays.

---

## 3. How to Add a New App

1. **Find the Iconify SVG:** Search [Iconify.design](https://icon-sets.iconify.design/) for the app logo (e.g., `simple-icons:cursor`).
2. **Set `icon_url`:** Assemble the SVG link: `https://api.iconify.design/simple-icons:cursor.svg`.
3. **Verify the PNG Mapping:** Ensure the `icon` string maps correctly in `src-tauri/src/services/engine.rs` (`get_icon_png_url` function) to an existing `.png` inside the `walkxcode/dashboard-icons` repository. If the repository names it differently, add a custom override to the Rust `match` statement.
