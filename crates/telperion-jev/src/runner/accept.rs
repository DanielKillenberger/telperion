//! The Accept stage: the owner looks at the tuned tree in the harness, and
//! `--accept` records the tree they looked at as a value table, one wire
//! pointer to one value, in `accepted.json`, writes the tree into core as the
//! species' preset and refreshes its pins. An acceptance names the tree's
//! key, so a later revision's tree waits for a look of its own, and it is
//! refused while the species' catalogue folder fails
//! `scripts/catalogue-check.mjs`: the palm shipped without its entry.
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{json, Value};

use super::preset::{self, Names};
use super::{catalogue, pins, start::flatten};
use crate::pipeline::canon::{read_json, write_canonical};
use telperion_core::{params, presets::Preset, Family};

pub fn file(out: &Path) -> PathBuf {
    out.join("accepted.json")
}

fn current(result: &Path) -> Result<Value, String> {
    let value = read_json(result).map_err(|e| e.to_string())?;
    let tree = value["outcome"]["current"].clone();
    match tree.is_null() {
        true => Err("the tuning result has no current tree to accept".into()),
        false => Ok(json!({"preset": value["outcome"]["preset"], "tree": tree})),
    }
}

/// True when the tree on disk is the one the owner accepted.
pub fn accepted(result: &Path, out: &Path) -> Result<bool, String> {
    let path = file(out);
    if !path.exists() || !result.exists() {
        return Ok(false);
    }
    let record = read_json(&path).map_err(|e| e.to_string())?;
    Ok(record["key"] == current(result)?["tree"]["key"])
}

/// The catalogue check's failures that name this species' folder, run from
/// the repository `root`. A check that did not run is an error, never a pass.
pub fn catalogue_failures(root: &Path, species: &str) -> Result<Vec<String>, String> {
    let output = Command::new("node")
        .current_dir(root)
        .arg("scripts/catalogue-check.mjs")
        .output()
        .map_err(|e| format!("node scripts/catalogue-check.mjs: {e}"))?;
    if output.status.success() {
        return Ok(vec![]);
    }
    let printed = String::from_utf8_lossy(&output.stderr);
    let counted = printed
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .is_some_and(|l| l.starts_with("catalogue: ") && l.contains("failure"));
    if !counted {
        return Err(format!(
            "the catalogue check did not run: {}",
            printed.trim()
        ));
    }
    let folder = format!("catalogue/{species}");
    Ok(printed
        .lines()
        .filter(|l| l.starts_with(&format!("{folder}:")) || l.starts_with(&format!("{folder}/")))
        .map(str::to_string)
        .collect())
}

/// The family the owner accepted: the tuning base with the tree's overlay.
pub fn family(result: &Path) -> Result<Family, String> {
    let now = current(result)?;
    let base = now["preset"]
        .as_str()
        .ok_or("the tuning result names no preset")?;
    let preset = Preset::from_id(base).ok_or(format!("unknown preset {base}"))?;
    params::overlay(&preset.parameters(), &now["tree"]["overrides"]).map_err(|e| format!("{e:?}"))
}

/// Accepts the current tree: its pins into the species' catalogue folder,
/// the folder through its check, the tree into core as the species' preset,
/// then the record. A refusal leaves the preset untouched.
pub fn run(
    root: &Path,
    names: &Names,
    folder: &Path,
    result: &Path,
    out: &Path,
) -> Result<String, String> {
    let accepted = family(result)?;
    pins::write(folder, &names.id, &accepted)?;
    let now = current(result)?;
    let shown: Vec<Value> = now["tree"]["stills"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|s| json!({"path": s["path"], "seed": s["seed"], "view": s["view"], "visual_status": "accepted"}))
        .collect();
    catalogue::stills(folder, &names.id, &names.id, &shown)?;
    catalogue::pages(root)?;
    let failures = catalogue_failures(root, &names.id)?;
    if !failures.is_empty() {
        return Err(format!(
            "the catalogue entry fails its check: {}",
            failures.join("; ")
        ));
    }
    let tree = &now["tree"];
    let note = format!(
        "Accepted by the owner on {} as tuning tree {} (species runner, fn-149).",
        &crate::pipeline::stage::now()[..10],
        tree["key"].as_str().unwrap_or_default()
    );
    let written = preset::write(root, names, &accepted, &note)?;
    let table = flatten(&tree["overrides"]);
    let record = json!({
        "schema": "runner-accepted", "schema_version": 1,
        "preset": now["preset"], "key": tree["key"],
        "values": table, "stills": tree["stills"],
    });
    write_canonical(&file(out), &record).map_err(|e| e.to_string())?;
    Ok(format!(
        "accepted {}: {written}; pins refreshed",
        tree["key"]
    ))
}
