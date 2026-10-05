//! The beech's measures at each age and seed, and a side view per tree for
//! a quick look (x across, z up, each internode as wide as its girth).
//!
//! Another tree-space species is measured by naming it (`oak`, `spruce`).
//!
//!   cargo run --release -p telperion-space --example beech -- <out dir> [species] [ages...] [--seeds 1,7]
use std::fmt::Write;
use std::time::Instant;
use telperion_space::{beech, expected_counts, grow, oak, spruce, Origin, Request, Structure};

const BUDGET: u32 = 20_000_000;

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let mut seeds = vec![1u64, 7];
    if let Some(at) = args.iter().position(|a| a == "--seeds") {
        let list = args.get(at + 1).expect("--seeds needs a list");
        seeds = list.split(',').map(|s| s.parse().unwrap()).collect();
        args.drain(at..at + 2);
    }
    let out = args.first().expect("usage: beech <out dir> [ages...]");
    let (name, rest) = match args.get(1).map(String::as_str) {
        Some(name @ ("beech" | "oak" | "spruce")) => (name, &args[2..]),
        _ => ("beech", &args[1..]),
    };
    let ages: Vec<u32> = match rest.iter().map(|a| a.parse()).collect() {
        Ok(ages) if !rest.is_empty() => ages,
        _ => vec![10, 20, 40, 80],
    };
    std::fs::create_dir_all(out).unwrap();
    let species = match name {
        "oak" => oak(),
        "spruce" => spruce(),
        _ => beech(),
    };
    for &age in &ages {
        let expected = expected_counts(&species, age).unwrap();
        let per_pa: Vec<f64> = (0..expected.pas).map(|pa| expected.total(pa)).collect();
        let total: f64 = per_pa.iter().sum();
        let shares: Vec<String> = per_pa.iter().map(|t| format!("{t:.0}")).collect();
        println!("age {age}: expected grown per PA {}", shares.join(" "));
        for &seed in &seeds {
            let started = Instant::now();
            let tree = match grow(
                &species,
                Request {
                    age,
                    seed,
                    budget: BUDGET,
                    light: telperion_space::Light::NEUTRAL,
                },
            ) {
                Ok(tree) => tree,
                Err(error) => {
                    println!("age {age} seed {seed}: {error:?}");
                    continue;
                }
            };
            let ms = started.elapsed().as_secs_f64() * 1e3;
            println!(
                "age {age} seed {seed}: {} ({ms:.1} ms, expected {total:.0} grown)",
                measures(&tree)
            );
            let svg = side(&tree);
            std::fs::write(format!("{out}/{name}-{age}-{seed}.svg"), svg).unwrap();
        }
    }
}

/// Height, width, the lowest living wood off the stem and the girth at the base.
fn measures(tree: &Structure) -> String {
    let tips = tree.axes.iter().flat_map(|a| &a.phytomers);
    let (mut low, mut high, mut top) = (f64::MAX, f64::MIN, 0.0f64);
    for p in tips {
        low = low.min(p.tip.x.min(p.tip.y));
        high = high.max(p.tip.x.max(p.tip.y));
        top = top.max(p.tip.z);
    }
    // The diameter at breast height, 1.3 m, on the stem: the seed axis and
    // the relays and continuations that carry it on.
    let mut stem = vec![false; tree.axes.len()];
    stem[0] = true;
    for (i, axis) in tree.axes.iter().enumerate().skip(1) {
        stem[i] = match axis.origin {
            Origin::Relay { parent, .. } | Origin::Continuation { parent } => stem[parent],
            _ => false,
        };
    }
    let dbh = 2.0
        * tree
            .axes
            .iter()
            .zip(&stem)
            .filter(|(_, &s)| s)
            .flat_map(|(a, _)| &a.phytomers)
            .filter(|p| p.tip.z >= 1.3)
            .min_by(|a, b| a.tip.z.total_cmp(&b.tip.z))
            .map_or(0.0, |p| p.radius);
    // The mean drawn length of an axis that never branches: a short shoot.
    let mut bears = vec![false; tree.axes.len()];
    for axis in &tree.axes {
        if let Some(parent) = axis.origin.parent() {
            bears[parent] = true;
        }
    }
    let shoots: Vec<f64> = tree
        .axes
        .iter()
        .zip(&bears)
        .filter(|(a, &b)| !b && a.pa + 1 == tree.pas)
        .map(|(a, _)| {
            a.phytomers
                .iter()
                .fold((a.base, 0.0), |(f, l), p| (p.tip, l + (p.tip - f).length()))
                .1
        })
        .collect();
    let short = shoots.iter().sum::<f64>() / shoots.len().max(1) as f64;
    format!(
        "{} phytomers, {} axes, height {top:.1} m, width {:.1} m, dbh {dbh:.2} m, short shoot {:.1} mm",
        tree.phytomer_count(),
        tree.axes.len(),
        high - low,
        short * 1e3
    )
}

fn side(tree: &Structure) -> String {
    let mut body = String::new();
    let (mut low, mut high, mut top) = (0.0f64, 0.0f64, 1.0f64);
    for axis in &tree.axes {
        let mut from = axis.base;
        for p in &axis.phytomers {
            let w = 2.0 * p.radius;
            writeln!(
                body,
                r#"<line x1="{:.3}" y1="{:.3}" x2="{:.3}" y2="{:.3}" stroke-width="{w:.4}"/>"#,
                from.x, -from.z, p.tip.x, -p.tip.z
            )
            .unwrap();
            low = low.min(p.tip.x);
            high = high.max(p.tip.x);
            top = top.max(p.tip.z);
            from = p.tip;
        }
    }
    let (w, h) = (high - low + 2.0, top + 2.0);
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{:.2} {:.2} {w:.2} {h:.2}" stroke="rgb(59,47,37)" stroke-linecap="round"><rect x="{:.2}" y="{:.2}" width="{w:.2}" height="{h:.2}" fill="white" stroke="none"/>{body}</svg>"#,
        low - 1.0,
        -top - 1.0,
        low - 1.0,
        -top - 1.0
    )
}
