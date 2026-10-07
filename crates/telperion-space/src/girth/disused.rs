//! Shinozaki's disused pipes (`PaState::retained`): a branch being shed
//! leaves a share of the pipe it had laid down in its bearer. As a kept
//! branch fades towards its shedding (`shed.rs`), its retained share grows
//! in by what its fade has taken, so its bearer meets the same section on
//! both sides of the branch's removal.
use super::{mean_of, own, radius, section, terms, Disused, Girth};
use crate::cores::{map, PIECE};
use crate::fixed::feedback::total::Total;
use crate::fixed::{mul, q32, ONE32};
use crate::geometry::dominance;
use crate::shed::Shed;
use crate::species::Species;
use crate::structure::{Axis, Origin, Phytomer};
use crate::upward::Tree;

/// One grown axis as shedding meets it: its bearer, the node it stood at
/// (none: the bearer's tip), its PA, whether the rule can shed it (a
/// lateral or a relay), and its base radius as grown.
#[derive(Debug, Clone, Copy)]
struct Link {
    parent: Option<usize>,
    node: Option<usize>,
    pa: usize,
    sheddable: bool,
    radius: f64,
}

/// The tree as grown, before shedding: each axis's link and the leaf
/// term's mean over all of it.
pub(crate) struct Grown {
    links: Vec<Link>,
    leaf_mean: Option<f64>,
}

/// A Q32 as its exact real.
fn real(q: i64) -> f64 {
    q as f64 / ONE32 as f64
}

/// Each axis's scale at its base, as `geometry::scale` will give it
/// (Q32, one floor a product, parents first).
fn bases(axes: &[Axis], species: &Species) -> Vec<i64> {
    let mut base = vec![ONE32; axes.len()];
    for (i, a) in axes.iter().enumerate() {
        let inherited = match a.origin {
            Origin::Seed => ONE32,
            Origin::Lateral { parent, node, .. } => {
                mul(q32(axes[parent].phytomers[node].scale), base[parent])
            }
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => base[parent],
        };
        let share = match a.origin {
            Origin::Lateral { .. } => dominance(species.states[a.pa].form.dominance, a.lineage),
            _ => 1.0,
        };
        base[i] = mul(mul(inherited, q32(a.vigour)), q32(share));
    }
    base
}

/// The tree as grown, sized as `geometry::scale` and `girth::thicken`
/// will size it, with the leaf term's mean taken over all of it, on up to
/// `threads` cores.
pub(crate) fn attachments(axes: &[Axis], species: &Species, age: u32, threads: usize) -> Grown {
    let base = bases(axes, species);
    let scale = |i: usize, p: &Phytomer| real(mul(mul(q32(p.scale), base[i]), q32(p.size)));
    let leaf_mean = species.states.iter().any(|s| s.leaf_girth > 0.0).then(|| {
        // Each phytomer's pipe and its light term on every core, summed
        // in the tree's order.
        let pieces: Vec<usize> = (0..axes.len().div_ceil(PIECE)).collect();
        let terms = map(pieces, 1, threads, &|_, k| {
            let mut out = Vec::new();
            for i in k * PIECE..((k + 1) * PIECE).min(axes.len()) {
                let state = &species.states[axes[i].pa];
                for p in &axes[i].phytomers {
                    let pipe = own(&state.form, p, age, scale(i, p), 1.0);
                    out.push(terms(pipe, p.light, state.leaf_girth));
                }
            }
            out
        });
        // Exact and order-free (contract A, 2.3).
        let (mut total, mut weighted) = (Total::default(), Total::default());
        for (pipe, term) in terms.into_iter().flatten() {
            total.add(pipe);
            weighted.add(term);
        }
        mean_of(total, weighted).unwrap_or(1.0)
    });
    let parents: Vec<Option<usize>> = axes.iter().map(|a| a.origin.parent()).collect();
    let tree = Tree::new(&parents);
    let mut none = vec![(); axes.len()];
    // Each axis's base radius: its own pipes and those its children carry
    // into its nodes and tip, the last child first.
    let radii = tree.upward(
        &mut none,
        threads,
        &|i, (), radii: &[f64], at_node: &mut Vec<i128>| {
            let axis = &axes[i];
            let state = &species.states[axis.pa];
            let e = state.form.exponent;
            at_node.clear();
            at_node.resize(axis.phytomers.len(), 0);
            let mut at_tip = 0i128;
            for &c in tree.children(i) {
                let section = section(radii[c], e);
                match axes[c].origin {
                    Origin::Lateral { node, .. } if node < at_node.len() => {
                        at_node[node] += section
                    }
                    _ => at_tip += section,
                }
            }
            let mut section = at_tip;
            for (k, p) in axis.phytomers.iter().enumerate().rev() {
                let leaf = super::leaf(p.light, state.leaf_girth, leaf_mean);
                section += super::section(own(&state.form, p, age, scale(i, p), leaf), e) + at_node[k];
            }
            radius(section, e)
        },
    );
    let links = axes
        .iter()
        .zip(radii)
        .map(|(a, radius)| Link {
            parent: a.origin.parent(),
            node: match a.origin {
                Origin::Lateral { node, .. } => Some(node),
                _ => None,
            },
            pa: a.pa,
            sheddable: matches!(a.origin, Origin::Lateral { .. } | Origin::Relay { .. }),
            radius,
        })
        .collect();
    Grown { links, leaf_mean }
}

/// What thickening keeps of the branches shed and fading: a shed branch
/// whose bearer is kept leaves its PA's share of its pipe; a kept branch
/// leaves that share of what its fade has taken. Each radius is scaled by
/// its bearers' fades, as the branch's own wood is.
pub(crate) fn disused(grown: &Grown, shed: &Shed, species: &Species) -> Girth {
    let n = grown.links.len();
    let mut above = vec![ONE32; n];
    for i in 0..n {
        if let Some(p) = grown.links[i].parent {
            above[i] = mul(above[p], q32(shed.fade[p]));
        }
    }
    let mut disused = Vec::new();
    for (i, link) in grown.links.iter().enumerate() {
        let share = species.states[link.pa].retained;
        let Some(parent) = link.parent else {
            continue;
        };
        if share <= 0.0 || !link.sheddable || shed.index[parent] == usize::MAX {
            continue;
        }
        let taken = if shed.index[i] == usize::MAX {
            ONE32
        } else {
            ONE32 - q32(shed.fade[i])
        };
        if taken > 0 {
            disused.push(Disused {
                axis: shed.index[parent],
                node: link.node,
                radius: real(mul(q32(link.radius), above[i])),
                share: real(mul(q32(share), taken)),
            });
        }
    }
    Girth {
        disused,
        leaf_mean: grown.leaf_mean,
    }
}
