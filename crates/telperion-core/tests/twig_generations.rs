//! `skeleton.twigs.generations`: the twig layer's depth as a row.
//!
//! The local law decided a lateral's kind by its radius alone, and the pipe
//! model sets every radius in the tree: a table that raised its fork exponent
//! for girth bought a deeper, wider twig layer it never asked for, and the
//! node ceiling refused it. This row states the depth instead, so the two are
//! independent. No device is needed; this is the core's own arithmetic.
mod specimens;
use telperion_core::{
    blend,
    branching::{append, HabitParams},
    colonization::GrowthConfig,
    envelope::Envelope,
    foliage::{self, TwigPlacement},
    math::Vec3,
    presets::{Family, Preset},
    tree::{BudFate, Node, NodeKind, Tree},
    twigs::{TwigParams, MAX_GENERATIONS},
    Error,
};

const IDS: [&str; 7] = [
    "ordinary",
    "oregon-white-oak",
    "norway-spruce",
    "european-beech",
    "silver-birch",
    "telperion",
    "laurelin",
];

/// FNV-1a over the bytes, the pattern the identity pins already hash with.
fn fnv(bytes: impl IntoIterator<Item = u8>) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for byte in bytes {
        hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
    }
    hash
}

/// Every node's position, parent and kind, base to tip.
fn skeleton(tree: &Tree) -> u64 {
    fnv(tree.nodes.iter().flat_map(|n| {
        let mut bytes = Vec::new();
        for v in [n.position.x, n.position.y, n.position.z, n.radius] {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        bytes.extend_from_slice(&n.parent.unwrap_or(u32::MAX).to_le_bytes());
        bytes.push(n.kind as u8);
        bytes
    }))
}

/// A node's twig-law generation: the count of lateral births between it and
/// the structural crown it hangs from. This is what `advance` carries as
/// `generation`, recovered from the bud fates the tree keeps.
fn generations(tree: &Tree) -> Vec<usize> {
    let mut gen = vec![0usize; tree.nodes.len()];
    for i in 1..tree.nodes.len() {
        let parent = tree.nodes[i].parent.expect("only the root has no parent") as usize;
        let base = if tree.nodes[parent].kind == NodeKind::Structural {
            0
        } else {
            gen[parent]
        };
        gen[i] = base + usize::from(tree.nodes[i].shoot.bud_fate == BudFate::Lateral);
    }
    gen
}

fn family(id: &str, seed: u32) -> Family {
    let mut f = Preset::from_id(id).expect("preset identity").parameters();
    f.skeleton.seed = seed;
    f
}

fn grow(f: &Family) -> Tree {
    specimens::tree(f)
}

/// A one-metre structural stem, thick enough that the radius never falls to
/// the twig threshold: the only thing that can stop this crown branching is
/// the row.
fn crown() -> Tree {
    let mut root = Node::root();
    root.radius = 0.1;
    root.start_radius = 0.1;
    let tip = Node {
        position: Vec3::new(0.0, 1.0, 0.0),
        parent: Some(0),
        radius: 0.1,
        start_radius: 0.1,
        branch: 1,
        ..Node::root()
    };
    Tree {
        nodes: vec![root, tip],
        crossover: 2,
        ..Default::default()
    }
}

/// Laterals that keep their parent's radius and their parent's length, on wood
/// far above both the twig and the bearing diameters. Without the row this
/// family branches until `MAX_LEVELS` drops it and the tree reports incomplete.
fn unfalling() -> TwigParams {
    let mut t = TwigParams {
        length_ratio: 1.0,
        ratio_power: 0.0,
        laterals: 1,
        internode_factor: 32.0,
        limb_radius: 0.0,
        vigour_variation: 0.0,
        angle_variation: 0.0,
        ..TwigParams::default()
    };
    t.twig.diameter = 1e-6;
    t.twig.bearing_diameter = 1e-6;
    t
}

fn branched(t: TwigParams) -> Tree {
    let mut tree = crown();
    append(
        &mut tree,
        &GrowthConfig {
            trunk_height: 0.0,
            max_nodes: 250_000,
            ..Default::default()
        },
        t,
        0,
        None,
        HabitParams::default(),
    )
    .expect("the synthetic crown branches");
    tree
}

#[test]
fn the_rail_is_one_to_six_and_refused_by_name() {
    for bad in [0, 7, 12, u32::MAX] {
        let t = TwigParams {
            generations: bad,
            ..TwigParams::default()
        };
        assert_eq!(
            t.resolved().err(),
            Some(Error::InvalidInput("twig generations")),
            "{bad} generations was accepted"
        );
    }
    for good in 1..=MAX_GENERATIONS {
        let t = TwigParams {
            generations: good,
            ..TwigParams::default()
        };
        assert!(t.resolved().is_ok(), "{good} generations was refused");
    }
}

#[test]
fn the_neutral_reaches_no_shipped_preset() {
    // Nothing in the catalogue branches six generations deep, so a table that
    // leaves the row at the top of its rail is the tree it always was. That is
    // why the oak, the spruce, the birch and the Two Trees did not move a byte
    // when the row arrived.
    for id in IDS {
        let mut f = family(id, 7);
        f.skeleton.twigs.generations = MAX_GENERATIONS;
        let tree = grow(&f);
        let deepest = generations(&tree).into_iter().max().unwrap_or(0);
        assert!(
            deepest < MAX_GENERATIONS as usize,
            "{id} branches {deepest} generations, so the neutral is not inert"
        );
    }
}

#[test]
fn the_beech_stays_at_its_stated_depth_whatever_its_radii() {
    // The beech is the table the row was added for. On its shipped rows its
    // radii happen to stop at the depth it states; thin the twig and bearing
    // thresholds so they would not, and the row - not the pipe model - holds
    // it there.
    let shipped = family("european-beech", 7);
    assert_eq!(
        shipped.skeleton.twigs.generations, 2,
        "the beech stopped stating its depth"
    );
    let mut fine = shipped.clone();
    fine.skeleton.twigs.twig.diameter = 1e-4;
    fine.skeleton.twigs.twig.bearing_diameter = 1e-4;
    for (label, f) in [("shipped", shipped), ("fine twigs", fine)] {
        let tree = grow(&f);
        let gen = generations(&tree);
        assert_eq!(
            gen.iter().copied().max(),
            Some(2),
            "{label}: the beech grew past its row"
        );
        for (i, node) in tree.nodes.iter().enumerate() {
            assert!(
                gen[i] < 2 || node.kind == NodeKind::Twig,
                "{label}: node {i} is still branching at the beech's stated depth"
            );
        }
        let mut free = f.clone();
        free.skeleton.twigs.generations = MAX_GENERATIONS;
        let deepest = generations(&grow(&free)).into_iter().max().unwrap_or(0);
        if label == "fine twigs" {
            assert!(
                deepest > 2,
                "fine twigs stop at {deepest} generations on their own"
            );
        }
    }
}

#[test]
fn no_lateral_is_born_past_the_cap_whatever_the_radii() {
    for cap in 1..=4 {
        let tree = branched(TwigParams {
            generations: cap,
            ..unfalling()
        });
        let gen = generations(&tree);
        let deepest = gen.iter().copied().max().expect("a crown has nodes");
        assert_eq!(
            deepest, cap as usize,
            "cap {cap} grew {deepest} generations on seed 0"
        );
        for (i, node) in tree.nodes.iter().enumerate() {
            assert!(
                gen[i] < cap as usize || node.kind == NodeKind::Twig,
                "cap {cap} left node {i} at generation {} branching on seed 0",
                gen[i]
            );
        }
        // The row stops the law before the structural level cap ever sees it.
        assert!(
            tree.diagnostics.complete(),
            "cap {cap} reported a truncated tree"
        );
    }
}

#[test]
fn a_lateral_the_cap_makes_a_twig_still_bears_its_leaves() {
    // A capped lateral is a twig, not a stump: it carries the twig anatomy's
    // stations, and the foliage pass hangs leaves on them.
    let t = TwigParams {
        generations: 2,
        ..unfalling()
    };
    let mut tree = branched(t);
    let gen = generations(&tree);
    let capped: Vec<usize> = (0..tree.nodes.len()).filter(|i| gen[*i] == 2).collect();
    assert!(!capped.is_empty(), "the cap made no twigs");
    assert!(
        capped.iter().all(|i| tree.nodes[*i].kind == NodeKind::Twig),
        "a capped lateral is not a twig"
    );
    let envelope = Envelope::default();
    telperion_core::radius::solve(&mut tree, envelope, Default::default()).expect("radii solve");
    let f = Family::default();
    let placed = foliage::place_on_surface(
        &tree,
        envelope,
        7,
        f.canopy,
        Some(TwigPlacement {
            internode_length: t.twig.internode_length,
            stations_per_internode: t.twig.stations_per_internode,
        }),
        &f.surface,
        // The wood is hand built, so the box is spanned over it with a metre
        // for the stand-off a station takes from the wood it sits on.
        foliage::Reference::spanning(
            tree.nodes.iter().fold(Vec3::new(1e9, 1e9, 1e9), |a, n| {
                Vec3::new(
                    a.x.min(n.position.x - 1.),
                    a.y.min(n.position.y - 1.),
                    a.z.min(n.position.z - 1.),
                )
            }),
            tree.nodes.iter().fold(Vec3::new(-1e9, -1e9, -1e9), |a, n| {
                Vec3::new(
                    a.x.max(n.position.x + 1.),
                    a.y.max(n.position.y + 1.),
                    a.z.max(n.position.z + 1.),
                )
            }),
        ),
    )
    .expect("leaves are placed");
    assert!(
        placed.len() > capped.len(),
        "the capped twigs carry no leaves"
    );
}

#[test]
fn the_planners_estimate_reserves_what_the_cap_grows() {
    // `nodes_for` bounds the local pass's node budget. Uncapped it counts
    // generations to `MAX_LEVELS`; with the row it must count to the row
    // instead, and it must not count short - a reserve under what the cap
    // grows truncates the tree and sets `node_capped`.
    let mut counts = Vec::new();
    for cap in 1..=MAX_GENERATIONS {
        let mut f = family("european-beech", 1);
        f.skeleton.twigs.generations = cap;
        let tree = grow(&f);
        assert!(
            tree.diagnostics.complete(),
            "cap {cap} grew past the reserve the planner made for it: {:?}",
            tree.diagnostics
        );
        counts.push(tree.nodes.len());
    }
    assert!(
        counts[0] < counts[1],
        "a deeper cap grew no more wood: {counts:?}"
    );
}

#[test]
fn the_same_seed_and_rows_grow_the_same_tree() {
    for cap in [1, 2, MAX_GENERATIONS] {
        for seed in [1, 7, 233] {
            let mut f = family("european-beech", seed);
            f.skeleton.twigs.generations = cap;
            assert_eq!(
                skeleton(&grow(&f)),
                skeleton(&grow(&f)),
                "seed {seed} at cap {cap} grew two trees"
            );
        }
    }
}

#[test]
fn a_blend_walks_the_count_in_integer_steps() {
    let mut a = family("european-beech", 7);
    a.skeleton.twigs.generations = 1;
    let mut b = a.clone();
    b.skeleton.twigs.generations = MAX_GENERATIONS;
    const STEPS: usize = 10;
    let mut seen = Vec::new();
    for step in 0..STEPS {
        let t = step as f64 / (STEPS - 1) as f64;
        let f = blend::families(&a, &b, t).expect("every point between two rails is a family");
        f.skeleton
            .twigs
            .resolved()
            .expect("a walked count stays on the rail");
        seen.push(f.skeleton.twigs.generations);
    }
    assert_eq!(seen.first(), Some(&1), "the walk left its first row");
    assert_eq!(
        seen.last(),
        Some(&MAX_GENERATIONS),
        "the walk left its last row"
    );
    assert!(
        seen.windows(2).all(|w| w[0] <= w[1]),
        "the count doubled back: {seen:?}"
    );
    assert!(
        seen.iter().collect::<std::collections::BTreeSet<_>>().len() > 2,
        "the count jumped rather than walked: {seen:?}"
    );
}
