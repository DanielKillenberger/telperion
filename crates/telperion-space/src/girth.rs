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
pub(super) fn own(form: &Form, phytomer: &Phytomer, age: u32, scale: f64, leaf: f64) -> f64 {
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

/// What thickening takes beyond the tree: the shed branches' pipes it
/// keeps, and the leaf term's mean where the tree as grown sets it.
#[derive(Debug, Clone, Default)]
pub(crate) struct Girth {
    pub disused: Vec<Disused>,
    pub leaf_mean: Option<f64>,
}

/// Sets every phytomer's radius from the pipes it carries, and the shed
/// branches' pipes it keeps.
pub(crate) fn thicken(structure: &mut Structure, species: &Species, girth: &Girth) {
    let age = structure.age;
    let mean = girth.leaf_mean.or_else(|| leaf_terms(structure, species));
    let disused = &girth.disused;
    let axes = &mut structure.axes;
    // Section carried into each axis's phytomers by its laterals, and into
    // its tip by its continuation or relay.
    let mut at_node: Vec<Vec<f64>> = axes.iter().map(|a| vec![0.0; a.phytomers.len()]).collect();
    let mut at_tip = vec![0.0; axes.len()];
    for d in disused.iter() {
        let section = d.share * d.radius.powf(species.states[axes[d.axis].pa].form.exponent);
        match d.node.and_then(|n| at_node[d.axis].get_mut(n)) {
            Some(at) => *at += section,
            None => at_tip[d.axis] += section,
        }
    }
    // Children follow their parents, so a backward pass meets every
    // carried section before the wood that carries it. Each child hands on
    // its base radius; its parent raises it to the parent's own exponent.
    for i in (0..axes.len()).rev() {
        let state = &species.states[axes[i].pa];
        let form = state.form;
        let e = form.exponent;
        let s = form.secondary;
        let mut section = at_tip[i];
        for (k, phytomer) in axes[i].phytomers.iter_mut().enumerate().rev() {
            let leaf = mean.map_or(1.0, |m| phytomer.light.powf(state.leaf_girth) * m);
            let own = own(&state.form, phytomer, age, phytomer.scale, leaf);
            section += own.powf(e) + at_node[i][k];
            // Laid down at the apex, the phytomer carried nothing but itself.
            let established = form.pipe * phytomer.scale * ripe(1.0, form.ripening);
            phytomer.radius = s * section.powf(1.0 / e) + (1.0 - s) * established;
        }
        let radius = section.powf(1.0 / e);
        let carried = |parent: usize| radius.powf(species.states[axes[parent].pa].form.exponent);
        match axes[i].origin {
            Origin::Seed => {}
            Origin::Lateral { parent, node, .. } => {
                let section = carried(parent);
                match at_node[parent].get_mut(node) {
                    Some(at) => *at += section,
                    None => at_tip[parent] += section,
                }
            }
            // A relay's section enters at its node as far as it stands
            // there, at the tip as far as it is still the continuation.
            Origin::Relay { parent, node } => {
                let section = carried(parent);
                let b = axes[i].blend;
                match at_node[parent].get_mut(node) {
                    Some(at) => {
                        *at += section * b;
                        at_tip[parent] += section * (1.0 - b);
                    }
                    None => at_tip[parent] += section,
                }
            }
            Origin::Continuation { parent } => at_tip[parent] += carried(parent),
        }
    }
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
