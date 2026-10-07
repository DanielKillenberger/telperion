//! The kept closed form against grown trees (fn-209 step 1, host decision
//! 9's oracle): exact on a deterministic tree that sheds, and on a
//! stochastic one the mean over every lateral made, the shed counting
//! nothing, with each subtree's last living cycle read off a twin that
//! never sheds and so keeps every axis it grew.
use super::{Kept, COUNT, LEAVES, LEAVES_ALONG, MASS, MASS_ALONG, SECTION, WOOD};
use crate::grow::{grow, Request};
use crate::light::Light;
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::species::{Form, NodeLaw, PaState, Species, Zone};
use crate::structure::{Origin, Structure, Vec3};
use crate::transcendental::Transcendental;
use std::collections::HashMap;
use std::f64::consts::PI;

fn state(lifespan: f64, zones: Vec<Zone>) -> PaState {
    PaState {
        lifespan,
        continuation: 0.0,
        viability: 1.0,
        zones,
        shedding: f64::INFINITY,
        internode: 0.3,
        insertion: 0.6,
        divergence: 2.4,
        abortion: 0.0,
        abortion_rise: 0.0,
        relay: 0.0,
        relay_ended: 0.0,
        relay_failed: 0.0,
        relay_at: 1.0,
        epitony: 0.0,
        erection: 0.0,
        readiness: 1.0,
        rhythm: 1.0,
        leaf_area: 0.0,
        shade_hazard: 0.0,
        shade_size: 0.0,
        apical_control: 0.5,
        upkeep: 0.0,
        balance_hazard: 0.0,
        tolerance: 0.0,
        retained: 0.0,
        leaf_girth: 0.0,
        straightening: 0.0,
        form: Form::default(),
    }
}

fn zone(nodes: NodeLaw, buds: f64, lateral: &[f64]) -> Zone {
    Zone {
        nodes,
        buds,
        dormant: vec![0.0; lateral.len()],
        delay: 0.0,
        rate: 0.0,
        lateral: lateral.to_vec(),
    }
}

fn tree(species: &Species, age: u32, seed: u64) -> crate::Result<Structure> {
    let request = Request {
        age,
        seed,
        budget: 20_000_000,
        light: Light::NEUTRAL,
    };
    grow(species, request)
}

/// Each axis's subtree, in the tree's frame: Σ l, Σ l x, Σ l |x|², the
/// pipe model's mass Σ l (r² + o²) and its moment Σ l x (r² + o²), and
/// its phytomers.
#[derive(Clone, Copy, Default)]
struct Raw {
    count: f64,
    wood: f64,
    first: Vec3,
    square: f64,
    mass: f64,
    moment: Vec3,
}

fn raw(tree: &Structure, species: &Species) -> Vec<Raw> {
    let mut sums = vec![Raw::default(); tree.axes.len()];
    for (i, axis) in tree.axes.iter().enumerate() {
        let pipe = species.states[axis.pa].form.pipe;
        let mut from = axis.base;
        for p in &axis.phytomers {
            let l = (p.tip - from).length();
            let x = (from + p.tip) * 0.5;
            let own = pipe * p.scale;
            let mass = l * (p.radius * p.radius + own * own);
            let s = &mut sums[i];
            s.count += 1.0;
            s.wood += l;
            s.first = s.first + x * l;
            s.square += l * x.dot(x);
            s.mass += mass;
            s.moment = s.moment + x * mass;
            from = p.tip;
        }
    }
    for i in (1..tree.axes.len()).rev() {
        if let Some(parent) = tree.axes[i].origin.parent() {
            let c = sums[i];
            let s = &mut sums[parent];
            s.count += c.count;
            s.wood += c.wood;
            s.first = s.first + c.first;
            s.square += c.square;
            s.mass += c.mass;
            s.moment = s.moment + c.moment;
        }
    }
    sums
}

/// Axis `i`'s subtree about its base along its heading: wood, along,
/// square, mass, mass moment, its base section in exponent `e`, and its
/// phytomers.
fn about(tree: &Structure, sums: &[Raw], i: usize, e: f64) -> [f64; 7] {
    let s = sums[i];
    let (o, h) = (tree.axes[i].base, tree.axes[i].heading);
    let section = tree.axes[i]
        .phytomers
        .first()
        .map_or(0.0, |p| p.radius.powf_fixed(e));
    [
        s.wood,
        (s.first - o * s.wood).dot(h),
        s.square - 2.0 * o.dot(s.first) + s.wood * o.dot(o),
        s.mass,
        (s.moment - o * s.mass).dot(h),
        section,
        s.count,
    ]
}

