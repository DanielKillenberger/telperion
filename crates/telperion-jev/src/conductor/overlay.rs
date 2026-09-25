//! The conductor's half of fn-135: before a tuning revision starts, the
//! sourced profile's derived values become the tuning config's
//! `initial_overrides`, with a provenance file beside the config, and the
//! tuning profile stops gating on metrics the measurer cannot read.
//!
//! A manual entry wins over a derived one. An entry counts as derived when
//! the last provenance recorded it at that value; every other entry, and
//! every entry a provenance already named manual, is the person's. Run twice
//! on the same inputs, it writes the same bytes.
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_json::{json, Map, Value};
use telperion_core::{params, presets::Preset};

use super::derive::{self, Derived, TABLE_JSON};
use super::{ConductorError, Config, Result};
use crate::pipeline::canon::{canonical_sha256, read_json, write_atomic};

/// The provenance file beside the tuning config: `tuning.json` has
/// `tuning.derived.json`.
pub fn provenance_file(config: &Config) -> PathBuf {
    let stem = config
        .tuning_config
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    config
        .tuning_config
        .with_file_name(format!("{stem}.derived.json"))
}

fn invalid(msg: String) -> ConductorError {
    ConductorError::Invalid(msg)
}

/// Writes the derived overlay and its provenance and says what it derived.
/// A species folder with no sourced profile has nothing to derive: `None`,
/// and nothing changes.
pub fn refresh(config: &Config) -> Result<Option<String>> {
    let profile_path = config.paths().packet("profile");
    if !profile_path.exists() {
        return Ok(None);
    }
    let mut tuning = read_json(&config.tuning_config)?;
    let field = |key: &str| {
        tuning[key]
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| invalid(format!("{}: no {key}", config.tuning_config.display())))
    };
    let (preset_id, profile_id) = (field("preset")?, field("profile_id")?);
    let preset = Preset::from_id(&preset_id)
        .ok_or_else(|| invalid(format!("unknown preset {preset_id}")))?;
    let packet = read_json(&profile_path)?;
    let profile = find(&packet, &profile_id).ok_or_else(|| {
        invalid(format!(
            "{}: no profile {profile_id}",
            profile_path.display()
        ))
    })?;
    let provenance_path = provenance_file(config);
    let prior = match provenance_path.exists() {
        true => read_json(&provenance_path)?,
        false => json!({}),
    };
    let manual = manual(&flatten(&tuning["initial_overrides"]), &prior);
    let family = params::overlay(&preset.parameters(), &unflatten(&manual))
        .map_err(|err| invalid(format!("manual overrides: {err:?}")))?;
    let derived = derive::derive(profile, &params::metadata(&family))?;
    let mut merged = manual.clone();
    for row in &derived.rows {
        merged.entry(row.path.clone()).or_insert(json!(row.value));
    }
    let overrides = unflatten(&merged);
    params::overlay(&preset.parameters(), &overrides)
        .map_err(|err| invalid(format!("derived overrides: {err:?}")))?;
    tuning["initial_overrides"] = overrides;
    write_if_changed(&config.tuning_config, &tuning)?;
    let mut contextual = strings(&prior["made_contextual"]);
    contextual.extend(reclassify(&tuning, &profile_id)?);
    let provenance = json!({
        "schema_version": 1,
        "meaning": "fn-135: the values the sourced profile derived for the tuning config's initial_overrides; a manual entry wins over a derived one",
        "profile": profile_path, "profile_sha256": canonical_sha256(profile),
        "table_sha256": crate::sha256_hex(TABLE_JSON.as_bytes()),
        "preset": preset_id, "rows": rows(&derived, &manual), "skipped": derived.skipped,
        "manual": manual.keys().collect::<Vec<_>>(), "made_contextual": contextual,
    });
    write_if_changed(&provenance_path, &provenance)?;
    Ok(Some(format!(
        "profile derived {} values ({} left out, {} manual)",
        derived.rows.len(),
        derived.skipped.len(),
        manual.len()
    )))
}

fn find<'a>(manifest: &'a Value, id: &str) -> Option<&'a Value> {
    manifest["profiles"]
        .as_array()?
        .iter()
        .find(|p| p["id"] == id)
}

fn strings(value: &Value) -> BTreeSet<String> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|s| s.as_str().map(str::to_string))
        .collect()
}

/// The person's entries: those the last provenance named manual, and those
/// it never derived at the value they now hold.
fn manual(current: &BTreeMap<String, Value>, prior: &Value) -> BTreeMap<String, Value> {
    let named = strings(&prior["manual"]);
    let derived: BTreeMap<&str, &Value> = prior["rows"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|r| Some((r["path"].as_str()?, &r["value"])))
        .collect();
    current
        .iter()
        .filter(|(path, value)| named.contains(*path) || derived.get(path.as_str()) != Some(value))
        .map(|(path, value)| (path.clone(), value.clone()))
        .collect()
}

fn rows(derived: &Derived, manual: &BTreeMap<String, Value>) -> Vec<Value> {
    derived
        .rows
        .iter()
        .map(|row| {
            let mut value = json!(row);
            if manual.contains_key(&row.path) {
                value["manual_wins"] = json!(true);
            }
            value
        })
        .collect()
}

/// Makes the tuning profile's unreadable gating metrics contextual.
fn reclassify(tuning: &Value, profile_id: &str) -> Result<Vec<String>> {
    let Some(path) = tuning["profiles"].as_str().map(Path::new) else {
        return Ok(Vec::new());
    };
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut manifest = read_json(path)?;
    let mut changed = Vec::new();
    for profile in manifest["profiles"].as_array_mut().into_iter().flatten() {
        if profile["id"] == profile_id {
            changed.extend(derive::measurable(profile));
        }
    }
    if !changed.is_empty() {
        write_if_changed(path, &manifest)?;
    }
    Ok(changed)
}

fn write_if_changed(path: &Path, value: &Value) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value).expect("a Value serializes");
    bytes.push(b'\n');
    if std::fs::read(path).ok().as_deref() != Some(bytes.as_slice()) {
        write_atomic(path, &bytes)?;
    }
    Ok(())
}

/// A partial wire object to its leaves, keyed by JSON pointer.
fn flatten(value: &Value) -> BTreeMap<String, Value> {
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

fn unflatten(leaves: &BTreeMap<String, Value>) -> Value {
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
