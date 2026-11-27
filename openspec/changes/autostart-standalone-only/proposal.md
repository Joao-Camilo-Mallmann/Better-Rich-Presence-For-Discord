## Why

At Windows sign-in the app opened a blank window with `localhost refused to connect` (`ERR_CONNECTION_REFUSED`). The `Run` registry entry pointed to `src-tauri\target\release\better-rich-presence.exe`, which was built with a plain `cargo build --release`. That build has no `custom-protocol` feature, so it loads the dev server URL (`http://localhost:1420`) instead of the embedded frontend, and no dev server runs at boot.

The cause is in the code: `select_startup_executable` prefers any `target\release` executable over the current one, and `sync_autostart_entry` rewrites the `Run` entry to it. A file in `target\release` says nothing about whether the frontend is embedded.

## What Changes

- Autostart selection only accepts the installed executable (`%LOCALAPPDATA%\Better Rich Presence\better-rich-presence.exe`) or the current executable when it is a standalone build (not `cfg!(dev)`).
- `target\release` is no longer a candidate.
- A dev build never writes or normalizes the `Run` entry to itself.
- When the existing `Run` entry points into a Cargo `target` directory and no installed executable exists, the entry is left untouched and a warning is logged.
- Entries the user disabled in Task Manager or MSConfig stay disabled (unchanged behavior).

## Capabilities

### New Capabilities
- `windows-autostart`: How the Windows `Run` entry for Better Rich Presence is chosen and normalized so startup always launches a build with an embedded frontend.

### Modified Capabilities
<!-- none -->

## Impact

- `src-tauri/src/domain/startup.rs`: `select_startup_executable` gets a new signature and rules, plus unit tests.
- `src-tauri/src/infrastructure/autostart.rs`: candidate collection drops `target\release`; sync skips writes from dev builds and logs a warning for target-dir entries.
- No frontend contract or Tauri command changes.
