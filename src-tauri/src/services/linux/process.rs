//! Linux process identity (`/proc`) and the rule-driven process scan used by
//! the process-scan fallback and Max Priority Mode.

use crate::models::types::AppRule;
use crate::services::engine::normalize_process_name;
use std::collections::HashMap;
use std::path::Path;

/// Lowercase file name of an executable path, without the ` (deleted)` suffix
/// the kernel appends when the binary was replaced on disk (e.g. after an update).
pub fn exe_file_name(path: &str) -> Option<String> {
    let path = path.trim().trim_end_matches(" (deleted)");
    let name = Path::new(path).file_name()?.to_string_lossy().to_lowercase();
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

/// File name of `/proc/<pid>/exe`. `None` for kernel threads and for
/// processes of other users.
pub fn exe_name(pid: u32) -> Option<String> {
    let target = std::fs::read_link(format!("/proc/{}/exe", pid)).ok()?;
    exe_file_name(&target.to_string_lossy())
}

/// Kernel process name (`/proc/<pid>/comm`, truncated to 15 bytes by the kernel).
pub fn comm_name(pid: u32) -> Option<String> {
    let comm = std::fs::read_to_string(format!("/proc/{}/comm", pid)).ok()?;
    let comm = comm.trim().to_lowercase();
    if comm.is_empty() {
        None
    } else {
        Some(comm)
    }
}

/// The name used to identify a process: executable file name, else `comm`.
pub fn process_name(pid: u32) -> Option<String> {
    exe_name(pid).or_else(|| comm_name(pid))
}

/// Alternative names for an app, in matching order, excluding `primary`
/// and duplicates: `primary` without a `.bin` wrapper suffix, the window
/// class, the last segment of a reverse-DNS class (`com.discordapp.Discord`
/// → `discord`), and the kernel `comm` name.
pub fn build_aliases(primary: &str, class: &str, comm: Option<&str>) -> Vec<String> {
    let mut candidates: Vec<String> = Vec::new();
    if let Some(stripped) = primary.strip_suffix(".bin") {
        candidates.push(stripped.to_string());
    }
    candidates.push(class.to_string());
    if class.matches('.').count() >= 2 {
        if let Some(last) = class.rsplit('.').next() {
            candidates.push(last.to_string());
        }
    }
    if let Some(comm) = comm {
        candidates.push(comm.to_string());
    }

    let primary = primary.trim().to_lowercase();
    let mut aliases: Vec<String> = Vec::new();
    for candidate in candidates {
        let candidate = candidate.trim().to_lowercase();
        if candidate.is_empty() || candidate == primary || aliases.contains(&candidate) {
            continue;
        }
        aliases.push(candidate);
    }
    aliases
}

/// Whether a rule's process name targets a scanned process name
/// (`.exe`-insensitive; a `.bin` wrapper suffix on the process is ignored).
pub fn rule_targets(rule_name: &str, process_name: &str) -> bool {
    let rule = normalize_process_name(rule_name);
    let process = normalize_process_name(process_name);
    rule == process || process.strip_suffix(".bin") == Some(rule.as_str())
}

/// Picks the running process that matches the enabled rule with the lowest
/// index (highest priority), ignoring `self_pid`.
pub fn pick_priority_rule(
    processes: &[(u32, String)],
    rules: &[AppRule],
    self_pid: u32,
) -> Option<(u32, String)> {
    let mut best: Option<(usize, u32, &str)> = None;
    for (pid, name) in processes {
        if *pid == self_pid {
            continue;
        }
        let Some(idx) = rules
            .iter()
            .position(|r| r.enabled && rule_targets(&r.process_name, name))
        else {
            continue;
        };
        if best.map_or(true, |(best_idx, _, _)| idx < best_idx) {
            best = Some((idx, *pid, name.as_str()));
        }
    }
    best.map(|(_, pid, name)| (pid, name.to_string()))
}

/// Lists running processes from `/proc`, caching each PID's name.
#[derive(Default)]
pub struct ProcessScanner {
    names: HashMap<u32, String>,
}

impl ProcessScanner {
    /// Returns `(pid, name)` for every visible process. Names of PIDs seen on
    /// the previous scan are reused; PIDs that are gone are dropped.
    pub fn scan(&mut self) -> Vec<(u32, String)> {
        let Ok(entries) = std::fs::read_dir("/proc") else {
            return Vec::new();
        };

        let mut current = HashMap::with_capacity(self.names.len());
        for entry in entries.flatten() {
            let Some(pid) = entry.file_name().to_str().and_then(|s| s.parse::<u32>().ok()) else {
                continue;
            };
            let name = match self.names.remove(&pid) {
                Some(name) => name,
                None => match process_name(pid) {
                    Some(name) => name,
                    None => continue,
                },
            };
            current.insert(pid, name);
        }
        self.names = current;

        self.names.iter().map(|(pid, name)| (*pid, name.clone())).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(process_name: &str, enabled: bool) -> AppRule {
        let mut r = AppRule::new(process_name, process_name, "", "", "auto");
        r.enabled = enabled;
        r
    }

    #[test]
    fn exe_file_name_handles_paths_and_deleted_suffix() {
        assert_eq!(exe_file_name("/usr/share/code/code").as_deref(), Some("code"));
        assert_eq!(exe_file_name("/usr/share/code/code (deleted)").as_deref(), Some("code"));
        assert_eq!(exe_file_name("/snap/notion-desktop/3/notion-desktop").as_deref(), Some("notion-desktop"));
        assert_eq!(exe_file_name("/app/discord/Discord").as_deref(), Some("discord"));
        assert_eq!(exe_file_name("/"), None);
    }

    #[test]
    fn aliases_are_deduplicated_and_exclude_primary() {
        assert_eq!(build_aliases("code", "Code", Some("code")), Vec::<String>::new());
        assert_eq!(
            build_aliases("electron", "code", Some("electron")),
            vec!["code".to_string()]
        );
        assert_eq!(
            build_aliases("rocketchat-desktop.bin", "rocket.chat", Some("rocketchat-desk")),
            vec![
                "rocketchat-desktop".to_string(),
                "rocket.chat".to_string(),
                "rocketchat-desk".to_string()
            ]
        );
        assert_eq!(
            build_aliases("discord", "com.discordapp.Discord", None),
            vec!["com.discordapp.discord".to_string()]
        );
        assert_eq!(
            build_aliases("bwrap", "org.mozilla.firefox", None),
            vec!["org.mozilla.firefox".to_string(), "firefox".to_string()]
        );
        assert_eq!(build_aliases("code", "", None), Vec::<String>::new());
    }

    #[test]
    fn priority_pick_prefers_lowest_rule_index() {
        let processes = vec![
            (10, "firefox".to_string()),
            (20, "code".to_string()),
            (30, "spotify".to_string()),
        ];
        let rules = vec![rule("code.exe", true), rule("firefox", true)];
        assert_eq!(pick_priority_rule(&processes, &rules, 1), Some((20, "code".to_string())));
    }

    #[test]
    fn priority_pick_skips_disabled_rules_and_self() {
        let processes = vec![(10, "firefox".to_string()), (20, "code".to_string())];
        let rules = vec![rule("code", false), rule("firefox", true)];
        assert_eq!(pick_priority_rule(&processes, &rules, 1), Some((10, "firefox".to_string())));
        assert_eq!(pick_priority_rule(&processes, &rules, 10), None);
        assert_eq!(pick_priority_rule(&processes, &[], 1), None);
    }

    #[test]
    fn priority_pick_ignores_bin_wrapper_suffix() {
        let processes = vec![(10, "rocketchat-desktop.bin".to_string())];
        let rules = vec![rule("rocketchat-desktop", true)];
        assert_eq!(
            pick_priority_rule(&processes, &rules, 1),
            Some((10, "rocketchat-desktop.bin".to_string()))
        );
    }

    #[test]
    fn scanner_sees_the_current_process() {
        let mut scanner = ProcessScanner::default();
        let me = std::process::id();
        let first = scanner.scan();
        assert!(first.iter().any(|(pid, name)| *pid == me && !name.is_empty()));
        // Second scan reuses the cache and still lists the process.
        let second = scanner.scan();
        assert!(second.iter().any(|(pid, _)| *pid == me));
    }
}
