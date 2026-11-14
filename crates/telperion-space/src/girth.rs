//! Girth by sapwood and heartwood (fn-354; the rule of PROBE-BRIEF.md as
//! PRODUCT-DESIGN.md makes it product). Each phytomer carries its whole
//! section and the sapwood it holds, in m², from the year it is laid: a new
//! phytomer's section is its primary radius's at its scale. Each year one
//! pass from the tips down gives every held phytomer the leaf above it
//! that year, its own, its axis's beyond it and every child's at or beyond
//! its node (a lateral at its node, a continuation or relay at the tip),
//! and lays what its sapwood is short of:
//!
//! ```text
//! need = leaf × sapwood × 1e-4    kept = S (1 − 1 / rings)
//! laid = max(0, need − kept)      T += laid    S = kept + laid
//! ```
//!
//! A species whose source measured the two rows at two tree sizes states
//! both, and the wood reads them by the tree's height that year (`tall`,
//! `rows`): no age class and no switch, one value where one is stated.
//!
//! Wood is never lost: a crown removed or shed lays nothing more and takes
//! nothing away. At the settle a phytomer's radius is sqrt(T × share / π),
//! `share` the fade its axis is drawn at: its applied fade, the least on
//! its way from the seed, the same number its scale carries (a subtree
//! fades once, fn-359).
use crate::species::{Form, PaState, Species};
use crate::structure::{Axis, Join, Node, Origin, Structure};
use crate::upward::Tree;
use std::f64::consts::PI;

#[cfg(test)]
mod tests;

/// The radius a phytomer was established with: its PA's primary, by its
/// scale.
pub(crate) fn established(form: &Form, scale: f64) -> f64 {
    f64::from(form.primary as f32 * scale as f32)
}

/// Whether any age lays wood beyond its primary section.
pub(crate) fn lays(species: &Species) -> bool {
    species.states.iter().any(|s| s.form.sapwood > 0.0)
}

/// The leaf phytomer `p` of a PA in `state` bears in `year`, m², while its
/// wood is open (fn-354 R5, LEAF-ACCOUNT.md as the host corrected it on
/// 12 Nov): its own leaf while its age is under its PA's `leaf_years`, the
/// last year's share of it at the fraction. Wood that has closed, on the
/// way to no living apex or sleeping bud, bears none; its callers leave it
/// out (`Working::closed`). The one leaf the light's deposit and the wood
/// read.
///
/// ```text
/// leaf = leaf_area × scale × size × clamp(leaf_years − age, 0, 1)
/// ```
pub(crate) fn leaf(state: &PaState, p: &impl Node, year: u32) -> f64 {
    let kept = (state.leaf_years - f64::from(year.saturating_sub(p.cycle()))).clamp(0.0, 1.0);
    crate::light::leaf_area(p.scale(), p.size(), state.leaf_area) * kept
}

/// The first of an axis's `phytomers` that bears leaf in `year`: those
/// before it are older than its PA's `leaf_years`. Laid in order, an
/// axis's phytomers are no younger than those before them.
pub(crate) fn bearing(state: &PaState, phytomers: &[impl Node], year: u32) -> usize {
    phytomers.partition_point(|p| f64::from(year.saturating_sub(p.cycle())) >= state.leaf_years)
}

/// A section and its sapwood, `[T, S]`, one year on under `need`, the
/// sapwood its leaf needs, at `rings` years of sapwood.
pub(crate) fn ring([t, s]: [f64; 2], need: f64, rings: f64) -> [f64; 2] {
    let kept = s * (1.0 - 1.0 / rings);
    let laid = (need - kept).max(0.0);
    [t + laid, kept + laid]
}

/// How far a tree of `height` metres stands between the two heights its
/// wood's rows are stated at: 0 up to `sapwood_from`, 1 from `sapwood_to`,
/// a straight line between. Outside them it holds the nearer row: nothing
/// was measured there, so nothing is extrapolated.
pub(crate) fn tall(form: &Form, height: f64) -> f64 {
    let (from, to) = (form.sapwood_from, form.sapwood_to);
    if height <= from {
        0.0
    } else if height >= to {
        1.0
    } else {
        (height - from) / (to - from)
    }
}

/// The wood's two rows on a tree `tall` of the way between its stated
/// heights: the sapwood a unit of leaf needs, cm² per m², and the years it
/// stays sapwood, each its own value times `1 + tall × (share − 1)`. At
/// shares of 1 they are the rows themselves, to the bit.
pub(crate) fn rows(form: &Form, tall: f64) -> [f64; 2] {
    let at = |share: f64| 1.0 + tall * (share - 1.0);
    [
        form.sapwood * at(form.sapwood_tall),
        form.sapwood_rings * at(form.sapwood_rings_tall),
    ]
}

