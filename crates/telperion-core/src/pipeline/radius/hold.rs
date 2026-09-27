//! The limb profile: each structural axis holds the girth it starts with over
//! `girthHold` of its reach, then falls to the pipe model's radius by its tip.
//! Stems, codominant siblings and limbs follow the one rule, and a node's
//! share of its axis's reach is its path from where the axis leaves its parent
//! against that plus its path on to the axis's own tip. Where the held girth
//! wins, a fork's parts carry more wood than their parent: conservation at
//! forks is given up over the hold, and each part's start is clamped to its
//! parent's radius at the junction.
use super::*;

/// Hold each structural axis's girth over the solved pipe radii. At zero
/// hold the tree is left as the pipe model solved it, to the bit.
pub(crate) fn hold(tree: &mut Tree, p: RadiusParams) {
    if p.girth_hold == 0.0 {
        return;
    }
    let count = tree.crossover;
    let step = |t: &Tree, i: usize| {
        let parent = t.nodes[i].parent.unwrap() as usize;
        t.nodes[parent].position.distance(t.nodes[i].position)
    };
    // The root is a point, not wood: every part leaving it starts an axis,
    // as do a lateral and a codominant sibling; a fork's primary above the
    // root carries its axis on.
    let begins = |t: &Tree, i: usize| {
        let n = &t.nodes[i];
        n.parent.is_none_or(|p| p == 0)
            || n.shoot.bud_fate == BudFate::Lateral
            || n.codominant.is_some()
    };
    // Bottom up, each node's path along its own axis to that axis's tip.
    let mut reach = vec![0.0_f64; count];
    for i in (1..count).rev() {
        if !begins(tree, i) {
            let parent = tree.nodes[i].parent.unwrap() as usize;
            reach[parent] = reach[parent].max(reach[i] + step(tree, i));
        }
    }
    // Top down, each node's path from where its axis leaves its parent, and
    // the girth the pipe model gave the axis there: its first node's start
    // radius. Every node of an axis lies past its start, so a hold rising
    // from zero moves no radius by a jump.
    let mut along = vec![0.0; count];
    let mut base = vec![0.0; count];
    let mut share = vec![1.0; count];
    for i in 1..count {
        let parent = tree.nodes[i].parent.unwrap() as usize;
        if begins(tree, i) {
            along[i] = step(tree, i);
            base[i] = tree.nodes[i].start_radius;
        } else {
            along[i] = along[parent] + step(tree, i);
            base[i] = base[parent];
        }
        let whole = along[i] + reach[i];
        share[i] = if whole > 0.0 { along[i] / whole } else { 1.0 };
    }
    tree.pipe = tree.nodes[..count]
        .iter()
        .map(|n| [n.start_radius, n.radius])
        .collect();
    for i in 1..count {
        let n = &tree.nodes[i];
        let parent = n.parent.unwrap() as usize;
        let radius = n.radius.max(profile(p, base[i], n.radius, share[i]));
        let start = if begins(tree, i) {
            n.start_radius
        } else {
            let held = profile(p, base[i], n.start_radius, share[parent]);
            n.start_radius.max(held)
        };
        let start = start.min(tree.nodes[parent].radius);
        tree.nodes[i].start_radius = start;
        tree.nodes[i].radius = radius.min(start);
    }
}

/// The held girth at share `t` of an axis's reach: its base over the hold,
/// then a smoothstep in log radius to the pipe radius `pipe`, over a fall
/// `girthFall` times shorter than the hold, cut at the tip.
fn profile(p: RadiusParams, base: f64, pipe: f64, t: f64) -> f64 {
    let hold = p.girth_hold;
    let end = (hold * (1.0 + 1.0 / p.girth_fall)).min(1.0);
    if t <= hold {
        return base;
    }
    if t >= end {
        return pipe;
    }
    let u = (t - hold) / (end - hold);
    let w = u * u * (3.0 - 2.0 * u);
    base * (pipe / base).powf_fixed(w)
}
