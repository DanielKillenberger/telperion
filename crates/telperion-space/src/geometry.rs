//! Geometry: axes of internodes of their PA's length times their scale,
//! each lateral turned from its parent's direction at its node by its PA's
//! insertion angle, around the parent at the azimuth its node's
//! phyllotactic rank and its slot in the whorl give, in a plane turned by
//! the parent's `plane`. Along an axis the direction bends towards its
//! PA's elevation (tropism) and wanders by keyed turns; a lateral's base
//! straightens towards the vertical by its PA's straightening, and each
//! phytomer turns down by its sag under the load it carries (`sag.rs`).
//! The seed stands at the origin, growing up (+z). Wood that reaches the
//! ground rests on it and runs along it; the trunk reaching the ground,
//! or an axis whose base is below it, is an error.
mod lay;
mod place;

use crate::lineage::{Key, DOMINANCE, ROLL};
use crate::species::{PaState, Species};
use crate::structure::{Axis, Origin, Structure, Vec3};
use lay::{rotated, UP};
pub(crate) use lay::{support, Layer};
pub(crate) use place::place;
use std::f64::consts::TAU;

/// The sine of the lean below which a parent has too little of an upper
/// side for epitony to turn its buds by all of it: about 11.5 degrees.
const UPRIGHT: f64 = 0.2;

/// Sizes every phytomer by its axis's vigour, its parent's scale and its
/// unit's own size, and sets each relay's node.
pub(crate) fn scale(structure: &mut Structure, species: &Species) {
    // Each axis's scale at its base, from its already scaled parent.
    let mut base_scale = vec![1.0; structure.axes.len()];
    for i in 0..structure.axes.len() {
        let inherited = match structure.axes[i].origin {
            Origin::Seed => 1.0,
            Origin::Lateral { parent, node, .. } => structure.axes[parent].phytomers[node].scale,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => base_scale[parent],
        };
        let axis = &mut structure.axes[i];
        let share = match axis.origin {
            Origin::Lateral { .. } => {
                dominance(species.states[axis.pa].form.dominance, axis.lineage)
            }
            _ => 1.0,
        };
        base_scale[i] = inherited * axis.vigour * share;
        for phytomer in &mut axis.phytomers {
            phytomer.scale *= base_scale[i];
        }
        // A relay's node: where along its parent's last growth unit it
        // stands, which girth and placement both read.
        if let Origin::Relay { parent, .. } = structure.axes[i].origin {
            let share = species.states[structure.axes[parent].pa].relay_at;
            let node = relay_point(&structure.axes[parent], share).node;
            structure.axes[i].origin = Origin::Relay { parent, node };
        }
    }
    // A unit's size is its own: applied once every axis has inherited its
    // bearer's scale without it.
    for phytomer in structure.axes.iter_mut().flat_map(|a| &mut a.phytomers) {
        phytomer.scale *= phytomer.size;
    }
}

/// How far an axis `years` old is pulled towards the vertical at its
/// base: its straightening, and its secondary erection over its years;
/// never a continuation's, whose base is its parent's tip, and a relay's
/// as far as it has left the continuation it replaces (`blend`).
pub(crate) fn bend(axis: &Axis, state: &PaState, years: f64) -> f64 {
    let straightening = match axis.origin {
        Origin::Lateral { .. } => state.straightening,
        Origin::Relay { .. } => state.straightening * axis.blend,
        Origin::Seed | Origin::Continuation { .. } => 0.0,
    };
    let erected = match axis.origin {
        Origin::Continuation { .. } => 0.0,
        Origin::Relay { .. } => axis.blend * (1.0 - (-state.erection * years).exp()),
        _ => 1.0 - (-state.erection * years).exp(),
    };
    1.0 - (1.0 - straightening) * (1.0 - erected)
}

/// The scaled length each axis's straightening fades over: its own, and
/// its relays' as far as each is still the continuation it replaces, so a
/// relay that has barely left the axis's line leaves the axis as it was.
fn reaches(axes: &[Axis]) -> Vec<f64> {
    let mut reach: Vec<f64> = axes
        .iter()
        .map(|a| a.phytomers.iter().map(|p| p.scale).sum())
        .collect();
    // Relays follow their parents, so a backward pass meets each one's
    // reach before its parent's.
    for i in (0..axes.len()).rev() {
        if let Origin::Relay { parent, .. } = axes[i].origin {
            reach[parent] += (1.0 - axes[i].blend) * reach[i];
        }
    }
    reach
}

