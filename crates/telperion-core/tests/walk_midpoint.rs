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

/// R3's base, oak to palm at 40 before the rule (commit 911262a6e,
/// `R1.md`): per seed, the largest log-wood step on the 0.025 grid, and
/// the frames of 41 whose wood falls below both ends'.
const R3_BASE: [(u32, f64, usize); 3] = [(1, 0.756, 11), (7, 0.900, 10), (42, 0.913, 5)];

/// One seed's R3 measures: the largest log-wood step, the frames below
/// both ends, and the largest share step at 0.0025 over the steepest
/// stretch (the coarse step of the largest share change, ± 0.05).
fn by_degree(seed: u32) -> (f64, usize, f64) {
    let (a, b, age) = WALKS[3];
    let (x, y) = (at(a, age, seed), at(b, age, seed));
    // t = k / 400: the coarse grid is every tenth k.
    let frame = |k: u32| {
        let f = families(&x, &y, f64::from(k) / 400.0).expect("a frame");
        let (share, _, wood) = measure(&drawn(&f));
        (share, wood)
    };
    let coarse: Vec<(f64, f64)> = (0..=40).map(|i| frame(10 * i)).collect();
    let largest = |values: &[f64]| {
        values.windows(2).map(|w| (w[1] - w[0]).abs()).fold(0.0, f64::max)
    };
    let logs: Vec<f64> = coarse.iter().map(|c| c.1.ln()).collect();
    let floor = coarse[0].1.min(coarse[40].1);
    let below = coarse.iter().filter(|c| c.1 < floor).count();
    let shares: Vec<f64> = coarse.iter().map(|c| c.0).collect();
    let step = |i: usize| (shares[i + 1] - shares[i]).abs();
    let steepest = (0..40).max_by(|&i, &j| step(i).total_cmp(&step(j))).expect("40 steps");
    let k = 10 * steepest as u32;
    let fine: Vec<f64> = (k.saturating_sub(20)..=(k + 30).min(400))
        .map(|k| frame(k).0)
        .collect();
    (largest(&logs), below, largest(&fine))
}

/// fn-308 R3, "by degree" as the host ruled it (2026-10-22): on oak to
/// palm at 40, seeds 1, 7 and 42, (a) the largest log-wood step is at
/// most 1.1 times the base's, (b) no share step over the steepest stretch
/// exceeds 0.2, and (c) no more frames fall below both ends than on the
/// base. One thread per seed: R1's unbounded probe hit the 8 GB cap.
#[test]
#[ignore = "a measurement: fn-308 R3, about a minute of growth"]
fn the_walk_to_the_palm_changes_by_degree() {
    let measured: Vec<(u32, f64, usize, (f64, usize, f64))> = std::thread::scope(|scope| {
        let runs: Vec<_> = R3_BASE
            .map(|(seed, step, below)| (seed, step, below, scope.spawn(move || by_degree(seed))))
            .into_iter()
            .collect();
        runs.into_iter()
            .map(|(seed, step, below, run)| (seed, step, below, run.join().expect("a seed")))
            .collect()
    });
    let mut refused = Vec::new();
    for (seed, base_step, base_below, (step, below, share)) in measured {
        println!(
            "seed {seed}: (a) log-wood step {step:.3} (base {base_step}, bound {:.3}) (b) share step {share:.3} (bound 0.2) (c) below both ends {below} (base {base_below})",
            1.1 * base_step
        );
        if step > 1.1 * base_step {
            refused.push(format!("seed {seed} (a) {step:.3}"));
        }
        if share > 0.2 {
            refused.push(format!("seed {seed} (b) {share:.3}"));
        }
        if below > base_below {
            refused.push(format!("seed {seed} (c) {below}"));
        }
    }
    assert!(refused.is_empty(), "R3 refuses {refused:?}");
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

/// What a frame's reach costs: `families` against the closed form, per walk.
#[test]
#[ignore = "a measurement: fn-308's reach cost"]
fn the_reach_cost() {
    for w in walks() {
        let (a, b, age) = WALKS[w];
        let (x, y) = (at(a, age, 1), at(b, age, 1));
        let clock = std::time::Instant::now();
        let frames: Vec<Family> = (1..20).map(|i| families(&x, &y, f64::from(i) / 20.0).unwrap()).collect();
        let built = clock.elapsed().as_secs_f64() / 19.0;
        let clock = std::time::Instant::now();
        for f in &frames {
            telperion_space::expected_counts(species(f), age as u32).unwrap();
        }
        let counted = clock.elapsed().as_secs_f64() / 19.0;
        println!("{a}>{b}: families {built:.4} s, closed form {counted:.4} s a frame");
    }
}

fn digest(text: &str) -> u64 {
    text.bytes()
        .fold(0xcbf2_9ce4_8422_2325, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3))
}

/// The built tree's fingerprint, or its refusal.
fn built(f: &Family) -> String {
    match telperion_core::mesh::build(f) {
        Ok(m) => format!("{}", digest(&format!("{m:?}"))),
        Err(e) => format!("ERR {e}"),
    }
}

