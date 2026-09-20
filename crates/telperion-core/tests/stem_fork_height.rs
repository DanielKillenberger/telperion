//! Where a clump's later stems part from its first. Neutral first - at one
//! stem the row reaches nothing on any table, to the byte - then its rail,
//! the fork a grown clump parts at and the socket its later stem leaves from,
//! the girth below the fork, and the walk from parting at the ground.
//! No device is needed; this is the core's own arithmetic.
mod specimens;
use telperion_core::{
    blend, branching, presets::Family, presets::Preset, surface, tree::Tree, Error,
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
/// Every step of the walk is grown under one cap, the way the sweep does.
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

/// Two stems on the oak's table, one upright and one leaning out, parting at
/// `fork` of the bole.
fn clump(fork: f64) -> Family {
    family(Preset::OregonWhiteOak, |f| {
        f.skeleton.habit.stems = 2;
        f.skeleton.habit.stem_lean = 24.0;
        f.skeleton.habit.stem_lean_spread = 1.0;
        f.skeleton.habit.stem_fork_height = fork;
        f.skeleton.growth.max_nodes = Some(WALK_NODES);
    })
}

/// The bole the row is a share of: the oak's crown base.
fn bole(f: &Family) -> f64 {
    f.skeleton.envelope.height * f.skeleton.envelope.crown_base
}

fn grow(family: &Family) -> Tree {
    specimens::tree(family)
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
fn at_one_stem_the_fork_height_reaches_no_table() {
    for preset in PRESETS {
        let bytes = |f: &Family| {
            let m = specimens::mesh(f);
            (
                fnv(m.wood.positions.iter().flat_map(|v| v.to_le_bytes())),
                fnv(m
                    .foliage
                    .instances
                    .leaves
                    .iter()
                    .flatten()
                    .flat_map(|w| w.to_le_bytes())),
            )
        };
        let was = bytes(&family(preset, |f| f.skeleton.habit.stems = 1));
        let dialled = family(preset, |f| {
            f.skeleton.habit.stems = 1;
            f.skeleton.habit.stem_fork_height = 0.5;
        });
        assert_eq!(bytes(&dialled), was, "{preset:?} moved at one stem");
    }
}

#[test]
fn the_rail_is_refused_by_the_name_of_the_row() {
    for value in [-0.01, 0.51, f64::NAN, f64::INFINITY] {
        let f = family(Preset::SilverBirch, |f| {
            f.skeleton.habit.stem_fork_height = value;
        });
        assert_eq!(
            branching::generate(&f.skeleton, f.radii).err(),
            Some(Error::InvalidInput("stem fork height")),
            "a fork height of {value} was accepted"
        );
    }
    for value in [0.0, 0.5] {
        let f = family(Preset::SilverBirch, |f| {
            f.skeleton.habit.stem_fork_height = value;
        });
        assert!(
            f.skeleton.habit.validate().is_ok(),
            "{value} is on the rail"
        );
    }
}

#[test]
fn the_later_stem_leaves_the_first_at_the_fork_height() {
    // At no height both stems leave the root; at a height one stem leaves it,
    // and the second leaves that stem on the first node at or above the height,
    // within one growth step of it, on its own lean.
    let ground = grow(&clump(0.0));
    assert_eq!(forks(&ground), vec![(0, forks(&ground)[0].1.clone())]);
    assert_eq!(
        forks(&ground)[0].1.len(),
        2,
        "the clump did not part at the ground"
    );
    for share in [0.2, 0.35, 0.5] {
        let f = clump(share);
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
    let f = clump(0.4);
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
    let f = clump(0.0);
    let under = buried(&f, &grow(&f))
        .into_iter()
        .filter(|y| *y < 0.0)
        .count();
    assert_eq!(under, 2, "at the ground {under} runs were buried");
    let f = clump(0.4);
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
fn the_walk_from_the_ground_lifts_the_fork() {
    // The row walks like any fraction. Every step grows a tree on two stems,
    // the first step is the clump that parts at the ground, and the fork only
    // ever climbs.
    let (from, to) = (clump(0.0), clump(0.5));
    const STEPS: usize = 11;
    let mut heights = Vec::new();
    for step in 0..STEPS {
        let t = step as f64 / (STEPS - 1) as f64;
        let f = blend::families(&from, &to, t).expect("the walk is a family");
        assert!((f.skeleton.habit.stem_fork_height - 0.5 * t).abs() < 1e-12);
        let tree = grow(&f);
        let found = forks(&tree);
        let (at, stems) = found.last().expect("the clump parts");
        assert_eq!(stems.len(), 2, "step {step} grew {} stems", stems.len());
        heights.push(tree.nodes[*at].position.y);
        if step == 0 {
            assert_eq!(tree, grow(&from), "the walk did not start at the ground");
        }
    }
    assert_eq!(heights[0], 0.0);
    for step in 1..STEPS {
        assert!(
            heights[step] >= heights[step - 1],
            "step {step}: the fork came down to {} from {}",
            heights[step],
            heights[step - 1]
        );
    }
    assert!(
        heights[STEPS - 1] > heights[0],
        "the fork never left the ground"
    );
}
