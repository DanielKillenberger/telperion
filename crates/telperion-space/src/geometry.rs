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
mod single;
mod stand;

use crate::cores::{fill, map};
use crate::fixed::feedback::angle::{atan2, between, sincos, wrap};
use crate::fixed::feedback::wide::prod_shr;
use crate::fixed::{mul, q32, ONE32};
use crate::lineage::{insertion, Key, DOMINANCE, ROLL};
use crate::species::{PaState, Species};
use crate::structure::{Axis, Origin, Structure, Vec3};
pub(crate) use lay::{cone, ground_zone, support_q, Layer, Stop};
pub(crate) use place::{place, place_from};
pub(crate) use q::amount;
use q::{add, cross, dot, q3, rotated, scaled, sub, unit, v3, Q3, UP};
pub(crate) use single::{bud_frame, lay_one};
pub(crate) use stand::relay_span;
use stand::stand;

/// The sine of the lean below which a parent has too little of an upper
/// side for epitony to turn its buds by all of it: about 11.5 degrees.
const UPRIGHT: f64 = 0.2;

/// A Q32 as its exact real.
fn real(q: i64) -> f64 {
    q as f64 / ONE32 as f64
}

/// `trunc(a 2^32 / b)` of Q32 values, Q32.
fn over(a: i64, b: i64) -> i64 {
    ((i128::from(a) << 32) / i128::from(b)) as i64
}

/// A rate per rank (radians) times a rank (Q32), as an amount: one floor,
/// whole turns kept (CONTRACT-A.md 2.4).
fn by_rank(rate: f64, rank: f64) -> i64 {
    by_rank_q(rate, q32(rank))
}

/// `by_rank` of a rank already in Q32.
fn by_rank_q(rate: f64, rank: i64) -> i64 {
    prod_shr([amount(rate), rank, 1, 1], 32).expect("a phyllotactic amount")
}

/// `side` turned about `heading` by the amount `turn`.
fn spun(heading: Q3, side: Q3, turn: i64) -> Q3 {
    let (cos, sin) = sincos(wrap(turn));
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
    // once its parent is scaled. Contract A: parents first, one floor a
    // product, in this order.
    let mut base_scale = vec![ONE32; n];
    for i in 0..n {
        let (drawn, vigour, share) = own[i];
        let inherited = match axes[i].origin {
            Origin::Seed => ONE32,
            Origin::Lateral { parent, .. } => mul(q32(drawn), base_scale[parent]),
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => base_scale[parent],
        };
        base_scale[i] = mul(mul(inherited, q32(vigour)), q32(share));
    }
    let pieces: Vec<(usize, &mut [Axis])> = axes.chunks_mut(1 << 8).enumerate().collect();
    map(pieces, 1, threads, &|_, (k, piece)| {
        for (j, axis) in piece.iter_mut().enumerate() {
            let base = base_scale[k * (1 << 8) + j];
            axis.phytomers
                .iter_mut()
                .for_each(|p| p.scale = real(mul(q32(p.scale), base)));
        }
    });
    // A unit's size is its own: applied once every axis has inherited its
    // bearer's scale without it.
    let pieces: Vec<&mut [Axis]> = axes.chunks_mut(1 << 8).collect();
    map(pieces, 1, threads, &|_, piece| {
        for phytomer in piece.iter_mut().flat_map(|a| &mut a.phytomers) {
            phytomer.scale = real(mul(q32(phytomer.scale), q32(phytomer.size)));
        }
    });
    let base_scale: Vec<f64> = base_scale.into_iter().map(real).collect();
    base_scale
}

