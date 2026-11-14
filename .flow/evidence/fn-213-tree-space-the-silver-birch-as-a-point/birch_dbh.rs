//! fn-213 round 14: one preset's stem diameter at breast height (1.3 m
//! above the root, the stem's edge across it, its radii interpolated) and
//! the short shoots' own base radius (runs under 0.05 m, the proximal
//! radius of their first edge). Copied into `crates/telperion-core/examples/`
//! to run, never committed there:
//!
//!   cargo run --profile ci -p telperion-core --example birch_dbh -- <preset> <age> <seed>
use std::collections::HashMap;

use telperion_core::{pipeline, presets::Preset};

const BREAST: f64 = 1.3;
const SHORT: f64 = 0.05;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [id, age, seed] = &args[..] else {
        return Err("usage: birch_dbh <preset> <age> <seed>".into());
    };
    let mut family = Preset::from_id(id).ok_or("no preset")?.parameters();
    family.age = age.parse().map_err(|e| format!("{e}"))?;
    family.skeleton.seed = seed.parse().map_err(|e| format!("{e}"))?;
    let built = pipeline::build(&family, pipeline::Request::mesh()).map_err(|e| e.to_string())?;
    let nodes = &built.skeleton.tree.nodes;
    let ground = nodes[0].position.y;
    // The stem edge that crosses breast height, the thickest if several.
    let mut dbh: f64 = 0.0;
    for n in nodes.iter().filter(|n| n.stem) {
        let Some(p) = n.parent else { continue };
        let (y0, y1) = (nodes[p as usize].position.y - ground, n.position.y - ground);
        if y0 <= BREAST && BREAST < y1 {
            let t = (BREAST - y0) / (y1 - y0);
            dbh = dbh.max(2.0 * (n.start_radius + t * (n.radius - n.start_radius)));
        }
    }
    // Short runs: nodes sharing a branch id, their summed length.
    let mut runs: HashMap<u32, (f64, usize)> = HashMap::new();
    for (i, n) in nodes.iter().enumerate().skip(1) {
        let p = n.parent.unwrap() as usize;
        let e = runs.entry(n.branch).or_insert((0.0, i));
        e.0 += (n.position - nodes[p].position).length();
    }
    let mut base: Vec<f64> = runs
        .values()
        .filter(|(len, _)| *len < SHORT)
        .map(|&(_, first)| nodes[first].start_radius * 1000.0)
        .collect();
    base.sort_by(f64::total_cmp);
    let q = |f: f64| base[((base.len() - 1) as f64 * f) as usize];
    println!(
        "preset {id} age {age} seed {seed}: {} nodes; DBH {:.1} cm; short-run base radius mm p10/p50/p90/max {:.2}/{:.2}/{:.2}/{:.2} (n {})",
        nodes.len(),
        dbh * 100.0,
        q(0.1),
        q(0.5),
        q(0.9),
        base[base.len() - 1],
        base.len()
    );
    Ok(())
}
