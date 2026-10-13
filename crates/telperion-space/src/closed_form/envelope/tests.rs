//! The envelope against grown trees (fn-209, the oracle step): exact on a
//! deterministic tree whose laterals stand in opposite pairs, and the mean
//! of many keyed subtrees on a stochastic one, both straight.
use super::{Envelope, StandIns};
use crate::grow::{grow, Request};
use crate::light::Light;
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::species::{Form, NodeLaw, PaState, Species, Zone};
use crate::structure::{Origin, Structure, Vec3};
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
        flowering: 0.0,
        cluster_leaves: 0.0,
        cluster_spread: 0.0,
        leaf_years: f64::INFINITY,
        elongation: 0.0,
        expansion: 0.0,
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

fn fixed(n: f64) -> NodeLaw {
    NodeLaw::Uniform { min: n, max: n }
}

fn tree(species: &Species, age: u32, seed: u64) -> crate::Result<Structure> {
    let request = Request {
        age,
        seed,
        budget: 10_000_000,
        light: Light::NEUTRAL,
    };
    grow(species, request)
}

/// Each axis's subtree's wood, raw: Σ l, Σ l x and Σ l |x|² in the
/// tree's frame, every axis's carried up to its parent's.
fn raw(tree: &Structure) -> Vec<(f64, Vec3, f64)> {
    let mut sums = vec![(0.0, Vec3::default(), 0.0); tree.axes.len()];
    for (i, axis) in tree.axes.iter().enumerate() {
        let mut from = axis.base;
        for p in &axis.phytomers {
            let l = (p.tip - from).length();
            let x = (from + p.tip) * 0.5;
            sums[i].0 += l;
            sums[i].1 = sums[i].1 + x * l;
            sums[i].2 += l * x.dot(x);
            from = p.tip;
        }
    }
    for i in (1..tree.axes.len()).rev() {
        if let Some(parent) = tree.axes[i].origin.parent() {
            let (w, a, q) = sums[i];
            sums[parent].0 += w;
            sums[parent].1 = sums[parent].1 + a;
            sums[parent].2 += q;
        }
    }
    sums
}

/// Axis `i`'s subtree's envelope about its base along its heading.
fn about(tree: &Structure, sums: &[(f64, Vec3, f64)], i: usize) -> Envelope {
    let (w, a, q) = sums[i];
    let (o, h) = (tree.axes[i].base, tree.axes[i].heading);
    Envelope {
        wood: w,
        along: (a - o * w).dot(h),
        square: q - 2.0 * o.dot(a) + w * o.dot(o),
    }
}

/// A deterministic tree, its laterals in opposite pairs and its trunk
/// relayed in line half way along every unit: the envelope is the tree's.
#[test]
fn the_envelope_is_a_deterministic_trees_wood() {
    let mut trunk = state(3.0, vec![zone(fixed(2.0), 2.0, &[0.0, 0.0, 1.0])]);
    (trunk.continuation, trunk.insertion) = (1.0, 0.0);
    (trunk.abortion, trunk.relay, trunk.relay_at) = (1.0, 1.0, 0.5);
    let bare = zone(fixed(1.0), 2.0, &[0.0, 0.0, 0.0]);
    let medial = zone(fixed(2.0), 2.0, &[0.0, 0.0, 1.0]);
    let mut leader = state(2.0, vec![bare, medial]);
    leader.internode = 0.5;
    let mut twig = state(2.0, vec![zone(fixed(3.0), 2.0, &[0.0; 3])]);
    (twig.insertion, twig.internode) = (0.9, 0.1);
    let species = Species {
        states: vec![trunk, leader, twig],
    };
    for age in [1, 3, 6] {
        let grown = tree(&species, age, 1).expect("grows");
        let sums = raw(&grown);
        let ours = about(&grown, &sums, 0);
        let windows = Windows::new(&species, age);
        let formula =
            StandIns::new(&species, &windows).of(0, age as usize, Carried::default(), None);
        for (what, a, b) in [
            ("wood", ours.wood, formula.wood),
            ("along", ours.along, formula.along),
            ("square", ours.square, formula.square),
        ] {
            assert!(
                // The tree grows in f32 (fn-254): a few of its last places.
                (a - b).abs() <= 1e-6 * b.abs().max(1.0),
                "age {age} {what}: {a} vs {b}"
            );
        }
    }
}

