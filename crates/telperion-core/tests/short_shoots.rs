//! Short shoots: spurs a few centimetres long along limb and branch wood, each
//! carrying a cluster of leaves. They are placements on the wood the skeleton
//! already has, so these tests hold the rows to their rails, the shoots to the
//! wood that may bear them, the skeleton to its node count, and the draws to
//! the wood's own identity whatever order the wood is stored or grown in.
use telperion_core::{
    branching,
    foliage::{self, CanopyParams, Instances, TwigPlacement},
    presets::{Family, Preset},
    tree::{NodeKind, Tree},
    Error,
};

const SEEDS: [u32; 3] = [1, 2, 3];

/// The beech's own table under a node cap, small enough to grow many times,
/// with a spur every 5 cm on wood under a third of the stem's radius.
fn beech(seed: u32) -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    f.skeleton.seed = seed;
    f.skeleton.growth.max_nodes = Some(12_000);
    f.canopy = CanopyParams {
        short_shoot_spacing: 0.05,
        short_shoot_radius: 0.35,
        short_shoot_length: 0.04,
        short_shoot_leaves: 5,
        short_shoot_spread: 70.0,
        // No gap between limb systems: every leaf a short shoot hangs stays.
        limb_clumping: 0.0,
        ..f.canopy
    };
    f
}

fn grown(f: &Family) -> Tree {
    branching::generate(&f.skeleton, f.radii).unwrap().tree
}

fn twig(f: &Family) -> Option<TwigPlacement> {
    let t = f.skeleton.twigs.resolved().unwrap().twig;
    Some(TwigPlacement {
        internode_length: t.internode_length,
        stations_per_internode: t.stations_per_internode,
    })
}

fn placed(f: &Family, tree: &Tree, canopy: CanopyParams) -> Instances {
    foliage::place(tree, f.skeleton.envelope, f.skeleton.seed, canopy, twig(f)).unwrap()
}

fn bytes(i: &Instances) -> Vec<u32> {
    i.matrices.iter().flatten().map(|v| v.to_bits()).collect()
}

fn none(p: CanopyParams) -> CanopyParams {
    CanopyParams {
        short_shoot_spacing: 0.0,
        ..p
    }
}

#[test]
fn the_beech_carries_most_of_its_leaves_on_short_shoots() {
    let mut f = Preset::EuropeanBeech.parameters();
    f.skeleton.seed = 1;
    assert!(f.canopy.short_shoot_spacing > 0.0);
    f.skeleton.growth.max_nodes = Some(40_000);
    let tree = grown(&f);
    let all = placed(&f, &tree, f.canopy).matrices.len();
    let own = placed(&f, &tree, none(f.canopy)).matrices.len();
    assert!(all - own > own, "{own} of {all} leaves stand on twigs");
}

#[test]
fn every_table_but_the_beech_grows_none() {
    for preset in [
        Preset::Ordinary,
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
        Preset::SilverBirch,
        Preset::Telperion,
        Preset::Laurelin,
    ] {
        let canopy = preset.parameters().canopy;
        assert_eq!(canopy.short_shoot_spacing, 0.0, "{preset:?}");
        assert_eq!(
            (canopy.short_shoot_radius, canopy.short_shoot_length),
            (0.15, 0.04)
        );
        assert_eq!(
            (canopy.short_shoot_leaves, canopy.short_shoot_spread),
            (3, 45.0)
        );
    }
}

#[test]
fn a_zero_spacing_is_inert_whatever_the_other_rows_say() {
    for seed in SEEDS {
        let mut f = beech(seed);
        // Slender wood bears leaves of its own under the cap, so the test
        // compares something.
        f.canopy = CanopyParams {
            shoot_radius: 0.05,
            ..none(f.canopy)
        };
        let tree = grown(&f);
        let moved = CanopyParams {
            short_shoot_radius: 1.0,
            short_shoot_length: 0.3,
            short_shoot_leaves: 8,
            short_shoot_spread: 10.0,
            ..f.canopy
        };
        let base = placed(&f, &tree, f.canopy);
        assert!(
            !base.matrices.is_empty(),
            "seed {seed}: the beech bears nothing"
        );
        assert_eq!(
            bytes(&placed(&f, &tree, moved)),
            bytes(&base),
            "seed {seed}"
        );
        let (e, s) = (f.skeleton.envelope, f.skeleton.seed);
        assert!(foliage::short_shoots(&tree, e, s, moved)
            .unwrap()
            .is_empty());
    }
}

