//! The clump and the fork above it, as the one fork rule builds them: a fork
//! at the root is a clump whose parts are order-zero axes born there, and a
//! fork higher on the trunk parts one trunk into several. The stems stand
//! inside the shell and apart, are swept as trunk runs at the ground and from
//! a socket above it, share the base through the pipe model, name how many
//! there were at breast height, and are never shed.
//! No device is needed; this is the core's own arithmetic.
use super::specimens;
use telperion_core::{
    branching, presets::Family, presets::Preset, surface, tree::NodeKind, tree::Tree, Error,
};

const SEED: u32 = 7;
/// Every tree is grown under one cap, the way the sweep does.
const NODES: usize = 8_000;

fn family(preset: Preset, row: impl Fn(&mut Family)) -> Family {
    let mut family = preset.parameters();
    family.skeleton.seed = SEED;
    row(&mut family);
    family
}

fn grow(family: &Family) -> Tree {
    specimens::tree(family)
}

/// The structural nodes a tree's stems leave the root on.
fn stem_roots(tree: &Tree) -> Vec<usize> {
    tree.nodes
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(_, n)| n.parent == Some(0) && n.kind == NodeKind::Structural)
        .map(|(i, _)| i)
        .collect()
}

/// A clump of `ways` parts the oak's table can stand on: every tree forks at
/// the root, wide apart in bearing and leaning far enough out to part there.
fn clump(ways: u32) -> Family {
    family(Preset::OregonWhiteOak, |f| {
        f.skeleton.habit.codominance = f64::from(u32::from(ways > 1));
        f.skeleton.habit.fork_ways = f64::from(ways.max(2));
        f.skeleton.habit.fork_divergence = 100.0;
        f.skeleton.habit.fork_lean = 16.0;
    })
}

/// The oak's bole: its crown base.
fn bole(f: &Family) -> f64 {
    f.skeleton.envelope.height * f.skeleton.envelope.crown_base
}

/// Two parts on the oak's table, one upright and one leaning out, forking at
/// `share` of the bole.
fn forked(share: f64) -> Family {
    family(Preset::OregonWhiteOak, |f| {
        f.skeleton.habit.codominance = 1.0;
        f.skeleton.habit.fork_height = share * f.skeleton.envelope.crown_base;
        f.skeleton.habit.fork_lean = 24.0;
        f.skeleton.habit.fork_lean_spread = 1.0;
        f.skeleton.growth.max_nodes = Some(NODES);
    })
}

