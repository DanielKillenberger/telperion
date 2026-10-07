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
use crate::transcendental::Transcendental;
mod lay;
mod place;
mod single;
mod stand;

use crate::cores::{fill, map};
use crate::lineage::{Key, DOMINANCE, ROLL};
use crate::species::{PaState, Species};
use crate::structure::{Axis, Origin, Structure, Vec3};
pub(crate) use lay::{ground_zone, support, Layer};
use lay::{rotated, UP};
pub(crate) use place::{place, place_from};
pub(crate) use single::{bud_frame, lay_one};
pub(crate) use stand::relay_span;
use stand::stand;
use std::f64::consts::TAU;

/// The sine of the lean below which a parent has too little of an upper
/// side for epitony to turn its buds by all of it: about 11.5 degrees.
const UPRIGHT: f64 = 0.2;

/// Sizes every phytomer by its axis's vigour, its parent's scale and its
/// unit's own size, on up to `threads` cores: each axis's scale at its
/// base is taken in turn, from its parent's, and the phytomers are scaled
/// by it each on their own. Each relay's node is its draws'
/// (`presence::assign`). Each axis's scale at its base, by index.
pub(crate) fn scale(structure: &mut Structure, species: &Species, threads: usize) -> Vec<f64> {
    let axes = &mut structure.axes;
    let n = axes.len();
    // Each axis's own share of its parent's scale: its vigour, its
    // dominance among its siblings and, for a lateral, its node's scale
    // as drawn.
    let mut own = vec![(1.0, 1.0, 1.0); n];
    {
        let axes = &*axes;
        fill(&mut own, 1 << 12, threads, &|i| {
            let axis = &axes[i];
            match axis.origin {
                Origin::Lateral { parent, node, .. } => (
                    axes[parent].phytomers[node].scale,
                    axis.vigour,
                    dominance(species.states[axis.pa].form.dominance, axis.lineage),
                ),
                _ => (1.0, axis.vigour, 1.0),
            }
        });
    }
    // Each axis's scale at its base, from its parent's: its node's scale
    // once its parent is scaled.
    let mut base_scale = vec![1.0; n];
    for i in 0..n {
        let (drawn, vigour, share) = own[i];
        let inherited = match axes[i].origin {
            Origin::Seed => 1.0,
            Origin::Lateral { parent, .. } => drawn * base_scale[parent],
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => base_scale[parent],
        };
        base_scale[i] = inherited * vigour * share;
    }
    let pieces: Vec<(usize, &mut [Axis])> = axes.chunks_mut(1 << 8).enumerate().collect();
    map(pieces, 1, threads, &|_, (k, piece)| {
        for (j, axis) in piece.iter_mut().enumerate() {
            let base = base_scale[k * (1 << 8) + j];
            axis.phytomers.iter_mut().for_each(|p| p.scale *= base);
        }
    });
    // A unit's size is its own: applied once every axis has inherited its
    // bearer's scale without it.
    let pieces: Vec<&mut [Axis]> = axes.chunks_mut(1 << 8).collect();
    map(pieces, 1, threads, &|_, piece| {
        for phytomer in piece.iter_mut().flat_map(|a| &mut a.phytomers) {
            phytomer.scale *= phytomer.size;
        }
    });
    base_scale
}

/// How far an axis `years` old is pulled towards the vertical at its
/// base: its straightening, and its secondary erection over its years;
/// an axis carried on only as far as it has left the line of the axis it
/// carries on (`Axis::carried_on`), so a continuation's none.
pub(crate) fn bend(axis: &Axis, state: &PaState, years: f64) -> f64 {
    let erected = 1.0 - (-state.erection * years).exp_fixed();
    let (straightening, erected) = match (axis.origin, axis.carried_on()) {
        (_, Some((_, b))) => (state.straightening * b, b * erected),
        (Origin::Lateral { .. }, None) => (state.straightening, erected),
        _ => (0.0, erected),
    };
    1.0 - (1.0 - straightening) * (1.0 - erected)
}

