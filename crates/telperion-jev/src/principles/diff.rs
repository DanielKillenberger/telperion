//! What a change touched: the files, and the lines it added at the head and
//! removed at the base, read from git between two immutable revisions.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::source::git;

#[derive(Debug, Clone, Default)]
pub struct Change {
    /// Paths at the head that were added or modified.
    pub touched: BTreeSet<String>,
    /// Paths added at the head.
    pub added_files: BTreeSet<String>,
    /// Paths removed from the base.
    pub removed_files: BTreeSet<String>,
    /// Head line numbers each touched file added.
    pub added: BTreeMap<String, BTreeSet<usize>>,
    /// Base line numbers each file removed, keyed by base path.
    pub removed: BTreeMap<String, BTreeSet<usize>>,
}

impl Change {
    pub fn of(repo: &Path, base: &str, head: &str) -> Result<Self, String> {
        let names = git(repo, &["diff", "--name-status", "--no-renames", base, head])?;
        let mut change = Self::default();
        for line in names.lines() {
            let mut parts = line.split('\t');
            let (Some(status), Some(path)) = (parts.next(), parts.next()) else { continue };
            match status {
                "D" => {
                    change.removed_files.insert(path.to_string());
                }
                "A" => {
                    change.added_files.insert(path.to_string());
                    change.touched.insert(path.to_string());
                }
                _ => {
                    change.touched.insert(path.to_string());
                }
            }
        }
        let patch = git(repo, &["diff", "-U0", "--no-renames", "--no-color", base, head])?;
        change.read_hunks(&patch);
        Ok(change)
    }

    fn read_hunks(&mut self, patch: &str) {
        let (mut old_path, mut new_path) = (String::new(), String::new());
        let (mut old_line, mut new_line) = (0, 0);
        for line in patch.lines() {
            if let Some(p) = line.strip_prefix("--- ") {
                old_path = p.strip_prefix("a/").unwrap_or(p).to_string();
            } else if let Some(p) = line.strip_prefix("+++ ") {
                new_path = p.strip_prefix("b/").unwrap_or(p).to_string();
            } else if let Some(h) = line.strip_prefix("@@ ") {
                let mut it = h.split_whitespace();
                old_line = start(it.next().unwrap_or("-0"));
                new_line = start(it.next().unwrap_or("+0"));
            } else if line.starts_with('+') {
                self.added.entry(new_path.clone()).or_default().insert(new_line);
                new_line += 1;
            } else if line.starts_with('-') {
                self.removed.entry(old_path.clone()).or_default().insert(old_line);
                old_line += 1;
            }
        }
    }

    /// True when the change added any line of `path` in `lines`.
    pub fn adds_within(&self, path: &str, lines: (usize, usize)) -> bool {
        self.added
            .get(path)
            .is_some_and(|set| set.range(lines.0..=lines.1).next().is_some())
    }

    pub fn added_count(&self) -> usize {
        self.added.values().map(BTreeSet::len).sum()
    }

    /// Paths touched or removed, for the guards' triggers.
    pub fn paths(&self) -> impl Iterator<Item = &String> {
        self.touched.iter().chain(&self.removed_files)
    }
}

/// `-12,3` or `+40` to its first line number.
fn start(range: &str) -> usize {
    range[1..]
        .split(',')
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}
