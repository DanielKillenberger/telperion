//! Where a relay stands on the axis it carries on (fn-199, host
//! 2026-10-07, rule 2'): at its PA's `relay_at` along that axis's last
//! growth unit, blended back towards the unit before by how much of the
//! last unit has grown, its extent: each node's unit presence times its
//! node presence times the share of its cycle the unit lived, summed, at
//! most one. Two readings of it (fn-244 host decision 4, option 1):
//!
//! - its frame (`stand`) reads units along the whole line of axes carried
//!   on, as one axis, so the weight belongs to the unit and not to the axis
//!   holding it, and a unit growing in from nothing turns no relay;
//! - its position (`position`) reads the parent's own units alone, short
//!   of a whole one back towards the parent's start, so the relay joins
//!   the wood it grows from: the one attachment girth, sag and the drawing
//!   read (`geometry::attach`).
//!
//! The frame keeps the line because a stop that splits a growing unit off
//! as a new, barely present relay hands that unit to another axis: read
//! on the parent alone, the frame of a relay standing on it would swing
//! from the unit before to the new relay's start at that crossing, a jump
//! fn-199's R3 walk catches (DENTS.md). In f32 (fn-254), the azimuth an
//! amount in turns, a position along an axis its node and the share of
//! that node's span.
use super::q::{add, dot, q3, scaled, sub, unit, Q3};
use super::{by_rank, phase};
use crate::species::Species;
use crate::structure::Axis;

/// A point along an axis and the frame there; its rank, once on a line,
/// the azimuth (an amount) it turns a bud by (`stand`).
#[derive(Clone, Copy)]
pub(super) struct Point {
    pub base: Q3,
    pub along: Q3,
    pub side: Q3,
    pub rank: f32,
}

impl Point {
    /// `t` of the way from this point to `to`, its frame turned as far.
    fn toward(self, to: Point, t: f32) -> Point {
        let blend = |a: Q3, b: Q3| unit(add(scaled(a, 1.0 - t), scaled(b, t))).unwrap_or(b);
        let along = blend(self.along, to.along);
        Point {
            base: add(self.base, scaled(sub(to.base, self.base), t)),
            along,
            side: square(blend(self.side, to.side), along),
            rank: self.rank + t * (to.rank - self.rank),
        }
    }
}

/// Where a relay of axis `p` stands, `node` its node on `p`'s last unit
/// (FINE-CONTRACT.md D18): the last unit of the line ending in `p` at
/// `relay_at`, blended back towards the one before by its extent, that
/// one towards the one before it by its own, down to a whole unit or the
/// line's base. Its rank is the azimuth a bud there turns by.
pub(super) fn stand(axes: &[Axis], species: &Species, p: usize, node: usize) -> Point {
    // The units newest first, each with its point and extent, to the
    // first grown whole.
    let mut units: Vec<(Point, f32)> = Vec::new();
    let mut a = p;
    let base = loop {
        let axis = &axes[a];
        let state = &species.states[axis.pa];
        let turn = |at: Point| Point {
            rank: phase(axes, species, a) + by_rank(state.divergence, at.rank),
            ..at
        };
        let ranges = unit_ranges(axis);
        for (u, &(first, end)) in ranges.iter().enumerate().rev() {
            let newest = a == p && u + 1 == ranges.len();
            let k = if newest {
                node.clamp(first, end - 1)
            } else {
                zone_node(axis, (first, end), state.relay_at)
            };
            let at = turn(point_in(axis, (first, end), state.relay_at, k));
            let extent = extent(axis, (first, end));
            units.push((at, extent.min(1.0)));
            if extent >= 1.0 {
                break;
            }
        }
        if units.last().is_some_and(|&(_, e)| e >= 1.0) {
            break None;
        }
        match axis.carried_on() {
            Some((parent, _)) => a = parent,
            None => {
                break Some(turn(Point {
                    base: q3(axis.base),
                    along: q3(axis.heading),
                    side: q3(axis.side),
                    rank: 0.0,
                }))
            }
        }
    };
    let mut point = match base {
        Some(base) => base,
        None => units.pop().expect("a whole unit").0,
    };
    while let Some((at, extent)) = units.pop() {
        point = point.toward(at, extent);
    }
    point
}

