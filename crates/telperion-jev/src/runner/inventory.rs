//! The reference photographs the reviewer compares every render against,
//! and their inventory, built by the Profile stage once the references are
//! chosen: one paid look (`tuning::inventory::run`). The photographs are the
//! tuning config's own when it lists any, else the ones the Profile stage
//! found (`pipeline::photos`), written to `runner/references.json`. The
//! inventory lives under `runner/inventory/<hash of the references>/`, so the
//! same photographs are looked at once and a changed set gets a look of its
//! own.
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::pipeline::canon::{read_json, write_canonical};
use crate::pipeline::photos;
use crate::pipeline::stage::Paths;
use crate::tuning::live::Config;

/// The photographs the Profile stage found, as the reviewer's images.
pub fn found(out: &Path) -> PathBuf {
    out.join("references.json")
}

/// Writes the recorded references the run holds a copy of as images.
pub fn record(paths: &Paths, out: &Path) -> Result<(), String> {
    let doc = read_json(&paths.packet("references")).unwrap_or_default();
    let images: Vec<Value> = doc["references"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|r| {
            let sha256 = r["asset_sha256"].as_str()?;
            let path = photos::copy(paths, sha256)?;
            let view = r["scale"][0].as_str().unwrap_or("whole");
            Some(json!({"path": path, "sha256": sha256, "view": view, "seed": 0}))
        })
        .collect();
    write_canonical(&found(out), &json!(images))
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// The references a revision compares against: the config's, else found.
pub fn references(template: &Path, out: &Path) -> Result<Value, String> {
    let config = read_json(template).map_err(|e| e.to_string())?;
    let listed = config["references"]
        .as_array()
        .is_some_and(|l| !l.is_empty());
    Ok(match listed {
        true => config["references"].clone(),
        false => read_json(&found(out)).unwrap_or_else(|_| json!([])),
    })
}

/// Where the inventory of the references lives.
pub fn dir(template: &Path, out: &Path) -> Result<PathBuf, String> {
    let key = crate::sha256_hex(&serde_json::to_vec(&references(template, out)?).unwrap());
    Ok(out.join("inventory").join(&key[..16]))
}

/// Whether the run has no reference photograph to compare against.
pub fn none(template: &Path, out: &Path) -> Result<bool, String> {
    Ok(references(template, out)?
        .as_array()
        .is_none_or(Vec::is_empty))
}

/// Builds the inventory unless these references already have one, and
/// skips it when there is no photograph (`gaps::note_references`).
pub fn build(template: &Path, out: &Path) -> Result<String, String> {
    if none(template, out)? {
        return Ok("no reference photographs: the inventory skipped".into());
    }
    let dir = dir(template, out)?;
    if dir.join("inventory.json").exists() {
        return Ok("reference inventory current".into());
    }
    let mut value = read_json(template).map_err(|e| e.to_string())?;
    if value["initial_overrides"].is_null() {
        value["initial_overrides"] = json!({});
    }
    value["references"] = references(template, out)?;
    value.as_object_mut().map(|o| o.remove("reference_first"));
    let config: Config =
        serde_json::from_value(value).map_err(|e| format!("tuning config: {e}"))?;
    crate::tuning::inventory::run(&config, &dir)?;
    Ok("reference inventory built".into())
}

/// The inventory as a revision's `reference_first` pins.
pub fn pins(template: &Path, out: &Path) -> Result<Value, String> {
    let dir = dir(template, out)?;
    let pin = |name: &str| -> Result<Value, String> {
        let path = dir.join(name);
        let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        Ok(json!({"path": path, "sha256": crate::sha256_hex(&bytes)}))
    };
    Ok(json!({"inventory": pin("inventory.json")?, "preparation": pin("preparation.json")?}))
}

/// The inventory's files and the found references, as inputs to what reads
/// them.
pub fn files(template: &Path, out: &Path) -> Result<Vec<PathBuf>, String> {
    if none(template, out)? {
        return Ok(vec![found(out)]);
    }
    let dir = dir(template, out)?;
    Ok(vec![
        dir.join("inventory.json"),
        dir.join("preparation.json"),
        found(out),
    ])
}
