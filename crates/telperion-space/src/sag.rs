//! Sag (fn-200): an axis bends under the load it carries, as a beam does.
//! Each phytomer carries the wood and foliage beyond it: its own and its
//! axis's further phytomers, and every axis they bear. A phytomer's wood
//! weighs as its volume, its length times its radius squared; its
//! foliage as its own pipe's section times its length (the pipe model: a
//! pipe serves the leaves it bears). Both are sized by presence, so a
//! branch growing in loads its bearer by degree. The moment is taken on
//! the tree as it stands before it bends, as small-deflection beam theory
//! does, and growth never reads it back.
use crate::girth::ripe;
use crate::species::Species;
use crate::structure::{Origin, Structure, Vec3};

/// Load carried: its mass and its first moment (mass times position).
#[derive(Clone, Copy, Default)]
struct Load {
    mass: f64,
    moment: Vec3,
}

impl Load {
    fn add(&mut self, other: Load, share: f64) {
        self.mass += other.mass * share;
        self.moment = self.moment + other.moment * share;
    }
}

/// Whether any PA sags: none, and the tree is laid once, as before sag.
pub(crate) fn any(species: &Species) -> bool {
    species.states.iter().any(|s| s.form.sag > 0.0)
}

/// Each phytomer's gravity torque about its far end, horizontal: the axis
/// it turns down about, its length the bending moment.
pub(crate) fn torques(structure: &Structure, species: &Species) -> Vec<Vec<Vec3>> {
    let age = structure.age;
    let axes = &structure.axes;
    let mut at_node: Vec<Vec<Load>> = axes
        .iter()
        .map(|a| vec![Load::default(); a.phytomers.len()])
        .collect();
    let mut at_tip = vec![Load::default(); axes.len()];
    let mut torques: Vec<Vec<Vec3>> = axes
        .iter()
        .map(|a| vec![Vec3::default(); a.phytomers.len()])
        .collect();
    // Children follow their parents, so a backward pass meets every load
    // before the wood that carries it.
    for i in (0..axes.len()).rev() {
        let axis = &axes[i];
        let state = &species.states[axis.pa];
        let mut load = at_tip[i];
        for k in (0..axis.phytomers.len()).rev() {
            let p = &axis.phytomers[k];
            let base = if k == 0 {
                axis.base
            } else {
                axis.phytomers[k - 1].tip
            };
            let length = (p.tip - base).length();
            let years = f64::from(age.saturating_sub(p.cycle)) + 1.0;
            let leaves = state.form.pipe * p.scale * ripe(years, state.form.ripening);
            let mass = length * (p.radius * p.radius + leaves * leaves);
            let middle = (base + p.tip) * 0.5;
            // The moment at the phytomer's far end: what its node bears
            // and all beyond it, about that end. An unloaded tip bends
            // by nothing and keeps its tropism.
            load.add(at_node[i][k], 1.0);
            // (S - W e) x (-z): the lever of the load about the end,
            // crossed with gravity.
            let lever = load.moment - p.tip * load.mass;
            torques[i][k] = Vec3::new(-lever.y, lever.x, 0.0);
            load.add(
                Load {
                    mass,
                    moment: middle * mass,
                },
                1.0,
            );
        }
        match axis.origin {
            Origin::Seed => {}
            Origin::Lateral { parent, node, .. } => match at_node[parent].get_mut(node) {
                Some(at) => at.add(load, 1.0),
                None => at_tip[parent].add(load, 1.0),
            },
            // A relay's load enters at its node as far as it stands there,
            // at the tip as far as it is still the continuation.
            Origin::Relay { parent, node } => {
                let b = axis.blend;
                match at_node[parent].get_mut(node) {
                    Some(at) => {
                        at.add(load, b);
                        at_tip[parent].add(load, 1.0 - b);
                    }
                    None => at_tip[parent].add(load, 1.0),
                }
            }
            Origin::Continuation { parent } => at_tip[parent].add(load, 1.0),
        }
    }
    torques
}

/// The turn, in radians, of a direction `down` radians from straight down
/// under a curvature `curvature` over `length`: the beam's turn, eased so
/// it never passes straight down.
pub(crate) fn turn(curvature: f64, length: f64, down: f64) -> f64 {
    if down <= 0.0 {
        return 0.0;
    }
    down * (1.0 - (-curvature * length / down).exp())
}
