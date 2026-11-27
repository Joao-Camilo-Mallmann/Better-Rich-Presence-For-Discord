//! Non-destructive management of Better RPC HTTP hooks in Claude Code's
//! `settings.json` (`$CLAUDE_CONFIG_DIR/settings.json` or `~/.claude/settings.json`).

use crate::domain::claude_code::{
    ClaudeHookInstallState, CLAUDE_CODE_HOOK_EVENTS, CLAUDE_CODE_HOOK_PATH,
};
use serde_json::{json, Map, Value};
use std::path::{Path, PathBuf};

/// Short hook timeout (seconds) so Claude Code never stalls if Better RPC is closed.
const HOOK_TIMEOUT_SECONDS: u64 = 2;

/// Builds the localhost endpoint URL for the given port.
pub fn hook_endpoint_url(port: u16) -> String {
    format!("http://127.0.0.1:{port}{CLAUDE_CODE_HOOK_PATH}")
}

/// Resolves the user's Claude Code `settings.json` path cross-platform.
///
/// Honors `CLAUDE_CONFIG_DIR` when set and non-empty, otherwise defaults to
/// `<home>/.claude/settings.json`.
pub fn resolve_settings_path() -> Result<PathBuf, String> {
    if let Ok(dir) = std::env::var("CLAUDE_CONFIG_DIR") {
        let trimmed = dir.trim();
        if !trimmed.is_empty() {
            return Ok(PathBuf::from(trimmed).join("settings.json"));
        }
    }

    let home = home_dir().ok_or_else(|| {
        "Could not determine home directory for ~/.claude/settings.json".to_string()
    })?;
    Ok(home.join(".claude").join("settings.json"))
}

fn home_dir() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        let trimmed = home.trim();
        if !trimmed.is_empty() {
            return Some(PathBuf::from(trimmed));
        }
    }
    if let Ok(profile) = std::env::var("USERPROFILE") {
        let trimmed = profile.trim();
        if !trimmed.is_empty() {
            return Some(PathBuf::from(trimmed));
        }
    }
    if let (Ok(drive), Ok(path)) = (std::env::var("HOMEDRIVE"), std::env::var("HOMEPATH")) {
        let combined = format!("{}{}", drive.trim(), path.trim());
        if !combined.is_empty() {
            return Some(PathBuf::from(combined));
        }
    }
    None
}

/// Whether a single hook handler JSON object belongs to Better RPC.
pub fn is_better_rpc_hook(handler: &Value) -> bool {
    let Some(obj) = handler.as_object() else {
        return false;
    };
    if obj.get("type").and_then(Value::as_str) != Some("http") {
        return false;
    }
    let Some(url) = obj.get("url").and_then(Value::as_str) else {
        return false;
    };
    let is_loopback = url.starts_with("http://127.0.0.1:") || url.starts_with("http://localhost:");
    is_loopback && url.ends_with(CLAUDE_CODE_HOOK_PATH)
}

fn better_rpc_matcher_group(port: u16) -> Value {
    json!({
        "hooks": [
            {
                "type": "http",
                "url": hook_endpoint_url(port),
                "timeout": HOOK_TIMEOUT_SECONDS
            }
        ]
    })
}

