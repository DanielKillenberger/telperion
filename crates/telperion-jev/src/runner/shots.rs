//! A found photograph's shot, chosen from code's candidates (host,
//! 2026-09-26). Not yet built.
use std::path::{Path, PathBuf};

/// The references with their chosen shots, written beside the run.
pub fn file(out: &Path) -> PathBuf {
    out.join("shots.json")
}
