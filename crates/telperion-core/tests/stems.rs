//! The clump: stems as order-zero axes born at the root. Neutral first — a
//! table that leaves the count at one grows the tree it always grew, to the
//! byte — then the rails, the stems inside the shell and apart from one
//! another, the trunk run each of them is swept as, the base they share
//! through the pipe model, the diameter proxy that names how many there were,
//! the walk from one stem to two, and the stem root the chronicle never
//! sheds.
//! No device is needed; this is the core's own arithmetic.
mod specimens;
use telperion_core::{
    blend, branching, presets::Family, presets::Preset, surface, tree::NodeKind, Error,
};

/// Every shipped table, so neutrality is asserted on all of them at once.
const PRESETS: [Preset; 7] = [
    Preset::Ordinary,
    Preset::OregonWhiteOak,
    Preset::NorwaySpruce,
    Preset::EuropeanBeech,
    Preset::SilverBirch,
    Preset::Telperion,
    Preset::Laurelin,
];
const SEED: u32 = 7;
/// Every step of every walk is grown under one cap, the way the sweep does:
/// what is being judged here is the shape of the walk, not its full size.
const WALK_NODES: usize = 8_000;

/// FNV-1a over the bytes, the pattern the identity pins already hash with.
fn fnv(bytes: impl IntoIterator<Item = u8>) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for byte in bytes {
        hash = (hash ^ u64::from(byte)).wrapping_mul(1099511628211);
    }
    hash
}

fn family(preset: Preset, row: impl Fn(&mut Family)) -> Family {
    let mut family = preset.parameters();
    family.skeleton.seed = SEED;
    row(&mut family);
    family
}

fn grow(family: &Family) -> telperion_core::tree::Tree {
    specimens::tree(family)
}

/// The structural nodes a tree's stems leave the root on.
fn stem_roots(tree: &telperion_core::tree::Tree) -> Vec<usize> {
    tree.nodes
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(_, n)| n.parent == Some(0) && n.kind == NodeKind::Structural)
        .map(|(i, _)| i)
        .collect()
}

#[test]
fn one_stem_is_every_shipped_tree_exactly_as_it_was() {
    // The count's neutral value is inert, and so are the two rows that say
    // how a clump stands: at one stem there is no neighbour to stand apart
    // from, so neither reaches anything. Every table is read at the neutral
    // count, the birch's own clump included, so the rule is asserted on all
    // seven alike; what each table builds as it is shipped is pinned in the
    // identity photograph beside this.
    for preset in PRESETS {
        let base = family(preset, |f| f.skeleton.habit.stems = 1);
        let bytes = |f: &Family| {
            let m = specimens::mesh(f);
            (
                fnv(m.wood.positions.iter().flat_map(|v| v.to_le_bytes())),
                fnv(m
                    .foliage
                    .instances
                    .matrices
                    .iter()
                    .flatten()
                    .flat_map(|v| v.to_le_bytes())),
            )
        };
        let was = bytes(&base);
        let dialled = family(preset, |f| {
            f.skeleton.habit.stems = 1;
            f.skeleton.habit.stem_divergence = 90.0;
            f.skeleton.habit.stem_lean = 30.0;
        });
        assert_eq!(bytes(&dialled), was, "{preset:?} moved at one stem");
    }
}

#[test]
fn each_rail_is_refused_by_the_name_of_the_row_that_is_wrong() {
    for (row, apply) in [
        (
            "stems",
            Box::new(|f: &mut Family| f.skeleton.habit.stems = 7) as Box<dyn Fn(&mut Family)>,
        ),
        (
            "stems",
            Box::new(|f: &mut Family| f.skeleton.habit.stems = 0),
        ),
        (
            "stem divergence",
            Box::new(|f: &mut Family| f.skeleton.habit.stem_divergence = 120.5),
        ),
        (
            "stem divergence",
            Box::new(|f: &mut Family| f.skeleton.habit.stem_divergence = -1.0),
        ),
        (
            "stem lean",
            Box::new(|f: &mut Family| f.skeleton.habit.stem_lean = 45.5),
        ),
        (
            "stem lean",
            Box::new(|f: &mut Family| f.skeleton.habit.stem_lean = f64::NAN),
        ),
    ] {
        let f = family(Preset::SilverBirch, |f| apply(f));
        assert_eq!(
            branching::generate(&f.skeleton, f.radii).err(),
            Some(Error::InvalidInput(row)),
            "{row} off its rail was accepted"
        );
    }
}

/// A clump of two the ordinary table can stand on: wide apart in bearing and
/// leaning far enough out to part at the base.
fn clump(stems: u32) -> Family {
    family(Preset::OregonWhiteOak, |f| {
        f.skeleton.habit.stems = stems;
        f.skeleton.habit.stem_divergence = 100.0;
        f.skeleton.habit.stem_lean = 16.0;
    })
}

