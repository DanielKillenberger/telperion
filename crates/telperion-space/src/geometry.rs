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
pub(crate) mod q;
mod sagged;
mod stand;

use crate::cores::{fill, map};
use crate::lineage::{insertion, Key, DOMINANCE, ROLL};
use crate::species::{PaState, Species};
use crate::structure::{Axis, Origin, Structure, Vec3};
pub(crate) use lay::{cone, support, Layer};
pub(crate) use place::place;
pub(crate) use q::amount;
use q::{add, atan2, between, cross, dot, q3, rotated, scaled, sincos, sub, unit, v3, Q3, UP};
use stand::stand;
pub(crate) use stand::{split, Position};

/// The sine of the lean below which a parent has too little of an upper
/// side for epitony to turn its buds by all of it: about 11.5 degrees.
const UPRIGHT: f32 = 0.2;

/// A rate per rank (radians) times a rank, as an amount in turns, whole
/// turns kept.
fn by_rank(rate: f64, rank: f32) -> f32 {
    amount(rate) * rank
}

/// `side` turned about `heading` by the amount `turn`.
fn spun(heading: Q3, side: Q3, turn: f32) -> Q3 {
    let (cos, sin) = sincos(turn);
    add(scaled(side, cos), scaled(cross(heading, side), sin))
}

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
    // once its parent is scaled. Parents first, in f32, in this order.
    let mut base_scale = vec![1.0f32; n];
    for i in 0..n {
        let (drawn, vigour, share) = own[i];
        let inherited = match axes[i].origin {
            Origin::Seed => 1.0,
            Origin::Lateral { parent, .. } => drawn as f32 * base_scale[parent],
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => base_scale[parent],
        };
        base_scale[i] = inherited * vigour as f32 * share as f32;
    }
    let pieces: Vec<(usize, &mut [Axis])> = axes.chunks_mut(1 << 8).enumerate().collect();
    map(pieces, 1, threads, &|_, (k, piece)| {
        for (j, axis) in piece.iter_mut().enumerate() {
            let base = base_scale[k * (1 << 8) + j];
            axis.phytomers
                .iter_mut()
                .for_each(|p| p.scale = f64::from(p.scale as f32 * base));
        }
    });
    // A unit's size is its own: applied once every axis has inherited its
    // bearer's scale without it.
    let pieces: Vec<&mut [Axis]> = axes.chunks_mut(1 << 8).collect();
    map(pieces, 1, threads, &|_, piece| {
        for phytomer in piece.iter_mut().flat_map(|a| &mut a.phytomers) {
            phytomer.scale = f64::from(phytomer.scale as f32 * phytomer.size as f32);
        }
    });
    base_scale.into_iter().map(f64::from).collect()
}

/// How far an axis `years` old is pulled towards the vertical at its
/// base: its straightening, and its secondary erection over its years;
/// an axis carried on only as far as it has left the line of the axis it
/// carries on (`Axis::carried_on`), so a continuation's none.
pub(crate) fn bend(axis: &Axis, state: &PaState, years: f64) -> f64 {
    // 1 - e^(-erection years): the contract's `fails` of its log2.
    let x = state.erection as f32 * years as f32;
    let grown = crate::contract::fails(x * std::f32::consts::LOG2_E);
    let (straightening, erected) = match (axis.origin, axis.carried_on()) {
        (_, Some((_, b))) => {
            let b = b as f32;
            (state.straightening as f32 * b, b * grown)
        }
        (Origin::Lateral { .. }, None) => (state.straightening as f32, grown),
        _ => (0.0, grown),
    };
    f64::from(1.0 - (1.0 - straightening) * (1.0 - erected))
}

/// The scaled length each axis's straightening fades over: its own, and
/// its relays' as far as each is still the continuation it replaces, so a
/// relay that has barely left the axis's line leaves the axis as it was.
pub(crate) fn reaches(axes: &[Axis]) -> Vec<f64> {
    let mut reach: Vec<f32> = axes
        .iter()
        .map(|a| a.phytomers.iter().map(|p| p.scale as f32).sum())
        .collect();
    // Relays follow their parents, so a backward pass meets each one's
    // reach before its parent's.
    for i in (0..axes.len()).rev() {
        if let Some((parent, b)) = axes[i].carried_on() {
            reach[parent] += (1.0 - b as f32) * reach[i];
        }
    }
    reach.into_iter().map(f64::from).collect()
}

/// A lateral's share of its vigour among its siblings: by `dominance`,
/// from all of it (0) towards a keyed share that few laterals hold whole,
/// the fourth power of a draw its lineage keys.
pub(crate) fn dominance(dominance: f64, lineage: u64) -> f64 {
    if dominance <= 0.0 {
        return 1.0;
    }
    let u = crate::lineage::unit(Key(lineage).child(DOMINANCE).draw());
    let held = u * u * u * u;
    f64::from(1.0 - dominance as f32 * (1.0 - held))
}

