//! The apical rosette and the pinnate frond: fronds borne only at the apex of
//! an unbranched stem, each one placement expanded into leaflets along a
//! rachis. fn-109.
use telperion_core::{
    branching,
    foliage::{self, Reference, TwigPlacement},
    math::Vec3,
    mesh,
    presets::{Family, Preset},
    tree::{NodeKind, Tree},
};

fn family(id: &str, seed: u32) -> Family {
    let mut f = Preset::from_id(id)
        .unwrap_or_else(|| panic!("{id} is a preset"))
        .parameters();
    f.skeleton.seed = seed;
    f
}

/// The crown one family places on its own grown skeleton, before any cull.
fn placed(f: &Family) -> (Tree, foliage::Instances) {
    let tree = mesh::grow(f).expect("the skeleton grows");
    let crown = foliage::place(
        &tree,
        f.skeleton.envelope,
        f.skeleton.seed,
        f.canopy,
        Some(TwigPlacement::of(f).expect("the twig rows resolve")),
        Reference::of(f).expect("the reference box"),
    )
    .expect("the crown is placed");
    (tree, crown)
}

/// Every apex a rosette stands on, read back from the tree itself.
fn apices(tree: &Tree) -> Vec<Vec3> {
    tree.stem_apices()
        .into_iter()
        .map(|i| tree.nodes[i].position)
        .collect()
}

/// R3: the palm bears one frond crown and nothing else. The count is the
/// rosette's own arithmetic, so a leaf borne anywhere else would show here.
#[test]
fn the_palm_bears_a_frond_for_every_row_its_table_states() {
    let f = family("date-palm", 1);
    let (tree, crown) = placed(&f);
    assert_eq!(apices(&tree).len(), f.skeleton.habit.stems as usize);
    assert_eq!(
        crown.placed(),
        f.skeleton.habit.stems as usize
            * f.canopy.rosette_fronds as usize
            * f.canopy.leaflet_count as usize,
        "a frond crown is stems x fronds x leaflets and nothing besides"
    );
}

/// R3: no foliage is borne below the apex. Every leaflet stands within one
/// rosette's depth and one rachis of a stem apex.
#[test]
fn no_leaflet_stands_below_the_frond_crown() {
    let f = family("date-palm", 1);
    let (tree, crown) = placed(&f);
    let apices = apices(&tree);
    let reach = f.canopy.rosette_depth + f.canopy.rachis_length;
    for i in 0..crown.placed() {
        let at = crown.position(i);
        let nearest = apices
            .iter()
            .map(|a| a.distance(at))
            .fold(f64::INFINITY, f64::min);
        assert!(
            nearest <= reach + 1e-6,
            "leaflet {i} at {at:?} stands {nearest} from the nearest apex, past {reach}"
        );
    }
}

/// R3: an apex that bears a rosette bears no twig. Bare twig wood under the
/// fronds is wood no palm carries.
#[test]
fn an_apex_that_bears_a_rosette_bears_no_twig() {
    let f = family("date-palm", 1);
    let tree = mesh::grow(&f).expect("the skeleton grows");
    let twigs = tree
        .nodes
        .iter()
        .filter(|n| n.kind != NodeKind::Structural)
        .count();
    assert_eq!(
        twigs, 0,
        "the palm's apex carries a twig layer under its fronds"
    );
}

/// The rosette absent, the family clothes its wood as it always did - the twig
/// layer back, the crown borne on it.
#[test]
fn the_rosette_absent_leaves_the_wood_clothed_as_it_was() {
    let mut f = family("date-palm", 1);
    f.canopy.rosette_fronds = 0;
    let (tree, crown) = placed(&f);
    assert!(
        tree.nodes.iter().any(|n| n.kind != NodeKind::Structural),
        "without a rosette the twig layer stands"
    );
    assert!(
        crown.placed() > 0,
        "without a rosette the wood bears leaves"
    );
}

/// R2's positive control: the shipped presets are byte-identical because their
/// tables leave the rosette absent, not because the switch does nothing.
#[test]
fn raising_the_rosette_moves_a_crown_the_default_leaves_where_it_was() {
    let f = family("silver-birch", 1);
    assert_eq!(f.canopy.rosette_fronds, 0);
    assert_eq!(f.canopy.leaflet_count, 1);
    assert_eq!(f.canopy.rachis_length, 0.);
    let (_, before) = placed(&f);
    let mut with = f.clone();
    with.canopy.rosette_fronds = 12;
    with.canopy.leaflet_count = 9;
    with.canopy.rachis_length = 0.4;
    let (tree, after) = placed(&with);
    assert_ne!(
        before.leaves, after.leaves,
        "a rosette raised off zero has to move the crown"
    );
    assert_eq!(
        after.placed(),
        apices(&tree).len() * 12 * 9,
        "twelve fronds of nine leaflets at every stem apex"
    );
}

/// The pinnate grouping is the placement's, not the rosette's alone: a family
/// that states leaflets and a rachis without a rosette draws compound leaves
/// on the wood it already clothed.
#[test]
fn leaflets_expand_a_placement_wherever_the_placement_stands() {
    let f = family("silver-birch", 1);
    let (_, plain) = placed(&f);
    let mut compound = f.clone();
    compound.canopy.leaflet_count = 5;
    compound.canopy.rachis_length = 0.12;
    let (_, grouped) = placed(&compound);
    assert_eq!(
        grouped.placed(),
        plain.placed() * 5,
        "each placement carries its own five leaflets"
    );
}

/// The rosette draws from the apex's own stream, so the same tree read twice
/// is the same crown.
#[test]
fn the_same_palm_is_the_same_crown_twice() {
    let f = family("date-palm", 1);
    let (_, once) = placed(&f);
    let (_, twice) = placed(&f);
    assert_eq!(once.leaves, twice.leaves);
    let other = family("date-palm", 2);
    let (_, moved) = placed(&other);
    assert_ne!(once.leaves, moved.leaves, "another seed is another palm");
}

/// Every new row is refused by its own name, as every row on the rail is.
#[test]
fn every_new_row_is_refused_by_its_own_name() {
    let refuse = |edit: fn(&mut Family)| {
        let mut f = family("date-palm", 1);
        edit(&mut f);
        let tree = branching::generate(&f.skeleton, f.radii).unwrap().tree;
        foliage::place(
            &tree,
            f.skeleton.envelope,
            f.skeleton.seed,
            f.canopy,
            Some(TwigPlacement::of(&f).unwrap()),
            Reference::of(&f).unwrap(),
        )
        .expect_err("the row is off its rail")
    };
    refuse(|f| f.canopy.rosette_fronds = 129);
    refuse(|f| f.canopy.rosette_pitch = 181.);
    refuse(|f| f.canopy.rosette_pitch_spread = -1.);
    refuse(|f| f.canopy.rosette_depth = 101.);
    refuse(|f| f.canopy.leaflet_count = 0);
    refuse(|f| f.canopy.rachis_length = 1001.);
    refuse(|f| f.canopy.leaflet_pitch = 91.);
    refuse(|f| f.canopy.rachis_arch = 1.5);
    refuse(|f| f.canopy.terminal_leaflet = 1.5);
}
