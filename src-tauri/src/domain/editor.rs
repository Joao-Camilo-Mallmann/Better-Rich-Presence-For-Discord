//! Pure window-title parsing for Zed and other catalog editors/IDEs.
//! Depends on `std` and `crate::domain::language` only.

use crate::domain::language::{detect_language, language_icon_url, LanguageInfo};

const SEPARATORS: [&str; 3] = [" — ", " – ", " - "];

const STATUS_TAGS: [&str; 6] = [
    "running",
    "debugging",
    "administrator",
    "read-only",
    "modified",
    "workspace",
];

const NON_WORKSPACE_PREFIXES: [&str; 5] = [
    "python ",
    "wsl:",
    "ssh:",
    "dev container:",
    "codespaces:",
];

const WEB_TLDS: [&str; 6] = ["com", "org", "net", "io", "dev", "app"];

/// Parsed editor context extracted from an editor/IDE window title.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EditorTitleContext {
    pub file_name: Option<String>,
    pub workspace_name: Option<String>,
    pub language: Option<LanguageInfo>,
}

impl EditorTitleContext {
    /// Returns `(small_image_url, small_text)` for the detected language, or
    /// empty strings when no language was matched.
    pub fn language_badge(&self) -> (String, String) {
        match self.language {
            Some(lang) => (language_icon_url(lang.icon), lang.name.to_string()),
            None => (String::new(), String::new()),
        }
    }
}

#[derive(Debug, Default)]
struct CleanedSegment {
    text: String,
    basename: String,
    had_dirty: bool,
    is_path: bool,
    inline_workspace: Option<String>,
}

fn non_empty(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

fn editor_aliases(app_id: &str, app_name: &str) -> Vec<&'static str> {
    let mut aliases: Vec<&'static str> = match app_id {
        "vscode" | "vscode_insiders" => {
            vec!["Visual Studio Code - Insiders", "Visual Studio Code", "VS Code"]
        }
        "cursor" => vec!["Cursor"],
        "vscodium" => vec!["VSCodium"],
        "windsurf" => vec!["Windsurf"],
        "antigravity" => vec!["Antigravity"],
        "zed" => vec!["Zed"],
        "visualstudio" => vec!["Microsoft Visual Studio", "Visual Studio"],
        "sublimetext" => vec!["Sublime Text"],
        "notepadplusplus" => vec!["Notepad++"],
        "neovim" => vec!["Neovim", "NVIM"],
        "vim" => vec!["GVIM", "Vim", "VIM"],
        "emacs" => vec!["GNU Emacs", "Emacs"],
        "eclipse" => vec!["Eclipse IDE", "Eclipse"],
        "netbeans" => vec!["Apache NetBeans IDE", "Apache NetBeans", "NetBeans"],
        "codeblocks" => vec!["Code::Blocks"],
        "rstudio" => vec!["RStudio"],
        "spyder" => vec!["Spyder"],
        "atom" => vec!["Atom"],
        "jupyter" => vec!["Jupyter Notebook", "JupyterLab", "Jupyter"],
        "xcode" => vec!["Xcode"],
        "intellij" => vec!["IntelliJ IDEA"],
        _ => Vec::new(),
    };
    let _ = app_name;
    // Keep longest aliases first so e.g. `Visual Studio Code - Insiders` strips before `Visual Studio Code`.
    aliases.sort_by_key(|a| std::cmp::Reverse(a.len()));
    aliases
}

fn strip_suffix_ignore_ascii_case<'a>(input: &'a str, suffix: &str) -> Option<&'a str> {
    if input.len() < suffix.len() {
        return None;
    }
    let split_idx = input.len() - suffix.len();
    if !input.is_char_boundary(split_idx) {
        return None;
    }
    input[split_idx..]
        .eq_ignore_ascii_case(suffix)
        .then_some(&input[..split_idx])
}

fn strip_prefix_ignore_ascii_case<'a>(input: &'a str, prefix: &str) -> Option<&'a str> {
    if input.len() < prefix.len() || !input.is_char_boundary(prefix.len()) {
        return None;
    }
    input[..prefix.len()]
        .eq_ignore_ascii_case(prefix)
        .then_some(&input[prefix.len()..])
}

