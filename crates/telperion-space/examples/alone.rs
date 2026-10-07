//! fn-209 probe P2: a tree grown and shed chain by chain alone, against
//! the exact one-pass engine (the reference).
//!
//!   cargo run --release -p telperion-space --example alone -- [species] [ages] [seeds]
//!
//! Prints per order: kept phytomers against the reference, the lineages
//! the two share, each shared lateral's own axis phytomers (equal where a
//! chain grown alone is the chain the whole tree grows), each shared
//! lateral's base radius over the reference's; then the cost of every
//! chain's growth and shedding, and of its geometry, on one core.
#[path = "junctions/stats.rs"]
mod stats;

use stats::{quantiles, tail};
use std::collections::HashMap;
use std::time::Instant;
use telperion_space::{
    beech, grow, grow_alone_with, palm, spruce, Expected, Light, Origin, Request, Species,
    Structure,
};

const BUDGET: u32 = 20_000_000;
const SPECIES: [(&str, fn() -> Species); 3] =
    [("beech", beech), ("spruce", spruce), ("palm", palm)];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let only = args.first().filter(|o| o.as_str() != "all");
    let list = |i: usize, d: &str| -> Vec<u64> {
        let text = args.get(i).map_or(d, String::as_str);
        text.split(',')
            .map(|v| v.parse().expect("a number"))
            .collect()
    };
    let (ages, seeds) = (list(1, "40"), list(2, "1,7"));
    for (name, make) in SPECIES {
        if only.is_some_and(|o| o != name) {
            continue;
        }
        let species = make();
        for &age in &ages {
            let t = Instant::now();
            let expected = Expected::new(&species, age as u32, 0);
            println!(
                "{name} age {age}: closed-form rows {:.2} s on every core",
                t.elapsed().as_secs_f64()
            );
            for &seed in &seeds {
                let request = Request {
                    age: age as u32,
                    seed,
                    budget: BUDGET,
                    light: Light::NEUTRAL,
                };
                run(
                    &format!("{name} age {age} seed {seed}"),
                    &species,
                    request,
                    &expected,
                );
            }
        }
    }
}

/// Each axis's order: the laterals it stands on.
fn orders(s: &Structure) -> Vec<usize> {
    let mut o = vec![0usize; s.axes.len()];
    for (i, a) in s.axes.iter().enumerate() {
        o[i] = match a.origin {
            Origin::Seed => 0,
            Origin::Lateral { parent, .. } => o[parent] + 1,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => o[parent],
        };
    }
    o
}

