//! Where a clump's stems part above the ground. The run through the fork
//! carries on into the straighter stem and eases the trunk's girth into it
//! over the fork's diameter; the other stem leaves from a socket in its side.
//! A clump that parts at the ground, and a limb anywhere, are left alone.
use super::{paths::paths, samples::sample_path, Sample, SurfaceParams};
use crate::{
    math::Vec3,
    presets::{Family, Preset},
    tree::{Node, Tree},
};

/// Node spacing along the hand-built clump, fine enough to see an ease.
const STEP: f64 = 0.05;

/// A trunk of 0.3 m to a fork a metre up, an upright stem of 0.18 m and a
/// wider stem of 0.22 m leaning 30 degrees out of it: stems if `stems`, a
/// trunk and two limbs if not.
fn clump(stems: bool) -> (Tree, usize) {
    let mut nodes = vec![Node {
        radius: 0.3,
        start_radius: 0.3,
        ..Node::root()
    }];
    let mut grow = |from: usize, heading: Vec3, count: usize, radius: f64| {
        let mut at = from;
        for _ in 0..count {
            let i = nodes.len();
            nodes.push(Node {
                position: nodes[at].position + heading * STEP,
                parent: Some(at as u32),
                radius,
                start_radius: radius,
                base_radius: radius,
                branch: i as u32,
                stem: stems,
                ..Node::root()
            });
            at = i;
        }
        at
    };
    let fork = grow(0, Vec3::Y, 20, 0.3);
    let lean = 30_f64.to_radians();
    grow(fork, Vec3::Y, 24, 0.18);
    grow(fork, Vec3::new(lean.sin(), lean.cos(), 0.0), 24, 0.22);
    let tree = Tree {
        crossover: nodes.len(),
        nodes,
        ..Tree::default()
    };
    (tree, fork)
}

/// A surface with no flare, so the radii read are the girths themselves.
fn plain() -> SurfaceParams {
    SurfaceParams {
        flare_radius: 1.0,
        ..SurfaceParams::default()
    }
}

/// The samples of every run: whether it is the trunk, its nodes and rings.
fn sweep(tree: &Tree, params: &SurfaceParams) -> Vec<(bool, Vec<usize>, Vec<Sample>)> {
    let paths = paths(&tree.nodes).unwrap();
    let mut distance = vec![0.0; tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        let p = n.parent.unwrap() as usize;
        distance[i] = distance[p] + tree.nodes[p].position.distance(n.position);
    }
    let mut samples = Vec::new();
    paths
        .runs
        .iter()
        .map(|run| {
            sample_path(tree, 18.0, params, &paths, run, &distance, &mut samples);
            let nodes = paths.nodes[run.start..run.end].to_vec();
            (run.trunk, nodes, samples.clone())
        })
        .collect()
}

#[test]
fn the_trunk_carries_on_into_the_straighter_stem_not_the_wider() {
    let (tree, fork) = clump(true);
    let runs = sweep(&tree, &plain());
    let (_, trunk, _) = runs.iter().find(|(trunk, ..)| *trunk).unwrap();
    let tip = tree.nodes[*trunk.last().unwrap()].position;
    assert!(tip.x.abs() < 1e-9, "the trunk ran on into the leaning stem");
    assert!(trunk.contains(&fork));
    // The leaning stem, the wider of the two, leaves from the fork's socket.
    let socketed: Vec<_> = runs
        .iter()
        .filter(|(_, nodes, _)| nodes[0] == fork)
        .collect();
    assert_eq!(socketed.len(), 1);
    let (trunk, nodes, samples) = socketed[0];
    assert!(!trunk && tree.nodes[*nodes.last().unwrap()].position.x > 0.5);
    assert!(samples[0].r < tree.nodes[fork].radius);
    // As limbs, the trunk follows the widest child, as it always has.
    let (tree, fork) = clump(false);
    let runs = sweep(&tree, &plain());
    let (_, trunk, _) = runs.iter().find(|(trunk, ..)| *trunk).unwrap();
    assert!(tree.nodes[*trunk.last().unwrap()].position.x > 0.5);
    assert!(runs.iter().any(|(_, nodes, _)| nodes[0] == fork
        && tree.nodes[*nodes.last().unwrap()].position.x.abs() < 1e-9));
}

