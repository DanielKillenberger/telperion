//! Where a relay stands on the axis it carries on (fn-199, host
//! 2026-10-07, rule 2'): at its PA's `relay_at` along that axis's last
//! growth unit, blended back towards the unit before by how much of the
//! last unit has grown, its extent: each node's unit presence times its
//! node presence times the share of its cycle the unit lived, summed, at
//! most one. Units are read along the whole line of axes carried on (a
//! relay or a continuation and the axis it carries on), as one axis, so
//! the weight belongs to the unit and not to the axis holding it, and a
//! unit growing in from nothing moves no relay.
use super::phase;
use crate::species::Species;
use crate::structure::{Axis, Vec3};

/// A point along an axis and the frame there; its rank, once on a line,
/// the azimuth it turns a bud by (`stand`).
#[derive(Clone, Copy)]
pub(super) struct Point {
    pub base: Vec3,
    pub along: Vec3,
    pub side: Vec3,
    pub rank: f64,
}

impl Point {
    /// `t` of the way from this point to `to`, its frame turned as far.
    fn toward(self, to: Point, t: f64) -> Point {
        let blend = |a: Vec3, b: Vec3| (a * (1.0 - t) + b * t).unit().unwrap_or(b);
        let along = blend(self.along, to.along);
        Point {
            base: self.base + (to.base - self.base) * t,
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
    let mut units: Vec<(Point, f64)> = Vec::new();
    let mut a = p;
    let base = loop {
        let axis = &axes[a];
        let state = &species.states[axis.pa];
        let turn = |at: Point| Point {
            rank: phase(axes, species, a) + state.divergence * at.rank,
            ..at
        };
        for (u, &(first, end)) in unit_ranges(axis).iter().enumerate().rev() {
            let newest = a == p && u + 1 == unit_ranges(axis).len();
            let k = if newest {
                node.clamp(first, end - 1)
            } else {
                zone_node(axis, (first, end), state.relay_at)
            };
            let at = turn(point_in(axis, (first, end), state.relay_at, k));
            let extent: f64 = axis.phytomers[first..end].iter().map(|q| q.grown).sum();
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
                    base: axis.base,
                    along: axis.heading,
                    side: axis.side,
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

/// `side` made square to `along`, so a bud's frame stays a frame however
/// far apart the frames it was blended from stood: a bud turned by epitony
/// reads its facing from it, which an oblique side would step at the
/// turn's cut (fn-199).
fn square(side: Vec3, along: Vec3) -> Vec3 {
    (side - along * side.dot(along)).unit().unwrap_or(side)
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
    let total: f64 = p.phytomers[first..end].iter().map(|q| q.scale).sum();
    if total <= 0.0 {
        let k = (share * (end - first) as f64).ceil() as usize;
        return first + k.clamp(1, end - first) - 1;
    }
    let mut before = 0.0;
    for (j, q) in p.phytomers[first..end].iter().enumerate() {
        if q.scale > 0.0 && before + q.scale >= share * total {
            return first + j;
        }
        before += q.scale;
    }
    end - 1
}

/// The point `share` of the way along the unit `range` of axis `p`, by
/// its phytomers' scaled lengths, in the span of its node `k`. The frame
/// turns from one phytomer's direction to the next's along each span, so
/// the point's frame moves by degree as `share` does.
fn point_in(p: &Axis, range: (usize, usize), share: f64, k: usize) -> Point {
    let t = span_in(p, range, share, k);
    let at = &p.phytomers[k];
    let start = if k == 0 {
        p.base
    } else {
        p.phytomers[k - 1].tip
    };
    let next = p.phytomers.get(k + 1);
    let blend = |a: Vec3, b: Vec3| (a * (1.0 - t) + b * t).unit().unwrap_or(a);
    let along = blend(at.heading, next.map_or(at.heading, |q| q.heading));
    Point {
        base: start + (at.tip - start) * t,
        along,
        side: square(blend(at.side, next.map_or(at.side, |q| q.side)), along),
        rank: at.rank + t * (next.map_or(p.rank, |q| q.rank) - at.rank),
    }
}

/// How far along the span of node `k` of the unit `range` (from node
/// `k - 1`, or the base, to node `k`) a relay at `share` of the unit
/// stands, by its phytomers' scaled lengths, so a node growing in from
/// nothing moves it by nothing (fn-206).
fn span_in(p: &Axis, (first, end): (usize, usize), share: f64, k: usize) -> f64 {
    let total: f64 = p.phytomers[first..end].iter().map(|q| q.scale).sum();
    let scale = p.phytomers[k].scale;
    if total > 0.0 && scale > 0.0 {
        let before: f64 = p.phytomers[first.min(k)..k].iter().map(|q| q.scale).sum();
        ((share * total - before) / scale).clamp(0.0, 1.0)
    } else if total > 0.0 {
        1.0
    } else {
        let f = first as f64 + share * (end - first) as f64;
        (f - k as f64).clamp(0.0, 1.0)
    }
}

/// `span_in` on axis `p`'s last unit: girth and sag take a relay's pipe
/// and load in at the two ends of that span by it (fn-199 R4). An axis of
/// no phytomers has no span: 1.
pub(crate) fn relay_span(p: &Axis, share: f64, k: usize) -> f64 {
    match unit_ranges(p).last() {
        Some(&range) => span_in(p, range, share, k.min(range.1 - 1)),
        None => 1.0,
    }
}