/// Where axis `p` ends: its last node and the frame there, or its base.
fn end(p: &Axis) -> (Q3, Q3, Q3) {
    let (base, heading, side) = p
        .phytomers
        .last()
        .map_or((p.base, p.heading, p.side), |last| {
            (last.tip, last.heading, last.side)
        });
    (q3(base), q3(heading), q3(side))
}

/// The base, heading and side of axis `i`, from its already placed parent
/// and the running direction and side its parent's walk ended on (`ends`).
/// In f32, angles as amounts in turns.
pub(crate) fn frame(
    axes: &[Axis],
    species: &Species,
    i: usize,
    ends: &dyn Fn(usize) -> (Vec3, Vec3),
) -> (Vec3, Vec3, Vec3) {
    let (base, heading, side) = framed(axes, species, i, ends);
    (v3(base), v3(heading), v3(side))
}

fn framed(
    axes: &[Axis],
    species: &Species,
    i: usize,
    ends: &dyn Fn(usize) -> (Vec3, Vec3),
) -> (Q3, Q3, Q3) {
    let ends = |p: usize| {
        let (heading, side) = ends(p);
        (q3(heading), q3(side))
    };
    let axis = &axes[i];
    match axis.origin {
        Origin::Seed => ([0.0; 3], UP, [1.0, 0.0, 0.0]),
        Origin::Lateral { .. } => lateral(axes, species, (axis.origin, axis.pa, axis.lineage)),
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
            let (tip, _, _) = end(&axes[parent]);
            let (along, across) = ends(parent);
            let b = b as f32;
            if b == 0.0 {
                // On the line alone: the general form below at blend 0, to
                // the bit, without a stand it scales by none.
                let heading = unit(along).unwrap_or(along);
                let side = unit(sub(across, scaled(heading, dot(across, heading))));
                return (tip, heading, side.unwrap_or(across));
            }
            let state = &species.states[axes[parent].pa];
            let at = stand(axes, species, parent, node);
            let (heading, side) = turned(
                (at.along, at.side),
                at.rank,
                angle(species, axis),
                amount(state.form.plane),
                state.epitony as f32,
            );
            // The axis's running direction and side at its tip, before its
            // pull towards the vertical, which the axis carried on runs on
            // with; its phyllotaxis runs on as a phase (`phase`).
            let heading = unit(add(scaled(along, 1.0 - b), scaled(heading, b))).unwrap_or(heading);
            let side = add(scaled(across, 1.0 - b), scaled(side, b));
            let side = unit(sub(side, scaled(heading, dot(side, heading)))).unwrap_or(across);
            // Its base where it joins the parent, on the parent's wood.
            let x = attach(axes, species, i).expect("a relay's attachment");
            let base = stand::at(&axes[parent], x);
            (base, heading, side)
        }
    }
}

/// How far relay `i` runs back onto the line it carries on (fn-286): its
/// PA's take-over as far as it has left that line (its blend), towards
/// its parent's running direction where the parent's walk ended (`ends`).
/// None on any other axis, or a line of no direction.
pub(crate) fn taken(
    axes: &[Axis],
    species: &Species,
    i: usize,
    ends: &dyn Fn(usize) -> (Vec3, Vec3),
) -> Take {
    let axis = &axes[i];
    let Origin::Relay { parent, .. } = axis.origin else {
        return NO_TAKE;
    };
    let take = species.states[axis.pa].takeover as f32 * axis.blend as f32;
    match unit(q3(ends(parent).0)) {
        Some(line) if take > 0.0 => (line, take),
        _ => NO_TAKE,
    }
}

/// A take-over: the line a relay runs back onto and how far (`taken`).
pub(crate) type Take = ([f32; 3], f32);
/// No take-over.
pub(crate) const NO_TAKE: Take = ([0.0; 3], 0.0);

/// The take-over an axis carried on runs with: its own as a relay
/// (`taken`) and what it carries on of its parent's (`Layer::handed_take`),
/// their lines weighted by how far each pulls. Both fade over the axis's
/// own reach, so a continuation goes on running back onto the line its
/// relay took over until that relay's reach is spent (fn-286 review).
pub(crate) fn joined((mine, own): Take, (line, carried): Take) -> Take {
    match (own > 0.0, carried > 0.0) {
        (true, true) => {
            let s = own + carried;
            let at = |c: usize| (mine[c] * own + line[c] * carried) / s;
            ([at(0), at(1), at(2)], s)
        }
        (true, false) => (mine, own),
        (false, true) => (line, carried),
        (false, false) => NO_TAKE,
    }
}

/// Where relay `i` joins its parent (fn-244 host decision 4): its stand's
/// place on the parent (`stand::position`) blended towards the parent's
/// tip by its blend along the parent's scaled length, as a position in the
/// parent's nodes (`stand::split`). The one place a relay joins: its base,
/// girth, sag and the drawing read it. None for any other axis.
pub(crate) fn attach(axes: &[Axis], species: &Species, i: usize) -> Option<Position> {
    let Origin::Relay { parent, node } = axes[i].origin else {
        return None;
    };
    let b = axes[i].blend as f32;
    let p = &axes[parent];
    if b == 0.0 {
        return Some((p.phytomers.len(), 0.0));
    }
    // Blended by scaled length, as the stand is (`stand::position`).
    let tip = stand::total(p);
    let at = stand::position(axes, species, parent, node);
    Some(stand::node_x(p, tip + (at - tip) * b))
}

