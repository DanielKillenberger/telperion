//! fn-211 task 9: what each pass of `grow_staged` costs, warm (the best of
//! three after one cold build), for the four species at 80 and the lit
//! oak, so a change to one arithmetic (contract A's fixed-point layout)
//! is priced against the base, pass by pass. The layout's share is the
//! sketch, the re-lay's place and sag, and the final lay's place and sag.
//!
//!   cargo run --profile ci -p telperion-space --example passcost -- [trees]
use std::collections::BTreeMap;
use std::time::Instant;
use telperion_space::{beech, grow_staged, oak, palm, spruce, Light, Pass, Request, Stage};

fn main() {
    let lit = Light {
        extinction: 0.5,
        sky: 0.5,
    };
    let trees = [
        ("beech", beech(), Light::NEUTRAL),
        ("spruce", spruce(), Light::NEUTRAL),
        ("oak", oak(), Light::NEUTRAL),
        ("oak lit", oak(), lit),
        ("palm", palm(), Light::NEUTRAL),
    ];
    let layout = [
        Pass::Sketch,
        Pass::RelayPlace,
        Pass::RelaySag,
        Pass::Place,
        Pass::SagPlace,
    ];
    println!("| Tree 80 s1 | build ms | layout ms | layout share | per pass ms |");
    println!("|---|--:|--:|--:|---|");
    let only: Vec<String> = std::env::args().skip(1).collect();
    for (name, species, light) in trees {
        if !only.is_empty() && !only.iter().any(|o| o == name) {
            continue;
        }
        let request = Request {
            age: 80,
            seed: 1,
            budget: 20_000_000,
            light,
        };
        let mut best: Option<(f64, BTreeMap<Pass, f64>)> = None;
        for round in 0..4 {
            let mut passes: BTreeMap<Pass, f64> = BTreeMap::new();
            let mut last = Instant::now();
            let start = last;
            grow_staged(&species, request, &mut |stage| {
                let now = Instant::now();
                if let Stage::Pass(p) = stage {
                    *passes.entry(p).or_default() += (now - last).as_secs_f64() * 1e3;
                }
                last = now;
            })
            .expect("grows");
            let total = start.elapsed().as_secs_f64() * 1e3;
            if round > 0 && best.as_ref().is_none_or(|b| total < b.0) {
                best = Some((total, passes));
            }
        }
        let (total, passes) = best.expect("timed");
        let lay: f64 = layout.iter().map(|p| passes.get(p).copied().unwrap_or(0.0)).sum();
        let each: Vec<String> = layout
            .iter()
            .map(|p| format!("{p:?} {:.0}", passes.get(p).copied().unwrap_or(0.0)))
            .collect();
        println!(
            "| {name} | {total:.0} | {lay:.0} | {:.1} % | {} |",
            100.0 * lay / total,
            each.join(", ")
        );
    }
}
