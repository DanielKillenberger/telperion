//! Geometry: axes of internodes of their PA's length times their scale,
//! each lateral turned from its parent's direction at its node by its PA's
//! insertion angle, around the parent at the azimuth its node's
//! phyllotactic rank and its slot in the whorl give, in a plane turned by
//! the parent's `plane`. Along an axis the direction bends towards its
//! PA's elevation (tropism) and wanders by keyed turns; a lateral's base
//! straightens towards the vertical by its PA's straightening. The seed
//! stands at the origin, growing up (+z); no wood goes below z = 0.
use crate::error::{Error, Result};
use crate::lineage::Key;
use crate::species::{PaState, Species};
use crate::structure::{Axis, Origin, Structure, Vec3};
use std::f64::consts::TAU;

/// Wood this far below the ground plane is rounding, not wood below it.
const GROUND_TOLERANCE: f64 = 1e-9;
const UP: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub(crate) fn place(structure: &mut Structure, species: &Species) -> Result<()> {
    let age = structure.age;
    // Each axis's scale at its base, from its already scaled parent.
    let mut base_scale = vec![1.0; structure.axes.len()];
    for i in 0..structure.axes.len() {
        let (base, heading, side) = frame(structure, species, i);
        if let Origin::Relay { parent, .. } = structure.axes[i].origin {
            let share = species.states[structure.axes[parent].pa].relay_at;
            let node = relay_point(&structure.axes[parent], share).node;
            structure.axes[i].origin = Origin::Relay { parent, node };
        }
        let inherited = match structure.axes[i].origin {
            Origin::Seed => 1.0,
            Origin::Lateral { parent, node, .. } => structure.axes[parent].phytomers[node].scale,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => base_scale[parent],
        };
        let axis = &mut structure.axes[i];
        base_scale[i] = inherited * axis.vigour;
        for phytomer in &mut axis.phytomers {
            phytomer.scale *= base_scale[i];
        }
        let state = &species.states[axis.pa];
        let straightening = match axis.origin {
            Origin::Lateral { .. } | Origin::Relay { .. } => state.straightening,
            Origin::Seed | Origin::Continuation { .. } => 0.0,
        };
        // Secondary erection over the axis's years; never a continuation,
        // whose base is its parent's tip.
        let erected = match axis.origin {
            Origin::Continuation { .. } => 0.0,
            _ => 1.0 - (-state.erection * f64::from(age.saturating_sub(axis.birth))).exp(),
        };
        let bend = 1.0 - (1.0 - straightening) * (1.0 - erected);
        lay(axis, (base, heading, side), state, bend)
            .map_err(|height| Error::BelowGround { axis: i, height })?;
    }
    Ok(())
}

/// Lays the axis's internodes from its base frame. A running direction
/// starts on the heading, bends towards the PA's elevation and wanders by
/// each node's keyed turn, both in proportion to the internode's length;
/// each phytomer's direction is that running direction pulled towards the
/// vertical by `bend` at the base, fading to none at the tip along the
/// axis's scaled length. The pull weakens as the heading turns down and
/// vanishes for a branch hanging straight down, which has no side to curl
/// up on.
fn lay(
    axis: &mut Axis,
    (base, heading, side): (Vec3, Vec3, Vec3),
    state: &PaState,
    bend: f64,
) -> std::result::Result<(), f64> {
    let form = state.form;
    let total: f64 = axis.phytomers.iter().map(|p| p.scale).sum();
    let pull = (1.0 + heading.z) / 2.0;
    let (mut running, mut across) = (heading, side);
    let mut tip = base;
    let mut run = 0.0;
    for phytomer in &mut axis.phytomers {
        let length = state.internode * phytomer.scale;
        let share = 1.0 - (-form.tropism * length).exp();
        let bent = toward_elevation(running, across, form.elevation, share);
        across = carried(across, running, bent);
        running = bent;
        if form.wander > 0.0 {
            let key = Key(phytomer.key).child(WANDER);
            let angle = form.wander * length * (2.0 * key.child(0).unit() - 1.0);
            let azimuth = TAU * key.child(1).unit();
            let pivot = across * azimuth.cos() + running.cross(across) * azimuth.sin();
            running = rotated(running, pivot, angle);
            across = rotated(across, pivot, angle);
        }
        let along = if total > 0.0 {
            (run + phytomer.scale / 2.0) / total
        } else {
            1.0
        };
        run += phytomer.scale;
        let lift = bend * (1.0 - along);
        let blend = running * (1.0 - lift) + UP * (lift * pull);
        let direction = match blend.unit() {
            Some(unit) if bend > 0.0 => unit,
            _ => running,
        };
        tip = tip + direction * length;
        if tip.z < -GROUND_TOLERANCE {
            return Err(-tip.z);
        }
        phytomer.tip = tip;
        phytomer.heading = direction;
        phytomer.side = (across - direction * across.dot(direction))
            .unit()
            .unwrap_or(across);
    }
    axis.base = base;
    axis.heading = heading;
    axis.side = side;
    Ok(())
}

