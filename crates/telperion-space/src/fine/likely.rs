//! Which roots the spare cores grow ahead for the landing, likeliest
//! first (fn-209 F1 task 9, `spare.rs`). A guess about where work runs:
//! the landing decides by the ground bound itself (`landing.rs`) and
//! grows any root not grown ahead, so a wrong guess costs time only.
//!
//! The guess is the lowest a root's bearer node could hang, its branch
//! straight down from where it leaves the trunk, less the root's
//! closed-form reach: a root no lower than the landing zone by that
//! cannot land, so long as the trunk stands where it was laid before
//! bending. Before the tree is laid the trunk's height is read as its
//! length; once laid, as laid, and the roots that land on the tree so
//! laid come first.
use super::expansion::{node, RootSeed};
use super::landing::reaches;
use super::realized::Stub;
use crate::closed_form::boundary::Rows;
use crate::fixed::presence::{real, times};
use crate::geometry::ground_zone;
use crate::species::Species;
use crate::structure::{Axis, Origin, Structure};

/// Where each axis leaves the trunk: none on it; for a branch, the
/// trunk axis and node, and its length from there to the axis's base.
type From = Option<(usize, usize, f64)>;

/// The live roots, with their bearers and scales.
pub(crate) type Live<'a> = (&'a [RootSeed], &'a [usize], &'a [usize], &'a [(f64, f64)]);

/// Before the tree is laid: the live roots that could reach the ground by
/// the trunk's length, likeliest first.
pub(crate) fn before_laid(
    (species, tree, rows): (&Species, &Structure, &Rows),
    live: Live,
) -> Vec<usize> {
    let lengths: Vec<Vec<f64>> = tree.axes.iter().map(|a| lengths(species, a)).collect();
    let mut up = vec![0.0; tree.axes.len()];
    let height = |i: usize, node: usize, up: &[f64]| up[i] + to(&lengths[i], node);
    let from = from_trunk(tree, &lengths, &mut up, &height);
    likeliest(
        (rows, &lengths, &from),
        live,
        &|b, n| up[b] + to(&lengths[b], n),
        &|_| 0.0,
    )
}

/// Once the tree is laid before bending (`unbent`, each live root's
/// stub): the roots that land on it, then, where any PA sags, those that
/// could by the trunk as laid, likeliest first.
pub(crate) fn once_laid(
    (species, tree, rows): (&Species, &Structure, &Rows),
    live: Live,
    (radii, unbent, internode): (&[f64], &[Stub], f64),
    sags: bool,
) -> Vec<usize> {
    let (seeds, roots, _, scales) = live;
    let mut out: Vec<usize> = (0..roots.len())
        .filter(|&j| {
            reaches(
                rows,
                internode,
                &seeds[roots[j]],
                scales[j],
                radii[j],
                unbent[j].low(),
            )
        })
        .collect();
    if !sags {
        return out;
    }
    let lengths: Vec<Vec<f64>> = tree.axes.iter().map(|a| lengths(species, a)).collect();
    let mut up = vec![0.0; tree.axes.len()];
    let height = |i: usize, node: usize, _: &[f64]| {
        tree.axes[i].phytomers[node.min(last(&tree.axes[i]))].tip.z
    };
    let from = from_trunk(tree, &lengths, &mut up, &height);
    let zone = |j: usize| {
        let (base, drawn) = scales[j];
        let s = base * drawn * real(times(seeds[roots[j]].birth[0], seeds[roots[j]].birth[1]));
        ground_zone(2.0 * radii[j], internode * s)
    };
    let low = |b: usize, n: usize| height(b, n, &up);
    let more = likeliest((rows, &lengths, &from), live, &low, &zone);
    let first: std::collections::HashSet<usize> = out.iter().copied().collect();
    out.extend(more.into_iter().filter(|j| !first.contains(j)));
    out
}

/// The live roots whose bearer node, hung from the trunk at `height`,
/// stands no higher than their reach and `zone`, the lowest first.
fn likeliest(
    (rows, lengths, from): (&Rows, &[Vec<f64>], &[From]),
    (seeds, live, bearers, scales): Live,
    height: &dyn Fn(usize, usize) -> f64,
    zone: &dyn Fn(usize) -> f64,
) -> Vec<usize> {
    let mut low: Vec<(f64, usize)> = (0..live.len())
        .filter_map(|j| {
            let (seed, b) = (&seeds[live[j]], bearers[j]);
            let (base, drawn) = scales[j];
            let axis = &seed.handed.bud.axis;
            let s = base * drawn * real(times(seed.birth[0], seed.birth[1]));
            let reach = rows.lateral(axis.pa, axis.birth).reach * s;
            let along = to(&lengths[b], node(seed));
            let low = match from[b] {
                None => height(b, node(seed)),
                Some((t, n, path)) => height(t, n) - path - along,
            };
            let margin = low - reach - zone(j);
            (margin <= 0.0).then_some((margin, j))
        })
        .collect();
    low.sort_by(|a, b| a.0.total_cmp(&b.0));
    low.into_iter().map(|(_, j)| j).collect()
}

fn last(axis: &Axis) -> usize {
    axis.phytomers.len().saturating_sub(1)
}

/// Each length along `axis` to the tip of its phytomers in turn.
fn lengths(species: &Species, axis: &Axis) -> Vec<f64> {
    let internode = species.states[axis.pa].internode;
    let mut run = 0.0;
    let along = axis.phytomers.iter().map(|p| {
        run += internode * p.scale;
        run
    });
    along.collect()
}

/// The length to the tip of the `node`th phytomer, from `lengths`.
fn to(lengths: &[f64], node: usize) -> f64 {
    lengths.get(node).or(lengths.last()).copied().unwrap_or(0.0)
}

/// For each axis: none on the trunk (`up`, each trunk axis's base's
/// height by `height`); for a branch, the trunk axis and node it leaves
/// the trunk at and its length from there to the axis's base.
fn from_trunk(
    tree: &Structure,
    lengths: &[Vec<f64>],
    up: &mut [f64],
    height: &dyn Fn(usize, usize, &[f64]) -> f64,
) -> Vec<From> {
    let mut from: Vec<From> = vec![None; tree.axes.len()];
    for (i, axis) in tree.axes.iter().enumerate() {
        let (on, node) = match axis.origin {
            Origin::Seed => continue,
            Origin::Continuation { parent } => (parent, usize::MAX),
            Origin::Relay { parent, node } | Origin::Lateral { parent, node, .. } => (parent, node),
        };
        let lateral = matches!(axis.origin, Origin::Lateral { .. });
        from[i] = match (from[on], lateral) {
            (None, false) => {
                up[i] = height(on, node, up);
                None
            }
            (None, true) => Some((on, node, 0.0)),
            (Some((t, n, path)), _) => Some((t, n, path + to(&lengths[on], node))),
        };
    }
    from
}
