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
//! bearer (`retained`; Shinozaki's disused pipes).
use crate::species::{Form, Species};
use crate::structure::{Axis, Origin, Phytomer, Structure};

/// A shed branch's pipe that stays in its bearer: the bearer's index, the
/// node it stood at (none: the bearer's tip), its base radius, and the
/// share kept.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Disused {
    pub axis: usize,
    pub node: Option<usize>,
    pub radius: f64,
    pub share: f64,
}

/// A phytomer's own pipe: its PA's, by its scale, ripened, by `leaf`.
fn own(form: &Form, phytomer: &Phytomer, age: u32, scale: f64, leaf: f64) -> f64 {
    let years = f64::from(age.saturating_sub(phytomer.cycle)) + 1.0;
    form.pipe * scale * ripe(years, form.ripening) * leaf
}

/// Each PA's light term for each phytomer, (light)^χ, and the factor
/// that keeps the tree's pipe-weighted mean whole: χ 0 everywhere leaves
/// every pipe as it was, to the bit.
fn leaf_terms(structure: &Structure, species: &Species) -> Option<f64> {
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

/// Sets every phytomer's radius from the pipes it carries, and the shed
/// branches' pipes it keeps.
pub(crate) fn thicken(structure: &mut Structure, species: &Species, disused: &[Disused]) {
    let age = structure.age;
    let mean = leaf_terms(structure, species);
    let axes = &mut structure.axes;
    // Section carried into each axis's phytomers by its laterals, and into
    // its tip by its continuation or relay.
    let mut at_node: Vec<Vec<f64>> = axes.iter().map(|a| vec![0.0; a.phytomers.len()]).collect();
    let mut at_tip = vec![0.0; axes.len()];
    for d in disused {
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

/// Each axis's base radius on the tree as grown, before shedding, sized
/// as `geometry::scale` will size it: what a branch shed from it had laid
/// down. Read only where some PA retains its shed pipes.
pub(crate) fn grown_radii(axes: &[Axis], species: &Species, age: u32) -> Vec<f64> {
    let n = axes.len();
    // Each axis's scale at its base, as `geometry::scale` gives it.
    let mut base = vec![1.0; n];
    for i in 0..n {
        let a = &axes[i];
        let inherited = match a.origin {
            Origin::Seed => 1.0,
            Origin::Lateral { parent, node, .. } => {
                axes[parent].phytomers[node].scale * base[parent]
            }
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => base[parent],
        };
        let share = match a.origin {
            Origin::Lateral { .. } => {
                crate::geometry::dominance(species.states[a.pa].form.dominance, a.lineage)
            }
            _ => 1.0,
        };
        base[i] = inherited * a.vigour * share;
    }
    let mut at_tip = vec![0.0; n];
    let mut radius = vec![0.0; n];
    let mut at_node: Vec<Vec<f64>> = axes.iter().map(|a| vec![0.0; a.phytomers.len()]).collect();
    for i in (0..n).rev() {
        let form = &species.states[axes[i].pa].form;
        let e = form.exponent;
        let mut section = at_tip[i];
        for (k, p) in axes[i].phytomers.iter().enumerate().rev() {
            let own = own(form, p, age, p.scale * base[i] * p.size, 1.0);
            section += own.powf(e) + at_node[i][k];
        }
        radius[i] = section.powf(1.0 / e);
        if let Some(parent) = axes[i].origin.parent() {
            let section = radius[i].powf(species.states[axes[parent].pa].form.exponent);
            match axes[i].origin {
                Origin::Lateral { node, .. } if node < at_node[parent].len() => {
                    at_node[parent][node] += section
                }
                _ => at_tip[parent] += section,
            }
        }
    }
    radius
}

/// Each grown axis's bearer, the node it stood at (none: the tip), its PA
/// and its base radius as grown.
pub(crate) fn attachments(
    axes: &[Axis],
    species: &Species,
    age: u32,
) -> Vec<(Option<usize>, Option<usize>, usize, f64)> {
    let radii = grown_radii(axes, species, age);
    axes.iter()
        .zip(radii)
        .map(|(a, r)| {
            let node = match a.origin {
                Origin::Lateral { node, .. } => Some(node),
                _ => None,
            };
            (a.origin.parent(), node, a.pa, r)
        })
        .collect()
}

/// The shed branches whose bearer was kept, with the share of their pipe
/// their PA keeps there.
pub(crate) fn disused(
    grown: &[(Option<usize>, Option<usize>, usize, f64)],
    index: &[usize],
    species: &Species,
) -> Vec<Disused> {
    grown
        .iter()
        .enumerate()
        .filter_map(|(i, &(parent, node, pa, radius))| {
            let parent = parent?;
            let share = species.states[pa].retained;
            (index[i] == usize::MAX && index[parent] != usize::MAX && share > 0.0).then_some(
                Disused {
                    axis: index[parent],
                    node,
                    radius,
                    share,
                },
            )
        })
        .collect()
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
