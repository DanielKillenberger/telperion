//! Laying an axis's internodes, one phytomer at a time: the final lay
//! walks an axis from its base, and the rough layout grown with the tree
//! (fn-197) carries the same walk on as each growth unit grows.
use crate::lineage::Key;
use crate::sag::{held, torque, turn, Lever};
use crate::species::PaState;
use crate::structure::{Axis, Phytomer, Vec3};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// The height, in the wood's own radii, below which it eases onto the
/// ground.
const CONTACT: f64 = 4.0;
/// The cone about straight down, in radians, within which tropism weakens
/// with an axis's lean.
const DOWNWARD: f64 = 0.05;
/// How far short of straight down sag leaves an axis, in radians.
const HANG: f64 = 0.15;
/// The internodes the landing turn is spread over: at most a sixth of the
/// slope is eased in one step.
const LANDING: f64 = 6.0;
/// Wood this far below the ground plane is rounding, not wood below it.
pub(crate) const GROUND_TOLERANCE: f64 = 1e-9;
pub(super) const UP: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const DOWN: Vec3 = Vec3::new(0.0, 0.0, -1.0);

/// Where a walk along an axis stands: its running direction and side,
/// its tip, the scaled length it has run, how far its heading lets a bend
/// pull it up, and its wander's curvature (fn-207): a vector square to the
/// running direction, in radians per metre, the pivot its turns go about.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Layer {
    running: Vec3,
    across: Vec3,
    tip: Vec3,
    run: f64,
    pull: f64,
    curve: Vec3,
}

impl Layer {
    /// At the base of an axis framed `(base, heading, side)` carrying on
    /// `curve`, its bearer's curvature, made square to its heading: a
    /// continuation or relay keeps bending as its bearer did, and a
    /// lateral starts afresh from none.
    pub fn bent((base, heading, side): (Vec3, Vec3, Vec3), curve: Vec3) -> Self {
        Self {
            running: heading,
            across: side,
            tip: base,
            run: 0.0,
            pull: (1.0 + heading.z) / 2.0,
            curve: curve - heading * curve.dot(heading),
        }
    }

    /// The wander's curvature where the walk stands.
    pub fn curve(&self) -> Vec3 {
        self.curve
    }

    /// The scaled length laid so far.
    pub fn run(&self) -> f64 {
        self.run
    }