/// The formula's [wood, along, square, mass, mass moment, section,
/// phytomers].
fn formula(b: &[f64]) -> [f64; 7] {
    [
        b[WOOD],
        b[super::ALONG],
        b[super::SQUARE],
        b[MASS] + b[LEAVES],
        b[MASS_ALONG] + b[LEAVES_ALONG],
        b[SECTION],
        b[COUNT],
    ]
}

const NAMES: [&str; 7] = [
    "wood",
    "along",
    "square",
    "mass",
    "moment",
    "section",
    "phytomers",
];

/// A deterministic species whose twigs die young and are shed; its trunk
/// relayed half way along every unit where `relayed`, or carried on whole.
fn deterministic(relayed: bool) -> Species {
    let fixed = |n: f64| NodeLaw::Uniform { min: n, max: n };
    let mut trunk = state(9.0, vec![zone(fixed(2.0), 2.0, &[0.0, 1.0, 0.0])]);
    trunk.insertion = 0.0;
    if relayed {
        (trunk.abortion, trunk.relay, trunk.relay_at) = (1.0, 1.0, 0.5);
    }
    let mut branch = state(4.0, vec![zone(fixed(1.0), 2.0, &[0.0, 0.0, 1.0])]);
    branch.internode = 0.5;
    let mut twig = state(2.0, vec![zone(fixed(3.0), 2.0, &[0.0; 3])]);
    (twig.insertion, twig.internode, twig.shedding) = (0.9, 0.1, 1.0);
    Species {
        states: vec![trunk, branch, twig],
    }
}

/// A deterministic tree that sheds keeps what the formula keeps, to
/// rounding, at every age, its trunk carried on whole or relayed half way
/// along every unit: mass and its moment included, since at exponent 2
/// with whole secondary growth the radius's square is linear in the
/// section and the delta method exact.
#[test]
fn a_deterministic_tree_keeps_what_the_formula_keeps() {
    for relayed in [false, true] {
        let species = deterministic(relayed);
        for age in [1, 3, 5, 7, 9] {
            let grown = tree(&species, age, 1).expect("grows");
            let sums = raw(&grown, &species);
            let ours = about(&grown, &sums, 0, 2.0);
            let windows = Windows::new(&species, age);
            let mut kept = Kept::new(&species, &windows);
            let table = kept.of(0, age as usize, Carried::default(), None);
            let (a, _, b) = table.at(age as i64);
            let theirs = formula(b.expect("born"));
            assert!((a - 1.0).abs() < 1e-12);
            for q in 0..7 {
                let (a, b) = (ours[q], theirs[q]);
                assert!(
                    (a - b).abs() <= 1e-9 * b.abs().max(1e-6),
                    "relayed {relayed}, age {age} {}: {a} vs {b}",
                    NAMES[q]
                );
            }
        }
    }
}

/// The stochastic species: a trunk that relays, branches that never shed
/// and twigs that die and are shed after a delay.
fn shedding_species() -> Species {
    let mut trunk = state(
        10.0,
        vec![zone(NodeLaw::Poisson { mean: 5.0 }, 2.5, &[0.0, 0.5, 0.0])],
    );
    (trunk.insertion, trunk.viability, trunk.relay_failed) = (0.0, 0.97, 1.0);
    (trunk.abortion, trunk.relay, trunk.relay_at) = (0.1, 1.0, 0.6);
    let bare = zone(NodeLaw::Uniform { min: 0.0, max: 2.0 }, 1.0, &[0.0; 3]);
    let medial = zone(NodeLaw::Poisson { mean: 3.0 }, 1.5, &[0.0, 0.0, 0.7]);
    let mut branch = state(5.0, vec![bare, medial]);
    (branch.viability, branch.abortion, branch.insertion) = (0.9, 0.1, 0.8);
    (branch.relay, branch.relay_failed, branch.internode) = (0.5, 0.5, 0.4);
    let mut twig = state(
        3.0,
        vec![zone(
            NodeLaw::Uniform { min: 1.0, max: 3.0 },
            1.0,
            &[0.0; 3],
        )],
    );
    (twig.viability, twig.abortion, twig.insertion) = (0.7, 0.2, 1.0);
    (twig.internode, twig.shedding) = (0.15, 1.0);
    let mut species = Species {
        states: vec![trunk, branch, twig],
    };
    for s in &mut species.states {
        s.form.roll = PI;
    }
    species
}

/// Every lateral branch made, by its birth: on average it keeps what the
/// formula keeps (a shed or unmade twig counting nothing), and its
/// subtree's last living cycle falls by each cycle as often as the
/// extinction curve says.
#[test]
fn branches_keep_on_average_what_the_formula_keeps() {
    branches_agree(&shedding_species());
}

