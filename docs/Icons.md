# Better Rich Presence: Icons Architecture

This document explains the standard pattern for assigning and rendering application icons in the **Better Rich Presence for Discord** project.

---

## 1. Overview

The project uses a **single-source icon architecture** based on the `icon_url` field in `apps.json`. Each app entry optionally specifies an `icon_url` pointing to a PNG hosted in the repository at `public/assets/icons/`.

- **Frontend (UI):** The React `AppIcon` component renders the `icon_url` directly. If the URL fails or is absent, it falls back to a generic `<Monitor/>` icon.
- **Backend (Discord RPC):** The Rust engine rewrites GitHub Raw URLs to jsDelivr CDN URLs for Discord consumption. If no `icon_url` is set, it sends `"default"` as the asset key.

---

## 2. The `apps.json` Pattern

When adding a new application to `apps.json`, define `icon_url` pointing to a PNG in `public/assets/icons/`:

```json
{
  "id": "zenbrowser",
  "name": "Zen Browser",
  "executables": ["zen.exe", "zen"],
  "client_id": "1517170930764480552",
  "category": "browser",
  "icon_url": "https://raw.githubusercontent.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/refs/heads/main/public/assets/icons/zenbrowser.png"
}
```

### `icon_url` (Optional)

- **Format:** Full HTTPS URL pointing to a PNG in `public/assets/icons/` via GitHub Raw.
- **Backend behavior:** The Rust engine rewrites `raw.githubusercontent.com` URLs to `cdn.jsdelivr.net` for Discord Rich Presence. Non-URL strings (like `"default"`) pass through unchanged.
- **Frontend behavior:** The React `AppIcon` component renders this URL as an `<img>`. On load failure, it shows a `<Monitor/>` fallback.
- **If absent:** The app shows a generic monitor icon in the UI and `"default"` in Discord.

---

## 3. How to Add a New App

1. **Create or find a PNG icon** for the app (ideally 128×128 or 256×256).
2. **Place it** in `public/assets/icons/<app-name>.png`.
3. **Set `icon_url`** in the app entry to the GitHub Raw URL:
   ```
   https://raw.githubusercontent.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/refs/heads/main/public/assets/icons/<app-name>.png
   ```
4. **Verify** the icon loads in the app UI and shows correctly in Discord Rich Presence.

---

## 4. Icon Resolution Flow

```
icon_url present?
  ├─ YES → GitHub Raw URL → jsDelivr CDN rewrite → Discord
  │                        → Direct render → React UI
  └─ NO  → "default" → Discord
           → <Monitor/> → React UI
```
