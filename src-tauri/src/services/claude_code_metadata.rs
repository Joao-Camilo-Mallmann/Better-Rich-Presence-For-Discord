//! Local metadata reader for Claude Code sessions.
//!
//! Reads a bounded slice of a session's JSONL transcript (`transcript_path`)
//! and falls back to `.git/HEAD` near `cwd` to resolve chat titles, model IDs,
//! and git branches locally without external network calls.

use crate::domain::claude_code::ClaudeSessionMetadata;
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

const HEAD_READ_BYTES: usize = 64 * 1024;
const TAIL_READ_BYTES: usize = 128 * 1024;
const MAX_FULL_READ_BYTES: u64 = (HEAD_READ_BYTES + TAIL_READ_BYTES) as u64;
const MAX_GIT_PARENT_DEPTH: usize = 5;

/// Parses a JSONL transcript slice into [`ClaudeSessionMetadata`].
/// Malformed or partial boundary lines are safely ignored.
pub fn parse_transcript_metadata(content: &str) -> ClaudeSessionMetadata {
    let mut custom_title: Option<String> = None;
    let mut summary: Option<String> = None;
    let mut agent_name: Option<String> = None;
    let mut last_prompt: Option<String> = None;
    let mut first_user_prompt: Option<String> = None;
    let mut git_branch: Option<String> = None;
    let mut model: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || !trimmed.starts_with('{') {
            continue;
        }
        let Ok(obj) = serde_json::from_str::<Value>(trimmed) else {
            continue;
        };

        if let Some(branch) = non_empty_str(obj.get("gitBranch")) {
            git_branch = Some(branch.to_string());
        }

        let entry_type = obj.get("type").and_then(Value::as_str).unwrap_or("");
        match entry_type {
            "custom-title" => {
                if let Some(t) = non_empty_str(obj.get("customTitle")) {
                    custom_title = Some(t.to_string());
                }
            }
            "summary" => {
                if let Some(s) = non_empty_str(obj.get("summary")) {
                    summary = Some(s.to_string());
                }
            }
            "agent-name" => {
                if let Some(a) = non_empty_str(obj.get("agentName")) {
                    agent_name = Some(a.to_string());
                }
            }
            "last-prompt" => {
                if let Some(p) = non_empty_str(obj.get("lastPrompt")) {
                    last_prompt = Some(p.to_string());
                }
            }
            "user" => {
                if first_user_prompt.is_none()
                    && obj.get("isMeta").and_then(Value::as_bool) != Some(true)
                {
                    if let Some(msg_content) = obj
                        .get("message")
                        .and_then(|m| non_empty_str(m.get("content")))
                    {
                        first_user_prompt = Some(msg_content.to_string());
                    }
                }
            }
            "assistant" => {
                if let Some(m) = obj
                    .get("message")
                    .and_then(|msg| non_empty_str(msg.get("model")))
                {
                    model = Some(m.to_string());
                }
            }
            _ => {}
        }
    }

    ClaudeSessionMetadata {
        chat_title: custom_title.or(summary).or(agent_name),
        fallback_prompt: last_prompt.or(first_user_prompt),
        model,
        git_branch,
    }
}

fn non_empty_str(val: Option<&Value>) -> Option<&str> {
    val.and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
}

/// Parses the contents of a `.git/HEAD` file into a branch name or short commit hash.
pub fn parse_git_head(content: &str) -> Option<String> {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(rest) = trimmed.strip_prefix("ref:") {
        let reference = rest.trim();
        let branch = reference
            .strip_prefix("refs/heads/")
            .unwrap_or(reference)
            .trim();
        if branch.is_empty() {
            None
        } else {
            Some(branch.to_string())
        }
    } else if trimmed.len() >= 7 && trimmed.bytes().all(|b| b.is_ascii_hexdigit()) {
        Some(trimmed[..7].to_string())
    } else {
        None
    }
}

/// Reads session metadata from a local transcript file and `.git/HEAD` near `cwd`.
pub fn read_session_metadata(
    transcript_path: Option<&str>,
    cwd: Option<&str>,
) -> ClaudeSessionMetadata {
    let mut meta = transcript_path
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .and_then(|p| read_bounded_transcript(Path::new(p)))
        .map(|text| parse_transcript_metadata(&text))
        .unwrap_or_default();

    if meta.git_branch.is_none() {
        if let Some(dir) = cwd.map(str::trim).filter(|s| !s.is_empty()) {
            meta.git_branch = read_git_branch_from_cwd(Path::new(dir));
        }
    }

    meta
}

fn read_bounded_transcript(path: &Path) -> Option<String> {
    let mut file = File::open(path).ok()?;
    let len = file.metadata().ok()?.len();

    if len <= MAX_FULL_READ_BYTES {
        let mut buf = Vec::with_capacity(len as usize);
        file.read_to_end(&mut buf).ok()?;
        return Some(String::from_utf8_lossy(&buf).into_owned());
    }

    let mut head = vec![0u8; HEAD_READ_BYTES];
    let head_read = file.read(&mut head).ok()?;
    head.truncate(head_read);

    file.seek(SeekFrom::End(-(TAIL_READ_BYTES as i64))).ok()?;
    let mut tail = Vec::with_capacity(TAIL_READ_BYTES);
    file.read_to_end(&mut tail).ok()?;

    let mut combined = String::with_capacity(head.len() + tail.len() + 1);
    combined.push_str(&String::from_utf8_lossy(&head));
    combined.push('\n');
    combined.push_str(&String::from_utf8_lossy(&tail));
    Some(combined)
}

