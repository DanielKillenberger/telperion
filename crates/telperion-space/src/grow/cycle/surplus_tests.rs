//! fn-364 R2: a living tip extends by what its balance leaves of what it
//! earns. Its unit's internodes are laid at its surplus share, its leaf at
//! the allotment's size; a tip charged nothing is laid as before.
use super::balance_tests::lateral;
use super::passes::surplus;
use super::{grow_cycle, Paused, Working};
use crate::geometry::Layer;
use crate::grow::bud;
use crate::grow::draw::draw_bent;
use crate::grow::sketch::Pencil;
use crate::light::{leaf_area, Light};
use crate::lineage::Key;
use crate::presence::{Draws, Shade, Unit};
use crate::species::Form;
use crate::structure::{Growing, Node, Origin, Vec3};
use crate::Request;
use std::collections::HashMap;

fn request(age: u32, seed: u64) -> Request {
    let light = Light {
        extinction: 0.5,
        sky: 0.5,
    };
    Request { age, seed, light }
}

/// A lateral 8 m above the root collar at a potential height of 32 m:
/// (h · inverse height)² = 1/16 of what it earns per unit of light.
const INVERSE: f32 = 1.0 / 32.0;

fn high() -> Vec<crate::structure::Axis<Growing>> {
    lateral(0.25, &[[2.0, 0.0, 8.25]])
}

/// The share is 1 − (h · inverse height)² / light: 7/8 in half the light,
/// none where the charge reaches the light or passes it, none in the
/// dark, and whole where the tip is charged nothing (no potential height,
/// or a tip at the collar's height), whatever its light.
#[test]
fn a_tip_keeps_the_share_its_balance_leaves() {
    assert_eq!(surplus(&high(), 1, INVERSE, 0.5), 0.875);
    assert_eq!(surplus(&high(), 1, INVERSE, 0.0625), 0.0);
    assert_eq!(surplus(&high(), 1, INVERSE, 0.05), 0.0);
    assert_eq!(surplus(&high(), 1, INVERSE, 0.0), 0.0);
    assert_eq!(surplus(&high(), 1, 0.0, 0.5), 1.0);
    assert_eq!(surplus(&high(), 1, 0.0, 0.0), 1.0);
    let collar = lateral(0.25, &[[2.0, 0.0, 0.25]]);
    assert_eq!(surplus(&collar, 1, INVERSE, 0.5), 1.0);
    assert_eq!(surplus(&collar, 1, INVERSE, 0.0), 1.0);
}

/// A seed of three phytomers in one unit, its internode 1 m and its unit
/// sized 0.5, laid upward from 1 m at `stretch`: its phytomers and the
/// path its tip has laid.
fn laid(stretch: f32) -> (Vec<Growing>, f32) {
    let mut species = crate::beech();
    let state = &mut species.states[0];
    (state.internode, state.form) = (1.0, Form::default());
    let mut axis = bud(Key(1), 0, 0, Origin::Seed);
    axis.phytomers = (0..3).map(|k| Growing::new(1, 11 + k)).collect();
    let mut draws = Draws::made([1.0, 1.0]);
    draws.units.push(Unit {
        survive: 1.0,
        persist: 1.0,
        share: 1.0,
        size: 1.0,
    });
    draws.nodes = vec![1.0; 3];
    draws.shade.push(Shade { size: 0.5, stretch });
    let (up, x) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
    let mut pencil = Pencil {
        layer: Layer::bent((Vec3::new(0.0, 0.0, 1.0), up, x), Vec3::default(), 1.0),
        base_scale: 1.0,
        running: 1.0,
        rank: 0.0,
        units: 0,
        laid: 0,
        trunk: true,
        light: 1.0,
        size: 1.0,
        stretch: 1.0,
        memory: 1.0,
        kept: 1.0,
        root: 0.0,
        path: Vec::new(),
    };
    draw_bent((&mut axis, &mut pencil), &draws, &species, (0, 0.0)).unwrap();
    (axis.phytomers, pencil.path_end())
}

/// Each phytomer's leaf, as the deposit and the wood read it.
fn leaves(phytomers: &[Growing]) -> Vec<f64> {
    let leaf = |p: &Growing| leaf_area(p.scale(), p.size(), 1.0);
    phytomers.iter().map(leaf).collect()
}