/// A lateral's share of its vigour among its siblings: by `dominance`,
/// from all of it (0) towards a keyed share that few laterals hold whole,
/// the fourth power of a draw its lineage keys.
pub(crate) fn dominance(dominance: f64, lineage: u64) -> f64 {
    if dominance <= 0.0 {
        return 1.0;
    }
    let held = Key(lineage).child(DOMINANCE).unit().powi(4);
    1.0 - dominance * (1.0 - held)
}

/// Where axis `p` ends: its last node and the frame there, or its base.
fn end(p: &Axis) -> (Vec3, Vec3, Vec3) {
    p.phytomers
        .last()
        .map_or((p.base, p.heading, p.side), |last| {
            (last.tip, last.heading, last.side)
        })
}

/// The base, heading and side of axis `i`, from its already placed parent
/// and the running direction and side its parent's walk ended on (`ends`).
pub(crate) fn frame(
    axes: &[Axis],
    species: &Species,
    i: usize,
    ends: &dyn Fn(usize) -> (Vec3, Vec3),
) -> (Vec3, Vec3, Vec3) {
    let axis = &axes[i];
    match axis.origin {
        Origin::Seed => (Vec3::default(), UP, Vec3::new(1.0, 0.0, 0.0)),
        Origin::Continuation { parent } => {
            let p = &axes[parent];
            // On from the parent's tip in the running direction and side it
            // ends on, where its pull towards the vertical has faded to
            // none, so a last unit all but gone turns nothing (fn-206).
            let (base, _, _) = end(p);
            let (heading, side) = ends(parent);
            // The phyllotaxis runs on across the change of PA.
            let turn = species.states[p.pa].divergence * p.rank;
            let side = side * turn.cos() + heading.cross(side) * turn.sin();
            (base, heading, side)
        }
        Origin::Lateral {
            parent,
            node,
            slot,
            whorl,
            woken,
        } => {
            let p = &axes[parent];
            let at = &p.phytomers[node];
            let parent_state = &species.states[p.pa];
            // Turned about its parent by its PA's roll, as its lineage keys.
            let roll = species.states[axis.pa].form.roll
                * (2.0 * Key(axis.lineage).child(ROLL).unit() - 1.0);
            let azimuth = parent_state.divergence * at.rank + TAU * f64::from(slot) / whorl + roll;
            let (heading, side) = turned(
                (at.heading, at.side),
                azimuth,
                species.states[axis.pa].insertion,
                parent_state.form.plane,
                0.0,
            );
            // A bud that slept stands on its slot's side half an internode
            // below the node, apart from the bud that grew at once.
            let base = if woken {
                let below = node.checked_sub(1).map_or(p.base, |n| p.phytomers[n].tip);
                (below + at.tip) * 0.5
            } else {
                at.tip
            };
            (base, heading, side)
        }
        Origin::Relay { parent, .. } => {
            // Between the continuation it replaces, at the tip on the
            // axis's line, and its own bud in the axis's span at the PA's
            // `relay_at`, facing where a bud there would, turned up by the
            // PA's epitony: by the presence of the stop that made it.
            let p = &axes[parent];
            let state = &species.states[p.pa];
            let at = relay_point(p, state.relay_at);
            let (heading, side) = turned(
                (at.along, at.side),
                state.divergence * at.rank,
                species.states[axis.pa].insertion,
                state.form.plane,
                state.epitony,
            );
            // The axis's running direction and side at its tip, before its
            // pull towards the vertical, which the relay carries on.
            let (tip, _, _) = end(p);
            let (along, across) = ends(parent);
            let turn = state.divergence * p.rank;
            let carried = across * turn.cos() + along.cross(across) * turn.sin();
            let b = axis.blend;
            let heading = (along * (1.0 - b) + heading * b).unit().unwrap_or(heading);
            let side = carried * (1.0 - b) + side * b;
            let side = (side - heading * side.dot(heading))
                .unit()
                .unwrap_or(carried);
            (tip + (at.base - tip) * b, heading, side)
        }
    }
}

