//! fn-276 (host, 2026-10-14): `telperion_space::most_units`, the units one
//! job grows in a cycle where none of its units fails, the floor the
//! device's units rounds and generations take, against what the CPU's
//! census counts: every shipped preset at every whole age from 1 to its
//! own and to 80, and the film's walk frames, at seeds 1, 7 and 42.
//!
//! `cargo test --profile ci -p telperion-core --test units_bound --
//! --ignored --nocapture`. `UNITS_PRESETS` names a comma list,
//! `UNITS_AGES` a range of ages ("a-b"), `UNITS_WALKS=0` leaves the walks
//! out, `UNITS_WORKERS` sets the trees grown at once, and `UNITS_GROW=0`
//! prints the floors alone.
use telperion_core::blend::families;
use telperion_core::pipeline::executor::growth;
use telperion_core::presets::{by_identity, Family, CATALOGUE};

const SEEDS: [u32; 3] = [1, 7, 42];

/// The film's walks (fn-272's `release.json`): each adjacent pair at its
/// age.
const WALKS: [(&str, &str, f64); 6] = [
    ("european-beech", "norway-spruce", 40.0),
    ("norway-spruce", "silver-birch", 40.0),
    ("silver-birch", "oregon-white-oak", 40.0),
    ("oregon-white-oak", "date-palm", 40.0),
    ("date-palm", "laurelin", 1.0),
    ("laurelin", "telperion", 80.0),
];
const T: [f64; 6] = [0.0, 0.01, 0.1, 0.5, 0.9, 1.0];

fn at(id: &str, age: f64, seed: u32) -> Family {
    let mut f = by_identity(id).expect("a shipped preset");
    (f.age, f.skeleton.seed) = (age, seed);
    f
}

fn floor(family: &Family) -> u32 {
    telperion_space::most_units(&growth(family).expect("a growth").species)
}

/// The floor's units where an apex chains through ages within a cycle:
/// the beech grows its sapling ages in a cycle with the next (2), and a
/// walk frame a hair from it lasts a sliver in the ages the beech passes
/// through, chaining four in one cycle as the CPU grew them (CAUSE.md).
/// Both ends reach those ages, so fn-299's rule leaves them mixed: the
/// pass-through sliver, one of the two mechanisms `SLACK` allows.
#[test]
fn the_floor_counts_the_ages_a_cycle_chains() {
    let (a, b) = (at("european-beech", 40.0, 7), at("norway-spruce", 40.0, 7));
    assert_eq!(floor(&a), 2);
    assert_eq!(floor(&families(&a, &b, 0.00037).expect("a frame")), 4);
}

/// What a walk frame may grow past its ends' most units a cycle (fn-299,
/// host 2026-10-20): the measured worst case once an age one end never
/// reaches takes the other end's settings (R1.md). It allows the two
/// mechanisms that rule leaves: a pass-through sliver, an age one end
/// passes through and the other lives, which counts a whole unit a hair
/// from the passing end; and a mixed outcome, an age that may both stop
/// with a relay and move on, which neither end does.
const SLACK: u32 = 2;

/// Whether `units` lies within the ends' range, `SLACK` past it.
fn within(units: u32, ends: [u32; 2]) -> bool {
    let (low, high) = (ends[0].min(ends[1]), ends[0].max(ends[1]));
    (low.saturating_sub(SLACK)..=high + SLACK).contains(&units)
}

