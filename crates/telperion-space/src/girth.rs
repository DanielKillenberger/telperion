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

use crate::contract::{exp2, log2};
use crate::species::{Form, Species};
use crate::structure::{Axis, Join, Origin, Phytomer, Structure};
use crate::upward::Tree;
pub(crate) use disused::{
    above, attachments, attachments_given, attachments_past, bases, disused, disused_below,
    grown_mean, leaf_partials, Grew, Stand,
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

/// A phytomer's own pipe: its PA's, by its scale, ripened, by `leaf`.
pub(crate) fn own(form: &Form, phytomer: &Phytomer, age: u32, scale: f64, leaf: f64) -> f64 {
    own_at(form, phytomer.cycle, age, scale, leaf)
}

/// `own` of a phytomer grown in `cycle`.
pub(crate) fn own_at(form: &Form, cycle: u32, age: u32, scale: f64, leaf: f64) -> f64 {
    let years = (age.saturating_sub(cycle) + 1) as f32;
    let ripe = ripened(years, form.ripening as f32);
    f64::from(form.pipe as f32 * scale as f32 * ripe * leaf as f32)
}

/// The pipe a phytomer was established with: its PA's, by its scale,
/// ripened over its first year.
pub(crate) fn established(form: &Form, scale: f64) -> f64 {
    let ripe = ripened(1.0, form.ripening as f32);
    f64::from(form.pipe as f32 * scale as f32 * ripe)
}

/// `min(years / ripening, 1)`: one where the ripening is none or less.
fn ripened(years: f32, ripening: f32) -> f32 {
    if ripening <= 0.0 {
        return 1.0;
    }
    (years / ripening).min(1.0)
}

/// A radius's section at exponent `e`: `2^(e log2 r)`, none for no radius.
pub(crate) fn section(radius: f64, e: f64) -> f32 {
    let r = radius as f32;
    if r <= 0.0 {
        return 0.0;
    }
    exp2(e as f32 * log2(r))
}

/// A section's radius at exponent `e`: `2^(log2 s / e)`, the reciprocal
/// `1/e` taken once; none for no section.
pub(crate) fn radius(section: f32, e: f64) -> f64 {
    if section <= 0.0 {
        return 0.0;
    }
    f64::from(exp2((1.0 / e) as f32 * log2(section)))
}

/// `s root + (1 - s) established`: secondary growth's share.
fn weighed(form: &Form, root: f64, established: f64) -> f64 {
    let s = form.secondary as f32;
    f64::from(s * root as f32 + (1.0 - s) * established as f32)
}

/// A phytomer's light term, light^χ, whole at χ none.
pub(crate) fn lit(light: f64, chi: f64) -> f32 {
    let (l, chi) = (light as f32, chi as f32);
    if chi == 0.0 {
        return 1.0;
    }
    if l <= 0.0 {
        return 0.0;
    }
    exp2(chi * log2(l))
}

/// A phytomer's leaf factor: its light term times the tree's mean.
pub(crate) fn leaf(light: f64, chi: f64, mean: Option<f64>) -> f64 {
    mean.map_or(1.0, |m| f64::from(lit(light, chi) * m as f32))
}

/// The tree's pipe-weighted mean from its sums, `Σpipe / Σpipe light^χ`;
/// none where the weighted sum is.
pub(crate) fn mean_of(total: f32, weighted: f32) -> Option<f64> {
    (weighted > 0.0).then(|| f64::from(total / weighted))
}

/// The terms a block of the leaf mean's reduction holds.
pub(crate) const LEAF_BLOCK: usize = 1024;

/// The leaf mean's two sums in the one reduction order both executors
/// take (fn-256 DESIGN.md sections 2 and 9.4): the terms in the order the
/// axes and their phytomers stand, in blocks of `LEAF_BLOCK`, each block
/// summed in order from zero, the blocks then added in order.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Blocked {
    sums: [f32; 2],
    block: [f32; 2],
    held: usize,
}

impl Blocked {
    /// The next term in the order.
    pub fn push(&mut self, (pipe, term): (f32, f32)) {
        self.block[0] += pipe;
        self.block[1] += term;
        self.held += 1;
        if self.held == LEAF_BLOCK {
            self.close();
        }
    }

    fn close(&mut self) {
        self.sums[0] += self.block[0];
        self.sums[1] += self.block[1];
        (self.block, self.held) = ([0.0; 2], 0);
    }

    /// The two sums, the last block added where it holds any term.
    pub fn sums(mut self) -> [f32; 2] {
        if self.held > 0 {
            self.close();
        }
        self.sums
    }
}

/// A pipe's two terms for the mean: the pipe, and the pipe by its light
/// term.
pub(crate) fn terms(pipe: f64, light: f64, chi: f64) -> (f32, f32) {
    let p = pipe as f32;
    (p, p * lit(light, chi))
}

