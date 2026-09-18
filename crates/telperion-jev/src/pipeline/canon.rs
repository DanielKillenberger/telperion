//! Canonical serialization and atomic artifact writes.
//!
//! Every artifact is JSON with keys sorted (serde_json's map is ordered),
//! compact separators, shortest round-trip number formatting and one trailing
//! newline, so two runs that produce the same values produce the same bytes.
//! A write lands in a sibling temporary file, is synced, and is renamed over
//! the target; a crash leaves the old artifact or none, never a partial one.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::sha256_hex;

#[derive(Debug)]
pub enum CanonError {
    Io { path: PathBuf, error: String },
    Json { path: PathBuf, error: String },
}

impl std::fmt::Display for CanonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, error } => write!(f, "{}: {error}", path.display()),
            Self::Json { path, error } => write!(f, "{}: json: {error}", path.display()),
        }
    }
}

impl std::error::Error for CanonError {}

/// The canonical bytes of a value: sorted keys, compact, trailing newline.
/// Numbers are the fixed point of serialize-then-parse, so a value read back
/// from disk and written again produces the same bytes.
pub fn canonical_bytes(value: &Value) -> Vec<u8> {
    let mut bytes = serde_json::to_vec(value).expect("a Value serializes");
    for _ in 0..3 {
        let again: Value = serde_json::from_slice(&bytes).expect("canonical bytes parse");
        let next = serde_json::to_vec(&again).expect("a Value serializes");
        if next == bytes {
            break;
        }
        bytes = next;
    }
    bytes.push(b'\n');
    bytes
}

/// SHA-256 of the canonical bytes.
pub fn canonical_sha256(value: &Value) -> String {
    sha256_hex(&canonical_bytes(value))
}

/// Writes `bytes` to `path` through a sibling temporary file and a rename.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), CanonError> {
    let io = |error: std::io::Error| CanonError::Io {
        path: path.to_path_buf(),
        error: error.to_string(),
    };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io)?;
    }
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let tmp = path.with_file_name(format!(".{name}.{}.tmp", std::process::id()));
    let result = (|| {
        let mut file = fs::File::create(&tmp)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        fs::rename(&tmp, path)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result.map_err(io)
}

/// Writes a value's canonical bytes atomically and returns their checksum.
pub fn write_canonical(path: &Path, value: &Value) -> Result<String, CanonError> {
    let bytes = canonical_bytes(value);
    write_atomic(path, &bytes)?;
    Ok(sha256_hex(&bytes))
}

pub fn read_json(path: &Path) -> Result<Value, CanonError> {
    let bytes = fs::read(path).map_err(|error| CanonError::Io {
        path: path.to_path_buf(),
        error: error.to_string(),
    })?;
    serde_json::from_slice(&bytes).map_err(|error| CanonError::Json {
        path: path.to_path_buf(),
        error: error.to_string(),
    })
}

/// SHA-256 of a file's bytes as they are on disk.
pub fn file_sha256(path: &Path) -> Result<String, CanonError> {
    fs::read(path)
        .map(|bytes| sha256_hex(&bytes))
        .map_err(|error| CanonError::Io {
            path: path.to_path_buf(),
            error: error.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "jev-canon-{tag}-{}-{}",
            std::process::id(),
            crate::ledger::new_entry_id()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn canonical_bytes_sort_keys_and_end_with_one_newline() {
        let a = json!({"b": 1, "a": [1.5, 2], "c": {"z": null, "y": "s"}});
        let b = json!({"c": {"y": "s", "z": null}, "a": [1.5, 2], "b": 1});
        assert_eq!(canonical_bytes(&a), canonical_bytes(&b));
        assert_eq!(
            String::from_utf8(canonical_bytes(&a)).unwrap(),
            "{\"a\":[1.5,2],\"b\":1,\"c\":{\"y\":\"s\",\"z\":null}}\n"
        );
        assert_eq!(canonical_sha256(&a), canonical_sha256(&b));
    }

    #[test]
    fn a_number_read_back_from_its_canonical_bytes_writes_the_same_bytes() {
        let value = json!({"reference_dbh_m": 0.23570950225444562, "n": 1e21, "f": 0.1});
        let bytes = canonical_bytes(&value);
        let again: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(canonical_bytes(&again), bytes);
    }

    #[test]
    fn write_atomic_leaves_no_temporary_file_and_replaces_the_target() {
        let dir = scratch("atomic");
        let path = dir.join("nested").join("artifact.json");
        write_atomic(&path, b"one\n").unwrap();
        write_atomic(&path, b"two\n").unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"two\n");
        let names: Vec<_> = fs::read_dir(path.parent().unwrap())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, vec!["artifact.json"]);
    }

    #[test]
    fn write_canonical_returns_the_checksum_of_the_bytes_on_disk() {
        let dir = scratch("checksum");
        let path = dir.join("a.json");
        let sha = write_canonical(&path, &json!({"k": 1})).unwrap();
        assert_eq!(sha, file_sha256(&path).unwrap());
        assert_eq!(read_json(&path).unwrap(), json!({"k": 1}));
    }

    #[test]
    fn read_json_names_the_path_on_a_missing_file() {
        let err = read_json(Path::new("/nonexistent/x.json")).unwrap_err();
        assert!(err.to_string().starts_with("/nonexistent/x.json"), "{err}");
    }
}
