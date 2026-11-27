## Why

Chromium-based and Firefox-based browsers often share the same executable filename as their upstream base (for example, Helium Browser runs as `chrome.exe`, Opera GX runs as `opera.exe`, and Tor Browser runs as `firefox.exe`), causing Better Rich Presence to misidentify them as Google Chrome, Opera, or Mozilla Firefox and display the wrong app name and icon. Expanding the browser catalog (including Helium Browser, Chromium, LibreWolf, Opera GX, Tor Browser, Safari, and improved Wondershare Filmora matching) and disambiguating browsers by window title and executable path ensures each browser is accurately detected with its own icon.

## What Changes

- Add a pure domain module (`src-tauri/src/domain/browser.rs`) that disambiguates Chromium-based and Firefox-based browser forks using window title suffixes and executable paths, and handles em-dash (` — `) and en-dash (` – `) title separators used by Firefox-family browsers.
- Pass executable path metadata from Windows and Linux window watchers into the presence engine so browser disambiguation works even when window titles omit the browser suffix and in Max Priority Mode.
- Expand `apps.json` with missing browsers (`helium`, `chromium`, `librewolf`, `operagx`, `torbrowser`, `safari`), add common cross-platform executable aliases for existing browsers (`chrome`, `firefox`, `brave`, `edge`, `opera`, `vivaldi`, `zen`), and broaden executable matching for `wondersharefilmora`.
- Add the `helium.png` icon to `public/assets/icons/` and update `wondersharefilmora.png` to its colored logo, registering them in `scripts/icon-sources.json` under `keep` so `npm run assets:generate` preserves them.

## Capabilities

### New Capabilities
- `browser-detection`: Accurate identification, icon resolution, and window-title parsing for Chromium-based and Firefox-based browsers even when they share an upstream executable name.

### Modified Capabilities

## Impact

- **Backend (Rust)**: `src-tauri/src/domain/browser.rs` (new), `src-tauri/src/domain/mod.rs`, `src-tauri/src/models/types.rs`, `src-tauri/src/services/watcher.rs`, `src-tauri/src/services/linux/mod.rs`, `src-tauri/src/services/linux/process.rs`, `src-tauri/src/services/engine.rs`, `src-tauri/src/services/app_registry.rs`.
- **Catalog & Assets**: `apps.json`, `scripts/icon-sources.json`, `public/assets/icons/helium.png`, `public/assets/icons/wondersharefilmora.png`.
- **Frontend**: Automatically picks up the new `apps.json` entries and icons via existing catalog imports (no IPC contract changes).
