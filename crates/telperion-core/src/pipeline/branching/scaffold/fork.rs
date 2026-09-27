//! Codominant forks: one rule from the ground to the crown.
//!
//! Every structural axis decides once, as it is born and from its own stream,
//! whether it forks and where: a height drawn from a bell about `forkHeight`
//! of the tree's height, `forkHeightSpread` wide. The axis forks where it
//! climbs through that height. A height at or below its birth draws no fork,
//! and one less than a station above it none either, so every fork follows a
//! station of growth; only the root's own axis forks at its base, height
//! zero, which is a clump.
//!
//! The rate grows a fork in rather than switching it on: each axis draws its
//! own point on the rate, and past it the fork's siblings grow in from the
//! fork over a short width of the rate, their length and their wood that
//! weight of a whole part's. At the fork the axis carries on as the primary
//! and the siblings leave beside it with its order, the rest of its length
//! and its bound. The parts fill four fixed slots in order - `forkWays` of
//! them, the fraction growing one more in - so a part added never moves the
//! ones already there: slot `k` stands `k` times `forkDivergence` of bearing
//! round from the primary's, about a bearing the axis's stream decides, and
//! leans `forkLean` from the axis; `forkLeanSpread` stands the primary back
//! along the axis. Every part, the primary among them, decides again for
//! itself, so forks repeat up the crown.
use super::*;
use std::f64::consts::TAU;

/// The stream the root's axis has always drawn from.
const ROOT_STREAM: u32 = 0x742b_e831;
/// The stream a fork's bearing is drawn from, laid over the axis's key with
/// the root's own stream, so the root's fork faces the bearing the seed gave
/// the retired stems rows.
const BEARING_STREAM: u32 = 0x3f6a_88c5;
/// The stream an axis decides its fork from.
const FORK_STREAM: u32 = 0x6c07_8965;
/// The station a fork's parts are keyed at, past any an axis bears laterals at.
const FORK_STATION: usize = 0x00fa_b1e5;

/// The bole's height: the crown's base, or the growth's own trunk height if
/// that stands higher.
fn bole(params: &SkeletonParams, config: &GrowthConfig) -> f64 {
    config
        .trunk_height
        .max(params.envelope.height * params.envelope.crown_base)
}

/// How far an order-zero axis runs: the leader's own rule, the bole plus the
/// share of the crown the apical dominance keeps for the leader.
fn top(params: &SkeletonParams, config: &GrowthConfig) -> f64 {
    let base = bole(params, config);
    base + (params.envelope.height - base) * params.habit.apical_dominance
}

/// The frontier as it starts: the root's one upright axis, which may fork at
/// the root itself.
pub(in crate::pipeline::branching) fn axes(
    params: &SkeletonParams,
    config: &GrowthConfig,
) -> VecDeque<Axis> {
    let top = top(params, config);
    if params.envelope.height <= 0.0 || top <= 0.0 {
        return VecDeque::new();
    }
    let mut root = Axis::new(0, Vec3::Y, top, 0, params.seed ^ ROOT_STREAM);
    root.fork = decide(&params.habit, root.key, params.envelope.height, 0.0, None);
    VecDeque::from([root])
}

/// The width of the rate over which a fork grows in from nothing: an axis
/// whose point on the rate the rate passes by this much forks whole.
const GROW_IN: f64 = 0.05;

/// An axis's fork: the height it forks at, how far its siblings have grown
/// in, 0 to 1, and the key it was drawn from, which its parts' keys and next
/// decisions derive from.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub(super) struct Fork {
    pub at: f64,
    pub weight: f64,
    pub key: u32,
}

/// The fork of an axis born at `base`, or none, from its own stream: its
/// point on the rate, so the rate grows forks in one at a time across seeds
/// and never takes one away, and a height that moves as the rows do. `least`
/// is how far above its base an axis must climb before it forks, and none
/// admits a fork at the base itself, which only the root's axis may take.
pub(super) fn decide(
    habit: &HabitParams,
    key: u32,
    height: f64,
    base: f64,
    least: Option<f64>,
) -> Option<Fork> {
    let mut rng = Rng::new(key ^ FORK_STREAM);
    let point = rng.next_f64() * (1.0 - GROW_IN);
    if habit.codominance <= point {
        return None;
    }
    let weight = crate::math::smoothstep(0.0, GROW_IN, habit.codominance - point);
    let (u, v) = (rng.next_f64(), rng.next_f64());
    let z = (-2.0 * (1.0 - u).ln_fixed()).sqrt() * (TAU * v).cos_fixed();
    let at = (habit.fork_height + habit.fork_height_spread * z).clamp(0.0, 1.0) * height;
    let reached = match least {
        None => at + TOLERANCE >= base,
        Some(rise) => at > base + rise + TOLERANCE,
    };
    reached.then_some(Fork { at, weight, key })
}