#[test]
fn a_clump_grows_one_axis_per_stem_inside_the_shell_and_clear_of_each_other() {
    for stems in 2..=4_u32 {
        let f = clump(stems);
        let tree = grow(&f);
        let roots = stem_roots(&tree);
        assert_eq!(roots.len(), stems as usize, "{stems} stems were not born");
        // Every stem stands inside the crown's own shell, and none of them is
        // still inside a neighbour where the two of them leave the ground.
        let radius = f.radii.trunk_radius * f.skeleton.envelope.height;
        for (k, &i) in roots.iter().enumerate() {
            let p = tree.nodes[i].position;
            assert!(p.y > 0.0, "stem {k} did not leave the root");
            for &other in &roots[..k] {
                assert!(
                    p.distance(tree.nodes[other].position) > 0.0,
                    "stem {k} was born where a neighbour was"
                );
            }
        }
        // Higher up, where the stems have had a run to part over, they stand
        // clear of one another by more than the trunk they came out of.
        let mut leader = vec![usize::MAX; tree.crossover];
        for (i, n) in tree.nodes.iter().enumerate().take(tree.crossover).skip(1) {
            let p = n.parent.unwrap() as usize;
            if leader[p] == usize::MAX || n.start_radius > tree.nodes[leader[p]].start_radius {
                leader[p] = i;
            }
        }
        let tips: Vec<_> = roots
            .iter()
            .map(|&i| {
                let mut at = i;
                while leader[at] != usize::MAX {
                    at = leader[at];
                }
                tree.nodes[at].position
            })
            .collect();
        for (k, tip) in tips.iter().enumerate() {
            for other in &tips[..k] {
                assert!(
                    tip.distance(*other) > radius,
                    "stem {k} runs inside a neighbour"
                );
            }
        }
    }
}

#[test]
fn a_clump_that_cannot_be_placed_names_the_stem_that_is_wrong() {
    // Two stems that share a heading are the same stem twice: no divergence
    // between them, or no lean to carry them apart.
    for (divergence, lean) in [(0.0, 16.0), (100.0, 0.0)] {
        let f = family(Preset::OregonWhiteOak, |f| {
            f.skeleton.habit.stems = 2;
            f.skeleton.habit.stem_divergence = divergence;
            f.skeleton.habit.stem_lean = lean;
        });
        assert_eq!(
            branching::generate(&f.skeleton, f.radii).err(),
            Some(Error::InvalidValue {
                field: "stems pass through each other",
                value: "stem 1".into(),
            }),
            "{divergence} deg apart at {lean} deg of lean was accepted"
        );
    }
}

#[test]
fn every_stem_is_swept_as_a_trunk_run() {
    // A trunk run starts on the root itself: buried under the flare's own
    // depth, widened by the flare where it meets the ground, and carrying no
    // fork socket there. Every other run starts sunk into the wood it forks
    // off and is capped by what that wood can contain, so the runs that carry
    // the flared root are exactly the stems - one per stem, and no more.
    for stems in 1..=3_u32 {
        let f = clump(stems);
        let tree = grow(&f);
        let height = f.skeleton.envelope.height;
        let mesh = surface::build(&tree, height, &f.surface).expect("the clump sweeps");
        let root = tree.nodes[0].radius;
        let flared = mesh
            .run_table
            .iter()
            .filter(|run| run.largest_radius > root)
            .count();
        assert_eq!(
            flared, stems as usize,
            "{stems} stems were swept as {flared} trunk runs"
        );
        // And each of them reaches below the ground by its own burial.
        let burial = f.surface.flare_depth * height;
        let floor = mesh
            .positions
            .as_chunks::<3>()
            .0
            .iter()
            .map(|p| f64::from(p[1]))
            .fold(f64::INFINITY, f64::min);
        assert!(
            floor <= -burial + 1e-9,
            "{stems} stems reach {floor}, above a burial of {burial}"
        );
    }
}

#[test]
fn the_stems_share_the_base_through_the_pipe_model() {
    // The radius solve keeps normalising on the root, so a clump's trunk is
    // the trunk its table authored however many stems leave it, and the
    // stems divide it through the fork exponent rather than each taking it
    // whole.
    let f = clump(2);
    let tree = grow(&f);
    let root = tree.nodes[0].radius;
    assert!(
        (root - f.radii.trunk_radius * f.skeleton.envelope.height).abs() < 1e-9,
        "the clump's root is not the authored trunk"
    );
    let roots = stem_roots(&tree);
    let carried: f64 = roots
        .iter()
        .map(|&i| tree.nodes[i].start_radius.powf(f.radii.fork_exponent))
        .sum();
    assert!(
        (carried.powf(1.0 / f.radii.fork_exponent) - root).abs() < 1e-9,
        "the stems do not add up to the base they share"
    );
    for &i in &roots {
        assert!(
            tree.nodes[i].start_radius < root,
            "a stem took the whole base"
        );
    }
}

