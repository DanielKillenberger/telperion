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
mod disused;

use crate::fixed::feedback::edge;
use crate::fixed::feedback::power::{exp2_w, log2_w};
use crate::fixed::feedback::total::Total;
use crate::fixed::feedback::wide::{mul_shr, prod_shr, ratio};
use crate::fixed::{exp2, log2, mul, q32, ONE32};
use crate::species::{Form, Species};
use crate::structure::{Origin, Phytomer, Structure};
use crate::upward::Tree;
pub(crate) use disused::{
    above, attachments, attachments_given, bases, disused, disused_below, grown_mean,
    leaf_partials, Stand,
};

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

/// A Q32 as its exact real.
fn real(q: i64) -> f64 {
    q as f64 / ONE32 as f64
}

/// The log2 of a positive Q32, Q32 (CONTRACT-A.md 2.3).
fn log2_q(x: i64) -> i64 {
    log2(x as u64) - (32 << 32)
}

/// A phytomer's own pipe: its PA's, by its scale, ripened, by `leaf`; one
/// rounding of the four (contract A, 2.5).
pub(crate) fn own(form: &Form, phytomer: &Phytomer, age: u32, scale: f64, leaf: f64) -> f64 {
    let years = i64::from(age.saturating_sub(phytomer.cycle) + 1) << 32;
    let ripe = edge::ripe(years, q32(form.ripening));
    let f = [q32(form.pipe), q32(scale), ripe, q32(leaf)];
    real(prod_shr(f, 96).expect("a pipe inside Q32"))
}

/// The pipe a phytomer was established with: its PA's, by its scale,
/// ripened over its first year; one rounding.
pub(crate) fn established(form: &Form, scale: f64) -> f64 {
    let ripe = edge::ripe(ONE32, q32(form.ripening));
    let f = [q32(form.pipe), q32(scale), ripe, 1];
    real(prod_shr(f, 64).expect("a pipe inside Q32"))
}

/// A radius's section at exponent `e`, in W: `2^(e log2 r)`, none for no
/// radius.
pub(crate) fn section(radius: f64, e: f64) -> i128 {
    let r = q32(radius);
    if r <= 0 {
        return 0;
    }
    exp2_w(mul(q32(e), log2_q(r))).expect("a section inside W") as i128
}

/// A section's radius at exponent `e`: `2^(log2 s / e)`, the reciprocal
/// `1/e` its own Q32 (CONTRACT-A.md 2.2); none for no section.
pub(crate) fn radius(section: i128, e: f64) -> f64 {
    if section <= 0 {
        return 0.0;
    }
    real(exp2(mul(q32(1.0 / e), log2_w(section as u128))))
}

/// `s root + (1 - s) established`, Q32: secondary growth's share.
fn weighed(form: &Form, root: f64, established: f64) -> f64 {
    let s = q32(form.secondary);
    real(mul(s, q32(root)) + mul(ONE32 - s, q32(established)))
}

/// `x b`, floored, of a W section and a Q32 share.
fn share_of(x: i128, b: f64) -> i128 {
    mul_shr(x, i128::from(q32(b)), 32).expect("a share of a section")
}

/// A phytomer's light term, light^χ (Q32), whole at χ none.
pub(crate) fn lit(light: f64, chi: f64) -> i64 {
    let (l, chi) = (q32(light), q32(chi));
    if chi == 0 {
        return ONE32;
    }
    if l <= 0 {
        return 0;
    }
    exp2(mul(chi, log2_q(l)))
}

/// A phytomer's leaf factor: its light term times the tree's mean.
pub(crate) fn leaf(light: f64, chi: f64, mean: Option<f64>) -> f64 {
    mean.map_or(1.0, |m| real(mul(lit(light, chi), q32(m))))
}

/// The tree's pipe-weighted mean from its exact sums, `Σpipe /
/// Σpipe light^χ` (W over W, to Q32); none where the weighted sum is.
pub(crate) fn mean_of(total: Total, weighted: Total) -> Option<f64> {
    let (t, w) = (total.get(), weighted.get());
    let (t, w) = (
        t.expect("a pipe sum inside W"),
        w.expect("a pipe sum inside W"),
    );
    (w > 0).then(|| {
        let q = ratio(t, w, 32).expect("a mean");
        real(i64::try_from(q).expect("a mean inside Q32"))
    })
}

