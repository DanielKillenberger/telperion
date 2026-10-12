//! fn-255, COUNCIL.md's early tests 1 and 2 (2026-10-11).
//!
//! - `three <preset> <age|default> <seed> cycle`: the one cycle with its
//!   census, one a process so it has its own peak memory. F4's builds
//!   beside it went with fn-256.
//!
//!   cargo run --profile ci -p telperion-core --example early -- three norway-spruce default 42 cycle
//!
//! - `live <preset> <age|default> <seed>`: fn-256 task 5's live set, each
//!   cycle's phytomers not retired against those grown by then.
use std::time::Instant;
use telperion_core::presets::Preset;
use telperion_space::{grow_census, Census, Light, Request, Species, Structure};

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
        "live" => live(&grow_census(&species, r).expect("grown").census.expect("a census")),
        "hint" => hint(&species, r.age),
        _ => panic!("three or live"),
    }
}

fn preset(name: &str) -> (Species, Light, f64) {
    let (p, _) = PRESETS.iter().find(|p| p.1 == name).expect("a shipped preset");
    let family = p.parameters();
    (family.space.species.expect("an engine species"), family.space.light, family.age)
}

fn three(species: &Species, r: Request, which: &str) {
    let started = Instant::now();
    assert_eq!(which, "cycle", "F4's builds went with fn-256");
    match grow_census(species, r) {
        Ok(c) => {
            tree("cycle", &c.structure, c.grown, c.seconds);
            census(&c.census.expect("a census"));
        }
        Err(e) => println!("  cycle: {e} after {:.1} s", started.elapsed().as_secs_f64()),
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
    // fn-256: what a busy cycle's device time scales to over the run.
    let sum = |f: &dyn Fn(&telperion_space::Cycle) -> usize| c.cycles.iter().map(f).sum::<usize>();
    let n = c.cycles.len();
    let at = |k: usize| &c.cycles[n - k];
    println!(
        "  over the run: active {}, leaves laid {}, axes held {}; cycle {}: active {}, axes {}; cycle {}: leaves {}",
        sum(&|c| c.active), sum(&|c| c.leaves), sum(&|c| c.axes),
        at(1).cycle, at(1).active, at(1).axes, at(2).cycle, at(2).leaves
    );
    println!(
        "  retired {} subtrees; grown hierarchy: {} axes; ancestors p50/p99/max {:?} (busiest cycle's apexes {:?}); attachments {:?}",
        c.retired, c.axes, c.depth_all, c.depth_busy, c.attachments
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

/// Each cycle's live phytomers (not retired) against those grown by its
/// end, then the largest live set and the largest share past cycle 10.
fn live(c: &Census) {
    println!("  cycle grown live share axes");
    for k in &c.cycles {
        let share = k.phytomers as f64 / k.grown.max(1) as f64;
        println!("  {} {} {} {share:.3} {}", k.cycle, k.grown, k.phytomers, k.axes);
    }
    let peak = c.cycles.iter().max_by_key(|k| k.phytomers).expect("a cycle");
    let share = |k: &telperion_space::Cycle| k.phytomers as f64 / k.grown.max(1) as f64;
    let most = c.cycles.iter().filter(|k| k.cycle > 10).map(share).fold(0.0, f64::max);
    println!(
        "  live set: largest {} in cycle {} ({:.3} of {} grown); largest share past cycle 10 {most:.3}; retired {} subtrees",
        peak.phytomers, peak.cycle, share(peak), peak.grown, c.retired
    );
}

/// The closed form's expected kept phytomers, the light's terms let go
/// where the species reads light, and its seconds (fn-256 task 5's
/// sizing hint).
fn hint(species: &Species, age: u32) {
    let t = Instant::now();
    let mut unshaded = species.clone();
    for s in &mut unshaded.states {
        (s.shade_hazard, s.shade_size, s.balance_hazard, s.leaf_girth) = (0.0, 0.0, 0.0, 0.0);
    }
    let kept = telperion_space::expected_kept(&unshaded, age).expect("a closed form");
    println!("  hint: {:.0} expected kept, unshaded, in {:.3} s", kept.unwrap_or(f64::NAN), t.elapsed().as_secs_f64());
}