fn read_git_branch_from_cwd(start: &Path) -> Option<String> {
    let mut current: Option<&Path> = Some(start);
    for _ in 0..=MAX_GIT_PARENT_DEPTH {
        let dir = current?;
        let dot_git = dir.join(".git");
        if dot_git.is_dir() {
            if let Ok(head) = std::fs::read_to_string(dot_git.join("HEAD")) {
                if let Some(branch) = parse_git_head(&head) {
                    return Some(branch);
                }
            }
            return None;
        } else if dot_git.is_file() {
            if let Ok(pointer) = std::fs::read_to_string(&dot_git) {
                if let Some(raw_gitdir) = pointer
                    .lines()
                    .find_map(|line| line.trim().strip_prefix("gitdir:"))
                    .map(str::trim)
                {
                    let gitdir_path = Path::new(raw_gitdir);
                    let resolved: PathBuf = if gitdir_path.is_absolute() {
                        gitdir_path.to_path_buf()
                    } else {
                        dir.join(gitdir_path)
                    };
                    if let Ok(head) = std::fs::read_to_string(resolved.join("HEAD")) {
                        if let Some(branch) = parse_git_head(&head) {
                            return Some(branch);
                        }
                    }
                }
            }
            return None;
        }
        current = dir.parent();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parses_transcript_metadata_and_ignores_partial_lines() {
        let sample = concat!(
            "{\"truncated_partial_line\n",
            "{\"type\":\"user\",\"isMeta\":true,\"message\":{\"content\":\"system meta\"}}\n",
            "{\"type\":\"user\",\"gitBranch\":\"feat/old\",\"message\":{\"content\":\"Initial user question\"}}\n",
            "{\"type\":\"assistant\",\"gitBranch\":\"feat/new\",\"message\":{\"model\":\"claude-opus-4-6\"}}\n",
            "{\"type\":\"summary\",\"summary\":\"Summary title\"}\n",
            "{\"type\":\"custom-title\",\"customTitle\":\"Custom chat name\"}\n",
            "{\"type\":\"last-prompt\",\"lastPrompt\":\"Latest prompt\"}\n",
        );

        let meta = parse_transcript_metadata(sample);
        assert_eq!(meta.chat_title.as_deref(), Some("Custom chat name"));
        assert_eq!(meta.fallback_prompt.as_deref(), Some("Latest prompt"));
        assert_eq!(meta.model.as_deref(), Some("claude-opus-4-6"));
        assert_eq!(meta.git_branch.as_deref(), Some("feat/new"));
    }

    #[test]
    fn parses_git_head_branches_and_detached_commits() {
        assert_eq!(
            parse_git_head("ref: refs/heads/main\n"),
            Some("main".to_string())
        );
        assert_eq!(
            parse_git_head("ref: refs/heads/feat/claude-rich\n"),
            Some("feat/claude-rich".to_string())
        );
        assert_eq!(
            parse_git_head("a1b2c3d4e5f60718293a4b5c6d7e8f9012345678\n"),
            Some("a1b2c3d".to_string())
        );
        assert_eq!(parse_git_head("   "), None);
    }

    #[test]
    fn reads_bounded_large_transcript_and_git_worktree_fallback() {
        let base = std::env::temp_dir().join(format!(
            "better-rpc-claude-meta-test-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&base);
        let work_dir = base.join("repo").join("sub");
        let git_dir = base.join("actual-gitdir");
        std::fs::create_dir_all(&work_dir).unwrap();
        std::fs::create_dir_all(&git_dir).unwrap();

        std::fs::write(git_dir.join("HEAD"), "ref: refs/heads/worktree-branch\n").unwrap();
        std::fs::write(
            base.join("repo").join(".git"),
            format!("gitdir: {}\n", git_dir.display()),
        )
        .unwrap();

        // Create a > 256 KiB transcript with an early user prompt and a late custom-title + model.
        let transcript = base.join("session.jsonl");
        let mut f = File::create(&transcript).unwrap();
        writeln!(
            f,
            r#"{{"type":"user","message":{{"content":"First prompt in large file"}}}}"#
        )
        .unwrap();
        let filler = format!(
            "{{\"type\":\"progress\",\"pad\":\"{}\"}}\n",
            "x".repeat(1024)
        );
        for _ in 0..260 {
            f.write_all(filler.as_bytes()).unwrap();
        }
        writeln!(
            f,
            r#"{{"type":"assistant","message":{{"model":"claude-sonnet-4-6"}}}}"#
        )
        .unwrap();
        writeln!(
            f,
            r#"{{"type":"custom-title","customTitle":"Large File Chat"}}"#
        )
        .unwrap();

        let meta = read_session_metadata(
            Some(transcript.to_str().unwrap()),
            Some(work_dir.to_str().unwrap()),
        );
        assert_eq!(meta.chat_title.as_deref(), Some("Large File Chat"));
        assert_eq!(meta.model.as_deref(), Some("claude-sonnet-4-6"));
        assert_eq!(meta.git_branch.as_deref(), Some("worktree-branch"));

        let _ = std::fs::remove_dir_all(&base);
    }
}
