//! fn-277 R3, the law (host ruling 5a, 2026-10-14): along every walk the
//! film draws, each leaf form's expected count per bearer, computed from
//! the walked rows alone and never rounded, changes by degree: by at most
//! `BOUND` of its largest value along the walk per step of 0.001 in t.
//! The draw is held to this law apart (`walk_leaves.rs`). A switch in the
//! law, such as the rosette's old takeover of the wood (the wood's leaves
//! per station from one to none at the first frond, all of its range in
//! one step), is refused; a steep but continuous slope is not.
use telperion_core::{catalogue, presets::by_identity, Family};

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
/// catalogue's rows by their own walks, the engine's by its. The size
/// solve moves only the lateral probabilities, which no form reads.
fn walked(a: &Family, b: &Family, t: f64) -> Family {
    let mut f = a.clone();
    catalogue::walk(a, b, t, &mut f);
    let (x, y) = (a.space.species.as_ref().unwrap(), b.space.species.as_ref().unwrap());
    f.space.species = Some(telperion_space::blend(x, y, t).unwrap());
    f
}

/// Each form's expected count per bearer at one point of a walk, by name.
fn law(f: &Family) -> Vec<(String, f64)> {
    let c = &f.canopy;
    let each = leaflets(f);
    let skirt = if c.skirt_length > 0.0 { c.skirt_fronds } else { 0.0 };
    let grouped = c.rachis_length > 0.0 && c.leaflet_count > 1.0;
    let mut out = vec![
        ("wood leaves per station".to_string(), c.cauline * each),
        ("crown leaves per apex".to_string(), (c.rosette_fronds + skirt) * each),
        ("spines per frond".to_string(), if grouped && c.acanthophyll_length > 0.0 { c.acanthophylls } else { 0.0 }),
        ("bases per stem".to_string(), if c.leaf_base_length > 0.0 { c.leaf_bases } else { 0.0 }),
    ];
    for (k, s) in f.space.species.iter().flat_map(|s| s.states.iter().enumerate()) {
        out.push((format!("cluster leaves per short shoot, age {k}"), s.cluster_leaves * each * c.cauline));
    }
    out
}

/// The steepest step of every form along one walk, as a share of the
/// form's largest value there; a form that is none all along is dormant.
fn steepest(from: &str, to: &str, age: f64, seed: u32) -> Vec<(String, f64, f64)> {
    let end = |id: &str| {
        let mut f = by_identity(id).unwrap();
        (f.age, f.skeleton.seed) = (age, seed);
        f
    };
    let (a, b) = (end(from), end(to));
    let points: Vec<Vec<(String, f64)>> = (0..=STEPS)
        .map(|i| law(&walked(&a, &b, i as f64 / STEPS as f64)))
        .collect();
    let mut out = Vec::new();
    for (k, (name, _)) in points[0].iter().enumerate() {
        let series: Vec<f64> = points.iter().map(|p| p[k].1).collect();
        let most = series.iter().fold(0.0f64, |m, v| m.max(v.abs()));
        if most == 0.0 {
            continue;
        }
        let (step, at) = (1..series.len())
            .map(|i| ((series[i] - series[i - 1]).abs() / most, i as f64 / STEPS as f64))
            .fold((0.0, 0.0), |m, s| if s.0 > m.0 { s } else { m });
        out.push((name.clone(), step, at));
    }
    out
}

#[test]
fn every_film_walk_changes_its_leaf_forms_by_degree() {
    // Each walk and seed at once.
    let walks: Vec<_> = WALKS.iter().flat_map(|w| SEEDS.map(|seed| (*w, seed))).collect();
    let found: Vec<_> = std::thread::scope(|scope| {
        let jobs: Vec<_> = (walks.iter())
            .map(|&((from, to, age), seed)| scope.spawn(move || ((from, to, age, seed), steepest(from, to, age, seed))))
            .collect();
        jobs.into_iter().map(|j| j.join().unwrap()).collect()
    });
    let mut refused = Vec::new();
    for ((from, to, age, seed), forms) in found {
        for (form, step, at) in forms {
            println!("{from} to {to} at {age}, seed {seed}: {form} {step:.5} at t {at:.3}");
            if step > BOUND {
                refused.push(format!("{from} to {to}, seed {seed}: {form} {step:.4} at t {at:.3}"));
            }
        }
    }
    assert!(refused.is_empty(), "{refused:#?}");
}
