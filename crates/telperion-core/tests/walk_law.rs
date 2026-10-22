//! fn-277 R3, the law (host ruling 5a, 2026-10-14): along every walk the
//! film draws, each leaf form's expected count per bearer, computed from
//! the walked rows alone and never rounded, changes by degree: by at most
//! `BOUND` of its largest value along the walk per step of 0.001 in t.
//! The draw is held to this law apart (`walk_leaves.rs`). A switch in the
//! law, such as the rosette's old takeover of the wood (the wood's leaves
//! per station from one to none at the first frond, all of its range in
//! one step), is refused; a steep but continuous slope is not. A per-age
//! form counts where the frame grows in its age, weighed by that reach
//! (fn-308 host ruling 2).
use telperion_core::{blend::families, catalogue, presets::by_identity, Family};

/// The film's walks (fn-272 cut 3): from, to and age.
const WALKS: [(&str, &str, f64); 6] = [
    ("european-beech", "norway-spruce", 40.0),
    ("norway-spruce", "silver-birch", 40.0),
    ("silver-birch", "oregon-white-oak", 40.0),
    ("oregon-white-oak", "date-palm", 40.0),
    ("date-palm", "laurelin", 1.0),
    ("laurelin", "telperion", 80.0),
];
const SEEDS: [u32; 3] = [1, 7, 42];
const STEPS: usize = 1000;
/// The most a form's expectation moves per step, as a share of its
/// largest value along the walk. Measured on the film's walks (BOUND.md):
/// the steepest is 0.0128, the wood's leaves per station by the palm,
/// where the cauline share falls to none as the leaflets rise on the log
/// scale; a linear row moves 0.001. The rosette's old takeover moved 1.
const BOUND: f64 = 0.02;

/// Leaflets a placement carries on average, unrounded (`organs::leaflets`
/// over its draw): the count where a rachis strings them, else one.
fn leaflets(f: &Family) -> f64 {
    let c = &f.canopy;
    if c.rachis_length > 0.0 && c.leaflet_count > 1.0 {
        c.leaflet_count
    } else {
        1.0
    }
}

/// The rows the law reads at `t` between `a` and `b`, walked as
/// `blend::families` walks them before it sizes the species: the
/// catalogue's rows by their own walks, the engine's by its at the walked
/// whole age (fn-308). The size
/// solve moves only the lateral probabilities, which no form reads.
fn walked(a: &Family, b: &Family, t: f64) -> Family {
    let mut f = a.clone();
    catalogue::walk(a, b, t, &mut f);
    let (x, y) = (
        a.space.species.as_ref().unwrap(),
        b.space.species.as_ref().unwrap(),
    );
    f.space.species = Some(telperion_space::blend(x, y, t, f.age as u32).unwrap());
    f
}

/// Each form's expected count per bearer at one point of a walk, by name,
/// with the age a per-age form reads.
fn law(f: &Family) -> Vec<(String, Option<usize>, f64)> {
    let c = &f.canopy;
    let each = leaflets(f);
    let skirt = if c.skirt_length > 0.0 {
        c.skirt_fronds
    } else {
        0.0
    };
    let grouped = c.rachis_length > 0.0 && c.leaflet_count > 1.0;
    let mut out = vec![
        (
            "wood leaves per station".to_string(),
            None,
            c.cauline * each,
        ),
        (
            "crown leaves per apex".to_string(),
            None,
            (c.rosette_fronds + skirt) * each,
        ),
        (
            "spines per frond".to_string(),
            None,
            if grouped && c.acanthophyll_length > 0.0 {
                c.acanthophylls
            } else {
                0.0
            },
        ),
        (
            "bases per stem".to_string(),
            None,
            if c.leaf_base_length > 0.0 {
                c.leaf_bases
            } else {
                0.0
            },
        ),
    ];
    for (k, s) in f
        .space
        .species
        .iter()
        .flat_map(|s| s.states.iter().enumerate())
    {
        out.push((
            format!("cluster leaves per short shoot, age {k}"),
            Some(k),
            s.cluster_leaves * each * c.cauline,
        ));
    }
    out
}

