//! Laying an axis's internodes, one phytomer at a time: the final lay
//! walks an axis from its base, and the rough layout grown with the tree
//! (fn-197) carries the same walk on as each growth unit grows.
use super::q::{self, add, amount, cross, dot, q3, rotated, scaled, sub, unit, v3, Q3};
use crate::fixed::feedback::angle::{asin, between, sincos, turns, wrap};
use crate::fixed::feedback::lattice::REACH;
use crate::fixed::feedback::vector::isqrt;
use crate::fixed::{exp2_neg, mul, q32, LOG2E, ONE32, P_ONE};
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
/// The same in Q32, floored (CONTRACT-A.md 2.4).
const GROUND_TOLERANCE_Q: i64 = 4;
const DOWN: Vec3 = Vec3::new(0.0, 0.0, -1.0);

/// Where a walk along an axis stands: its running direction and side,
/// its tip, the scaled length it has run, how far its heading lets a bend
/// pull it up, and its wander's curvature (fn-207): a vector square to the
/// running direction, in radians per metre, the pivot its turns go about.
/// Contract A (CONTRACT-A.md 2.4): every field is Q32 and every step one
/// fixed-point walk, the rough layout's and the final lay's alike.
#[derive(Debug, Clone)]
pub(crate) struct Layer {
    running: Q3,
    across: Q3,
    tip: Q3,
    run: i64,
    pull: i64,
    curve: Q3,
    /// The pulls a relay carries on from the axis it replaces, as far as
    /// it is still that axis's continuation (fn-206).
    inherited: Vec<Lift>,
}

/// A pull of an axis's internodes towards the vertical: `bend` at `offset`
/// along the scaled length it fades over, to none at `total`, weakened by
/// `pull` as the heading it was taken from turns down. Q32.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Lift {
    bend: i64,
    pull: i64,
    total: i64,
    offset: i64,
}

/// Why a walk stopped: the trunk would reach below the ground by this
/// much, or a point would leave the contract's domain.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Stop {
    Below(f64),
    Beyond,
}

impl Stop {
    /// The error it is for axis `axis` (its PA, birth and base height).
    pub(crate) fn error(self, axis: usize, pa: usize, birth: u32, base: f64) -> crate::error::Error {
        match self {
            Stop::Below(height) => crate::error::Error::BelowGround {
                axis,
                pa,
                birth,
                base,
                height,
            },
            Stop::Beyond => crate::error::Error::Refused {
                input: format!("axes[{axis}]"),
                reason: "a position beyond the contract's 4,096 m",
            },
        }
    }
}

impl Layer {
    /// At the base of an axis framed `(base, heading, side)` carrying on
    /// `curve`, its bearer's curvature, made square to its heading: a
    /// continuation or relay keeps bending as its bearer did, and a
    /// lateral starts afresh from none.
    pub fn bent((base, heading, side): (Vec3, Vec3, Vec3), curve: Vec3) -> Self {
        let (heading, curve) = (q3(heading), q3(curve));
        Self {
            running: heading,
            across: q3(side),
            tip: q3(base),
            run: 0,
            pull: (ONE32 + heading[2]) >> 1,
            curve: sub(curve, scaled(heading, dot(curve, heading))),
            inherited: Vec::new(),
        }
    }

    /// The same, carrying on the pulls `inherited` from the axis it
    /// relays (`handed`).
    pub fn inheriting(mut self, inherited: Vec<Lift>) -> Self {
        self.inherited = inherited;
        self
    }

    /// The running direction and side the walk stands on, before its pull
    /// towards the vertical: what a continuation or relay carries on.
    pub fn ends(&self) -> (Vec3, Vec3) {
        (v3(self.running), v3(self.across))
    }

    /// The pulls a relay of this axis carries on, `blend` of the way from
    /// its continuation to its own bud: the axis's own at `bend` over its
    /// reach `total`, and those it carries, each as far as the relay is
    /// still the continuation and on from the axis's scaled length.
    pub fn handed(&self, (bend, total): (f64, f64), blend: f64) -> Vec<Lift> {
        let w = ONE32 - q32(blend);
        // A relay that has left the axis's line carries on none of it.
        if w <= 0 {
            return Vec::new();
        }
        let own = Lift {
            bend: q32(bend),
            pull: self.pull,
            total: q32(total),
            offset: 0,
        };
        std::iter::once(own)
            .chain(self.inherited.iter().copied())
            .map(|lift| Lift {
                bend: mul(lift.bend, w),
                offset: lift.offset + self.run,
                ..lift
            })
            // A pull of none, or one already faded past its reach, pulls
            // nothing on.
            .filter(|lift| lift.bend > 0 && lift.offset < lift.total)
            .collect()
    }

