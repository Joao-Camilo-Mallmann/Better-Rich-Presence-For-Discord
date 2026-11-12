# Proposal

## Why

The main window is one long scroll that is hard to read at its default 420×720 size. The header wraps onto two rows. The live preview sits inside a second decorative box. The Discord connection state is buried below the preview. Settings hide in an accordion at the very bottom and need their own Save button, while every other switch saves at once. The app list hides how priority works, and the edit form still has Portuguese text. Users have to scroll and guess to answer the three questions the window exists for: "Is Discord showing my activity?", "Which apps are shown, and how?", and "How do I change the behavior?"

## What Changes

- Title bar on a single row at every window width: logo, the static "Better RPC" name, and the minimize, maximize/restore and close buttons drawn with icons.
- A full-width status band under the title bar. Its color and label show the state: live on Discord, sharing off, connecting, not connected, or paused for a game. It holds the global Share switch, which replaces the "OFF / ACTIVE" toggle in the header.
- Three tabs, "Now", "Apps" and "Settings", replace the single scroll and the bottom "Advanced Settings" accordion.
- **Now** tab: the Discord profile preview without the extra box around it, a small "What friends see" tag, the Max priority mode notice when it applies, and a note when sharing is off. The "Engine" and "Session" chips are removed: the band shows the connection and the preview shows the elapsed time.
- **Apps** tab: a heading with an "Add app" button, one line explaining that list order sets priority in Max priority mode, search, and app rows with a single preview line and a visible edit control. The edit form uses plain labels (Name, First line, Second line) and moves Image and Discord application ID under "More options". The Portuguese "Opcional" is translated. The app picker closes on Escape and on a click outside it, and its footer no longer claims arrow-key navigation it does not have.
- **Settings** tab: settings grouped as Startup, Priority, When you're away, Timing and Activity detection, with plain-language labels and help text. The detection hint stays above the settings list. A "Save changes" button stays visible at the bottom and is enabled only when something changed.
- One accessible switch control (a button with the `switch` role) replaces the clickable `div` toggles, so every switch works with the keyboard.
- **Behavior change**: turning sharing off no longer dims and locks the whole window. Apps and settings stay editable, and only the preview is dimmed.
- When the window is maximized, the content keeps a readable column width instead of stretching.

Non-goals: no new settings, no change to Tauri commands, events or payloads, no backend changes, and no change to the design tokens in `docs/DESIGN.md`.

## Capabilities

### New Capabilities
- `main-window`: How the main window is organized and what it shows: title bar, status band with the global switch, the Now / Apps / Settings views, and the behavior of their controls.

### Modified Capabilities
<!-- None. The `detection-status` requirements from the in-flight `linux-desktop-support` change (notice above the settings list, backends shown, "Start on login" label) are kept as written. -->

## Impact

- **Frontend only** (`src/`): `App.tsx` (title bar, status band, tabs), `pages/Dashboard.tsx` (Now), `pages/Apps.tsx`, `pages/Settings.tsx`, `components/AppRuleCard.tsx`, `components/AppList.tsx`, `components/ProcessPicker.tsx`, `components/DetectionStatus.tsx`, new `components/Toggle.tsx` and `components/StatusBand.tsx`, and small additions to `globals.css`.
- **Unchanged**: Tauri commands and events, `src/types.ts` payloads, the Rust backend, and the design tokens.
- **Dependencies**: none added (icons come from `lucide-react`, already installed).
