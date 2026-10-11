//! fn-255 (DESIGN.md section 4 (c)): fn-254's two amplification figures
//! re-taken on the one cycle, on the leafy walk tree at seed 3: the gain
//! from a few f32 places of `sky` to the subtree light totals, and the
//! memory change at which a shed decision tips.
//!
//!   cargo run --profile ci -p telperion-space --example amplify -- [sky] [leaf] [ulps]
#[path = "../tests/walk/mod.rs"]
mod walk;

use std::collections::HashMap;
use telperion_space::{trace_cycle, Candidate, Light, Model, Request, Traced};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let sky: f64 = args.first().map_or(0.005, |v| v.parse().unwrap());
    let mut species = walk::species();
    walk::leafy(&mut species);
    let mut models = vec![("lattice".to_string(), Model::Lattice)];
    for leaf in args.get(1).map_or("1,11.75", String::as_str).split(',') {
        let leaf = leaf.parse().unwrap();
        models.push((format!("volumes {leaf}"), Model::Volumes(Candidate { leaf, ..Candidate::default() })));
    }
    // A few f32 places of the setting: three ulps of sky as an f32.
    let ulps: u32 = args.get(2).map_or(3, |v| v.parse().unwrap());
    let nudged = if ulps == 0 { sky } else { f64::from(f32::from_bits((sky as f32).to_bits() + ulps)) };
    let rel_in = (nudged - sky).abs() / sky;
    for (name, model) in models {
        let run = |sky: f64| {
            let light = Light { sky, ..walk::SITE };
            let r = Request { age: walk::AGE, seed: 3, budget: 2_000_000, light };
            trace_cycle(&species, r, model).unwrap()
        };
        let (a, b) = (run(sky), run(nudged));
        report(&name, &a, &b, rel_in);
    }
}

fn report(name: &str, a: &Traced, b: &Traced, rel_in: f64) {
    println!("\n{name}: sky moved {rel_in:.2e} relative");
    for (cycle, (x, y)) in a.iter().zip(b).enumerate() {
        // Axes by index: the runs make them in the same order until a
        // decision differs; a lineage may name more than one axis.
        let same = x.iter().zip(y).filter(|(p, q)| p.0 != q.0).count();
        let other: HashMap<usize, (f64, f64, bool)> = y.iter().enumerate().map(|(i, t)| (i, (t.1, t.2, t.3))).collect();
        let (mut gain, mut memory, mut flips, mut most) = (0.0f64, 0.0f64, 0, 0.0f64);
        for (i, &(_, q, m, alive)) in x.iter().enumerate() {
            let Some(&(q2, m2, alive2)) = other.get(&i) else { continue };
            if alive != alive2 {
                flips += 1;
            }
            if q > 0.0 && q2 > 0.0 {
                let rel = (q2 - q).abs() / q;
                if rel > most {
                    (most, gain) = (rel, rel / rel_in);
                }
            }
            if m != 0.0 {
                memory = memory.max((m2 - m).abs() / m.abs());
            }
        }
        let only = x.len().abs_diff(y.len());
        println!(
            "  cycle {:>2}: largest Q change {most:.2e} relative ({gain:.0}x the input), largest memory change {:.4}%, apexes living in one only {flips}, axes in one only {only}, lineages differing by index {same}",
            cycle + 1, 100.0 * memory
        );
    }
}
