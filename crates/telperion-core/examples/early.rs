//! fn-255, COUNCIL.md's early tests 1 and 2 (2026-10-11).
//!
//! - `three <preset> <age|default> <seed> <skip|grow|cycle>`: one of the
//!   three builds, one a process so each has its own peak memory: F4's
//!   build with laterals certain to be shed skipped (today) or grown, and
//!   the one cycle with its census.
//! - `deposit <preset> <age> <seed>`: the quantised deposit against f32,
//!   first read beside it on the same leaves and buds every cycle, then
//!   driving growth.
//!
//!   cargo run --profile ci -p telperion-core --example early -- three norway-spruce default 42 cycle
use std::time::Instant;
use telperion_core::presets::Preset;
use telperion_space::{
    agree, grow_build, grow_census, grow_cycle, Census, Deposit, Light, Request, Species, Structure,
};

const PRESETS: [(Preset, &str); 8] = [
    (Preset::EuropeanBeech, "european-beech"),
    (Preset::OregonWhiteOak, "oregon-white-oak"),
    (Preset::NorwaySpruce, "norway-spruce"),
    (Preset::DatePalm, "date-palm"),
    (Preset::SilverBirch, "silver-birch"),
    (Preset::Ordinary, "ordinary"),
    (Preset::Laurelin, "laurelin"),
    (Preset::Telperion, "telperion"),
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (species, light, default) = preset(&args[1]);
    let age = args[2].parse().unwrap_or(default as u32);
    let seed = args[3].parse().expect("a seed");
    let r = Request { age, seed, budget: 20_000_000, light };
    println!("{} age {age} seed {seed}", args[1]);
    match args[0].as_str() {
        "three" => three(&species, r, &args[4]),
        "deposit" => deposit(&species, r),
        _ => panic!("three or deposit"),
    }
}

fn preset(name: &str) -> (Species, Light, f64) {
    let (p, _) = PRESETS.iter().find(|p| p.1 == name).expect("a shipped preset");
    let family = p.parameters();
    (family.space.species.expect("an engine species"), family.space.light, family.age)
}

fn three(species: &Species, r: Request, which: &str) {
    let started = Instant::now();
    if which == "cycle" {
        match grow_census(species, r, Deposit::F32, false) {
            Ok(c) => {
                tree("cycle", &c.structure, c.grown, c.seconds);
                census(&c.census.expect("a census"));
            }
            Err(e) => println!("  cycle: {e} after {:.1} s", started.elapsed().as_secs_f64()),
        }
        return;
    }
    match grow_build(species, r, which == "grow") {
        Ok(s) => {
            let grown = s.coarse_grown + s.fine_grown;
            tree(which, &s.structure, grown, started.elapsed().as_secs_f64());
            let cycles: f64 = s.years.iter().sum();
            let busiest = s.years.iter().cloned().fold(0.0, f64::max);
            println!(
                "  phases: structure above the cut, its cycles {cycles:.2} s (slowest {busiest:.3} s); roots grown alone and the tree settled {:.2} s; roots {}, grown above the cut {} and in roots {}",
                s.settle, s.roots, s.coarse_grown, s.fine_grown
            );
        }
        Err(e) => println!("  {which}: {e} after {:.1} s", started.elapsed().as_secs_f64()),
    }
}

/// The kept tree's size, ancestors and attachments.
fn tree(label: &str, s: &Structure, grown: u64, seconds: f64) {
    let mut depth = vec![0u32; s.axes.len()];
    let mut born = vec![0u32; s.axes.len()];
    for (i, a) in s.axes.iter().enumerate() {
        if let Some(p) = a.origin.parent() {
            depth[i] = depth[p] + 1;
            born[p] += 1;
        }
    }
    let mut bearing: Vec<u32> = born.into_iter().filter(|&b| b > 0).collect();
    println!(
        "  {label}: grown {grown}, kept {} phytomers on {} axes, {seconds:.2} s; kept ancestors p50/p99/max {:?}, attachments {:?}",
        s.phytomer_count(), s.axes.len(), quantiles(&mut depth), quantiles(&mut bearing)
    );
}