/// A pipe's two terms for the mean: the pipe in W, and the pipe by its
/// light term (Q64, W).
pub(crate) fn terms(pipe: f64, light: f64, chi: f64) -> (i128, i128) {
    let p = i128::from(q32(pipe));
    (p << 32, p * i128::from(lit(light, chi)))
}

/// Each PA's light term for each phytomer, (light)^χ, and the factor
/// that keeps the tree's pipe-weighted mean whole: χ 0 everywhere leaves
/// every pipe as it was, to the bit.
pub(super) fn leaf_terms(structure: &Structure, species: &Species) -> Option<f64> {
    if species.states.iter().all(|s| s.leaf_girth == 0.0) {
        return None;
    }
    // Exact and order-free (contract A, 2.3).
    let (mut total, mut weighted) = (Total::default(), Total::default());
    for axis in &structure.axes {
        let state = &species.states[axis.pa];
        for p in &axis.phytomers {
            let pipe = own(&state.form, p, structure.age, p.scale, 1.0);
            let (a, b) = terms(pipe, p.light, state.leaf_girth);
            total.add(a);
            weighted.add(b);
        }
    }
    mean_of(total, weighted)
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
    let links: Vec<Origin> = axes.iter().map(|a| a.origin).collect();
    let parents: Vec<Option<usize>> = links.iter().map(|l| l.parent()).collect();
    let tree = Tree::new(&parents);
    // Section carried into each axis's phytomers by the pipes it keeps,
    // then by its laterals and relays, and into its tip by its
    // continuation, the last child first, as a backward pass meets them. Each
    // child hands on its base radius; its parent raises it to the
    // parent's own exponent.
    tree.upward(
        axes,
        threads,
        &|i, axis, radii: &[f64], at_node: &mut Vec<i128>| {
            let state = &species.states[axis.pa];
            let form = state.form;
            let e = form.exponent;
            at_node.clear();
            at_node.resize(axis.phytomers.len(), 0);
            let mut at_tip = 0i128;
            for d in &kept[i] {
                let section = share_of(section(d.radius, e), d.share);
                match d.node.and_then(|n| at_node.get_mut(n)) {
                    Some(at) => *at += section,
                    None => at_tip += section,
                }
            }
            for &c in tree.children(i) {
                let section = section(given(c).unwrap_or(radii[c]), e);
                match links[c] {
                    Origin::Seed => {}
                    // A relay's wood enters at the node it grows from, as a
                    // lateral's does, so its parent below carries it whole and
                    // the stopped module's head beyond only its own; the
                    // drawing hangs it there (fn-244 host decision).
                    Origin::Lateral { node, .. } | Origin::Relay { node, .. } => {
                        match at_node.get_mut(node) {
                            Some(at) => *at += section,
                            None => at_tip += section,
                        }
                    }
                    Origin::Continuation { .. } => at_tip += section,
                }
            }
            let mut section = at_tip;
            for (k, phytomer) in axis.phytomers.iter_mut().enumerate().rev() {
                let leaf = leaf(phytomer.light, state.leaf_girth, mean);
                let own = own(&state.form, phytomer, age, phytomer.scale, leaf);
                section += self::section(own, e) + at_node[k];
                // Laid down at the apex, the phytomer carried nothing but itself.
                let established = established(&form, phytomer.scale);
                phytomer.radius = weighed(&form, radius(section, e), established);
            }
            radius(section, e)
        },
    )
}

/// How far a phytomer `years` old has laid down its own wood, ripening
/// over `ripening` years: whole at once where `ripening` is none or less
/// (`edge::ripe`, Q32).
pub(crate) fn ripe(years: f64, ripening: f64) -> f64 {
    real(edge::ripe(q32(years), q32(ripening)))
}

#[cfg(test)]
mod tests;