/// A tip 8 m up in half the light lays its unit at 7/8 of the length the
/// allotment gives it (three nodes of 0.5 m, 1.5 m, laid 1.3125 m), every
/// node 7/8 as high above its base, and bears the same leaf.
#[test]
fn a_unit_is_laid_at_its_share_with_its_leaf_whole() {
    let share = surplus(&high(), 1, INVERSE, 0.5);
    let (whole, length) = laid(1.0);
    let (short, shortened) = laid(share);
    assert_eq!(length, 1.5);
    assert_eq!(shortened, 1.5 * share);
    for (k, p) in short.iter().enumerate() {
        let up = p.tip[2] - 1.0;
        assert!((up - 0.4375 * (k + 1) as f32).abs() < 1e-6, "node {k} {up}");
    }
    assert_eq!(leaves(&short), leaves(&whole));
    assert!(leaves(&short).iter().all(|&l| l == 0.5));
}

/// A tip whose charge passes its light lays its unit at no length: every
/// node at the unit's base, every phytomer kept with its leaf.
#[test]
fn a_tip_past_its_limit_lays_zero_length_internodes_and_keeps_its_nodes_and_leaves() {
    let share = surplus(&high(), 1, INVERSE, 0.05);
    assert_eq!(share, 0.0);
    let (whole, _) = laid(1.0);
    let (none, length) = laid(share);
    assert_eq!(length, 0.0);
    assert_eq!(none.len(), 3);
    assert!(none.iter().all(|p| p.tip == [0.0, 0.0, 1.0]));
    assert_eq!(leaves(&none), leaves(&whole));
}

/// With no potential height every tip is charged nothing, so every share
/// is whole and the oak is fn-363's to the bit: `fine::digest` pinned on
/// fn-363's product (`49ec1f379`), at 20 in light, seeds 1 and 7.
#[test]
fn without_a_potential_height_the_tree_is_fn_363s() {
    let mut species = crate::oak();
    for state in &mut species.states {
        state.inverse_height = 0.0;
    }
    for (seed, pin) in [(1, 14394896050340871), (7, 5712598232629544231)] {
        let tree = grow_cycle(&species, request(20, seed)).unwrap().structure;
        let stretches = tree.axes.iter().flat_map(|a| &a.phytomers);
        assert!(stretches.map(|p| p.stretch).all(|s| s == 1.0));
        assert_eq!(crate::fine::digest(&tree), pin, "seed {seed}");
    }
}

/// The settle lays every phytomer at the length growth laid it: the oak
/// at 25 in light, with nothing a later year adds (sag, erection,
/// straightening, take-over, shedding), stands where its rough layout put
/// it, every node above 0.5 m within 0.1 mm (9.5e-6 m on fn-363's
/// product), with many of its units shortened.
#[test]
fn the_settled_lengths_are_the_growing_lays() {
    let mut species = crate::oak();
    for state in &mut species.states {
        state.form.sag = 0.0;
        state.erection = 0.0;
        state.straightening = 0.0;
        state.takeover = 0.0;
        state.shedding = f64::INFINITY;
    }
    let r = request(25, 3);
    let mut p = Paused::with(std::sync::Arc::new(species), r, false, Working::indexed()).unwrap();
    p.grow_to(r.age).unwrap();
    let grown = p.g.axes.iter().flat_map(|a| &a.phytomers);
    let at: HashMap<u64, [f32; 3]> = grown.map(|q| (q.key, q.tip)).collect();
    let settled = p.settle().unwrap();
    let mut worst = 0.0f64;
    let mut shortened = 0;
    for q in settled.axes.iter().flat_map(|a| &a.phytomers) {
        shortened += usize::from(q.stretch < 0.99);
        if let (Some(t), true) = (at.get(&q.key), q.tip.z > 0.5) {
            let laid = Vec3::new(f64::from(t[0]), f64::from(t[1]), f64::from(t[2]));
            worst = worst.max((q.tip - laid).length());
        }
    }
    assert!(shortened > 1000, "{shortened} shortened");
    assert!(
        worst < 1e-4,
        "a node above 0.5 m stands {worst} m from its laid place"
    );
}
