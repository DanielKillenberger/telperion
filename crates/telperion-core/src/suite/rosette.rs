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
    assert_eq!(apices(&tree).len(), stems(&f));
    // fn-120's skirt hangs its dead fronds under the same crown.
    assert_eq!(
        crown.placed(),
        stems(&f)
            * (f.canopy.rosette_fronds + f.canopy.skirt_fronds) as usize
            * f.canopy.leaflet_count as usize,
        "a frond crown is stems x fronds x leaflets and nothing besides"
    );
}

/// R3: no foliage is borne below the apex. Every leaflet stands within one
/// rosette's depth and one arched rachis of a stem apex: the arch lifts the
/// rachis out of its straight line by up to `rachis_arch` of its length.
#[test]
fn no_leaflet_stands_below_the_frond_crown() {
    let f = family("date-palm", 1);
    let (tree, crown) = placed(&f);
    let apices = apices(&tree);
    // The skirt continues the crown's spacing below its oldest living frond.
    let last = f64::from(f.canopy.rosette_fronds + f.canopy.skirt_fronds - 1);
    let spacing = f.canopy.rosette_depth / f64::from(f.canopy.rosette_fronds - 1);
    let rachis = f.canopy.rachis_length * f.canopy.rachis_arch.hypot(1.0);
    let reach = spacing * last + rachis;
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
        .filter(|n| n.kind == NodeKind::Twig)
        .count();
    assert_eq!(
        twigs, 0,
        "the palm's apex carries a twig layer under its fronds"
    );
    // fn-110 hangs the retained leaf bases on the stem as wood of their own,
    // so the count above is the twig layer's rather than every node that is
    // not scaffold. Nothing at all stands above an apex, which is the claim.
    let apices: Vec<u32> = tree.stem_apices().iter().map(|&i| i as u32).collect();
    assert!(
        !tree
            .nodes
            .iter()
            .any(|n| n.parent.is_some_and(|p| apices.contains(&p))),
        "wood stands above a stem apex, under the fronds"
    );
}

/// The rosette absent, the family clothes its wood as it always did - the twig
/// layer back, the crown borne on it. The palm's own column is too narrow to
/// bear a twig crown, so the wood is grown in the default envelope.
#[test]
fn the_rosette_absent_leaves_the_wood_clothed_as_it_was() {
    let mut f = family("date-palm", 1);
    f.canopy.rosette_fronds = 0;
    f.skeleton.envelope = Family::default().skeleton.envelope;
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
    refuse(|f| f.canopy.leaf_bases = 257);
    refuse(|f| f.canopy.leaf_base_length = 11.);
    refuse(|f| f.canopy.leaf_base_radius = 1.5);
    refuse(|f| f.canopy.leaf_base_pitch = 181.);
    refuse(|f| f.canopy.leaf_base_weathering = -0.1);
    refuse(|f| f.canopy.acanthophylls = 257);
    refuse(|f| f.canopy.acanthophyll_length = 1.5);
    refuse(|f| f.canopy.acanthophyll_pitch = 91.);
    refuse(|f| f.canopy.skirt_fronds = 129);
    refuse(|f| f.canopy.skirt_pitch = 181.);
    refuse(|f| f.canopy.skirt_length = 1.5);
    refuse(|f| f.canopy.frond_establishment = 101.);
    refuse(|f| f.canopy.frond_early_share = 1.5);
}

/// R3 (fn-110): the first leaflets of a frond are borne as spines. Each one is
/// drawn at its own share of the leaflet it replaces and leaves the rachis at
/// its own pitch, and every other leaflet on the frond is the leaflet it was:
/// the spine moves no draw, so the crown either side of it is untouched.
#[test]
fn the_first_leaflets_of_a_frond_are_borne_as_spines() {
    let f = family("date-palm", 1);
    let mut blades = f.clone();
    blades.canopy.acanthophylls = 0;
    let (_, without) = placed(&blades);
    let (_, with) = placed(&f);
    assert_eq!(with.leaves.len(), without.leaves.len());
    let per = f.canopy.leaflet_count as usize;
    let spines = f.canopy.acanthophylls as usize;
    assert!(spines > 0 && spines < per);
    let scale = |crown: &foliage::Instances, i: usize| crown.reference.scale(crown.leaves[i]);
    for i in 0..with.leaves.len() {
        let (borne, blade) = (scale(&with, i), scale(&without, i));
        if i % per < spines {
            assert!(
                (borne - blade * f.canopy.acanthophyll_length).abs() < blade * 0.02,
                "leaflet {i} is borne at {borne}, not the spine's share of {blade}"
            );
        } else {
            assert_eq!(
                with.leaves[i], without.leaves[i],
                "leaflet {i} is no spine and moved anyway"
            );
        }
    }
}

