## ADDED Requirements

### Requirement: Target-aware log filtering
The backend logger SHALL filter log records by module target so internal application diagnostics remain visible while third-party crate noise is suppressed by default, and SHALL support `RUST_LOG` overrides.

#### Scenario: Default filtering suppresses external trace and debug spam
- **WHEN** `RUST_LOG` is unset or empty
- **THEN** internal targets (`better_rich_presence_lib` and `better_rich_presence` and their submodules) SHALL allow `DEBUG` and higher severity while blocking `TRACE`, and external crate targets (such as `discord_presence`) SHALL allow `WARN` and `ERROR` while blocking `INFO`, `DEBUG`, and `TRACE`

#### Scenario: RUST_LOG overrides target verbosity
- **WHEN** `RUST_LOG` contains comma-separated directives such as `warn,better_rich_presence_lib::services::engine=trace,discord_presence=debug`
- **THEN** the logger SHALL apply the longest matching target prefix rule (falling back to the global directive) and ignore empty or invalid tokens without panicking

### Requirement: Colorized structured terminal output and plain-text event emission
The backend logger SHALL format terminal output with aligned timestamp, level, component tag, and ANSI colors (when color is enabled), while emitting ANSI-free text to the `"app-log"` Tauri event.

#### Scenario: Terminal lines are aligned and colorized by default
- **WHEN** color output is enabled (and `NO_COLOR` is unset/empty, `CLICOLOR` is not `"0"`, and `TERM` is not `"dumb"`)
- **THEN** terminal lines SHALL include `HH:MM:SS`, aligned level, aligned component tag (`[Watcher]`, `[Engine]`, `[Discord]`, `[Claude]`, `[Linux]`, `[App]`, or external crate name), ANSI color styling with resets, and highlighted `Switch:`, `Active:`, and `Update:` transitions

#### Scenario: Color output respects NO_COLOR and non-color flags
- **WHEN** `NO_COLOR` is set to a non-empty value, `CLICOLOR=0`, or `TERM=dumb` (and `CLICOLOR_FORCE` is not overriding it)
- **THEN** terminal lines SHALL omit all ANSI escape sequences while preserving timestamps, levels, component tags, and messages

#### Scenario: Frontend app-log event remains plain text
- **WHEN** a log record is emitted to the frontend `"app-log"` Tauri event
- **THEN** the `LogPayload.message` field SHALL NOT contain ANSI escape sequences

### Requirement: Human-readable presence transition and app switch logging
Backend services SHALL log friendly application names, `from → to` presence switches, rate-limit coalescing queues, Claude Code overrides, and idle transitions without redundant manual component prefixes.

#### Scenario: Presence engine logs active, switch, and update transitions
- **WHEN** the engine commits an initial presence, switches from one app to another, or updates the same app's details
- **THEN** it SHALL log `Active: <app> | <summary>`, `Switch: <from> → <to> | <summary>`, or `Update: <app> | <summary>` at `INFO` level

#### Scenario: Rate-limited updates and window changes log app names clearly
- **WHEN** a window change occurs or an update is coalesced due to Discord's rate limit
- **THEN** the watcher SHALL log the detected process and UTF-8-safe truncated title at `DEBUG` level, and the engine SHALL log the queued app name, remaining delay, and summary at `DEBUG` level
