## 1. Clean apps.json

- [x] 1.1 Remove the `icon` field from all 286 entries in `apps.json`
- [x] 1.2 Remove or clear `icon_url` from the 31 apps currently pointing to `api.iconify.design` URLs

## 2. Rust backend — remove icon field and Iconify paths

- [x] 2.1 Remove `pub icon: Option<String>` from `AppEntry` struct in `src-tauri/src/services/app_registry.rs`
- [x] 2.2 In `engine.rs`, remove the `else if let Some(ref icon_str) = app.icon` fallback branch in `build_presence_from_rule()` (around L342-343)
- [x] 2.3 In `engine.rs`, remove the `else if let Some(ref icon_str) = app.icon` fallback branch in `build_fallback_presence()` (around L385-386)
- [x] 2.4 In `engine.rs`, remove the `icon: Option<String>` field from the Discord details destructure (around L28) and update the usage at L31 to use a different field or remove
- [x] 2.5 In `engine.rs` `get_discord_asset_key()`, remove the `else if icon_trimmed.contains(':')` branch (L505-517) — function should only handle URL inputs and plain string passthrough
- [x] 2.6 Remove `test_get_discord_asset_key_iconify_colon_format` test and add a test for plain string passthrough (e.g. `"default"` → `"default"`)

## 3. React frontend — remove Iconify fallback

- [x] 3.1 Remove `icon?: string` from `AppDefinition` interface in `src/apps/app-types.ts`
- [x] 3.2 In `src/components/AppIcon.tsx`, remove the Iconify API fallback branch (`else if (trimmed.includes(":"))` at L43-44) — non-URL non-catalog names should fall through to `<Monitor/>`
- [x] 3.3 In `src/App.tsx`, remove the Iconify favicon fallback (L29-33 where `presence.large_image.includes(":")` constructs an Iconify URL)

## 4. Verification

- [x] 4.1 Run `cargo test` to verify Rust tests pass
- [x] 4.2 Run `npm run build` (or equivalent) to verify React builds without errors
- [x] 4.3 Verify that apps with `icon_url` still display correctly and apps without it show the generic fallback
