# Spec Delta

## MODIFIED Requirements

### Requirement: Session tracking, precedence, and auto-expiry
The system SHALL track concurrent Claude Code sessions by `session_id` and restore normal app detection when sessions end, expire, or integration is disabled. Whether the most recently active session replaces the window presence SHALL follow the position of the Claude Code entry (`process_name` `claude_code`) in the Apps priority list.

#### Scenario: Active session overrides window detection
- **WHEN** `claude_code_enabled` is `true`, a Claude Code session is active, and the app selected by window detection has no enabled rule ranked above the Claude Code entry
- **THEN** the engine publishes the Claude Code presence (`details: "Claude Code"`, `large_image` resolved to `claude.png`, `large_text: "Claude Code"`, `timestamp` set to session start)

#### Scenario: Higher-ranked app wins over Claude Code
- **WHEN** a Claude Code session is active and the selected window app is `code` whose enabled rule is ranked above the Claude Code entry
- **THEN** the engine publishes the Visual Studio Code window presence
- **AND** when the selected window app changes to an app ranked below the Claude Code entry, the Claude Code presence is published again

#### Scenario: Disabled Claude Code entry
- **WHEN** the Claude Code entry in the Apps list is disabled
- **THEN** Claude Code sessions never replace the window presence

#### Scenario: Claude Code entry is created
- **WHEN** the app starts and the saved Apps list has no Claude Code entry and no rule for the Claude Code CLI process (`claude`, `claude.exe`, `claude-code`, `claude-code.exe`)
- **THEN** an enabled Claude Code entry is inserted at position 1

#### Scenario: Existing Claude rule becomes the Claude Code entry
- **WHEN** the app starts and the saved Apps list has a rule for process `claude` at position 2, disabled or enabled
- **THEN** that rule is replaced in place by the Claude Code entry, keeping position 2 and its enabled state

#### Scenario: Session end restores desktop window presence
- **WHEN** a `SessionEnd` event closes the last active Claude Code session
- **THEN** the engine immediately restores presence from the latest cached window snapshot

#### Scenario: Inactivity auto-expiry
- **WHEN** a waiting Claude Code session receives no hook events for `claude_code_timeout_minutes` (default 5 minutes), or an in-flight session receives no events for the safety timeout (`max(timeout * 2, 15)` minutes)
- **THEN** the engine prunes the expired session and restores the latest cached window presence

#### Scenario: Disabling in Settings restores window detection
- **WHEN** the user turns off `claude_code_enabled` while a Claude Code session is active
- **THEN** the engine immediately restores normal window-based presence

#### Scenario: Rapid hook bursts do not stall the engine
- **WHEN** multiple hook events arrive within Discord's 15-second rate-limit window
- **THEN** the engine coalesces pending updates without blocking its event loop and publishes the latest state once the rate-limit window elapses
