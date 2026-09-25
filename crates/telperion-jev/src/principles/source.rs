//! A tree of source files: a checkout on disk or an immutable revision read
//! through git. Every guard and extractor reads through here, so a push is
//! judged on the revision pushed, never on whatever the checkout holds.

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

/// Repository-relative path to file text. Binary files are left out.
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    pub files: BTreeMap<String, String>,
}

/// The files a guard reads: Rust, shaders, TypeScript, Python, JSON, specs.
pub fn wanted(path: &str) -> bool {
    let ext = path.rsplit('.').next().unwrap_or("");
    let skip = path.starts_with("experiments/")
        || path.starts_with("node_modules/")
        || path.starts_with(".flow/")
        || path.contains("/fixtures/")
        || path.contains("/fixture/");
    !skip && matches!(ext, "rs" | "wgsl" | "ts" | "tsx" | "py" | "mjs" | "toml")
}

impl Snapshot {
    pub fn get(&self, path: &str) -> Option<&str> {
        self.files.get(path).map(String::as_str)
    }

    /// Files under `root` on disk that `keep` accepts.
    pub fn from_dir(root: &Path, keep: fn(&str) -> bool) -> Result<Self, String> {
        let mut files = BTreeMap::new();
        let mut stack = vec![root.to_path_buf()];
        while let Some(dir) = stack.pop() {
            let entries = std::fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
            for entry in entries.flatten() {
                let path = entry.path();
                let rel = path
                    .strip_prefix(root)
                    .map_err(|e| e.to_string())?
                    .to_string_lossy()
                    .replace('\\', "/");
                if rel == "target" || rel.starts_with('.') && path.is_dir() {
                    continue;
                }
                if path.is_dir() {
                    if rel != "node_modules" && !rel.ends_with("/target") {
                        stack.push(path);
                    }
                } else if keep(&rel) {
                    if let Ok(text) = std::fs::read_to_string(&path) {
                        files.insert(rel, text);
                    }
                }
            }
        }
        Ok(Self { files })
    }

    /// Files at `rev` that `keep` accepts, read in one `git cat-file` pass.
    pub fn from_git(repo: &Path, rev: &str, keep: fn(&str) -> bool) -> Result<Self, String> {
        let list = git(repo, &["ls-tree", "-r", "--full-tree", "--format=%(objectname) %(path)", rev])?;
        let wanted: Vec<(String, String)> = list
            .lines()
            .filter_map(|line| line.split_once(' '))
            .filter(|(_, path)| keep(path))
            .map(|(oid, path)| (oid.to_string(), path.to_string()))
            .collect();
        let mut child = Command::new("git")
            .current_dir(repo)
            .args(["cat-file", "--batch"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("git cat-file: {e}"))?;
        let mut stdin = child.stdin.take().expect("piped stdin");
        let oids: String = wanted.iter().map(|(oid, _)| format!("{oid}\n")).collect();
        let writer = std::thread::spawn(move || stdin.write_all(oids.as_bytes()));
        let mut out = BufReader::new(child.stdout.take().expect("piped stdout"));
        let mut files = BTreeMap::new();
        for (_, path) in &wanted {
            let mut header = String::new();
            out.read_line(&mut header).map_err(|e| e.to_string())?;
            let size: usize = header
                .split_whitespace()
                .nth(2)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| format!("git cat-file header: {header}"))?;
            let mut body = vec![0; size + 1];
            out.read_exact(&mut body).map_err(|e| e.to_string())?;
            body.pop();
            if let Ok(text) = String::from_utf8(body) {
                files.insert(path.clone(), text);
            }
        }
        writer
            .join()
            .map_err(|_| "git cat-file writer panicked".to_string())?
            .map_err(|e| e.to_string())?;
        child.wait().map_err(|e| e.to_string())?;
        Ok(Self { files })
    }
}

/// Runs git in `repo` and returns stdout, or its stderr as the error.
pub fn git(repo: &Path, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .current_dir(repo)
        .args(args)
        .output()
        .map_err(|e| format!("git {}: {e}", args.join(" ")))?;
    if !out.status.success() {
        return Err(format!(
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}
