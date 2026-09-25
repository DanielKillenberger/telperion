//! The Tune stage: one tuning revision over the live dials.
//!
//! A revision starts from the last kept tree, or from the Start overlay on
//! the first. Its dials are the rows of the dial table compiled into this
//! binary, never a copy frozen into the config, so a row the generator gained
//! or lost is offered or dropped on the next revision. The revision ends when
//! its rounds stop keeping anything; whatever stopped it is recorded in its
//! result, and its result is the stage's artifact.
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::caller::{load_key, UreqTransport};
use crate::pipeline::canon::read_json;
use crate::tuning::actions::Dial;

const DIALS_JSON: &str = include_str!("../../data/dials.json");

/// The stage's artifact: the latest revision's result.
pub fn result(out: &Path) -> PathBuf {
    out.join("tuning").join("result.json")
}

/// The live rows the config asks for: every row of the compiled table whose
/// id the config names, or every row when it names none. Returns the rows
/// and the ids the table no longer has.
pub fn live_dials(asked: &Value) -> Result<(Vec<Dial>, Vec<String>), String> {
    let table: Vec<Dial> = serde_json::from_str(DIALS_JSON).map_err(|e| e.to_string())?;
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

/// Runs the next revision and copies its result to the stage's artifact.
pub fn run(template: &Path, out: &Path) -> Result<String, String> {
    let mut config = read_json(template).map_err(|e| e.to_string())?;
    let (dials, gone) = live_dials(&config["dials"])?;
    config["dials"] = json!(dials);
    config["initial_overrides"] = base(out)?;
    let revisions = out.join("tuning");
    let revision = next_revision(&revisions)?;
    let dir = revisions.join(revision.to_string());
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let config_path = dir.join("config.json");
    let bytes = serde_json::to_vec_pretty(&config).map_err(|e| e.to_string())?;
    std::fs::write(&config_path, bytes).map_err(|e| e.to_string())?;
    let ended = crate::tuning::command::run_with(&config_path, &dir, None, &UreqTransport, &|| {
        load_key().map_err(|e| e.to_string())
    });
    let written = dir.join("result.json");
    if !written.exists() {
        return Err(format!(
            "revision {revision} wrote no result: {}",
            ended.err().unwrap_or_default()
        ));
    }
    std::fs::copy(&written, result(out)).map_err(|e| e.to_string())?;
    let stopped = read_json(&written).map_err(|e| e.to_string())?["outcome"]["stopped"]
        .as_str()
        .unwrap_or("ended")
        .to_string();
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
