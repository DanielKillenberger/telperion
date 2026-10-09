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
use crate::fixed::feedback::vector::length;
use crate::fixed::feedback::wide::{mul_shr, I256};
use crate::fixed::q32;
use crate::geometry::q::{add, q3, sub};
use crate::geometry::support_q;
use crate::species::Species;
use crate::structure::{Origin, Structure, Vec3};
use crate::upward::Tree;

/// A W vector: three W components.
pub(crate) type W3 = [i128; 3];

/// 2^64, W's one, as f64.
const W_ONE: f64 = 18_446_744_073_709_551_616.0;

/// Load carried, in W (contract A, 2.5): its mass and its first moment
/// (mass times position).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Load {
    pub mass: i128,
    pub moment: W3,
}

impl Load {
    /// A load given in reals (a GPU executor's, a borne bud's), each
    /// component the nearest W.
    pub(crate) fn of(mass: f64, moment: Vec3) -> Self {
        let w = |x: f64| (x * W_ONE).round_ties_even() as i128;
        Self {
            mass: w(mass),
            moment: [w(moment.x), w(moment.y), w(moment.z)],
        }
    }

    /// Its mass and moment as the nearest reals.
    pub(crate) fn real(self) -> (f64, Vec3) {
        let r = |x: i128| x as f64 / W_ONE;
        let m = self.moment;
        (r(self.mass), Vec3::new(r(m[0]), r(m[1]), r(m[2])))
    }

    fn add(&mut self, other: Load) {
        let sum = |a: i128, b: i128| a.checked_add(b).expect("a load inside W");
        self.mass = sum(self.mass, other.mass);
        self.moment = std::array::from_fn(|k| sum(self.moment[k], other.moment[k]));
    }

    /// `b` of it, each component floored (`b` Q32).
    fn share(self, b: i64) -> Load {
        let f = |x: i128| mul_shr(x, i128::from(b), 32).expect("a load inside W");
        Load {
            mass: f(self.mass),
            moment: self.moment.map(f),
        }
    }

    /// What is left of it once `part` is taken, exact.
    fn less(self, part: Load) -> Load {
        Load {
            mass: self.mass - part.mass,
            moment: std::array::from_fn(|k| self.moment[k] - part.moment[k]),
        }
    }
}

/// `p m` of a Q32 point and a W mass, W, floored: a first moment.
fn at(p: [i64; 3], m: i128) -> W3 {
    p.map(|c| mul_shr(i128::from(c), m, 32).expect("a moment inside W"))
}

/// Whether any PA sags: none, and the tree is laid once, as before sag.
pub(crate) fn any(species: &Species) -> bool {
    species.states.iter().any(|s| s.form.sag > 0.0)
}

/// What a phytomer carries on the unbent tree, about its far end.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Lever {
    /// The mass it carries times the offset of that mass's centre from
    /// the end, W. Turned as the phytomer has turned, crossed with
    /// gravity, it is the torque.
    pub moment: W3,
    /// From the phytomer's base to the tip of its axis and the axes that
    /// carry it on, Q32: turned likewise, where the wood beyond would meet
    /// the ground.
    pub chord: [i64; 3],
}

/// Each phytomer's lever about its far end on the unbent tree.
pub(crate) fn levers(structure: &Structure, species: &Species, threads: usize) -> Vec<Vec<Lever>> {
    levers_given(structure, species, &|_| None, threads).0
}

