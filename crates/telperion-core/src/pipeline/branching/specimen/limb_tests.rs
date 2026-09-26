//! fn-61 R2: a first-order axis `raggedReach` stops short keeps what it bears
//! within its own share of the crown - its deeper axes and the twigs on them
//! end inside the crown's shell scaled about its station by the share it kept.
use super::*;
use crate::presets::Preset;

/// A synthetic crown: no pull points, two rule-built orders, twigs that do
/// not hang, nothing shed, and limbs that stop well short.
fn grown(ragged_reach: f64) -> Specimen {
    let mut p = Preset::Ordinary.parameters().skeleton;
    p.habit.ragged_reach = ragged_reach;
    p.habit.attractor_weight = 0.0;
    p.habit.lateral_orders = 2;
    p.habit.shedding_threshold = 0.0;
    p.twigs.hang = 0.0;
    p.growth.max_nodes = Some(200_000);
    Specimen::grow(&p, Preset::Ordinary.parameters().radii).unwrap()
}

#[test]
fn a_shortened_limbs_descendants_end_within_its_share() {
    let s = grown(0.6);
    let (tree, e, seed) = (&s.tree, s.params.envelope, s.params.seed);
    let (mut structural, mut twigs) = (0, 0);
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        let bound = s.scaffold.limbs().of(tree, i);
        if !bound.short() {
            continue;
        }
        match n.kind {
            NodeKind::Structural => structural += 1,
            _ => twigs += 1,
        }
        assert!(
            e.contains(bound.map(n.position), 1e-6, seed),
            "{:?} node {i} at {:?} left its limb's share",
            n.kind,
            n.position
        );
    }
    assert!(structural > 100 && twigs > 1_000, "{structural} {twigs}");
}

#[test]
fn a_limb_kept_whole_names_no_bound() {
    let s = grown(0.0);
    assert!((1..s.tree.nodes.len()).all(|i| !s.scaffold.limbs().of(&s.tree, i).short()));
}
