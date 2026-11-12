# Tasks

## 1. Shared controls

- [x] 1.1 Add `components/Toggle.tsx` (button with `role="switch"`, `aria-checked`, `aria-label`, `.neo-toggle` styling) and verify in the preview that it toggles with a click and with Space when focused
- [x] 1.2 Add `components/StatusBand.tsx` with the state derivation from design decision 3 and the label/color table, and verify in the preview that `?s=live`, `?s=off` and `?s=disconnected` show "Live on Discord", "Sharing off" and "Not connected to Discord"

## 2. Window shell

- [x] 2.1 Rebuild the title bar in `App.tsx` on one row (logo, "Better RPC", icon window buttons with the maximize/restore swap) and verify at 420 px and 360 px wide that nothing wraps
- [x] 2.2 Add the status band with the Share switch under the title bar, replacing the header toggle, and verify the switch saves `global_enabled` and updates the band
- [x] 2.3 Add the Now / Apps / Settings tabs (`role="tablist"`, arrow-key navigation, pressed style for the active tab) with all views mounted and the inactive ones `hidden`, remove the accordion and the sharing-off dimming wrappers, center the content column (`max-w-[560px]`), and verify tab switching with mouse and arrow keys and the column width when maximized

## 3. Views

- [x] 3.1 Rework `pages/Dashboard.tsx` into the Now view (preview without the extra box, "What friends see" tag, priority notice, sharing-off note, no Engine/Session chips) and verify `?s=live`, `?s=off` and `?s=priority` in the preview
- [x] 3.2 Rework `pages/Apps.tsx` and `components/AppList.tsx` (heading, "Add app" button, priority explanation, search, empty state copy) and verify `?s=live` and `?s=empty` in the preview
- [x] 3.3 Rework `components/AppRuleCard.tsx` (one preview line, row button with chevron, `Toggle`, edit form labels Name / First line / Second line, "More options" with Image and "Discord application ID (optional)", process name in the form header, no Portuguese text) and verify edit, save, cancel, delete, move and enable in the preview
- [x] 3.4 Update `components/ProcessPicker.tsx` (close on Escape and backdrop click, honest footer and placeholder copy) and verify both close paths in the preview
- [x] 3.5 Rework `pages/Settings.tsx` and `components/DetectionStatus.tsx` (hint notice on top, groups Startup / Priority / When you're away / Timing / Activity detection, plain labels and help text, `Toggle`, sticky "Save changes" enabled only when dirty, Share state untouched on save) and verify `?s=live` and `?s=hint` in the preview, including that the button is disabled until a change and again after saving

## 4. Verification

- [x] 4.1 Run `bun run compiler:check` and verify no function in the touched files is reported (the existing `AssignmentPattern` reports for untouched code aside)
- [x] 4.2 Run `bun run build` and verify it succeeds
- [ ] 4.3 Manual check in the running app (`bun run tauri dev`): window buttons, maximize/restore icon, Share switch, each band state that can be reproduced (close Discord for "Not connected"), tabs, rule editing and reordering, settings save

## 5. Documentation

- [x] 5.1 Add a short "App window layout" note to `docs/DESIGN.md` (title bar, status band states and colors, tabs as pressed buttons) and verify it matches the shipped labels
- [x] 5.2 Update `.agents/AGENTS.md` if conventions, workflows, or reference docs changed (mark N/A otherwise) — N/A: no agent convention or workflow changed; the layout rules live in `docs/DESIGN.md`, which AGENTS.md already links.

## Workflow follow-up

- Archive the change with `/opsx:archive` after the manual check passes.
