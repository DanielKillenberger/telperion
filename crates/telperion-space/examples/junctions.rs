//! fn-209's junction competition: one contender against the exact
//! one-pass engine (the reference), by the brief's definitions.
//!
//!   cargo run --release -p telperion-space --example junctions -- <what> [species] [ages] [seeds]
//!
//! - `fidelity`: per order, each lateral's base radius over the
//!   reference's (p5, p50, p95), and its bearer's radius at its node
//!   likewise; thickness variety (the coefficient of variation of base
//!   radius among siblings: laterals of one bearing chain, one PA and one
//!   five-cycle birth band), contender against reference.
//! - `pop`: the tree refined in three orders (top-down, random, a camera
//!   flying in); every junction radius as first drawn against the final
//!   one, and each order's tree hashed against the one-pass build on one
//!   and on every core.
//! - `perf`: single chain resolution times, the whole tree on one and on
//!   every core, the fly-in's work per frame, memory at the hero view and
//!   fully resolved, and the closed form's rows.
//!
//! The contender is `telperion_space::Bounded` (option 4: committed
//! expected boundaries, boundary-constrained chain geometry).
#[path = "junctions/camera.rs"]
mod camera;
#[path = "junctions/stats.rs"]
mod stats;

use camera::{fly_in, hero, View};

use stats::{cv_by_group, hash, quantiles};
use std::collections::HashMap;
use std::time::Instant;
use telperion_space::{
    beech, grow, oak, palm, spruce, Bounded, Expected, Light, Origin, Request, Species,
};

const BUDGET: u32 = 20_000_000;
const SPECIES: [(&str, fn() -> Species, bool); 4] = [
    ("beech", beech, false),
    ("spruce", spruce, false),
    ("oak", oak, true),
    ("palm", palm, false),
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let what = args.first().map_or("fidelity", String::as_str);
    let only = args.get(1).filter(|o| o.as_str() != "all");
    let list = |i: usize, d: &str| -> Vec<u64> {
        let text = args.get(i).map_or(d, String::as_str);
        text.split(',')
            .map(|v| v.parse().expect("a number"))
            .collect()
    };
    let (ages, seeds) = (list(2, "40,80"), list(3, "1,7"));
    for (name, make, lit) in SPECIES {
        if only.is_some_and(|o| o != name) {
            continue;
        }
        let species = make();
        for &age in &ages {
            let expected = Expected::new(&species, age as u32, 0);
            if what == "perf" {
                let (all, held) = expected.cost();
                let one = Expected::new(&species, age as u32, 1).cost().0;
                println!(
                    "{name} age {age}: closed-form rows {held} ({} kB), read in {one:.2} s on one core, {all:.2} s on every core",
                    held * std::mem::size_of::<[f64; 9]>() / 1024
                );
            }
            for &seed in &seeds {
                let request = Request {
                    age: age as u32,
                    seed,
                    budget: BUDGET,
                    light: if lit {
                        Light {
                            extinction: 0.5,
                            sky: 0.5,
                        }
                    } else {
                        Light::NEUTRAL
                    },
                };
                let label = format!("{name} age {age} seed {seed}");
                match what {
                    "fidelity" => fidelity(&label, &species, request, &expected),
                    "pop" => pop(&label, &species, request, &expected),
                    "perf" => perf(&label, &species, request, &expected),
                    _ => panic!("junctions fidelity | pop | perf [species] [ages] [seeds]"),
                }
            }
        }
    }
}