fn strip_editor_suffix<'a>(title: &'a str, app_id: &str, app_name: &str) -> &'a str {
    let mut candidates: Vec<&str> = editor_aliases(app_id, app_name);
    let trimmed_app_name = app_name.trim();
    if !trimmed_app_name.is_empty()
        && !candidates
            .iter()
            .any(|c| c.eq_ignore_ascii_case(trimmed_app_name))
    {
        candidates.push(trimmed_app_name);
        candidates.sort_by_key(|a| std::cmp::Reverse(a.len()));
    }

    if app_id == "emacs" {
        for sep in SEPARATORS {
            let marker = format!("{sep}gnu emacs at ");
            if let Some(idx) = find_ascii_case_insensitive(title, &marker) {
                return title[..idx].trim_end();
            }
        }
    }

    for alias in candidates {
        for sep in SEPARATORS {
            if let Some(before_alias) = strip_suffix_ignore_ascii_case(title, alias) {
                if let Some(prefix) = before_alias.strip_suffix(sep) {
                    return prefix.trim_end();
                }
            }
        }
    }

    title
}

fn find_ascii_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    let h_lower = haystack.to_ascii_lowercase();
    let n_lower = needle.to_ascii_lowercase();
    h_lower.find(&n_lower)
}

fn strip_leading_app_prefix<'a>(title: &'a str, app_name: &str) -> &'a str {
    let trimmed_name = app_name.trim();
    if trimmed_name.is_empty() {
        return title;
    }
    let Some(mut rest) = strip_prefix_ignore_ascii_case(title, trimmed_name) else {
        return title;
    };

    let after_name = rest.trim_start();
    if let Some(paren_inner) = after_name.strip_prefix('(') {
        if let Some(close_idx) = paren_inner.find(')') {
            rest = &paren_inner[close_idx + 1..];
        }
    }

    for sep in SEPARATORS {
        if let Some(after_sep) = rest.strip_prefix(sep) {
            return after_sep.trim_start();
        }
    }
    title
}

fn is_bare_app_name(title: &str, app_id: &str, app_name: &str) -> bool {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return true;
    }
    let trimmed_name = app_name.trim();
    if !trimmed_name.is_empty() {
        if trimmed.eq_ignore_ascii_case(trimmed_name) {
            return true;
        }
        if let Some(rest) = strip_prefix_ignore_ascii_case(trimmed, trimmed_name) {
            let rest = rest.trim();
            if rest.starts_with('(') && rest.ends_with(')') {
                return true;
            }
        }
    }
    if app_id == "emacs" && trimmed.to_ascii_lowercase().starts_with("gnu emacs") {
        return true;
    }
    editor_aliases(app_id, app_name)
        .into_iter()
        .any(|alias| trimmed.eq_ignore_ascii_case(alias))
}

fn strip_vscode_remote_suffixes(raw: &str) -> &str {
    const REMOTE_PREFIXES: [&str; 4] = ["[WSL:", "[SSH:", "[Dev Container:", "[Codespaces:"];
    let mut s = raw.trim();
    loop {
        if let Some(stripped) = s.strip_suffix("(Workspace)") {
            s = stripped.trim_end();
            continue;
        }
        if s.ends_with(']') {
            if let Some(open_idx) = s.rfind('[') {
                let bracketed = &s[open_idx..];
                if REMOTE_PREFIXES
                    .iter()
                    .any(|prefix| bracketed.starts_with(prefix))
                {
                    s = s[..open_idx].trim_end();
                    continue;
                }
            }
        }
        break;
    }
    s
}

fn strip_jetbrains_path_bracket(raw: &str) -> &str {
    let s = raw.trim();
    if let Some(without_close) = s.strip_suffix(']') {
        if let Some(open_idx) = without_close.rfind('[') {
            let inner = without_close[open_idx + 1..].trim();
            if inner.contains('/') || inner.contains('\\') || inner.contains(":\\") || inner.contains('~') {
                return without_close[..open_idx].trim_end();
            }
        }
    }
    s
}

fn strip_dirty_markers(raw: &str) -> (&str, bool) {
    let mut s = raw.trim();
    let mut dirty = false;

    loop {
        if let Some(rest) = s
            .strip_prefix('●')
            .or_else(|| s.strip_prefix('•'))
            .or_else(|| s.strip_prefix('*'))
        {
            s = rest.trim_start();
            dirty = true;
            continue;
        }
        break;
    }

    loop {
        if let Some(rest) = s.strip_suffix('•').or_else(|| s.strip_suffix('●')) {
            s = rest.trim_end();
            dirty = true;
            continue;
        }
        if let Some(rest) = s.strip_suffix(" +") {
            s = rest.trim_end();
            dirty = true;
            continue;
        }
        break;
    }

    (s, dirty)
}

