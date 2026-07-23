# Universal IDE Rich Presence

## Why

The current Discord Rich Presence integration provides limited context about the user's development activity. In most cases, it only displays the name of the IDE, such as:

```text
Playing Visual Studio Code
```

This does not communicate what the user is actually working on.

The active window title already contains useful information about the current development context, including the active file, workspace, and IDE. By parsing this information, the application can provide a much richer and more dynamic Discord Rich Presence experience without requiring complex IDE extensions or direct integrations with each IDE.

The goal is to make the activity feel more personal and informative:

```text
Editing main.rs
my-project • Rust
```

With:

* The file type represented by the Large Image.
* The IDE represented by the Small Image.
* The current workspace displayed in the State.
* The session duration represented by the elapsed timestamp.

---

## What Changes

The current implementation will be extended with a universal IDE presence system.

Instead of implementing support specifically for VS Code-based IDEs, the new capability will be designed around the concept of an IDE context.

The capability will initially support:

* Visual Studio Code
* VSCodium
* Cursor
* Windsurf
* Antigravity IDE

The architecture should remain extensible to support additional development environments in the future, such as:

* Zed
* Sublime Text
* JetBrains IDEs
* Neovim
* Other editors and IDEs

The capability will be named:

```text
ide-universal-presence
```

The core idea is:

```text
Active Window
      ↓
Detection
      ↓
Parsing
      ↓
IdeContext
      ↓
Session Tracking
      ↓
Activity Builder
      ↓
Discord Rich Presence
```

---

## Architecture Direction

The implementation should separate the process into independent responsibilities.

### Window Detection

The active window title should first be checked to determine whether it belongs to a supported IDE.

This layer should be responsible only for identifying the IDE.

It should not know about Discord Activities, file extensions, or session timestamps.

---

### Window Title Parsing

Once a supported IDE is detected, the window title should be parsed into a normalized development context.

For example:

```text
main.rs - my-project - Cursor
```

Should become:

```text
IDE: Cursor
File: main.rs
Workspace: my-project
Extension: rs
```

This context should be represented independently from Discord:

```rust
struct IdeContext {
    pub ide: Ide,
    pub file_name: Option<String>,
    pub workspace_name: Option<String>,
    pub file_extension: Option<String>,
}
```

The parser should be robust enough to handle file names containing `-` by parsing the title from the right after identifying the IDE.

For example:

```text
README - v2.md - my-project - Cursor
```

Should be interpreted as:

```text
File: README - v2.md
Workspace: my-project
IDE: Cursor
```

The parser should also gracefully handle incomplete or unexpected window titles.

---

## IDE Registry

Supported IDEs should be represented through a centralized registry rather than being handled by hardcoded conditionals throughout the application.

Each IDE should define the information necessary for detection and Discord asset mapping.

Conceptually:

```text
IDE
├── Display Name
├── Window Title Identifier
└── Discord Small Image Asset
```

This allows new IDEs to be added without modifying the core parsing or Discord Activity logic.

Adding a new IDE should primarily consist of registering its identity and assets.

---

## Activity Generation

The parsed `IdeContext` should be converted into a Discord-specific Activity only after the development context has been normalized.

The Activity Builder should be responsible for transforming:

```text
IdeContext
```

into:

```text
DiscordActivity
```

The preferred activity format is:

```text
Details: Editing main.rs
State: my-project • Rust
```

The mapping should be:

```text
File Name
    ↓
Details

Workspace + Language
    ↓
State

File Extension
    ↓
Large Image

IDE
    ↓
Small Image
```

For example:

```text
main.rs - my-project - Cursor
```

Should result in:

```text
Details: Editing main.rs
State: my-project • Rust
Large Image: rust-logo
Small Image: cursor-logo
```

The Discord-specific activity generation must remain separate from the window title parser.

This allows the same `IdeContext` to be reused by future presence formats or features.

---

## Dynamic Asset Resolution

The Large Image should be dynamically selected based on the active file extension.

Examples:

```text
.rs  → rust-logo
.ts  → typescript-logo
.vue → vue-logo
.py  → python-logo
.go  → go-logo
```

The Small Image should represent the active IDE:

```text
Cursor              → cursor-logo
Visual Studio Code  → visual-studio-code
VSCodium            → vscodium
Windsurf            → windsurf
Antigravity         → antigravity
```

Unknown file extensions or missing mappings should use generic fallback assets instead of causing the Activity generation to fail.

---

## Development Session Tracking

The system should track the current development session in memory.

The session should be associated with the current:

```text
Workspace + IDE
```

Changing the active file inside the same workspace should not reset the elapsed timestamp.

Example:

```text
my-project + Cursor
    │
    ├── main.rs
    ├── lib.rs
    └── config.rs
```

All of these files should belong to the same session.

However, changing the workspace should start a new session:

```text
my-project
    ↓
another-project
```

Likewise, switching the active IDE should start a new session:

```text
Cursor
    ↓
Antigravity
```

This allows the Discord Activity to show how long the user has been working in the current development context.

---

## Expected Result

The final experience should transform generic activity:

```text
Playing Cursor
```

into contextual activity:

```text
Editing main.rs
my-project • Rust
```

With:

```text
Large Image: Rust
Small Image: Cursor
Timestamp: Development session start
```

The experience should update dynamically as the user:

* Changes files.
* Changes workspaces.
* Changes IDEs.
* Opens different supported development environments.

The implementation should avoid unnecessary Discord updates when the resulting Activity has not meaningfully changed.

---

## Design Principles

The implementation should follow these principles:

### Separation of Concerns

The window title parser should not know about Discord.

```text
Window Title
    ↓
IdeContext
    ↓
Discord Activity
```

### Extensibility

Adding a new IDE should not require rewriting the core system.

### Graceful Fallbacks

Unknown files, missing extensions, and unexpected window titles should never crash the application.

### Minimal Integration Requirements

The initial implementation should rely only on the active window title and local in-memory state.

No IDE extension should be required.

### Efficient Updates

Discord should only receive an update when the resulting activity meaningfully changes.

---

## Future Possibilities

This architecture could later be expanded to support richer development context, such as:

* Git branch information.
* Project type detection.
* Framework detection.
* Language-specific icons.
* Build or debug states.
* Terminal activity.
* Test execution states.
* AI coding assistant activity.
* Multiple presence providers.

However, these features are outside the scope of this initial implementation.

The initial goal is to establish a reliable and extensible foundation for universal IDE-based Discord Rich Presence.
