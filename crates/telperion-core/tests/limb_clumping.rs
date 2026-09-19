//! Limb systems clump their leaves: the row thins every source of leaves
//! toward the walls between limb systems, keeps what it keeps where the
//! stations and the short shoots put it, refuses a value off its rail by
//! name, and reaches the growth view as it reaches a one-shot build.
use telperion_core::{
    branching,
    foliage::{self, CanopyParams, Instances, TwigPlacement},
    presets::{Family, Preset},
    specimen::SpecimenView,
    tree::Tree,
    Error,
};

/// An ordinary tree under a node cap, with short shoots along its limbs so
/// both sources of leaves are present, and no clumping of its own.
fn family(seed: u32) -> Family {
    let mut f = Preset::Ordinary.parameters();
    f.skeleton.seed = seed;
    f.skeleton.growth.max_nodes = Some(12_000);
    f.canopy = CanopyParams {
        short_shoot_spacing: 0.08,
        short_shoot_radius: 0.4,
        short_shoot_leaves: 4,
        limb_clumping: 0.0,
        ..f.canopy
    };
    f
}

fn placed(f: &Family, tree: &Tree, clumping: f64) -> Result<Instances, Error> {
    let t = f.skeleton.twigs.resolved().unwrap().twig;
    let twig = Some(TwigPlacement {
        internode_length: t.internode_length,
        stations_per_internode: t.stations_per_internode,
    });
    let canopy = CanopyParams {
        limb_clumping: clumping,
        ..f.canopy
    };
    foliage::place(
        tree,
        f.skeleton.envelope,
        f.skeleton.seed,
        canopy,
        twig,
        foliage::Reference::of(f).unwrap(),
    )
}

#[test]
fn a_clumped_crown_is_the_whole_crown_thinned_in_order() {
    for seed in [1, 2] {
        let f = family(seed);
        let tree = branching::generate(&f.skeleton, f.radii).unwrap().tree;
        let whole = placed(&f, &tree, 0.0).unwrap();
        let clumped = placed(&f, &tree, 0.5).unwrap();
        assert_eq!(clumped, placed(&f, &tree, 0.5).unwrap(), "seed {seed}");
        let (n, m) = (whole.len(), clumped.len());
        assert!(m < n * 9 / 10, "seed {seed}: {m} of {n} leaves stayed");
        assert!(m > n / 4, "seed {seed}: only {m} of {n} leaves stayed");
        // Every leaf that stays is one the whole crown placed, in its order.
        let mut rest = whole.leaves.iter();
        for leaf in &clumped.leaves {
            assert!(rest.any(|w| w == leaf), "seed {seed}: a leaf moved");
        }
        // A deeper gap keeps fewer.
        assert!(placed(&f, &tree, 1.0).unwrap().len() < m);
    }
}

#[test]
fn a_value_off_the_rail_is_refused_by_name() {
    let f = family(1);
    let tree = branching::generate(&f.skeleton, f.radii).unwrap().tree;
    for bad in [-0.1, 1.5, f64::NAN, f64::INFINITY] {
        assert_eq!(
            placed(&f, &tree, bad).err(),
            Some(Error::InvalidInput("limb clumping")),
            "{bad} was accepted"
        );
    }
}

#[test]
fn the_growth_view_clumps_its_crown_as_well() {
    let mut f = family(3);
    f.age = 14.0;
    let whole = SpecimenView::build(&f).unwrap().mesh().unwrap();
    f.canopy.limb_clumping = 0.5;
    let clumped = SpecimenView::build(&f).unwrap().mesh().unwrap();
    let (n, m) = (
        whole.foliage.instances.len(),
        clumped.foliage.instances.len(),
    );
    assert!(n > 0, "the view must carry leaves");
    assert!(m < n, "the view kept all {n} leaves");
    // The wood is the same wood: only leaves are thinned.
    assert_eq!(whole.wood.positions, clumped.wood.positions);
}