#[test]
fn the_trunks_girth_eases_into_the_stem_over_the_forks_diameter() {
    let (tree, fork) = clump(true);
    let runs = sweep(&tree, &plain());
    let (_, nodes, samples) = runs.into_iter().find(|(trunk, ..)| *trunk).unwrap();
    // The buried root sample comes first, then one ring per node.
    let rings = &samples[samples.len() - nodes.len()..];
    let at = nodes.iter().position(|&i| i == fork).unwrap();
    let (trunk, own) = (0.3, 0.18);
    assert_eq!(rings[at].r, trunk);
    let diameter = 2.0 * trunk;
    let eased: Vec<f64> = rings[at..]
        .iter()
        .take_while(|s| s.d - rings[at].d < diameter)
        .map(|s| s.r)
        .collect();
    assert!(eased.len() > 8, "the ease spans {} rings", eased.len());
    for pair in eased.windows(2) {
        // It only ever narrows, and never by a ledge: no ring drops more
        // than a fifth of the whole difference.
        assert!(pair[1] <= pair[0] && pair[1] >= own);
        assert!(pair[0] - pair[1] < 0.2 * (trunk - own), "{pair:?}");
    }
    for s in &rings[at + eased.len()..] {
        assert_eq!(s.r, own, "the stem is not its own girth past the ease");
    }
}

/// Two stems on the oak's table, one upright and one leaning out, parting at
/// `fork` of the bole.
fn grown(fork: f64) -> (Family, Tree) {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.skeleton.seed = 7;
    f.skeleton.habit.stems = 2;
    f.skeleton.habit.stem_lean = 24.0;
    f.skeleton.habit.stem_lean_spread = 1.0;
    f.skeleton.habit.stem_fork_height = fork;
    f.skeleton.growth.max_nodes = Some(8_000);
    let tree = crate::branching::generate(&f.skeleton, f.radii)
        .unwrap()
        .tree;
    (f, tree)
}

#[test]
fn a_grown_clump_parts_cleanly_and_one_at_the_ground_is_untouched() {
    let (_, tree) = grown(0.4);
    let paths = paths(&tree.nodes).unwrap();
    let forks: Vec<usize> = (0..tree.nodes.len()).filter(|&i| paths.forks[i]).collect();
    assert_eq!(forks.len(), 1, "the clump parts at {forks:?}");
    let fork = forks[0];
    let lean = |i: usize| {
        let d = tree.nodes[i].position - tree.nodes[fork].position;
        (d.y / d.length()).acos().to_degrees()
    };
    // The run through the fork carries on into the upright stem; the leaning
    // one leaves from the fork as a run of its own.
    let through = paths
        .runs
        .iter()
        .map(|r| &paths.nodes[r.start..r.end])
        .find(|nodes| nodes[1..].contains(&fork))
        .unwrap();
    let on = through[through.iter().position(|&i| i == fork).unwrap() + 1];
    assert!(
        lean(on) < 1.0,
        "the trunk carries on {} degrees out",
        lean(on)
    );
    let off: Vec<usize> = paths
        .runs
        .iter()
        .filter(|r| paths.nodes[r.start] == fork && tree.nodes[paths.nodes[r.start + 1]].stem)
        .map(|r| paths.nodes[r.start + 1])
        .collect();
    assert_eq!(off.len(), 1);
    assert!((lean(off[0]) - 24.0).abs() < 2.0);
    // At the ground no node is a fork: both stems leave the root.
    let (_, tree) = grown(0.0);
    assert!(!super::paths::paths(&tree.nodes)
        .unwrap()
        .forks
        .contains(&true));
}

#[test]
fn a_clump_that_parts_at_the_ground_sweeps_as_it_did_without_the_flag() {
    // Clearing every stem flag is the surface before stems were told from
    // limbs: at the ground nothing moves, to the bit.
    let (f, tree) = grown(0.0);
    let mut limbs = tree.clone();
    for n in &mut limbs.nodes {
        n.stem = false;
    }
    let height = f.skeleton.envelope.height;
    assert_eq!(
        super::build(&tree, height, &f.surface).unwrap(),
        super::build(&limbs, height, &f.surface).unwrap()
    );
}