    /// The wander's curvature where the walk stands.
    pub fn curve(&self) -> Vec3 {
        v3(self.curve)
    }

    /// The scaled length laid so far.
    pub fn run(&self) -> f64 {
        self.run as f64 / ONE32 as f64
    }

    /// Lays the next phytomer. A running direction bends towards the PA's
    /// elevation and wanders by the node's keyed turn, both in proportion
    /// to the internode's length; the phytomer's direction is that running
    /// direction pulled towards the vertical by `bend` at the base, fading
    /// to none at `total`, the axis's scaled reach. With `load`, it turns
    /// down by its sag first.
    pub fn step(
        &mut self,
        phytomer: &mut Phytomer,
        state: &PaState,
        (bend, total): (f64, f64),
        load: Option<&Lever>,
        trunk: bool,
    ) -> std::result::Result<(), Stop> {
        let form = state.form;
        let (mut running, mut across, tip, mut run, pull, mut curve) = (
            self.running,
            self.across,
            self.tip,
            self.run,
            self.pull,
            self.curve,
        );
        let scale = q32(phytomer.scale);
        let length = mul(q32(state.internode), scale);
        let share = falloff(mul(q32(form.tropism), length));
        let bent = toward_elevation(running, across, amount(form.elevation), share);
        across = carried(across, running, bent);
        curve = carried(curve, running, bent);
        running = bent;
        // The node's wander: a turn of keyed angle about a keyed pivot,
        // none without wander.
        let kick = (form.wander > 0.0).then(|| {
            let key = Key(phytomer.key).child(WANDER);
            // 2u - 1 of the draw's top 32 bits, and its azimuth as a T.
            let signed = ((key.child(0).draw().0 >> 21) as i64) * 2 - ONE32;
            let draw = mul(q32(form.wander), signed);
            let (cos, sin) = sincos((key.child(1).draw().0 >> 21) as u32);
            let pivot = add(scaled(across, cos), scaled(cross(running, across), sin));
            (signed, draw, pivot)
        });
        if form.bend_length > 0.0 {
            // The curvature relaxes over the bend length and is kicked by
            // the node's draw, its stationary spread the draw's; the turn
            // is the curvature over the internode (fn-207). An inherited
            // curvature relaxes on without wander.
            let keep = falloff_keep(over(length, q32(form.bend_length)));
            curve = scaled(curve, keep);
            if let Some((_, draw, pivot)) = kick {
                curve = add(curve, scaled(pivot, mul(draw, rest(keep))));
            }
            curve = sub(curve, scaled(running, dot(curve, running)));
            let angle = turns(mul(q::length(curve), length));
            if let Some(axis) = unit(curve) {
                running = rotated(running, axis, angle);
                across = rotated(across, axis, angle);
            }
        } else if let Some((signed, draw, pivot)) = kick {
            // No memory: the node's own turn, as wander always drew it,
            // and the curvature it leaves a continuation is that turn's.
            let angle = turns(mul(mul(q32(form.wander), length), signed));
            running = rotated(running, pivot, angle);
            across = rotated(across, pivot, angle);
            curve = scaled(pivot, draw);
        } else {
            curve = [0; 3];
        }
        // Sag: the beam's curvature, its moment over its stiffness, turns
        // the axis down about the torque's axis (`sag_angle`).
        if let (Some(load), true) = (load, form.sag > 0.0 && phytomer.radius > 0.0) {
            if let Some((pivot, angle)) = sag_angle(load, phytomer, state, (running, across, tip), trunk) {
                running = rotated(running, pivot, angle);
                across = rotated(across, pivot, angle);
                curve = rotated(curve, pivot, angle);
            }
        }
        // Its own pull, and those it carries on from the axis it relays,
        // each fading over its reach (fn-206).
        let own = Lift {
            bend: q32(bend),
            pull,
            total: q32(total),
            offset: 0,
        };
        let (mut lift, mut up) = (0i64, 0i64);
        for l in std::iter::once(&own).chain(&self.inherited) {
            let along = if l.total > 0 {
                over(l.offset + run + scale / 2, l.total)
            } else {
                ONE32
            };
            let here = mul(l.bend, (ONE32 - along).max(0));
            lift += here;
            up += mul(here, l.pull);
        }
        run += scale;
        let blend = add(scaled(running, ONE32 - lift), [0, 0, up]);
        let mut direction = match unit(blend) {
            Some(unit) if lift > 0 => unit,
            _ => running,
        };
        // Landing: within a few radii of the ground, and `LANDING`
        // internodes so the turn is spread over steps, wood's descent
        // eases with the square root of its height, to none at the ground,
        // which is reached in a finite run; its lean is kept, never
        // normalised on its own, so a step near straight down stays near
        // straight down.
        let radius = q32(phytomer.radius);
        if !trunk && direction[2] < 0 {
            let ease = support_q(tip[2], radius, length);
            if ease < ONE32 {
                if let Some(unit) = unit([direction[0], direction[1], mul(direction[2], ease)]) {
                    direction = unit;
                }
            }
        }
        let mut next = add(tip, scaled(direction, length));
        if next.iter().any(|c| c.unsigned_abs() >= REACH as u64) {
            return Err(Stop::Beyond);
        }
        if next[2] < -GROUND_TOLERANCE_Q && trunk {
            return Err(Stop::Below(-(next[2] as f64) / ONE32 as f64));
        }
        // Resting: wood a step would take below the ground stays on it.
        if !trunk && next[2] < 0 {
            next[2] = 0;
            if let Some(unit) = unit(sub(next, tip)) {
                direction = unit;
            }
        }
        phytomer.tip = v3(next);
        phytomer.heading = v3(direction);
        let side = sub(across, scaled(direction, dot(across, direction)));
        phytomer.side = v3(unit(side).unwrap_or(across));
        (self.running, self.across, self.tip, self.run, self.curve) =
            (running, across, next, run, curve);
        Ok(())
    }
}

