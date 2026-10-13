//! fn-270 task 1: the premises of growth that continues, on the CPU cycle,
//! for one shipped preset at its default age, one process a tree so each
//! peak memory is its own (PREMISES.md).
//!
//!   cargo run --profile ci -p telperion-core --example premises -- <step> <preset> <seed>
//!
//! - `kept`: R1 (c), retired at the age grown against today's retirement:
//!   the kept tree to the bit, and the live set per cycle.
//! - `split`: R1 (d), paused at 10, 40 and the year before the horizon,
//!   settled twice and advanced by no year there, then grown on.
//! - `walk`: R1 (b), the tree settled at every year of one run, against the
//!   run retiring nothing and today's retirement; and how far each
//!   year's tree moves from the last, against today's trees of each age.
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Instant;
use telperion_core::presets::Preset;
use telperion_space::{grow_census, grow_cycle, tree_digest, Census, Light, Paused, Request, Retiring, Species, Structure};

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
    let (p, _) = PRESETS.iter().find(|p| p.1 == args[1]).expect("a shipped preset");
    let family = p.parameters();
    let species = family.space.species.expect("an engine species");
    let seed = args[2].parse().expect("a seed");
    let r = request(family.age as u32, seed, family.space.light);
    println!("{} age {} seed {seed}", args[1], r.age);
    let started = Instant::now();
    match args[0].as_str() {
        "kept" => kept(&species, r),
        "split" => split(&species, r),
        "walk" => walk(&species, r),
        _ => panic!("kept, split or walk"),
    }
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let peak = status.lines().find(|l| l.starts_with("VmHWM")).unwrap_or("VmHWM: unknown");
    println!("  {:.1} s, {peak}", started.elapsed().as_secs_f64());
}

fn request(age: u32, seed: u64, light: Light) -> Request {
    Request { age, seed, budget: 20_000_000, light }
}

/// R1 (c): the kept tree and the live set, today's against the age grown.
fn kept(species: &Species, r: Request) {
    let today = grow_census(species, r).expect("grown");
    let held_today = held(today.census.as_ref().expect("a census"));
    let digest = tree_digest(&today.structure);
    drop(today);
    let mut p = Paused::begin(species, r, Retiring::AtAge, true).expect("begun");
    p.grow_to(r.age).expect("grown");
    let held_age = held(p.census().expect("a census"));
    let tree = p.settle().expect("settled");
    let rises: Vec<f64> = (held_age.iter().zip(&held_today)).map(|(&a, &t)| a as f64 / t.max(1) as f64).collect();
    let (worst, at) = rises.iter().enumerate().fold((0.0, 0), |m, (k, &x)| if x > m.0 { (x, k + 1) } else { m });
    let peak = |h: &[usize]| h.iter().copied().max().unwrap_or(0);
    println!(
        "  KEPT same {}; live peak today {} at age {}; rise at peak {:.4}; worst cycle {at} {worst:.4}",
        tree_digest(&tree) == digest,
        peak(&held_today),
        peak(&held_age),
        peak(&held_age) as f64 / peak(&held_today) as f64,
    );
    println!("  LIVE today {held_today:?}");
    println!("  LIVE atage {held_age:?}");
}

/// Each cycle's phytomers held.
fn held(c: &Census) -> Vec<usize> {
    c.cycles.iter().map(|c| c.phytomers).collect()
}

/// R1 (d): paused, settled twice, advanced by none, grown on.
fn split(species: &Species, r: Request) {
    let today = tree_digest(&grow_cycle(species, r).expect("grown").structure);
    let mut years: Vec<u32> = [10, 40, r.age - 1].into_iter().filter(|&a| a > 0 && a < r.age).collect();
    years.dedup();
    for a in years {
        let mut p = Paused::begin(species, r, Retiring::AtAge, false).expect("begun");
        p.grow_to(a).expect("grown");
        let before = p.digest();
        let first = tree_digest(&p.settle().expect("settled"));
        p.grow_to(a).expect("none grown");
        let again = tree_digest(&p.settle().expect("settled"));
        let moved = p.digest() != before;
        p.grow_to(r.age).expect("grown on");
        let on = tree_digest(&p.settle().expect("settled"));
        println!("  SPLIT {a}: settles repeat {}, grower moved {moved}, grown on is today's {}", first == again, on == today);
    }
}

