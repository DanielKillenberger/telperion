//! fn-61 R2: a first-order axis `raggedReach` stops short keeps what it bears
//! within its own share of the crown - its deeper axes and the twigs on them
//! end inside the crown's shell scaled about its station by the share it kept,
//! or, for a curtain that drops, in the band below that shell.
use super::super::in_curtain_band;
use super::*;
use crate::presets::Preset;

fn grown(preset: Preset, ragged_reach: f64, tweak: impl Fn(&mut SkeletonParams)) -> Specimen {
    let family = preset.parameters();
    let mut p = family.skeleton;
    p.habit.ragged_reach = ragged_reach;
    p.habit.attractor_weight = 0.0;
    p.habit.shedding_threshold = 0.0;
    tweak(&mut p);
    Specimen::grow(&p, family.radii).unwrap()
}

/// Every node of a shortened limb system lies in that system's shell or,
/// hanging, in the band below it; the counts of structural and twig nodes
/// judged.
fn within_share(s: &Specimen) -> (usize, usize) {
    let (tree, e, seed) = (&s.tree, s.params.envelope, s.params.seed);
    let t = s.params.twigs.resolved().unwrap();
    let (mut structural, mut twigs) = (0, 0);
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        let bound = s.scaffold.limbs().of(tree, i);
        if !bound.short() {
            continue;
        }
        let p = bound.map(n.position);
        match n.kind {
            NodeKind::Structural => structural += 1,
            _ => twigs += 1,
        }
        assert!(
            e.contains(p, 1e-6, seed) || in_curtain_band(&e, &t, seed, p, 1e-6),
            "{:?} node {i} at {:?} left its limb's share",
            n.kind,
            n.position
        );
    }
    (structural, twigs)
}

#[test]
fn a_shortened_limbs_descendants_end_within_its_share() {
    let s = grown(Preset::Ordinary, 0.6, |p| {
        p.habit.lateral_orders = 2;
        p.twigs.hang = 0.0;
    });
    let (structural, twigs) = within_share(&s);
    assert!(structural > 100 && twigs > 1_000, "{structural} {twigs}");
}

#[test]
fn a_shortened_limbs_curtain_falls_in_its_own_band() {
    let s = grown(Preset::SilverBirch, 0.6, |_| {});
    assert!(s.params.twigs.hang > 0.0 && s.params.twigs.curtain_drop > 0.0);
    let (_, twigs) = within_share(&s);
    assert!(twigs > 1_000, "{twigs}");
}

#[test]
fn limbs_that_leave_one_node_keep_their_own_shares() {
    // Upright laterals with no spread and no wander leave a station from one
    // point; each still carries the share it drew.
    let s = grown(Preset::Ordinary, 0.6, |p| {
        p.habit.lateral_pitch = 0.0;
        p.habit.pitch_variation = 0.0;
        p.habit.crookedness = 0.0;
        p.habit.laterals_per_station = 3;
    });
    let tree = &s.tree;
    let mut firsts: Vec<(Vec3, usize)> = (1..tree.nodes.len())
        .filter(|&i| {
            let parent = tree.nodes[i].parent.unwrap() as usize;
            tree.nodes[parent].stem && !tree.nodes[i].stem
        })
        .map(|i| (tree.nodes[i].position, i))
        .collect();
    firsts.sort_by(|a, b| a.0.x.total_cmp(&b.0.x).then(a.0.y.total_cmp(&b.0.y)));
    let shared = firsts
        .windows(2)
        .filter(|w| w[0].0 == w[1].0)
        .inspect(|w| {
            let (a, b) = (
                s.scaffold.limbs().of(tree, w[0].1),
                s.scaffold.limbs().of(tree, w[1].1),
            );
            assert_ne!(a, b, "two limbs from one node share one bound");
        })
        .count();
    assert!(shared > 0, "no two limbs left one node");
}

#[test]
fn a_limb_kept_whole_names_no_bound() {
    let s = grown(Preset::Ordinary, 0.0, |_| {});
    assert!((1..s.tree.nodes.len()).all(|i| !s.scaffold.limbs().of(&s.tree, i).short()));
}