#[test]
fn the_diameter_proxy_reports_the_largest_stem_and_how_many_there_were() {
    // The metrics example owns the proxy; what the core owes it is a tree
    // whose stems each cross breast height on their own edge.
    let f = clump(2);
    let tree = grow(&f);
    let crossings: Vec<f64> = tree
        .nodes
        .iter()
        .enumerate()
        .skip(1)
        .take(tree.crossover - 1)
        .filter(|(_, n)| n.kind == NodeKind::Structural)
        .filter_map(|(_, n)| {
            let p = &tree.nodes[n.parent.unwrap() as usize];
            let (a, b) = (p.position.y, n.position.y);
            ((a <= 1.3 && b > 1.3) || (b <= 1.3 && a > 1.3))
                .then(|| 2.0 * (n.start_radius + (n.radius - n.start_radius) * (1.3 - a) / (b - a)))
        })
        .collect();
    assert_eq!(
        crossings.len(),
        2,
        "a two-stemmed tree crossed breast height {} times",
        crossings.len()
    );
    let largest = crossings.iter().copied().fold(0.0, f64::max);
    assert!(largest > 0.0 && largest < 2.0 * tree.nodes[0].radius);
}

#[test]
fn the_walk_from_one_stem_to_two_opens_the_clump_rather_than_switching_it() {
    // The count is walked the way a leaf's lobes are, and the two rows that
    // say how a clump stands walk up from nothing beside it. Every point of
    // the walk is a family that grows a tree - no step is a frame where the
    // tree changes kind or the build refuses - the count never goes back, and
    // once the second stem is there it only ever stands further out: it parts
    // from the first over the walk rather than arriving splayed.
    let from = family(Preset::OregonWhiteOak, |_| {});
    let to = clump(2);
    const STEPS: usize = 11;
    let (mut counts, mut apart, mut skeletons) = (Vec::new(), Vec::new(), Vec::new());
    for step in 0..STEPS {
        let t = step as f64 / (STEPS - 1) as f64;
        let mut f = blend::families(&from, &to, t).expect("the walk is a family");
        f.skeleton.growth.max_nodes = Some(WALK_NODES);
        let tree = grow(&f);
        let roots = stem_roots(&tree);
        assert_eq!(
            roots.len(),
            f.skeleton.habit.stems as usize,
            "step {step} grew {} stems for a row of {}",
            roots.len(),
            f.skeleton.habit.stems
        );
        counts.push(f.skeleton.habit.stems);
        apart.push(match roots.as_slice() {
            [a, b] => tree.nodes[*a].position.distance(tree.nodes[*b].position),
            _ => 0.0,
        });
        skeletons.push(fnv(tree.nodes.iter().skip(1).flat_map(|n| {
            [n.position.x, n.position.y, n.position.z]
                .into_iter()
                .flat_map(f64::to_le_bytes)
        })));
    }
    assert_eq!(counts[0], 1, "the walk did not start on the oak's one stem");
    assert_eq!(counts[STEPS - 1], 2, "the walk did not reach the clump");
    for step in 1..STEPS {
        assert!(
            counts[step] >= counts[step - 1],
            "step {step} lost a stem the step before it had"
        );
    }
    let born = counts.iter().position(|&c| c > 1).expect("a second stem");
    // Before the second stem the two clump rows reach nothing, so the tree is
    // the oak itself; after it, every step opens the clump further.
    for step in 0..born {
        assert_eq!(skeletons[step], skeletons[0], "step {step} moved the oak");
    }
    for step in born + 1..STEPS {
        assert!(
            apart[step] > apart[step - 1],
            "step {step} closed the clump instead of opening it"
        );
        assert_ne!(
            skeletons[step - 1],
            skeletons[step],
            "step {step} grew the tree the step before it did"
        );
    }
}

#[test]
fn a_stems_own_root_is_never_shed() {
    // A stem's own root node is the base of a trunk rather than a shoot: the
    // chronicle thins the crown around it and never takes it, or the tree
    // would be standing on nothing. It is born in the slice it grew in, like
    // every other node - a read of the tree at an age is the tree a fresh
    // build of that age grows, and a fresh build at year zero has grown
    // nothing, so a stem stamped with the root's own year would be in the one
    // and not the other.
    let mut f = clump(2);
    f.age = 12.0;
    f.skeleton.growth.max_nodes = Some(WALK_NODES);
    let mut specimen = branching::Specimen::build(&f).expect("the clump starts growing");
    specimen.advance(6.0).expect("the clump grows on");
    let tree = specimen.tree();
    let roots = stem_roots(tree);
    assert_eq!(roots.len(), 2, "the clump lost a stem");
    for &i in &roots {
        assert_eq!(
            tree.nodes[i].shoot.death_year, None,
            "a stem's root was shed"
        );
        assert!(
            tree.nodes[i].shoot.birth_year > 0.0,
            "a stem's root claims the root's own year"
        );
    }
}