/// The tree's height, m: its highest axis tip above the seed's base.
fn height<P: Node>(axes: &[Axis<P>]) -> f64 {
    let Some(seed) = axes.first() else { return 0.0 };
    let top = |a: &Axis<P>| a.phytomers.last().map_or(a.base.z, |p| p.tip().z);
    axes.iter().map(top).fold(seed.base.z, f64::max) - seed.base.z
}

/// The year's wood laid on the growing tree on up to `threads` cores: each
/// phytomer laid since the last pass given its primary section, then every
/// held phytomer's sapwood set by the leaf above it in `year`, each axis's
/// leaf (`leaf`), at its PA's rows for the tree's height that year (`rows`).
///
/// An axis `closed` names is skipped, its subtree with it: nothing in it
/// lives or sleeps, so it bears no leaf and none stands above it again, and
/// it lays nothing, its section exact. Its sapwood stops retiring and is
/// unread while it is skipped. Closed wood never reopens today
/// (`Working::open_from`); wood that did would first retire its sapwood
/// for the `k` years skipped, `S (1 − 1 / rings)^k`.
pub(crate) fn lay<P: Node + Send>(
    axes: &mut [Axis<P>],
    species: &Species,
    year: u32,
    (closed, threads): (&(dyn Fn(usize) -> bool + Sync), usize),
) {
    let links: Vec<Origin> = axes.iter().map(|a| a.origin).collect();
    let parents: Vec<Option<usize>> = links.iter().map(|l| l.parent()).collect();
    let tree = Tree::new(&parents);
    let height = height(axes);
    tree.upward(
        axes,
        threads,
        &|i, axis, done: &[f64], at_node: &mut Vec<f64>| {
            if closed(i) {
                return 0.0;
            }
            let state = &species.states[axis.pa];
            let form = &state.form;
            at_node.clear();
            at_node.resize(axis.phytomers.len(), 0.0);
            let mut sum = 0.0;
            for &c in tree.children(i) {
                match links[c] {
                    Origin::Lateral { node, .. } if node < at_node.len() => {
                        at_node[node] += done[c]
                    }
                    _ => sum += done[c],
                }
            }
            let [sapwood, rings] = rows(form, tall(form, height));
            let per_leaf = sapwood * 1e-4;
            for (k, p) in axis.phytomers.iter_mut().enumerate().rev() {
                sum += leaf(state, p, year) + at_node[k];
                let scale = p.scale();
                let (section, sapwood) = p.wood();
                if *section == 0.0 {
                    let e = established(form, scale);
                    *section = (PI * e * e) as f32;
                }
                let held = [f64::from(*section), f64::from(*sapwood)];
                let [t, s] = ring(held, sum * per_leaf, rings);
                (*section, *sapwood) = (t as f32, s as f32);
            }
            sum
        },
    );
}

/// Every phytomer's radius from its wood, `sqrt(T × share / π)`, `shares`
/// each axis's applied fade (`Shed::applied`); each relay joined at the radius of the wood
/// its join stands in, on both sides, since its leaf enters its bearer at
/// the tip.
pub(crate) fn thicken(structure: &mut Structure, species: &Species, shares: &[f64]) {
    for (axis, &share) in structure.axes.iter_mut().zip(shares) {
        for p in &mut axis.phytomers {
            p.radius = (f64::from(p.section) * share / PI).sqrt();
        }
    }
    for i in 0..structure.axes.len() {
        if !matches!(structure.axes[i].origin, Origin::Relay { .. }) {
            continue;
        }
        let Some((node, t)) = attached(&structure.axes, species, i) else {
            continue;
        };
        let parent = structure.axes[i].origin.parent().expect("a relay's bearer");
        let r = structure.axes[parent].phytomers[node].radius;
        structure.axes[i].joined = Some(Box::new(Join {
            node,
            t,
            below: r,
            above: r,
        }));
    }
}

/// Axis `i`'s attachment as its parent's node and share
/// (`geometry::attach`, `Join`): none for all but a relay on a parent with
/// nodes.
pub(crate) fn attached(axes: &[Axis], species: &Species, i: usize) -> Option<(usize, f32)> {
    let x = crate::geometry::attach(axes, species, i)?;
    let last = axes[axes[i].origin.parent()?]
        .phytomers
        .len()
        .checked_sub(1)?;
    Some(crate::geometry::split(x, last))
}
