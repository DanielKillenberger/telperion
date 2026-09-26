//! The packet records and the sidecar the generate stage writes: closed
//! shapes that gain no key, and the shipped values laid over the preset.

use std::path::Path;

use serde_json::{json, Value};

use crate::pipeline::canon::{file_sha256, read_json, write_canonical};
use crate::pipeline::manifest::Manifest;
use crate::pipeline::render::family_for;
use crate::pipeline::stage::{Context, Paths, StageError};

use super::{failed, Shipped};

pub(super) fn pointer_for(dial: &str) -> String {
    format!("/parameters/{}", dial.replace('.', "/"))
}

/// The measured preset's parameters, when the example's `run` event carries
/// them. No example binary means no parameters, never invented ones.
pub(super) fn run_parameters(receipt: &Path) -> Option<Value> {
    let text = std::fs::read_to_string(receipt).ok()?;
    text.lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .find(|event| event["event"] == "run")
        .and_then(|event| event.get("parameters").cloned())
        .filter(Value::is_object)
}

/// A leaf in the overlay replaces the one below it; every other row stands.
fn merge(into: &mut Value, from: &Value) {
    match (into.as_object_mut(), from.as_object()) {
        (Some(target), Some(source)) => {
            for (key, value) in source {
                merge(target.entry(key.clone()).or_insert(Value::Null), value);
            }
        }
        _ => *into = from.clone(),
    }
}

/// The preset's parameters with every shipped value laid over them.
pub(super) fn overlaid(mut parameters: Value, shipped: &Shipped) -> Value {
    for (dial, value) in &shipped.values {
        merge(&mut parameters, &family_for(dial, *value));
    }
    parameters
}

pub(super) fn write_sidecar(ctx: &Context, shipped: &Shipped) -> Result<(), StageError> {
    let path = ctx.paths.sidecar();
    let empty = json!({"schema": "provenance", "schema_version": 1, "entries": {}});
    let mut sidecar = if path.exists() {
        read_json(&path)?
    } else {
        empty
    };
    let entries = sidecar["entries"]
        .as_object_mut()
        .ok_or_else(|| failed("the sidecar has no entries object"))?;
    for (pointer, entry) in &shipped.entries {
        entries.insert(pointer.clone(), entry.clone());
    }
    write_canonical(&path, &sidecar)?;
    Ok(())
}

/// The packet's `species.json` and `specimens.json`, closed shapes that gain
/// no key. The seed audit and the expert's eye are still outstanding.
pub(super) fn write_packet(
    paths: &Paths,
    manifest: &Manifest,
    parameters: Value,
) -> Result<(), StageError> {
    let fixed = [manifest.seed, manifest.seed + 1, manifest.seed + 2];
    let holdout = draw_holdout_seeds(&manifest.species, &fixed);
    let taxon = &manifest.taxon;
    write_canonical(
        &paths.packet("species"),
        &json!({
            "id": manifest.species, "scientific_name": taxon.scientific_name,
            "taxon_rank": taxon.rank, "cultivar": taxon.cultivar, "context": manifest.context,
            "profile_id": manifest.profile_id, "profile_path": "packet/profile.json",
            "profile_sha256": file_sha256(&paths.packet("profile")).unwrap_or_default(),
            "preset": manifest.preset, "required_capabilities": [], "parameters": parameters,
            "fixed_seeds": fixed, "holdout_seeds": holdout, "reference_ids": [],
        }),
    )?;
    let case = |seed: u32, role: &str| {
        json!({"id": format!("{}-{seed}", manifest.species), "species_id": manifest.species,
               "seed": seed, "seed_role": role,
               "parameter_species_id": manifest.species})
    };
    let cases: Vec<Value> = fixed
        .iter()
        .map(|seed| case(*seed, "regression"))
        .chain(holdout.iter().map(|seed| case(*seed, "holdout")))
        .collect();
    write_canonical(
        &paths.packet("specimens"),
        &json!({
            "schema": "specimens", "schema_version": 1,
            "benchmark_id": "fn19-v1", "cases": cases, "seed_evidence": "unaudited",
            "fresh_for_future_tuning": false, "receipts": [],
            "generation_status": "measured-by-pipeline", "expert_status": "unassessed",
        }),
    )?;
    Ok(())
}

