//! The skirt: the dead fronds a rosette keeps below its living crown, hung on
//! the crown's own spiral and marked withered so the crown draws them in the
//! dead colour. fn-120.
use telperion_core::{
    foliage::{self, withered, Reference, TwigPlacement},
    mesh,
    presets::{Family, Preset},
    tree::Tree,
};

fn palm() -> Family {
    let mut f = Preset::from_id("date-palm")
        .expect("the date palm is a preset")
        .parameters();
    f.skeleton.seed = 1;
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

fn bare(f: &Family) -> Family {
    let mut g = f.clone();
    g.canopy.skirt_fronds = 0;
    g
}

/// R1: the skirt is that many fronds after the living crown, every leaf of
/// them withered, and the living crown before them is the crown it was. The
/// box the crown is quantised against deepens with the skirt, so a living
/// leaf keeps its rotation and scale words and moves by no more than the
/// box's own step.
#[test]
fn the_skirt_hangs_its_count_of_withered_fronds_after_the_living_crown() {
    let f = palm();
    assert_eq!(f.skeleton.habit.stems, 1, "one stem, one crown, one skirt");
    let (_, living) = placed(&bare(&f));
    let (_, crown) = placed(&f);
    let per = f.canopy.leaflet_count as usize;
    let dead = f.canopy.skirt_fronds as usize * per;
    assert!(dead > 0);
    assert_eq!(crown.leaves.len(), living.leaves.len() + dead);
    let step = crown
        .reference
        .step()
        .length()
        .max(living.reference.step().length());
    for (i, (&now, &was)) in crown.leaves.iter().zip(&living.leaves).enumerate() {
        assert_eq!(now[0], was[0], "living leaf {i} turned");
        assert_eq!(now[2] >> 16, was[2] >> 16, "living leaf {i} rescaled");
        let moved = crown
            .reference
            .position(now)
            .distance(living.reference.position(was));
        assert!(
            moved <= step,
            "living leaf {i} moved {moved}, past the step {step}"
        );
    }
    assert!(living.leaves.iter().all(|&l| !withered(l)));
    assert!(crown.leaves[living.leaves.len()..]
        .iter()
        .all(|&l| withered(l)));
}

/// R1: the dead droop. Each dead frond leaves the axis below the oldest
/// living one and hangs down from there, so every leaflet of the skirt stands
/// further down the stem's axis than the lowest living insertion.
#[test]
fn every_dead_leaflet_hangs_below_the_living_crown() {
    let f = palm();
    let (tree, crown) = placed(&f);
    let rosette = foliage::rosettes(&tree)[0];
    let from = crown.leaves.len() - (f.canopy.skirt_fronds * f.canopy.leaflet_count) as usize;
    for i in from..crown.leaves.len() {
        let down = (rosette.at - crown.position(i)).dot(rosette.axis);
        assert!(
            down > f.canopy.rosette_depth,
            "dead leaflet {i} stands {down} down the axis, above the living crown"
        );
    }
}

/// R1: a dead frond is drawn at its share of a living one's length. The same
/// frond at half the share is the same draw at half the scale.
#[test]
fn the_length_share_scales_every_dead_leaflet() {
    let mut full = palm();
    full.canopy.skirt_length = 1.0;
    let mut half = full.clone();
    half.canopy.skirt_length = 0.5;
    let (_, a) = placed(&full);
    let (_, b) = placed(&half);
    assert_eq!(a.leaves.len(), b.leaves.len());
    let from = a.leaves.len() - (full.canopy.skirt_fronds * full.canopy.leaflet_count) as usize;
    for i in from..a.leaves.len() {
        let (long, short) = (
            a.reference.scale(a.leaves[i]),
            b.reference.scale(b.leaves[i]),
        );
        assert!(
            (short - long * 0.5).abs() < long * 0.01,
            "leaflet {i} is {short} at half the share of {long}"
        );
    }
}

/// R1's neutral: at a count of zero, or a length of zero, no dead frond is
/// kept and the crown is the living one alone.
#[test]
fn a_skirt_of_no_count_or_no_length_draws_nothing() {
    let f = palm();
    let (_, living) = placed(&bare(&f));
    let mut unlengthed = f.clone();
    unlengthed.canopy.skirt_length = 0.0;
    let (_, none) = placed(&unlengthed);
    assert_eq!(living.leaves, none.leaves);
}

/// The withered bit rides the scale's sign, which no scale uses: a withered
/// leaf decodes to the transform it was packed from.
#[test]
fn a_withered_leaf_decodes_to_the_same_transform() {
    let (_, mut crown) = placed(&bare(&palm()));
    let before: Vec<_> = crown.matrices().collect();
    crown.wither(0);
    assert!(crown.leaves.iter().all(|&l| withered(l)));
    assert_eq!(before, crown.matrices().collect::<Vec<_>>());
}
