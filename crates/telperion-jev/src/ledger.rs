//! Ledger entry: the one shape every tool shares.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::Value;

static ENTRY_SEQ: AtomicU64 = AtomicU64::new(0);

/// Unique immutable id for one ledger write. Distinct even when the state
/// hash and the clock second match.
pub fn new_entry_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let seq = ENTRY_SEQ.fetch_add(1, Ordering::Relaxed);
    format!("{nanos:x}-{:x}-{seq:x}", std::process::id())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceRef {
    pub id: String,
    pub url: String,
    pub sha256: String,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Usage {
    pub input_tokens: u64,
    pub output_tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub id: String,
    pub tool: String,
    pub state_sha256: String,
    pub source: Option<SourceRef>,
    pub model: String,
    pub questions: Value,
    pub answers: Value,
    pub usage: Option<Usage>,
    pub elapsed_ms: u64,
    pub recorded_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Derived from the state checksum, the questions and the model name, so
    /// identical requests share one identity across runs and drivers.
    #[serde(default)]
    pub identity: String,
}

/// `sha256(state checksum, canonical questions, model)`, 24 hex characters.
pub fn derived_identity(state_sha256: &str, questions: &Value, model: &str) -> String {
    let questions = serde_json::to_vec(questions).expect("questions serialize");
    let digest = crate::sha256_hex(
        format!("{state_sha256}\n{}\n{model}", crate::sha256_hex(&questions)).as_bytes(),
    );
    digest[..24].to_string()
}

impl LedgerEntry {
    pub fn reference(&self) -> String {
        format!("{}:{}", self.tool, self.id)
    }

    pub fn choice(&self, question: &str) -> Option<String> {
        self.answers
            .get(question)
            .and_then(|ans| ans.get("choice"))
            .and_then(Value::as_str)
            .map(str::to_string)
    }

    pub fn noul(&self, question: &str) -> Option<f64> {
        self.answers
            .get(question)
            .and_then(|ans| ans.get("noul"))
            .and_then(Value::as_f64)
    }

    pub fn score(&self, question: &str) -> Option<f64> {
        self.answers
            .get(question)
            .and_then(|ans| ans.get("score"))
            .and_then(Value::as_f64)
    }

    pub fn confidence(&self, question: &str) -> Option<f64> {
        self.answers
            .get(question)
            .and_then(|ans| ans.get("confidence"))
            .and_then(Value::as_f64)
    }

    pub fn probabilities(&self, question: &str) -> Option<&Value> {
        self.answers.get(question)?.get("probabilities")
    }

    pub fn top_probability(&self, question: &str) -> f64 {
        if let Some(noul) = self.noul(question) {
            return noul.max(1.0 - noul);
        }
        self.probabilities(question)
            .and_then(Value::as_object)
            .and_then(|map| {
                map.values()
                    .filter_map(Value::as_f64)
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            })
            .unwrap_or(0.0)
    }
}

pub fn write_entry(dir: &Path, entry: &LedgerEntry) -> Result<PathBuf, String> {
    fs::create_dir_all(dir).map_err(|err| err.to_string())?;
    let name = format!(
        "{}-{}-{}.json",
        entry.recorded_at.replace(':', ""),
        entry.tool,
        entry.id
    );
    let path = dir.join(name);
    let bytes = serde_json::to_vec_pretty(entry).map_err(|err| err.to_string())?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|err| {
            if err.kind() == std::io::ErrorKind::AlreadyExists {
                format!("ledger file already exists: {}", path.display())
            } else {
                err.to_string()
            }
        })?;
    file.write_all(&bytes).map_err(|err| err.to_string())?;
    file.sync_all().map_err(|err| err.to_string())?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample(id: &str) -> LedgerEntry {
        LedgerEntry {
            id: id.into(),
            tool: "screen".into(),
            state_sha256: "abcd".into(),
            source: None,
            model: "jev-latest".into(),
            questions: json!({}),
            answers: json!({}),
            usage: None,
            elapsed_ms: 1,
            recorded_at: "2026-09-16T00:00:00Z".into(),
            error: None,
            identity: String::new(),
        }
    }

    fn tempdir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "jev-ledger-{}-{}",
            std::process::id(),
            new_entry_id()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn ids_are_unique_and_used_as_the_reference() {
        let a = new_entry_id();
        let b = new_entry_id();
        assert_ne!(a, b);
        let entry = sample(&a);
        assert_eq!(entry.reference(), format!("screen:{a}"));
        assert!(!entry.reference().contains(&entry.state_sha256));
    }

    #[test]
    fn write_refuses_to_overwrite() {
        let dir = tempdir();
        let entry = sample("fixed-id");
        write_entry(&dir, &entry).unwrap();
        let err = write_entry(&dir, &entry).unwrap_err();
        assert!(err.contains("already exists"), "{err}");
        assert_eq!(fs::read_dir(&dir).unwrap().count(), 1);
    }
}
