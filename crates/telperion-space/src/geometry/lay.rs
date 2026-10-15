//! Laying an axis's internodes, one phytomer at a time: the final lay
//! walks an axis from its base, and the rough layout grown with the tree
//! (fn-197) carries the same walk on as each growth unit grows.
use super::q::{self, add, amount, atan2, between, carry, cross, dot, q3, scaled, sincos, sub};
use super::q::{turned_by, turns, unit, v3, Q3};
use crate::contract::{exp2, fails};
use crate::lineage::{internode, signed, tropism, unit as uniform, Key, WANDER};
use crate::sag::Lever;
use crate::species::PaState;
use crate::structure::{Axis, Phytomer, Vec3};
use crate::transcendental::Transcendental;

/// The height, in the wood's own radii, below which it eases onto the
/// ground.
const CONTACT: f64 = 4.0;
/// The cone about straight down, in radians, within which tropism weakens
/// with an axis's lean.
const DOWNWARD: f64 = 0.05;
/// How far short of straight down sag leaves an axis, in radians.
pub(super) const HANG: f64 = 0.15;
/// The internodes the landing turn is spread over: at most a sixth of the
/// slope is eased in one step.
const LANDING: f64 = 6.0;
/// Wood this far below the ground plane is rounding, not wood below it.
pub(crate) const GROUND_TOLERANCE: f64 = 1e-9;
/// The positions a walk may reach, a coordinate below 2^12 m: far past any
/// tree, and a refusal by name past it (CONTRACT-A.md 2.2).
const REACH: f32 = 4096.0;

/// Where a walk along an axis stands: its running direction and side,
/// its tip, the scaled length it has run, how far its heading lets a bend
/// pull it up, and its wander's curvature (fn-207): a vector square to the
/// running direction, in radians per metre, the pivot its turns go about.
/// Every field is f32 and every step one walk, the rough layout's and the
/// final lay's alike (fn-254).
#[derive(Debug, Clone)]
pub(crate) struct Layer {
    running: Q3,
    across: Q3,
    tip: Q3,
    run: f32,
    pull: f32,
    curve: Q3,
    /// The axis's own tropism over its PA's, as its lineage draws it
    /// (fn-224; `lineage::tropism`).
    turning: f32,
    /// The pulls a relay carries on from the axis it replaces, as far as
    /// it is still that axis's continuation (fn-206).
    inherited: Vec<Lift>,
    /// A relay's take-over (fn-286): the line it runs back onto, a unit
    /// direction, and how far at its base, fading as its own pull does.
    /// None on every other axis.
    line: Q3,
    take: f32,
}

