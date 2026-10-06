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
use crate::geometry::support;
use crate::girth::ripe;
use crate::species::Species;
use crate::structure::{Origin, Structure, Vec3};
use crate::upward::Tree;
use std::f64::consts::PI;

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

/// What a phytomer carries on the unbent tree, about its far end.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Lever {
    /// The mass it carries times the offset of that mass's centre from
    /// the end. Turned as the phytomer has turned, crossed with gravity,
    /// it is the torque.
    pub moment: Vec3,
    /// From the phytomer's base to the tip of its axis and the axes that
    /// carry it on: turned likewise, where the wood beyond would meet the
    /// ground.
    pub chord: Vec3,
}

/// Each phytomer's lever about its far end on the unbent tree.
pub(crate) fn levers(structure: &Structure, species: &Species, threads: usize) -> Vec<Vec<Lever>> {
    let axes = &structure.axes;
    let all: Vec<usize> = (0..axes.len()).collect();
    gather(
        axes,
        species,
        structure.age,
        (&all, &trunks(axes)),
        &[],
        threads,
    )
}

/// Whether each axis is the trunk: the seed axis and what carries it on,
/// which the ground never carries.
pub(crate) fn trunks(axes: &[crate::structure::Axis]) -> Vec<bool> {
    let mut trunk = vec![false; axes.len()];
    for (i, axis) in axes.iter().enumerate() {
        trunk[i] = match axis.origin {
            Origin::Seed => true,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => trunk[parent],
            Origin::Lateral { .. } => false,
        };
    }
    trunk
}

/// A load from outside the axes gathered, standing at a node of one of
/// them (fn-209, option 4: a borne bud's committed mass and moment).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Borne {
    pub axis: usize,
    pub node: usize,
    pub mass: f64,
    pub moment: Vec3,
}

/// The levers of `members` (ascending; each axis's whole tree, or one
/// chain), from their own wood, what members bear, and `borne`: each
/// member's levers, in the members' order. An axis outside the members
/// loads nothing.
pub(crate) fn gather(
    axes: &[crate::structure::Axis],
    species: &Species,
    age: u32,
    (members, trunk): (&[usize], &[bool]),
    borne: &[Borne],
    threads: usize,
) -> Vec<Vec<Lever>> {
    let whole = members.len() == axes.len();
    let local = |i: usize| {
        if whole {
            Some(i)
        } else {
            members.binary_search(&i).ok()
        }
    };
    // The loads borne at each member's nodes, in their order.
    let mut held: Vec<Vec<(usize, Load)>> = vec![Vec::new(); members.len()];
    for b in borne {
        if let Some(j) = local(b.axis) {
            let load = Load {
                mass: b.mass,
                moment: b.moment,
            };
            held[j].push((b.node, load));
        }
    }
    // Each axis's far tip: its own, or that of the axis that carries it
    // on. Continuations follow their parents, so a backward pass meets
    // each before its parent.
    let mut far: Vec<Vec3> = members
        .iter()
        .map(|&i| axes[i].phytomers.last().map_or(axes[i].base, |p| p.tip))
        .collect();
    for j in (0..members.len()).rev() {
        if let Origin::Continuation { parent } = axes[members[j]].origin {
            if let Some(q) = local(parent) {
                far[q] = far[j];
            }
        }
    }
    let mut levers: Vec<Vec<Lever>> = members
        .iter()
        .map(|&i| vec![Lever::default(); axes[i].phytomers.len()])
        .collect();
    let parents: Vec<Option<usize>> = members
        .iter()
        .map(|&i| axes[i].origin.parent().and_then(local))
        .collect();
    let tree = Tree::new(&parents);
    // Each member takes what it bears at its nodes, then what its
    // children carry, the last first, as a backward pass meets them, and
    // hands its parent the load at its base.
    tree.upward(
        &mut levers,
        threads,
        &|j, levers: &mut Vec<Lever>, loads: &[Load]| {
            let i = members[j];
            let axis = &axes[i];
            let state = &species.states[axis.pa];
            let mut at_node = vec![Load::default(); axis.phytomers.len()];
            let mut at_tip = Load::default();
            for &(node, load) in &held[j] {
                if let Some(at) = at_node.get_mut(node) {
                    at.add(load, 1.0);
                }
            }
            for &c in tree.children(j) {
                let (child, load) = (&axes[members[c]], loads[c]);
                match child.origin {
                    Origin::Seed => {}
                    Origin::Lateral { node, .. } => match at_node.get_mut(node) {
                        Some(at) => at.add(load, 1.0),
                        None => at_tip.add(load, 1.0),
                    },
                    // A relay's load enters at its node as far as it stands
                    // there, at the tip as far as it is still the continuation.
                    Origin::Relay { node, .. } => {
                        let b = child.blend;
                        match at_node.get_mut(node) {
                            Some(at) => {
                                at.add(load, b);
                                at_tip.add(load, 1.0 - b);
                            }
                            None => at_tip.add(load, 1.0),
                        }
                    }
                    Origin::Continuation { .. } => at_tip.add(load, 1.0),
                }
            }
            let mut load = at_tip;
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
                load.add(at_node[k], 1.0);
                // S - W e: the lever of the load about the end.
                levers[k] = Lever {
                    moment: load.moment - p.tip * load.mass,
                    chord: far[j] - base,
                };
                load.add(
                    Load {
                        mass,
                        moment: middle * mass,
                    },
                    1.0,
                );
                // Wood resting on the ground is carried by it: it adds no
                // lever to the wood before it (host, 2026-10-05).
                if !trunk[i] {
                    let held = support(base.z.min(p.tip.z), p.radius, length);
                    load = Load {
                        mass: load.mass * held,
                        moment: load.moment * held,
                    };
                }
            }
            load
        },
    );
    levers
}