/// A point along an axis and the frame there.
struct Point {
    /// The phytomer whose span holds it.
    node: usize,
    base: Vec3,
    along: Vec3,
    side: Vec3,
    rank: f64,
}

/// The point `share` of the way along axis `p`'s last growth unit, by its
/// phytomers' scaled lengths. The frame turns from one phytomer's direction to the next's along
/// each span, so the point's frame moves by degree as `share` does; at 1
/// it is the last node and the frame of its phytomer, at 0 the base of the
/// unit's first internode.
fn relay_point(p: &Axis, share: f64) -> Point {
    let n = p.phytomers.len();
    if n == 0 {
        return Point {
            node: 0,
            base: p.base,
            along: p.heading,
            side: p.side,
            rank: p.rank,
        };
    }
    // The last growth unit's nodes: the module's curvature zone, wherever
    // relays of it before have split the axis.
    let last = p.phytomers[n - 1].cycle;
    let first = p
        .phytomers
        .iter()
        .position(|q| q.cycle == last)
        .unwrap_or(0);
    // `share` of the unit's length, its phytomers by their scale, so a node
    // growing in from nothing moves the point by nothing (fn-206).
    let total: f64 = p.phytomers[first..].iter().map(|q| q.scale).sum();
    let (k, t) = if total > 0.0 {
        let target = share * total;
        let mut before = 0.0;
        let mut found = (n - 1, 1.0);
        for (j, q) in p.phytomers.iter().enumerate().skip(first) {
            if q.scale > 0.0 && before + q.scale >= target {
                found = (j, ((target - before) / q.scale).clamp(0.0, 1.0));
                break;
            }
            before += q.scale;
        }
        found
    } else {
        let f = first as f64 + share * (n - first) as f64;
        let k = (f.ceil() as usize).clamp(first + 1, n) - 1;
        (k, (f - k as f64).clamp(0.0, 1.0))
    };
    let at = &p.phytomers[k];
    let start = if k == 0 {
        p.base
    } else {
        p.phytomers[k - 1].tip
    };
    let next = p.phytomers.get(k + 1);
    let blend = |a: Vec3, b: Vec3| (a * (1.0 - t) + b * t).unit().unwrap_or(a);
    Point {
        node: k,
        base: start + (at.tip - start) * t,
        along: blend(at.heading, next.map_or(at.heading, |q| q.heading)),
        side: blend(at.side, next.map_or(at.side, |q| q.side)),
        rank: at.rank + t * (next.map_or(p.rank, |q| q.rank) - at.rank),
    }
}

/// The heading and side of a bud at `azimuth` around a parent going
/// `along` with `side`, turned `epitony` of the way round to the parent's
/// upper side, inserted at `angle`, its plane turned by `plane`.
fn turned(
    (along, side): (Vec3, Vec3),
    azimuth: f64,
    angle: f64,
    plane: f64,
    epitony: f64,
) -> (Vec3, Vec3) {
    let mut toward = side * azimuth.cos() + along.cross(side) * azimuth.sin();
    // Turned towards the upper side through the smaller angle; the turn
    // fades as the bud faces straight down, where no side is nearer, so
    // no bud flips from one side to the other, and as the parent stands
    // upright, where its upper side is no side at all and its direction
    // would swing with the least lean (fn-206).
    let level = UP - along * UP.dot(along);
    if let Some(upper) = level.unit() {
        let facing = toward.dot(upper);
        let turn = along.dot(toward.cross(upper)).atan2(facing);
        let sided = (level.length() / UPRIGHT).min(1.0);
        toward = rotated(toward, along, epitony * sided * turn * (1.0 + facing) / 2.0);
    }
    let heading = along * angle.cos() + toward * angle.sin();
    let side = toward * angle.cos() - along * angle.sin();
    let side = side * plane.cos() + heading.cross(side) * plane.sin();
    (heading, side)
}

#[cfg(test)]
mod tests;
