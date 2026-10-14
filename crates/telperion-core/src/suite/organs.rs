//! Counts of organs between two whole numbers (fn-277): each bearer draws
//! the whole count either side of its row, so the expected count is the
//! row's, a whole count draws nothing, and the cauline share keeps that
//! share of the leaf-bearing wood.
use telperion_core::{
    foliage::{self, Reference, TwigPlacement},
    mesh,
    presets::{Family, Preset},
};

fn birch() -> Family {
    let mut f = Preset::SilverBirch.parameters();
    f.skeleton.seed = 1;
    f
}

/// The leaves one family places on its own grown skeleton, before any cull.
fn placed(f: &Family) -> foliage::Instances {
    let tree = mesh::grow(f).expect("the skeleton grows");
    let twig = TwigPlacement::of(f).expect("the twig rows resolve");
    let reference = Reference::of(f).expect("the reference box");
    foliage::place(&tree, f.skeleton.envelope, f.skeleton.seed, f.canopy, Some(twig), reference)
        .expect("the crown is placed")
}

/// The cauline share keeps that share of the leaf-bearing wood by its
/// draws: all of it at one, as before, none at zero, about half between.
#[test]
fn the_cauline_share_keeps_its_share_of_the_wood() {
    let all = placed(&birch());
    let at = |share: f64| {
        let mut f = birch();
        f.canopy.cauline = share;
        placed(&f).placed() as f64 / all.placed() as f64
    };
    assert_eq!(placed(&birch()).leaves, all.leaves, "one draws nothing");
    assert_eq!(at(0.0), 0.0);
    let half = at(0.5);
    assert!((0.4..0.6).contains(&half), "{half}");
    assert!(at(0.25) < half && half < at(0.75));
}

/// A fractional leaflet count gives each placement the whole leaflets
/// either side of it, so the crown carries the row's count on the whole.
#[test]
fn a_fractional_leaflet_count_carries_its_share() {
    let one = placed(&birch()).placed() as f64;
    let mut f = birch();
    (f.canopy.leaflet_count, f.canopy.rachis_length) = (2.5, 0.1);
    let ratio = placed(&f).placed() as f64 / one;
    assert!((2.4..2.6).contains(&ratio), "{ratio}");
}

/// A crown stands beside the wood its cauline share clothes: a birch
/// given fronds keeps the leaves along its wood at a share of one, and
/// bears its fronds alone at none.
#[test]
fn a_crown_stands_beside_the_cauline_wood() {
    let mut f = birch();
    f.canopy.rosette_fronds = 12.0;
    let both = placed(&f).placed();
    f.canopy.cauline = 0.0;
    let crown = placed(&f).placed();
    assert!(crown > 0 && both > 10 * crown, "{crown}, {both}");
}

/// The palm's direct build keeps its every frond's leaflets: a frond
/// emerging at no size still stands the leaflets it was made with
/// (fn-270 R5b). The palm at 1, seed 7, was 1,017 leaves before fn-277
/// (FILM.md), every frond 113 leaflets; its seedling fronds are entire
/// (fn-275 R2b), so it is 10.
#[test]
fn an_emerging_frond_keeps_its_leaflets() {
    let mut palm = Preset::DatePalm.parameters();
    (palm.age, palm.skeleton.seed) = (1.0, 7);
    assert_eq!(mesh::build(&palm).unwrap().foliage_instances(), 10);
}

/// An apex that draws no living frond stands no rosette, its skirt with
/// it; and the reference box holds the deepest skirt either whole count
/// of living fronds hangs, so no frond is clamped to its floor.
#[test]
fn a_fractional_crown_stands_within_its_box() {
    let mut palm = Preset::DatePalm.parameters();
    (palm.age, palm.skeleton.seed) = (1.0, 1);
    palm.canopy.frond_life = 0.0;
    palm.canopy.skirt_fronds = 2.0;
    palm.canopy.rosette_fronds = 0.01;
    assert_eq!(mesh::build(&palm).unwrap().foliage_instances(), 0, "no living frond");
    (palm.skeleton.seed, palm.canopy.leaf_bases) = (7, 0.0);
    (palm.canopy.rosette_fronds, palm.canopy.skirt_fronds) = (2.01, 128.0);
    (palm.canopy.rosette_depth, palm.canopy.leaflet_count, palm.canopy.rachis_length) = (100.0, 1.0, 0.0);
    let leaves = mesh::build(&palm).unwrap().foliage.instances;
    assert!(!leaves.leaves.is_empty());
    let floor = leaves.reference.min.y;
    let lowest = leaves.leaves.iter().map(|l| leaves.reference.position(*l).y).fold(f64::INFINITY, f64::min);
    assert!(lowest > floor + 1e-3 * leaves.reference.extent.y, "{lowest} on the floor {floor}");
}