/// R1 (b): every year's tree of one run, retired at the age grown, against
/// the run retiring nothing and today's; how far each year moves.
fn walk(species: &Species, r: Request) {
    let small = r.age <= 40;
    let mut p = Paused::begin(species, r, Retiring::AtAge, false).expect("begun");
    let mut digests = Vec::new();
    let mut last: Option<Structure> = None;
    let mut moved = Vec::new();
    for y in 1..=r.age {
        p.grow_to(y).expect("grown");
        // The grower's digest is costly on the large trees: `split` holds
        // them to it at three years.
        let before = small.then(|| p.digest());
        let tree = p.settle().expect("settled");
        assert_eq!(small.then(|| p.digest()), before, "a settle at {y} moved the grower");
        digests.push(tree_digest(&tree));
        if let Some(prev) = &last {
            moved.push(movement(prev, &tree));
        }
        last = Some(tree);
    }
    drop((p, last));
    let never = walk_digests(species, r, Retiring::Never);
    let same = never.iter().zip(&digests).filter(|(a, b)| a == b).count();
    println!("  WALK at-age against none retired: {same} of {} years the same", r.age);
    let horizon = walk_digests(species, r, Retiring::AtHorizon);
    let first = horizon.iter().zip(&digests).position(|(a, b)| a != b).map(|k| k + 1);
    let failed = horizon.iter().filter(|d| **d == 0).count();
    println!("  WALK today's retirement: first year apart {first:?}, settles that failed {failed}");
    print_moves("horizon", &moved);
    if small {
        let mut moved = Vec::new();
        let mut last: Option<Structure> = None;
        for y in 1..=r.age {
            let tree = grow_cycle(species, Request { age: y, ..r }).expect("grown").structure;
            if let Some(prev) = &last {
                moved.push(movement(prev, &tree));
            }
            last = Some(tree);
        }
        print_moves("today's", &moved);
    }
}

/// Every year's tree's digest, none where its settle failed.
fn walk_digests(species: &Species, r: Request, retiring: Retiring) -> Vec<u64> {
    let mut p = Paused::begin(species, r, retiring, false).expect("begun");
    (1..=r.age)
        .map(|y| {
            p.grow_to(y).expect("grown");
            let settled = catch_unwind(AssertUnwindSafe(|| p.settle()));
            settled.ok().and_then(Result::ok).map_or(0, |t| tree_digest(&t))
        })
        .collect()
}

/// How far a year moves the phytomers the year before held, each known
/// by its axis's lineage and place: the most an old one's scale moved,
/// how many moved by over a quarter, how many of over a quarter went, and
/// of those how many went with their whole axis (shed); and the lineages
/// two axes share.
#[derive(Debug, Clone, Copy, Default)]
struct Moved {
    most: f64,
    jumped: usize,
    gone: usize,
    shed: usize,
    shared: usize,
}

fn movement(prev: &Structure, now: &Structure) -> Moved {
    let mut m = Moved::default();
    let mut at: HashMap<u64, &[telperion_space::Phytomer]> = HashMap::new();
    for a in &now.axes {
        m.shared += usize::from(at.insert(a.lineage, &a.phytomers).is_some());
    }
    for a in &prev.axes {
        let now = at.get(&a.lineage);
        for (k, q) in a.phytomers.iter().enumerate() {
            match now.and_then(|p| p.get(k)) {
                Some(p) => {
                    let d = (p.scale - q.scale).abs();
                    m.most = m.most.max(d);
                    m.jumped += usize::from(d > 0.25);
                }
                None if q.scale > 0.25 => {
                    m.gone += 1;
                    m.shed += usize::from(now.is_none());
                }
                None => {}
            }
        }
    }
    m
}

fn print_moves(label: &str, moved: &[Moved]) {
    let most = moved.iter().map(|m| m.most).fold(0.0, f64::max);
    let jumped: usize = moved.iter().map(|m| m.jumped).sum();
    let gone: usize = moved.iter().map(|m| m.gone).sum();
    let shed: usize = moved.iter().map(|m| m.shed).sum();
    let shared: usize = moved.iter().map(|m| m.shared).sum();
    println!("  MOVES {label}: most {most:.4}, over a quarter {jumped}, gone over a quarter {gone} ({shed} shed), lineages shared {shared}");
    let years: Vec<String> = moved.iter().enumerate().map(|(k, m)| format!("{}:{:.3}/{}/{}", k + 2, m.most, m.jumped, m.gone)).collect();
    println!("  MOVES {label} by year {}", years.join(" "));
}
