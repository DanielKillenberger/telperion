//! What each stage read and wrote, by content hash, and the run's log.
//!
//! A stage is current when every file it read hashes as it did and every file
//! it writes exists. An output is never compared with what its stage wrote: a
//! file edited by hand, or written again by a later stage, is an input to what
//! reads it, so the edit reruns the readers and leaves the writer alone. An
//! output removed by hand reruns its stage.
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::pipeline::canon::{read_json, write_canonical};

/// The hash a record keeps for a file: its bytes', or `absent`.
pub fn hash(path: &Path) -> Result<String, String> {
    match std::fs::read(path) {
        Ok(bytes) => Ok(crate::sha256_hex(&bytes)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok("absent".into()),
        Err(e) => Err(format!("{}: {e}", path.display())),
    }
}

fn hashes(paths: &[PathBuf]) -> Result<BTreeMap<String, String>, String> {
    paths
        .iter()
        .map(|p| Ok((super::shown(p), hash(p)?)))
        .collect()
}

pub struct Records {
    dir: PathBuf,
    stages: BTreeMap<String, Value>,
}

impl Records {
    pub fn load(dir: &Path) -> Result<Self, String> {
        let path = dir.join("records.json");
        let stages = match path.exists() {
            true => serde_json::from_value(
                read_json(&path).map_err(|e| e.to_string())?["stages"].clone(),
            )
            .map_err(|e| format!("{}: {e}", path.display()))?,
            false => BTreeMap::new(),
        };
        Ok(Self {
            dir: dir.to_path_buf(),
            stages,
        })
    }

    /// True when the stage's inputs hash as recorded and its outputs exist.
    pub fn current(
        &self,
        stage: &str,
        inputs: &[PathBuf],
        outputs: &[PathBuf],
    ) -> Result<bool, String> {
        let Some(record) = self.stages.get(stage) else {
            return Ok(false);
        };
        Ok(record["inputs"] == json!(hashes(inputs)?) && outputs.iter().all(|p| p.exists()))
    }

    pub fn set(&mut self, stage: &str, inputs: &[PathBuf]) -> Result<(), String> {
        let record = json!({"inputs": hashes(inputs)?});
        self.stages.insert(stage.into(), record);
        let value = json!({"schema": "runner-records", "schema_version": 1, "stages": self.stages});
        write_canonical(&self.dir.join("records.json"), &value)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    /// Appends one line to `log.jsonl`: every stage's word and every
    /// condition the run logged instead of stopping.
    pub fn log(&self, stage: &str, word: &str) -> Result<(), String> {
        let line = json!({"at": now(), "stage": stage, "word": word});
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.dir.join("log.jsonl"))
            .map_err(|e| e.to_string())?;
        writeln!(file, "{line}").map_err(|e| e.to_string())
    }
}

/// Seconds since the epoch, as the log records them.
pub fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}
