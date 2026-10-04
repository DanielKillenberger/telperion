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
use crate::error::{Error, Result};
use crate::lineage::{Key, DOMINANCE, ROLL};
use crate::sag::turn;
use crate::species::{PaState, Species};
use crate::structure::{Axis, Origin, Structure, Vec3};
use std::f64::consts::TAU;

/// The height, in the wood's own radii, below which it eases onto the
/// ground.
const CONTACT: f64 = 4.0;
/// How far short of straight down sag leaves an axis, in radians.
const HANG: f64 = 0.15;
/// The internodes the landing turn is spread over: at most a sixth of the
/// slope is eased in one step.
const LANDING: f64 = 6.0;
/// Wood this far below the ground plane is rounding, not wood below it.
const GROUND_TOLERANCE: f64 = 1e-9;
const UP: Vec3 = Vec3::new(0.0, 0.0, 1.0);

/// Sizes every phytomer by its axis's vigour and its parent's scale, and
/// sets each relay's node.
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
}

/// Lays every axis from its parent's frame; with `torques` (`sag.rs`),
/// each phytomer also turns down under the load it carries.
pub(crate) fn place(
    structure: &mut Structure,
    species: &Species,
    torques: Option<&[Vec<Vec3>]>,
) -> Result<()> {
    let age = structure.age;
    // The trunk: the seed axis and what carries it on.
    let mut trunk = vec![false; structure.axes.len()];
    let reaches = reaches(&structure.axes);
    for (i, &reach) in reaches.iter().enumerate() {
        let (base, heading, side) = frame(structure, species, i);
        let axis = &mut structure.axes[i];
        let state = &species.states[axis.pa];
        // A relay straightens as far as it has left the continuation it
        // replaces (`blend`).
        let straightening = match axis.origin {
            Origin::Lateral { .. } => state.straightening,
            Origin::Relay { .. } => state.straightening * axis.blend,
            Origin::Seed | Origin::Continuation { .. } => 0.0,
        };
        // Secondary erection over the axis's years; never a continuation,
        // whose base is its parent's tip, and a relay as far as it is one.
        let years = f64::from(age.saturating_sub(axis.birth));
        let erected = match axis.origin {
            Origin::Continuation { .. } => 0.0,
            Origin::Relay { .. } => axis.blend * (1.0 - (-state.erection * years).exp()),
            _ => 1.0 - (-state.erection * years).exp(),
        };
        let bend = 1.0 - (1.0 - straightening) * (1.0 - erected);
        trunk[i] = match axis.origin {
            Origin::Seed => true,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => trunk[parent],
            Origin::Lateral { .. } => false,
        };
        let load = torques.map(|t| t[i].as_slice());
        let (pa, birth) = (axis.pa, axis.birth);
        let below = move |height: f64| Error::BelowGround {
            axis: i,
            pa,
            birth,
            base: base.z,
            height,
        };
        if base.z < -GROUND_TOLERANCE {
            return Err(below(-base.z));
        }
        lay(
            axis,
            (base, heading, side),
            state,
            (bend, reach),
            load,
            trunk[i],
        )
        .map_err(below)?;
    }
    Ok(())
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

/// Lays the axis's internodes from its base frame. A running direction
/// starts on the heading, bends towards the PA's elevation and wanders by
/// each node's keyed turn, both in proportion to the internode's length;
/// each phytomer's direction is that running direction pulled towards the
/// vertical by `bend` at the base, fading to none at the end of the
/// axis's scaled reach (`reaches`). The pull weakens as the heading turns down and
/// vanishes for a branch hanging straight down, which has no side to curl
/// up on.
fn lay(
    axis: &mut Axis,
    (base, heading, side): (Vec3, Vec3, Vec3),
    state: &PaState,
    (bend, total): (f64, f64),
    load: Option<&[Vec3]>,
    trunk: bool,
) -> std::result::Result<(), f64> {
    let form = state.form;
    let pull = (1.0 + heading.z) / 2.0;
    let (mut running, mut across) = (heading, side);
    let mut tip = base;
    let mut run = 0.0;
    for (k, phytomer) in axis.phytomers.iter_mut().enumerate() {
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
        // Sag: the beam's curvature, its moment over its stiffness, turns
        // the axis down about the torque's axis.
        if let (Some(load), true) = (load, form.sag > 0.0 && phytomer.radius > 0.0) {
            // The bending moment, taken on the tree before it bent, bends
            // the axis down in its own vertical plane however its bearers'
            // sag has turned it, and never lifts it. Normalised with no
            // cutoff: a vanishing moment bends by nothing, never by a jump.
            let moment = load[k].length();
            let across_plane = running.cross(Vec3::new(0.0, 0.0, -1.0));
            let width = across_plane.length();
            if width > 0.0 {
                let pivot = across_plane * (1.0 / width);
                let curvature = form.sag * moment / phytomer.radius.powi(4);
                // Short of straight down by `HANG`: a hanging axis keeps
                // the side it hangs to, so tropism never meets a vertical
                // it cannot place.
                let down = ((-running.z).clamp(-1.0, 1.0).acos() - HANG).max(0.0);
                let angle = turn(curvature, length, down);
                running = rotated(running, pivot, angle);
                across = rotated(across, pivot, angle);
            }
        }
        let along = if total > 0.0 {
            (run + phytomer.scale / 2.0) / total
        } else {
            1.0
        };
        run += phytomer.scale;
        let lift = bend * (1.0 - along);
        let blend = running * (1.0 - lift) + UP * (lift * pull);
        let mut direction = match blend.unit() {
            Some(unit) if bend > 0.0 => unit,
            _ => running,
        };
        // Landing: within a few radii of the ground, and `LANDING`
        // internodes so the turn is spread over steps, wood's downward
        // slope eases with its height, so it comes down tangent to the
        // ground and runs along it.
        if !trunk && direction.z < 0.0 {
            let zone = CONTACT * phytomer.radius + LANDING * length;
            if tip.z < zone {
                let level = |v: Vec3| Vec3::new(v.x, v.y, 0.0).unit();
                if let Some(flat) = level(direction).or_else(|| level(across)) {
                    // As the square root of the height: the slope vanishes
                    // at the ground, which is reached in a finite run.
                    let ease = (tip.z.max(0.0) / zone).sqrt();
                    let pitch = direction.z.max(-1.0).asin() * ease;
                    direction = flat * pitch.cos() + UP * pitch.sin();
                }
            }
        }
        let mut next = tip + direction * length;
        if next.z < -GROUND_TOLERANCE && trunk {
            return Err(-next.z);
        }
        if next.z < 0.0 {
            // Resting: down to the ground and along it, the internode's
            // length kept, towards where the wood was heading.
            let level = |v: Vec3| Vec3::new(v.x, v.y, 0.0).unit();
            let flat = level(direction)
                .or_else(|| level(across))
                .unwrap_or(Vec3::new(1.0, 0.0, 0.0));
            let drop = tip.z;
            let reach = (length * length - drop * drop).max(0.0).sqrt();
            next = tip + flat * reach;
            next.z = 0.0;
            if let Some(unit) = (next - tip).unit() {
                direction = unit;
            }
        }
        tip = next;
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

/// A lateral's share of its vigour among its siblings: by `dominance`,
/// from all of it (0) towards a keyed share that few laterals hold whole,
/// the fourth power of a draw its lineage keys.
fn dominance(dominance: f64, lineage: u64) -> f64 {
    if dominance <= 0.0 {
        return 1.0;
    }
    let held = Key(lineage).child(DOMINANCE).unit().powi(4);
    1.0 - dominance * (1.0 - held)
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
            // Turned about its parent by its PA's roll, as its lineage keys.
            let roll = species.states[axis.pa].form.roll
                * (2.0 * Key(axis.lineage).child(ROLL).unit() - 1.0);
            let azimuth =
                parent_state.divergence * at.rank + TAU * f64::from(slot) / f64::from(whorl) + roll;
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
            // Between the continuation it replaces, at the tip on the
            // axis's line, and its own bud in the axis's span at the PA's
            // `relay_at`, facing where a bud there would, turned up by the
            // PA's epitony: by the presence of the stop that made it.
            let p = &structure.axes[parent];
            let state = &species.states[p.pa];
            let at = relay_point(p, state.relay_at);
            let (heading, side) = turned(
                (at.along, at.side),
                state.divergence * at.rank,
                species.states[axis.pa].insertion,
                state.form.plane,
                state.epitony,
            );
            let (tip, along, across) = end(p);
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

/// The point `share` of the way along the nodes of axis `p`'s last growth
/// unit. The frame turns from one phytomer's direction to the next's along
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
    let f = first as f64 + share * (n - first) as f64;
    let k = (f.ceil() as usize).clamp(first + 1, n) - 1;
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
    // Turned towards the upper side through the smaller angle; the turn
    // fades as the bud faces straight down, where no side is nearer, so
    // no bud flips from one side to the other.
    if let Some(upper) = (UP - along * UP.dot(along)).unit() {
        let facing = toward.dot(upper);
        let turn = along.dot(toward.cross(upper)).atan2(facing);
        toward = rotated(toward, along, epitony * turn * (1.0 + facing) / 2.0);
    }
    let heading = along * angle.cos() + toward * angle.sin();
    let side = toward * angle.cos() - along * angle.sin();
    let side = side * plane.cos() + heading.cross(side) * plane.sin();
    (heading, side)
}