fn extract_basename(text: &str) -> &str {
    text.rsplit(['/', '\\'])
        .next()
        .unwrap_or(text)
        .trim()
}

fn has_plausible_file_ext(name: &str) -> bool {
    let trimmed = name.trim();
    let Some((stem, ext)) = trimmed.rsplit_once('.') else {
        return false;
    };
    if stem.is_empty() || stem.starts_with('.') {
        return false;
    }
    if ext.is_empty() || ext.len() > 8 || !ext.bytes().all(|b| b.is_ascii_alphabetic()) {
        return false;
    }
    !WEB_TLDS
        .iter()
        .any(|tld| ext.eq_ignore_ascii_case(tld))
}

fn normalize_inline_workspace(raw: &str) -> Option<String> {
    let trimmed = raw.trim().trim_end_matches(['/', '\\']);
    let without_tilde = trimmed
        .strip_prefix("~/")
        .or_else(|| trimmed.strip_prefix("~\\"))
        .unwrap_or(trimmed);
    let base = without_tilde
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(without_tilde)
        .trim();
    (!base.is_empty()).then(|| base.to_string())
}

fn is_non_workspace_tag(inner: &str) -> bool {
    let lower = inner.trim().to_ascii_lowercase();
    lower.is_empty()
        || STATUS_TAGS.iter().any(|tag| lower == *tag)
        || NON_WORKSPACE_PREFIXES
            .iter()
            .any(|prefix| lower.starts_with(prefix))
}

fn extract_inline_workspace(raw: &str) -> (&str, Option<String>) {
    let s = raw.trim();
    let pair = if let Some(stripped) = s.strip_suffix(')') {
        stripped.rfind('(').map(|idx| (idx, &stripped[idx + 1..]))
    } else if let Some(stripped) = s.strip_suffix(']') {
        stripped.rfind('[').map(|idx| (idx, &stripped[idx + 1..]))
    } else {
        None
    };

    let Some((open_idx, inner)) = pair else {
        return (s, None);
    };

    let prefix = s[..open_idx].trim_end();
    if prefix.is_empty() || is_non_workspace_tag(inner) {
        return (s, None);
    }

    let (clean_prefix, had_dirty) = strip_dirty_markers(prefix);
    let is_path = clean_prefix.contains('/') || clean_prefix.contains('\\');
    let base = extract_basename(clean_prefix);
    let prefix_looks_file = had_dirty
        || is_path
        || detect_language(base).is_some()
        || has_plausible_file_ext(base);

    if !prefix_looks_file {
        return (s, None);
    }

    match normalize_inline_workspace(inner) {
        Some(ws) => (prefix, Some(ws)),
        None => (s, None),
    }
}

fn clean_segment(raw: &str, allow_inline_workspace: bool) -> CleanedSegment {
    let mut s = strip_vscode_remote_suffixes(raw);
    s = strip_jetbrains_path_bracket(s);

    let inline_workspace = if allow_inline_workspace {
        let (next, ws) = extract_inline_workspace(s);
        s = next;
        ws
    } else {
        None
    };

    let (clean, had_dirty) = strip_dirty_markers(s);
    let is_path = clean.contains('/') || clean.contains('\\');
    let basename = extract_basename(clean).to_string();

    CleanedSegment {
        text: clean.to_string(),
        basename,
        had_dirty,
        is_path,
        inline_workspace,
    }
}

fn prefers_workspace_first(app_id: &str, sep: &str) -> bool {
    let is_dash = sep == " — " || sep == " – ";
    match app_id {
        "zed" => is_dash,
        "intellij" | "pycharm" | "webstorm" | "phpstorm" | "rubymine" | "clion" | "goland"
        | "rider" | "datagrip" | "androidstudio" => is_dash,
        _ => false,
    }
}

