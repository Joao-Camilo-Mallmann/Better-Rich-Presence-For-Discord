## ADDED Requirements

### Requirement: Disambiguate Chromium-based and Firefox-based browser forks
The system SHALL identify specific browser forks even when their running process shares an upstream executable name (such as `chrome.exe`, `chromium`, `opera.exe`, or `firefox.exe`) by inspecting the window title suffix and executable path.

#### Scenario: Helium Browser running as chrome.exe on Windows
- **WHEN** the active window process is `chrome.exe` with executable path `C:\Users\alice\AppData\Local\imput\Helium\Application\chrome.exe` and window title `GitHub - Helium`
- **THEN** the engine resolves the app as `helium` (`Helium Browser`) with `helium.png` instead of `Google Chrome`

#### Scenario: Helium Browser with bare or untitled window
- **WHEN** the active window process is `chrome.exe` with executable path `C:\Users\alice\AppData\Local\imput\Helium\Application\chrome.exe` and window title `New Tab`
- **THEN** the engine resolves the app as `helium` (`Helium Browser`) via the executable path

#### Scenario: Opera GX running as opera.exe on Windows
- **WHEN** the active window process is `opera.exe` with executable path `C:\Users\alice\AppData\Local\Programs\Opera GX\opera.exe`
- **THEN** the engine resolves the app as `operagx` (`Opera GX`) with `operagx.png` instead of standard `Opera`

#### Scenario: Tor Browser running as firefox.exe
- **WHEN** the active window process is `firefox.exe` with window title `DuckDuckGo — Tor Browser` or executable path containing `Tor Browser`
- **THEN** the engine resolves the app as `torbrowser` (`Tor Browser`) with `torbrowser.png` instead of `Firefox`

#### Scenario: Non-browser process is never misclassified
- **WHEN** the active window process is `code.exe` with window title `helium.rs - Visual Studio Code`
- **THEN** browser disambiguation returns no override and the process remains `code.exe`

### Requirement: Parse browser window titles across hyphen, em-dash, and en-dash separators
The system SHALL strip the trailing browser name from window titles separated by ` - `, ` — `, or ` – `, and fall back to `"Browsing the web"` when the title is empty, `"New Tab"`, or only the browser name.

#### Scenario: Firefox-family browser with em-dash title
- **WHEN** the active window is `Zen Browser` with title `Rust Documentation — Zen Browser`
- **THEN** the formatted presence state is `Browsing Rust Documentation`

#### Scenario: Bare browser name or New Tab title
- **WHEN** the active window is `Helium Browser` with title `Helium` or `New Tab - Helium`
- **THEN** the formatted presence state is `Browsing the web`

### Requirement: Expanded browser and Filmora catalog entries and icons
The catalog (`apps.json`) SHALL include `helium`, `chromium`, `librewolf`, `operagx`, `torbrowser`, and `safari` with valid `icon_url` entries backed by committed 256×256 PNGs in `public/assets/icons/`, broadened cross-platform executable lists for existing browsers and `wondersharefilmora`, and `scripts/icon-sources.json` SHALL track `helium` and `wondersharefilmora` under `keep` so `npm run assets:generate` passes with zero unmapped icons.

#### Scenario: Catalog lookup for newly added browsers
- **WHEN** `AppRegistry::find_app` is queried with `helium.exe`, `chromium`, `librewolf.exe`, `operagx`, `torbrowser`, `safari`, or `wondershare filmora.exe`
- **THEN** it returns the matching catalog entry with its dedicated `icon_url`