/// How far along axis `p` a relay stands, `node` its node on `p`'s last
/// unit, in `p`'s scaled length from its base (`node_x`): `p`'s last unit
/// at `relay_at`, blended back towards the one before by its extent, that
/// one towards the one before it by its own, down to a whole unit or
/// `p`'s start. Blended by length, so a node growing in from nothing
/// moves no relay (fn-244 review).
pub(super) fn position(axes: &[Axis], species: &Species, p: usize, node: usize) -> f32 {
    let axis = &axes[p];
    let relay_at = species.states[axis.pa].relay_at;
    // The units newest first, each with its position and extent, to the
    // first grown whole.
    let mut units: Vec<(f32, f32)> = Vec::new();
    let ranges = unit_ranges(axis);
    for (u, &(first, end)) in ranges.iter().enumerate().rev() {
        let k = if u + 1 == ranges.len() {
            node.clamp(first, end - 1)
        } else {
            zone_node(axis, (first, end), relay_at)
        };
        let x = length_to(axis, k, span_in(axis, (first, end), relay_at, k));
        let extent = extent(axis, (first, end));
        units.push((x, extent.min(1.0)));
        if extent >= 1.0 {
            break;
        }
    }
    let mut x = match units.last() {
        Some(&(x, e)) if e >= 1.0 => {
            units.pop();
            x
        }
        _ => 0.0,
    };
    while let Some((to, extent)) = units.pop() {
        x += (to - x) * extent;
    }
    x
}

/// A unit's extent: its nodes' grown shares, summed base to tip.
fn extent(p: &Axis, (first, end): (usize, usize)) -> f32 {
    p.phytomers[first..end].iter().map(|q| q.grown as f32).sum()
}

/// The scaled length of the phytomers `range` of axis `p`, base to tip.
fn length_of(p: &Axis, (first, end): (usize, usize)) -> f32 {
    p.phytomers[first..end].iter().map(|q| q.scale as f32).sum()
}

/// The scaled length along axis `p` to `t` of the way along its node `k`'s
/// span.
fn length_to(p: &Axis, k: usize, t: f32) -> f32 {
    length_of(p, (0, k)) + t * p.phytomers[k].scale as f32
}

/// The scaled length along axis `p` as a position in its nodes (`split`):
/// in the first span that reaches it, the tip past them all.
pub(super) fn node_x(p: &Axis, length: f32) -> Position {
    let mut before = 0.0;
    for (k, q) in p.phytomers.iter().enumerate() {
        let scale = q.scale as f32;
        if scale > 0.0 && before + scale >= length {
            return (k, ((length - before) / scale).clamp(0.0, 1.0));
        }
        before += scale;
    }
    (p.phytomers.len(), 0.0)
}

/// The scaled length of axis `p`.
pub(super) fn total(p: &Axis) -> f32 {
    length_of(p, (0, p.phytomers.len()))
}

/// A position along an axis: a node, and the share of its span from the
/// node before (or the base) to it; a node past the last is the tip.
pub(crate) type Position = (usize, f32);

/// The point at position `x` along axis `p`: `t` of the way along the span
/// of node `k` from node `k - 1` (or the base) to node `k`; `p`'s base
/// where it has no nodes.
pub(super) fn at(p: &Axis, x: Position) -> Q3 {
    let Some(n) = p.phytomers.len().checked_sub(1) else {
        return q3(p.base);
    };
    let (k, t) = split(x, n);
    let start = q3(if k == 0 {
        p.base
    } else {
        p.phytomers[k - 1].tip
    });
    add(start, scaled(sub(q3(p.phytomers[k].tip), start), t))
}

