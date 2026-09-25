//! The Accept stage: the owner looks at the tuned tree in the harness, and
//! `--accept` records the tree they looked at as a value table, one wire
//! pointer to one value, in `accepted.json`. An acceptance names the tree's
//! key, so a later revision's tree waits for a look of its own, and it is
//! refused while the species' catalogue folder fails
//! `scripts/catalogue-check.mjs`: the palm shipped without its entry.
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{json, Value};

use super::start::flatten;
use crate::pipeline::canon::{read_json, write_canonical};

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

/// Writes the acceptance of the current tree, once the catalogue entry holds.
pub fn run(root: &Path, species: &str, result: &Path, out: &Path) -> Result<String, String> {
    let failures = catalogue_failures(root, species)?;
    if !failures.is_empty() {
        return Err(format!(
            "the catalogue entry fails its check: {}",
            failures.join("; ")
        ));
    }
    let now = current(result)?;
    let tree = &now["tree"];
    let table = flatten(&tree["overrides"]);
    let record = json!({
        "schema": "runner-accepted", "schema_version": 1,
        "preset": now["preset"], "key": tree["key"],
        "values": table, "stills": tree["stills"],
    });
    write_canonical(&file(out), &record).map_err(|e| e.to_string())?;
    Ok(format!("accepted {} ({} values)", tree["key"], table.len()))
}
