//! The exchange where any PA keeps its tree as grown, its shed pipes
//! (`retained`) or its leaves' girth (`leaf_girth`): host decision on
//! fn-211 task 12, PHASES.md's settle row. After phase 1 each root
//! reports up its leaf partials as grown, from its bearer's base and its
//! node's drawn scale handed down; the tree's mean comes down with each
//! root's bearers' fades at its base, and each root reports up its base
//! radius as grown, keeping its own shed pipes for phase 2. The partials
//! are exact and order-free (`fixed::sum`), so the mean is the whole
//! tree's whatever the roots' order.
use super::expansion::Roots;
use crate::error::Result;
use crate::fixed::ONE32;
use crate::girth::{above, attachments_given, bases, disused, grown_mean, leaf_partials};
use crate::girth::{Girth, Stand};
use crate::shed::Shed;
use crate::species::Species;
use crate::structure::Axis;

/// Up: every root's leaf partials as grown, merged with those of the
/// structure above the cut (`axes`, as grown); the tree's leaf mean, none
/// where no PA thickens by its leaves.
pub(super) fn partials(
    species: &Species,
    (age, threads): (u32, usize),
    axes: &[Axis],
    seats: &[(usize, usize)],
    roots: &mut dyn Roots,
) -> Result<Option<f64>> {
    let base = bases(axes, species, None);
    let stands: Vec<(i64, f64)> = seats
        .iter()
        .map(|&(at, node)| {
            let b = axes[at].origin.parent().expect("a root is borne");
            (base[b], axes[b].phytomers[node].scale)
        })
        .collect();
    let parts = roots.grown_partials(&stands)?;
    if species.states.iter().all(|s| s.leaf_girth == 0.0) {
        return Ok(None);
    }
    let [mut total, mut weighted] = leaf_partials(axes, species, (age, threads), Stand::WHOLE);
    for [t, w] in parts {
        total.merge(t);
        weighted.merge(w);
    }
    Ok(Some(grown_mean([total, weighted])))
}

/// Down: the tree's mean and each root's bearers' fades at its base;
/// up: each root's base radius as grown. The structure above the cut's
/// shed pipes, each root's stand-in (`stood`, by axis) standing for it.
pub(super) fn attach(
    species: &Species,
    (age, threads): (u32, usize),
    (axes, seats, stood): (&[Axis], &[(usize, usize)], &[Option<usize>]),
    (shed, mean): (&Shed, Option<f64>),
    roots: &mut dyn Roots,
) -> Result<Girth> {
    let parents: Vec<Option<usize>> = axes.iter().map(|a| a.origin.parent()).collect();
    let above = above(&parents, &shed.fade, ONE32);
    let at_roots: Vec<i64> = seats.iter().map(|&(at, _)| above[at]).collect();
    let radii = roots.grown_radii(mean, &at_roots)?;
    let given = |i: usize| stood[i].map(|k: usize| radii[k]);
    let grown = attachments_given(axes, species, (age, threads), (Stand::WHOLE, mean), &given);
    Ok(disused(&grown, shed, species))
}
