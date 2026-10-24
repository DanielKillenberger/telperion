//! A loaded phytomer's sag in the walk (CONTRACT-A.md 2.5, in f32 since
//! fn-254): its lever carried from its first lay's frame to its frame now
//! by the rotation the two frames make, its room unwrapped as an amount,
//! and its turn (`sag::turn`).
use super::lay::{support, HANG};
use super::q::{add, amount, atan2, between, cross, dot, q3, scaled, sub, unit, Q3};
use crate::contract::INF;
use crate::sag::{held, log2_q, torque, turn, unit_w, Lever};
use crate::species::PaState;
use crate::structure::Phytomer;

/// The sag of a loaded phytomer `length` metres long as it is laid, its
/// node's own internode draw taken (`sag.rs`): its pivot and amount, or
/// none (CONTRACT-A.md 2.5); its room is an amount, unwrapped.
pub(super) fn sag_angle(
    load: &Lever,
    (phytomer, length): (&Phytomer, f32),
    state: &PaState,
    (running, across, tip): (Q3, Q3, Q3),
    trunk: bool,
) -> Option<(Q3, f32)> {
    let form = state.form;
    let radius = phytomer.radius as f32;
    // The lever was taken on the tree before it bent. The load beyond is
    // turned rigidly by the rotation that takes the phytomer's frame then
    // (its heading and side, still the first lay's) to its frame now
    // (fn-203).
    // Where either frame has no side (a phytomer never laid, as a chain
    // resolved alone meets it), the lever is not carried: as before.
    let frames = Frame::of(q3(phytomer.heading), q3(phytomer.side)).zip(Frame::of(running, across));
    let now = frames
        .as_ref()
        .map_or(load.moment, |(from, to)| from.carry(to, load.moment));
    let chord = frames
        .as_ref()
        .map_or(load.chord, |(from, to)| from.carry(to, load.chord));
    // The ground carries what of the wood beyond would rest on it.
    let carried = if trunk { 1.0 } else { held(tip[2], chord) };
    let now = scaled(now, carried);
    // Normalised with no cutoff: a vanishing moment bends by nothing,
    // never by a jump.
    let (pivot, moment) = unit_w(torque(now));
    if moment <= -INF {
        return None;
    }
    // The backstop: the room left to turn, short of straight down by
    // `HANG`; measured from -1/4 to 3/4 turn, so wood near straight up has
    // the room over the top to its load's side; past straight up it shrinks
    // again, to none at the angle's cut.
    let square = sub(running, scaled(pivot, dot(running, pivot)));
    let down = [0.0, 0.0, -1.0];
    let room = between(
        0.0,
        atan2(dot(cross(square, down), pivot), dot(square, down)),
    );
    let room = if room < -0.25 { room + 1.0 } else { room };
    let room = if room > 0.5 {
        2.0 * (0.75 - room)
    } else {
        room
    };
    // The unbent lever's moment caps the turned one's.
    let cap = unit_w(torque(load.moment)).1 + log2_q(carried);
    let angle = turn(now, cap, form.sag, phytomer.radius, f64::from(length));
    let angle = angle.min((room - amount(HANG)).max(0.0));
    // Wood resting on the ground is carried by it and keeps its direction
    // there.
    let angle = if trunk {
        angle
    } else {
        angle * support(tip[2], radius, length)
    };
    Some((pivot, angle))
}

/// A frame: a unit heading, its side made square and unit, and their
/// normal; carrying a vector from one frame to another is the rotation
/// built from the two frames (no angle).
struct Frame {
    heading: Q3,
    side: Q3,
    normal: Q3,
}

impl Frame {
    fn of(heading: Q3, side: Q3) -> Option<Self> {
        let side = unit(sub(side, scaled(heading, dot(side, heading))))?;
        Some(Self {
            heading,
            side,
            normal: cross(heading, side),
        })
    }

    /// The components of `v` in this frame, carried into `to`.
    fn carry(&self, to: &Frame, v: Q3) -> Q3 {
        let parts = [dot(v, self.heading), dot(v, self.side), dot(v, self.normal)];
        let axes = [to.heading, to.side, to.normal];
        (0..3).fold([0.0; 3], |sum, k| add(sum, scaled(axes[k], parts[k])))
    }
}