/// How much a frame grows in each age: the closed form's expected
/// phytomers there over the frame's (fn-308 host ruling 2). It reads the
/// frame as `families` builds it, sized: the size solve's factor on the
/// lateral probabilities moves how much grows in each age, though no form
/// reads those rows (fn-308 completion review).
fn reach(f: &Family) -> Vec<f64> {
    let s = f.space.species.as_ref().unwrap();
    let counts = telperion_space::expected_counts(s, f.age as u32).unwrap();
    let all: f64 = (0..counts.pas).map(|k| counts.total(k)).sum();
    (0..counts.pas).map(|k| counts.total(k) / all).collect()
}

/// One step of a form from `v.0` to `v.1`, as a share of its largest
/// value `most`. A step the rows pass is passed as read; one they would
/// refuse is read again with each value weighed by its frame's reach in
/// the form's age (`weigh`, both frames), computed only then for its cost.
fn step(v: (f64, f64), most: f64, weigh: impl FnOnce() -> (f64, f64)) -> f64 {
    let read = (v.1 - v.0).abs() / most;
    if read <= BOUND {
        return read;
    }
    let (w0, w1) = weigh();
    (v.1 * w1 - v.0 * w0).abs() / most
}

/// The steepest step of every form along one walk, as a share of the
/// form's largest value there; a form that is none all along is dormant.
/// A canopy-wide form weighs one in every frame.
fn steepest(from: &str, to: &str, age: f64, seed: u32) -> Vec<(String, f64, f64)> {
    let end = |id: &str| {
        let mut f = by_identity(id).unwrap();
        (f.age, f.skeleton.seed) = (age, seed);
        f
    };
    let (a, b) = (end(from), end(to));
    let at = |i: usize| i as f64 / STEPS as f64;
    let points: Vec<_> = (0..=STEPS).map(|i| law(&walked(&a, &b, at(i)))).collect();
    let mut reached = std::collections::HashMap::new();
    let mut reach_at = |i: usize, k: usize| -> f64 {
        reached
            .entry(i)
            .or_insert_with(|| reach(&families(&a, &b, at(i)).unwrap()))[k]
    };
    let mut out = Vec::new();
    for (j, (name, pa, _)) in points[0].iter().enumerate() {
        let series: Vec<f64> = points.iter().map(|p| p[j].2).collect();
        let most = series.iter().fold(0.0f64, |m, v| m.max(v.abs()));
        if most == 0.0 {
            continue;
        }
        let mut found = (0.0, 0.0);
        for i in 1..series.len() {
            let s = step((series[i - 1], series[i]), most, || match pa {
                Some(k) => (reach_at(i - 1, *k), reach_at(i, *k)),
                None => (1.0, 1.0),
            });
            if s > found.0 {
                found = (s, at(i));
            }
        }
        out.push((name.clone(), found.0, found.1));
    }
    out
}

/// fn-308 host ruling 2, and fn-277's ruling 5a with it: a whole count's
/// jump in a per-age row passes at an age the frame barely grows in, and
/// the same jump at an age holding most of the frame is refused.
#[test]
fn a_per_age_step_weighs_by_the_frame_reach_in_its_age() {
    assert!(step((0.0, 1.0), 1.0, || (0.001, 0.001)) <= BOUND);
    assert!(step((0.0, 1.0), 1.0, || (0.9, 0.9)) > BOUND);
}

#[test]
fn every_film_walk_changes_its_leaf_forms_by_degree() {
    // Each walk and seed at once.
    let walks: Vec<_> = WALKS
        .iter()
        .flat_map(|w| SEEDS.map(|seed| (*w, seed)))
        .collect();
    let found: Vec<_> = std::thread::scope(|scope| {
        let jobs: Vec<_> = (walks.iter())
            .map(|&((from, to, age), seed)| {
                scope.spawn(move || ((from, to, age, seed), steepest(from, to, age, seed)))
            })
            .collect();
        jobs.into_iter().map(|j| j.join().unwrap()).collect()
    });
    let mut refused = Vec::new();
    for ((from, to, age, seed), forms) in found {
        for (form, step, at) in forms {
            println!("{from} to {to} at {age}, seed {seed}: {form} {step:.5} at t {at:.3}");
            if step > BOUND {
                refused.push(format!(
                    "{from} to {to}, seed {seed}: {form} {step:.4} at t {at:.3}"
                ));
            }
        }
    }
    assert!(refused.is_empty(), "{refused:#?}");
}