/// As above, the branches' pipe exponent above their twigs' (host
/// decision 15: the section handed over by the delta method).
#[test]
fn branches_of_another_exponent_keep_what_the_formula_keeps() {
    let mut species = shedding_species();
    species.states[1].form.exponent = 2.4;
    branches_agree(&species);
    species.states[1].form.exponent = 2.0;
    species.states[2].form.exponent = 2.6;
    branches_agree(&species);
}

fn branches_agree(species: &Species) {
    let species = species.clone();
    let e = species.states[1].form.exponent;
    let mut twin = species.clone();
    twin.states
        .iter_mut()
        .for_each(|s| s.shedding = f64::INFINITY);
    let age = 12u32;
    let windows = Windows::new(&species, age);
    let mut kept = Kept::every_cycle(&species, &windows);
    // By birth: each branch's kept quantities per unit base scale, and its
    // subtree's last living cycle, from its birth.
    let mut groups: HashMap<u32, Vec<([f64; 7], u32)>> = HashMap::new();
    for seed in 0..60 {
        let (Ok(grown), Ok(whole)) = (tree(&species, age, seed), tree(&twin, age, seed)) else {
            continue;
        };
        let sums = raw(&grown, &species);
        // A relay carries its axis's lineage: the first axis of a lineage
        // is the one that began it.
        let mut by_lineage: HashMap<u64, usize> = HashMap::new();
        for (i, a) in grown.axes.iter().enumerate() {
            by_lineage.entry(a.lineage).or_insert(i);
        }
        let last = last_living(&whole);
        for (i, axis) in whole.axes.iter().enumerate() {
            let Origin::Lateral { parent, node, .. } = axis.origin else {
                continue;
            };
            if axis.pa != 1 {
                continue;
            }
            let base = whole.axes[parent].phytomers[node].scale * axis.vigour;
            if base <= 0.0 {
                continue;
            }
            let ours = match by_lineage.get(&axis.lineage) {
                Some(&k) => {
                    let v = about(&grown, &sums, k, e);
                    let p = [1.0, 2.0, 3.0, 3.0, 4.0, e, 0.0];
                    std::array::from_fn(|q| v[q] / base.powf_fixed(p[q]))
                }
                None => [0.0; 7],
            };
            groups
                .entry(axis.birth)
                .or_default()
                .push((ours, last[i] - axis.birth));
        }
    }
    let mut tested = 0;
    let mut births: Vec<_> = groups.keys().copied().collect();
    births.sort_unstable();
    for birth in births {
        let subjects = &groups[&birth];
        if subjects.len() < 200 {
            continue;
        }
        let m = (age - birth) as usize;
        let table = kept.of(1, m, Carried::default(), None);
        let theirs = formula(table.at(m as i64).2.expect("born"));
        let n = subjects.len() as f64;
        for q in 0..7 {
            let values: Vec<f64> = subjects.iter().map(|s| s.0[q]).collect();
            let (mean, error) = moments(&values);
            println!(
                "e {e}, born {birth} ({n}): {} {mean:.5} ± {error:.5} vs {:.5}",
                NAMES[q], theirs[q]
            );
            assert!(
                (mean - theirs[q]).abs() <= 4.5 * error + 1e-9,
                "born {birth}: {} {mean} ± {error} vs {}",
                NAMES[q],
                theirs[q]
            );
        }
        for t in 0..m {
            let share = subjects.iter().filter(|s| s.1 as usize <= t).count() as f64 / n;
            let error = (table.a[t] * (1.0 - table.a[t]) / n).sqrt();
            assert!(
                (share - table.a[t]).abs() <= 4.5 * error + 1e-9,
                "born {birth}: P(T <= {t}) {share} vs {}",
                table.a[t]
            );
        }
        tested += 1;
    }
    assert!(tested >= 4, "too few branches to test: {tested}");
}

/// Each axis's subtree's last living cycle: the latest cycle any axis in
/// it grew a unit.
fn last_living(tree: &Structure) -> Vec<u32> {
    let mut last: Vec<u32> = tree
        .axes
        .iter()
        .map(|a| a.birth + a.units.len() as u32)
        .collect();
    for i in (1..tree.axes.len()).rev() {
        if let Some(p) = tree.axes[i].origin.parent() {
            last[p] = last[p].max(last[i]);
        }
    }
    last
}

fn moments(values: &[f64]) -> (f64, f64) {
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let var = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1.0);
    (mean, (var / n).sqrt())
}

mod angles;
mod bound;
mod measure;