/// How far an axis `years` old is pulled towards the vertical at its
/// base: its straightening, and its secondary erection over its years;
/// an axis carried on only as far as it has left the line of the axis it
/// carries on (`Axis::carried_on`), so a continuation's none.
pub(crate) fn bend(axis: &Axis, state: &PaState, years: f64) -> f64 {
    // 1 - e^(-erection years), one product, then Q62's exponential.
    let x = mul(q32(state.erection), q32(years));
    let grown = ((crate::fixed::P_ONE
        - crate::fixed::exp2_neg(((i128::from(x) * crate::fixed::LOG2E) >> 62) as i64))
        >> 30) as i64;
    let (straightening, erected) = match (axis.origin, axis.carried_on()) {
        (_, Some((_, b))) => {
            let b = q32(b);
            (mul(q32(state.straightening), b), mul(b, grown))
        }
        (Origin::Lateral { .. }, None) => (q32(state.straightening), grown),
        _ => (0, grown),
    };
    real(ONE32 - mul(ONE32 - straightening, ONE32 - erected))
}

/// The scaled length each axis's straightening fades over: its own, and
/// its relays' as far as each is still the continuation it replaces, so a
/// relay that has barely left the axis's line leaves the axis as it was.
pub(crate) fn reaches(axes: &[Axis]) -> Vec<f64> {
    let mut reach: Vec<i64> = axes
        .iter()
        .map(|a| a.phytomers.iter().map(|p| q32(p.scale)).sum())
        .collect();
    // Relays follow their parents, so a backward pass meets each one's
    // reach before its parent's.
    for i in (0..axes.len()).rev() {
        if let Some((parent, b)) = axes[i].carried_on() {
            reach[parent] += mul(ONE32 - q32(b), reach[i]);
        }
    }
    reach.into_iter().map(real).collect()
}

/// A lateral's share of its vigour among its siblings: by `dominance`,
/// from all of it (0) towards a keyed share that few laterals hold whole,
/// the fourth power of a draw its lineage keys.
pub(crate) fn dominance(dominance: f64, lineage: u64) -> f64 {
    if dominance <= 0.0 {
        return 1.0;
    }
    let u = (Key(lineage).child(DOMINANCE).draw().0 >> 21) as i64;
    let held = mul(mul(mul(u, u), u), u);
    real(ONE32 - mul(q32(dominance), ONE32 - held))
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
/// Contract A (CONTRACT-A.md 2.4): Q32 throughout, angles as amounts.
pub(crate) fn frame(
    axes: &[Axis],
    species: &Species,
    i: usize,
    ends: &dyn Fn(usize) -> (Vec3, Vec3),
) -> (Vec3, Vec3, Vec3) {
    let (base, heading, side) = framed_q(axes, species, i, ends);
    (v3(base), v3(heading), v3(side))
}

fn framed_q(
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
        Origin::Seed => ([0; 3], UP, [ONE32, 0, 0]),
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
            let state = &species.states[axes[parent].pa];
            let at = stand(axes, species, parent, node);
            let (heading, side) = turned(
                (at.along, at.side),
                at.rank,
                angle(species, axis),
                amount(state.form.plane),
                q32(state.epitony),
            );
            // The axis's running direction and side at its tip, before its
            // pull towards the vertical, which the axis carried on runs on
            // with; its phyllotaxis runs on as a phase (`phase`).
            let (tip, _, _) = end(&axes[parent]);
            let (along, across) = ends(parent);
            let b = q32(b);
            let heading =
                unit(add(scaled(along, ONE32 - b), scaled(heading, b))).unwrap_or(heading);
            let side = add(scaled(across, ONE32 - b), scaled(side, b));
            let side = unit(sub(side, scaled(heading, dot(side, heading)))).unwrap_or(across);
            (add(tip, scaled(sub(at.base, tip), b)), heading, side)
        }
    }
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
    let signed = ((Key(lineage).child(ROLL).draw().0 >> 21) as i64) * 2 - ONE32;
    let roll = mul(amount(species.states[pa].form.roll), signed);
    let slot = over(i64::from(slot) << 32, q32(whorl));
    let azimuth =
        phase(axes, species, parent) + by_rank(parent_state.divergence, at.rank) + slot + roll;
    let state = &species.states[pa];
    let (heading, side) = turned(
        (q3(at.heading), q3(at.side)),
        azimuth,
        insertion(lineage, state.insertion, state.form.insertion_spread),
        amount(parent_state.form.plane),
        0,
    );
    // A bud that slept stands on its slot's side half an internode below
    // the node, apart from the bud that grew at once.
    let tip = q3(at.tip);
    let base = if woken {
        let below = node.checked_sub(1).map_or(p.base, |n| p.phytomers[n].tip);
        add(q3(below), tip).map(|c| c >> 1)
    } else {
        tip
    };
    (base, heading, side)
}

