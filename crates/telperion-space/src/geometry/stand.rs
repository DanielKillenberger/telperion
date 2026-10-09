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
//! fn-199's R3 walk catches (DENTS.md). Contract A (CONTRACT-A.md 2.4):
//! Q32 throughout, the azimuth an amount, one floor a product.
use super::q::{add, dot, q3, scaled, sub, unit, Q3};
use super::{by_rank_q, over, phase};
use crate::fixed::{mul, q32, ONE32};
use crate::species::Species;
use crate::structure::Axis;

/// A point along an axis and the frame there, Q32; its rank, once on a
/// line, the azimuth (an amount) it turns a bud by (`stand`).
#[derive(Clone, Copy)]
pub(super) struct Point {
    pub base: Q3,
    pub along: Q3,
    pub side: Q3,
    pub rank: i64,
}

impl Point {
    /// `t` (Q32) of the way from this point to `to`, its frame turned as far.
    fn toward(self, to: Point, t: i64) -> Point {
        let blend = |a: Q3, b: Q3| unit(add(scaled(a, ONE32 - t), scaled(b, t))).unwrap_or(b);
        let along = blend(self.along, to.along);
        Point {
            base: add(self.base, scaled(sub(to.base, self.base), t)),
            along,
            side: square(blend(self.side, to.side), along),
            rank: self.rank + mul(t, to.rank - self.rank),
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
    let mut units: Vec<(Point, i64)> = Vec::new();
    let mut a = p;
    let base = loop {
        let axis = &axes[a];
        let state = &species.states[axis.pa];
        let turn = |at: Point| Point {
            rank: phase(axes, species, a) + by_rank_q(state.divergence, at.rank),
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
            let extent: i64 = axis.phytomers[first..end]
                .iter()
                .map(|q| q32(q.grown))
                .sum();
            units.push((at, extent.min(ONE32)));
            if extent >= ONE32 {
                break;
            }
        }
        if units.last().is_some_and(|&(_, e)| e >= ONE32) {
            break None;
        }
        match axis.carried_on() {
            Some((parent, _)) => a = parent,
            None => {
                break Some(turn(Point {
                    base: q3(axis.base),
                    along: q3(axis.heading),
                    side: q3(axis.side),
                    rank: 0,
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
/// unit, in `p`'s nodes, Q32 (`at`): `p`'s last unit at `relay_at`,
/// blended back towards the one before by its extent, that one towards the
/// one before it by its own, down to a whole unit or `p`'s start.
pub(super) fn position(axes: &[Axis], species: &Species, p: usize, node: usize) -> i64 {
    let axis = &axes[p];
    let relay_at = species.states[axis.pa].relay_at;
    // The units newest first, each with its position and extent, to the
    // first grown whole.
    let mut units: Vec<(i64, i64)> = Vec::new();
    let ranges = unit_ranges(axis);
    for (u, &(first, end)) in ranges.iter().enumerate().rev() {
        let k = if u + 1 == ranges.len() {
            node.clamp(first, end - 1)
        } else {
            zone_node(axis, (first, end), relay_at)
        };
        let x = ((k as i64) << 32) + span_in(axis, (first, end), relay_at, k);
        let extent: i64 = axis.phytomers[first..end]
            .iter()
            .map(|q| q32(q.grown))
            .sum();
        units.push((x, extent.min(ONE32)));
        if extent >= ONE32 {
            break;
        }
    }
    let mut x = match units.last() {
        Some(&(x, e)) if e >= ONE32 => {
            units.pop();
            x
        }
        _ => 0,
    };
    while let Some((to, extent)) = units.pop() {
        x += mul(to - x, extent);
    }
    x
}

/// The point at position `x` along axis `p`, in nodes, Q32: `t` of the way
/// along the span of node `k` from node `k - 1` (or the base) to node `k`,
/// where `x` is `k` and `t`; `p`'s base where it has no nodes.
pub(super) fn at(p: &Axis, x: i64) -> Q3 {
    let Some(n) = p.phytomers.len().checked_sub(1) else {
        return q3(p.base);
    };
    let (k, t) = split(x, n);
    let start = q3(if k == 0 { p.base } else { p.phytomers[k - 1].tip });
    add(start, scaled(sub(q3(p.phytomers[k].tip), start), t))
}

/// Position `x` along an axis of last node `last`, in nodes, Q32, as its
/// node and the share of that node's span, Q32: the last node's whole
/// span at the tip and beyond.
pub(crate) fn split(x: i64, last: usize) -> (usize, i64) {
    let k = usize::try_from(x >> 32).unwrap_or(0).min(last);
    (k, (x - ((k as i64) << 32)).clamp(0, ONE32))
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
    let share = q32(share);
    let total: i64 = p.phytomers[first..end].iter().map(|q| q32(q.scale)).sum();
    if total <= 0 {
        // ceil(share n), the share Q32 and n whole: exact.
        let k = ((share * (end - first) as i64 + ONE32 - 1) >> 32) as usize;
        return first + k.clamp(1, end - first) - 1;
    }
    let (want, mut before) = (mul(share, total), 0);
    for (j, q) in p.phytomers[first..end].iter().enumerate() {
        let scale = q32(q.scale);
        if scale > 0 && before + scale >= want {
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
/// node's, Q32.
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
        unit(add(scaled(a, ONE32 - t), scaled(b, t))).unwrap_or(a)
    };
    let along = blend(at.heading, next.map_or(at.heading, |q| q.heading));
    let rank = q32(at.rank);
    Point {
        base: add(start, scaled(sub(q3(at.tip), start), t)),
        along,
        side: square(blend(at.side, next.map_or(at.side, |q| q.side)), along),
        rank: rank + mul(t, q32(next.map_or(p.rank, |q| q.rank)) - rank),
    }
}

/// How far, Q32, along the span of node `k` of the unit `range` (from
/// node `k - 1`, or the base, to node `k`) a relay at `share` of the unit
/// stands, by its phytomers' scaled lengths, so a node growing in from
/// nothing moves it by nothing (fn-206).
fn span_in(p: &Axis, (first, end): (usize, usize), share: f64, k: usize) -> i64 {
    let share = q32(share);
    let total: i64 = p.phytomers[first..end].iter().map(|q| q32(q.scale)).sum();
    let scale = q32(p.phytomers[k].scale);
    if total > 0 && scale > 0 {
        let before: i64 = p.phytomers[first.min(k)..k]
            .iter()
            .map(|q| q32(q.scale))
            .sum();
        over(mul(share, total) - before, scale).clamp(0, ONE32)
    } else if total > 0 {
        ONE32
    } else {
        let f = ((first as i64) << 32) + mul(share, ((end - first) as i64) << 32);
        (f - ((k as i64) << 32)).clamp(0, ONE32)
    }
}
