## Context

`sync_autostart_entry` (`infrastructure/autostart.rs`) runs at launch. If a `Run` value exists and is not disabled in `StartupApproved`, it computes a desired command through `select_startup_executable` (`domain/startup.rs`) and rewrites the value. The current rule is "if the running exe is a dev build, use the first available release exe", and the candidates include `target\release`. A `target\release` exe built without `tauri build` is still a dev build (`cfg!(dev)` is true there), so it loads `localhost:1420` at boot.

## Goals / Non-Goals

**Goals:**
- Never write a `Run` entry that targets an exe without an embedded frontend.
- Keep the selection rule pure and unit-tested in `domain/`.

**Non-Goals:**
- Inspecting another exe's binary to detect `custom-protocol`.
- Changing how the user enables or disables autostart in Settings.
- Linux autostart.

## Decisions

- **Only trust two sources.** The installed exe (put there by the Tauri installer, so always a standalone build) and the running exe when `!cfg!(dev)`. A `target\release` file is untrusted because a plain `cargo build --release` produces the same path. Rejected alternative: scan the binary for `localhost:1420`. It is fragile and couples to the dev URL.
- **New pure function.** `select_startup_executable(current_exe, is_dev, installed_exe: Option<&Path>) -> Option<PathBuf>`: returns the installed exe if present, else the current exe if not dev and not inside a `target` directory, else `None`. `None` means "do not write". A helper `is_cargo_target_path(&Path) -> bool` checks for a `target\{debug,release}` parent and is also used for the warning.
- **Warning only, no deletion.** When the selection is `None` and the existing entry is in a `target` directory, log a warning and leave the entry alone. Removing it silently would turn off a setting the user chose.

## Risks / Trade-offs

- A developer who only runs dev builds no longer gets autostart rewritten → acceptable. Autostart is meant for installed builds; the warning explains the situation.
- The existing broken entry on this machine stays until the app is installed or the setting is toggled → the warning in the log points to it.
