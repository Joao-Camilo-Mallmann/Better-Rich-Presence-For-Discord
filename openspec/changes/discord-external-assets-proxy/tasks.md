## 1. Engine Asset Key & URL Preservation

- [x] 1.1 Update `get_discord_asset_key` in `src-tauri/src/services/engine.rs` to preserve complete HTTP/HTTPS URLs when present instead of stripping down to key fragments.
- [x] 1.2 Update presence builder functions in `engine.rs` to prioritize external icon URLs when specified in application configurations.

## 2. Discord RPC External Asset Formatting

- [x] 2.1 Refactor asset assignment in `src-tauri/src/services/discord.rs` to support external image URLs via Discord proxy image formats or direct HTTPS asset payloads.
- [x] 2.2 Add fallback logic to return to `"default"` or a valid static asset key if an external URL format is invalid or unreachable.

## 3. Verification & Testing

- [x] 3.1 Test Discord RPC updates with Zen Browser (`zenbrowser` / external HTTPS icon URL) to verify the large image renders without displaying `?`.
- [x] 3.2 Verify existing static asset keys continue to function as expected without regression.