/// Every node more than one stem leaves, and the root if any does: the root
/// of a clump, and the fork it parts at.
fn forks(tree: &Tree) -> Vec<(usize, Vec<usize>)> {
    let mut runs = vec![Vec::new(); tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        if n.stem {
            runs[n.parent.unwrap() as usize].push(i);
        }
    }
    runs.into_iter()
        .enumerate()
        .filter(|(i, r)| r.len() > 1 || (*i == 0 && !r.is_empty()))
        .collect()
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
            f.skeleton.habit.codominance = 1.0;
            f.skeleton.habit.fork_divergence = divergence;
            f.skeleton.habit.fork_lean = lean;
        });
        assert_eq!(
            branching::generate(&f.skeleton, f.radii).err(),
            Some(Error::InvalidValue {
                field: "fork parts pass through each other",
                value: format!(
                    "part 1 leaves on part 0's heading at forkDivergence {divergence}, \
                     forkLean {lean} and forkLeanSpread 0"
                ),
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
fn the_later_stem_leaves_the_first_at_the_fork_height() {
    // At no height both stems leave the root; at a height one stem leaves it,
    // and the second leaves that stem on the first node at or above the height,
    // within one growth step of it, on its own lean.
    let ground = grow(&forked(0.0));
    assert_eq!(forks(&ground), vec![(0, forks(&ground)[0].1.clone())]);
    assert_eq!(
        forks(&ground)[0].1.len(),
        2,
        "the clump did not part at the ground"
    );
    for share in [0.2, 0.35, 0.5] {
        let f = forked(share);
        let tree = grow(&f);
        let found = forks(&tree);
        assert_eq!(
            found.len(),
            2,
            "at {share} the clump parted {} times",
            found.len()
        );
        assert_eq!(found[0].0, 0);
        assert_eq!(found[0].1.len(), 1, "at {share} two stems left the root");
        let (at, stems) = &found[1];
        assert_eq!(stems.len(), 2, "at {share} the fork is not two stems");
        let height = share * bole(&f);
        let y = tree.nodes[*at].position.y;
        let step = f.skeleton.resolved_growth(0).unwrap().step_distance;
        assert!(
            y >= height - 1e-9 && y < height + step,
            "at {share} the stems part at {y} m for {height} m"
        );
        let leans: Vec<f64> = stems
            .iter()
            .map(|&i| {
                let d = tree.nodes[i].position - tree.nodes[*at].position;
                (d.y / d.length()).acos().to_degrees()
            })
            .collect();
        // The first edge is the heading bent by the axis's own rise, so the
        // lean is read to a couple of degrees rather than to the bit.
        assert!(leans[0] < 1.0 && (leans[1] - 24.0).abs() < 2.0, "{leans:?}");
        // Below the fork the clump is one trunk: every node on the way down
        // has the one structural child.
        let mut k = *at;
        while let Some(p) = tree.nodes[k].parent {
            let below = tree.nodes.iter().filter(|n| n.parent == Some(p)).count();
            assert_eq!(below, 1, "at {share} a node under the fork branches");
            k = p as usize;
        }
    }
}

#[test]
fn the_girth_below_the_fork_is_the_pipe_models_sum() {
    let f = forked(0.4);
    let tree = grow(&f);
    let (at, stems) = forks(&tree)[1].clone();
    let e = f.radii.fork_exponent;
    let carried: f64 = stems
        .iter()
        .map(|&i| tree.nodes[i].start_radius.powf(e))
        .sum();
    assert!(
        (carried.powf(1.0 / e) - tree.nodes[at].radius).abs() < 1e-9,
        "the stems do not add up to the trunk below them"
    );
    let root = tree.nodes[0].radius;
    assert!(
        (root - f.radii.trunk_radius * f.skeleton.envelope.height).abs() < 1e-9,
        "the forked clump's root is not the authored trunk"
    );
}

/// The lowest point of every swept run, in the order the mesh emits them.
fn floors(mesh: &surface::SurfaceMesh) -> Vec<f64> {
    mesh.run_table
        .iter()
        .map(|run| {
            let span = run.first_index as usize..(run.first_index + run.index_count) as usize;
            mesh.indices[span]
                .iter()
                .map(|&v| f64::from(mesh.positions[v as usize * 3 + 1]))
                .fold(f64::INFINITY, f64::min)
        })
        .collect()
}

#[test]
fn a_stem_born_on_a_stem_leaves_from_a_socket_not_the_ground() {
    // At the ground both stems are trunk runs buried by the flare's depth. At
    // a height only the run leaving the root is buried; the later stem starts
    // sunk into the socket of the trunk it forks off, at the fork and not
    // under the ground, and no other run starts in the bole.
    let height = |f: &Family| f.skeleton.envelope.height;
    let buried = |f: &Family, tree: &Tree| {
        let mesh = surface::build(tree, height(f), &f.surface).expect("the clump sweeps");
        floors(&mesh)
    };
    let f = forked(0.0);
    let under = buried(&f, &grow(&f))
        .into_iter()
        .filter(|y| *y < 0.0)
        .count();
    assert_eq!(under, 2, "at the ground {under} runs were buried");
    let f = forked(0.4);
    let tree = grow(&f);
    let (at, _) = forks(&tree)[1].clone();
    let fork = &tree.nodes[at];
    let floors = buried(&f, &tree);
    assert_eq!(
        floors.iter().filter(|y| **y < 0.0).count(),
        1,
        "the fork was buried"
    );
    let socketed: Vec<f64> = floors
        .into_iter()
        .filter(|y| *y >= 0.0 && *y < bole(&f) * 0.9)
        .collect();
    assert_eq!(
        socketed.len(),
        1,
        "{} runs start in the bole",
        socketed.len()
    );
    assert!(
        socketed[0] < fork.position.y && socketed[0] > fork.position.y - 2.0 * fork.radius,
        "the later stem starts at {} for a fork at {}",
        socketed[0],
        fork.position.y
    );
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
    f.skeleton.growth.max_nodes = Some(NODES);
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
