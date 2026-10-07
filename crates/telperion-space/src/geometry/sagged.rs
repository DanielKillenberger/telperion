//! A loaded phytomer's sag in the walk (contract A, CONTRACT-A.md 2.5):
//! its lever carried from its first lay's frame to its frame now by the
//! rotation the two frames make, normalised from W, its room unwrapped as
//! an amount, and its turn (`sag::turn`).
use super::lay::{support_q, HANG};
use super::q::{add, amount, cross, dot, q3, scaled, sub, unit, Q3};
use crate::fixed::feedback::angle::{atan2, between};
use crate::fixed::feedback::vector::unit_w;
use crate::fixed::feedback::wide::{mul_shr, I256};
use crate::fixed::{mul, q32, INF, ONE32};
use crate::sag::{held, log2_q, torque, turn, Lever, W3};
use crate::species::PaState;
use crate::structure::Phytomer;

/// The sag of a loaded phytomer (`sag.rs`): its pivot and amount, or
/// none (CONTRACT-A.md 2.5). The lever stays W throughout and is
/// normalised from W; its room is an amount, unwrapped exactly.
pub(super) fn sag_angle(
    load: &Lever,
    phytomer: &Phytomer,
    state: &PaState,
    (running, across, tip): (Q3, Q3, Q3),
    trunk: bool,
) -> Option<(Q3, i64)> {
    let form = state.form;
    let length = mul(q32(state.internode), q32(phytomer.scale));
    let radius = q32(phytomer.radius);
    // The lever was taken on the tree before it bent. The load beyond is
    // turned rigidly by the rotation that takes the phytomer's frame then
    // (its heading and side, still the first lay's) to its frame now
    // (fn-203).
    let frames = (
        Frame::of(q3(phytomer.heading), q3(phytomer.side))?,
        Frame::of(running, across)?,
    );
    let now = frames.0.carry_w(&frames.1, load.moment);
    // The ground carries what of the wood beyond would rest on it.
    let carried = if trunk {
        ONE32
    } else {
        held(tip[2], frames.0.carry(&frames.1, load.chord))
    };
    let now = now.map(|c| mul_shr(c, i128::from(carried), 32).expect("a lever inside W"));
    // Normalised from W with no cutoff: a vanishing moment bends by
    // nothing, never by a jump.
    let (pivot, moment) = unit_w(torque(now));
    if moment <= -INF {
        return None;
    }
    // The backstop: the room left to turn, short of straight down by
    // `HANG`; measured from -1/4 to 3/4 turn, so wood near straight up has
    // the room over the top to its load's side; past straight up it shrinks
    // again, to none at the angle's cut.
    let square = sub(running, scaled(pivot, dot(running, pivot)));
    let down = [0, 0, -ONE32];
    let room = between(0, atan2(dot(cross(square, down), pivot), dot(square, down)));
    let room = if room < -(1 << 30) { room + ONE32 } else { room };
    let room = if room > 1 << 31 { 2 * ((3 << 30) - room) } else { room };
    // The unbent lever's moment caps the turned one's.
    let cap = unit_w(torque(load.moment)).1 + log2_q(carried);
    let angle = turn(now, cap, form.sag, phytomer.radius, real(length));
    let angle = angle.min((room - amount(HANG)).max(0));
    // Wood resting on the ground is carried by it and keeps its direction
    // there.
    let angle = if trunk {
        angle
    } else {
        mul(angle, support_q(tip[2], radius, length))
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
        (0..3).fold([0; 3], |sum, k| add(sum, scaled(axes[k], parts[k])))
    }

    /// The same of a W vector, each product floored from 256 bits.
    fn carry_w(&self, to: &Frame, v: W3) -> W3 {
        let part = |axis: Q3| {
            let wide = (0..3)
                .map(|k| I256::mul(v[k], i128::from(axis[k])))
                .fold(I256::of(0), I256::add);
            wide.floor_shift(32).narrow().expect("a lever inside W")
        };
        let parts = [part(self.heading), part(self.side), part(self.normal)];
        let axes = [to.heading, to.side, to.normal];
        std::array::from_fn(|c| {
            (0..3)
                .map(|k| mul_shr(parts[k], i128::from(axes[k][c]), 32).expect("a lever inside W"))
                .sum()
        })
    }
}

/// A Q32 as its exact real.
fn real(q: i64) -> f64 {
    q as f64 / ONE32 as f64
}

