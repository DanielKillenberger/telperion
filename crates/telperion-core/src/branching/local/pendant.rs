//! The curtain a hanging shoot grows: how far it droops, how long it runs,
//! how close to the floor beneath it it may reach, and how far apart its
//! neighbours stand.
//!
//! A shoot's own weight bends it as it runs: the sag row turns its course
//! toward straight down along the run, from the departure the droop gave it.
//! And no two shoots need run alike: the variation row gives each its own
//! share of the pendulous length, drawn from the shoot's own key. The drop
//! row lets a hanging shoot fall past the shell's lower surface, down a share
//! of the way to a clearance above the ground: the limbs make the crown's
//! shape and the strands hang from them below it.
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
/// The salt a shoot's own share of the pendulous length is drawn with, beside
/// the ones the local law draws a lateral's vigour and departure with.
const RUN: u32 = 0x3c6ef372;
/// The steps a column is searched in for the shell's lower surface, per lobe
/// or per crown where the crown is shorter, and the halvings that close on it.
const SEARCH: f64 = 32.0;
const HALVINGS: usize = 32;

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
    /// down from that tip to `base`, the height the crown's own room stops at,
    /// and the drop row carries that on down toward the clearance.
    pub fn new(t: TwigParams, at: Vec3, tip: Option<f64>, base: f64) -> Self {
        let hang = if tip.is_some() { t.hang } else { 0.0 };
        let bottom = walk(base, t.curtain_clearance.min(base), dropped(t));
        Self {
            hang,
            across: Vec3::new(-at.z, 0.0, at.x).normalized(),
            floor: tip.map(|tip| walk(tip, bottom, t.sag * hang.min(1.0))),
        }
    }

    pub fn hangs(self) -> bool {
        self.hang > 0.0
    }

    /// Whether this curtain's shoots may fall past the shell at all. A curtain
    /// nobody hangs, and a table that states no drop, do not.
    pub fn drops(self, t: TwigParams) -> bool {
        self.hangs() && t.curtain_drop > 0.0
    }

    /// Whether a candidate at `p` may be born: inside the room the config
    /// allows it, or, for a curtain that drops, in the band below the shell.
    /// A curtain that does not drop is bound exactly as every other shoot.
    pub fn admits(self, config: &GrowthConfig, t: TwigParams, p: Vec3) -> bool {
        !rejected(config, p)
            || (self.drops(t)
                && config
                    .shell
                    .is_some_and(|shell| in_band(&shell, &t, config.seed, p, 0.0)))
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

    /// A pendulous shoot stops at its own pendulous length; the hang row walks
    /// that cap in from the length the branch law asked for. Under a sag the
    /// pendulous length is what the shoot runs rather than a cap on what the
    /// branch law allows: the allometry that gives a self-supporting limb its
    /// length by its own thickness has nothing to say about a strand hanging
    /// from one, and the hang row then walks that run in from the law's own
    /// length rather than extrapolating past it. `shoot` is the shoot's key
    /// and the family seed, which its own length is drawn from.
    pub fn length(self, length: f64, t: TwigParams, shoot: u32) -> f64 {
        let own = pendulous(t, shoot);
        if t.sag == 0.0 {
            return walk(length, length.min(own), self.hang);
        }
        let hung = walk(length.min(own), own, t.sag);
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
    /// a course this does not bend is returned as it came. The arc is spent
    /// over the shoot's own pendulous length, so a short strand ends as near
    /// vertical as a long one.
    pub fn sagged(self, course: Vec3, travelled: f64, t: TwigParams, shoot: u32) -> Vec3 {
        if !self.sags(t) {
            return course;
        }
        let unit = course.normalized();
        let cosine = (-unit.y).clamp(-1.0, 1.0);
        let turn = cosine.acos_fixed() * (1.0 - remaining(t, travelled, pendulous(t, shoot)));
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

/// Whether `p` lies in the band a hanging shoot of a curtain with the rows
/// `t` may fall into: below the shell's lower surface in `p`'s own column, and
/// no lower than the drop row's share of the way from that surface down to
/// the clearance. A column the shell never reaches is not under the crown's
/// footprint and has no band, and neither has a table that hangs nothing or
/// drops nothing. The clearance never stands above the crown's own base.
/// `tolerance` is metres of slack, as `Envelope::contains` takes it; with the
/// shell's own containment this is the invariant every node past the
/// crossover keeps.
pub fn in_band(shell: &Envelope, t: &TwigParams, seed: u32, p: Vec3, tolerance: f64) -> bool {
    let share = dropped(*t);
    if share <= 0.0 || !p.is_finite() {
        return false;
    }
    let clearance = t.curtain_clearance.min(shell.height * shell.crown_base);
    if p.y < clearance - tolerance {
        return false;
    }
    // The band's top is the surface itself, and its foot rises with it: a
    // surface above `limit` puts `p` below the foot of its band.
    let limit = if share < 1.0 {
        (p.y + tolerance - share * clearance) / (1.0 - share)
    } else {
        shell.height
    };
    lower_surface(shell, seed, p, limit).is_some_and(|surface| {
        p.y < surface + tolerance && p.y + tolerance >= walk(surface, clearance, share)
    })
}

/// The share of the way to the clearance a table lets its hanging shoots fall.
/// The hang row walks it in from nothing, as it walks the floor.
fn dropped(t: TwigParams) -> f64 {
    t.curtain_drop * t.hang.min(1.0)
}

/// The height of the shell's lower surface over `p`'s column: the lowest
/// height at or below `limit` at which a point as far from the axis on the
/// same bearing lies inside the shell, or None where the column does not meet
/// it by then. The lobes scale the smooth radius by at most one plus the
/// irregularity, so the column is outside everywhere below the smooth
/// shell's own lower surface for that much less radius; the search climbs
/// from there in steps finer than a lobe and closes on the crossing by halving.
fn lower_surface(shell: &Envelope, seed: u32, p: Vec3, limit: f64) -> Option<f64> {
    let radial = p.x.hypot_fixed(p.z);
    let outside = |y: f64| radial > shell.radius_toward(Vec3::new(p.x, y, p.z), seed);
    let base = shell.height * shell.crown_base;
    let span = shell.height - base;
    let widest = shell.max_radius() * (1.0 + shell.irregularity);
    if span <= 0.0 || widest <= 0.0 || radial > widest {
        return None;
    }
    let shoulder = shell.shoulder.max(0.1);
    let rising = (1.0 - (radial / widest).powf_fixed(shoulder))
        .max(0.0)
        .powf_fixed(1.0 / shoulder);
    let mut low = base + span * shell.fullness.clamp(0.001, 0.999) * (1.0 - rising);
    if !outside(low) {
        return Some(low);
    }
    let step = span.min(shell.lobe_scale * shell.height) / SEARCH;
    let top = limit.min(shell.height);
    while low < top {
        let mut high = low + step;
        if outside(high) {
            low = high;
            continue;
        }
        for _ in 0..HALVINGS {
            let mid = (low + high) / 2.0;
            if outside(mid) {
                low = mid
            } else {
                high = mid
            }
        }
        return Some(high);
    }
    None
}

/// A hanging shoot's own pendulous length: the table's, shortened by the
/// variation row times a draw in 0 to 1 keyed by `shoot`. The key is the
/// shoot's identity and the seed, not its place in the order the tree grows
/// in, so the monthly and the one-shot build agree; at a variation of 0 the
/// draw multiplies nothing and every shoot has the table's length to the byte.
fn pendulous(t: TwigParams, shoot: u32) -> f64 {
    let share = Rng::new(shoot ^ RUN).next_f64();
    t.pendulous_length * (1.0 - t.pendulous_variation * share)
}

/// The share of its angle to straight down a hanging shoot still carries
/// after running `travelled` of its own pendulous length `run`. The turn eases
/// out as a cube, so half of it is spent in the first fifth of the run and the
/// lower half of a shoot at full sag hangs within a few degrees of vertical: a
/// weeping stem arches over where it leaves the wood that bears it and falls
/// straight for the rest of its length. A run longer than the pendulous
/// length holds the angle it reached.
fn remaining(t: TwigParams, travelled: f64, run: f64) -> f64 {
    let u = (travelled / run).clamp(0.0, 1.0);
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Only a curtain that hangs is let below the shell, only into its band,
    /// and only by a table that drops it.
    #[test]
    fn only_a_hanging_curtain_is_let_into_the_band() {
        let shell = Envelope {
            height: 10.0,
            crown_base: 0.3,
            ..Envelope::default()
        };
        let config = default_growth(shell, 0, 0.01);
        let t = TwigParams {
            hang: 1.0,
            sag: 1.0,
            curtain_drop: 1.0,
            curtain_clearance: 1.0,
            ..TwigParams::default()
        };
        let at = Vec3::new(1.0, 5.0, 0.0);
        let hanging = Curtain::new(t, at, Some(5.0), config.trunk_height);
        // Under the crown, below its lower surface, above the clearance.
        let under = Vec3::new(1.0, 2.0, 0.0);
        assert!(hanging.admits(&config, t, under));
        assert!(!Curtain::default().admits(&config, t, under));
        let dry = TwigParams {
            curtain_drop: 0.0,
            ..t
        };
        let still = Curtain::new(dry, at, Some(5.0), config.trunk_height);
        assert!(!still.admits(&config, dry, under));
        // Below the clearance, beside the crown's footprint, and outside the
        // shell above its lower surface, the shell binds as it always did.
        for p in [
            Vec3::new(1.0, 0.5, 0.0),
            Vec3::new(3.5, 2.0, 0.0),
            Vec3::new(2.9, 9.0, 0.0),
        ] {
            assert!(!hanging.admits(&config, t, p), "{p:?} was let in");
        }
        // Inside the shell every curtain is admitted, whatever it drops.
        let inside = Vec3::new(1.0, 5.0, 0.0);
        assert!(Curtain::default().admits(&config, t, inside));
        assert!(hanging.admits(&config, t, inside));
    }
}