fn fidelity(label: &str, species: &Species, request: Request, expected: &Expected) {
    let reference = grow(species, request).expect("the reference grows");
    let mut tree = Bounded::with(species, request, expected).expect("settles");
    tree.resolve_all(0).expect("resolves");
    let tree_ref = &tree;
    let ours = tree_ref.structure();
    assert_eq!(
        ours.axes.len(),
        reference.axes.len(),
        "{label}: one topology"
    );
    // Per order: base and bearer-at-node ratios.
    let mut base: HashMap<usize, Vec<f64>> = HashMap::new();
    let mut bearer: HashMap<usize, Vec<f64>> = HashMap::new();
    let (mut none, mut laterals) = (0usize, 0usize);
    let mut groups: Vec<((usize, usize, u32), f64, f64)> = Vec::new();
    for (i, (a, r)) in ours.axes.iter().zip(&reference.axes).enumerate() {
        assert_eq!(a.lineage, r.lineage, "{label}: axis {i} is one bud");
        let Origin::Lateral { parent, node, .. } = a.origin else {
            continue;
        };
        let (Some(pa), Some(pr)) = (a.phytomers.first(), r.phytomers.first()) else {
            continue;
        };
        laterals += 1;
        if pa.radius == 0.0 && pr.radius > 0.0 {
            none += 1;
        }
        let order = tree.order(tree.chain_of(i)).min(4);
        if pr.radius > 0.0 {
            base.entry(order).or_default().push(pa.radius / pr.radius);
        }
        let (bp, br) = (
            ours.axes[parent].phytomers[node].radius,
            reference.axes[parent].phytomers[node].radius,
        );
        if br > 0.0 {
            bearer.entry(order).or_default().push(bp / br);
        }
        groups.push((
            (tree.chain_of(parent), a.pa, a.birth / 5),
            pa.radius,
            pr.radius,
        ));
    }
    let trunk = (
        ours.axes[0].phytomers[0].radius,
        reference.axes[0].phytomers[0].radius,
    );
    println!(
        "{label}: trunk base {:.4} m against {:.4} m ({:.3}); {laterals} laterals, {none} drawn at no radius",
        trunk.0,
        trunk.1,
        trunk.0 / trunk.1
    );
    let mut orders: Vec<_> = base.keys().copied().collect();
    orders.sort_unstable();
    for o in orders {
        let (b, p) = (
            quantiles(&base[&o]),
            quantiles(bearer.get(&o).map_or(&[][..], |v| v)),
        );
        println!(
            "  order {}{}: base / reference p5 {:.3} p50 {:.3} p95 {:.3} (n {}); bearer at node p5 {:.3} p50 {:.3} p95 {:.3}",
            o,
            if o == 4 { "+" } else { "" },
            b.0,
            b.1,
            b.2,
            base[&o].len(),
            p.0,
            p.1,
            p.2
        );
    }
    let (ours_cv, ref_cv, n) = cv_by_group(&groups);
    println!(
        "  thickness variety: median sibling CV {ours_cv:.3} against reference {ref_cv:.3} ({:.2}), over {n} sibling groups",
        ours_cv / ref_cv
    );
}

fn pop(label: &str, species: &Species, request: Request, expected: &Expected) {
    let mut one = Bounded::with(species, request, expected).expect("settles");
    one.resolve_all(1).expect("resolves");
    let single = hash(one.structure());
    let mut all = Bounded::with(species, request, expected).expect("settles");
    all.resolve_all(0).expect("resolves");
    let every = hash(all.structure());
    let view = View::new(fly_in(&one, 0));
    drop((one, all));
    println!("{label}: one-pass hash {single:016x} on one core, {every:016x} on every core");
    for order in ["top-down", "random", "fly-in"] {
        let mut tree = Bounded::with(species, request, expected).expect("settles");
        // Each junction as first drawn: the lateral's base, its bearer at
        // its node, by the lateral's axis.
        let mut drawn: HashMap<usize, (f64, f64)> = HashMap::new();
        let mut seen = 0;
        if order == "fly-in" {
            let children = camera::children(&tree);
            tree.resolve(0).expect("the trunk resolves");
            seen += 1;
            let mut frontier = children[0].clone();
            for cam in &view.path {
                let mut note = |t: &Bounded, c: usize| record(t, c, &mut drawn);
                seen += camera::refine(&mut tree, cam, &mut frontier, &children, &mut note).0;
            }
        }
        for c in sequence(&tree, order, request.seed) {
            if tree.is_resolved(c) {
                continue;
            }
            tree.resolve(c).expect("resolves");
            seen += 1;
            record(&tree, c, &mut drawn);
        }
        let s = tree.structure();
        let mut worst: f64 = 0.0;
        for (&i, &(b, p)) in &drawn {
            let Origin::Lateral { parent, node, .. } = s.axes[i].origin else {
                continue;
            };
            worst = worst.max((s.axes[i].phytomers[0].radius - b).abs());
            worst = worst.max((s.axes[parent].phytomers[node].radius - p).abs());
        }
        let h = hash(s);
        println!(
            "  {order}: {seen} chains, {} junctions drawn, max abs change {worst:e}; hash {h:016x}, equal to one-pass {}",
            drawn.len(),
            h == single && h == every
        );
    }
}

/// Records chain `c`'s junction where it first meets its bearer.
fn record(tree: &Bounded, c: usize, drawn: &mut HashMap<usize, (f64, f64)>) {
    let s = tree.structure();
    let i = tree.axes(c)[0];
    let Origin::Lateral { parent, node, .. } = s.axes[i].origin else {
        return;
    };
    if let Some(p) = s.axes[i].phytomers.first() {
        drawn.insert(i, (p.radius, s.axes[parent].phytomers[node].radius));
    }
}