#[test]
fn short_shoots_append_and_leave_every_other_leaf_its_bytes() {
    for seed in SEEDS {
        let f = beech(seed);
        let tree = grown(&f);
        let without = placed(&f, &tree, none(f.canopy));
        let with = placed(&f, &tree, f.canopy);
        let shoots = foliage::short_shoots(&tree, f.skeleton.envelope, seed, f.canopy).unwrap();
        let leaves = f.canopy.short_shoot_leaves as usize;
        assert!(!shoots.is_empty(), "seed {seed}: no short shoot grew");
        assert_eq!(
            with.matrices.len(),
            without.matrices.len() + shoots.len() * leaves,
            "seed {seed}: a cluster is not the row's leaf count"
        );
        assert_eq!(
            with.matrices[..without.matrices.len()],
            without.matrices[..]
        );
        // Each cluster hangs at its own shoot's tip, in the order they stand.
        let clusters = with.matrices[without.matrices.len()..].chunks(leaves);
        for (shoot, cluster) in shoots.iter().zip(clusters) {
            for m in cluster {
                let at = [m[12], m[13], m[14]];
                let tip = [shoot.tip.x, shoot.tip.y, shoot.tip.z].map(|v| v as f32);
                assert_eq!(at, tip, "seed {seed}: a leaf is off its cluster");
            }
        }
    }
}

#[test]
fn every_cluster_carries_the_row_s_count_of_leaves() {
    let f = beech(1);
    let tree = grown(&f);
    let (e, s) = (f.skeleton.envelope, f.skeleton.seed);
    let shoots = foliage::short_shoots(&tree, e, s, f.canopy).unwrap().len();
    for leaves in 1..=foliage::MAX_SHORT_SHOOT_LEAVES {
        let p = CanopyParams {
            short_shoot_leaves: leaves,
            ..f.canopy
        };
        let mut out = Instances::default();
        foliage::place_short_shoots(&tree, e, s, p, &mut out).unwrap();
        assert_eq!(
            out.matrices.len(),
            shoots * leaves as usize,
            "{leaves} leaves"
        );
    }
}

#[test]
fn a_short_shoot_stands_only_on_limb_and_branch_wood_under_the_row() {
    for seed in SEEDS {
        let f = beech(seed);
        let tree = grown(&f);
        let p = f.canopy;
        let floor = f.skeleton.envelope.crown_base * f.skeleton.envelope.height;
        let thickest = tree.nodes[0].radius * p.short_shoot_radius;
        let shoots = foliage::short_shoots(&tree, f.skeleton.envelope, seed, p).unwrap();
        let mut on = vec![0usize; tree.nodes.len()];
        for s in &shoots {
            let n = &tree.nodes[s.wood];
            let from = tree.nodes[n.parent.unwrap() as usize].position;
            assert_ne!(
                n.kind,
                NodeKind::Twig,
                "seed {seed}: a short shoot on twig wood"
            );
            assert!(
                n.radius.max(n.start_radius) <= thickest,
                "seed {seed}: a short shoot on wood thicker than the row"
            );
            assert!(
                s.base.y >= floor,
                "seed {seed}: a short shoot below the crown base"
            );
            // It leaves the bark: its distance off the wood's axis is the
            // wood's own radius there, and it runs the row's length.
            let axis = n.position - from;
            let t = ((s.base - from).dot(axis) / axis.length_squared()).clamp(0.0, 1.0);
            let off = s.base.distance(from.lerp(n.position, t));
            assert!(
                off >= n.radius - 1e-9 && off <= n.start_radius + 1e-9,
                "seed {seed}: a short shoot off the bark"
            );
            let length = s.tip.distance(s.base);
            assert!((length - p.short_shoot_length).abs() < 1e-9, "seed {seed}");
            on[s.wood] += 1;
        }
        // And along every piece of wood that may bear them, one per spacing.
        let mut eligible = 0;
        for (i, n) in tree.nodes.iter().enumerate().skip(1) {
            let from = tree.nodes[n.parent.unwrap() as usize].position;
            let clear = from.y.min(n.position.y) - n.start_radius >= floor;
            if n.kind == NodeKind::Twig || n.radius.max(n.start_radius) > thickest || !clear {
                continue;
            }
            let per = from.distance(n.position) / p.short_shoot_spacing;
            eligible += 1;
            assert!(
                on[i] == per.floor() as usize || on[i] == per.floor() as usize + 1,
                "seed {seed}: wood {i} carries {} short shoots for {per:.2} spacings",
                on[i]
            );
        }
        assert!(eligible > 100, "seed {seed}: the test must reach limb wood");
    }
}