/// `levers`, each axis `given` names standing for a subtree gathered
/// elsewhere whose load at its base it gives; and each axis's load at its
/// base, by index.
pub(crate) fn levers_given(
    structure: &Structure,
    species: &Species,
    given: crate::shed::Given<Load>,
    threads: usize,
) -> (Vec<Vec<Lever>>, Vec<Load>) {
    let axes = &structure.axes;
    let all: Vec<usize> = (0..axes.len()).collect();
    gather(
        axes,
        species,
        structure.age,
        (&all, &trunks(axes)),
        (&[], given),
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
/// member's levers, in the members' order, and the load at its base. An
/// axis outside the members loads nothing; an axis `given` names stands
/// for a subtree gathered elsewhere, whose load at its base it gives
/// (fn-209 F1, realized mechanics).
pub(crate) fn gather(
    axes: &[crate::structure::Axis],
    species: &Species,
    age: u32,
    (members, trunk): (&[usize], &[bool]),
    (borne, given): (&[Borne], crate::shed::Given<Load>),
    threads: usize,
) -> (Vec<Vec<Lever>>, Vec<Load>) {
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
            held[j].push((b.node, Load::of(b.mass, b.moment)));
        }
    }
    // Each axis's far tip: its own, or that of the axis that carries it
    // on. Continuations follow their parents, so a backward pass meets
    // each before its parent.
    let mut far: Vec<[i64; 3]> = members
        .iter()
        .map(|&i| q3(axes[i].phytomers.last().map_or(axes[i].base, |p| p.tip)))
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
    let loads = tree.upward(
        &mut levers,
        threads,
        &|j, levers: &mut Vec<Lever>, loads: &[Load], at_node: &mut Vec<Load>| {
            let i = members[j];
            let axis = &axes[i];
            let state = &species.states[axis.pa];
            at_node.clear();
            at_node.resize(axis.phytomers.len(), Load::default());
            let (mut at_tip, mut at_base) = (Load::default(), Load::default());
            for &(node, load) in &held[j] {
                if let Some(at) = at_node.get_mut(node) {
                    at.add(load);
                }
            }
            for &c in tree.children(j) {
                let (child, load) = (&axes[members[c]], given(members[c]).unwrap_or(loads[c]));
                match child.origin {
                    Origin::Seed => {}
                    Origin::Lateral { node, .. } => match at_node.get_mut(node) {
                        Some(at) => at.add(load),
                        None => at_tip.add(load),
                    },
                    // A relay's load enters where its wood does (`girth`,
                    // fn-244): by its share at the node whose span it
                    // stands in, the rest at the node before (or the base).
                    Origin::Relay { .. } => match crate::girth::attached(axes, species, members[c]) {
                        Some((k, t)) => {
                            let at = load.share(t);
                            at_node[k].add(at);
                            let below = load.less(at);
                            match k.checked_sub(1) {
                                Some(n) => at_node[n].add(below),
                                None => at_base.add(below),
                            }
                        }
                        None => at_tip.add(load),
                    },
                    Origin::Continuation { .. } => at_tip.add(load),
                }
            }
            let mut load = at_tip;
            for k in (0..axis.phytomers.len()).rev() {
                let p = &axis.phytomers[k];
                let base = q3(if k == 0 {
                    axis.base
                } else {
                    axis.phytomers[k - 1].tip
                });
                let tip = q3(p.tip);
                let length = length(sub(tip, base));
                let leaves = q32(crate::girth::own(&state.form, p, age, p.scale, 1.0));
                let r = q32(p.radius);
                // length (r^2 + leaves^2): the squares exact, one floor to W.
                let square =
                    i128::from(r) * i128::from(r) + i128::from(leaves) * i128::from(leaves);
                let mass = I256::mul(square, i128::from(length))
                    .floor_shift(32)
                    .narrow()
                    .expect("a mass inside W");
                let middle = add(base, tip).map(|c| c >> 1);
                // The moment at the phytomer's far end: what its node bears
                // and all beyond it, about that end. An unloaded tip bends
                // by nothing and keeps its tropism.
                load.add(at_node[k]);
                // S - W e: the lever of the load about the end.
                let pull = at(tip, load.mass);
                levers[k] = Lever {
                    moment: std::array::from_fn(|c| load.moment[c] - pull[c]),
                    chord: sub(far[j], base),
                };
                load.add(Load {
                    mass,
                    moment: at(middle, mass),
                });
                // Wood resting on the ground is carried by it: it adds no
                // lever to the wood before it (host, 2026-10-05).
                if !trunk[i] {
                    load = load.share(support_q(base[2].min(tip[2]), r, length));
                }
            }
            load.add(at_base);
            load
        },
    );
    (levers, loads)
}

mod turn;
pub(crate) use turn::{held, log2_q, torque, turn};

#[cfg(test)]
mod tests;
