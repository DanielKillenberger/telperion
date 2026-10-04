//! Our half of R5's side-by-side sheet: for every oracle parameter set, the
//! trees at seeds 1 and 2 as planar SVG drawings, and the time per structure.
//!
//!   cargo run --release -p telperion-space --example sheet -- \
//!     crates/telperion-space/tests/fixtures/greenlab-oracle.json <out dir>
//!
//! `node scripts/greenlab-oracle.mjs sheet` then sets them beside the
//! simulators' own drawings.
#[path = "../tests/common/mod.rs"]
mod common;
use std::fmt::Write;
use std::time::Instant;
use telperion_space::Structure;

const COLOURS: [&str; 5] = ["#1b4f8a", "#3a7d2c", "#b5402a", "#c98a1b", "#6b3fa0"];
const TIMED: u64 = 2_000;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [fixture, out] = args.as_slice() else {
        panic!("usage: sheet <fixture.json> <out dir>");
    };
    std::fs::create_dir_all(out).unwrap();
    let text = std::fs::read_to_string(fixture).unwrap();
    let sets: serde_json::Value = serde_json::from_str(&text).unwrap();
    let mut timing = serde_json::Map::new();
    for set in sets["sets"].as_array().unwrap() {
        let name = set["name"].as_str().unwrap();
        let species = common::species(set);
        let age = common::age(set);
        for seed in [1, 2] {
            let svg = draw(&common::tree(&species, age, seed));
            std::fs::write(format!("{out}/{name}.{seed}.svg"), svg).unwrap();
        }
        let started = Instant::now();
        let mut totals = vec![0.0; species.states.len()];
        for seed in 0..TIMED {
            let counts = common::tree(&species, age, seed).counts();
            for (pa, total) in totals.iter_mut().enumerate() {
                *total += counts.total(pa) / TIMED as f64;
            }
        }
        let us = started.elapsed().as_secs_f64() * 1e6 / TIMED as f64;
        println!(
            "{name}: {us:.2} us per structure, {:.1} phytomers",
            totals.iter().sum::<f64>()
        );
        timing.insert(
            name.into(),
            serde_json::json!({ "us_per_structure": us, "totals": totals }),
        );
    }
    let json = serde_json::to_string_pretty(&timing).unwrap();
    std::fs::write(format!("{out}/timing.json"), json).unwrap();
}

/// The tree seen from the side (x across, z up), one line per internode.
fn draw(tree: &Structure) -> String {
    let mut lines = String::new();
    let (mut low, mut high) = ((0.0f64, 0.0f64), (0.0f64, 0.0f64));
    for axis in &tree.axes {
        let mut from = axis.base;
        let width = 0.12 / (1.0 + axis.pa as f64);
        for phytomer in &axis.phytomers {
            let to = phytomer.tip;
            let colour = COLOURS[axis.pa.min(COLOURS.len() - 1)];
            let (x1, y1, x2, y2) = (from.x, -from.z, to.x, -to.z);
            writeln!(lines, r#"<line x1="{x1:.3}" y1="{y1:.3}" x2="{x2:.3}" y2="{y2:.3}" stroke="{colour}" stroke-width="{width:.3}"/>"#).unwrap();
            low = (low.0.min(to.x), low.1.min(-to.z));
            high = (high.0.max(to.x), high.1.max(-to.z));
            from = to;
        }
    }
    let pad = 0.3;
    let (w, h) = (high.0 - low.0 + 2.0 * pad, high.1 - low.1 + 2.0 * pad);
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{:.3} {:.3} {w:.3} {h:.3}" stroke-linecap="round">{lines}</svg>"#,
        low.0 - pad,
        low.1 - pad,
    )
}