/// A lateral bud's base, heading and side on its laid bearer: `frame`'s
/// lateral, and `bud_frame`'s for a bud record.
pub(super) fn lateral(
    axes: &[Axis],
    species: &Species,
    (origin, pa, lineage): (Origin, usize, u64),
) -> (Q3, Q3, Q3) {
    let Origin::Lateral {
        parent,
        node,
        slot,
        whorl,
        woken,
    } = origin
    else {
        unreachable!("a lateral")
    };
    let p = &axes[parent];
    let at = &p.phytomers[node];
    let parent_state = &species.states[p.pa];
    // Turned about its parent by its PA's roll, as its lineage keys.
    let signed = crate::lineage::signed(Key(lineage).child(ROLL).draw());
    let roll = amount(species.states[pa].form.roll) * signed;
    let slot = f32::from(slot) / whorl as f32;
    let azimuth = phase(axes, species, parent)
        + by_rank(parent_state.divergence, at.rank as f32)
        + slot
        + roll;
    let state = &species.states[pa];
    let (heading, side) = turned(
        (q3(at.heading), q3(at.side)),
        azimuth,
        insertion(lineage, state.insertion, state.form.insertion_spread),
        amount(parent_state.form.plane),
        0.0,
    );
    // A bud that slept stands on its slot's side half an internode below
    // the node, apart from the bud that grew at once.
    let tip = q3(at.tip);
    let base = if woken {
        let below = node.checked_sub(1).map_or(p.base, |n| p.phytomers[n].tip);
        add(q3(below), tip).map(|c| c * 0.5)
    } else {
        tip
    };
    (base, heading, side)
}

/// Axis `axis`'s insertion angle, an amount: its PA's, offset by its
/// lineage's draw at its PA's spread (fn-224).
fn angle(species: &Species, axis: &Axis) -> f32 {
    let state = &species.states[axis.pa];
    insertion(axis.lineage, state.insertion, state.form.insertion_spread)
}

/// The phase, an amount in turns, axis `i`'s phyllotaxis starts from:
/// each axis it carries on adds its divergence times its rank, so an axis
/// carried on turns its laterals on from where that axis's would have
/// stood (fn-199, host 2026-10-07), as far as it is still its
/// continuation; a lateral or the seed starts afresh.
pub(crate) fn phase(axes: &[Axis], species: &Species, i: usize) -> f32 {
    phase_by(axes, &|pa| species.states[pa].divergence, i)
}

/// `phase`, each PA's divergence (radians per rank) given by `divergence`.
pub(crate) fn phase_by(axes: &[Axis], divergence: &dyn Fn(usize) -> f64, i: usize) -> f32 {
    let mut line = Vec::new();
    let mut at = i;
    while let Some((parent, b)) = axes[at].carried_on() {
        line.push((parent, b));
        at = parent;
    }
    // Each axis carried on runs its parent's phyllotaxis on as far as it
    // is still that axis's continuation (`1 - blend`): a relay become its
    // own bud starts afresh, as a lateral does (fn-199).
    let mut phase = axes[at].phase;
    for &(parent, b) in line.iter().rev() {
        let p = &axes[parent];
        phase = (1.0 - b as f32) * (phase + by_rank(divergence(p.pa), p.rank as f32));
    }
    phase
}

/// The heading and side of a bud at `azimuth` around a parent going
/// `along` with `side`, turned `epitony` of the way round to the parent's
/// upper side, inserted at `angle`, its plane turned by `plane` (amounts).
fn turned((along, side): (Q3, Q3), azimuth: f32, angle: f32, plane: f32, epitony: f32) -> (Q3, Q3) {
    let mut toward = spun(along, side, azimuth);
    // Turned towards the upper side through the smaller angle; the turn
    // fades as the bud faces straight down, where no side is nearer, so
    // no bud flips from one side to the other, and as the parent stands
    // upright, where its upper side is no side at all and its direction
    // would swing with the least lean (fn-206).
    let level = sub(UP, scaled(along, dot(UP, along)));
    if let Some(upper) = unit(level) {
        let facing = dot(toward, upper);
        let turn = between(0.0, atan2(dot(along, cross(toward, upper)), facing));
        let sided = (q::length(level) / UPRIGHT).min(1.0);
        let epi = epitony * sided * turn * (1.0 + facing) * 0.5;
        toward = rotated(toward, along, epi);
    }
    let (cos, sin) = sincos(angle);
    let heading = add(scaled(along, cos), scaled(toward, sin));
    let side = sub(scaled(toward, cos), scaled(along, sin));
    (heading, spun(heading, side, plane))
}

#[cfg(test)]
mod angles_tests;
#[cfg(test)]
mod internodes_tests;
#[cfg(test)]
mod tests;