/// Axis `axis`'s insertion angle, an amount: its PA's, offset by its
/// lineage's draw at its PA's spread (fn-224).
fn angle(species: &Species, axis: &Axis) -> i64 {
    let state = &species.states[axis.pa];
    insertion(axis.lineage, state.insertion, state.form.insertion_spread)
}

/// The phase, an amount (Q32 turns), axis `i`'s phyllotaxis starts from:
/// each axis it carries on adds its divergence times its rank, so an axis
/// carried on turns its laterals on from where that axis's would have
/// stood (fn-199, host 2026-10-07), as far as it is still its
/// continuation; a lateral or the seed starts afresh.
pub(crate) fn phase(axes: &[Axis], species: &Species, i: usize) -> i64 {
    phase_by(axes, &|pa| species.states[pa].divergence, i)
}

/// `phase`, each PA's divergence (radians per rank) given by `divergence`.
pub(crate) fn phase_by(axes: &[Axis], divergence: &dyn Fn(usize) -> f64, i: usize) -> i64 {
    let mut line = Vec::new();
    let mut at = i;
    while let Some((parent, b)) = axes[at].carried_on() {
        line.push((parent, b));
        at = parent;
    }
    // Each axis carried on runs its parent's phyllotaxis on as far as it
    // is still that axis's continuation (`1 - blend`): a relay become its
    // own bud starts afresh, as a lateral does (fn-199). One floor a
    // product (CONTRACT-A.md 2.4).
    let mut phase = axes[at].phase;
    for &(parent, b) in line.iter().rev() {
        let p = &axes[parent];
        phase = mul(ONE32 - q32(b), phase + by_rank(divergence(p.pa), p.rank));
    }
    phase
}

/// The heading and side of a bud at `azimuth` around a parent going
/// `along` with `side`, turned `epitony` (Q32) of the way round to the
/// parent's upper side, inserted at `angle`, its plane turned by `plane`
/// (amounts).
fn turned((along, side): (Q3, Q3), azimuth: i64, angle: i64, plane: i64, epitony: i64) -> (Q3, Q3) {
    let mut toward = spun(along, side, azimuth);
    // Turned towards the upper side through the smaller angle; the turn
    // fades as the bud faces straight down, where no side is nearer, so
    // no bud flips from one side to the other, and as the parent stands
    // upright, where its upper side is no side at all and its direction
    // would swing with the least lean (fn-206).
    let level = sub(UP, scaled(along, dot(UP, along)));
    if let Some(upper) = unit(level) {
        let facing = dot(toward, upper);
        let turn = between(0, atan2(dot(along, cross(toward, upper)), facing));
        let sided = over(q::length(level), q32(UPRIGHT)).min(ONE32);
        // One rounding of the four factors and the half (n = 97).
        let epi = prod_shr([epitony, sided, turn, ONE32 + facing], 97).expect("an epitony");
        toward = rotated(toward, along, epi);
    }
    let (cos, sin) = sincos(wrap(angle));
    let heading = add(scaled(along, cos), scaled(toward, sin));
    let side = sub(scaled(toward, cos), scaled(along, sin));
    (heading, spun(heading, side, plane))
}

#[cfg(test)]
mod tests;