    /// Lays the next phytomer. A running direction bends towards the PA's
    /// elevation and wanders by the node's keyed turn, both in proportion
    /// to the internode's length; the phytomer's direction is that running
    /// direction pulled towards the vertical by `bend` at the base, fading
    /// to none at `total`, the axis's scaled reach. With `load`, it turns
    /// down by its sag first. Err: the trunk would reach below the ground,
    /// by this much.
    pub fn step(
        &mut self,
        phytomer: &mut Phytomer,
        state: &PaState,
        (bend, total): (f64, f64),
        load: Option<&Lever>,
        trunk: bool,
    ) -> std::result::Result<(), f64> {
        let form = state.form;
        let Self {
            mut running,
            mut across,
            mut tip,
            mut run,
            pull,
            mut curve,
        } = *self;
        let length = state.internode * phytomer.scale;
        let share = 1.0 - (-form.tropism * length).exp();
        let bent = toward_elevation(running, across, form.elevation, share);
        across = carried(across, running, bent);
        curve = carried(curve, running, bent);
        running = bent;
        if form.wander > 0.0 {
            let key = Key(phytomer.key).child(WANDER);
            let draw = form.wander * (2.0 * key.child(0).unit() - 1.0);
            let azimuth = TAU * key.child(1).unit();
            let pivot = across * azimuth.cos() + running.cross(across) * azimuth.sin();
            if form.bend_length > 0.0 {
                // The curvature relaxes over the bend length and is kicked
                // by the node's draw, its stationary spread the draw's; the
                // turn is the curvature over the internode (fn-207).
                let keep = (-length / form.bend_length).exp();
                curve = curve * keep + pivot * (draw * (1.0 - keep * keep).sqrt());
                curve = curve - running * curve.dot(running);
                let angle = curve.length() * length;
                if let Some(axis) = curve.unit() {
                    running = rotated(running, axis, angle);
                    across = rotated(across, axis, angle);
                }
            } else {
                // No memory: the node's own turn, as wander always drew it.
                let angle = form.wander * length * (2.0 * key.child(0).unit() - 1.0);
                running = rotated(running, pivot, angle);
                across = rotated(across, pivot, angle);
            }
        }
        // Sag: the beam's curvature, its moment over its stiffness, turns
        // the axis down about the torque's axis.
        if let (Some(load), true) = (load, form.sag > 0.0 && phytomer.radius > 0.0) {
            // The lever was taken on the tree before it bent. The load
            // beyond is turned rigidly by the rotation that takes the
            // phytomer's frame then (its heading and side, still the
            // first lay's) to its frame now, which its bearers' sag and
            // its own have turned: a whole frame, so no direction is
            // ambiguous. As the branch droops its lever turns towards the
            // vertical and its torque shrinks, so a heavy branch hangs and
            // stops (fn-203). Normalised with no cutoff: a vanishing
            // moment bends by nothing, never by a jump.
            let frames = ((phytomer.heading, phytomer.side), (running, across));
            let now = framed(load.moment, frames.0, frames.1);
            // The ground carries what of the wood beyond would rest on it.
            let carried = if trunk {
                1.0
            } else {
                held(tip.z, framed(load.chord, frames.0, frames.1))
            };
            let now = now * carried;
            let turning = torque(now);
            let moment = turning.length();
            if moment > 0.0 {
                let pivot = turning * (1.0 / moment);
                // The backstop: the room left to turn, the signed angle
                // about the pivot from the direction to straight down,
                // short of it by `HANG`. Wood already past straight down, curled under,
                // is not lifted: it has no room, and turns by nothing.
                // Measured from -pi / 2 to 3 pi / 2, so wood near straight
                // up has the room over the top to its load's side; past
                // straight up the room shrinks again, to none at the
                // angle's cut, so no side of the cut turns it.
                let square = running - pivot * running.dot(pivot);
                let room = square.cross(DOWN).dot(pivot).atan2(square.dot(DOWN));
                let room = if room < -FRAC_PI_2 { room + TAU } else { room };
                let room = if room > PI {
                    2.0 * (1.5 * PI - room)
                } else {
                    room
                };
                // The unbent lever's moment caps the turned one's.
                let cap = torque(load.moment).length() * carried;
                let angle =
                    turn(now, cap, form.sag, phytomer.radius, length).min((room - HANG).max(0.0));
                // Wood resting on the ground is carried by it and keeps
                // its direction there.
                let angle = if trunk {
                    angle
                } else {
                    angle * support(tip.z, phytomer.radius, length)
                };
                running = rotated(running, pivot, angle);
                across = rotated(across, pivot, angle);
                curve = rotated(curve, pivot, angle);
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
        // internodes so the turn is spread over steps, wood's descent
        // eases with the square root of its height, to none at the ground,
        // which is reached in a finite run; its lean is kept, never
        // normalised on its own, so a step near straight down stays near
        // straight down.
        if !trunk && direction.z < 0.0 {
            let ease = support(tip.z, phytomer.radius, length);
            if ease < 1.0 {
                if let Some(unit) = Vec3::new(direction.x, direction.y, direction.z * ease).unit() {
                    direction = unit;
                }
            }
        }
        let mut next = tip + direction * length;
        if next.z < -GROUND_TOLERANCE && trunk {
            return Err(-next.z);
        }
        // Resting: wood a step would take below the ground stays on it.
        if !trunk && next.z < 0.0 {
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
        *self = Self {
            running,
            across,
            tip,
            run,
            pull,
            curve,
        };
        Ok(())
    }
}

/// Lays the axis's internodes from its base frame, phytomer by
/// phytomer (`Layer::step`), with `load` (`sag.rs`) each one's lever;
/// where the walk ends.
pub(crate) fn lay(
    axis: &mut Axis,
    (frame, curve): ((Vec3, Vec3, Vec3), Vec3),
    state: &PaState,
    bend: (f64, f64),
    load: Option<&[Lever]>,
    trunk: bool,
) -> std::result::Result<Layer, f64> {
    let mut layer = Layer::bent(frame, curve);
    for (k, phytomer) in axis.phytomers.iter_mut().enumerate() {
        layer.step(phytomer, state, bend, load.map(|l| &l[k]), trunk)?;
    }
    (axis.base, axis.heading, axis.side) = frame;
    Ok(layer)
}

/// How far wood at height `z`, `radius` thick and `length` long, stands
/// free of the ground: 1 above its landing zone (a few radii and
/// `LANDING` internodes), easing with the square root of its height to
/// none on the ground.
pub(crate) fn support(z: f64, radius: f64, length: f64) -> f64 {
    let zone = CONTACT * radius + LANDING * length;
    if z >= zone {
        1.0
    } else {
        (z.max(0.0) / zone).sqrt()
    }
}

/// The key step of a node's wander draws.
const WANDER: u64 = 1;

/// `direction` turned in its vertical plane by `share` of its gap to
/// `elevation`. A vertical direction turns towards its side. Within
/// `DOWNWARD` of straight down the turn weakens with the direction's lean,
/// to nothing at straight down, as gravitropism's sine law has it: no
/// side can be chosen continuously for every lean in a cone (a lean's
/// direction cannot be blended into one carried side without a point
/// where they cancel), so a direction swept through straight down turns
/// by degree only if the turn vanishes there.
pub(super) fn toward_elevation(direction: Vec3, side: Vec3, elevation: f64, share: f64) -> Vec3 {
    if share <= 0.0 {
        return direction;
    }
    let level = |v: Vec3| Vec3::new(v.x, v.y, 0.0).unit();
    let lean = Vec3::new(direction.x, direction.y, 0.0).length();
    let cone = DOWNWARD.sin();
    let share = if direction.z < 0.0 && lean < cone {
        share * lean / cone
    } else {
        share
    };
    let Some(out) = level(direction).or_else(|| level(side)) else {
        return direction;
    };
    let now = direction.z.clamp(-1.0, 1.0).asin();
    let to = now + (elevation - now) * share;
    out * to.cos() + UP * to.sin()
}

/// `v` carried by the rotation that takes the frame of unit `heading` and
/// `side` to the frame of `heading2` and `side2` (each side made square to
/// its heading).
fn framed(v: Vec3, (heading, side): (Vec3, Vec3), (heading2, side2): (Vec3, Vec3)) -> Vec3 {
    let square = |h: Vec3, s: Vec3| (s - h * s.dot(h)).unit();
    let (Some(side), Some(side2)) = (square(heading, side), square(heading2, side2)) else {
        return v;
    };
    let (normal, normal2) = (heading.cross(side), heading2.cross(side2));
    heading2 * v.dot(heading) + side2 * v.dot(side) + normal2 * v.dot(normal)
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
pub(super) fn rotated(v: Vec3, pivot: Vec3, angle: f64) -> Vec3 {
    let (sin, cos) = angle.sin_cos();
    v * cos + pivot.cross(v) * sin + pivot * (pivot.dot(v) * (1.0 - cos))
}