/// The host's premise (2026-10-22): a lifespan at or past the tree's age
/// is dormant, so clamping it changes no single species' tree. Date palm
/// and beech at 40, seed 1.
#[test]
#[ignore = "a measurement: fn-308 the clamp's premise"]
fn a_lifespan_past_the_age_is_dormant() {
    let palm = at("date-palm", 40.0, 1);
    for l in [150.0, 41.0, 40.0, 39.0] {
        let mut f = palm.clone();
        f.space.species.as_mut().unwrap().states[2].lifespan = l;
        println!("date palm at 40, age-2 lifespan {l}: {}", built(&f));
    }
    let beech = at("european-beech", 40.0, 1);
    let mut f = beech.clone();
    let mut touched = Vec::new();
    for (k, s) in f.space.species.as_mut().unwrap().states.iter_mut().enumerate() {
        if s.lifespan >= 40.0 {
            touched.push((k, s.lifespan));
            s.lifespan = 40.0;
        }
    }
    println!("beech at 40: {}", built(&beech));
    println!("beech at 40, lifespans >= 40 set to 40 {touched:?}: {}", built(&f));
}

/// The reader: beech to spruce at age 0.5, t = 0.5, clamp on (as the walk
/// blends) and off (rule B alone), grown one cycle: the first quantity
/// that differs.
#[test]
#[ignore = "a measurement: fn-308 what reads a dormant lifespan"]
fn what_reads_a_dormant_lifespan() {
    let (mut a, mut b) = (at("european-beech", 0.5, 1), at("norway-spruce", 0.5, 1));
    let on = families(&a, &b, 0.5).unwrap();
    let on_species = species(&on).clone();
    let off_species = telperion_space::blend(species(&a), species(&b), 0.5, 2_000_000).unwrap();
    let bare = telperion_space::blend(species(&a), species(&b), 0.5, 0).unwrap();
    println!("families' species is the blend at age 0 unsized: {}", on_species == bare);
    for (k, (x, y)) in on_species.states.iter().zip(&off_species.states).enumerate() {
        if format!("{x:?}") != format!("{y:?}") {
            println!("  age {k}: lifespan on {} off {}", x.lifespan, y.lifespan);
        }
    }
    let mut off = on.clone();
    off.space.species = Some(off_species.clone());
    println!("built at 0.5: on {} off {}", built(&on), built(&off));
    // Grown one whole cycle by the engine itself.
    (a.age, b.age) = (1.0, 1.0);
    let request = growth(&{
        let mut f = on.clone();
        f.age = 1.0;
        f
    })
    .unwrap()
    .request;
    println!("request {request:?}");
    let diff: Vec<usize> = (0..on_species.states.len())
        .filter(|&k| format!("{:?}", on_species.states[k]) != format!("{:?}", bare.states[k]))
        .collect();
    println!("families' species differs from the bare clamped blend at ages {diff:?}");
    let g_bare = telperion_space::grow(&bare, request).unwrap();
    println!("grown at 1, bare clamped blend: {} axes {}", g_bare.axes.len(), digest(&format!("{g_bare:?}")));
    for (name, s) in [("on", &on_species), ("off", &off_species)] {
        let c = telperion_space::expected_counts(s, 1).unwrap();
        let totals: Vec<String> = (0..s.states.len())
            .filter(|&k| c.total(k) > 0.0)
            .map(|k| format!("{k}:{:.4}", c.total(k)))
            .collect();
        println!("closed form at 1, {name}: {}", totals.join(" "));
    }
    let (g_on, g_off) = (
        telperion_space::grow(&on_species, request).unwrap(),
        telperion_space::grow(&off_species, request).unwrap(),
    );
    println!(
        "grown at 1: on {} axes {}, off {} axes {}",
        g_on.axes.len(),
        digest(&format!("{g_on:?}")),
        g_off.axes.len(),
        digest(&format!("{g_off:?}"))
    );
    for (i, (x, y)) in g_on.axes.iter().zip(&g_off.axes).enumerate() {
        if format!("{x:?}") != format!("{y:?}") {
            println!("  first axis that differs: {i}");
            println!("    on:  pa {} birth {} apex_end {:?} origin {:?} vigour {} phytomers {}", x.pa, x.birth, x.apex_end, x.origin, x.vigour, x.phytomers.len());
            println!("    off: pa {} birth {} apex_end {:?} origin {:?} vigour {} phytomers {}", y.pa, y.birth, y.apex_end, y.origin, y.vigour, y.phytomers.len());
            let p = x.phytomers.iter().zip(&y.phytomers).position(|(p, q)| p != q);
            if let Some(j) = p {
                println!("    first phytomer that differs: {j}: on {:?}", x.phytomers[j]);
                println!("                                 off {:?}", y.phytomers[j]);
            }
            break;
        }
    }
}
