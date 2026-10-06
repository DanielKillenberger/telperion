//! Task 1's adversarial cases (fn-209 R1): fractional lifespans, relay and
//! dormant buds, ancestors' shedding, retained pipes and a loaded
//! junction, each the same tree at every cut as grown whole, each case
//! shown to be present among the fine roots. Threshold-adjacent draws
//! are `draws.rs`'s.
use super::{request, same_at_cuts};
use crate::fine::CommittedFine;
use crate::species::Species;
use crate::structure::{Origin, Structure};
use crate::{oak, spruce};

const CUTS: [f64; 4] = [0.0, 64.0, 1024.0, f64::INFINITY];

/// The oak with its light switched off, so its fine roots can be grown
/// (F2b carries light): relays at every stop, epicormic sleeping buds,
/// shedding at every order and retained pipes.
fn unlit_oak() -> Species {
    let mut species = oak();
    for s in &mut species.states {
        (s.shade_hazard, s.shade_size) = (0.0, 0.0);
        (s.balance_hazard, s.leaf_girth) = (0.0, 0.0);
    }
    species
}

/// Each root's axes in the tree: its first axis's index and the indices
/// of every axis that stands on it.
fn root_axes(fine: &CommittedFine) -> Vec<Vec<usize>> {
    let axes = &fine.structure.axes;
    let mut root = vec![usize::MAX; axes.len()];
    let mut out: Vec<Vec<usize>> = Vec::new();
    for (i, axis) in axes.iter().enumerate().skip(fine.above) {
        let parent = axis.origin.parent().expect("borne");
        root[i] = if parent < fine.above {
            out.push(Vec::new());
            out.len() - 1
        } else {
            root[parent]
        };
        out[root[i]].push(i);
    }
    out
}

#[test]
fn fractional_lifespans() {
    let mut species = unlit_oak();
    for (k, s) in species.states.iter_mut().enumerate() {
        if s.lifespan > 0.0 {
            s.lifespan = (s.lifespan * 1.37 + 0.11 * k as f64).max(0.6);
        }
    }
    assert!(species.states.iter().any(|s| s.lifespan.fract() != 0.0));
    let trees = same_at_cuts(&species, request(32, 3), &CUTS);
    let woken = trees
        .iter()
        .flat_map(|t| &t.roots)
        .filter(|(r, ..)| r.woken());
    assert!(
        woken.count() > 0,
        "no woken root, whose sleep is a fraction"
    );
}

#[test]
fn relay_and_dormant_buds() {
    let trees = same_at_cuts(&unlit_oak(), request(24, 1), &CUTS);
    let fine = &trees[2];
    let woken = fine.roots.iter().filter(|(r, ..)| r.woken()).count();
    let relayed = fine
        .structure
        .axes
        .iter()
        .skip(fine.above)
        .filter(|a| matches!(a.origin, Origin::Relay { .. }))
        .count();
    assert!(
        woken > 0 && relayed > 0,
        "{woken} woken roots, {relayed} relays"
    );
}

#[test]
fn ancestors_shedding() {
    for species in [unlit_oak(), spruce()] {
        let trees = same_at_cuts(&species, request(24, 2), &CUTS);
        let fine = &trees[1];
        let conditioned = fine.roots.iter().filter(|(r, ..)| r.context.is_some());
        assert!(
            conditioned.count() > 0,
            "no root's keeping read its bearer's"
        );
    }
}

#[test]
fn retained_pipes() {
    let species = unlit_oak();
    assert!(species.states.iter().any(|s| s.retained > 0.0));
    let trees = same_at_cuts(&species, request(24, 5), &CUTS);
    // A root its bearer was handed more than its kept pipes: the share of
    // its shed wood's pipes the bearer retains.
    let fine = &trees[2];
    assert!(fine
        .roots
        .iter()
        .any(|(r, ..)| species.states[r.record.pa].retained > 0.0 && r.committed.section > 0.0));
}

/// The spruce's limbs sag under their roots' committed loads; every root
/// stands exactly on its bearer's laid node, before and after.
#[test]
fn a_loaded_junction() {
    let species = spruce();
    let trees = same_at_cuts(&species, request(30, 1), &CUTS);
    for fine in &trees[1..3] {
        let loaded = fine.roots.iter().filter(|(r, ..)| r.committed.mass > 0.0);
        assert!(loaded.count() > 0);
        let bent = fine.structure.axes.iter().take(fine.above);
        assert!(bent.clone().any(|a| species.states[a.pa].form.sag > 0.0));
        for axes in root_axes(fine) {
            on_its_node(&fine.structure, axes[0]);
        }
    }
}

/// Axis `i` stands where its bud's frame puts it on its laid bearer.
fn on_its_node(tree: &Structure, i: usize) {
    let axis = &tree.axes[i];
    let Origin::Lateral {
        parent,
        node,
        woken,
        ..
    } = axis.origin
    else {
        panic!("a root is a lateral");
    };
    let bearer = &tree.axes[parent];
    let tip = bearer.phytomers[node].tip;
    let base = if woken {
        let below = node
            .checked_sub(1)
            .map_or(bearer.base, |n| bearer.phytomers[n].tip);
        (below + tip) * 0.5
    } else {
        tip
    };
    assert_eq!(axis.base, base, "axis {i} off its node");
}