fn run(label: &str, species: &Species, request: Request, expected: &Expected) {
    let t = Instant::now();
    let reference = grow(species, request).expect("the reference grows");
    let reference_s = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let correct = std::env::var("UNCORRECTED").is_err();
    let mut tree = grow_alone_with(species, request, expected, correct).expect("grows alone");
    let alone_s = t.elapsed().as_secs_f64();
    // Geometry chain by chain, top-down, one core.
    let mut geometry = Vec::with_capacity(tree.bounded.chains());
    for c in 0..tree.bounded.chains() {
        let t = Instant::now();
        tree.bounded.resolve(c).expect("resolves");
        geometry.push(t.elapsed().as_secs_f64() * 1e6);
    }
    let geometry_s: f64 = geometry.iter().sum::<f64>() / 1e6;
    let ours = tree.bounded.structure();
    let (ro, oo) = (orders(&reference), orders(ours));
    let cap = |o: usize| o.min(4);
    let mut count: HashMap<usize, (usize, usize)> = HashMap::new();
    for (a, &o) in reference.axes.iter().zip(&ro) {
        count.entry(cap(o)).or_default().1 += a.phytomers.len();
    }
    for (a, &o) in ours.axes.iter().zip(&oo) {
        count.entry(cap(o)).or_default().0 += a.phytomers.len();
    }
    let by_lineage: HashMap<u64, usize> = reference
        .axes
        .iter()
        .enumerate()
        .filter(|(_, a)| matches!(a.origin, Origin::Lateral { .. }))
        .map(|(i, a)| (a.lineage, i))
        .collect();
    let mut shared: HashMap<usize, (usize, usize, usize, Vec<f64>)> = HashMap::new();
    let mut ours_laterals: HashMap<usize, usize> = HashMap::new();
    for (i, a) in ours.axes.iter().enumerate() {
        if !matches!(a.origin, Origin::Lateral { .. }) {
            continue;
        }
        let o = cap(oo[i]);
        *ours_laterals.entry(o).or_default() += 1;
        let Some(&j) = by_lineage.get(&a.lineage) else {
            continue;
        };
        let r = &reference.axes[j];
        let e = shared.entry(o).or_default();
        e.0 += 1;
        if r.phytomers.len() != a.phytomers.len() {
            e.1 += 1;
        }
        let (Some(pa), Some(pr)) = (a.phytomers.first(), r.phytomers.first()) else {
            continue;
        };
        if pr.radius > 0.0 {
            e.3.push(pa.radius / pr.radius);
        }
        e.2 += 1;
    }
    let mut ref_laterals: HashMap<usize, usize> = HashMap::new();
    for (i, a) in reference.axes.iter().enumerate() {
        if matches!(a.origin, Origin::Lateral { .. }) {
            *ref_laterals.entry(cap(ro[i])).or_default() += 1;
        }
    }
    let trunk = (
        ours.axes[0].phytomers[0].radius,
        reference.axes[0].phytomers[0].radius,
    );
    println!(
        "{label}: kept phytomers {} against {} ({:.3}); trunk base {:.4} m against {:.4} m ({:.3})",
        ours.phytomer_count(),
        reference.phytomer_count(),
        ours.phytomer_count() as f64 / reference.phytomer_count() as f64,
        trunk.0,
        trunk.1,
        trunk.0 / trunk.1
    );
    let mut keys: Vec<usize> = count.keys().copied().collect();
    keys.sort_unstable();
    for o in keys {
        let (a, r) = count[&o];
        let (n_ours, n_ref) = (
            ours_laterals.get(&o).copied().unwrap_or(0),
            ref_laterals.get(&o).copied().unwrap_or(0),
        );
        let s = shared.get(&o);
        let q = s.map_or((f64::NAN, f64::NAN, f64::NAN), |s| quantiles(&s.3));
        println!(
            "  order {o}{}: phytomers {a} against {r} ({:.3}); laterals {n_ours} against {n_ref}, {} shared, {} own axes of another length; base / reference p5 {:.3} p50 {:.3} p95 {:.3}",
            if o == 4 { "+" } else { "" },
            a as f64 / r.max(1) as f64,
            s.map_or(0, |s| s.0),
            s.map_or(0, |s| s.1),
            q.0,
            q.1,
            q.2
        );
    }
    let pooled: Vec<f64> = shared.values().flat_map(|s| s.3.iter().copied()).collect();
    let q = quantiles(&pooled);
    println!(
        "  every shared lateral: base / reference p5 {:.3} p50 {:.3} p95 {:.3} (n {})",
        q.0,
        q.1,
        q.2,
        pooled.len()
    );
    let mut grow_us: Vec<f64> = tree.costs.iter().map(|c| c.micros).collect();
    let grown: u64 = tree.costs.iter().map(|c| c.grown).sum();
    let shed_chains = tree.costs.iter().filter(|c| !c.kept).count();
    let (g50, g99, gmax) = tail(&mut grow_us);
    let kept: Vec<f64> = tree
        .costs
        .iter()
        .filter(|c| c.kept)
        .map(|c| c.micros)
        .collect();
    let mut total: Vec<f64> = kept.iter().zip(&geometry).map(|(a, b)| a + b).collect();
    let (t50, t99, tmax) = tail(&mut total);
    let (m50, m99, mmax) = tail(&mut geometry);
    println!(
        "  chains grown {} ({shed_chains} shed whole on growing, {grown} phytomers grown); reference {reference_s:.3} s, alone growth and shedding {alone_s:.3} s on one core",
        tree.costs.len()
    );
    println!(
        "  chain growth and shedding us p50 {g50:.1} p99 {g99:.1} max {gmax:.0}; geometry p50 {m50:.1} p99 {m99:.1} max {mmax:.0} (all {geometry_s:.3} s); both, per kept chain, p50 {t50:.1} p99 {t99:.1} max {tmax:.0}"
    );
}
