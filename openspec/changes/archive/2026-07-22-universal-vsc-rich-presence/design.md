## Context

Currently, the Rust backend sets a static and generic Discord Rich Presence for IDEs, relying solely on the application process name. We want to provide a detailed, "Wow" factor presence without the overhead of building native IDE extensions. We have decided to use OS-level window title scanning, focusing entirely on VS Code and its forks (Cursor, VSCodium, Windsurf), as they all share a predictable window title format (`<file> - <workspace> - <ide name>`).

## Goals / Non-Goals

**Goals:**
- Extract File Name, Workspace, and IDE Name from active window titles.
- Map file extensions to specific Discord asset images (e.g., Rust logo for `.rs`).
- Track session duration per workspace.
- Support Visual Studio Code, Cursor, VSCodium, and Windsurf natively.

**Non-Goals:**
- Showing error/warning counts or Git branch data (this would require IDE plugins).
- Supporting IDEs outside of the VS Code family in this iteration (e.g., IntelliJ, Visual Studio).

## Decisions

### 1. Window Title Parsing Strategy
Instead of complex Regex, we will split the window title string using the exact separator ` - `.
Because file names or workspace names might contain ` - ` (e.g., `my-file.js - better-rich-presence`), we will parse from right to left:
- `parts.last()` -> IDE Name (e.g., `Visual Studio Code`, `Cursor`). We validate this against a known list of supported IDEs.
- `parts[len - 2]` -> Workspace Name.
- `parts[0..len-2].join(" - ")` -> File Name.

### 2. Asset Mapping
We will maintain a static map of file extensions to asset keys matching those uploaded to the Discord Developer Portal:
- `rs` -> `rust-logo`
- `ts`, `tsx` -> `typescript-logo`
- `js`, `jsx` -> `javascript-logo`
- `md` -> `markdown-logo`
- Default -> `file-icon`

### 3. Session Timestamps
To provide the `01:23 elapsed` timer, the Rust backend will hold an in-memory state tracking the `current_workspace` and `workspace_start_time`.
- If the active window changes but the `Workspace Name` remains the same, the timestamp is preserved.
- If the `Workspace Name` changes, `workspace_start_time` is reset to `now()`.

## Risks / Trade-offs

- **Risk**: A user configures their VS Code settings to change the window title format (`window.title` setting).
  - *Mitigation*: We will gracefully fallback to the generic "Playing <IDE Name>" if the split logic fails or doesn't match the expected 3+ parts.
- **Risk**: Missing Discord assets for niche file extensions.
  - *Mitigation*: Fallback to a generic file icon.