/// Inspects a parsed `settings.json` value to determine whether Better RPC hooks
/// are absent, installed and up to date, or outdated/partial.
pub fn inspect_hooks_in_json(root: &Value, port: u16) -> ClaudeHookInstallState {
    let Some(hooks) = root.get("hooks").and_then(Value::as_object) else {
        return ClaudeHookInstallState::NotInstalled;
    };

    let expected_url = hook_endpoint_url(port);
    let mut found_any = false;
    let mut all_expected_exact = true;
    let mut has_stale_or_duplicate = false;

    for (event_name, groups_val) in hooks {
        let Some(groups) = groups_val.as_array() else {
            continue;
        };
        let mut count_for_event = 0usize;
        for group in groups {
            let Some(handlers) = group.get("hooks").and_then(Value::as_array) else {
                continue;
            };
            for handler in handlers {
                if is_better_rpc_hook(handler) {
                    found_any = true;
                    count_for_event += 1;
                    let url_matches =
                        handler.get("url").and_then(Value::as_str) == Some(expected_url.as_str());
                    if !url_matches {
                        has_stale_or_duplicate = true;
                    }
                }
            }
        }
        if count_for_event > 0 {
            if !CLAUDE_CODE_HOOK_EVENTS.contains(&event_name.as_str()) || count_for_event > 1 {
                has_stale_or_duplicate = true;
            }
        }
    }

    if !found_any {
        return ClaudeHookInstallState::NotInstalled;
    }

    for &event in CLAUDE_CODE_HOOK_EVENTS {
        let has_exact = hooks
            .get(event)
            .and_then(Value::as_array)
            .is_some_and(|groups| {
                groups.iter().any(|group| {
                    group
                        .get("hooks")
                        .and_then(Value::as_array)
                        .is_some_and(|handlers| {
                            handlers.iter().any(|h| {
                                is_better_rpc_hook(h)
                                    && h.get("url").and_then(Value::as_str)
                                        == Some(expected_url.as_str())
                            })
                        })
                })
            });
        if !has_exact {
            all_expected_exact = false;
            break;
        }
    }

    if all_expected_exact && !has_stale_or_duplicate {
        ClaudeHookInstallState::Installed
    } else {
        ClaudeHookInstallState::Outdated
    }
}

/// Installs or updates Better RPC's HTTP hooks in `root` without touching any
/// other user settings or user-defined hooks. Idempotent.
pub fn install_hooks_in_json(root: &mut Value, port: u16) -> Result<(), String> {
    let root_obj = root
        .as_object_mut()
        .ok_or_else(|| "Claude Code settings.json root must be a JSON object".to_string())?;

    if !root_obj.contains_key("hooks") {
        root_obj.insert("hooks".to_string(), Value::Object(Map::new()));
    }

    // Remove any existing Better RPC hooks first so reinstall/port changes never duplicate.
    let hooks_obj = root_obj
        .get_mut("hooks")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| {
            "Expected \"hooks\" in Claude Code settings.json to be an object".to_string()
        })?;

    strip_better_rpc_from_hooks_map(hooks_obj);

    let entry = better_rpc_matcher_group(port);
    for &event in CLAUDE_CODE_HOOK_EVENTS {
        if !hooks_obj.contains_key(event) {
            hooks_obj.insert(event.to_string(), Value::Array(Vec::new()));
        }
        let arr = hooks_obj
            .get_mut(event)
            .and_then(Value::as_array_mut)
            .ok_or_else(|| {
                format!("Expected hooks.\"{event}\" in Claude Code settings.json to be an array")
            })?;
        arr.push(entry.clone());
    }

    Ok(())
}

/// Removes ONLY Better RPC's HTTP hooks from `root` and prunes empty wrapper
/// groups/arrays/objects that became empty as a result. Returns `true` if any
/// entry was removed.
pub fn remove_hooks_from_json(root: &mut Value) -> Result<bool, String> {
    let root_obj = root
        .as_object_mut()
        .ok_or_else(|| "Claude Code settings.json root must be a JSON object".to_string())?;

    let Some(hooks_val) = root_obj.get_mut("hooks") else {
        return Ok(false);
    };

    let hooks_obj = hooks_val.as_object_mut().ok_or_else(|| {
        "Expected \"hooks\" in Claude Code settings.json to be an object".to_string()
    })?;

    let removed = strip_better_rpc_from_hooks_map(hooks_obj);

    if removed && hooks_obj.is_empty() {
        root_obj.remove("hooks");
    }

    Ok(removed)
}