/// A stochastic tree whose laterals roll evenly about their bearers: each
/// lateral's subtree, by its PA and birth, has on average the envelope of
/// a bud of its PA and age. The closed form counts every drawn element
/// whole, and the grower grows one inside its presence window at a share,
/// so these subtrees fall 2 to 6 % short; the tolerance is statistical
/// (fn-209 host decision 10; real twigs are within 1 %).
#[test]
fn the_envelope_is_the_mean_of_keyed_subtrees() {
    let mut trunk = state(
        10.0,
        vec![zone(NodeLaw::Poisson { mean: 6.0 }, 2.5, &[0.0, 0.5, 0.2])],
    );
    (trunk.insertion, trunk.viability, trunk.relay_failed) = (0.0, 0.95, 0.5);
    (trunk.abortion, trunk.relay, trunk.relay_at) = (0.1, 0.6, 0.6);
    let bare = zone(NodeLaw::Uniform { min: 0.0, max: 1.5 }, 1.0, &[0.0; 3]);
    let mut medial = zone(NodeLaw::Poisson { mean: 3.0 }, 1.5, &[0.0, 0.0, 0.6]);
    (medial.dormant, medial.delay, medial.rate) = (vec![0.0, 0.0, 0.3], 1.0, 0.5);
    let mut branch = state(3.5, vec![bare, medial]);
    (branch.continuation, branch.viability, branch.abortion) = (0.7, 0.9, 0.15);
    (branch.insertion, branch.internode) = (0.8, 0.4);
    let mut twig = state(
        2.0,
        vec![zone(
            NodeLaw::Uniform { min: 1.0, max: 3.0 },
            1.0,
            &[0.0; 3],
        )],
    );
    (twig.viability, twig.insertion, twig.internode) = (0.9, 1.0, 0.15);
    let mut species = Species {
        states: vec![trunk, branch, twig],
    };
    for s in &mut species.states {
        s.form.roll = PI;
    }
    let age = 12;
    let windows = Windows::new(&species, age);
    let mut stand_ins = StandIns::new(&species, &windows);
    // By PA and birth: each subtree's envelope per unit base scale.
    let mut groups: std::collections::BTreeMap<(usize, u32), Vec<Envelope>> = Default::default();
    for seed in 0..40 {
        // A seed whose trunk dies at once grows nothing.
        let Ok(grown) = tree(&species, age, seed) else {
            continue;
        };
        let sums = raw(&grown);
        for (i, axis) in grown.axes.iter().enumerate() {
            let Origin::Lateral {
                parent,
                node,
                woken: false,
                ..
            } = axis.origin
            else {
                continue;
            };
            let base = grown.axes[parent].phytomers[node].scale * axis.vigour;
            if base <= 0.0 {
                continue;
            }
            let e = about(&grown, &sums, i);
            let per = Envelope {
                wood: e.wood / base,
                along: e.along / base.powi(2),
                square: e.square / base.powi(3),
            };
            groups.entry((axis.pa, axis.birth)).or_default().push(per);
        }
    }
    let mut tested = 0;
    for ((pa, birth), subtrees) in groups.iter().filter(|(_, v)| v.len() >= 300) {
        let formula = stand_ins.of(*pa, (age - birth) as usize, Carried::default(), None);
        let n = subtrees.len() as f64;
        for (what, value, expected) in [
            (
                "wood",
                (|e: &Envelope| e.wood) as fn(&Envelope) -> f64,
                formula.wood,
            ),
            ("along", |e: &Envelope| e.along, formula.along),
            ("square", |e: &Envelope| e.square, formula.square),
        ] {
            let values: Vec<f64> = subtrees.iter().map(value).collect();
            let mean = values.iter().sum::<f64>() / n;
            let var = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1.0);
            let error = (var / n).sqrt();
            println!("PA {pa} born {birth} ({n}): {what} {mean:.5} ± {error:.5} vs {expected:.5}");
            assert!(
                (mean - expected).abs() <= 4.5 * error + 1e-9,
                "PA {pa} born {birth}: {what} {mean} ± {error} vs {expected}"
            );
        }
        tested += 1;
    }
    assert!(tested >= 4, "too few subtrees to test: {tested}");
}
