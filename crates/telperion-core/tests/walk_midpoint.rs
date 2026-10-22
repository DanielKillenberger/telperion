//! fn-308 R1 (owner, 2026-10-22: "there's no branches still"): what a walk
//! frame draws between its ends. Measurements, all ignored: the lateral
//! share of the drawn wood and the lateral count against t, the trace of
//! one frame's axes by age, and the frames' wire objects for the stills.
//!
//! `cargo test --profile ci -p telperion-core --test walk_midpoint --
//! --ignored --nocapture --test-threads 1`. `MID_WALKS` names walk indices
//! ("3" is oak to palm), `MID_N` the frames (t = i / n), `MID_SEEDS` the
//! seeds, `MID_T` the traced and written frames, `MID_OUT` the wire's
//! directory.
use telperion_core::blend::families;
use telperion_core::params;
use telperion_core::pipeline::executor::growth;
use telperion_core::presets::{by_identity, Family};
use telperion_space::{Origin, Species, Structure, Vec3};

/// The film's walks (fn-272's `release.json`), as `units_bound` names them.
const WALKS: [(&str, &str, f64); 6] = [
    ("european-beech", "norway-spruce", 40.0),
    ("norway-spruce", "silver-birch", 40.0),
    ("silver-birch", "oregon-white-oak", 40.0),
    ("oregon-white-oak", "date-palm", 40.0),
    ("date-palm", "laurelin", 1.0),
    ("laurelin", "telperion", 80.0),
];

fn at(id: &str, age: f64, seed: u32) -> Family {
    let mut f = by_identity(id).expect("a shipped preset");
    (f.age, f.skeleton.seed) = (age, seed);
    f
}

fn list<T: std::str::FromStr>(name: &str, default: &str) -> Vec<T> {
    let raw = std::env::var(name).unwrap_or_else(|_| default.into());
    raw.split(',')
        .filter_map(|v| v.trim().parse().ok())
        .collect()
}

fn walks() -> Vec<usize> {
    list("MID_WALKS", "0,1,2,3,4,5")
}

fn species(f: &Family) -> &Species {
    f.space.species.as_ref().expect("an engine species")
}

/// The drawn tree: the structure the pipeline grows (`pipeline/space.rs`).
fn drawn(f: &Family) -> Structure {
    let g = growth(f).expect("a growth");
    telperion_space::grow(&g.species, g.request).expect("grown")
}

fn length(base: Vec3, axis: &telperion_space::Axis) -> f64 {
    let mut from = base;
    let mut sum = 0.0;
    for p in &axis.phytomers {
        let (dx, dy, dz) = (p.tip.x - from.x, p.tip.y - from.y, p.tip.z - from.z);
        sum += (dx * dx + dy * dy + dz * dz).sqrt();
        from = p.tip;
    }
    sum
}

/// Each axis's branching order: a lateral one more than its bearer, a
/// continuation or relay its bearer's.
fn orders(s: &Structure) -> Vec<u32> {
    let mut out = vec![0u32; s.axes.len()];
    for (i, a) in s.axes.iter().enumerate() {
        out[i] = match a.origin {
            Origin::Seed => 0,
            Origin::Lateral { parent, .. } => out[parent] + 1,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => out[parent],
        };
    }
    out
}

/// The lateral share of the wood's length, the laterals drawn (a lateral
/// axis with wood), and the whole wood's length in metres.
fn measure(s: &Structure) -> (f64, usize, f64) {
    let order = orders(s);
    let (mut all, mut side, mut count) = (0.0, 0.0, 0);
    for (a, o) in s.axes.iter().zip(&order) {
        let l = length(a.base, a);
        all += l;
        if *o >= 1 {
            side += l;
        }
        if matches!(a.origin, Origin::Lateral { .. }) && l > 1e-6 {
            count += 1;
        }
    }
    (if all > 0.0 { side / all } else { 0.0 }, count, all)
}