fn quantiles(v: &mut [u32]) -> [u32; 3] {
    if v.is_empty() {
        return [0; 3];
    }
    v.sort_unstable();
    let at = |q: f64| v[((v.len() - 1) as f64 * q).round() as usize];
    [at(0.5), at(0.99), v[v.len() - 1]]
}

fn census(c: &Census) {
    let mut phases: Vec<_> = c.phases.iter().collect();
    phases.sort_by(|a, b| b.1.total_cmp(a.1));
    let line: Vec<String> = phases.iter().take(40).map(|(k, v)| format!("{k} {v:.2}")).collect();
    println!("  phases (s): {}", line.join(", "));
    let busy = &c.cycles[c.busy as usize - 1];
    let last = c.cycles.last().expect("a cycle");
    println!(
        "  records: busiest cycle {} holds {} active apexes against {} axes and {} phytomers of history; at the end {} active, {} axes, {} phytomers",
        busy.cycle, busy.active, busy.axes, busy.phytomers, last.active, last.axes, last.phytomers
    );
    println!(
        "  grown hierarchy: {} axes; ancestors p50/p99/max {:?} (busiest cycle's apexes {:?}); attachments {:?}",
        c.axes, c.depth_all, c.depth_busy, c.attachments
    );
    let read = c.cycles.iter().filter(|c| c.nodes > 0);
    if let Some(big) = read.clone().max_by_key(|c| c.nodes) {
        let (n, occupied) = read.fold((0usize, 0usize), |(n, o), c| (n + c.nodes, o + c.occupied));
        println!(
            "  lattice: largest in cycle {}: {:?} nodes = {}, {} occupied ({:.1}%) by {} leaves; over the run {:.1}% occupied",
            big.cycle, big.size, big.nodes, big.occupied, 100.0 * big.occupied as f64 / big.nodes as f64, big.leaves,
            100.0 * occupied as f64 / n as f64
        );
    }
}

fn deposit(species: &Species, r: Request) {
    let frozen = grow_census(species, r, Deposit::F32, true).unwrap();
    let c = frozen.census.expect("a census");
    let (mut buds, mut sum, mut most, mut differ) = (0usize, 0.0, 0.0f64, 0usize);
    for k in &c.cycles {
        (buds, sum, most, differ) = (buds + k.buds, sum + k.sum, most.max(k.most), differ + k.differ);
    }
    println!(
        "  frozen (f32 drives growth, quantised read beside): {buds} buds, mean |diff| {:.2e}, most {most:.2e}, buds differing {differ} ({:.2}%)",
        sum / buds.max(1) as f64, 100.0 * differ as f64 / buds.max(1) as f64
    );
    for k in c.cycles.iter().filter(|k| k.buds > 0 && k.most > 0.0).take(3) {
        println!("    cycle {}: {} buds, most {:.2e}, differing {}", k.cycle, k.buds, k.most, k.differ);
    }
    let quantised = grow_cycle(species, r, Deposit::Quantised).unwrap();
    for (label, s, g) in [("f32", &frozen.structure, frozen.grown), ("quantised", &quantised.structure, quantised.grown)] {
        tree(label, s, g, f64::NAN);
    }
    let a = agree::agreement(&frozen.structure, &quantised.structure);
    println!("  through feedback: identical {}; within agree::PATHS {}; {a:?}", frozen.structure == quantised.structure, a.within(&agree::PATHS));
    // The same measure between two f32 builds that differ only in their
    // sums' order: F4's build against the cycle.
    let (f4, ..) = telperion_space::grow_marked(species, r).unwrap();
    let b = agree::agreement(&f4, &frozen.structure);
    println!("  F4's build against the f32 cycle: identical {}; within agree::PATHS {}; {b:?}", f4 == frozen.structure, b.within(&agree::PATHS));
}
