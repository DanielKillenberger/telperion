//! The reference inventory the reviewer compares every render against, built
//! by the Profile stage once the references are chosen: one paid look over
//! the tuning config's reference photographs (`tuning::inventory::run`). It
//! lives under `runner/inventory/<hash of the references>/`, so the same
//! photographs are looked at once and a changed set gets a look of its own.
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::pipeline::canon::read_json;
use crate::tuning::live::Config;

/// Where the inventory of the config's references lives.
pub fn dir(template: &Path, out: &Path) -> Result<PathBuf, String> {
    let config = read_json(template).map_err(|e| e.to_string())?;
    let key = crate::sha256_hex(&serde_json::to_vec(&config["references"]).unwrap());
    Ok(out.join("inventory").join(&key[..16]))
}

/// Builds the inventory unless these references already have one.
pub fn build(template: &Path, out: &Path) -> Result<String, String> {
    let dir = dir(template, out)?;
    if dir.join("inventory.json").exists() {
        return Ok("reference inventory current".into());
    }
    let mut value = read_json(template).map_err(|e| e.to_string())?;
    if value["initial_overrides"].is_null() {
        value["initial_overrides"] = json!({});
    }
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

/// The inventory's files, as inputs to what reads them.
pub fn files(template: &Path, out: &Path) -> Result<Vec<PathBuf>, String> {
    let dir = dir(template, out)?;
    Ok(vec![
        dir.join("inventory.json"),
        dir.join("preparation.json"),
    ])
}
