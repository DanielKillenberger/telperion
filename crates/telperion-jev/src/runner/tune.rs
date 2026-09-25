//! The Tune stage: one tuning revision over the live dials.
//!
//! A revision starts from the last kept tree, or from the Start overlay on
//! the first. Its dials are the rows of the dial table read from disk when it
//! starts, never a copy frozen into the config, so a row the generator gained
//! or lost is offered or dropped on the next revision. It draws and measures
//! with the tools the runner just built. The revision ends when its rounds
//! stop keeping anything, and its result becomes the stage's artifact. A
//! revision that failed (no key, an interrupted paid call, no tree kept)
//! leaves the last kept result alone and fails the stage, so the next run
//! tries again.
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use super::tools::Tools;
use crate::caller::{load_key, UreqTransport};
use crate::pipeline::canon::read_json;
use crate::tuning::actions::Dial;

/// The dial table, as a stage input and as the rows a revision offers.
pub fn table() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data/dials.json")
}

/// The files the tuning config names whose bytes a revision reads: the
/// profile manifest, the camera references, the reference inventory and its
/// preparation, and the contact-sheet protocol.
pub fn referenced(template: &Path) -> Result<Vec<PathBuf>, String> {
    let config = read_json(template).map_err(|e| e.to_string())?;
    let named = [
        "/profiles",
        "/matched/references",
        "/reference_first/inventory/path",
        "/reference_first/preparation/path",
        "/sheet/protocol",
    ];
    Ok(named
        .iter()
        .filter_map(|at| config.pointer(at).and_then(Value::as_str))
        .map(PathBuf::from)
        .collect())
}

/// The stage's artifact: the latest revision's result.
pub fn result(out: &Path) -> PathBuf {
    out.join("tuning").join("result.json")
}

/// The live rows the config asks for: every row of the compiled table whose
/// id the config names, or every row when it names none. Returns the rows
/// and the ids the table no longer has.
pub fn live_dials(asked: &Value) -> Result<(Vec<Dial>, Vec<String>), String> {
    let bytes = std::fs::read(table()).map_err(|e| format!("{}: {e}", table().display()))?;
    let table: Vec<Dial> = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let ids: Vec<String> = asked
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|d| d["id"].as_str().or(d.as_str()).map(str::to_string))
        .collect();
    if ids.is_empty() {
        return Ok((table, vec![]));
    }
    let gone = ids
        .iter()
        .filter(|id| !table.iter().any(|d| &&d.id == id))
        .cloned()
        .collect();
    let rows = table.into_iter().filter(|d| ids.contains(&d.id)).collect();
    Ok((rows, gone))
}

/// The overlay the next revision starts from: the last kept tree's, or the
/// Start overlay when no revision has kept one.
pub fn base(out: &Path) -> Result<Value, String> {
    let last = result(out);
    if last.exists() {
        let kept =
            read_json(&last).map_err(|e| e.to_string())?["outcome"]["current"]["overrides"].clone();
        if !kept.is_null() {
            return Ok(kept);
        }
    }
    let start = read_json(&super::start::file(out)).map_err(|e| e.to_string())?;
    Ok(start["overrides"].clone())
}

/// Runs the next revision and, when it ended with a tree, copies its result
/// to the stage's artifact.
pub fn run(template: &Path, tools: &Tools, out: &Path) -> Result<String, String> {
    let mut config = read_json(template).map_err(|e| e.to_string())?;
    let (dials, gone) = live_dials(&config["dials"])?;
    config["dials"] = json!(dials);
    config["initial_overrides"] = base(out)?;
    config["measure_binary"] = json!(tools.species_measure);
    config["matched"]["headless"] = json!(tools.headless);
    let revisions = out.join("tuning");
    let revision = next_revision(&revisions)?;
    let dir = revisions.join(revision.to_string());
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let config_path = dir.join("config.json");
    let bytes = serde_json::to_vec_pretty(&config).map_err(|e| e.to_string())?;
    std::fs::write(&config_path, bytes).map_err(|e| e.to_string())?;
    let ended = crate::tuning::command::run_with(&config_path, &dir, &UreqTransport, &|| {
        load_key().map_err(|e| e.to_string())
    });
    ended.map_err(|e| format!("revision {revision} failed: {e}"))?;
    let written = dir.join("result.json");
    let record = read_json(&dir.join("run.json")).map_err(|e| e.to_string())?;
    let outcome = read_json(&written).map_err(|e| e.to_string())?["outcome"].clone();
    let stopped = outcome["stopped"].as_str().unwrap_or("ended").to_string();
    if let Some(pending) = record["pending"].as_str() {
        return Err(format!(
            "revision {revision} was interrupted during {pending}: {stopped}"
        ));
    }
    if outcome["current"].is_null() {
        return Err(format!("revision {revision} kept no tree: {stopped}"));
    }
    std::fs::copy(&written, result(out)).map_err(|e| e.to_string())?;
    let mut word = format!("revision {revision}: {stopped}");
    if !gone.is_empty() {
        word.push_str(&format!(
            "; dials no longer in the table: {}",
            gone.join(", ")
        ));
    }
    Ok(word)
}

fn next_revision(dir: &Path) -> Result<u64, String> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Ok(1);
    };
    let mut last = 0;
    for entry in entries {
        let name = entry.map_err(|e| e.to_string())?.file_name();
        if let Ok(n) = name.to_string_lossy().parse::<u64>() {
            last = last.max(n);
        }
    }
    Ok(last + 1)
}
