//! Sag (fn-200): an axis bends under the load it carries, as a beam does.
//! Each phytomer carries the wood and foliage beyond it: its own and its
//! axis's further phytomers, and every axis they bear. A phytomer's wood
//! weighs as its volume, its length times its radius squared; its
//! foliage as its own pipe's section times its length (the pipe model: a
//! pipe serves the leaves it bears). Both are sized by presence, so a
//! branch growing in loads its bearer by degree. The load is gathered on
//! the tree as it stands before it bends; the moment is taken when the
//! tree is laid again, base to tip, with the load beyond each phytomer
//! turned rigidly by the bends already made before it, so its lever
//! shrinks as the branch droops and a heavy branch hangs and stops (fn-203,
//! the first-order large-deflection correction). Growth never reads it
//! back.
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

/// Each phytomer's lever about its far end on the unbent tree: the mass
/// it carries times the offset of that mass's centre from the end. Turned
/// as the phytomer has turned, crossed with gravity, it is the torque.
pub(crate) fn levers(structure: &Structure, species: &Species) -> Vec<Vec<Vec3>> {
    let age = structure.age;
    let axes = &structure.axes;
    let mut at_node: Vec<Vec<Load>> = axes
        .iter()
        .map(|a| vec![Load::default(); a.phytomers.len()])
        .collect();
    let mut at_tip = vec![Load::default(); axes.len()];
    let mut levers: Vec<Vec<Vec3>> = axes
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
            // S - W e: the lever of the load about the end.
            levers[i][k] = load.moment - p.tip * load.mass;
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
    levers
}

/// The torque gravity puts on a `lever`: (S - W e) x (-z), horizontal,
/// the axis the wood turns down about, its length the moment.
pub(crate) fn torque(lever: Vec3) -> Vec3 {
    Vec3::new(-lever.y, lever.x, 0.0)
}

/// The turn, in radians, of a phytomer `length` metres long and `radius`
/// thick under `lever` (turned into its frame now) and its PA's `sag`.
/// The beam's curvature is sag times the moment over the radius to the
/// fourth, and the moment is the lever's reach times the sine of its fall,
/// the angle from hanging straight below the phytomer's end; carried
/// rigidly, the load's fall closes as dfall/ds = -k sin(fall), whose exact
/// solution over the phytomer is tan(fall / 2) e^(-k length). A light load
/// turns as the small-deflection beam does; a heavy one turns at most
/// until it hangs, and never past.
pub(crate) fn turn(lever: Vec3, sag: f64, radius: f64, length: f64) -> f64 {
    let reach = lever.length();
    if reach <= 0.0 {
        return 0.0;
    }
    let fall = (-lever.z / reach).clamp(-1.0, 1.0).acos();
    let k = sag * reach / radius.powi(4) * length;
    fall - 2.0 * ((fall / 2.0).tan() * (-k).exp()).atan()
}