/// The key step of a node's wander draws.
const WANDER: u64 = 1;

/// `direction` turned in its vertical plane by `share` of its gap to
/// `elevation`. A vertical direction turns towards its side.
fn toward_elevation(direction: Vec3, side: Vec3, elevation: f64, share: f64) -> Vec3 {
    if share <= 0.0 {
        return direction;
    }
    let level = |v: Vec3| Vec3::new(v.x, v.y, 0.0).unit();
    let Some(out) = level(direction).or_else(|| level(side)) else {
        return direction;
    };
    let now = direction.z.clamp(-1.0, 1.0).asin();
    let to = now + (elevation - now) * share;
    out * to.cos() + UP * to.sin()
}

/// `v` carried by the least rotation that takes unit `from` to unit `to`.
fn carried(v: Vec3, from: Vec3, to: Vec3) -> Vec3 {
    let axis = from.cross(to);
    let sin = axis.length();
    match axis.unit() {
        Some(pivot) => rotated(v, pivot, sin.atan2(from.dot(to))),
        None => v,
    }
}

/// `v` rotated by `angle` about the unit `pivot` (Rodrigues).
fn rotated(v: Vec3, pivot: Vec3, angle: f64) -> Vec3 {
    let (sin, cos) = angle.sin_cos();
    v * cos + pivot.cross(v) * sin + pivot * (pivot.dot(v) * (1.0 - cos))
}

/// Where axis `p` ends: its last node and the frame there, or its base.
fn end(p: &Axis) -> (Vec3, Vec3, Vec3) {
    p.phytomers
        .last()
        .map_or((p.base, p.heading, p.side), |last| {
            (last.tip, last.heading, last.side)
        })
}

/// The base, heading and side of axis `i`, from its already placed parent.
fn frame(structure: &Structure, species: &Species, i: usize) -> (Vec3, Vec3, Vec3) {
    let axis = &structure.axes[i];
    match axis.origin {
        Origin::Seed => (Vec3::default(), UP, Vec3::new(1.0, 0.0, 0.0)),
        Origin::Continuation { parent } => {
            let p = &structure.axes[parent];
            let (base, heading, side) = end(p);
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
        } => {
            let p = &structure.axes[parent];
            let at = &p.phytomers[node];
            let parent_state = &species.states[p.pa];
            let azimuth =
                parent_state.divergence * at.rank + TAU * f64::from(slot) / f64::from(whorl);
            let (heading, side) = turned(
                (at.heading, at.side),
                azimuth,
                species.states[axis.pa].insertion,
                parent_state.form.plane,
                0.0,
            );
            (at.tip, heading, side)
        }
        Origin::Relay { parent, .. } => {
            // In its parent's span at the parent PA's `relay_at`, facing
            // where a bud there would, turned up by the PA's epitony.
            let p = &structure.axes[parent];
            let state = &species.states[p.pa];
            let at = relay_point(p, state.relay_at);
            let azimuth = state.divergence * at.rank;
            let (heading, side) = turned(
                (at.along, at.side),
                azimuth,
                species.states[axis.pa].insertion,
                state.form.plane,
                state.epitony,
            );
            (at.base, heading, side)
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

/// The point `share` of the way along axis `p`'s nodes from its base. The
/// frame turns from one phytomer's direction to the next's along each span,
/// so the point's frame moves by degree as `share` does; at 1 it is the
/// last node and the frame of its phytomer, at 0 the base.
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
    let f = share * n as f64;
    let k = (f.ceil() as usize).clamp(1, n) - 1;
    let t = (f - k as f64).clamp(0.0, 1.0);
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
    if let Some(upper) = (UP - along * UP.dot(along)).unit() {
        let turn = along.dot(toward.cross(upper)).atan2(toward.dot(upper));
        toward = rotated(toward, along, epitony * turn);
    }
    let heading = along * angle.cos() + toward * angle.sin();
    let side = toward * angle.cos() - along * angle.sin();
    let side = side * plane.cos() + heading.cross(side) * plane.sin();
    (heading, side)
}
