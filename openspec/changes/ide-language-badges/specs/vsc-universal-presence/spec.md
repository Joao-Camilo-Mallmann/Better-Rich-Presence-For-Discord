## MODIFIED Requirements

### Requirement: VS Code Universal Parser
The system SHALL parse active window titles to detect VS Code derived IDEs and extract relevant data for Discord Rich Presence, cleaning dirty file indicators and remote workspace suffixes and supporting three-part titles, standalone file titles, and two-part workspace-only titles.

#### Scenario: Valid VS Code Window
- **WHEN** the active window title matches the pattern `<filename> - <workspace> - Visual Studio Code`
- **THEN** the system extracts filename and workspace accurately, and identifies the IDE as Visual Studio Code.

#### Scenario: Valid Forked IDE Window
- **WHEN** the active window title matches the pattern `<filename> - <workspace> - Cursor`
- **THEN** the system extracts filename and workspace accurately, and identifies the IDE as Cursor.

#### Scenario: Complex Filename
- **WHEN** the filename contains ` - ` (e.g., `my-file - test.rs - workspace - VSCodium`)
- **THEN** the system parses the IDE and workspace from the right side, grouping the remainder as the filename (`my-file - test.rs`).

#### Scenario: Dirty indicator and remote suffix cleanup
- **WHEN** the active window title is `● main.rs - my-app [WSL: Ubuntu] - Visual Studio Code`
- **THEN** the extracted filename is `main.rs` and the extracted workspace is `my-app`.

#### Scenario: Standalone file without workspace
- **WHEN** the active window title is `main.rs - Visual Studio Code` (or `● notes.txt - Visual Studio Code`)
- **THEN** the extracted filename is `main.rs` (or `notes.txt`) and workspace is `None`.

#### Scenario: Two-part workspace-only title
- **WHEN** the active window title is `my-app - Cursor`
- **THEN** the extracted workspace is `my-app` and filename is `None`.

#### Scenario: VS Code Insiders
- **WHEN** the active window title ends with ` - Visual Studio Code - Insiders`
- **THEN** the system identifies the IDE as Visual Studio Code.

## ADDED Requirements

### Requirement: Process-Gated Editor Detection
The system SHALL gate IDE and editor window-title parsing by process identity so non-editor applications (such as web browsers whose page titles end in ` - Cursor` or ` - Zed`) are never misclassified as IDEs.

#### Scenario: Browser page ending with an IDE name
- **WHEN** the foreground process is a non-editor catalog app (such as Firefox) and its window title is `main.rs - Cursor`
- **THEN** the system does not parse it as an IDE window and attaches no programming language badge.

### Requirement: Rich Parsing and Language Badges for Zed and Catalog Editors
When a foreground or prioritized process belongs to a catalog editor (`category == "editor"`, including Zed, JetBrains IDEs, Visual Studio, Sublime Text, Notepad++, Neovim, Vim, Emacs, Eclipse, NetBeans, Code::Blocks, RStudio, Spyder, Atom, Jupyter, and Xcode) and no custom rule text overrides it, the system SHALL parse its window title for workspace and active file, track session timestamps per workspace, and build the rich IDE activity with `<IDE> · <workspace>` (or `<IDE>`), `Editing <file> (<Language>)` (or `Editing <file>` / `Working on <workspace>` / `Active`), the editor's `large_image`, and the detected language's `small_image` and `small_text`.

#### Scenario: Zed native em-dash title
- **WHEN** the process is `zed.exe` and the window title is `Better-Rich-Presence-For-Discord — engine.rs`
- **THEN** `details` is `Zed · Better-Rich-Presence-For-Discord`, `state` is `Editing engine.rs (Rust)`, `large_image` resolves to the Zed icon, `small_image` resolves to `rust.png`, and `small_text` is `Rust`.

#### Scenario: JetBrains IDE title with path suffix
- **WHEN** the process is `pycharm64.exe` and the window title is `my-api – src/server.py [C:\work\my-api]`
- **THEN** `details` is `PyCharm · my-api`, `state` is `Editing server.py (Python)`, `large_image` resolves to the PyCharm icon, `small_image` resolves to `python.png`, and `small_text` is `Python`.

#### Scenario: Zed workspace-only title
- **WHEN** the process is `zed.exe` and the window title is `Better-Rich-Presence-For-Discord`
- **THEN** `details` is `Zed · Better-Rich-Presence-For-Discord`, `state` is `Working on Better-Rich-Presence-For-Discord`, and `small_image` / `small_text` are empty.
