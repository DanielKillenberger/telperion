//! Girth by the pipe model (as Pałubicki et al. 2009 use it): a section is
//! the sum of the sections it carries, a section being the radius to its
//! PA's exponent (2, the area, by default; a larger one keeps what it
//! carries thicker beside it). Each phytomer adds its PA's pipe, scaled by
//! its presence, so a branch growing in thickens what bears it by degree.
use crate::species::Species;
use crate::structure::{Origin, Structure};

/// Sets every phytomer's radius from the pipes it carries.
pub(crate) fn thicken(structure: &mut Structure, species: &Species) {
    let axes = &mut structure.axes;
    // Section carried into each axis's phytomers by its laterals, and into
    // its tip by its continuation or relay.
    let mut at_node: Vec<Vec<f64>> = axes.iter().map(|a| vec![0.0; a.phytomers.len()]).collect();
    let mut at_tip = vec![0.0; axes.len()];
    // Children follow their parents, so a backward pass meets every
    // carried section before the wood that carries it. Each child hands on
    // its base radius; its parent raises it to the parent's own exponent.
    for i in (0..axes.len()).rev() {
        let form = species.states[axes[i].pa].form;
        let e = form.exponent;
        let mut section = at_tip[i];
        for (k, phytomer) in axes[i].phytomers.iter_mut().enumerate().rev() {
            let own = form.pipe * phytomer.scale;
            section += own.powf(e) + at_node[i][k];
            phytomer.radius = section.powf(1.0 / e);
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