/// The heading of slot `k` of a fork on the axis keyed `key`, with the axis
/// along +Y: the primary, slot zero, leans by as much of its lean as the
/// spread leaves it and the fork has grown in, every sibling by all of it.
pub(super) fn slot(habit: &HabitParams, key: u32, k: usize, weight: f64) -> Vec3 {
    let bearing = Rng::new(key ^ ROOT_STREAM ^ BEARING_STREAM).range(0.0, TAU);
    let azimuth = bearing + k as f64 * habit.fork_divergence.to_radians();
    let lean = habit.fork_lean.to_radians();
    let tilt = match k {
        0 => lean * (1.0 - habit.fork_lean_spread) * weight,
        _ => lean,
    };
    let out = Vec3::new(azimuth.cos_fixed(), 0.0, azimuth.sin_fixed());
    (Vec3::Y * tilt.cos_fixed() + out * tilt.sin_fixed()).normalized()
}

/// The slots `forkWays` fills beside the primary, each with the share of a
/// whole part it grows: whole to the ways' whole part, the fraction in the
/// next, none past it.
pub(super) fn siblings(habit: &HabitParams) -> impl Iterator<Item = (usize, f64)> {
    let ways = habit.fork_ways;
    (1..4).filter_map(move |k| Some((k, (ways - k as f64).clamp(0.0, 1.0))).filter(|s| s.1 > 0.0))
}

/// `local`, a heading about +Y, turned to stand about `axis` instead.
pub(super) fn about(local: Vec3, axis: Vec3) -> Vec3 {
    let turn = Vec3::Y.cross(axis);
    let (s, c) = (turn.length(), Vec3::Y.dot(axis));
    if s <= 1e-12 {
        return if c > 0.0 {
            local
        } else {
            Vec3::new(local.x, -local.y, -local.z)
        };
    }
    local.rotate_sin_cos(turn / s, (s, c)).normalized()
}

/// `v` turned by the least rotation that carries `from` onto `to`.
pub(super) fn turn(from: Vec3, to: Vec3, v: Vec3) -> Vec3 {
    let axis = from.cross(to);
    let (s, c) = (axis.length(), from.dot(to));
    if s <= 1e-15 {
        return v;
    }
    v.rotate_sin_cos(axis / s, (s, c))
}

/// The key of part `k` of a fork on the axis keyed `key`.
pub(super) fn part_key(key: u32, k: usize) -> u32 {
    axis_key(key, FORK_STATION, k)
}

/// Two headings this close are one heading: the parts would be the same wood
/// grown twice, node for node, rather than two standing beside each other.
const SAME_HEADING: f64 = 1e-9;

/// Refuses a fork whose parts leave on one heading - no divergence between
/// them and no spread to stand the primary back, or no lean to carry them
/// apart at all - by the part that is wrong and the rows that put it there.
/// Headings apart are all it
/// guarantees: whether wood grown from them meets is measured, not refused.
/// A family that never forks has nothing to refuse, whatever its other fork
/// rows say.
///
/// Degeneracy and not clearance on purpose: any positive clearance would make
/// some point between two valid rows invalid, and every point between two
/// valid families is itself a valid family. Parts close enough to touch are a
/// narrow fork, which is a look and not an error.
pub(in crate::pipeline::branching) fn placed(params: &SkeletonParams) -> Result<()> {
    let habit = &params.habit;
    if habit.codominance <= 0.0 {
        return Ok(());
    }
    let headings: Vec<Vec3> = std::iter::once(0)
        .chain(siblings(habit).map(|(k, _)| k))
        .map(|k| slot(habit, 0, k, 1.0))
        .collect();
    for (k, heading) in headings.iter().enumerate() {
        let Some(j) = headings[..k]
            .iter()
            .position(|other| (*heading - *other).length() <= SAME_HEADING)
        else {
            continue;
        };
        return Err(Error::InvalidValue {
            field: "fork parts pass through each other",
            value: format!(
                "part {k} leaves on part {j}'s heading at forkDivergence {}, forkLean {} and \
                 forkLeanSpread {}",
                habit.fork_divergence, habit.fork_lean, habit.fork_lean_spread
            ),
        });
    }
    Ok(())
}

#[cfg(test)]
#[path = "fork_tests.rs"]
mod tests;