/// A pull of an axis's internodes towards the vertical: `bend` at `offset`
/// along the scaled length it fades over, to none at `total`, weakened by
/// `pull` as the heading it was taken from turns down.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Lift {
    bend: f32,
    pull: f32,
    total: f32,
    offset: f32,
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
    pub(crate) fn error(
        self,
        axis: usize,
        pa: usize,
        birth: u32,
        base: f64,
    ) -> crate::error::Error {
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
    /// lateral starts afresh from none. Its tropism is its PA's times
    /// `turning`, the axis's own draw (`lineage::tropism`).
    pub fn bent((base, heading, side): (Vec3, Vec3, Vec3), curve: Vec3, turning: f32) -> Self {
        let (heading, curve) = (q3(heading), q3(curve));
        Self {
            running: heading,
            across: q3(side),
            tip: q3(base),
            run: 0.0,
            pull: (1.0 + heading[2]) * 0.5,
            curve: sub(curve, scaled(heading, dot(curve, heading))),
            turning,
            inherited: Vec::new(),
            line: [0.0; 3],
            take: 0.0,
        }
    }

    /// The same, running back onto `line` by `take` at its base (`taken`).
    pub fn taking_over(mut self, (line, take): super::Take) -> Self {
        (self.line, self.take) = (line, take);
        self
    }

    /// Its take-over as the device holds it: how far, then the line.
    pub(crate) fn take_words(&self) -> [f32; 4] {
        [self.take, self.line[0], self.line[1], self.line[2]]
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
        let w = 1.0 - blend as f32;
        // A relay that has left the axis's line carries on none of it.
        if w <= 0.0 {
            return Vec::new();
        }
        let own = Lift {
            bend: bend as f32,
            pull: self.pull,
            total: total as f32,
            offset: 0.0,
        };
        std::iter::once(own)
            .chain(self.inherited.iter().copied())
            .map(|lift| Lift {
                bend: lift.bend * w,
                offset: lift.offset + self.run,
                ..lift
            })
            // A pull of none, or one already faded past its reach, pulls
            // nothing on.
            .filter(|lift| lift.bend > 0.0 && lift.offset < lift.total)
            .collect()
    }

    /// The wander's curvature where the walk stands.
    pub fn curve(&self) -> Vec3 {
        v3(self.curve)
    }

    /// Whether the walk's tip stands on the ground: where the resting
    /// clamp put it, or the run along the ground it began (fn-252 R2).
    pub fn rests(&self) -> bool {
        self.tip[2] <= 0.0
    }

    /// The scaled length laid so far.
    pub fn run(&self) -> f64 {
        f64::from(self.run)
    }

    /// The walk as the device holds it (fn-256): running, across, tip,
    /// run, pull, curve and turning, then each pull it carries on as its
    /// bend, pull, total and offset.
    pub(crate) fn words(&self) -> Vec<f32> {
        let mut out: Vec<f32> = [self.running, self.across, self.tip].concat();
        out.extend([self.run, self.pull]);
        out.extend(self.curve);
        out.push(self.turning);
        for l in &self.inherited {
            out.extend([l.bend, l.pull, l.total, l.offset]);
        }
        out
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
        let scale = phytomer.scale as f32;
        // Its internode varies node to node by its own draw (fn-222), and
        // its tropism axis to axis (fn-224), the plain product without
        // spread.
        let varied = internode(phytomer.key, form.internode_spread);
        let length = state.internode as f32 * scale * varied;
        let tropic = form.tropism as f32 * self.turning * length;
        let share = falloff(tropic);
        let bent = toward_elevation(running, across, amount(form.elevation), share);
        // Carried by the least rotation from the old running direction to
        // the bent one, built from the two vectors (no angle).
        if let Some(t) = carry(running, bent) {
            across = turned_by(across, &t);
            curve = turned_by(curve, &t);
        }
        running = bent;
        // The node's wander: a turn of keyed angle about a keyed pivot,
        // none without wander.
        let kick = (form.wander > 0.0).then(|| {
            let key = Key(phytomer.key).child(WANDER);
            // 2u - 1 of the draw's uniform, and its azimuth in turns.
            let signed = signed(key.child(0).draw());
            let draw = form.wander as f32 * signed;
            let (cos, sin) = sincos(uniform(key.child(1).draw()));
            let pivot = add(scaled(across, cos), scaled(cross(running, across), sin));
            (signed, draw, pivot)
        });
        if form.bend_length > 0.0 {
            // The curvature relaxes over the bend length and is kicked by
            // the node's draw, its stationary spread the draw's; the turn
            // is the curvature over the internode (fn-207). An inherited
            // curvature relaxes on without wander.
            let keep = falloff_keep(length / form.bend_length as f32);
            curve = scaled(curve, keep);
            if let Some((_, draw, pivot)) = kick {
                curve = add(curve, scaled(pivot, draw * rest(keep)));
            }
            curve = sub(curve, scaled(running, dot(curve, running)));
            let angle = turns(q::length(curve) * length);
            if let Some(axis) = unit(curve) {
                let t = q::turn(axis, angle);
                running = turned_by(running, &t);
                across = turned_by(across, &t);
            }
        } else if let Some((signed, draw, pivot)) = kick {
            // No memory: the node's own turn, as wander always drew it,
            // and the curvature it leaves a continuation is that turn's.
            let angle = turns(form.wander as f32 * length * signed);
            let t = q::turn(pivot, angle);
            running = turned_by(running, &t);
            across = turned_by(across, &t);
            curve = scaled(pivot, draw);
        } else {
            curve = [0.0; 3];
        }
        // Sag: the beam's curvature, its moment over its stiffness, turns
        // the axis down about the torque's axis (`sag_angle`).
        if let (Some(load), true) = (load, form.sag > 0.0 && phytomer.radius > 0.0) {
            let at = (running, across, tip);
            if let Some((pivot, angle)) =
                super::sagged::sag_angle(load, (phytomer, length), state, at, trunk)
            {
                let t = q::turn(pivot, angle);
                running = turned_by(running, &t);
                across = turned_by(across, &t);
                curve = turned_by(curve, &t);
            }
        }
        // Its own pull, and those it carries on from the axis it relays,
        // each fading over its reach (fn-206).
        let own = Lift {
            bend: bend as f32,
            pull,
            total: total as f32,
            offset: 0.0,
        };
        let (mut lift, mut up, mut fade) = (0.0f32, 0.0f32, 0.0f32);
        for (k, l) in std::iter::once(&own).chain(&self.inherited).enumerate() {
            let along = if l.total > 0.0 {
                (l.offset + run + scale * 0.5) / l.total
            } else {
                1.0
            };
            let left = (1.0 - along).max(0.0);
            if k == 0 {
                fade = left;
            }
            let here = l.bend * left;
            lift += here;
            up += here * l.pull;
        }
        run += scale;
        // A relay's take-over runs it back onto its line, fading as its
        // own pull does, the pulls above sharing what it leaves (fn-286).
        let over = self.take * fade;
        let blend = if over > 0.0 {
            let keep = 1.0 - over;
            let pulled = add(scaled(running, (1.0 - lift) * keep), [0.0, 0.0, up * keep]);
            add(pulled, scaled(self.line, over))
        } else {
            add(scaled(running, 1.0 - lift), [0.0, 0.0, up])
        };
        let mut direction = match unit(blend) {
            Some(unit) if lift > 0.0 || over > 0.0 => unit,
            _ => running,
        };
        // Landing: within a few radii of the ground, and `LANDING`
        // internodes so the turn is spread over steps, wood's descent
        // eases with the square root of its height, to none at the ground,
        // which is reached in a finite run; its lean is kept, never
        // normalised on its own, so a step near straight down stays near
        // straight down.
        let radius = phytomer.radius as f32;
        if !trunk && direction[2] < 0.0 {
            let ease = support(tip[2], radius, length);
            if ease < 1.0 {
                if let Some(unit) = unit([direction[0], direction[1], direction[2] * ease]) {
                    direction = unit;
                }
            }
        }
        let mut next = add(tip, scaled(direction, length));
        if next.iter().any(|c| c.abs() >= REACH) {
            return Err(Stop::Beyond);
        }
        if f64::from(next[2]) < -GROUND_TOLERANCE && trunk {
            return Err(Stop::Below(-f64::from(next[2])));
        }
        // Resting: wood a step would take below the ground stays on it.
        if !trunk && next[2] < 0.0 {
            next[2] = 0.0;
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

/// `1 - e^-x` (natural): the contract's `fails` of its log2.
fn falloff(x: f32) -> f32 {
    fails(x * std::f32::consts::LOG2_E)
}

/// `e^-x`.
fn falloff_keep(x: f32) -> f32 {
    exp2(-x * std::f32::consts::LOG2_E)
}

/// `sqrt(1 - k^2)` of a `k` in [0, 1].
fn rest(k: f32) -> f32 {
    let k = k.clamp(0.0, 1.0);
    (1.0 - k * k).sqrt()
}

/// Lays the axis's internodes from its base frame, phytomer by
/// phytomer (`Layer::step`), with `load` (`sag.rs`) each one's lever;
/// where the walk ends.
pub(crate) fn lay(
    axis: &mut Axis,
    (frame, curve, inherited, take): ((Vec3, Vec3, Vec3), Vec3, Vec<Lift>, super::Take),
    state: &PaState,
    bend: (f64, f64),
    load: Option<&[Lever]>,
    trunk: bool,
) -> std::result::Result<Layer, Stop> {
    let turning = tropism(axis.lineage, state.form.tropism_spread);
    let mut layer = Layer::bent(frame, curve, turning).inheriting(inherited).taking_over(take);
    for (k, phytomer) in axis.phytomers.iter_mut().enumerate() {
        layer.step(phytomer, state, bend, load.map(|l| &l[k]), trunk)?;
    }
    (axis.base, axis.heading, axis.side) = frame;
    Ok(layer)
}

/// How far wood at height `z`, `radius` thick and `length` long, stands
/// free of the ground: one above its landing zone (a few radii and
/// `LANDING` internodes), easing with the square root of its height to
/// none on the ground.
pub(crate) fn support(z: f32, radius: f32, length: f32) -> f32 {
    let zone = CONTACT as f32 * radius + LANDING as f32 * length;
    if z >= zone {
        return 1.0;
    }
    (z.max(0.0) / zone).sqrt()
}

/// The cone about straight down as the sine its lean is held to: a
/// per-request constant a second executor is handed.
pub(crate) fn cone() -> f32 {
    DOWNWARD.sin_fixed() as f32
}

/// `direction` turned in its vertical plane by `share` of its gap
/// to the amount `elevation`. A vertical direction turns towards its side.
/// Within `DOWNWARD` of straight down the turn weakens with the
/// direction's lean, to nothing at straight down, as gravitropism's sine
/// law has it: no side can be chosen continuously for every lean in a
/// cone, so a direction swept through straight down turns by degree only
/// if the turn vanishes there.
pub(super) fn toward_elevation(direction: Q3, side: Q3, elevation: f32, share: f32) -> Q3 {
    if share <= 0.0 {
        return direction;
    }
    let level = |v: Q3| unit([v[0], v[1], 0.0]);
    let lean = q::length([direction[0], direction[1], 0.0]);
    let cone = cone();
    let share = if direction[2] < 0.0 && lean < cone {
        share * lean / cone
    } else {
        share
    };
    let Some(out) = level(direction).or_else(|| level(side)) else {
        return direction;
    };
    // Its elevation from its own height and lean (host decision,
    // 2026-10-07: the lean already taken, no second root).
    let now = between(0.0, atan2(direction[2], lean));
    let to = now + (elevation - now) * share;
    let (cos, sin) = sincos(to);
    add(scaled(out, cos), scaled(q::UP, sin))
}
