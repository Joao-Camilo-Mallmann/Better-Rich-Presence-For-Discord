# Design

## Context

`Settings.priority_mode_enabled` already selects between two detection paths in `services/watcher.rs`: the foreground window, or the highest-ranked running rule. Only the presentation was a boolean switch in Settings. `EngineEvent::ManualProfile` is a leftover from a removed manual-profile feature: the variant and its engine arm exist, but no code constructs it.

## Decisions

### Keep the boolean setting

- **Decision**: The selector reads and writes `priority_mode_enabled` (`true` = Priority list, `false` = Dynamic).
- **Rationale**: Two modes map exactly onto the boolean. Renaming it to an enum would change the frontend contract and need a store migration for no behavior gain.
- **Alternative**: a `presence_mode` enum field. Worth it only when a third mode appears.

### The selector lives in the Apps view

- **Decision**: The selector sits in the Apps view, between the "Your apps" heading and the search field. The Settings view loses its Priority group.
- **Rationale**: The mode decides whether the order of the Apps list matters, so the control belongs next to the list. One place only, so the two views can never disagree.

### Immediate save

- **Decision**: Selecting an option saves the setting at once through the same `updateSettings` path the Share switch uses.
- **Rationale**: The Apps view has no "Save changes" button; every other control there (enable, reorder, edit) applies immediately.
- **Consequence**: `priority_mode_enabled` leaves the Settings draft. When Settings saves, it takes the current `priority_mode_enabled` from the saved settings, exactly as it already does for `global_enabled`, so a stale draft cannot revert the mode.

### Segmented control with tooltips

- **Decision**: The reusable `SegmentedControl` (`role="radiogroup"` with `role="radio"` buttons, arrow-key navigation, roving tab index, Neo-Brutalist tokens) accepts an optional `hint` per option. The hint of the hovered or focused option shows in a tooltip below the control; Escape hides it. Each radio is linked to its hint text with `aria-describedby`, so assistive technology reads it without the visual tooltip.
- **Rationale**: A switch cannot name both states. A radio group is the accessible pattern for one-of-two named choices, and the tooltip explains both options, not only the selected one.
- The line under the "Your apps" heading keeps describing the selected mode, including how it interacts with Claude Code.

### Wording

- "Max priority mode" becomes "Priority list"; the focus-following behavior is "Dynamic".
- Tooltip text:
  - Dynamic: "Discord shows whatever app you're using and switches along with you."
  - Priority list: "Discord sticks to the highest app in this list that's running, even while you use something else."
- The `main-window` Now and Apps requirements are reworded in this change. The `linux-activity-detection` requirement "Max Priority Mode on Linux" is renamed to "Priority list mode on Linux" with the same behavior.

### Remove `ManualProfile`

- Delete the variant and its match arm. No command, event or frontend code references it.

## Risks

- None for stored data: the field and its default (`true`) are unchanged.
- Settings and Apps both call `updateSettings` with a full settings object. The Settings save overlays the live `global_enabled` and `priority_mode_enabled`, so neither view overwrites the other's field.
