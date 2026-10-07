//! fn-211.4: the oak under fn-209's cut. Its structure above the cut is
//! grown year by year with today's passes, every fine root an aggregate
//! in them; each fine subtree is then grown alone from its records, and
//! the whole is laid as today lays it. Against today's oak: the pass's
//! cost and the build's, the fidelity (`light/fidelity.rs`), the bud
//! light per fine root against its record, and the pass's light field
//! against the laid tree's (`light/field.rs`).
//!
//!   cargo run --profile ci -p telperion-space --example couple -- [ages] [seeds] [cut] [recorded|light]
#[path = "light/fidelity.rs"]
mod fidelity;
#[path = "light/field.rs"]
mod field;
#[path = "junctions/stats.rs"]
mod stats;

use stats::quantiles;
use std::collections::HashMap;
use std::time::Instant;
use telperion_space::{
    bud_light, grow_coupled, grow_oracle, oak, Expected, Light, Measured, Read, Request, Split,
    Structure,
};

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let list = |i: usize, d: &str| -> Vec<u64> {
        let text = args.get(i).map_or(d, String::as_str);
        text.split(',')
            .map(|v| v.parse().expect("a number"))
            .collect()
    };
    let (ages, seeds) = (list(0, "40,80"), list(1, "1,7"));
    let cut: f64 = args.get(2).map_or(1024.0, |c| c.parse().expect("a cut"));
    let split = match args.get(3).map(String::as_str) {
        Some("light") => Split::PerLight,
        _ => Split::Recorded,
    };
    let species = oak();
    for &age in &ages {
        let expected = Expected::new(&species, age as u32, 0);
        for &seed in &seeds {
            let request = Request {
                age: age as u32,
                seed,
                budget: 20_000_000,
                light: LIT,
            };
            let t = Instant::now();
            let today = grow_oracle(&species, request, &expected, cut, Read::Exact).expect("grows");
            let today_s = t.elapsed().as_secs_f64();
            let t = Instant::now();
            let (ours, m) =
                grow_coupled(&species, request, &expected, (cut, split)).expect("couples");
            let ours_s = t.elapsed().as_secs_f64();
            println!(
                "oak {age} s{seed} cut {cut}: today's {today_s:.2} s; coupled {ours_s:.2} s (pass {:.2} s, fine subtrees {:.2} s, laid {:.2} s); {} coupling laterals, {} fine roots, records {:.2} MB; grown {} in the pass, {} in fine subtrees (aggregates expected {:.0})",
                m.pass_s,
                m.fine_s,
                ours_s - m.pass_s - m.fine_s,
                m.coupled,
                m.fine,
                m.record_bytes as f64 / 1e6,
                m.pass_grown,
                m.fine_grown,
                m.expected_fine
            );
            fidelity::describe(
                "coupled",
                (&ours, &m.growth),
                (&today.structure, &today.growth),
            );
            per_root(&ours, &m);
            field::compare(&ours, &m, &species, &LIT);
        }
    }
}

/// Each living bud of a fine root in the laid tree against its root's
/// recorded light for the last cycle.
fn per_root(s: &Structure, m: &Measured) {
    let record: HashMap<usize, f64> = m
        .reads
        .iter()
        .filter(|r| r.3 && r.0 < m.tag.len() && m.tag[r.0] != usize::MAX)
        .map(|r| (m.tag[r.0], r.2))
        .collect();
    let lights = bud_light(s, &oak(), &LIT);
    let mean = lights.iter().map(|l| l.1).sum::<f64>() / lights.len().max(1) as f64;
    let d: Vec<f64> = lights
        .iter()
        .filter_map(|&(i, l)| record.get(&m.tag[i]).map(|r| (l - r).abs()))
        .collect();
    let (_, p50, p95) = quantiles(&d);
    println!(
        "  bud light per fine root, |bud - its root's record| (n {} of {} living buds, mean {mean:.3}): p50 {p50:.4} ({:.1} %) p95 {p95:.4} ({:.1} %)",
        d.len(),
        lights.len(),
        100.0 * p50 / mean,
        100.0 * p95 / mean
    );
}
