//! The beech's measures at each age and seed, and a side view per tree for
//! a quick look (x across, z up, each internode as wide as its girth).
//!
//!   cargo run --release -p telperion-space --example beech -- <out dir> [ages...]
use std::fmt::Write;
use std::time::Instant;
use telperion_space::{beech, expected_counts, grow, Request, Structure};

const BUDGET: u32 = 5_000_000;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = args.first().expect("usage: beech <out dir> [ages...]");
    let ages: Vec<u32> = match args[1..].iter().map(|a| a.parse()).collect() {
        Ok(ages) if !args[1..].is_empty() => ages,
        _ => vec![10, 20, 40, 80],
    };
    std::fs::create_dir_all(out).unwrap();
    let species = beech();
    for &age in &ages {
        let expected = expected_counts(&species, age).unwrap();
        let per_pa: Vec<f64> = (0..expected.pas).map(|pa| expected.total(pa)).collect();
        let total: f64 = per_pa.iter().sum();
        let shares: Vec<String> = per_pa.iter().map(|t| format!("{t:.0}")).collect();
        println!("age {age}: expected grown per PA {}", shares.join(" "));
        for seed in [1, 7] {
            let started = Instant::now();
            let tree = match grow(
                &species,
                Request {
                    age,
                    seed,
                    budget: BUDGET,
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
            std::fs::write(format!("{out}/beech-{age}-{seed}.svg"), svg).unwrap();
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
    let base = tree.axes[0].phytomers.first().map_or(0.0, |p| p.radius);
    format!(
        "{} phytomers, {} axes, height {top:.1} m, width {:.1} m, base radius {base:.3} m",
        tree.phytomer_count(),
        tree.axes.len(),
        high - low
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
