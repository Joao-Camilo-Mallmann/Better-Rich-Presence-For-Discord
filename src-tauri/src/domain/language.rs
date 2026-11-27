//! Pure programming-language detection from file names and icon URL mapping.
//! Depends on `std` only.

use std::path::Path;

const LANGUAGE_ICON_BASE: &str =
    "https://raw.githubusercontent.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/refs/heads/main/public/assets/icons/";

/// Display name and `public/assets/icons/<icon>.png` slug for a detected language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageInfo {
    pub name: &'static str,
    pub icon: &'static str,
}

impl LanguageInfo {
    const fn new(name: &'static str, icon: &'static str) -> Self {
        Self { name, icon }
    }
}

/// Builds the raw GitHub icon URL for a language icon slug in `public/assets/icons/`.
pub fn language_icon_url(icon: &str) -> String {
    format!("{LANGUAGE_ICON_BASE}{}.png", icon.trim())
}

/// Detects the programming language for a file name (or path) using special file
/// names first, then case-insensitive extension matching.
pub fn detect_language(file_name: &str) -> Option<LanguageInfo> {
    let trimmed = file_name.trim();
    if trimmed.is_empty() {
        return None;
    }

    let base = trimmed.rsplit(['/', '\\']).next().unwrap_or(trimmed).trim();
    if base.is_empty() {
        return None;
    }

    let lower = base.to_ascii_lowercase();

    if let Some(special) = match_special_filename(&lower) {
        return Some(special);
    }

    let ext = Path::new(&lower).extension()?.to_str()?;
    match_extension(ext)
}

fn match_special_filename(lower: &str) -> Option<LanguageInfo> {
    if lower == "dockerfile"
        || lower.starts_with("dockerfile.")
        || lower == "containerfile"
        || lower == ".dockerignore"
    {
        return Some(LanguageInfo::new("Docker", "docker"));
    }
    if lower == ".env" || lower.starts_with(".env.") {
        return Some(LanguageInfo::new("Shell", "shell"));
    }

    match lower {
        "makefile" | "gnumakefile" => Some(LanguageInfo::new("Makefile", "make")),
        "cmakelists.txt" => Some(LanguageInfo::new("CMake", "cmake")),
        "cargo.toml" | "cargo.lock" => Some(LanguageInfo::new("Rust", "rust")),
        "package.json" => Some(LanguageInfo::new("Node.js", "nodedotjs")),
        ".gitignore" | ".gitattributes" | ".gitmodules" => Some(LanguageInfo::new("Git", "git")),
        ".bashrc" | ".bash_profile" | ".bash_logout" => Some(LanguageInfo::new("Bash", "bash")),
        ".zshrc" | ".zprofile" | ".zshenv" => Some(LanguageInfo::new("Zsh", "zsh")),
        ".profile" => Some(LanguageInfo::new("Shell", "shell")),
        "go.mod" | "go.sum" => Some(LanguageInfo::new("Go", "go")),
        "gemfile" | "rakefile" => Some(LanguageInfo::new("Ruby", "ruby")),
        "vagrantfile" => Some(LanguageInfo::new("Vagrant", "vagrant")),
        "jenkinsfile" => Some(LanguageInfo::new("Jenkins", "jenkins")),
        _ => None,
    }
}

