//! Leaves cluster at the engine's short shoots (fn-223): an age that
//! states a cluster seats its leaves at each of its axes' tips and none
//! along them, through the one plan both executors place from; the rows
//! are refused off their rails by name, and the budget counts the
//! clusters.
use telperion_core::{
    foliage::prepared::PreparedStations,
    math::Vec3,
    mesh,
    pipeline::executor::{self, Expansion},
    presets::{Family, Preset},
    tree::cluster_key,
};

/// The beech's short shoots' age.
const SHORT: usize = 14;

/// The beech young, its short shoots bearing `leaves` at their tips.
fn clustering(leaves: f64, spread: f64) -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    f.age = 15.0;
    let species = f.space.species.as_mut().expect("an engine species");
    (
        species.states[SHORT].cluster_leaves,
        species.states[SHORT].cluster_spread,
    ) = (leaves, spread);
    f
}

fn expanded(f: &Family) -> Expansion {
    executor::grow(f).unwrap().expansion().unwrap()
}

fn stations(x: &Expansion) -> PreparedStations {
    x.stations().unwrap().expect("the beech's stations")
}

/// One segment of three leaves at each tip, on the tip's own stream; no
/// station on a short shoot's wood; and the CPU placement seats exactly
/// the stations, three at each tip.
#[test]
fn a_short_shoot_bears_its_leaves_at_its_tip_alone() {
    let f = clustering(3.0, 0.6);
    let x = expanded(&f);
    let tree = x.tree();
    assert!(tree.clusters.len() > 100, "{} tips", tree.clusters.len());
    let p = stations(&x);
    let clusters: Vec<_> = p.segments.iter().filter(|s| s.cluster.is_some()).collect();
    assert_eq!(clusters.len(), tree.clusters.len());
    for (s, c) in clusters.iter().zip(&tree.clusters) {
        let tip = c.node as usize;
        assert_eq!((s.count, s.cluster), (3, Some(0.6)));
        assert_eq!(s.endpoints, [tree.nodes[tip].position; 2]);
        assert_eq!(s.key, cluster_key(tree.key(tip)));
    }
    // A run's segment ends at its distal node; none ends on a short shoot.
    let marked: Vec<Vec3> = (0..tree.nodes.len())
        .filter(|&i| tree.is_clustered(i))
        .map(|i| tree.nodes[i].position)
        .collect();
    for s in p.segments.iter().filter(|s| s.cluster.is_none()) {
        assert!(
            !marked.contains(&s.endpoints[1]),
            "a station on a short shoot"
        );
    }
    let leaves = mesh::build(&f).unwrap().foliage.instances;
    assert_eq!(leaves.placed(), p.count as usize);
    for c in &tree.clusters {
        let at = tree.nodes[c.node as usize].position;
        let near = (0..leaves.len()).filter(|&k| leaves.position(k).distance(at) < 1e-3);
        assert!(near.count() >= 3, "a tip seats fewer than its cluster");
    }
}

/// Clustering takes the short shoots' leaves off their wood: the long
/// shoots' stations alone stay, with three at each tip in place of the
/// rest.
#[test]
fn the_replaced_leaves_are_gone() {
    let off = stations(&expanded(&clustering(0.0, 0.0)));
    let x = expanded(&clustering(3.0, 0.6));
    let on = stations(&x);
    let tips = 3 * x.tree().clusters.len() as u32;
    let runs: u32 = (on.segments.iter())
        .filter(|s| s.cluster.is_none())
        .map(|s| s.count)
        .sum();
    assert_eq!(runs + tips, on.count);
    assert!(runs < off.count, "{runs} stations stayed of {}", off.count);
}

/// At no cluster a spread is dormant: the stations are the beech's own.
#[test]
fn a_spread_with_no_cluster_changes_nothing() {
    let neutral = stations(&expanded(&clustering(0.0, 0.0)));
    let dormant = stations(&expanded(&clustering(0.0, 1.2)));
    assert_eq!(format!("{neutral:?}"), format!("{dormant:?}"));
    assert!(neutral.segments.iter().all(|s| s.cluster.is_none()));
}

/// A crown over its budget is refused by name, clusters counted.
#[test]
fn the_budget_counts_the_clusters() {
    let mut f = clustering(3.0, 0.6);
    let count = stations(&expanded(&f)).count as usize;
    f.canopy.max_instances = count - 1;
    let error = expanded(&f).stations().unwrap_err();
    assert!(
        error.to_string().contains("foliage instance budget"),
        "{error}"
    );
    f.canopy.max_instances = count;
    assert!(expanded(&f).stations().is_ok());
}

/// Each row off its rail is refused by its own name.
#[test]
fn a_cluster_off_its_rail_is_refused_by_name() {
    for (leaves, spread, name) in [
        (9.0, 0.0, "cluster_leaves"),
        (2.5, 0.0, "cluster_leaves"),
        (-1.0, 0.0, "cluster_leaves"),
        (f64::NAN, 0.0, "cluster_leaves"),
        (3.0, 1.6, "cluster_spread"),
        (3.0, -0.1, "cluster_spread"),
    ] {
        let error = executor::grow(&clustering(leaves, spread)).err().unwrap();
        assert!(
            error.to_string().contains(name),
            "{leaves}, {spread}: {error}"
        );
    }
}

/// The cost of clustering on the beech at 15 (`cargo test --profile ci
/// -p telperion-core cost_of_clustering -- --ignored --nocapture`):
/// reported, not held.
#[test]
#[ignore]
fn cost_of_clustering() {
    for (leaves, spread) in [(0.0, 0.0), (3.0, 0.6)] {
        let f = clustering(leaves, spread);
        let mut best = f64::MAX;
        for _ in 0..5 {
            let t = std::time::Instant::now();
            let built = mesh::build(&f).unwrap();
            best = best.min(t.elapsed().as_secs_f64() * 1e3);
            std::hint::black_box(built);
        }
        let x = expanded(&f);
        let t = std::time::Instant::now();
        let p = stations(&x);
        let ms = t.elapsed().as_secs_f64() * 1e3;
        println!(
            "leaves {leaves}: build {best:.1} ms, stations {ms:.2} ms, {} stations",
            p.count
        );
    }
}
