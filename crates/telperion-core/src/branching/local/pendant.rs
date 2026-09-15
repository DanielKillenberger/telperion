//! The curtain a hanging shoot grows: how far it droops, how long it runs,
//! how close to the floor beneath it it may reach, and how far apart its
//! neighbours stand.
//!
//! A shoot's own weight bends it as it runs: the sag row turns its course
//! toward straight down along the run, from the departure the droop gave it.
//!
//! Every magnitude here is a twig row scaled by `hang`. At hang 0 a shoot
//! carries no curtain at all and the local law reaches none of this; at hang 1
//! the droop is the constant the hidden mode had, and above 1 a shoot hangs
//! steeper than that mode ever allowed, to three times its droop;
//! the droop is the one the curtain had while it was a constant, so a table
//! that states the rows reproduces the tree the constants grew.
use super::*;

/// The droop one shoot may take at full hang, and the rate it reaches that cap
/// at over its own pendulous length. The hang row scales both, so a walk of
/// the row carries the curtain down to nothing rather than switching it off.
const DROOP_CAP: f64 = 0.35;
const DROOP_SLOPE: f64 = 0.5;
/// The share of its clearance above the floor one step of a pendulous shoot
/// may spend. Never zero, so a curtain stops above its floor rather than
/// stacking shoots on it.
const CLEARANCE: f64 = 0.8;

/// One shoot's curtain: how strongly it hangs, the bearing its laterals spread
/// along, and the height its first descending ancestor's tip set as a floor.
#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub(super) struct Curtain {
    hang: f64,
    across: Vec3,
    floor: Option<f64>,
}

impl Curtain {
    /// A station's shoots hang when the hang row is set and the wood is thin
    /// enough; the floor is only asked for where they do.
    pub fn hangs_at(t: TwigParams, radius: f64, root_radius: f64) -> bool {
        t.hang > 0.0 && radius <= t.pendulous_radius * root_radius
    }

    /// The curtain a station seeds. Without a descending ancestor there is no
    /// floor and nothing hangs, whatever the row says.
    ///
    /// `tip` is the height the first descending ancestor ends at, which is
    /// where a shoot held out by its own wood comes to rest. A shoot that
    /// gives in to its weight falls past it, so the sag row carries the floor
    /// down from that tip to `base`, the height the crown's own room stops at.
    pub fn new(t: TwigParams, at: Vec3, tip: Option<f64>, base: f64) -> Self {
        let hang = if tip.is_some() { t.hang } else { 0.0 };
        Self {
            hang,
            across: Vec3::new(-at.z, 0.0, at.x).normalized(),
            floor: tip.map(|tip| walk(tip, base, t.sag * hang.min(1.0))),
        }
    }

    pub fn hangs(self) -> bool {
        self.hang > 0.0
    }

    /// Whether a candidate has passed below the floor the curtain may not
    /// cross.
    pub fn below(self, y: f64) -> bool {
        self.floor.is_some_and(|floor| y < floor)
    }

    /// The most of `length` a step descending at `descent` may take without
    /// passing the floor, keeping a share of the clearance in hand.
    pub fn clear(self, y: f64, descent: f64, length: f64) -> f64 {
        self.floor.map_or(length, |floor| {
            length.min((y - floor).max(0.0) / descent.max(1e-9) * CLEARANCE)
        })
    }

    /// A pendulous shoot stops at `pendulous_length`; the hang row walks that
    /// cap in from the length the branch law asked for. Under a sag the
    /// pendulous length is what the shoot runs rather than a cap on what the
    /// branch law allows: the allometry that gives a self-supporting limb its
    /// length by its own thickness has nothing to say about a strand hanging
    /// from one, and the hang row then walks that run in from the law's own
    /// length rather than extrapolating past it.
    pub fn length(self, length: f64, t: TwigParams) -> f64 {
        if t.sag == 0.0 {
            return walk(length, length.min(t.pendulous_length), self.hang);
        }
        let hung = walk(length.min(t.pendulous_length), t.pendulous_length, t.sag);
        walk(length, hung, self.hang.min(1.0))
    }

    /// Neighbouring shoots in a curtain stand `curtain_separation` degrees
    /// apart; the hang row walks that in from the twig law's own separation.
    /// Both are cosines of a half-angle, as the caller compares them.
    pub fn separation(self, ordinary: f64, t: TwigParams) -> f64 {
        walk(
            ordinary,
            t.curtain_separation.to_radians().cos_fixed(),
            self.hang,
        )
    }

    /// Where one lateral of a curtain departs: across the crown to alternating
    /// sides and down by a droop that grows with the clearance below it. The
    /// hang row walks the whole departure in from the one the twig law asked
    /// for, so no value of the row is the frame where the shoot changes kind.
    pub fn direction(self, upright: Vec3, y: f64, side: f64, t: TwigParams) -> Vec3 {
        let droop = self
            .floor
            .map_or(DROOP_CAP, |floor| {
                (y - floor) / t.pendulous_length * DROOP_SLOPE
            })
            .clamp(0.0, DROOP_CAP)
            * self.hang;
        let hung = (self.across * side - Vec3::Y * droop).normalized();
        if self.hang >= 1.0 {
            hung
        } else {
            walk3(upright, hung, self.hang).normalized()
        }
    }

    /// Whether the shoots of this curtain give in to their own weight at all.
    /// A curtain nobody hangs, and a table that states no sag, do not.
    pub fn sags(self, t: TwigParams) -> bool {
        self.hangs() && t.sag > 0.0
    }

    /// Where a hanging shoot goes `travelled` into its run: the course the
    /// branch law holds, turned toward straight down by the share of its own
    /// angle the sag row has spent by then. The turn is about the axis the
    /// course and the down vector span, so the shoot bends in its own plane
    /// and keeps the bearing about the trunk the droop gave it. Weight is not
    /// a turn the tip steers, so the law's own turn limit does not bound it;
    /// a course this does not bend is returned as it came.
    pub fn sagged(self, course: Vec3, travelled: f64, t: TwigParams) -> Vec3 {
        if !self.sags(t) {
            return course;
        }
        let unit = course.normalized();
        let cosine = (-unit.y).clamp(-1.0, 1.0);
        let turn = cosine.acos_fixed() * (1.0 - remaining(t, travelled));
        if turn <= 1e-12 {
            return course;
        }
        let toward = -Vec3::Y - unit * cosine;
        let toward = if toward.length_squared() > 1e-18 {
            toward.normalized()
        } else {
            unit.perpendicular()
        };
        let (sine, cosine) = turn.sin_cos_fixed();
        (unit * cosine + toward * sine).normalized()
    }
}

/// The share of its angle to straight down a hanging shoot still carries
/// after running `travelled` of its pendulous length. The turn eases out as a
/// cube, so half of it is spent in the first fifth of the run and the lower
/// half of a shoot at full sag hangs within a few degrees of vertical: a
/// weeping stem arches over where it leaves the wood that bears it and falls
/// straight for the rest of its length. A run longer than the pendulous
/// length holds the angle it reached.
fn remaining(t: TwigParams, travelled: f64) -> f64 {
    let u = (travelled / t.pendulous_length).clamp(0.0, 1.0);
    let left = 1.0 - u;
    1.0 - t.sag * (1.0 - left * left * left)
}

/// Linear between the two, exact at both ends: a row at 0 or 1 is the end
/// itself and not a rounding of it.
fn walk(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + b * t
}

fn walk3(a: Vec3, b: Vec3, t: f64) -> Vec3 {
    Vec3::new(walk(a.x, b.x, t), walk(a.y, b.y, t), walk(a.z, b.z, t))
}