/// `trunc(a 2^32 / b)` of Q32 values, Q32.
fn over(a: i64, b: i64) -> i64 {
    ((i128::from(a) << 32) / i128::from(b)) as i64
}

/// `1 - e^-x` of a Q32 `x` (natural), Q32: `P_ONE - 2^-(x log2 e)` in
/// Q62, then floored to Q32.
fn falloff(x: i64) -> i64 {
    ((P_ONE - exp2_neg(((i128::from(x) * LOG2E) >> 62) as i64)) >> 30) as i64
}

/// `e^-x` of a Q32 `x`, Q32, floored.
fn falloff_keep(x: i64) -> i64 {
    (exp2_neg(((i128::from(x) * LOG2E) >> 62) as i64) >> 30) as i64
}

/// `sqrt(1 - k^2)` of a Q32 `k` in [0, 1], Q32, floored.
fn rest(k: i64) -> i64 {
    let k = u128::from(k.clamp(0, ONE32).unsigned_abs());
    isqrt((1u128 << 64) - k * k) as i64
}

/// The sag of a loaded phytomer (`sag.rs`): its pivot and amount, or
/// none. The lever's arithmetic is the re-lay and sag pass's
/// (CONTRACT-A.md 2.5); here its angle enters as an amount, rounded once.
fn sag_angle(
    load: &Lever,
    phytomer: &Phytomer,
    state: &PaState,
    (running, across, tip): (Q3, Q3, Q3),
    trunk: bool,
) -> Option<(Q3, i64)> {
    let form = state.form;
    let length = state.internode * phytomer.scale;
    let (running, across, tip) = (v3(running), v3(across), v3(tip));
    // The lever was taken on the tree before it bent. The load beyond is
    // turned rigidly by the rotation that takes the phytomer's frame then
    // (its heading and side, still the first lay's) to its frame now
    // (fn-203). Normalised with no cutoff: a vanishing moment bends by
    // nothing, never by a jump.
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
    if moment <= 0.0 {
        return None;
    }
    let pivot = turning * (1.0 / moment);
    // The backstop: the room left to turn, short of straight down by
    // `HANG`; measured from -pi / 2 to 3 pi / 2, so wood near straight up
    // has the room over the top to its load's side.
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
    let angle = turn(now, cap, form.sag, phytomer.radius, length).min((room - HANG).max(0.0));
    // Wood resting on the ground is carried by it and keeps its direction
    // there.
    let angle = if trunk {
        angle
    } else {
        angle * support(tip.z, phytomer.radius, length)
    };
    let pivot = unit(q3(pivot))?;
    Some((pivot, amount(angle)))
}

