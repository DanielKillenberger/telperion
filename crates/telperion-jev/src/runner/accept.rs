//! The Accept stage: the owner looks at the tuned tree in the harness, and
//! `--accept` records the tree they looked at as a value table, one wire
//! pointer to one value, in `accepted.json`. An acceptance names the tree's
//! key, so a later revision's tree waits for a look of its own.
use std::path::{Path, PathBuf};

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

/// Writes the acceptance of the current tree.
pub fn run(result: &Path, out: &Path) -> Result<String, String> {
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