/// The scaled length each axis's straightening fades over: its own, and
/// its relays' as far as each is still the continuation it replaces, so a
/// relay that has barely left the axis's line leaves the axis as it was.
pub(crate) fn reaches(axes: &[Axis]) -> Vec<f64> {
    let mut reach: Vec<f64> = axes
        .iter()
        .map(|a| a.phytomers.iter().map(|p| p.scale).sum())
        .collect();
    // Relays follow their parents, so a backward pass meets each one's
    // reach before its parent's.
    for i in (0..axes.len()).rev() {
        if let Some((parent, b)) = axes[i].carried_on() {
            reach[parent] += (1.0 - b) * reach[i];
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
            let azimuth = phase(axes, species, parent)
                + parent_state.divergence * at.rank
                + TAU * f64::from(slot) / whorl
                + roll;
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
        Origin::Relay { .. } | Origin::Continuation { .. } => {
            // An axis carried on, a continuation at blend 0 (fn-199, host
            // 2026-10-07): between the axis's line at its tip, running and
            // facing as it runs, and its own bud in the axis's span at the
            // PA's `relay_at`, facing where a bud there would, turned up by
            // the PA's epitony: by the presence of the stop that made it.
            let (parent, b) = axis.carried_on().expect("carried on");
            let node = match axis.origin {
                Origin::Relay { node, .. } => node,
                _ => 0,
            };
            let p = &axes[parent];
            let state = &species.states[p.pa];
            let at = stand(axes, species, parent, node);
            let (heading, side) = turned(
                (at.along, at.side),
                at.rank,
                species.states[axis.pa].insertion,
                state.form.plane,
                state.epitony,
            );
            // The axis's running direction and side at its tip, before its
            // pull towards the vertical, which the axis carried on runs on
            // with; its phyllotaxis runs on as a phase (`phase`).
            let (tip, _, _) = end(p);
            let (along, across) = ends(parent);
            let heading = (along * (1.0 - b) + heading * b).unit().unwrap_or(heading);
            let side = across * (1.0 - b) + side * b;
            let side = (side - heading * side.dot(heading))
                .unit()
                .unwrap_or(across);
            (tip + (at.base - tip) * b, heading, side)
        }
    }
}

/// The phase axis `i`'s phyllotaxis starts from: each axis it carries on
/// adds its divergence times its rank, so an axis carried on turns its
/// laterals on from where that axis's would have stood (fn-199, host
/// 2026-10-07), and a lateral or the seed starts afresh.
pub(crate) fn phase(axes: &[Axis], species: &Species, i: usize) -> f64 {
    phase_by(axes, &|pa| species.states[pa].divergence, i)
}

/// `phase`, each PA's divergence given by `divergence`.
pub(crate) fn phase_by(axes: &[Axis], divergence: &dyn Fn(usize) -> f64, i: usize) -> f64 {
    let mut phase = 0.0;
    let mut at = i;
    while let Some((parent, _)) = axes[at].carried_on() {
        let p = &axes[parent];
        phase += divergence(p.pa) * p.rank;
        at = parent;
    }
    phase + axes[at].phase
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
    let mut toward = side * azimuth.cos_fixed() + along.cross(side) * azimuth.sin_fixed();
    // Turned towards the upper side through the smaller angle; the turn
    // fades as the bud faces straight down, where no side is nearer, so
    // no bud flips from one side to the other, and as the parent stands
    // upright, where its upper side is no side at all and its direction
    // would swing with the least lean (fn-206).
    let level = UP - along * UP.dot(along);
    if let Some(upper) = level.unit() {
        let facing = toward.dot(upper);
        let turn = along.dot(toward.cross(upper)).atan2_fixed(facing);
        let sided = (level.length() / UPRIGHT).min(1.0);
        toward = rotated(toward, along, epitony * sided * turn * (1.0 + facing) / 2.0);
    }
    let heading = along * angle.cos_fixed() + toward * angle.sin_fixed();
    let side = toward * angle.cos_fixed() - along * angle.sin_fixed();
    let side = side * plane.cos_fixed() + heading.cross(side) * plane.sin_fixed();
    (heading, side)
}

#[cfg(test)]
mod tests;