fn strip_better_rpc_from_hooks_map(hooks_obj: &mut Map<String, Value>) -> bool {
    let mut removed_any = false;
    let mut empty_events = Vec::new();

    for (event_name, groups_val) in hooks_obj.iter_mut() {
        let Some(groups) = groups_val.as_array_mut() else {
            continue;
        };

        let mut event_removed = false;
        groups.retain_mut(|group| {
            let Some(handlers) = group.get_mut("hooks").and_then(Value::as_array_mut) else {
                return true;
            };
            let before = handlers.len();
            handlers.retain(|h| !is_better_rpc_hook(h));
            if handlers.len() != before {
                event_removed = true;
                removed_any = true;
                // Drop this matcher group only if removing Better RPC emptied it.
                !handlers.is_empty()
            } else {
                true
            }
        });

        if event_removed && groups.is_empty() {
            empty_events.push(event_name.clone());
        }
    }

    for event_name in empty_events {
        hooks_obj.remove(&event_name);
    }

    removed_any
}

/// Reads and inspects `path` on disk. A missing file reports `NotInstalled`.
pub fn inspect_settings_file(path: &Path, port: u16) -> Result<ClaudeHookInstallState, String> {
    if !path.exists() {
        return Ok(ClaudeHookInstallState::NotInstalled);
    }
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {e}", path.display()))?;
    if content.trim().is_empty() {
        return Ok(ClaudeHookInstallState::NotInstalled);
    }
    let value: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Invalid JSON in {}: {e}", path.display()))?;
    Ok(inspect_hooks_in_json(&value, port))
}

/// Installs Better RPC hooks into `path`, creating parent directories and the
/// file if needed, and writing atomically.
pub fn install_hooks_at(path: &Path, port: u16) -> Result<(), String> {
    let mut root = read_or_empty_object(path)?;
    install_hooks_in_json(&mut root, port)?;
    write_json_atomic(path, &root)
}

/// Removes Better RPC hooks from `path` if present. Succeeds as a no-op if the
/// file does not exist.
pub fn remove_hooks_at(path: &Path) -> Result<bool, String> {
    if !path.exists() {
        return Ok(false);
    }
    let mut root = read_or_empty_object(path)?;
    let removed = remove_hooks_from_json(&mut root)?;
    if removed {
        write_json_atomic(path, &root)?;
    }
    Ok(removed)
}

fn read_or_empty_object(path: &Path) -> Result<Value, String> {
    if !path.exists() {
        return Ok(Value::Object(Map::new()));
    }
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {e}", path.display()))?;
    if content.trim().is_empty() {
        return Ok(Value::Object(Map::new()));
    }
    serde_json::from_str(&content).map_err(|e| format!("Invalid JSON in {}: {e}", path.display()))
}

