//! Shinozaki's disused pipes (`PaState::retained`): a branch being shed
//! leaves a share of the pipe it had laid down in its bearer. As a kept
//! branch fades towards its shedding (`shed.rs`), its retained share grows
//! in by what its fade has taken, so its bearer meets the same section on
//! both sides of the branch's removal.
use super::{mean_of, own, radius, section, terms, Disused, Girth};
use crate::cores::{map, PIECE};
use crate::geometry::dominance;
use crate::shed::{Given, Shed};
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

/// Each axis's scale at its base, as `geometry::scale` will give it
/// (parents first, in its order of products); axis 0's is `top` where
/// given (a fine root's bearer's stub: its bearer's base).
pub(crate) fn bases(axes: &[Axis], species: &Species, top: Option<f32>) -> Vec<f32> {
    let mut base = vec![1.0f32; axes.len()];
    for (i, a) in axes.iter().enumerate() {
        let inherited = match a.origin {
            Origin::Seed => 1.0,
            Origin::Lateral { parent, node, .. } => {
                axes[parent].phytomers[node].scale as f32 * base[parent]
            }
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => base[parent],
        };
        let share = match a.origin {
            Origin::Lateral { .. } => dominance(species.states[a.pa].form.dominance, a.lineage),
            _ => 1.0,
        };
        base[i] = match (i, top) {
            (0, Some(top)) => top,
            _ => inherited * a.vigour as f32 * share as f32,
        };
    }
    base
}

/// A phytomer's scale as `geometry::scale` will give it: its scale by its
/// axis's base, then by its unit's size.
fn sized(p: &Phytomer, base: f32) -> f64 {
    f64::from(p.scale as f32 * base * p.size as f32)
}

/// Where a subtree as grown stands in the tree: axis 0's base (none:
/// axis 0 is the seed), and the first axis whose phytomers its leaf
/// partials count (1 for a fine root, past its bearer's stub).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Stand {
    pub top: Option<f32>,
    pub first: usize,
}

impl Stand {
    pub const WHOLE: Self = Self {
        top: None,
        first: 0,
    };
}

/// The leaf term's two sums over `axes` as grown, from `from.first` on:
/// each phytomer's pipe and its pipe by its light term (`terms`), taken
/// on up to `threads` cores and summed in the order the phytomers stand.
pub(crate) fn leaf_partials(
    axes: &[Axis],
    species: &Species,
    (age, threads): (u32, usize),
    from: Stand,
) -> [f32; 2] {
    let base = bases(axes, species, from.top);
    let scale = |i: usize, p: &Phytomer| sized(p, base[i]);
    let pieces: Vec<usize> = (0..axes.len().div_ceil(PIECE)).collect();
    let terms = map(pieces, 1, threads, &|_, k| {
        let mut out = Vec::new();
        for i in (k * PIECE).max(from.first)..((k + 1) * PIECE).min(axes.len()) {
            let state = &species.states[axes[i].pa];
            for p in &axes[i].phytomers {
                let pipe = own(&state.form, p, age, scale(i, p), 1.0);
                out.push(terms(pipe, p.light, state.leaf_girth));
            }
        }
        out
    });
    let (mut total, mut weighted) = (0.0, 0.0);
    for (pipe, term) in terms.into_iter().flatten() {
        total += pipe;
        weighted += term;
    }
    [total, weighted]
}

/// The leaf term's mean from its merged sums, whole where none is.
pub(crate) fn grown_mean([total, weighted]: [f32; 2]) -> f64 {
    mean_of(total, weighted).unwrap_or(1.0)
}

/// The tree as grown, sized as `geometry::scale` and `girth::thicken`
/// will size it, with the leaf term's mean taken over all of it, on up to
/// `threads` cores.
pub(crate) fn attachments(axes: &[Axis], species: &Species, age: u32, threads: usize) -> Grown {
    let leaf_mean = species
        .states
        .iter()
        .any(|s| s.leaf_girth > 0.0)
        .then(|| grown_mean(leaf_partials(axes, species, (age, threads), Stand::WHOLE)));
    attachments_given(
        axes,
        species,
        (age, threads),
        (Stand::WHOLE, leaf_mean),
        &|_| None,
    )
}

/// `attachments` under a mean taken elsewhere, each axis `given` names
/// standing for a subtree grown elsewhere whose base radius as grown it
/// gives (a fine root's stand-in, fn-211 task 12).
pub(crate) fn attachments_given(
    axes: &[Axis],
    species: &Species,
    (age, threads): (u32, usize),
    (from, leaf_mean): (Stand, Option<f64>),
    given: Given<f64>,
) -> Grown {
    let base = bases(axes, species, from.top);
    let scale = |i: usize, p: &Phytomer| sized(p, base[i]);
    let parents: Vec<Option<usize>> = axes.iter().map(|a| a.origin.parent()).collect();
    let tree = Tree::new(&parents);
    let mut none = vec![(); axes.len()];
    // Each axis's base radius: its own pipes and those its children carry
    // into its nodes and tip, the last child first.
    let radii = tree.upward(
        &mut none,
        threads,
        &|i, (), radii: &[f64], at_node: &mut Vec<f32>| {
            if let Some(r) = given(i) {
                return r;
            }
            let axis = &axes[i];
            let state = &species.states[axis.pa];
            let e = state.form.exponent;
            at_node.clear();
            at_node.resize(axis.phytomers.len(), 0.0);
            let mut at_tip = 0.0f32;
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
                section +=
                    super::section(own(&state.form, p, age, scale(i, p), leaf), e) + at_node[k];
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
    disused_below(grown, shed, species, None)
}

/// `disused`, in a fine root's tree where `stub` is given: axis 0 is its
/// bearer's stub, the root standing at `stub` of its bearers' fades, and
/// the root's own pipe is its bearer's to keep, so none is kept at the
/// stub.
pub(crate) fn disused_below(
    grown: &Grown,
    shed: &Shed,
    species: &Species,
    stub: Option<f32>,
) -> Girth {
    let parents: Vec<Option<usize>> = grown.links.iter().map(|l| l.parent).collect();
    let above = above(&parents, &shed.fade, stub.unwrap_or(1.0));
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
            1.0
        } else {
            1.0 - shed.fade[i] as f32
        };
        if taken > 0.0 {
            disused.push(Disused {
                axis: shed.index[parent],
                node: link.node,
                radius: f64::from(link.radius as f32 * above[i]),
                share: f64::from(share as f32 * taken),
            });
        }
    }
    if stub.is_some() {
        disused.retain(|d| d.axis != 0);
    }
    Girth {
        disused,
        leaf_mean: grown.leaf_mean,
    }
}

/// Each axis's share left by its bearers' fades, parents first, axis 0's
/// being `top`.
pub(crate) fn above(parents: &[Option<usize>], fade: &[f64], top: f32) -> Vec<f32> {
    let mut above = vec![1.0f32; parents.len()];
    if let Some(a) = above.first_mut() {
        *a = top;
    }
    for i in 0..parents.len() {
        if let Some(p) = parents[i] {
            above[i] = above[p] * fade[p] as f32;
        }
    }
    above
}

impl Grown {
    /// Axis `i`'s base radius as grown.
    pub fn radius(&self, i: usize) -> f64 {
        self.links[i].radius
    }
}
