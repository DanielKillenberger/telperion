//! The cells a revision must judge on a run from a name (host, 2026-09-26):
//! the reference photographs the Profile stage kept, never a hand-matched
//! reference a config names. A whole tree in leaf is required at the fixed
//! and a fresh seed; a bare tree and a bark close-up join when kept. Only a
//! reference with a selected shot is compared against.
use serde_json::{json, Value};

/// The seed a required cell draws beside the config's fixed one.
pub const FRESH_SEED: u32 = 42;

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
        match record["view"].as_str() {
            Some("leaf-on") => {
                cells.push(cell("crown-character", seed));
                cells.push(cell("crown-character", fresh));
                numeric.push(id.to_string());
            }
            Some("bare") => {
                cells.push(cell("branching-character", seed));
                numeric.push(id.to_string());
            }
            Some("bark") => cells.push(cell("bark-base", seed)),
            _ => {}
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
    /// selected shot, and one of another view, add none.
    #[test]
    fn the_kept_views_are_the_required_cells() {
        let shot = json!({"camera": {}});
        let references = json!({"references": [
            {"id": "photo-1", "view": "leaf-on", "shot": shot},
            {"id": "photo-2", "view": "bark", "shot": shot},
            {"id": "photo-3", "view": "bare"},
            {"id": "photo-4", "view": "other", "shot": shot},
        ]});
        let (cells, numeric) = derive(&references, 1).unwrap();
        assert_eq!(
            cells,
            [
                json!({"item": "crown-character", "view": "photo-1", "seed": 1}),
                json!({"item": "crown-character", "view": "photo-1", "seed": 42}),
                json!({"item": "bark-base", "view": "photo-2", "seed": 1}),
            ]
        );
        assert_eq!(numeric, ["photo-1"]);
        let bare = json!({"references": [{"id": "photo-3", "view": "bare", "shot": shot}]});
        let err = derive(&bare, 1).unwrap_err();
        assert!(err.contains("whole tree in leaf"), "{err}");
    }
}