/// Parses an editor or IDE window title into file, workspace, and detected language.
pub fn parse_editor_title(app_id: &str, app_name: &str, window_title: &str) -> EditorTitleContext {
    let trimmed = window_title.trim();
    if trimmed.is_empty() {
        return EditorTitleContext::default();
    }

    let without_modified = strip_suffix_ignore_ascii_case(trimmed, " - Modified").unwrap_or(trimmed);
    let without_suffix = strip_editor_suffix(without_modified, app_id, app_name);
    let remaining = strip_leading_app_prefix(without_suffix, app_name);

    if is_bare_app_name(remaining, app_id, app_name) {
        return EditorTitleContext::default();
    }

    let split = SEPARATORS
        .iter()
        .find_map(|&sep| remaining.rsplit_once(sep).map(|(l, r)| (l, r, sep)));

    match split {
        Some((raw_left, raw_right, sep)) => {
            let left = clean_segment(raw_left, false);
            let right = clean_segment(raw_right, false);

            let left_lang = detect_language(&left.basename);
            let right_lang = detect_language(&right.basename);

            let left_file_like = left_lang.is_some()
                || left.had_dirty
                || left.is_path
                || has_plausible_file_ext(&left.basename);
            let right_file_like = right_lang.is_some()
                || right.had_dirty
                || right.is_path
                || has_plausible_file_ext(&right.basename);

            let use_right_as_file = if right_lang.is_some() && left_lang.is_none() {
                true
            } else if left_lang.is_some() && right_lang.is_none() {
                false
            } else if right_file_like && !left_file_like {
                true
            } else if left_file_like && !right_file_like {
                false
            } else {
                prefers_workspace_first(app_id, sep)
            };

            if use_right_as_file {
                EditorTitleContext {
                    file_name: non_empty(&right.basename),
                    workspace_name: non_empty(&left.text),
                    language: right_lang,
                }
            } else {
                EditorTitleContext {
                    file_name: non_empty(&left.basename),
                    workspace_name: non_empty(&right.text),
                    language: left_lang,
                }
            }
        }
        None => {
            let only = clean_segment(remaining, true);
            let lang = detect_language(&only.basename);
            let is_file = lang.is_some()
                || only.had_dirty
                || only.is_path
                || only.inline_workspace.is_some()
                || has_plausible_file_ext(&only.basename);

            if is_file {
                EditorTitleContext {
                    file_name: non_empty(&only.basename),
                    workspace_name: only.inline_workspace,
                    language: lang,
                }
            } else {
                EditorTitleContext {
                    file_name: None,
                    workspace_name: non_empty(&only.text),
                    language: None,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_zed_titles() {
        let ctx = parse_editor_title("zed", "Zed", "Better-Rich-Presence-For-Discord — engine.rs");
        assert_eq!(ctx.workspace_name.as_deref(), Some("Better-Rich-Presence-For-Discord"));
        assert_eq!(ctx.file_name.as_deref(), Some("engine.rs"));
        assert_eq!(ctx.language, Some(LanguageInfo { name: "Rust", icon: "rust" }));

        let ctx = parse_editor_title(
            "zed",
            "Zed",
            "Better-Rich-Presence-For-Discord — src/services/engine.rs",
        );
        assert_eq!(ctx.workspace_name.as_deref(), Some("Better-Rich-Presence-For-Discord"));
        assert_eq!(ctx.file_name.as_deref(), Some("engine.rs"));
        assert_eq!(ctx.language.map(|l| l.name), Some("Rust"));

        let ctx = parse_editor_title("zed", "Zed", "engine.rs — Better-Rich-Presence-For-Discord");
        assert_eq!(ctx.workspace_name.as_deref(), Some("Better-Rich-Presence-For-Discord"));
        assert_eq!(ctx.file_name.as_deref(), Some("engine.rs"));
        assert_eq!(ctx.language.map(|l| l.name), Some("Rust"));

        let ctx = parse_editor_title("zed", "Zed", "● main.rs - project - Zed");
        assert_eq!(ctx.workspace_name.as_deref(), Some("project"));
        assert_eq!(ctx.file_name.as_deref(), Some("main.rs"));
        assert_eq!(ctx.language.map(|l| l.name), Some("Rust"));

        let ctx = parse_editor_title("zed", "Zed", "Better-Rich-Presence-For-Discord");
        assert_eq!(ctx.workspace_name.as_deref(), Some("Better-Rich-Presence-For-Discord"));
        assert_eq!(ctx.file_name, None);
        assert_eq!(ctx.language, None);

        let ctx = parse_editor_title("zed", "Zed", "notes.md");
        assert_eq!(ctx.workspace_name, None);
        assert_eq!(ctx.file_name.as_deref(), Some("notes.md"));
        assert_eq!(ctx.language.map(|l| l.name), Some("Markdown"));

        assert_eq!(parse_editor_title("zed", "Zed", "Zed"), EditorTitleContext::default());
        assert_eq!(parse_editor_title("zed", "Zed", ""), EditorTitleContext::default());
    }

    #[test]
    fn parses_jetbrains_titles() {
        let ctx = parse_editor_title("intellij", "IntelliJ IDEA", "my-spring-app – UserController.java");
        assert_eq!(ctx.workspace_name.as_deref(), Some("my-spring-app"));
        assert_eq!(ctx.file_name.as_deref(), Some("UserController.java"));
        assert_eq!(ctx.language.map(|l| l.name), Some("Java"));

        let ctx = parse_editor_title(
            "pycharm",
            "PyCharm",
            "my-api – src/routes/auth.py [C:\\Projects\\my-api]",
        );
        assert_eq!(ctx.workspace_name.as_deref(), Some("my-api"));
        assert_eq!(ctx.file_name.as_deref(), Some("auth.py"));
        assert_eq!(ctx.language.map(|l| l.name), Some("Python"));

        let ctx = parse_editor_title(
            "intellij",
            "IntelliJ IDEA",
            "MyClass.java — my-spring-project [C:\\Projects\\my-spring-project]",
        );
        assert_eq!(ctx.workspace_name.as_deref(), Some("my-spring-project"));
        assert_eq!(ctx.file_name.as_deref(), Some("MyClass.java"));
        assert_eq!(ctx.language.map(|l| l.name), Some("Java"));

        let ctx = parse_editor_title(
            "androidstudio",
            "Android Studio",
            "my-app [C:\\Projects\\my-app] – main.kt - Android Studio",
        );
        assert_eq!(ctx.workspace_name.as_deref(), Some("my-app"));
        assert_eq!(ctx.file_name.as_deref(), Some("main.kt"));
        assert_eq!(ctx.language.map(|l| l.name), Some("Kotlin"));

        let ctx = parse_editor_title("webstorm", "WebStorm", "my-api [~/work/my-api]");
        assert_eq!(ctx.workspace_name.as_deref(), Some("my-api"));
        assert_eq!(ctx.file_name, None);
        assert_eq!(ctx.language, None);
    }

    #[test]
    fn parses_sublime_and_notepadplusplus_titles() {
        let ctx = parse_editor_title(
            "sublimetext",
            "Sublime Text",
            "C:\\Projects\\web\\index.ts (web) - Sublime Text",
        );
        assert_eq!(ctx.workspace_name.as_deref(), Some("web"));
        assert_eq!(ctx.file_name.as_deref(), Some("index.ts"));
        assert_eq!(ctx.language.map(|l| l.name), Some("TypeScript"));

        let ctx = parse_editor_title("sublimetext", "Sublime Text", "main.py • - Sublime Text");
        assert_eq!(ctx.file_name.as_deref(), Some("main.py"));
        assert_eq!(ctx.workspace_name, None);
        assert_eq!(ctx.language.map(|l| l.name), Some("Python"));

        let ctx = parse_editor_title("sublimetext", "Sublime Text", "• main.py - Sublime Text");
        assert_eq!(ctx.file_name.as_deref(), Some("main.py"));
        assert_eq!(ctx.language.map(|l| l.name), Some("Python"));

        let ctx = parse_editor_title(
            "notepadplusplus",
            "Notepad++",
            "*C:\\Users\\alice\\config.yaml - Notepad++",
        );
        assert_eq!(ctx.file_name.as_deref(), Some("config.yaml"));
        assert_eq!(ctx.workspace_name, None);
        assert_eq!(ctx.language.map(|l| l.name), Some("YAML"));

        let ctx = parse_editor_title("notepadplusplus", "Notepad++", "script.ps1 - Notepad++");
        assert_eq!(ctx.file_name.as_deref(), Some("script.ps1"));
        assert_eq!(ctx.language.map(|l| l.name), Some("PowerShell"));
    }

    #[test]
    fn parses_neovim_vim_and_visual_studio_titles() {
        let ctx = parse_editor_title("neovim", "Neovim", "engine.rs (~/Better-RPC) - NVIM");
        assert_eq!(ctx.file_name.as_deref(), Some("engine.rs"));
        assert_eq!(ctx.workspace_name.as_deref(), Some("Better-RPC"));
        assert_eq!(ctx.language.map(|l| l.name), Some("Rust"));

        let ctx = parse_editor_title("vim", "Vim", "main.c + (~/src) - GVIM");
        assert_eq!(ctx.file_name.as_deref(), Some("main.c"));
        assert_eq!(ctx.workspace_name.as_deref(), Some("src"));
        assert_eq!(ctx.language.map(|l| l.name), Some("C"));

        let ctx = parse_editor_title(
            "visualstudio",
            "Visual Studio",
            "Program.cs - MySolution - Microsoft Visual Studio",
        );
        assert_eq!(ctx.file_name.as_deref(), Some("Program.cs"));
        assert_eq!(ctx.workspace_name.as_deref(), Some("MySolution"));
        assert_eq!(ctx.language.map(|l| l.name), Some("C#"));

        let ctx = parse_editor_title(
            "visualstudio",
            "Visual Studio",
            "MySolution - Microsoft Visual Studio",
        );
        assert_eq!(ctx.file_name, None);
        assert_eq!(ctx.workspace_name.as_deref(), Some("MySolution"));
        assert_eq!(ctx.language, None);
    }

    #[test]
    fn parses_remaining_catalog_editors_and_custom_vscode_titles() {
        let ctx = parse_editor_title("eclipse", "Eclipse", "my-java - Main.java - Eclipse IDE");
        assert_eq!(ctx.workspace_name.as_deref(), Some("my-java"));
        assert_eq!(ctx.file_name.as_deref(), Some("Main.java"));
        assert_eq!(ctx.language.map(|l| l.name), Some("Java"));

        let ctx = parse_editor_title("codeblocks", "Code::Blocks", "main.cpp [my-project] - Code::Blocks");
        assert_eq!(ctx.file_name.as_deref(), Some("main.cpp"));
        assert_eq!(ctx.workspace_name.as_deref(), Some("my-project"));
        assert_eq!(ctx.language.map(|l| l.name), Some("C++"));

        let ctx = parse_editor_title("rstudio", "RStudio", "analysis.r - RStudio");
        assert_eq!(ctx.file_name.as_deref(), Some("analysis.r"));
        assert_eq!(ctx.language.map(|l| l.name), Some("R"));

        let ctx = parse_editor_title("spyder", "Spyder", "Spyder (Python 3.11) - script.py");
        assert_eq!(ctx.file_name.as_deref(), Some("script.py"));
        assert_eq!(ctx.language.map(|l| l.name), Some("Python"));

        let ctx = parse_editor_title("atom", "Atom", "index.js — web — Atom");
        assert_eq!(ctx.file_name.as_deref(), Some("index.js"));
        assert_eq!(ctx.workspace_name.as_deref(), Some("web"));
        assert_eq!(ctx.language.map(|l| l.name), Some("JavaScript"));

        let ctx = parse_editor_title("emacs", "Emacs", "init.el - GNU Emacs at devbox");
        assert_eq!(ctx.file_name.as_deref(), Some("init.el"));

        let ctx = parse_editor_title("jupyter", "Jupyter Notebook", "model.ipynb - Jupyter Notebook");
        assert_eq!(ctx.file_name.as_deref(), Some("model.ipynb"));
        assert_eq!(ctx.language.map(|l| l.name), Some("Jupyter"));

        let ctx = parse_editor_title("xcode", "Xcode", "AppDelegate.swift — MyApp — Xcode");
        assert_eq!(ctx.file_name.as_deref(), Some("AppDelegate.swift"));
        assert_eq!(ctx.workspace_name.as_deref(), Some("MyApp"));
        assert_eq!(ctx.language.map(|l| l.name), Some("Swift"));

        let ctx = parse_editor_title("vscode", "Visual Studio Code", "Better-RPC — App.tsx");
        assert_eq!(ctx.workspace_name.as_deref(), Some("Better-RPC"));
        assert_eq!(ctx.file_name.as_deref(), Some("App.tsx"));
        assert_eq!(ctx.language, Some(LanguageInfo { name: "TypeScript", icon: "react" }));
    }
}