/// The spines absent, the frond is the frond it was: the column scale is not
/// applied at all, so nothing is multiplied by one.
#[test]
fn no_spine_is_borne_where_the_rows_state_none() {
    let mut f = family("date-palm", 1);
    f.canopy.acanthophylls = 0;
    let (_, blades) = placed(&f);
    let mut length = f.clone();
    length.canopy.acanthophyll_length = 0.;
    length.canopy.acanthophylls = 6;
    let (_, none) = placed(&length);
    assert_eq!(
        blades.leaves, none.leaves,
        "a spine of no length is no spine at all"
    );
}

/// The stems a table stands on: one unless every tree forks at the root.
fn stems(f: &telperion_core::presets::Family) -> usize {
    assert_eq!(f.skeleton.habit.codominance, 0.0, "the table forks");
    1
}

/// fn-270 R5b (host): the palm makes its living crown's count over a
/// frond's life. A frond made in a year before stands whole all year, one
/// of this year's grows from nothing, and across the turn of the year no
/// frond changes size.
#[test]
fn last_years_fronds_stay_whole_and_the_year_turns_with_no_pop() {
    use telperion_core::{foliage::Crown, tree::Year};
    let f = family("date-palm", 1);
    let year = |age: f64| Year { age, budburst: 0.0, elongation: 226.0, expansion: 139.0 };
    let crown = |age: f64| Crown::new(Some(year(age)), &f.canopy);
    let per_year = f64::from(f.canopy.rosette_fronds) / f.canopy.frond_life;
    for age in [11.001, 11.3, 11.6, 11.999] {
        let c = crown(age);
        let before = (per_year * 11.0).floor() as i64;
        assert!((0..before).all(|s| c.unfolded(s as f64) == 1.0), "{age}: an old frond shrank");
        assert!(c.made() - per_year * 11.0 <= per_year, "{age}: more than a year's fronds");
    }
    // In its first year a crown draws only the fronds it has made.
    for age in [0.05, 0.1, 0.5, 1.0] {
        let c = crown(age);
        assert!(c.slots() as f64 <= c.made().ceil(), "{age}: {} fronds of {} made", c.slots(), c.made());
    }
    let (end, start) = (crown(11.0), crown(11.0 + 1e-9));
    for s in 0..(end.made().ceil() as i64) {
        let (a, b) = (end.unfolded(s as f64), start.unfolded(s as f64));
        assert!((a - b).abs() < 1e-6, "frond {s}: {a} at the year's end, {b} after it");
    }
}

/// fn-275 R2, R4: a young palm makes each frond at a share of a mature
/// one set by the crown's age when it made it, from the early share to
/// whole over its establishment, and the frond keeps that size: each new
/// frond is no smaller than the last, and no year turns a frond's size.
#[test]
fn a_young_palms_fronds_are_made_small_and_keep_their_size() {
    use telperion_core::{foliage::Crown, tree::Year};
    let f = family("date-palm", 1);
    let (early, years) = (f.canopy.frond_early_share, f.canopy.frond_establishment);
    assert!(years > 0.0 && early < 1.0, "the palm states its establishment");
    let year = |age: f64| Year { age, budburst: 0.0, elongation: 226.0, expansion: 139.0 };
    let crown = |age: f64| Crown::new(Some(year(age)), &f.canopy);
    let per_year = f64::from(f.canopy.rosette_fronds) / f.canopy.frond_life;
    let (c, mut last) = (crown(12.0), 0.0);
    for s in 0..(per_year * 12.0) as i64 {
        let share = c.established(s as f64);
        assert!(share >= last, "frond {s} at {share}, smaller than the one before at {last}");
        last = share;
    }
    let first = c.established(0.0);
    assert!(first >= early && first < early + 0.01, "the first frond at {first}");
    // A frond of year 3 is sized by the age it emerges at, the first day
    // the crown draws it.
    let serial = (per_year * 2.5) as i64;
    let emerges = (0..365).map(|d| 2.0 + f64::from(d) / 365.0).find(|&age| {
        crown(age + 1e-9).unfolded(serial as f64) > 0.0
    });
    let born = emerges.expect("the frond emerges in its year");
    let expected = 1.0 - (1.0 - early) * (1.0 - telperion_space::unfold::course(born / years));
    let share = c.established(serial as f64);
    assert!((share - expected).abs() < 2e-3, "frond {serial} at {share}, emerged at {born} for {expected}");
    let mature = (per_year * years.ceil()).ceil() as i64;
    assert_eq!(c.established(mature as f64), 1.0, "a frond made after establishment is whole");
    // The same frond at every age it stands, across each year's turn.
    for s in 0..mature {
        let at = |age: f64| crown(age).established(s as f64);
        let made = at(12.0);
        for age in [2.0, 2.0 + 1e-9, 3.5, 6.0, 6.0 + 1e-9, 40.0] {
            assert_eq!(at(age), made, "frond {s} changed size at {age}");
        }
    }
}