/// Each PA's light term for each phytomer, (light)^χ, and the factor
/// that keeps the tree's pipe-weighted mean whole: χ 0 everywhere leaves
/// every pipe as it was, to the bit.
pub(super) fn leaf_terms(structure: &Structure, species: &Species) -> Option<f64> {
    if species.states.iter().all(|s| s.leaf_girth == 0.0) {
        return None;
    }
    let mut sums = Blocked::default();
    for axis in &structure.axes {
        let state = &species.states[axis.pa];
        for p in &axis.phytomers {
            let pipe = own(&state.form, p, structure.age, p.scale, 1.0);
            sums.push(terms(pipe, p.light, state.leaf_girth));
        }
    }
    let [total, weighted] = sums.sums();
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

/// A relay's section as its parent takes it at the relay's attachment
/// (`geometry::attach`): `t` of it at the node whose span it stands in,
/// the rest at the node before (or the base), so the parent's samples
/// move by degree as it moves along (fn-199 R4); the parts.
pub(crate) fn joining(x: f32, t: f32) -> (f32, f32) {
    let at = x * t;
    (at, x - at)
}

/// `thicken`, each axis `given` names standing for a subtree grown
/// elsewhere whose base radius it gives (fn-209 F1, realized mechanics):
/// each axis's base radius, by index. Each relay gets its parent's girth
/// where it joins (`Axis::joined`).
pub(crate) fn thicken_given(
    structure: &mut Structure,
    species: &Species,
    girth: &Girth,
    given: crate::shed::Given<f64>,
    threads: usize,
) -> Vec<f64> {
    let age = structure.age;
    let mean = girth.leaf_mean.or_else(|| leaf_terms(structure, species));
    let attached: Vec<_> = (0..structure.axes.len())
        .map(|i| self::attached(&structure.axes, species, i))
        .collect();
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
    // parent's own exponent; each relay's join is read where it stands.
    let given_out = tree.upward(
        axes,
        threads,
        &|i, axis, out: &[(f64, Vec<(usize, Join)>)], at_node: &mut Vec<f32>| {
            let state = &species.states[axis.pa];
            let form = state.form;
            let e = form.exponent;
            at_node.clear();
            at_node.resize(axis.phytomers.len(), 0.0);
            let (mut at_tip, mut at_base) = (0.0f32, 0.0f32);
            for d in &kept[i] {
                let section = section(d.radius, e) * d.share as f32;
                match d.node.and_then(|n| at_node.get_mut(n)) {
                    Some(at) => *at += section,
                    None => at_tip += section,
                }
            }
            // Each relay's node, share and parts, for its join.
            let mut relays: Vec<(usize, usize, f32, f32, f32)> = Vec::new();
            for &c in tree.children(i) {
                let section = section(given(c).unwrap_or(out[c].0), e);
                match (links[c], attached[c]) {
                    (Origin::Seed, _) => {}
                    (Origin::Lateral { node, .. }, _) => match at_node.get_mut(node) {
                        Some(at) => *at += section,
                        None => at_tip += section,
                    },
                    // A relay's wood enters where it joins (fn-244).
                    (Origin::Relay { .. }, Some((k, t))) => {
                        let (at, below) = joining(section, t);
                        at_node[k] += at;
                        match k.checked_sub(1) {
                            Some(j) => at_node[j] += below,
                            None => at_base += below,
                        }
                        relays.push((c, k, t, at, below));
                    }
                    (Origin::Relay { .. } | Origin::Continuation { .. }, _) => at_tip += section,
                }
            }
            let mut joins = Vec::with_capacity(relays.len());
            let mut section = at_tip;
            for (k, phytomer) in axis.phytomers.iter_mut().enumerate().rev() {
                let leaf = leaf(phytomer.light, state.leaf_girth, mean);
                let own = own(&state.form, phytomer, age, phytomer.scale, leaf);
                section += self::section(own, e) + at_node[k];
                // Laid down at the apex, the phytomer carried nothing but itself.
                let established = established(&form, phytomer.scale);
                phytomer.radius = weighed(&form, radius(section, e), established);
                for &(c, _, t, at, below) in relays.iter().filter(|r| r.1 == k) {
                    let weigh = |s: f32| weighed(&form, radius(s, e), established);
                    let (below, above) = (weigh(section + below), weigh(section - at));
                    joins.push((c, Join { node: k, t, below, above }));
                }
            }
            (radius(section + at_base, e), joins)
        },
    );
    for (c, join) in given_out.iter().flat_map(|o| &o.1) {
        structure.axes[*c].joined = Some(*join);
    }
    given_out.into_iter().map(|o| o.0).collect()
}

/// Axis `i`'s attachment as its parent's node and share
/// (`geometry::attach`, `Join`): none for all but a relay on a parent with
/// nodes.
pub(crate) fn attached(axes: &[Axis], species: &Species, i: usize) -> Option<(usize, f32)> {
    let x = crate::geometry::attach(axes, species, i)?;
    let last = axes[axes[i].origin.parent()?].phytomers.len().checked_sub(1)?;
    Some(crate::geometry::split(x, last))
}

/// How far a phytomer `years` old has laid down its own wood, ripening
/// over `ripening` years: whole at once where `ripening` is none or less.
pub(crate) fn ripe(years: f64, ripening: f64) -> f64 {
    f64::from(ripened(years as f32, ripening as f32))
}

#[cfg(test)]
mod tests;
