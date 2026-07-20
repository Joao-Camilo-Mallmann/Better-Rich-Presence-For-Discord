//! # Presets Module
//!
//! Contains default app rules to seed the store on first run.

use crate::models::types::AppRule;

/// Returns the default list of AppRules.
pub fn default_app_rules() -> Vec<AppRule> {
    Vec::new()
}
