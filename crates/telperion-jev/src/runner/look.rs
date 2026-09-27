//! `species <id> --look`: the tree Tune kept, written where the harness's dev
//! server serves it, and the URL that opens it. The look carries the overlay
//! and the families core makes of it, the preset alone and the preset with
//! the overlay laid over it through `params::overlay`, the call
//! `headless --family` makes, so the harness draws core's tree and checks its
//! own overlay against it. Accepting stays `--accept`.
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::pipeline::canon::{read_json, write_canonical};
use telperion_core::{catalogue, params, presets::Preset};

/// Where the dev server serves looks from, under the repository root; the
/// directory is ignored.
pub const DIR: &str = "harness/looks";

/// The dev server's address, as `npm run dev` serves it.
pub const SERVER: &str = "http://localhost:5173";

/// A look on disk and the harness URL that opens it.
pub struct Written {
    pub path: PathBuf,
    pub url: String,
}

/// Writes the kept tree of `result` (a run's `runner/tuning/result.json`)
/// to `<looks>/<species>.json`. Refused, naming the file, when the run kept
/// no tree; refused, naming the path, when the overlay names a row the
/// preset's family does not have.
pub fn write(species: &str, result: &Path, looks: &Path) -> Result<Written, String> {
    let at = result.display();
    if !result.exists() {
        return Err(format!(
            "no kept tree: {at} does not exist; Tune has not kept one"
        ));
    }
    let tuned = read_json(result).map_err(|e| e.to_string())?;
    let outcome = &tuned["outcome"];
    let current = &outcome["current"];
    if current.is_null() {
        return Err(format!("no kept tree: {at} has no outcome.current"));
    }
    let base = outcome["preset"]
        .as_str()
        .ok_or(format!("{at}: outcome.preset names no preset"))?;
    let preset = Preset::from_id(base)
        .ok_or(format!("{at}: unknown preset {base}"))?
        .parameters();
    let overrides = &current["overrides"];
    let wire = params::metadata(&preset);
    if let Some(path) = unknown(&wire, overrides, "") {
        let why = catalogue::retired(&path).map_or(String::new(), |w| format!(" ({w})"));
        return Err(format!(
            "{at}: the overlay path {path} is not a row of the {base} family{why}"
        ));
    }
    let kept = params::overlay(&preset, overrides).map_err(|e| format!("{at}: {e:?}"))?;
    let seed = outcome["seed"]
        .as_u64()
        .unwrap_or(u64::from(kept.skeleton.seed));
    let look = json!({
        "schema": "harness-look", "schema_version": 1,
        "species": species, "preset": base, "seed": seed,
        "source": {
            "run": run_dir(result).map(|p| p.display().to_string()),
            "revision": revision(result)?,
            "round": current["round"], "label": current["label"],
            "kept": outcome["adoptions_kept"], "key": current["key"],
        },
        "overrides": overrides,
        "preset_family": wire,
        "family": params::metadata(&kept),
    });
    let path = looks.join(format!("{species}.json"));
    std::fs::create_dir_all(looks).map_err(|e| format!("{}: {e}", looks.display()))?;
    write_canonical(&path, &look).map_err(|e| e.to_string())?;
    let url = format!("{SERVER}/?look={species}&seed={seed}");
    Ok(Written { path, url })
}

/// The first path `over` names that `wire` has not, where an object meets a
/// value or a key is absent.
pub fn unknown(wire: &Value, over: &Value, at: &str) -> Option<String> {
    let Some(map) = over.as_object() else {
        return (!at.is_empty() && wire.is_object()).then(|| at.to_string());
    };
    for (key, value) in map {
        let path = format!("{at}/{key}");
        match wire.get(key) {
            None => return Some(path),
            Some(slot) if value.is_object() && !slot.is_object() => return Some(path),
            Some(slot) => {
                if let Some(found) = unknown(slot, value, &path) {
                    return Some(found);
                }
            }
        }
    }
    None
}

/// The run directory of `<run-dir>/runner/tuning/result.json`.
fn run_dir(result: &Path) -> Option<&Path> {
    result.parent()?.parent()?.parent()
}

/// The revision whose result the stage's artifact is: the highest numbered
/// revision directory beside it holding the same bytes. None when none does.
fn revision(result: &Path) -> Result<Option<u64>, String> {
    let bytes = std::fs::read(result).map_err(|e| format!("{}: {e}", result.display()))?;
    let Some(dir) = result.parent() else {
        return Ok(None);
    };
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Ok(None);
    };
    let mut found = None;
    for entry in entries.flatten() {
        let Ok(n) = entry.file_name().to_string_lossy().parse::<u64>() else {
            continue;
        };
        let same = std::fs::read(entry.path().join("result.json")).is_ok_and(|b| b == bytes);
        if same && found.is_none_or(|f| n > f) {
            found = Some(n);
        }
    }
    Ok(found)
}
