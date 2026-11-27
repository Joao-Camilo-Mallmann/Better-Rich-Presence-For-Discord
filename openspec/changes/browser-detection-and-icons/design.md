## Context

Better Rich Presence detects foreground and prioritized applications by executable filename (`process_name`) and matches them against user rules and `apps.json`. For browsers, this breaks down whenever a Chromium-based or Firefox-based fork retains its upstream executable name:

- **Helium Browser** runs as `chrome.exe` on Windows (`...\imput\Helium\Application\chrome.exe`) with window titles ending in ` - Helium`.
- **Opera GX** runs as `opera.exe` on Windows (`...\Opera GX\opera.exe`).
- **Tor Browser** runs as `firefox.exe` (`...\Tor Browser\Browser\firefox.exe`) with window titles ending in ` — Tor Browser`.
- Several popular browsers already have icons in `public/assets/icons/` (`chromium.png`, `librewolf.png`, `operagx.png`, `safari.png`, `torbrowser.png`) but are missing from `apps.json`.
- Firefox-family browsers (`Firefox`, `Zen Browser`, `LibreWolf`, `Tor Browser`) use an em-dash (` — `) separator between the page title and browser name, which `app_registry::title_subject` currently ignores because it only splits on `" - "`.

Additionally, Wondershare Filmora's icon in `public/assets/icons/wondersharefilmora.png` was auto-converted to `mono-white`, and its `apps.json` entry misses versioned Windows executables (e.g., `wondershare filmora.exe`, `filmora14.exe`).

## Goals / Non-Goals

**Goals:**
- Accurately identify Chromium-based and Firefox-based browsers (and their icons) even when they share an upstream executable name like `chrome.exe`, `opera.exe`, or `firefox.exe`.
- Keep all disambiguation and title-parsing logic pure inside `src-tauri/src/domain/browser.rs` (`std`-only, unit-tested without Tauri/OS dependencies) in accordance with the backend Clean Architecture rules.
- Add `helium`, `chromium`, `librewolf`, `operagx`, `torbrowser`, and `safari` to `apps.json`, expand executable aliases for existing browsers and Wondershare Filmora, and commit `helium.png` plus the colored `wondersharefilmora.png` under `keep` in `scripts/icon-sources.json`.
- Support ` - `, ` — `, and ` – ` separators when extracting the page subject from browser window titles.

**Non-Goals:**
- Changing the frontend/Tauri IPC contract.
- Extracting browser tab URLs via OS accessibility APIs or browser extensions.
- Reading Windows PE version-info resources (`GetFileVersionInfoW`) — executable path + window title suffix covers the target browsers cleanly and cross-platform without extra Win32 FFI complexity.

## Decisions

1. **Pure domain disambiguator (`domain/browser.rs`)**:
   - Exposes `disambiguate_browser(process_name: &str, exe_path: Option<&str>, window_title: &str) -> Option<&'static str>` returning the canonical catalog id (e.g., `"helium"`, `"operagx"`, `"torbrowser"`, `"zen"`, `"librewolf"`, `"brave"`, `"vivaldi"`, `"arc"`, `"chromium"`, `"edge"`, `"opera"`, `"chrome"`, `"firefox"`).
   - **Guard**: Only runs when the normalized `process_name` is a known browser executable or upstream base (`chrome`, `chromium`, `firefox`, `opera`, `brave`, `msedge`, `vivaldi`, `arc`, `zen`, `librewolf`, `torbrowser`, `helium`, and their common Linux variants). Non-browser processes (e.g., `code.exe` editing `helium.rs`) are never touched.
   - **Precedence**:
     1. Window title suffix (after the last ` - `, ` — `, or ` – `, or the entire title if it is a bare browser name like `"Helium"` or `"Zen Browser"`).
     2. Executable path segment matching (normalizing `\` to `/`, case-insensitive, checking directory names such as `/helium/`, `/opera gx/`, `/tor browser/`, `/librewolf/`, `/zen browser/`, `/zen/`, `/bravesoftware/`, `/vivaldi/`, `/chromium/`).
   - *Alternative considered*: Putting disambiguation inside `watcher.rs` (Win32-only). Rejected because it violates Clean Architecture, skips Linux, and is harder to unit test.

2. **Plumbing `exe_path` through `EngineEvent::WindowChanged` and priority scanning**:
   - Add `exe_path: Option<String>` to `EngineEvent::WindowChanged` and `WindowSnapshot`.
   - On Windows, `sysinfo::Process::exe()` already provides the full path without additional Win32 calls. On Linux, `/proc/<pid>/exe` is already read by `process::exe_name` and can also provide the full path string.
   - In `resolve_effective_name` (and Windows priority scan), when `disambiguate_browser` returns `Some(browser_id)`, that canonical id is prioritized ahead of the raw `process_name` so both `AppRegistry::find_app` and user `AppRule`s match the specific browser rather than the upstream base.

3. **Multi-dash browser title parsing**:
   - Add `strip_browser_title_suffix` / `extract_browser_subject` in `domain/browser.rs` (used by `app_registry::format_app_presence`) that splits on the rightmost `" - "`, `" — "`, or `" – "` separator, and treats bare browser names (e.g., `"Mozilla Firefox"`, `"Zen Browser"`, `"Google Chrome"`, `"Helium"`) or `"New Tab"` as having no page subject (`"Browsing the web"`).

4. **Icon management**:
   - Generate `public/assets/icons/helium.png` (256×256 transparent PNG, 8px margin matching `generate-colored-icons.mjs`) from the user-provided Helium logo and add `"helium"` to `"keep"` in `scripts/icon-sources.json`.
   - Update `public/assets/icons/wondersharefilmora.png` with the user-provided colored logo and move `"wondersharefilmora"` to `"keep"` (removing it from `"icons"`) so `npm run assets:generate` preserves the colored artwork instead of regenerating a white monochrome cutout.

## Risks / Trade-offs

- **Custom/portable install directories** → If a user installs Helium into an unrelated folder name AND a page title happens to omit the ` - Helium` suffix (e.g., a fullscreen PWA), disambiguation falls back to `chrome.exe`. Mitigation: checking both window title suffix and executable path covers standard installers, scoop/winget/portable folders, and normal browser windows.
- **Existing user rules for `chrome.exe` / `firefox.exe`** → Once Helium is disambiguated as `helium`, a user's generic `chrome.exe` rule will no longer apply to Helium (which is the desired behavior so Helium gets its own icon and name; users can add a dedicated `helium` rule if they want custom text).
- **jsDelivr icon cache** → New/updated PNGs (`helium.png`, `wondersharefilmora.png`) become live in Discord RPC after merging to `main` (per `docs/Icons.md`).