/// Position `x` along an axis of last node `last` as a node of it and the
/// share of that node's span: the last node's whole span at the tip and
/// beyond.
pub(crate) fn split((k, t): Position, last: usize) -> (usize, f32) {
    if k > last {
        return (last, 1.0);
    }
    (k, t.clamp(0.0, 1.0))
}

/// `side` made square to `along`, so a bud's frame stays a frame however
/// far apart the frames it was blended from stood: a bud turned by epitony
/// reads its facing from it, which an oblique side would step at the
/// turn's cut (fn-199).
fn square(side: Q3, along: Q3) -> Q3 {
    unit(sub(side, scaled(along, dot(side, along)))).unwrap_or(side)
}

/// Each growth unit's phytomers, as index ranges, base to tip.
fn unit_ranges(p: &Axis) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::new();
    for (j, q) in p.phytomers.iter().enumerate() {
        match out.last_mut() {
            Some(r) if p.phytomers[r.0].cycle == q.cycle => r.1 = j + 1,
            _ => out.push((j, j + 1)),
        }
    }
    out
}

/// The node of the unit `range` of axis `p` a relay at `share` of it
/// stands in, by the phytomers' scaled lengths, as the contract's node
/// is decided on their presences.
fn zone_node(p: &Axis, (first, end): (usize, usize), share: f64) -> usize {
    let share = share as f32;
    let total = length_of(p, (first, end));
    if total <= 0.0 {
        let k = (share * (end - first) as f32).ceil() as usize;
        return first + k.clamp(1, end - first) - 1;
    }
    let (want, mut before) = (share * total, 0.0);
    for (j, q) in p.phytomers[first..end].iter().enumerate() {
        let scale = q.scale as f32;
        if scale > 0.0 && before + scale >= want {
            return first + j;
        }
        before += scale;
    }
    end - 1
}

/// The point `share` of the way along the unit `range` of axis `p`, by
/// its phytomers' scaled lengths, in the span of its node `k`. The frame
/// turns from one phytomer's direction to the next's along each span, so
/// the point's frame moves by degree as `share` does. Its rank is the
/// node's.
fn point_in(p: &Axis, range: (usize, usize), share: f64, k: usize) -> Point {
    let t = span_in(p, range, share, k);
    let at = &p.phytomers[k];
    let start = q3(if k == 0 {
        p.base
    } else {
        p.phytomers[k - 1].tip
    });
    let next = p.phytomers.get(k + 1);
    let blend = |a, b| {
        let (a, b) = (q3(a), q3(b));
        unit(add(scaled(a, 1.0 - t), scaled(b, t))).unwrap_or(a)
    };
    let along = blend(at.heading, next.map_or(at.heading, |q| q.heading));
    let rank = at.rank as f32;
    Point {
        base: add(start, scaled(sub(q3(at.tip), start), t)),
        along,
        side: square(blend(at.side, next.map_or(at.side, |q| q.side)), along),
        rank: rank + t * (next.map_or(p.rank, |q| q.rank) as f32 - rank),
    }
}

/// How far along the span of node `k` of the unit `range` (from node `k -
/// 1`, or the base, to node `k`) a relay at `share` of the unit stands, by
/// its phytomers' scaled lengths, so a node growing in from nothing moves
/// it by nothing (fn-206).
fn span_in(p: &Axis, (first, end): (usize, usize), share: f64, k: usize) -> f32 {
    let share = share as f32;
    let total = length_of(p, (first, end));
    let scale = p.phytomers[k].scale as f32;
    if total > 0.0 && scale > 0.0 {
        let before = length_of(p, (first.min(k), k));
        ((share * total - before) / scale).clamp(0.0, 1.0)
    } else if total > 0.0 {
        1.0
    } else {
        (share * (end - first) as f32 - (k - first) as f32).clamp(0.0, 1.0)
    }
}
