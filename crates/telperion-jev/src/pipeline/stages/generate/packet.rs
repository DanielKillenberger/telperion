//! The packet records and the sidecar the generate stage writes: closed
//! shapes that gain no key, and the shipped values laid over the preset.

use std::path::Path;

use serde_json::{json, Value};

use crate::pipeline::canon::{file_sha256, read_json, write_canonical};
use crate::pipeline::manifest::Manifest;
use crate::pipeline::render::family_for;
use crate::pipeline::stage::{Context, StageError};

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
    ctx: &Context,
    manifest: &Manifest,
    parameters: Value,
) -> Result<(), StageError> {
    let seeds = [manifest.seed, manifest.seed + 1, manifest.seed + 2];
    let taxon = &manifest.taxon;
    write_canonical(
        &ctx.paths.packet("species"),
        &json!({
            "id": manifest.species, "scientific_name": taxon.scientific_name,
            "taxon_rank": taxon.rank, "cultivar": taxon.cultivar, "context": manifest.context,
            "profile_id": manifest.profile_id, "profile_path": "packet/profile.json",
            "profile_sha256": file_sha256(&ctx.paths.packet("profile")).unwrap_or_default(),
            "preset": manifest.preset, "required_capabilities": [], "parameters": parameters,
            "fixed_seeds": seeds, "holdout_seeds": [], "reference_ids": [],
        }),
    )?;
    let cases: Vec<Value> = seeds
        .iter()
        .map(|seed| {
            json!({"id": format!("{}-{seed}", manifest.species), "species_id": manifest.species,
                   "seed": seed, "seed_role": "regression",
                   "parameter_species_id": manifest.species})
        })
        .collect();
    write_canonical(
        &ctx.paths.packet("specimens"),
        &json!({
            "benchmark_id": "fn19-v1", "cases": cases, "seed_evidence": "unaudited",
            "fresh_for_future_tuning": false, "receipts": [],
            "generation_status": "measured-by-pipeline", "expert_status": "unassessed",
        }),
    )?;
    Ok(())
}
