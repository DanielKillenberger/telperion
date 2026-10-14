//! Counts of organs between two whole numbers (fn-277): each bearer draws
//! the whole count either side of its row, so the expected count is the
//! row's, a whole count draws nothing, and the cauline share keeps that
//! share of the leaf-bearing wood.
use telperion_core::{
    foliage::{self, Reference, TwigPlacement},
    mesh,
    pipeline::executor,
    presets::{Family, Preset, CATALOGUE},
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

/// The device draws whole counts only: every shipped preset is whole, and
/// a fraction of any count of organs, or a share between none and all, is
/// not (`executor::whole`).
#[test]
fn only_whole_counts_are_whole() {
    for (_, id, ..) in CATALOGUE {
        let f = telperion_core::presets::by_identity(id).unwrap();
        assert!(executor::whole(&f), "{id}");
    }
    let birch = birch();
    let mut fractions: Vec<Family> = Vec::new();
    for set in [
        |f: &mut Family| f.canopy.rosette_fronds = 0.5,
        |f: &mut Family| f.canopy.leaflet_count = 1.5,
        |f: &mut Family| f.canopy.skirt_fronds = 0.5,
        |f: &mut Family| f.canopy.leaf_bases = 2.5,
        |f: &mut Family| f.canopy.acanthophylls = 0.5,
        |f: &mut Family| f.canopy.cauline = 0.5,
        |f: &mut Family| f.space.species.as_mut().unwrap().states[14].cluster_leaves = 1.5,
    ] {
        let mut f = birch.clone();
        set(&mut f);
        fractions.push(f);
    }
    assert!(fractions.iter().all(|f| !executor::whole(f)));
}