/// A walk frame grows within its ends (fn-299 R1, host 2026-10-20): on oak
/// to palm at 40 the floor at every t lies between the two ends' floors.
/// fn-298's device survey refused t = 0.962 to 0.99998 at 12 units a cycle
/// against the oak's 2 and the palm's 1.
#[test]
fn a_walk_frame_grows_within_its_ends() {
    const T: [f64; 8] = [0.0, 0.5, 0.9, 0.95, 0.962, 0.99, 0.99998, 1.0];
    let mut outside = Vec::new();
    for seed in SEEDS {
        let (x, y) = (
            at("oregon-white-oak", 40.0, seed),
            at("date-palm", 40.0, seed),
        );
        let frame = |t: f64| {
            let f = families(&x, &y, t);
            floor(&f.unwrap_or_else(|e| panic!("oak to palm t {t} seed {seed}: {e}")))
        };
        let ends = [frame(0.0), frame(1.0)];
        for t in T {
            let units = frame(t);
            println!("oak to palm at 40, seed {seed}, t {t}: {units} (ends {ends:?})");
            if !within(units, ends) {
                outside.push(format!("seed {seed} t {t}: {units} past {ends:?}"));
            }
        }
    }
    assert!(
        outside.is_empty(),
        "frames outside their ends: {outside:#?}"
    );
}

/// Every case the sweep builds: name and family.
fn cases() -> Vec<(String, Family)> {
    let only = std::env::var("UNITS_PRESETS").ok();
    let range = std::env::var("UNITS_AGES").ok();
    let range = range
        .as_deref()
        .and_then(|r| r.split_once('-'))
        .and_then(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?)));
    let mut out = Vec::new();
    for id in CATALOGUE
        .iter()
        .map(|c| c.1)
        .filter(|id| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *id)))
    {
        let last = (by_identity(id).expect("a shipped preset").age.round() as u32).max(80);
        let (from, to) = range.unwrap_or((1, last));
        for age in from..=to.min(last) {
            for seed in SEEDS {
                out.push((format!("{id} {age} {seed}"), at(id, f64::from(age), seed)));
            }
        }
    }
    if std::env::var("UNITS_WALKS").map_or(true, |w| w != "0") {
        for (a, b, age) in WALKS {
            for seed in SEEDS {
                let (x, y) = (at(a, age, seed), at(b, age, seed));
                for t in T {
                    out.push((
                        format!("{a} to {b} at {age} t {t} {seed}"),
                        families(&x, &y, t).expect("a frame"),
                    ));
                }
            }
        }
    }
    out
}

/// The floor and the most units the CPU's census counted one living
/// apex's job and one waking bud's grow in a cycle, each case; the cases
/// a failure relay took past the floor, which the device holds by the
/// counts it observes (a measurement: the floor is not a bound).
#[test]
#[ignore = "a measurement: every preset at every age, on the CPU"]
fn the_floor_against_the_units_the_cpu_grows() {
    let cases = cases();
    let grow = std::env::var("UNITS_GROW").map_or(true, |g| g != "0");
    let workers = std::env::var("UNITS_WORKERS")
        .ok()
        .and_then(|w| w.parse().ok())
        .unwrap_or(2);
    let next = std::sync::atomic::AtomicUsize::new(0);
    let (most, past) = (
        std::sync::atomic::AtomicU32::new(0),
        std::sync::Mutex::new(Vec::new()),
    );
    std::thread::scope(|s| {
        for _ in 0..workers {
            s.spawn(|| loop {
                let k = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some((name, family)) = cases.get(k) else {
                    break;
                };
                let g = growth(family).expect("a growth");
                let floor = telperion_space::most_units(&g.species);
                most.fetch_max(floor, std::sync::atomic::Ordering::Relaxed);
                if !grow {
                    println!("{name}: floor {floor}");
                    continue;
                }
                let census = telperion_space::grow_census(&g.species, g.request)
                    .expect("grown")
                    .census
                    .expect("a census");
                let counted = census
                    .cycles
                    .iter()
                    .fold([0; 2], |m, c| std::array::from_fn(|w| m[w].max(c.jobs[w])));
                println!(
                    "{name}: floor {floor}, counted {} living, {} waking",
                    counted[0], counted[1]
                );
                if counted.iter().any(|&c| c > floor) {
                    past.lock()
                        .unwrap()
                        .push(format!("{name}: floor {floor}, counted {counted:?}"));
                }
            });
        }
    });
    println!("the most floor: {}", most.into_inner());
    println!("past the floor: {:#?}", past.into_inner().unwrap());
}
