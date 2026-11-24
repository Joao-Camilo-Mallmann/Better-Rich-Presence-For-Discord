# Proposal

## Why

Presence selection used to be an on/off "Max priority mode" switch in Settings. That hides the fact that there are two distinct behaviors, the "off" behavior has no name, and the switch sits far from the Apps list whose order it gives meaning to. Users should pick between two named modes, right above the list they affect, with a short explanation of each.

## What Changes

- The Apps view shows a two-option "Presence mode" selector above the app list:
  - **Dynamic**: Discord follows the app in focus and changes as the user switches windows (Alt+Tab).
  - **Priority list**: Discord shows the highest running app in the Apps list, even when another app is in focus.
- Each option has a tooltip, shown on hover and on keyboard focus, that explains in one friendly sentence what the mode does.
- Choosing an option saves immediately, like the Share switch. There is no "Save changes" step for it.
- The Settings view no longer has a Priority group, and "Save changes" never changes the presence mode.
- The stored setting stays `priority_mode_enabled` (`true` = Priority list, `false` = Dynamic), so there is no contract change and no migration.
- User-facing text that said "Max priority mode" in the Apps and Now views now names the modes "Priority list" and "Dynamic".
- Remove the legacy manual-profile engine event (`EngineEvent::ManualProfile`) and its handler; nothing sends it.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `linux-activity-detection`: the Max Priority Mode requirement is renamed to Priority list mode; behavior is unchanged.
- `main-window`: the Apps view offers the presence mode selector with tooltips; the Settings view drops the Priority group; saving settings leaves the presence mode alone; Now and Apps wording uses the new mode names.

## Impact

- Frontend: `src/components/SegmentedControl.tsx` (per-option tooltip), `src/pages/Apps.tsx` (selector), `src/App.tsx` (immediate save), `src/pages/Settings.tsx` (remove the Priority group), wording in `src/pages/Dashboard.tsx`.
- Backend: `src-tauri/src/models/types.rs` and `src-tauri/src/services/engine.rs` (remove `ManualProfile`). Detection behavior is unchanged.
- Docs: README feature table wording, and the segmented control and tooltip rules in `docs/DESIGN.md`.
