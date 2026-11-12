# Design

## Context

The window opens at 420×720 (minimum 360×550) and is often maximized. Everything is in `src/`: `App.tsx` renders the header, then `Dashboard`, `Apps` and an accordion with `Settings` in one scrolling column. Every responsive class uses the `sm:` breakpoint (640 px), which the default window never reaches, so the header always stacks into two rows. `App` and `Settings` each call `useSettings()` and keep separate copies. `Settings` handles that by re-reading `global_enabled` from the backend before it saves. Switches are clickable `div`s that cannot get keyboard focus. The visual language (tokens, borders, hard shadows, Space Grotesk + Inter) is fixed by `docs/DESIGN.md` and stays as it is. The React Compiler rules in `.agents/AGENTS.md` apply: no `try/finally`, and no conditional expressions inside `try`.

## Goals / Non-Goals

**Goals:**
- One place for each question (state → band, apps → Apps, behavior → Settings), with nothing duplicated across views.
- Keep the design system and make it calmer: one loud element per view, sentence-case body text, nothing below 11 px.
- Keyboard-operable tabs and switches.

**Non-Goals:**
- Routing, a state library, or new dependencies.
- Changes to hooks' public shapes, `src/types.ts`, or any Tauri command or event.
- Light or AMOLED theme work (the app forces dark today).

## Decisions

### 1. Tabs, not a sidebar or a longer scroll
Three tabs under the status band. A sidebar costs about 25 % of a 420 px window. A single scroll is the current problem. Tabs fit the narrow window, and each view has a clear job. The active tab is drawn as a pressed arcade button (shifted by the shadow offset, no shadow, Blurple fill), so the selection uses the design system's own press metaphor instead of a new indicator.

### 2. Keep all three views mounted and hide the inactive ones
Each view is wrapped in a `role="tabpanel"` element with the `hidden` attribute. Unmounting would throw away unsaved settings when the user switches tabs, and would re-run the loading state and backend fetches on every switch. The cost is that the preview timer and the 10 s platform-status poll keep running in the background, which is negligible.

Alternative considered: unmount the inactive views and lift the settings draft into `App`. That adds plumbing for no visible benefit.

### 3. The status band derives one state from three inputs
A pure function maps (`global_enabled`, `PresenceState`, `ConnectionInfo`) to `off | paused | live | connecting | disconnected`, in that order of precedence:

1. Sharing off: `off`.
2. Presence state `PausedByGame`: `paused`.
3. `connection.connected`, or presence state `Connected` or `Updating`: `live`.
4. Presence state `WaitingDiscord`: `connecting`.
5. Anything else: `disconnected`.

A table maps each state to its colors, label and indicator. `live` uses Blurple (the brand's normal state), `off` and `paused` use Yellow with dark text, and `connecting` and `disconnected` use the black surface with a yellow or red indicator. The band stays off Green, because `docs/DESIGN.md` reserves Green for the single highest-intent action. Green appears only as the "on" fill of switches and as the "Add app" button.

### 4. One accessible `Toggle`
`components/Toggle.tsx` renders `<button type="button" role="switch" aria-checked>` with the existing `.neo-toggle` classes and takes an `aria-label`. A native button gives focus, Space and Enter for free. It replaces the three hand-written toggles in `App.tsx`, `Settings.tsx` and `AppRuleCard.tsx`.

### 5. Settings save: one settings source, dirty tracking and a sticky footer
`App` owns the only `useSettings()` instance and passes `settings` and `updateSettings` to `Settings`. Before, `Settings` had a second copy and re-read `global_enabled` from the backend before saving. The backend never changes `global_enabled` by itself (only `update_settings` writes it), so one shared copy can't go stale, and the Apps view's priority hint updates as soon as Max priority mode is saved.

`Settings` keeps a draft. "Save changes" is enabled when any field other than `global_enabled` differs from the saved `settings`. Saving merges the draft with the current `global_enabled`, so the Share switch is never overwritten. Once the shared `settings` update, the button turns off again by itself. The button sits in a footer that stays at the bottom of the scrolling view (`sticky bottom-0`), so it stays visible without a second scroll container.

Alternative considered: save each setting immediately, like the Share switch. That is cleaner in principle, but numeric fields would then save on every keystroke or need blur handling. It would also change persistence behavior beyond this layout change.

### 6. Rule rows: one preview line, explicit edit, advanced fields folded
The collapsed row shows the position, icon, name and `First line · Second line`. The process name moves into the edit form header, because it is technical. The row body is a real `<button>` that opens the editor and carries a chevron. Image and Discord application ID sit under a "More options" disclosure that opens by default when either field is already set.

### 7. Sharing off dims only the preview
The `pointer-events-none` and opacity wrappers around Dashboard and Apps are removed. `Dashboard` receives a `sharingOff` flag, dims the card, and shows one line of guidance.

### 8. Width
The content area and the tab bar share one centered column (`max-w-[560px] mx-auto`). The title bar and status band stay full-bleed, like the design system's color bands.

## Risks / Trade-offs

- [Users who relied on editing being blocked while sharing is off] → Low risk: nothing is published while sharing is off, so editing is safe.
- [The preview harness mocks IPC, so it can miss backend timing issues] → The task list ends with a manual check in the running app.
- [Hidden views still run their effects] → Only a 1 s preview timer and a 10 s status poll, which are both cheap.
- [Wording drift between band labels and spec] → The labels live in one table in `StatusBand.tsx`, and the spec scenarios quote them.

## Migration Plan

Frontend-only and stateless: no stored data changes. Rollback is reverting the commit.