/// R1: the lateral share and count against t, each walk, each seed.
#[test]
#[ignore = "a measurement: fn-308 R1's curves"]
fn the_lateral_wood_against_t() {
    let n: u32 = list("MID_N", "40")[0];
    // `MID_FROM` and `MID_TO` narrow the grid to a stretch of the walk.
    let (from, to): (f64, f64) = (list("MID_FROM", "0")[0], list("MID_TO", "1")[0]);
    let seeds: Vec<u32> = list("MID_SEEDS", "1,7,42");
    let rows = std::sync::Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for w in walks() {
            for &seed in &seeds {
                let rows = &rows;
                scope.spawn(move || {
                    let (a, b, age) = WALKS[w];
                    let (x, y) = (at(a, age, seed), at(b, age, seed));
                    for i in 0..=n {
                        let t = from + (to - from) * f64::from(i) / f64::from(n);
                        let f = families(&x, &y, t).expect("a frame");
                        let (share, count, all) = measure(&drawn(&f));
                        let line = format!(
                            "{w} {a}>{b} seed {seed} t {t:.5}: share {share:.4} laterals {count} wood {all:.1} m L2 {:.2}",
                            species(&f).states[2].lifespan
                        );
                        rows.lock().unwrap().push((w, seed, i, line));
                    }
                });
            }
        }
    });
    let mut rows = rows.into_inner().unwrap();
    rows.sort_by_key(|r| (r.0, r.1, r.2));
    for r in rows {
        println!("{}", r.3);
    }
}

/// R1: each end's age-2 lifespan and continuation, each walk; and the scale
/// each timing row mixes on.
#[test]
#[ignore = "a measurement: fn-308 R1's rows"]
fn the_ends_age_two() {
    for name in ["lifespan", "continuation", "relay_ended", "readiness"] {
        println!("{name}: {:?}", telperion_space::scale(name));
    }
    for (a, b, age) in WALKS {
        let (x, y) = (at(a, age, 1), at(b, age, 1));
        let row = |f: &Family| {
            let s = &species(f).states[2];
            format!("lifespan {} continuation {}", s.lifespan, s.continuation)
        };
        let half = families(&x, &y, 0.5).expect("a frame");
        println!(
            "{a} ({}) > {b} ({}) at {age}: t 0.5 {}",
            row(&x),
            row(&y),
            row(&half)
        );
    }
}

/// R1's trace: one frame's axes by age. The main stem's ages, and the
/// laterals grouped by bearer's age and own age: drawn, living and wood.
#[test]
#[ignore = "a measurement: fn-308 R1's trace"]
fn the_frame_traced() {
    let seeds: Vec<u32> = list("MID_SEEDS", "1");
    let ts: Vec<f64> = list("MID_T", "0,0.5");
    let ages: Vec<f64> = list("MID_AGES", "40");
    let (a, b, _) = WALKS[walks()[0]];
    for &seed in &seeds {
        for &age in &ages {
            let (x, y) = (at(a, age, seed), at(b, age, seed));
            for &t in &ts {
                let f = families(&x, &y, t).expect("a frame");
                trace(&format!("{a}>{b} age {age} seed {seed} t {t}"), &f);
            }
        }
    }
}

