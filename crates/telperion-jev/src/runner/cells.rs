//! The cells a revision must judge on a run from a name (host, 2026-09-26):
//! the reference photographs the Profile stage kept, never a hand-matched
//! reference a config names. A whole tree in leaf is required at the fixed
//! and a fresh seed; a bare tree and a bark close-up join when kept. Only a
//! reference with a selected shot is compared against.
use serde_json::{json, Value};

/// The seed a required cell draws beside the config's fixed one.
pub const FRESH_SEED: u32 = 42;

/// A reference's view: its own `view`, or for a curated record that states
/// none, the first of its scales that names one - a whole tree in leaf, a
/// bare tree, or the bark at its base.
pub fn view(record: &Value) -> Option<&str> {
    if let Some(view) = record["view"].as_str() {
        return Some(view);
    }
    let leaf = record["shot"]["foliage"] == "leaf-on";
    let mut scales = record["scale"].as_array()?.iter().filter_map(Value::as_str);
    scales.find_map(|scale| match scale {
        "whole" if leaf => Some("leaf-on"),
        "bare" => Some("bare"),
        "base" => Some("bark"),
        _ => None,
    })
}

/// The required cells and the numeric references drawn from the recorded
/// references, or an error naming what is missing.
pub fn derive(references: &Value, seed: u32) -> Result<(Vec<Value>, Vec<String>), String> {
    let fresh = if seed == FRESH_SEED {
        FRESH_SEED + 1
    } else {
        FRESH_SEED
    };
    let (mut cells, mut numeric) = (Vec::new(), Vec::new());
    let shot = references["references"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|r| r["shot"].is_object());
    for record in shot {
        let Some(id) = record["id"].as_str() else {
            continue;
        };
        let cell = |item: &str, seed: u32| json!({"item": item, "view": id, "seed": seed});
        let view = view(record).unwrap_or_default();
        match view {
            "leaf-on" => {
                cells.push(cell("crown-character", seed));
                cells.push(cell("crown-character", fresh));
            }
            "bare" => cells.push(cell("branching-character", seed)),
            "bark" => cells.push(cell("bark-base", seed)),
            _ => continue,
        }
        // A photograph whose box or crown base matched no candidate is
        // judged by eye but sets no numeric target.
        if view != "bark" && record["shot"]["tree"].is_object() {
            numeric.push(id.to_string());
        }
    }
    if !cells.iter().any(|c| c["item"] == "crown-character") {
        return Err("no kept whole tree in leaf has a selected shot".into());
    }
    Ok((cells, numeric))
}

/// Fills a revision config's required cells and numeric references from
/// its recorded references when it names no required cell.
pub fn fill(config: &mut Value) -> Result<(), String> {
    if config["required"].as_array().is_some_and(|c| !c.is_empty()) {
        return Ok(());
    }
    let path = config["matched"]["references"].as_str().unwrap_or_default();
    let references = crate::pipeline::canon::read_json(std::path::Path::new(path))
        .map_err(|e| format!("{path}: {e}"))?;
    let seed = config["seed"].as_u64().unwrap_or(1) as u32;
    let (cells, numeric) = derive(&references, seed)?;
    config["required"] = json!(cells);
    config["matched"]["numeric_references"] = json!(numeric);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The kept views become the required cells; a reference with no
    /// selected shot, and one of another view, add none; one whose box or
    /// crown base matched nothing is judged but sets no numeric target.
    #[test]
    fn the_kept_views_are_the_required_cells() {
        let shot = json!({"camera": {}, "tree": {"box": [0, 0, 1, 1], "crownBase": 0.1}});
        let references = json!({"references": [
            {"id": "photo-1", "view": "leaf-on", "shot": shot},
            {"id": "photo-2", "view": "bark", "shot": shot},
            {"id": "photo-3", "view": "bare"},
            {"id": "photo-4", "view": "other", "shot": shot},
            {"id": "photo-5", "view": "leaf-on", "shot": {"camera": {}, "tree": null}},
        ]});
        let (cells, numeric) = derive(&references, 1).unwrap();
        assert_eq!(
            cells,
            [
                json!({"item": "crown-character", "view": "photo-1", "seed": 1}),
                json!({"item": "crown-character", "view": "photo-1", "seed": 42}),
                json!({"item": "bark-base", "view": "photo-2", "seed": 1}),
                json!({"item": "crown-character", "view": "photo-5", "seed": 1}),
                json!({"item": "crown-character", "view": "photo-5", "seed": 42}),
            ]
        );
        assert_eq!(numeric, ["photo-1"]);
        let bare = json!({"references": [{"id": "photo-3", "view": "bare", "shot": shot}]});
        let err = derive(&bare, 1).unwrap_err();
        assert!(err.contains("whole tree in leaf"), "{err}");
    }

    /// fn-179: the beech's curated records carry their scale and no view.
    /// Read as the catalogue stores them, they give the cells fn-62's run
    /// had to patch in by hand: B-WHOLE in leaf, B-BARE bare, B-BASE bark.
    #[test]
    fn a_curated_record_takes_its_view_from_its_scale() {
        let beech: Value = serde_json::from_str(include_str!(
            "../../../../catalogue/european-beech/packet/references.json"
        ))
        .unwrap();
        let (cells, numeric) = derive(&beech, 1).unwrap();
        assert_eq!(
            cells,
            [
                json!({"item": "crown-character", "view": "B-WHOLE", "seed": 1}),
                json!({"item": "crown-character", "view": "B-WHOLE", "seed": 42}),
                json!({"item": "branching-character", "view": "B-BARE", "seed": 1}),
                json!({"item": "bark-base", "view": "B-BASE", "seed": 1}),
            ]
        );
        assert_eq!(numeric, ["B-WHOLE", "B-BARE"]);
    }
}