/// The torque gravity puts on a `lever`: (S - W e) x (-z), horizontal,
/// the axis the wood turns down about, its length the moment.
pub(crate) fn torque(lever: Vec3) -> Vec3 {
    Vec3::new(-lever.y, lever.x, 0.0)
}

/// The share of a lever the ground leaves to the wood before it: where
/// the wood from a phytomer's base `height` above the ground, turned
/// rigidly to its pose now along `chord`, would go below it, the ground
/// carries that share, and the rest, nearer, bears with a lever as much
/// shorter (host, 2026-10-05). Measured from the base, so it falls by
/// degree as the phytomer's own end reaches the ground.
pub(crate) fn held(height: f64, chord: Vec3) -> f64 {
    let low = height + chord.z;
    if low >= 0.0 {
        return 1.0;
    }
    if height <= 0.0 {
        return 0.0;
    }
    let free = height / (height - low);
    free * free
}

/// The turn, in radians, of a phytomer `length` metres long and `radius`
/// thick under `lever` (turned into its frame now) and its PA's `sag`,
/// its moment never more than `cap`, the moment its unbent lever puts on
/// it (host, 2026-10-05: the correction only ever shrinks the lever).
/// The beam's curvature is sag times the moment over the radius to the
/// fourth. The load, carried rigidly, falls `fall` from hanging straight
/// below the phytomer's end, and its moment is its reach times sin(fall),
/// so dfall/ds = -c min(reach sin(fall), cap). Where the cap holds, the
/// fall closes at a constant rate; elsewhere tan(fall / 2) shrinks by
/// e^(-c reach s). Integrated exactly over the phytomer: a light load
/// turns as the small-deflection beam does, a heavy one turns until it
/// hangs and never past, and upright wood never buckles.
pub(crate) fn turn(lever: Vec3, cap: f64, sag: f64, radius: f64, length: f64) -> f64 {
    let reach = lever.length();
    if reach <= 0.0 || cap <= 0.0 {
        return 0.0;
    }
    let c = sag / radius.powi(4);
    let start = (-lever.z / reach).clamp(-1.0, 1.0).acos();
    let low = (cap / reach).min(1.0).asin();
    let high = PI - low;
    let settle = |fall: f64, s: f64| 2.0 * ((fall / 2.0).tan() * (-c * reach * s).exp()).atan();
    let (mut fall, mut left) = (start, length);
    // Near upright the turned lever is the smaller, until its moment
    // reaches the cap.
    if fall > high {
        let s = ((fall / 2.0).tan() / (high / 2.0).tan()).ln() / (c * reach);
        if s >= left {
            return start - settle(fall, left);
        }
        (fall, left) = (high, left - s);
    }
    // Where the cap holds, the moment is the unbent one.
    if fall > low {
        let rate = c * cap;
        let s = (fall - low) / rate;
        if s >= left {
            return start - (fall - rate * left);
        }
        (fall, left) = (low, left - s);
    }
    start - settle(fall, left)
}