fn trace(name: &str, f: &Family) {
    let sp = species(f);
    println!("== {name}");
    for (k, s) in sp.states.iter().enumerate() {
        let lat: f64 = s.zones.iter().flat_map(|z| z.lateral.iter()).sum();
        println!(
            "  age {k:2}: lifespan {:8.3} continuation {:.3} relay_ended {:.3} readiness {:.3} shedding {:.2} viability {:.3} laterals-sum {lat:.3}",
            s.lifespan, s.continuation, s.relay_ended, s.readiness, s.shedding, s.viability
        );
    }
    let s = drawn(f);
    let order = orders(&s);
    // The main stem: the seed and what carries it on.
    let mut stem = vec![0usize];
    loop {
        let last = *stem.last().unwrap();
        let next = s.axes.iter().position(|a| {
            matches!(a.origin, Origin::Continuation { parent } | Origin::Relay { parent, .. } if parent == last)
        });
        match next {
            Some(n) => stem.push(n),
            None => break,
        }
    }
    for &i in &stem {
        let a = &s.axes[i];
        println!(
            "  stem axis {i}: age {} birth {} apex_end {:?} phytomers {} wood {:.2} m origin {:?}",
            a.pa,
            a.birth,
            a.apex_end,
            a.phytomers.len(),
            length(a.base, a),
            a.origin
        );
    }
    // Laterals by (bearer's age, own age).
    let mut groups: std::collections::BTreeMap<(usize, usize), (usize, usize, f64, u32)> =
        Default::default();
    for (i, a) in s.axes.iter().enumerate() {
        if let Origin::Lateral { parent, .. } = a.origin {
            let g = groups.entry((s.axes[parent].pa, a.pa)).or_default();
            g.0 += 1;
            if a.apex_end.is_none() {
                g.1 += 1;
            }
            g.2 += length(a.base, a);
            g.3 = g.3.max(order[i]);
        }
    }
    for ((p, q), (n, alive, wood, o)) in groups {
        println!("  laterals of age {p} into age {q}: {n} drawn, {alive} living, {wood:.2} m (orders to {o})");
    }
    let (share, count, all) = measure(&s);
    println!(
        "  share {share:.4} laterals {count} wood {all:.1} m axes {}",
        s.axes.len()
    );
}

/// The frames' wire objects, for `headless --family` (R1's stills).
#[test]
#[ignore = "writes wire objects: fn-308 R1's stills"]
fn the_frames_written() {
    let out = std::path::PathBuf::from(std::env::var("MID_OUT").expect("MID_OUT"));
    std::fs::create_dir_all(&out).unwrap();
    let ts: Vec<f64> = list("MID_T", "0.25,0.5,0.75");
    let seed: u32 = list("MID_SEEDS", "1")[0];
    let (a, b, age) = WALKS[walks()[0]];
    let (x, y) = (at(a, age, seed), at(b, age, seed));
    for t in ts {
        let f = families(&x, &y, t).expect("a frame");
        let path = out.join(format!("{a}-{b}-{age}-s{seed}-t{t}.json"));
        std::fs::write(&path, serde_json::to_string(&params::metadata(&f)).unwrap()).unwrap();
        println!("{}", path.display());
    }
}

/// Every walk's ends at seeds 1, 7 and 42 as `families` returns them, in
/// Debug text, for a byte comparison across a rule (fn-299 R4's check).
#[test]
#[ignore = "writes the walks' ends: fn-308's end check"]
fn the_ends_written() {
    let out = std::path::PathBuf::from(std::env::var("MID_OUT").expect("MID_OUT"));
    std::fs::create_dir_all(&out).unwrap();
    for (a, b, age) in WALKS {
        for seed in [1, 7, 42] {
            let (x, y) = (at(a, age, seed), at(b, age, seed));
            for t in [0.0, 1.0] {
                let f = families(&x, &y, t).expect("an end");
                std::fs::write(
                    out.join(format!("{a}-{b}-s{seed}-t{t}.txt")),
                    format!("{f:?}"),
                )
                .unwrap();
            }
        }
    }
}

/// How much a walk frame grows in each age (the closed form's expected
/// phytomers, `total`), at the ends and a hair inside them: whether a law
/// that refuses an age's rows reads an age the frame barely enters.
#[test]
#[ignore = "a measurement: fn-308 C4's reach at the walk law's refusals"]
fn the_reach_of_each_age_near_the_ends() {
    for w in walks() {
        let (a, b, age) = WALKS[w];
        let (x, y) = (at(a, age, 1), at(b, age, 1));
        for t in [0.0, 0.001, 0.5, 0.999, 1.0] {
            let f = families(&x, &y, t).expect("a frame");
            let s = species(&f);
            let counts = telperion_space::expected_counts(s, age as u32).expect("counts");
            let row: Vec<String> = (0..s.states.len())
                .map(|k| {
                    format!(
                        "{k}:{:.3e}/{:.2}",
                        counts.total(k),
                        s.states[k].cluster_leaves
                    )
                })
                .collect();
            println!("{a}>{b} t {t}: {}", row.join(" "));
        }
    }
}