fn match_extension(ext: &str) -> Option<LanguageInfo> {
    match ext {
        "rs" => Some(LanguageInfo::new("Rust", "rust")),
        "ts" | "mts" | "cts" => Some(LanguageInfo::new("TypeScript", "typescript")),
        "tsx" => Some(LanguageInfo::new("TypeScript", "react")),
        "js" | "mjs" | "cjs" => Some(LanguageInfo::new("JavaScript", "javascript")),
        "jsx" => Some(LanguageInfo::new("JavaScript", "react")),
        "py" | "pyw" | "pyi" => Some(LanguageInfo::new("Python", "python")),
        "go" => Some(LanguageInfo::new("Go", "go")),
        "java" => Some(LanguageInfo::new("Java", "java")),
        "kt" | "kts" => Some(LanguageInfo::new("Kotlin", "kotlin")),
        "c" | "h" => Some(LanguageInfo::new("C", "c")),
        "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" => Some(LanguageInfo::new("C++", "cplusplus")),
        "cs" | "csx" => Some(LanguageInfo::new("C#", "csharp")),
        "fs" | "fsi" | "fsx" | "fsscript" => Some(LanguageInfo::new("F#", "fsharp")),
        "rb" | "rake" | "gemspec" => Some(LanguageInfo::new("Ruby", "ruby")),
        "php" | "phtml" => Some(LanguageInfo::new("PHP", "php")),
        "swift" => Some(LanguageInfo::new("Swift", "swift")),
        "dart" => Some(LanguageInfo::new("Dart", "dart")),
        "lua" => Some(LanguageInfo::new("Lua", "lua")),
        "sh" => Some(LanguageInfo::new("Shell", "shell")),
        "bash" => Some(LanguageInfo::new("Bash", "bash")),
        "zsh" => Some(LanguageInfo::new("Zsh", "zsh")),
        "nu" => Some(LanguageInfo::new("Nushell", "nushell")),
        "ps1" | "psm1" | "psd1" => Some(LanguageInfo::new("PowerShell", "powershell")),
        "html" | "htm" => Some(LanguageInfo::new("HTML", "html5")),
        "css" => Some(LanguageInfo::new("CSS", "css")),
        "scss" | "sass" => Some(LanguageInfo::new("Sass", "sass")),
        "less" => Some(LanguageInfo::new("Less", "less")),
        "json" | "jsonc" | "json5" => Some(LanguageInfo::new("JSON", "json")),
        "yaml" | "yml" => Some(LanguageInfo::new("YAML", "yaml")),
        "toml" => Some(LanguageInfo::new("TOML", "toml")),
        "md" | "markdown" | "mdx" => Some(LanguageInfo::new("Markdown", "markdown")),
        "sql" => Some(LanguageInfo::new("SQL", "sqlite")),
        "vue" => Some(LanguageInfo::new("Vue", "vuedotjs")),
        "svelte" => Some(LanguageInfo::new("Svelte", "svelte")),
        "astro" => Some(LanguageInfo::new("Astro", "astro")),
        "ex" | "exs" | "heex" | "leex" => Some(LanguageInfo::new("Elixir", "elixir")),
        "erl" | "hrl" => Some(LanguageInfo::new("Erlang", "erlang")),
        "hs" | "lhs" => Some(LanguageInfo::new("Haskell", "haskell")),
        "scala" | "sc" | "sbt" => Some(LanguageInfo::new("Scala", "scala")),
        "clj" | "cljc" | "edn" => Some(LanguageInfo::new("Clojure", "clojure")),
        "cljs" => Some(LanguageInfo::new("ClojureScript", "clojurescript")),
        "zig" => Some(LanguageInfo::new("Zig", "zig")),
        "nim" | "nims" => Some(LanguageInfo::new("Nim", "nim")),
        "ml" | "mli" => Some(LanguageInfo::new("OCaml", "ocaml")),
        "r" | "rmd" => Some(LanguageInfo::new("R", "r")),
        "jl" => Some(LanguageInfo::new("Julia", "julia")),
        "tf" | "tfvars" => Some(LanguageInfo::new("Terraform", "terraform")),
        "hcl" => Some(LanguageInfo::new("HCL", "hcl")),
        "graphql" | "gql" => Some(LanguageInfo::new("GraphQL", "graphql")),
        "sol" => Some(LanguageInfo::new("Solidity", "solidity")),
        "svg" => Some(LanguageInfo::new("SVG", "svg")),
        "tex" | "latex" | "bib" | "sty" | "cls" => Some(LanguageInfo::new("LaTeX", "latex")),
        "ipynb" => Some(LanguageInfo::new("Jupyter", "jupyter")),
        "dockerfile" => Some(LanguageInfo::new("Docker", "docker")),
        "mk" | "mak" => Some(LanguageInfo::new("Makefile", "make")),
        "cmake" => Some(LanguageInfo::new("CMake", "cmake")),
        "ada" | "adb" | "ads" => Some(LanguageInfo::new("Ada", "ada")),
        "apex" | "trigger" => Some(LanguageInfo::new("Apex", "apex")),
        "apl" => Some(LanguageInfo::new("APL", "apl")),
        "au3" => Some(LanguageInfo::new("AutoIt", "autoit")),
        "awk" => Some(LanguageInfo::new("AWK", "awk")),
        "ceylon" => Some(LanguageInfo::new("Ceylon", "ceylon")),
        "clar" => Some(LanguageInfo::new("Clarity", "clarity")),
        "cob" | "cbl" | "cpy" => Some(LanguageInfo::new("COBOL", "cobol")),
        "lisp" | "lsp" | "asd" => Some(LanguageInfo::new("Common Lisp", "commonlisp")),
        "cr" => Some(LanguageInfo::new("Crystal", "crystal")),
        "d" | "di" => Some(LanguageInfo::new("D", "d")),
        "f" | "f90" | "f95" | "f03" | "f08" | "for" => {
            Some(LanguageInfo::new("Fortran", "fortran"))
        }
        "gleam" => Some(LanguageInfo::new("Gleam", "gleam")),
        "hack" => Some(LanguageInfo::new("Hack", "hack")),
        "hx" | "hxml" => Some(LanguageInfo::new("Haxe", "haxe")),
        "imba" => Some(LanguageInfo::new("Imba", "imba")),
        "jule" => Some(LanguageInfo::new("Jule", "jule")),
        "mlx" => Some(LanguageInfo::new("MATLAB", "matlab")),
        "mint" => Some(LanguageInfo::new("Mint", "mint")),
        "mojo" => Some(LanguageInfo::new("Mojo", "mojo")),
        "asm" | "nasm" | "s" => Some(LanguageInfo::new("Assembly", "nasm")),
        "nix" => Some(LanguageInfo::new("Nix", "nixos")),
        "m" | "mm" => Some(LanguageInfo::new("Objective-C", "objectivec")),
        "odin" => Some(LanguageInfo::new("Odin", "odin")),
        "cl" => Some(LanguageInfo::new("OpenCL", "opencl")),
        "pl" | "pm" => Some(LanguageInfo::new("Perl", "perl")),
        "prolog" | "plg" => Some(LanguageInfo::new("Prolog", "prolog")),
        "purs" => Some(LanguageInfo::new("PureScript", "purescript")),
        "rkt" => Some(LanguageInfo::new("Racket", "racket")),
        "re" | "rei" => Some(LanguageInfo::new("ReasonML", "reasonml")),
        "rexx" | "rex" => Some(LanguageInfo::new("Rexx", "rexx")),
        "v" | "vsh" => Some(LanguageInfo::new("V", "v")),
        "vala" | "vapi" => Some(LanguageInfo::new("Vala", "vala")),
        "vim" | "vimrc" => Some(LanguageInfo::new("Vim Script", "vim")),
        "vb" | "vbs" | "bas" => Some(LanguageInfo::new("Visual Basic", "visualbasic")),
        "vy" => Some(LanguageInfo::new("Vyper", "vyper")),
        "wat" | "wast" | "wasm" => Some(LanguageInfo::new("WebAssembly", "webassembly")),
        "w" => Some(LanguageInfo::new("Wing", "winglang")),
        "xtend" => Some(LanguageInfo::new("Xtend", "xtend")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_common_extensions() {
        let cases = [
            ("main.rs", "Rust", "rust"),
            ("app.ts", "TypeScript", "typescript"),
            ("App.tsx", "TypeScript", "react"),
            ("index.js", "JavaScript", "javascript"),
            ("mod.mjs", "JavaScript", "javascript"),
            ("cfg.cjs", "JavaScript", "javascript"),
            ("Button.jsx", "JavaScript", "react"),
            ("script.py", "Python", "python"),
            ("main.go", "Go", "go"),
            ("App.java", "Java", "java"),
            ("Main.kt", "Kotlin", "kotlin"),
            ("main.c", "C", "c"),
            ("header.h", "C", "c"),
            ("engine.cpp", "C++", "cplusplus"),
            ("engine.cc", "C++", "cplusplus"),
            ("engine.hpp", "C++", "cplusplus"),
            ("Program.cs", "C#", "csharp"),
            ("Program.fs", "F#", "fsharp"),
            ("app.rb", "Ruby", "ruby"),
            ("index.php", "PHP", "php"),
            ("View.swift", "Swift", "swift"),
            ("main.dart", "Dart", "dart"),
            ("init.lua", "Lua", "lua"),
            ("run.sh", "Shell", "shell"),
            ("run.bash", "Bash", "bash"),
            ("init.zsh", "Zsh", "zsh"),
            ("deploy.ps1", "PowerShell", "powershell"),
            ("index.html", "HTML", "html5"),
            ("page.htm", "HTML", "html5"),
            ("styles.css", "CSS", "css"),
            ("theme.scss", "Sass", "sass"),
            ("theme.sass", "Sass", "sass"),
            ("vars.less", "Less", "less"),
            ("tsconfig.json", "JSON", "json"),
            ("ci.yaml", "YAML", "yaml"),
            ("ci.yml", "YAML", "yaml"),
            ("settings.toml", "TOML", "toml"),
            ("README.md", "Markdown", "markdown"),
            ("schema.sql", "SQL", "sqlite"),
            ("App.vue", "Vue", "vuedotjs"),
            ("App.svelte", "Svelte", "svelte"),
            ("index.astro", "Astro", "astro"),
            ("app.ex", "Elixir", "elixir"),
            ("mix.exs", "Elixir", "elixir"),
            ("server.erl", "Erlang", "erlang"),
            ("Main.hs", "Haskell", "haskell"),
            ("App.scala", "Scala", "scala"),
            ("core.clj", "Clojure", "clojure"),
            ("main.zig", "Zig", "zig"),
            ("app.nim", "Nim", "nim"),
            ("main.ml", "OCaml", "ocaml"),
            ("plot.r", "R", "r"),
            ("model.jl", "Julia", "julia"),
            ("main.tf", "Terraform", "terraform"),
            ("config.hcl", "HCL", "hcl"),
            ("schema.graphql", "GraphQL", "graphql"),
            ("query.gql", "GraphQL", "graphql"),
            ("Token.sol", "Solidity", "solidity"),
            ("logo.svg", "SVG", "svg"),
            ("paper.tex", "LaTeX", "latex"),
            ("notebook.ipynb", "Jupyter", "jupyter"),
        ];

        for (file, name, icon) in cases {
            assert_eq!(
                detect_language(file),
                Some(LanguageInfo { name, icon }),
                "failed for {file}"
            );
        }
    }

    #[test]
    fn detects_additional_languages() {
        let cases = [
            ("hello.adb", "Ada", "ada"),
            ("payroll.cob", "COBOL", "cobol"),
            ("sim.f90", "Fortran", "fortran"),
            ("server.cr", "Crystal", "crystal"),
            ("app.gleam", "Gleam", "gleam"),
            ("Main.hx", "Haxe", "haxe"),
            ("flake.nix", "Nix", "nixos"),
            ("game.odin", "Odin", "odin"),
            ("module.wat", "WebAssembly", "webassembly"),
        ];

        for (file, name, icon) in cases {
            assert_eq!(
                detect_language(file),
                Some(LanguageInfo { name, icon }),
                "failed for {file}"
            );
        }
    }

    #[test]
    fn detects_special_filenames() {
        let cases = [
            ("Dockerfile", "Docker", "docker"),
            ("Dockerfile.prod", "Docker", "docker"),
            ("Makefile", "Makefile", "make"),
            ("CMakeLists.txt", "CMake", "cmake"),
            ("Cargo.toml", "Rust", "rust"),
            ("Cargo.lock", "Rust", "rust"),
            ("package.json", "Node.js", "nodedotjs"),
            (".gitignore", "Git", "git"),
            (".gitattributes", "Git", "git"),
            (".env", "Shell", "shell"),
            (".env.local", "Shell", "shell"),
            (".bashrc", "Bash", "bash"),
            (".zshrc", "Zsh", "zsh"),
        ];

        for (file, name, icon) in cases {
            assert_eq!(
                detect_language(file),
                Some(LanguageInfo { name, icon }),
                "failed for {file}"
            );
        }
    }

    #[test]
    fn matching_is_case_insensitive_and_handles_paths() {
        assert_eq!(
            detect_language("MAIN.RS"),
            Some(LanguageInfo::new("Rust", "rust"))
        );
        assert_eq!(
            detect_language("DOCKERFILE"),
            Some(LanguageInfo::new("Docker", "docker"))
        );
        assert_eq!(
            detect_language("cargo.toml"),
            Some(LanguageInfo::new("Rust", "rust"))
        );
        assert_eq!(
            detect_language("src/main.rs"),
            Some(LanguageInfo::new("Rust", "rust"))
        );
        assert_eq!(
            detect_language("src\\app.ts"),
            Some(LanguageInfo::new("TypeScript", "typescript"))
        );
        assert_eq!(
            detect_language("  my-file - test.rs  "),
            Some(LanguageInfo::new("Rust", "rust"))
        );
    }

    #[test]
    fn unknown_and_empty_filenames_return_none() {
        for file in ["", "   ", "test.unknown", "NOTES", ".unknown"] {
            assert_eq!(detect_language(file), None, "expected None for {file:?}");
        }
    }

    #[test]
    fn language_icon_url_uses_raw_github_icons_path() {
        assert_eq!(
            language_icon_url("rust"),
            "https://raw.githubusercontent.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/refs/heads/main/public/assets/icons/rust.png"
        );
    }
}
