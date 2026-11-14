//! Leaves by their wood's age (fn-248): a station keeps its leaf by its
//! wood's retention, clamp(leaf_years - age, 0, 1), counted the same in
//! the plan, the placement and the stations;
//! a station that sheds its leaf still takes its draws. Since fn-354 R5
//! the grower reads the same row, so each value grows its own tree.
use telperion_core::{
    foliage::Leaf,
    math::Vec3,
    mesh,
    pipeline::executor,
    presets::{Family, Preset},
    tree::Tree,
};

/// The beech young, every age's wood bearing for `years`, its short
/// shoots bearing along their wood as its long shoots do: no cluster,
/// which bears whatever its wood's age.
fn retaining(years: f64) -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    f.age = 15.0;
    let species = f.space.species.as_mut().expect("an engine species");
    for s in &mut species.states {
        (s.leaf_years, s.cluster_leaves) = (years, 0.0);
    }
    f
}

fn leaves(f: &Family) -> Vec<Leaf> {
    mesh::build(f).unwrap().foliage.instances.leaves
}

/// The distance from `p` to the nearest wood segment `bears` names, a
/// segment the stretch from a node's parent to it.
fn nearest(tree: &Tree, bears: &dyn Fn(usize) -> bool, p: Vec3) -> f64 {
    let mut best = f64::INFINITY;
    for (i, n) in tree.nodes.iter().enumerate() {
        let Some(parent) = n.parent.filter(|_| bears(i)) else {
            continue;
        };
        let a = tree.nodes[parent as usize].position;
        let ab = n.position - a;
        let t = ((p - a).dot(ab) / ab.dot(ab).max(1e-18)).clamp(0.0, 1.0);
        best = best.min(p.distance(a + ab * t));
    }
    best
}

/// A leaf is drawn only on wood younger than its age's `leaf_years`
/// (fn-248; fn-354 R5, host 12 Nov): on each value's own tree, every drawn
/// leaf stands within a leaf's reach of wood whose retention keeps some
/// stations, and none stands where no wood bears.
#[test]
fn a_leaf_stays_only_where_its_wood_keeps_it() {
    // A beech leaf and its petiole, with room for the station's offset.
    const REACH: f64 = 0.25;
    for years in [3.0, 2.0, 1.5, 1.0] {
        let f = retaining(years);
        let x = executor::grow(&f).unwrap().expansion().unwrap();
        let tree = x.tree();
        let bears = |i: usize| tree.leafing(i).keep != 0;
        let all = |_: usize| true;
        let placed = mesh::build(&f).unwrap().foliage.instances;
        assert!(!placed.leaves.is_empty(), "{years}");
        assert!(
            (0..tree.nodes.len()).any(|i| !bears(i)),
            "{years}: old wood"
        );
        for (k, &leaf) in placed.leaves.iter().enumerate() {
            let at = placed.reference.position(leaf);
            let young = nearest(tree, &bears, at);
            assert!(
                young <= REACH,
                "{years}: leaf {k} {young} m from bearing wood, {} from any",
                nearest(tree, &all, at)
            );
        }
    }
    assert!(leaves(&retaining(0.0)).is_empty());
}

/// Of one tree's stations on wood laid the year before, the share that
/// bears a leaf is its age's share of that year (fn-248; recast within one
/// tree, fn-354 R5, host 12 Nov): none at one year, a quarter at 1.25, all
/// at two, within four standard errors of the draws. Each value grows its
/// own tree, and the stations are read off its own runs; a segment of
/// mixed wood is left out.
#[test]
fn a_share_of_a_year_keeps_that_share() {
    for (years, want) in [(1.0, 0.0), (1.25, 0.25), (2.0, 1.0)] {
        let mut f = retaining(years);
        f.age = 30.0;
        let g = executor::growth(&f).unwrap();
        let structure = telperion_space::grow(&g.species, g.request).unwrap();
        let cycle: std::collections::HashMap<u64, u32> = (structure.axes.iter())
            .flat_map(|a| a.phytomers.iter().map(|p| (p.key(), p.cycle)))
            .collect();
        let x = executor::grow(&f).unwrap().expansion().unwrap();
        let tree = x.tree();
        let (runs, twig) = x.runs().unwrap().expect("regular stations");
        let year_old = |i: usize| {
            let born = cycle.get(&tree.nodes[i].identity.birth_order());
            born == Some(&(structure.age - 1)) && tree.leafing(i).older == 0.0
        };
        let (mut stations, mut kept) = (0u64, 0u64);
        for run in &runs {
            for s in 0..run.nodes.len() - 1 {
                if !year_old(run.nodes[s + 1]) {
                    continue;
                }
                let (first, last) = run.segment_stations(s, twig);
                stations += u64::from(last - first);
                kept += u64::from(run.kept_between(tree, twig, (first, last)));
            }
        }
        assert!(
            stations > 1000,
            "{years}: {stations} stations on last year's wood"
        );
        let share = kept as f64 / stations as f64;
        let error = (want * (1.0 - want) / stations as f64).sqrt();
        assert!(
            (share - want).abs() <= 4.0 * error,
            "{years}: {kept} of {stations}, {share}"
        );
    }
}

/// The plan, the placement and the prepared stations count the same.
#[test]
fn every_count_is_the_stations_kept() {
    for years in [1.0, 1.5] {
        let f = retaining(years);
        let x = executor::grow(&f).unwrap().expansion().unwrap();
        let prepared = x.stations().unwrap().unwrap();
        let placed = mesh::build(&f).unwrap().foliage.instances;
        assert_eq!(placed.placed(), prepared.count as usize, "{years}");
        let kept: u32 = prepared.segments.iter().map(|s| s.count).sum();
        assert_eq!(kept, prepared.count, "{years}");
        // A segment some of whose stations shed says where: a share of a
        // year sheds some of a node's.
        let partial = prepared.segments.iter().filter(|s| s.retention.is_some());
        assert!(years.fract() == 0.0 || partial.count() > 0, "{years}");
    }
}
