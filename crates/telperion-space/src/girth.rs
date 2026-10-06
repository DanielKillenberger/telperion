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
    let (mut total, mut weighted) = (0.0, 0.0);
    for axis in &structure.axes {
        let state = &species.states[axis.pa];
        for p in &axis.phytomers {
            let pipe = own(&state.form, p, structure.age, p.scale, 1.0);
            total += pipe;
            weighted += pipe * p.light.powf(state.leaf_girth);
        }
    }
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
    tree.upward(axes, threads, &|i, axis, radii: &[f64]| {
        let state = &species.states[axis.pa];
        let form = state.form;
        let (e, s) = (form.exponent, form.secondary);
        let mut at_node = vec![0.0; axis.phytomers.len()];
        let mut at_tip = 0.0;
        for d in &kept[i] {
            let section = d.share * d.radius.powf(e);
            match d.node.and_then(|n| at_node.get_mut(n)) {
                Some(at) => *at += section,
                None => at_tip += section,
            }
        }
        for &c in tree.children(i) {
            let section = radii[c].powf(e);
            match links[c].0 {
                Origin::Seed => {}
                Origin::Lateral { node, .. } => match at_node.get_mut(node) {
                    Some(at) => *at += section,
                    None => at_tip += section,
                },
                // A relay's section enters at its node as far as it stands
                // there, at the tip as far as it is still the continuation.
                Origin::Relay { node, .. } => {
                    let b = links[c].1;
                    match at_node.get_mut(node) {
                        Some(at) => {
                            *at += section * b;
                            at_tip += section * (1.0 - b);
                        }
                        None => at_tip += section,
                    }
                }
                Origin::Continuation { .. } => at_tip += section,
            }
        }
        let mut section = at_tip;
        for (k, phytomer) in axis.phytomers.iter_mut().enumerate().rev() {
            let leaf = mean.map_or(1.0, |m| phytomer.light.powf(state.leaf_girth) * m);
            let own = own(&state.form, phytomer, age, phytomer.scale, leaf);
            section += own.powf(e) + at_node[k];
            // Laid down at the apex, the phytomer carried nothing but itself.
            let established = form.pipe * phytomer.scale * ripe(1.0, form.ripening);
            phytomer.radius = s * section.powf(1.0 / e) + (1.0 - s) * established;
        }
        section.powf(1.0 / e)
    });
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
