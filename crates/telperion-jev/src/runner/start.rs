//! The Start stage: the sourced profile's values mapped onto dials (fn-135)
//! become the overlay the first tuning revision starts from.
//!
//! The tuning config's own `initial_overrides` are the person's entries and
//! win over a derived one. The stage writes `start.json` with the overlay and
//! a row per derived value naming its source and formula; run twice on the
//! same inputs it writes the same bytes.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{json, Map, Value};
use telperion_core::{params, presets::Preset};

use super::derive::{self, TABLE_JSON};
use crate::pipeline::canon::{canonical_sha256, read_json, write_canonical};

/// The derivation table this binary was built with, as a stage input.
pub fn table() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data/profile-to-preset.json")
}

/// The file the stage writes.
pub fn file(out: &Path) -> PathBuf {
    out.join("start.json")
}

/// A string field of the tuning config.
pub fn field(tuning: &Value, key: &str) -> Result<String, String> {
    tuning[key]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| format!("tuning config: no {key}"))
}

/// Derives the starting overlay and writes it; the word says what it did.
pub fn run(profile_path: &Path, tuning_path: &Path, out: &Path) -> Result<String, String> {
    let read = |p: &Path| read_json(p).map_err(|e| e.to_string());
    let tuning = read(tuning_path)?;
    let (preset_id, profile_id) = (field(&tuning, "preset")?, field(&tuning, "profile_id")?);
    let preset = Preset::from_id(&preset_id).ok_or(format!("unknown preset {preset_id}"))?;
    let packet = read(profile_path)?;
    let profile = packet["profiles"]
        .as_array()
        .and_then(|list| list.iter().find(|p| p["id"] == profile_id.as_str()))
        .ok_or(format!(
            "{}: no profile {profile_id}",
            profile_path.display()
        ))?;
    let manual = flatten(&tuning["initial_overrides"]);
    let family = params::overlay(&preset.parameters(), &unflatten(&manual))
        .map_err(|e| format!("manual overrides: {e:?}"))?;
    let derived = derive::derive(profile, &params::metadata(&family))?;
    let mut merged = manual.clone();
    for row in &derived.rows {
        merged.entry(row.path.clone()).or_insert(json!(row.value));
    }
    let overrides = unflatten(&merged);
    params::overlay(&preset.parameters(), &overrides)
        .map_err(|e| format!("derived overrides: {e:?}"))?;
    let contextual = reclassify(&tuning, &profile_id)?;
    let rows: Vec<Value> = derived
        .rows
        .iter()
        .map(|row| {
            let mut value = json!(row);
            if manual.contains_key(&row.path) {
                value["manual_wins"] = json!(true);
            }
            value
        })
        .collect();
    let record = json!({
        "schema": "runner-start", "schema_version": 1,
        "preset": preset_id, "overrides": overrides,
        "profile_sha256": canonical_sha256(profile),
        "table_sha256": crate::sha256_hex(TABLE_JSON.as_bytes()),
        "rows": rows, "skipped": derived.skipped,
        "manual": manual.keys().collect::<Vec<_>>(), "made_contextual": contextual,
    });
    write_canonical(&file(out), &record).map_err(|e| e.to_string())?;
    Ok(format!(
        "derived {} values ({} left out, {} manual)",
        derived.rows.len(),
        derived.skipped.len(),
        manual.len()
    ))
}

/// Makes the tuning profile's gating metrics the measurer cannot read
/// contextual, so the tree is never gated on a number nobody measures.
fn reclassify(tuning: &Value, profile_id: &str) -> Result<Vec<String>, String> {
    let Some(path) = tuning["profiles"].as_str().map(Path::new) else {
        return Ok(Vec::new());
    };
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut manifest = read_json(path).map_err(|e| e.to_string())?;
    let mut changed = Vec::new();
    for profile in manifest["profiles"].as_array_mut().into_iter().flatten() {
        if profile["id"] == profile_id {
            changed.extend(derive::measurable(profile));
        }
    }
    if !changed.is_empty() {
        let mut bytes = serde_json::to_vec_pretty(&manifest).expect("a Value serializes");
        bytes.push(b'\n');
        crate::pipeline::canon::write_atomic(path, &bytes).map_err(|e| e.to_string())?;
    }
    Ok(changed)
}

/// A partial wire object to its leaves, keyed by JSON pointer.
pub fn flatten(value: &Value) -> BTreeMap<String, Value> {
    fn walk(prefix: String, value: &Value, out: &mut BTreeMap<String, Value>) {
        match value.as_object() {
            Some(map) => {
                for (key, child) in map {
                    walk(format!("{prefix}/{key}"), child, out);
                }
            }
            None if !prefix.is_empty() => {
                out.insert(prefix, value.clone());
            }
            None => {}
        }
    }
    let mut out = BTreeMap::new();
    walk(String::new(), value, &mut out);
    out
}

pub fn unflatten(leaves: &BTreeMap<String, Value>) -> Value {
    let mut root = Map::new();
    for (path, value) in leaves {
        let keys: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        let (last, parents) = keys.split_last().expect("a pointer has a key");
        let mut node = &mut root;
        for key in parents {
            node = node
                .entry(key.to_string())
                .or_insert_with(|| json!({}))
                .as_object_mut()
                .expect("a wire group is an object");
        }
        node.insert(last.to_string(), value.clone());
    }
    Value::Object(root)
}
