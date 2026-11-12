# Proposal

## Why

The Status Preview in the app and the Rich Presence that Discord actually shows have drifted apart. The preview puts the detected app's name in bold with its icon. In Discord, users see only "Playing Better Rich Presence", a blank image, or nothing at all. Three verified causes:

- All 286 catalog apps share one Discord application (`DEFAULT_CLIENT_ID`). Discord always takes the bold activity name from that application's registered name ("Better Rich Presence"), so the `name` field the app sends has no effect over local RPC.
- That Discord application has **no uploaded art assets and no icon**: `GET /oauth2/applications/{id}/assets` returns `[]`. Every asset key the app sends (`default`, `idle`, `rust-logo`, `file-icon`, `visual-studio-code`, `cursor-logo`, …) therefore renders blank. Only full image URLs render.
- The preview renders text Discord never shows, such as `large_text` as the bold title. While Rich Presence is switched off, the engine stops updating the presence entirely, so the preview is stuck on the startup placeholder.

## What Changes

- **Readable layout for the shared client**: for generated presences (catalog apps, unknown apps, IDE windows), `details` carries the app name and `state` carries the activity (for example `Firefox` / `Browsing GitHub`). User-authored rule text is kept as today.
- **Status text in the member list**: every activity sets `status_display_type = Details`, so friends and member lists read "Playing Firefox" instead of "Playing Better Rich Presence".
- **Only images Discord can render**: the large image is always an HTTPS URL. Asset keys that are not uploaded to the Discord application are resolved through the catalog (same lookup as the React `AppIcon`), with the Better Rich Presence logo URL as the final fallback. This covers idle, the default presence, apps without `icon_url`, and IDE presences.
- **IDE presences**: the large image becomes the IDE's catalog icon, and no small image is sent. The extension-to-asset-key mapping (`rust-logo`, `file-icon`, …) is removed: those keys render blank and the repository has no language icons to replace them.
- **No `name` override**: the app stops sending the `name` field (crate feature `unstable_name`), which Discord ignores. What Discord shows becomes deterministic, so the preview can mirror it.
- **Preview mirrors Discord**: the card shows the Discord application name as the bold title, then `details`, `state` and the elapsed timer. It also adds a "Playing …" member-list line. The "sharing off" and "not connected" notices belong to the in-flight `reorganize-main-window` change (status band and dimmed preview), not to this one.
- **Preview stays current while paused**: the engine keeps building the presence while Rich Presence is off, so the preview shows what would be sent. Turning it on sends that presence.
- **Full diff check**: a presence update is skipped only when every field except the timestamp is unchanged. Today it compares only `details`, `state` and `large_image`, so a change to `large_text` alone never reaches Discord.
- **English text**: the default presence ("Navegando no PC" / "Só de boa...") and the idle presence ("Inativo") become English.
- `PresenceData` gains `activity_name` (the bold title Discord shows). The new field has a serde default, so the frontend contract is extended without breaking anything.

## Capabilities

### New Capabilities

- `presence-display`: what the Discord activity contains for each presence source (text layout, status display type, image resolution, activity name), and how the Status Preview card mirrors it.

### Modified Capabilities

- `icon-url-only`: an app without `icon_url` falls back to the Better Rich Presence logo URL instead of the non-existent `"default"` asset key.
- `vsc-universal-presence`: IDE presences use the app-first text layout and the IDE's catalog icon URL instead of the `Editing …` / `Workspace: …` text and the non-existent asset keys (`rust-logo`, `file-icon`, IDE keys).

## Impact

- Rust: `services/engine.rs`, `services/discord.rs`, `services/activity_builder.rs`, `services/app_registry.rs`, `services/extension_mapping.rs` (removed), `services/ide_registry.rs`, `models/types.rs`, `Cargo.toml` (drop the `unstable_name` feature).
- Frontend: `src/components/PresenceCard.tsx`, `src/types.ts`. `src/pages/Dashboard.tsx` and `src/App.tsx` are owned by the in-flight `reorganize-main-window` change and are not touched here.
- Frontend contract: the `presence-updated` payload and `get_current_presence` return value gain `activity_name`. Existing fields are unchanged.
- Overlap: `rust-clean-architecture` (not started) moves these same Rust files and its task 7.2 also translates the idle text. Landing this change first turns that part of 7.2 into a no-op.
- Not in scope: registering separate Discord applications per app (that is the only way to change the bold name; the code already supports per-app `client_id`), and activity types other than Playing.
