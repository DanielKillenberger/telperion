//! Girth by the pipe model (as Pałubicki et al. 2009 use it): a section is
//! the sum of the sections it carries, a section being the radius to its
//! PA's exponent (2, the area, by default; a larger one keeps what it
//! carries thicker beside it). Each phytomer adds its PA's pipe, scaled by
//! its presence and ripened over its PA's `ripening` years, so a branch
//! growing in thickens what bears it by degree and a young tip stays fine.
//! Secondary growth, a share, weighs that radius against the one the
//! phytomer was established with (`Form::secondary`): the load is carried
//! on as the pipe model's sections whatever the share.
//! Two terms of fn-197 step 5, each neutral at 0: a phytomer's own pipe
//! follows its leaves' light against the tree's pipe-weighted mean
//! (`leaf_girth`), and a shed branch leaves a share of its pipe in its
//! bearer (`retained`; Shinozaki's disused pipes; `girth/disused.rs`).
use crate::transcendental::Transcendental;
mod disused;

use crate::fixed::sum::Sum;
use crate::geometry::relay_span;
use crate::species::{Form, Species};
use crate::structure::{Origin, Phytomer, Structure};
use crate::upward::Tree;
pub(crate) use disused::{attachments, disused};

/// A branch's pipe that stays in its bearer as it is shed: the bearer's
/// index, the node it stood at (none: the bearer's tip), its base radius
/// as grown, and the share kept.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Disused {
    pub axis: usize,
    pub node: Option<usize>,
    pub radius: f64,
    pub share: f64,
}

/// A phytomer's own pipe: its PA's, by its scale, ripened, by `leaf`.
pub(crate) fn own(form: &Form, phytomer: &Phytomer, age: u32, scale: f64, leaf: f64) -> f64 {
    let years = f64::from(age.saturating_sub(phytomer.cycle)) + 1.0;
    form.pipe * scale * ripe(years, form.ripening) * leaf
}

/// Each PA's light term for each phytomer, (light)^χ, and the factor
/// that keeps the tree's pipe-weighted mean whole: χ 0 everywhere leaves
/// every pipe as it was, to the bit.
pub(super) fn leaf_terms(structure: &Structure, species: &Species) -> Option<f64> {
    if species.states.iter().all(|s| s.leaf_girth == 0.0) {
        return None;
    }
    // Order-free (`fixed::sum`).
    let (mut total, mut weighted) = (Sum::default(), Sum::default());
    for axis in &structure.axes {
        let state = &species.states[axis.pa];
        for p in &axis.phytomers {
            let pipe = own(&state.form, p, structure.age, p.scale, 1.0);
            total.add(pipe);
            weighted.add(pipe * p.light.powf_fixed(state.leaf_girth));
        }
    }
    let (total, weighted) = (total.value(), weighted.value());
    (weighted > 0.0).then(|| total / weighted)
}

/// The leaf term's mean as `thicken` takes it from the tree alone.
pub(crate) fn leaf_mean(structure: &Structure, species: &Species) -> Option<f64> {
    leaf_terms(structure, species)
}

/// What thickening takes beyond the tree: the shed branches' pipes it
/// keeps, and the leaf term's mean where the tree as grown sets it.
#[derive(Debug, Clone, Default)]
pub(crate) struct Girth {
    pub disused: Vec<Disused>,
    pub leaf_mean: Option<f64>,
}

/// Sets every phytomer's radius from the pipes it carries, and the shed
/// branches' pipes it keeps, on up to `threads` cores.
pub(crate) fn thicken(structure: &mut Structure, species: &Species, girth: &Girth, threads: usize) {
    thicken_given(structure, species, girth, &|_| None, threads);
}

