//! Digests of what every shipped preset builds, one JSON line a preset and
//! seed: the pipeline's artifacts for a full request and for the field alone,
//! and the mesh. Two runs of the same generator print the same lines; a
//! refactor that keeps the bytes keeps every digest. What the device builds
//! is digested by the render crate's `resident::digest_tests`.
//! `cargo run --profile ci -p telperion-render --example generation_digest
//! [preset ...]`; `DIGEST_SEEDS` overrides seeds 1 and 7.
use serde_json::{json, Value};
use std::fmt::{Debug, Write};
use telperion_core::{
    mesh,
    pipeline::{self, FieldRequest, Request},
    presets::Preset,
    tree::Tree,
    Family,
};

const PRESETS: [&str; 8] = [
    "ordinary",
    "oregon-white-oak",
    "norway-spruce",
    "european-beech",
    "silver-birch",
    "date-palm",
    "telperion",
    "laurelin",
];

/// FNV-1a over the value's `Debug` text, which prints every float exactly.
struct Fnv(u64);
impl Write for Fnv {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for b in s.bytes() {
            self.0 = (self.0 ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
        }
        Ok(())
    }
}
fn digest(v: &impl Debug) -> String {
    let mut h = Fnv(0xcbf2_9ce4_8422_2325);
    write!(h, "{v:?}").expect("hashing never fails");
    format!("{:016x}", h.0)
}
/// The skeleton as its consumers read it: the nodes and every row drawn
/// beside them, never the `Tree` it is held in (`artifact_digest`'s).
fn skeleton(t: &Tree) -> impl Debug + '_ {
    (
        (&t.nodes, t.crossover, t.diagnostics, &t.sections, &t.frames),
        (&t.keys, &t.grown, &t.blooms, &t.clusters, &t.clustered),
        (&t.leafing, &t.unfolding, t.year),
    )
}
/// The field at 36 cells a side, at the family's limb order.
const FIELD: Option<FieldRequest> = Some(FieldRequest {
    n: 36,
    limb_order: None,
});
fn full(f: &Family) -> Result<Value, Box<dyn std::error::Error>> {
    let request = Request {
        wood: true,
        leaves: true,
        field: FIELD,
        structure: true,
        ..Request::default()
    };
    let built = pipeline::build(f, request)?;
    let o = &built.outputs;
    let structure = o.structure.as_ref().map(|s| (&s.nodes, &s.topology));
    Ok(json!({
        "tree": digest(&skeleton(&built.skeleton.tree)),
        "element": digest(&o.element),
        "plan": digest(&o.plan),
        "wood": digest(&o.wood),
        "leaves": digest(&o.leaves),
        "field": digest(&o.field),
        "structure": digest(&structure),
    }))
}
fn field_only(f: &Family) -> Result<Value, Box<dyn std::error::Error>> {
    let request = Request {
        field: FIELD,
        ..Request::default()
    };
    let o = pipeline::build(f, request)?.outputs;
    Ok(json!({"field": digest(&o.field), "leaves": digest(&o.leaves), "plan": digest(&o.plan)}))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let named: Vec<String> = std::env::args().skip(1).collect();
    let presets: Vec<&str> = if named.is_empty() {
        PRESETS.to_vec()
    } else {
        named.iter().map(String::as_str).collect()
    };
    let seeds: Vec<u32> = std::env::var("DIGEST_SEEDS")
        .unwrap_or_else(|_| "1,7".into())
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    for id in presets {
        for &seed in &seeds {
            let mut f = Preset::from_id(id).ok_or("unknown preset")?.parameters();
            f.skeleton.seed = seed;
            let line = json!({
                "preset": id,
                "seed": seed,
                "pipeline": full(&f)?,
                "fieldOnly": field_only(&f)?,
                "mesh": digest(&mesh::build(&f)?),
            });
            println!("{line}");
        }
    }
    Ok(())
}