/// Three seeds drawn deterministically from the species id, distinct from
/// the fixed seeds and from each other, so a rerun keeps the same holdouts
/// and nobody chooses them by looking at a render.
fn draw_holdout_seeds(species: &str, fixed: &[u32; 3]) -> [u32; 3] {
    let mut drawn: Vec<u32> = Vec::with_capacity(3);
    let mut salt: u32 = 0;
    while drawn.len() < 3 {
        let digest = crate::sha256_hex(format!("{species}:holdout:{salt}").as_bytes());
        let seed = u32::from_str_radix(&digest[..8], 16).expect("a hex digest parses");
        salt += 1;
        if fixed.contains(&seed) || drawn.contains(&seed) {
            continue;
        }
        drawn.push(seed);
    }
    [drawn[0], drawn[1], drawn[2]]
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::pipeline::manifest::tests::minimal;

    fn scratch() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "jev-generate-packet-{}-{}",
            std::process::id(),
            crate::ledger::new_entry_id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn manifest() -> Manifest {
        serde_json::from_value(minimal()).unwrap()
    }

    #[test]
    fn write_packet_draws_three_holdout_seeds_distinct_from_the_fixed_ones() {
        let dir = scratch();
        let paths = Paths::new(&dir);
        let manifest = manifest();
        write_packet(&paths, &manifest, json!({})).unwrap();

        let species: Value = read_json(&paths.packet("species")).unwrap();
        let fixed = [manifest.seed, manifest.seed + 1, manifest.seed + 2];
        let holdout: Vec<u32> = species["holdout_seeds"]
            .as_array()
            .expect("holdout_seeds is an array")
            .iter()
            .map(|v| v.as_u64().expect("a seed is a number") as u32)
            .collect();
        assert_eq!(holdout.len(), 3, "three holdout seeds: {holdout:?}");
        let unique: std::collections::BTreeSet<u32> = holdout.iter().copied().collect();
        assert_eq!(unique.len(), 3, "holdout seeds must be pairwise distinct");
        for seed in &holdout {
            assert!(
                !fixed.contains(seed),
                "holdout seed {seed} collides with a fixed seed {fixed:?}"
            );
        }

        let specimens: Value = read_json(&paths.packet("specimens")).unwrap();
        // The catalogue check reads the record by its schema (fn-149).
        assert_eq!(
            (&specimens["schema"], &specimens["schema_version"]),
            (&json!("specimens"), &json!(1))
        );
        let cases = specimens["cases"].as_array().expect("cases is an array");
        let count = |role: &str| cases.iter().filter(|c| c["seed_role"] == role).count();
        assert_eq!(count("regression"), 3, "three fixed cases");
        assert_eq!(count("holdout"), 3, "three holdout cases");

        let holdout_case_seeds: Vec<u32> = cases
            .iter()
            .filter(|c| c["seed_role"] == "holdout")
            .map(|c| c["seed"].as_u64().unwrap() as u32)
            .collect();
        assert_eq!(
            holdout_case_seeds, holdout,
            "holdout cases carry the same seeds as species.json"
        );
    }

    #[test]
    fn write_packet_draws_the_same_holdout_seeds_on_a_rerun() {
        let dir = scratch();
        let paths = Paths::new(&dir);
        let manifest = manifest();
        write_packet(&paths, &manifest, json!({})).unwrap();
        let first: Value = read_json(&paths.packet("species")).unwrap();
        write_packet(&paths, &manifest, json!({})).unwrap();
        let second: Value = read_json(&paths.packet("species")).unwrap();
        assert_eq!(first["holdout_seeds"], second["holdout_seeds"]);
    }
}
