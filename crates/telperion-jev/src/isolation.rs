//! Guard: Jev stays out of generation, rendering, presets and the browser.

use std::fs;
use std::path::{Path, PathBuf};

use crate::caller::ENDPOINT;

const FORBIDDEN_CRATES: &[&str] = &[
    "crates/telperion-core",
    "crates/telperion-render",
    "crates/telperion-wasm",
    "src",
];

const FORBIDDEN_MARKERS: &[&str] = &[
    "https://api.typesafe.ai/v1/systemone",
    "telperion-jev",
    "telperion_jev",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsolationHit {
    pub file: PathBuf,
    pub marker: String,
}

impl IsolationHit {
    pub fn message(&self) -> String {
        format!(
            "{} names {} — Jev never runs in generation, rendering, presets or the browser source",
            self.file.display(),
            self.marker
        )
    }
}

/// Walk the generation path and report every file that imports the caller or
/// names the endpoint.
pub fn scan(repo_root: &Path) -> Vec<IsolationHit> {
    let mut hits = Vec::new();
    for rel in FORBIDDEN_CRATES {
        let dir = repo_root.join(rel);
        walk(&dir, repo_root, &mut hits);
    }
    hits
}

fn walk(dir: &Path, repo_root: &Path, hits: &mut Vec<IsolationHit>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "target" || name == "node_modules" || name == "render" {
                continue;
            }
            walk(&path, repo_root, hits);
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !matches!(ext, "rs" | "ts" | "tsx" | "js" | "mjs" | "toml") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for marker in FORBIDDEN_MARKERS {
            if text.contains(marker) {
                let rel = path.strip_prefix(repo_root).unwrap_or(&path).to_path_buf();
                hits.push(IsolationHit {
                    file: rel,
                    marker: (*marker).to_string(),
                });
            }
        }
    }
}

/// The endpoint string the isolation scan looks for. Tests assert it matches
/// the caller constant so a rename cannot hide the import.
pub fn endpoint_marker() -> &'static str {
    ENDPOINT
}