fn write_json_atomic(path: &Path, value: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create {}: {e}", parent.display()))?;
    }

    let mut serialized = serde_json::to_string_pretty(value)
        .map_err(|e| format!("Failed to serialize settings JSON: {e}"))?;
    serialized.push('\n');

    let tmp_name = format!(
        ".{}.better-rpc-{}.tmp",
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("settings.json"),
        std::process::id()
    );
    let tmp_path = path.with_file_name(tmp_name);

    std::fs::write(&tmp_path, serialized)
        .map_err(|e| format!("Failed to write temporary settings file: {e}"))?;

    if let Err(rename_err) = std::fs::rename(&tmp_path, path) {
        // On Windows, `rename` fails if the target already exists.
        if path.exists() {
            std::fs::remove_file(path)
                .map_err(|e| format!("Failed to replace {}: {e}", path.display()))?;
            std::fs::rename(&tmp_path, path)
                .map_err(|e| format!("Failed to move settings into place: {e}"))?;
        } else {
            let _ = std::fs::remove_file(&tmp_path);
            return Err(format!("Failed to save {}: {rename_err}", path.display()));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_creates_hooks_and_is_idempotent() {
        let mut root = json!({
            "model": "claude-opus-4-6",
            "permissions": { "allow": ["Read"] }
        });

        install_hooks_in_json(&mut root, 47892).unwrap();
        assert_eq!(
            inspect_hooks_in_json(&root, 47892),
            ClaudeHookInstallState::Installed
        );
        assert_eq!(root["model"], "claude-opus-4-6");
        assert_eq!(root["permissions"]["allow"][0], "Read");
        assert!(root["hooks"].get("SessionStart").is_none());
        assert!(root["hooks"].get("PermissionRequest").is_none());

        // Second install produces no duplicates.
        install_hooks_in_json(&mut root, 47892).unwrap();
        for &event in CLAUDE_CODE_HOOK_EVENTS {
            assert_eq!(root["hooks"][event].as_array().unwrap().len(), 1);
        }
        assert_eq!(
            inspect_hooks_in_json(&root, 47892),
            ClaudeHookInstallState::Installed
        );
    }

    #[test]
    fn install_and_remove_preserve_existing_user_hooks() {
        let mut root = json!({
            "env": { "FOO": "bar" },
            "hooks": {
                "PreToolUse": [
                    {
                        "matcher": "Bash",
                        "hooks": [
                            { "type": "command", "command": "/usr/local/bin/my-guard.sh" }
                        ]
                    }
                ],
                "SessionStart": [
                    {
                        "hooks": [
                            { "type": "command", "command": "echo hello" }
                        ]
                    }
                ]
            }
        });

        install_hooks_in_json(&mut root, 47892).unwrap();
        assert_eq!(root["hooks"]["PreToolUse"].as_array().unwrap().len(), 2);
        assert_eq!(root["hooks"]["SessionStart"].as_array().unwrap().len(), 1);

        let removed = remove_hooks_from_json(&mut root).unwrap();
        assert!(removed);
        assert_eq!(
            inspect_hooks_in_json(&root, 47892),
            ClaudeHookInstallState::NotInstalled
        );

        // User's hooks and settings are untouched; Better RPC-only events were cleaned up.
        assert_eq!(root["env"]["FOO"], "bar");
        assert_eq!(root["hooks"]["PreToolUse"].as_array().unwrap().len(), 1);
        assert_eq!(
            root["hooks"]["PreToolUse"][0]["hooks"][0]["command"],
            "/usr/local/bin/my-guard.sh"
        );
        assert_eq!(root["hooks"]["SessionStart"].as_array().unwrap().len(), 1);
        assert!(root["hooks"].get("Stop").is_none());
    }

    #[test]
    fn remove_prunes_hooks_key_when_only_better_rpc_hooks_existed() {
        let mut root = json!({ "theme": "dark" });
        install_hooks_in_json(&mut root, 47892).unwrap();
        assert!(remove_hooks_from_json(&mut root).unwrap());
        assert!(root.get("hooks").is_none());
        assert_eq!(root["theme"], "dark");
    }

    #[test]
    fn inspect_detects_outdated_port_and_reinstall_repairs_it() {
        let mut root = json!({});
        install_hooks_in_json(&mut root, 47892).unwrap();
        assert_eq!(
            inspect_hooks_in_json(&root, 47900),
            ClaudeHookInstallState::Outdated
        );

        install_hooks_in_json(&mut root, 47900).unwrap();
        assert_eq!(
            inspect_hooks_in_json(&root, 47900),
            ClaudeHookInstallState::Installed
        );
        for &event in CLAUDE_CODE_HOOK_EVENTS {
            assert_eq!(root["hooks"][event].as_array().unwrap().len(), 1);
        }
    }

    #[test]
    fn file_roundtrip_and_malformed_json_protection() {
        let dir =
            std::env::temp_dir().join(format!("better-rpc-claude-cfg-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let settings_path = dir.join("settings.json");

        assert_eq!(
            inspect_settings_file(&settings_path, 47892).unwrap(),
            ClaudeHookInstallState::NotInstalled
        );

        install_hooks_at(&settings_path, 47892).unwrap();
        assert_eq!(
            inspect_settings_file(&settings_path, 47892).unwrap(),
            ClaudeHookInstallState::Installed
        );

        assert!(remove_hooks_at(&settings_path).unwrap());
        assert_eq!(
            inspect_settings_file(&settings_path, 47892).unwrap(),
            ClaudeHookInstallState::NotInstalled
        );

        // Malformed JSON must error and remain untouched.
        std::fs::write(&settings_path, "{ not valid json").unwrap();
        assert!(install_hooks_at(&settings_path, 47892).is_err());
        assert_eq!(
            std::fs::read_to_string(&settings_path).unwrap(),
            "{ not valid json"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }
}
