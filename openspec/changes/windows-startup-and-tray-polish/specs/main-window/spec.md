## ADDED Requirements

### Requirement: Native OS window title stays Better Rich Presence
The native OS window title (`document.title`) SHALL remain `"Better Rich Presence"` regardless of which third-party application is currently detected on Discord, while the custom in-app title bar continues to display `"Better RPC"`.

#### Scenario: Detected app changes
- **WHEN** the detected app changes to Visual Studio Code or Firefox
- **THEN** the native OS window title remains `"Better Rich Presence"`
- **AND** the custom in-app header continues to display `"Better RPC"`