/// `thicken`, each axis `given` names standing for a subtree grown
/// elsewhere whose base radius it gives (fn-209 F1, realized mechanics):
/// each axis's base radius, by index.
pub(crate) fn thicken_given(
    structure: &mut Structure,
    species: &Species,
    girth: &Girth,
    given: crate::shed::Given<f64>,
    threads: usize,
) -> Vec<f64> {
    let age = structure.age;
    let mean = girth.leaf_mean.or_else(|| leaf_terms(structure, species));
    let axes = &mut structure.axes;
    // The shed branches' pipes each axis keeps, in their order.
    let mut kept: Vec<Vec<&Disused>> = vec![Vec::new(); axes.len()];
    for d in &girth.disused {
        kept[d.axis].push(d);
    }
    let links: Vec<(Origin, f64)> = axes.iter().map(|a| (a.origin, a.blend)).collect();
    let parents: Vec<Option<usize>> = links.iter().map(|l| l.0.parent()).collect();
    let tree = Tree::new(&parents);
    // Section carried into each axis's phytomers by the pipes it keeps,
    // then by its laterals, and into its tip by its continuation or
    // relay, the last child first, as a backward pass meets them. Each
    // child hands on its base radius; its parent raises it to the
    // parent's own exponent.
    tree.upward(
        axes,
        threads,
        &|i, axis, radii: &[f64], at_node: &mut Vec<f64>| {
            let state = &species.states[axis.pa];
            let form = state.form;
            let (e, s) = (form.exponent, form.secondary);
            at_node.clear();
            at_node.resize(axis.phytomers.len(), 0.0);
            let (mut at_tip, mut at_base) = (0.0, 0.0);
            for d in &kept[i] {
                let section = d.share * d.radius.powf_fixed(e);
                match d.node.and_then(|n| at_node.get_mut(n)) {
                    Some(at) => *at += section,
                    None => at_tip += section,
                }
            }
            for &c in tree.children(i) {
                let section = given(c).unwrap_or(radii[c]).powf_fixed(e);
                match links[c].0 {
                    Origin::Seed => {}
                    Origin::Lateral { node, .. } => match at_node.get_mut(node) {
                        Some(at) => *at += section,
                        None => at_tip += section,
                    },
                    // A relay's section enters at its node as far as it stands
                    // there, spread over its span, at the tip as far as it is
                    // still the continuation.
                    Origin::Relay { node, .. } => {
                        let t = relay_span(axis, state.relay_at, node);
                        for (at, share) in relay_shares(node, t, links[c].1) {
                            match at {
                                Entry::Node(k) if k < at_node.len() => {
                                    at_node[k] += section * share
                                }
                                Entry::Base => at_base += section * share,
                                _ => at_tip += section * share,
                            }
                        }
                    }
                    Origin::Continuation { .. } => at_tip += section,
                }
            }
            let mut section = at_tip;
            for (k, phytomer) in axis.phytomers.iter_mut().enumerate().rev() {
                let leaf = mean.map_or(1.0, |m| phytomer.light.powf_fixed(state.leaf_girth) * m);
                let own = own(&state.form, phytomer, age, phytomer.scale, leaf);
                section += own.powf_fixed(e) + at_node[k];
                // Laid down at the apex, the phytomer carried nothing but itself.
                let established = form.pipe * phytomer.scale * ripe(1.0, form.ripening);
                phytomer.radius = s * section.powf_fixed(1.0 / e) + (1.0 - s) * established;
            }
            (section + at_base).powf_fixed(1.0 / e)
        },
    )
}

/// Where on its parent a relay's pipe or load enters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Entry {
    /// At the parent's node.
    Node(usize),
    /// At the parent's base, below its first node.
    Base,
    /// At the parent's tip, as its continuation.
    Tip,
}

/// The shares of a relay's pipe or load that enter its parent where: `b`
/// of it where it stands, `t` of the way along the span of node `node`
/// (`geometry::relay_span`), so at that node by `t` and at the one before
/// (or the base) by the rest; at the tip as far as it is still the
/// continuation. A relay moving past a node moves its pipe by degree
/// (fn-199 R4).
pub(crate) fn relay_shares(node: usize, t: f64, b: f64) -> [(Entry, f64); 3] {
    let below = node.checked_sub(1).map_or(Entry::Base, Entry::Node);
    [
        (Entry::Node(node), b * t),
        (below, b * (1.0 - t)),
        (Entry::Tip, 1.0 - b),
    ]
}

/// How far a phytomer `years` old has laid down its own wood, ripening
/// over `ripening` years: whole at once where `ripening` is 0.
pub(crate) fn ripe(years: f64, ripening: f64) -> f64 {
    if ripening <= 0.0 {
        1.0
    } else {
        (years / ripening).min(1.0)
    }
}

#[cfg(test)]
mod tests;
