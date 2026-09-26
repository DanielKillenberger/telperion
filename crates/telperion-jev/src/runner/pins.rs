//! A species' `pins.json`: the identity of its accepted tree at seed 7, the
//! numbers `crates/telperion-core/src/suite/identity.rs` pins for the shipped
//! species, computed from the family itself rather than copied by hand.
use std::path::Path;

use serde_json::{json, Value};
use telperion_core::{mesh, pipeline, Family};

use crate::pipeline::canon::read_json;

/// The seed every pins record is drawn at.
pub const SEED: u32 = 7;

/// FNV-1a over the bytes, as the identity test hashes.
fn fnv(bytes: impl IntoIterator<Item = u8>) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for byte in bytes {
        hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
    }
    hash
}

/// The pins of `family` at seed 7.
pub fn compute(family: &Family) -> Result<Value, String> {
    let mut family = family.clone();
    family.skeleton.seed = SEED;
    // The skeleton the pipeline ships: a request for no output grows it alone.
    let tree = pipeline::build(&family, pipeline::Request::default())
        .map_err(|e| e.to_string())?
        .skeleton
        .tree;
    let skeleton = fnv(tree.nodes.iter().skip(1).flat_map(|n| {
        [n.position.x, n.position.y, n.position.z]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .chain(n.parent.unwrap_or_default().to_le_bytes())
    }));
    let m = mesh::build(&family).map_err(|e| e.to_string())?;
    let placement = fnv(m
        .foliage
        .instances
        .leaves
        .iter()
        .flatten()
        .flat_map(|w| w.to_le_bytes()));
    let e = &m.foliage.element;
    let element = fnv(e
        .positions
        .iter()
        .flat_map(|p| [p.x, p.y, p.z])
        .flat_map(f64::to_le_bytes)
        .chain(e.indices.iter().flat_map(|i| i.to_le_bytes())));
    Ok(json!({
        "wood_vertices": m.wood_vertices(), "wood_triangles": m.wood_triangles(),
        "instances": m.foliage_instances(),
        "min": [m.bounds.min.x, m.bounds.min.y, m.bounds.min.z],
        "max": [m.bounds.max.x, m.bounds.max.y, m.bounds.max.z],
        "skeleton": skeleton.to_string(), "placement": placement.to_string(),
        "element": element.to_string(),
    }))
}

/// The decade band a leaf count sits in: from its power of ten to two above.
pub fn leaf_band(instances: u64) -> [u64; 2] {
    let decade = 10u64.pow(instances.max(1).ilog10());
    [decade, decade * 100]
}

/// Writes `<folder>/pins.json` for `family`. A record's own leaf band is kept;
/// a new record takes the decade band of its leaf count.
pub fn write(folder: &Path, species: &str, family: &Family) -> Result<String, String> {
    let path = folder.join("pins.json");
    let pins = compute(family)?;
    let old = read_json(&path).ok();
    let band = old
        .as_ref()
        .map(|o| o["leaf_band"].clone())
        .filter(Value::is_array)
        .unwrap_or_else(|| json!(leaf_band(pins["instances"].as_u64().unwrap_or(0))));
    let profile = std::fs::read(folder.join("packet/profile.json")).map_err(|e| e.to_string())?;
    let record = json!({
        "schema": "pins", "schema_version": 1, "species": species, "seed": SEED,
        "profile_sha256": crate::sha256_hex(&profile),
        "growth_reference": {"age": family.age, "height_m": family.skeleton.envelope.height},
        "leaf_band": band, "pins": pins,
    });
    let mut bytes = serde_json::to_vec_pretty(&record).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
    Ok(format!("pins.json at seed {SEED}"))
}
