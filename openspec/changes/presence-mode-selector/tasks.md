# Tasks

## 1. Frontend

- [x] 1.1 Add `src/components/SegmentedControl.tsx`: accessible radio group (arrow keys, roving tab index) in the Neo-Brutalist style.
- [x] 1.2 Give `SegmentedControl` an optional per-option `hint`: tooltip on hover and keyboard focus, hidden with Escape, linked with `aria-describedby`.
- [x] 1.3 In `src/pages/Apps.tsx`, add the "Presence mode" selector (Dynamic / Priority list, with hints) between the heading and the search field; `src/App.tsx` saves the choice immediately.
- [x] 1.4 In `src/pages/Settings.tsx`, remove the Priority group and `priority_mode_enabled` from the editable keys; saving keeps the current `priority_mode_enabled`.
- [x] 1.5 Update wording in `src/pages/Apps.tsx` and `src/pages/Dashboard.tsx` ("Priority list" / "Dynamic").

## 2. Backend

- [x] 2.1 Remove `EngineEvent::ManualProfile` from `src-tauri/src/models/types.rs` and its match arm in `src-tauri/src/services/engine.rs`.

## 3. Verification

- [x] 3.1 `cargo test` in `src-tauri/`, `bun run compiler:check`, `bun run build`. (Frontend checks rerun after the move to the Apps view; the backend did not change again.)
- [ ] 3.2 Manual check in the running app: switch modes in the Apps view, read both tooltips, Alt+Tab between two apps in the list, save an unrelated setting and confirm the mode is kept.

## 4. Documentation

- [x] 4.1 Update the feature table wording in `README.md`, `README.en.md`, `README.pt.md`.
- [x] 4.2 Document the segmented control and its tooltip in `docs/DESIGN.md`.
- [x] 4.3 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise). — N/A.
