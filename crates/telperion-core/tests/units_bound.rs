//! fn-276 (host, 2026-10-14): the closed form over the age table,
//! `telperion_space::most_units`, bounds the units one job grows in a
//! cycle, a living apex's and a waking bud's, as the CPU's census counts
//! them: every shipped preset at every whole age from 1 to its own and to
//! 80, and the film's walk frames, at seeds 1, 7 and 42. A single case
//! under is a stop. `cargo test --profile ci -p telperion-core --test
//! units_bound -- --ignored --nocapture`; `UNITS_PRESETS` names a comma
//! list, `UNITS_AGES` a range of ages, `UNITS_WALKS=0` leaves the walks
//! out and `UNITS_WORKERS` sets the trees grown at once.
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

/// The bound and the most the CPU counted, a living apex's job and a
/// waking bud's, over every cycle of `family`.
fn counted(family: &Family) -> (u32, [u32; 2]) {
    let g = growth(family).expect("a growth");
    let bound = telperion_space::most_units(&g.species);
    let grown = telperion_space::grow_census(&g.species, g.request).expect("grown");
    let census = grown.census.expect("a census");
    let most = census.cycles.iter().fold([0; 2], |m, c| std::array::from_fn(|w| m[w].max(c.jobs[w])));
    (bound, most)
}

#[test]
#[ignore = "a measurement: every preset at every age, on the CPU"]
fn the_closed_form_bounds_every_job_the_cpu_grows() {
    let only = std::env::var("UNITS_PRESETS").ok();
    let mut cases: Vec<(String, Family)> = Vec::new();
    for id in CATALOGUE.iter().map(|c| c.1).filter(|id| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *id))) {
        let own = by_identity(id).expect("a shipped preset").age.round() as u32;
        // `UNITS_AGES` "a-b": those ages alone (a run resumed).
        let ages = std::env::var("UNITS_AGES").ok().and_then(|r| r.split_once('-').and_then(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?))));
        let (from, to) = ages.unwrap_or((1, own.max(80)));
        for age in from..=to.min(own.max(80)) {
            for seed in SEEDS {
                cases.push((format!("{id} {age} {seed}"), at(id, f64::from(age), seed)));
            }
        }
    }
    if std::env::var("UNITS_WALKS").map_or(true, |w| w != "0") {
        for (a, b, age) in WALKS {
            for seed in SEEDS {
                let (x, y) = (at(a, age, seed), at(b, age, seed));
                for t in T {
                    cases.push((format!("{a} to {b} at {age} t {t} {seed}"), families(&x, &y, t).expect("a frame")));
                }
            }
        }
    }
    let workers = std::env::var("UNITS_WORKERS").ok().and_then(|w| w.parse().ok()).unwrap_or(4);
    let next = std::sync::atomic::AtomicUsize::new(0);
    let under = std::sync::Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for _ in 0..workers {
            s.spawn(|| loop {
                let k = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some((name, family)) = cases.get(k) else { break };
                let (bound, most) = counted(family);
                println!("{name}: bound {bound}, counted {} living, {} waking", most[0], most[1]);
                if most.iter().any(|&m| m > bound) {
                    under.lock().unwrap().push(format!("{name}: bound {bound} under {most:?}"));
                }
            });
        }
    });
    let under = under.into_inner().unwrap();
    assert!(under.is_empty(), "the closed form is under the CPU's count: {under:#?}");
}

