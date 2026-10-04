//! Girth by the pipe model (as Pałubicki et al. 2009 use it): a section is
//! the sum of the sections it carries. Each phytomer adds its PA's pipe,
//! scaled by its presence, so a branch growing in thickens what bears it
//! by degree.
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
    // carried section before the wood that carries it.
    for i in (0..axes.len()).rev() {
        let pipe = species.states[axes[i].pa].form.pipe;
        let mut section = at_tip[i];
        for (k, phytomer) in axes[i].phytomers.iter_mut().enumerate().rev() {
            let own = pipe * phytomer.scale;
            section += own * own + at_node[i][k];
            phytomer.radius = section.sqrt();
        }
        match axes[i].origin {
            Origin::Seed => {}
            Origin::Lateral { parent, node, .. } => match at_node[parent].get_mut(node) {
                Some(at) => *at += section,
                None => at_tip[parent] += section,
            },
            // A relay's section enters at its node as far as it stands
            // there, at the tip as far as it is still the continuation.
            Origin::Relay { parent, node } => {
                let b = axes[i].blend;
                match at_node[parent].get_mut(node) {
                    Some(at) => {
                        *at += section * b;
                        at_tip[parent] += section * (1.0 - b);
                    }
                    None => at_tip[parent] += section,
                }
            }
            Origin::Continuation { parent } => at_tip[parent] += section,
        }
    }
}