#[test]
fn every_row_is_refused_by_name_off_its_rail() {
    let f = beech(1);
    let tree = grown(&f);
    let (e, s) = (f.skeleton.envelope, f.skeleton.seed);
    let (low, high) = foliage::SHORT_SHOOT_SPACING;
    type Set = fn(&mut CanopyParams, f64);
    let spacing: Set = |p, v| p.short_shoot_spacing = v;
    let radius: Set = |p, v| p.short_shoot_radius = v;
    let length: Set = |p, v| p.short_shoot_length = v;
    let spread: Set = |p, v| p.short_shoot_spread = v;
    let leaves: Set = |p, v| p.short_shoot_leaves = v as u32;
    for (set, name, bad, good) in [
        (
            spacing,
            "short shoot spacing",
            vec![0.005, -0.1, high * 1.01, f64::NAN],
            vec![0.0, low, high],
        ),
        (
            radius,
            "short shoot radius",
            vec![-0.01, 1.01, f64::INFINITY],
            vec![0.0, 1.0],
        ),
        (
            length,
            "short shoot length",
            vec![-0.01, 0.51],
            vec![0.0, 0.5],
        ),
        (
            spread,
            "short shoot spread",
            vec![-1.0, 91.0],
            vec![0.0, 90.0],
        ),
        (leaves, "short shoot leaves", vec![0.0, 9.0], vec![1.0, 8.0]),
    ] {
        for value in bad {
            let mut p = f.canopy;
            set(&mut p, value);
            let refused = Some(Error::InvalidInput(name));
            assert_eq!(
                foliage::place(&tree, e, s, p, twig(&f)).err(),
                refused,
                "{name} {value}"
            );
            assert_eq!(
                foliage::short_shoots(&tree, e, s, p).err(),
                refused,
                "{name} {value}"
            );
            let mut out = Instances::default();
            let placed = foliage::place_short_shoots(&tree, e, s, p, &mut out);
            assert_eq!(placed.err(), refused, "{name} {value}");
        }
        for value in good {
            let mut p = f.canopy;
            set(&mut p, value);
            assert!(
                foliage::short_shoots(&tree, e, s, p).is_ok(),
                "{name} {value}"
            );
        }
    }
}

#[test]
fn short_shoots_add_no_node_to_the_skeleton() {
    for seed in SEEDS {
        let with = beech(seed);
        let without = Family {
            canopy: none(with.canopy),
            ..with.clone()
        };
        let (a, b) = (grown(&with), grown(&without));
        assert_eq!(a, b, "seed {seed}: a canopy row reached the skeleton");
        let (a, b) = (
            branching::Specimen::build(&with).unwrap(),
            branching::Specimen::build(&without).unwrap(),
        );
        assert_eq!(a.tree().nodes.len(), b.tree().nodes.len(), "seed {seed}");
        let full = telperion_core::mesh::build(&with, telperion_core::mesh::Detail::Full);
        let bare = telperion_core::mesh::build(&without, telperion_core::mesh::Detail::Full);
        let (full, bare) = (full.unwrap(), bare.unwrap());
        assert_eq!(
            full.wood, bare.wood,
            "seed {seed}: short shoots moved the wood"
        );
        assert!(
            full.foliage_instances() > bare.foliage_instances(),
            "seed {seed}"
        );
    }
}

#[test]
fn the_instance_budget_counts_short_shoot_leaves() {
    let f = beech(2);
    let tree = grown(&f);
    let total = placed(&f, &tree, f.canopy).matrices.len();
    let at = |max_instances| CanopyParams {
        max_instances,
        ..f.canopy
    };
    let fits = foliage::place(&tree, f.skeleton.envelope, 2, at(total), twig(&f));
    assert_eq!(fits.unwrap().matrices.len(), total);
    assert_eq!(
        foliage::place(&tree, f.skeleton.envelope, 2, at(total - 1), twig(&f)).err(),
        Some(Error::ResourceLimit("foliage instance budget"))
    );
}

/// Build orders and the blend walk: their own file, beside the rows'.
#[path = "short_shoots/order.rs"]
mod order;