/// The chains in the order `order` resolves them, each after its bearer.
fn sequence(tree: &Bounded, order: &str, seed: u64) -> Vec<usize> {
    let n = tree.chains();
    match order {
        "top-down" | "fly-in" => (0..n).collect(),
        _ => {
            // Any chain whose bearer is resolved, drawn at random.
            let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
            for c in 1..n {
                children[tree.parent(c).expect("borne")].push(c);
            }
            let mut ready = vec![0usize];
            let mut out = Vec::with_capacity(n);
            let mut state = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1;
            while !ready.is_empty() {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let k = (state % ready.len() as u64) as usize;
                let c = ready.swap_remove(k);
                out.push(c);
                ready.extend(&children[c]);
            }
            out
        }
    }
}

fn perf(label: &str, species: &Species, request: Request, expected: &Expected) {
    let started = Instant::now();
    let reference = grow(species, request).expect("the reference grows");
    let reference_s = started.elapsed().as_secs_f64();
    let phytomers = reference.phytomer_count();
    drop(reference);
    let rss0 = stats::rss();
    let mut tree = Bounded::with(species, request, expected).expect("settles");
    let (settled, _) = tree.timings();
    let (rows, held) = expected.cost();
    let rss_settled = stats::rss();
    // Single chains, top-down on one core.
    let mut times = Vec::with_capacity(tree.chains());
    for c in 0..tree.chains() {
        let t = Instant::now();
        tree.resolve(c).expect("resolves");
        times.push(t.elapsed().as_secs_f64() * 1e6);
    }
    let one: f64 = times.iter().sum::<f64>() / 1e6;
    let rss_full = stats::rss();
    let (p50, p99, max) = stats::tail(&mut times);
    let mut sizes: Vec<f64> = (0..tree.chains()).map(|c| tree.phytomers(c) as f64).collect();
    let (s50, s99, smax) = stats::tail(&mut sizes);
    let per = settled / phytomers as f64 * 1e6;
    let mut again = Bounded::with(species, request, expected).expect("settles");
    let t = Instant::now();
    again.resolve_all(1).expect("resolves");
    let one_pass = t.elapsed().as_secs_f64();
    let mut again = Bounded::with(species, request, expected).expect("settles");
    let t = Instant::now();
    again.resolve_all(0).expect("resolves");
    let every = t.elapsed().as_secs_f64();
    println!(
        "{label}: {phytomers} phytomers, {} chains; reference grown in {reference_s:.2} s",
        tree.chains()
    );
    println!(
        "  grow and settle {settled:.2} s (whole topology, not per chain); closed-form rows {rows:.3} s, {} rows ({} kB)",
        held,
        held * 72 / 1024
    );
    println!(
        "  chain sizes p50 {s50:.0} p99 {s99:.0} max {smax:.0} phytomers; growth and settling {per:.2} us a phytomer, so growth alone would add about {:.0} / {:.0} / {:.0} us at those sizes",
        s50 * per, s99 * per, smax * per
    );
    println!(
        "  chain resolution us: p50 {p50:.1} p99 {p99:.1} max {max:.0}; all chains one by one {one:.3} s; one-pass on 1 core {one_pass:.3} s, on every core {every:.3} s"
    );
    let path = fly_in(&tree, 0);
    let flight = camera::fly(species, request, expected, &path);
    println!(
        "  fly-in 40 m to 5 cm, 120 frames: first frame {} chains ({} phytomers) in {:.1} ms; after it worst frame {} chains in {:.2} ms, total {:.1} ms; {:.0} % of phytomers resolved",
        flight.first.0, flight.first.1, flight.first.2, flight.worst.0, flight.worst.2, flight.rest, flight.share * 100.0
    );
    let (hero_chains, hero_phytomers) = hero(&tree);
    let bytes = |n: usize| n as f64 * 2.0 * 128.0 / 1e6;
    println!(
        "  memory: RSS settled +{:.0} MB, fully resolved +{:.0} MB over the process before; resolved geometry at the hero view {hero_chains} chains, {hero_phytomers} phytomers ({:.0} MB at two lays of 128 B), fully {:.0} MB",
        (rss_settled - rss0) / 1e6,
        (rss_full - rss0) / 1e6,
        bytes(hero_phytomers),
        bytes(phytomers)
    );
}
