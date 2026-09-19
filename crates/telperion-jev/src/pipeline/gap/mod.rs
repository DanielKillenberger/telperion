//! The gap loop (fn-63): options, escalation and resume.
//!
//! A gap is a typed halt: an `onboarding-gate` or `level-miss` decision the
//! species run filed. The agent, or a stronger model, writes two to four
//! candidate fixes in a fixed shape; Jev answers narrow questions over the
//! set; code reads the answers into signals and looks the route up in one
//! threshold table a person can read and change. The chosen fix is minted as
//! its own spec, reviewed, and on landing the run's idempotence keys expire
//! from the halted stage down. Every record lives under `DIR/gaps/<slug>/`,
//! the value rounds under `DIR/rounds.json` and the per-run numbers under
//! `DIR/metrics.json`. Jev never writes an option, a fix or a number.

pub mod cli;
pub mod metrics;
pub mod option;
pub mod questions;
pub mod resume;
pub mod rounds;
pub mod route;
pub mod table;

use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use super::canon::{read_json, write_canonical, CanonError};
use super::decision::{read_decisions, Decision};
use super::stage::Paths;
use crate::caller::CallerError;

pub const GAP_SCHEMA_VERSION: u32 = 1;
/// The decision kinds a gap starts from; anything else stays inside the run.
pub const HALT_KINDS: [&str; 2] = ["onboarding-gate", "level-miss"];
/// The pseudo-stage the loop's own decisions are filed under.
pub const STAGE: &str = "gap";

#[derive(Debug)]
pub enum GapError {
    File(CanonError),
    Invalid(String),
    Caller(CallerError),
}

impl std::fmt::Display for GapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(err) => write!(f, "{err}"),
            Self::Invalid(msg) => write!(f, "gap: {msg}"),
            Self::Caller(err) => write!(f, "gap: {err}"),
        }
    }
}

impl std::error::Error for GapError {}

impl From<CanonError> for GapError {
    fn from(err: CanonError) -> Self {
        Self::File(err)
    }
}

impl From<CallerError> for GapError {
    fn from(err: CallerError) -> Self {
        Self::Caller(err)
    }
}

/// `stage-kind-field` from a decision id: one species per directory, so the
/// species is dropped and the slashes become dashes.
pub fn slug(decision_id: &str) -> String {
    decision_id
        .split_once('/')
        .map(|(_, rest)| rest)
        .unwrap_or(decision_id)
        .replace('/', "-")
}

pub fn gap_dir(paths: &Paths, decision_id: &str) -> PathBuf {
    paths.dir.join("gaps").join(slug(decision_id))
}

pub fn record_path(paths: &Paths, decision_id: &str) -> PathBuf {
    gap_dir(paths, decision_id).join("gap.json")
}

/// The halting decision a gap starts from, or why it is not a gap.
pub fn halt(paths: &Paths, decision_id: &str) -> Result<Decision, GapError> {
    let decisions = read_decisions(&paths.decisions())?;
    let Some(decision) = decisions.into_iter().find(|d| d.id == decision_id) else {
        return Err(GapError::Invalid(format!(
            "{decision_id} is not a decision of this run"
        )));
    };
    if !HALT_KINDS.contains(&decision.kind.as_str()) {
        return Err(GapError::Invalid(format!(
            "{decision_id} is a {} decision; the loop starts only from {}",
            decision.kind,
            HALT_KINDS.join(" or ")
        )));
    }
    Ok(decision)
}

/// The gap record, created from the halting decision on first use.
pub fn open(paths: &Paths, decision_id: &str) -> Result<Value, GapError> {
    let path = record_path(paths, decision_id);
    if path.exists() {
        return Ok(read_json(&path)?);
    }
    let decision = halt(paths, decision_id)?;
    Ok(json!({
        "schema": "gap",
        "schema_version": GAP_SCHEMA_VERSION,
        "gap": decision.id,
        "species": decision.species,
        "halt": {
            "id": decision.id,
            "stage": decision.stage,
            "kind": decision.kind,
            "field": decision.field,
            "inputs_sha256": decision.inputs_sha256,
            "payload": decision.payload,
        },
        "sets": [],
        "routes": [],
        "route": Value::Null,
        "chosen": Value::Null,
        "spec": Value::Null,
        "reviews": [],
        "landed": Value::Null,
    }))
}

/// A gap record that must already exist.
pub fn read(paths: &Paths, decision_id: &str) -> Result<Value, GapError> {
    let path = record_path(paths, decision_id);
    if !path.exists() {
        return Err(GapError::Invalid(format!(
            "no gap record for {decision_id}; write its option set first"
        )));
    }
    Ok(read_json(&path)?)
}

pub fn write(paths: &Paths, record: &Value) -> Result<PathBuf, GapError> {
    let id = record["gap"].as_str().unwrap_or_default();
    let path = record_path(paths, id);
    write_canonical(&path, record)?;
    Ok(path)
}

/// Every gap record under `DIR/gaps`, in slug order.
pub fn all(dir: &Path) -> Result<Vec<Value>, GapError> {
    let root = dir.join("gaps");
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut entries: Vec<PathBuf> = std::fs::read_dir(&root)
        .map_err(|error| CanonError::Io {
            path: root.clone(),
            error: error.to_string(),
        })?
        .filter_map(|entry| entry.ok().map(|e| e.path().join("gap.json")))
        .filter(|path| path.exists())
        .collect();
    entries.sort();
    entries.iter().map(|path| Ok(read_json(path)?)).collect()
}

/// The clock, once per record, so a test can pin it.
pub fn now() -> String {
    std::env::var("TELPERION_GAP_CLOCK").unwrap_or_else(|_| crate::caller::now_rfc3339())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_slug_drops_the_species_and_joins_the_rest() {
        assert_eq!(
            slug("oregon-white-oak/gate/onboarding-gate/capability"),
            "gate-onboarding-gate-capability"
        );
        assert_eq!(slug("no-slash"), "no-slash");
    }
}
