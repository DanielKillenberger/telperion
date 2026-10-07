//! fn-209 F1, task 1: the cut's predictor (a bud's expected kept
//! phytomers, light-neutral) against the fine roots' realized sizes, the
//! empty roots, and the work inside an item, at N = 1,024 (or the cut
//! given).
//!
//!   cargo run --profile ci -p telperion-space --example cut -- [ages] [seeds] [cut]
//!
//! The beech and spruce are grown above the cut and expanded below it
//! (`grow_fine`). The oak reads light, so its fine roots cannot be
//! expanded yet (F2b): its predictor is surveyed against the lit tree the
//! engine grows whole (`survey_lit`, sizes only), and its item work is
//! read off the oak with its light switched off, expanded.
#[path = "junctions/stats.rs"]
mod stats;

use stats::{quantiles, tail};
use std::time::Instant;
use telperion_space::{
    beech, grow_fine, oak, spruce, survey_lit, Expected, Light, Plan, Request, Species, Work,
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
    println!("| Tree | Roots | Empty | Above the cut (phytomers) | Realized / predicted p5 / p50 / p95 | Realized over the cut: roots, most | Σ realized / Σ predicted | Grown p50 / p95 / most | Chains p50 / p95 / most | Chain-cycles p50 / p95 / most | Axes made most | µs p50 / p99 / most | Wall s |");
    println!("|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    let all: [(&str, fn() -> Species); 3] = [("beech", beech), ("spruce", spruce), ("oak", oak)];
    for (name, make) in all {
        for &age in &ages {
            let species = make();
            let expected = Expected::new(&species, age as u32, 0);
            for &seed in &seeds {
                let request = |light| Request {
                    age: age as u32,
                    seed,
                    budget: 20_000_000,
                    light,
                };
                let label = format!("{name} {age} seed {seed}");
                if name == "oak" {
                    let lit = Light {
                        extinction: 0.5,
                        sky: 0.5,
                    };
                    let t = Instant::now();
                    let s = survey_lit(&species, request(lit), &expected, cut).expect("grows");
                    let sizes: Vec<(f64, usize)> = s.roots.clone();
                    let wall = t.elapsed().as_secs_f64();
                    row(
                        &format!("{label}, lit, sizes"),
                        &sizes,
                        s.above,
                        &[],
                        cut,
                        wall,
                    );
                    let unlit = unlit(&species);
                    expand(
                        &format!("{label}, light off"),
                        &unlit,
                        request(Light::NEUTRAL),
                        cut,
                    );
                } else {
                    expand(&label, &species, request(Light::NEUTRAL), cut);
                }
            }
        }
    }
}

fn expand(label: &str, species: &Species, request: Request, cut: f64) {
    let t = Instant::now();
    let plan = Plan {
        cut,
        ..Plan::default()
    };
    let fine = grow_fine(species, request, plan).expect("grows");
    let wall = t.elapsed().as_secs_f64();
    let sizes: Vec<(f64, usize)> = fine.roots.iter().map(|r| (r.predicted, r.kept)).collect();
    let work: Vec<Work> = fine.roots.iter().map(|r| r.work).collect();
    let above: usize = fine.structure.axes[..fine.above]
        .iter()
        .map(|a| a.phytomers.len())
        .sum();
    row(label, &sizes, above, &work, cut, wall);
}

fn row(label: &str, sizes: &[(f64, usize)], above: usize, work: &[Work], cut: f64, wall: f64) {
    let empty = sizes.iter().filter(|s| s.1 == 0).count();
    let ratios: Vec<f64> = sizes
        .iter()
        .filter(|s| s.1 > 0 && s.0 > 0.0)
        .map(|s| s.1 as f64 / s.0)
        .collect();
    let (r5, r50, r95) = quantiles(&ratios);
    let over: Vec<usize> = sizes
        .iter()
        .filter(|s| s.1 as f64 > cut)
        .map(|s| s.1)
        .collect();
    let most_over = over.iter().max().copied().unwrap_or(0);
    let predicted: f64 = sizes.iter().map(|s| s.0).sum();
    let realized: usize = sizes.iter().map(|s| s.1).sum();
    let col = |f: &dyn Fn(&Work) -> f64| -> String {
        if work.is_empty() {
            return "–".into();
        }
        let mut v: Vec<f64> = work.iter().map(f).collect();
        let (p50, _, most) = tail(&mut v);
        let (_, _, p95) = quantiles(&v);
        format!("{p50:.0} / {p95:.0} / {most:.0}")
    };
    let micros = if work.is_empty() {
        "–".into()
    } else {
        let mut v: Vec<f64> = work.iter().map(|w| w.micros).collect();
        let (p50, p99, most) = tail(&mut v);
        format!("{p50:.0} / {p99:.0} / {most:.0}")
    };
    let axes = work
        .iter()
        .map(|w| w.axes)
        .max()
        .map_or("–".into(), |a| a.to_string());
    println!(
        "| {label} | {} | {empty} | {above} | {r5:.2} / {r50:.2} / {r95:.2} | {}, {most_over} | {:.3} | {} | {} | {} | {axes} | {micros} | {wall:.2} |",
        sizes.len(),
        over.len(),
        realized as f64 / predicted.max(f64::MIN_POSITIVE),
        col(&|w| w.grown as f64),
        col(&|w| w.chains as f64),
        col(&|w| w.cycles as f64),
    );
}

/// The oak with its light switched off: every term that reads light
/// dormant, and no shed pipe retained, so its fine roots can be
/// expanded.
fn unlit(species: &Species) -> Species {
    let mut s = species.clone();
    for state in &mut s.states {
        (state.shade_hazard, state.shade_size) = (0.0, 0.0);
        (state.balance_hazard, state.leaf_girth) = (0.0, 0.0);
        state.retained = 0.0;
    }
    s
}
