## MODIFIED Requirements

### Requirement: Rule presences keep the user's text
For a matching enabled rule, `details` and `state` SHALL be the rule's templates with `{file}` and `{title}` filled in. On the shared client, when `details` does not already contain the rule's display name, `details` SHALL be prefixed with `<display name> - `. When the rendered `details` is empty, `details` SHALL be the display name. When the matched application is an IDE or catalog editor (`category == "editor"`) with a recognizable open file, `{file}` SHALL resolve to the parsed filename and `small_image` / `small_text` SHALL carry the detected programming language icon URL and language name while preserving the user's custom `details` and `state`.

#### Scenario: Rule with templates
- **WHEN** a rule with display name `Notes`, details `Writing {title}` and state `Focus mode` matches a window titled `Plan`
- **THEN** `details` is `Notes - Writing Plan` and `state` is `Focus mode`
- **AND** `small_image` and `small_text` are empty

#### Scenario: Rule with empty details
- **WHEN** a matching rule with display name `Notes` has empty details and state `Focus mode`
- **THEN** `details` is `Notes` and `state` is `Focus mode`

#### Scenario: Rule with a legacy placeholder
- **WHEN** a matching rule with display name `Notes` has details `Using the computer` or `Usando o computador`
- **THEN** `details` is `Notes`

#### Scenario: Custom VS Code rule keeps text and shows language badge
- **WHEN** an enabled rule for `code.exe` has display name `VScode`, details `Coding`, and state `Working`, and the active window is `App.tsx - Better-Rich-Presence - Visual Studio Code`
- **THEN** `details` is `VScode - Coding`, `state` is `Working`, `large_text` is `VScode`
- **AND** `small_image` resolves to the jsDelivr `react.png` icon URL and `small_text` is `TypeScript`

#### Scenario: Custom Zed rule substitutes parsed filename and shows language badge
- **WHEN** an enabled rule for `zed.exe` has display name `Zed`, details `Hacking {file}`, and state `Focused`, and the active window is `Better-Rich-Presence — engine.rs`
- **THEN** `details` is `Zed - Hacking engine.rs`, `state` is `Focused`
- **AND** `small_image` resolves to the jsDelivr `rust.png` icon URL and `small_text` is `Rust`

#### Scenario: Custom editor rule without a recognized file clears badge
- **WHEN** an enabled custom rule for `code.exe` matches a workspace-only window `my-app - Visual Studio Code`
- **THEN** `small_image` and `small_text` are empty
