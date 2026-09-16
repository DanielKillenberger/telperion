//! Ledger entry: the one shape every tool shares.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

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
}

impl LedgerEntry {
    pub fn reference(&self) -> String {
        format!(
            "{}:{}",
            self.tool,
            &self.state_sha256[..12.min(self.state_sha256.len())]
        )
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
        &entry.state_sha256[..12.min(entry.state_sha256.len())]
    );
    let path = dir.join(name);
    let bytes = serde_json::to_vec_pretty(entry).map_err(|err| err.to_string())?;
    let mut file = fs::File::create(&path).map_err(|err| err.to_string())?;
    file.write_all(&bytes).map_err(|err| err.to_string())?;
    Ok(path)
}