/// Lays the axis's internodes from its base frame, phytomer by
/// phytomer (`Layer::step`), with `load` (`sag.rs`) each one's lever;
/// where the walk ends.
pub(crate) fn lay(
    axis: &mut Axis,
    (frame, curve, inherited): ((Vec3, Vec3, Vec3), Vec3, Vec<Lift>),
    state: &PaState,
    bend: (f64, f64),
    load: Option<&[Lever]>,
    trunk: bool,
) -> std::result::Result<Layer, Stop> {
    let mut layer = Layer::bent(frame, curve).inheriting(inherited);
    for (k, phytomer) in axis.phytomers.iter_mut().enumerate() {
        layer.step(phytomer, state, bend, load.map(|l| &l[k]), trunk)?;
    }
    (axis.base, axis.heading, axis.side) = frame;
    Ok(layer)
}

/// The height below which wood `radius` thick and `length` long feels
/// the ground: its landing zone.
pub(crate) fn ground_zone(radius: f64, length: f64) -> f64 {
    CONTACT * radius + LANDING * length
}

/// `support` in Q32: `isqrt` of the Q32 quotient `max(z, 0) / zone`
/// shifted to Q64 (CONTRACT-A.md 2.4), one above the zone.
fn support_q(z: i64, radius: i64, length: i64) -> i64 {
    let zone = CONTACT as i64 * radius + LANDING as i64 * length;
    if z >= zone {
        return ONE32;
    }
    isqrt((over(z.max(0), zone) as u128) << 32) as i64
}

/// How far wood at height `z`, `radius` thick and `length` long, stands
/// free of the ground: 1 above its landing zone (a few radii and
/// `LANDING` internodes), easing with the square root of its height to
/// none on the ground.
pub(crate) fn support(z: f64, radius: f64, length: f64) -> f64 {
    support_q(q32(z), q32(radius), q32(length)) as f64 / ONE32 as f64
}

/// The key step of a node's wander draws.
const WANDER: u64 = 1;

/// `direction` turned in its vertical plane by `share` (Q32) of its gap
/// to the amount `elevation`. A vertical direction turns towards its side.
/// Within `DOWNWARD` of straight down the turn weakens with the
/// direction's lean, to nothing at straight down, as gravitropism's sine
/// law has it: no side can be chosen continuously for every lean in a
/// cone, so a direction swept through straight down turns by degree only
/// if the turn vanishes there.
pub(super) fn toward_elevation(direction: Q3, side: Q3, elevation: i64, share: i64) -> Q3 {
    if share <= 0 {
        return direction;
    }
    let level = |v: Q3| unit([v[0], v[1], 0]);
    let lean = q::length([direction[0], direction[1], 0]);
    let cone = q32(DOWNWARD.sin());
    let share = if direction[2] < 0 && lean < cone {
        ((i128::from(share) * i128::from(lean)) / i128::from(cone)) as i64
    } else {
        share
    };
    let Some(out) = level(direction).or_else(|| level(side)) else {
        return direction;
    };
    let now = between(0, asin(direction[2]));
    let to = now + ((i128::from(elevation - now) * i128::from(share)) >> 32) as i64;
    let (cos, sin) = sincos(wrap(to));
    add(scaled(out, cos), scaled(q::UP, sin))
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

/// `v` carried by the least rotation that takes unit `from` to unit `to`:
/// about `unit(from × to)` by `atan2(|from × to|, from · to)`.
fn carried(v: Q3, from: Q3, to: Q3) -> Q3 {
    let axis = cross(from, to);
    match unit(axis) {
        Some(pivot) => {
            let angle = crate::fixed::feedback::angle::atan2(q::length(axis), dot(from, to));
            rotated(v, pivot, between(0, angle))
        }
        None => v,
    }
}
