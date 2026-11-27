## 1. Domain selection rule

- [x] 1.1 Add `is_cargo_target_path(&Path) -> bool` to `src-tauri/src/domain/startup.rs` (true when a parent chain is `target\debug` or `target\release`)
- [x] 1.2 Change `select_startup_executable` to `(current_exe, is_dev, installed_exe: Option<&Path>) -> Option<PathBuf>`: installed exe first; else current exe when not dev and not in a target dir; else `None`
- [x] 1.3 Unit tests: installed wins, dev without installed gives `None`, standalone current exe is used, current exe in `target\release` gives `None`, `is_cargo_target_path` cases

## 2. Windows adapter

- [x] 2.1 In `src-tauri/src/infrastructure/autostart.rs`, replace `collect_release_candidates` with a lookup of the installed exe only (drop `target\release`)
- [x] 2.2 When the selection is `None`, do not write; if the existing entry points into a target dir, log a warning naming the path
- [x] 2.3 Keep the `StartupApproved` check before any write

## 3. Verification

- [x] 3.1 `cargo test` in `src-tauri/` passes
- [x] 3.2 `cargo clippy` in `src-tauri/` has no new warnings
- [ ] 3.3 Manual: run a dev build with the current `target\release` `Run` entry, confirm the entry is unchanged and the warning appears in the log

## 4. Documentation

- [x] 4.1 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise) — N/A
