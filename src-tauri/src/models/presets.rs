//! # Presets Module
//!
//! Contains default app rules to seed the store on first run.

use crate::domain::claude_code::{CLAUDE_CODE_DISPLAY_NAME, CLAUDE_CODE_RULE_ID};
use crate::models::types::AppRule;

/// Returns the default list of AppRules.
pub fn default_app_rules() -> Vec<AppRule> {
    let mut rules = Vec::new();
    ensure_claude_code_rule(&mut rules);
    rules
}

/// The reserved Apps-list entry that ranks Claude Code sessions.
fn claude_code_rule(enabled: bool) -> AppRule {
    let mut rule = AppRule::new(
        CLAUDE_CODE_RULE_ID,
        CLAUDE_CODE_DISPLAY_NAME,
        CLAUDE_CODE_DISPLAY_NAME,
        "",
        CLAUDE_CODE_RULE_ID,
    );
    rule.enabled = enabled;
    rule
}

/// Makes sure the list has the Claude Code entry. A rule for the Claude Code CLI
/// process (`claude`, `claude-code`, with or without `.exe`) is replaced in place,
/// keeping its position and enabled state; otherwise the entry goes to the top.
/// Returns whether the list changed.
pub fn ensure_claude_code_rule(rules: &mut Vec<AppRule>) -> bool {
    if rules.iter().any(|r| r.process_name == CLAUDE_CODE_RULE_ID) {
        return false;
    }
    let is_cli = |name: &str| {
        let name = name.trim().to_ascii_lowercase();
        matches!(name.trim_end_matches(".exe"), "claude" | "claude-code")
    };
    match rules.iter().position(|r| is_cli(&r.process_name)) {
        Some(idx) => rules[idx] = claude_code_rule(rules[idx].enabled),
        None => rules.insert(0, claude_code_rule(true)),
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(name: &str, enabled: bool) -> AppRule {
        let mut r = AppRule::new(name, name, "", "", "auto");
        r.enabled = enabled;
        r
    }

    #[test]
    fn inserts_the_entry_at_the_top() {
        let mut rules = vec![rule("code", true)];
        assert!(ensure_claude_code_rule(&mut rules));
        assert_eq!(rules[0].process_name, "claude_code");
        assert_eq!(rules[0].display_name, "Claude Code");
        assert!(rules[0].enabled);
        assert_eq!(rules[1].process_name, "code");
    }

    #[test]
    fn replaces_the_cli_rule_in_place() {
        let mut rules = vec![
            rule("antigravity", true),
            rule("claude", false),
            rule("code", true),
        ];
        assert!(ensure_claude_code_rule(&mut rules));
        let names: Vec<_> = rules.iter().map(|r| r.process_name.as_str()).collect();
        assert_eq!(names, ["antigravity", "claude_code", "code"]);
        assert!(!rules[1].enabled);
    }

    #[test]
    fn leaves_an_existing_entry_alone() {
        let mut rules = vec![
            rule("code", true),
            claude_code_rule(true),
            rule("claude", true),
        ];
        assert!(!ensure_claude_code_rule(&mut rules));
        assert_eq!(rules.len(), 3);
    }

    #[test]
    fn defaults_have_the_entry() {
        let rules = default_app_rules();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].process_name, "claude_code");
    }
}
