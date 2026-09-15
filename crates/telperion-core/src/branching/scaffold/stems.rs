//! The stems a family is born with: the order-zero axes that leave the root.
//!
//! One stem is the single upright trunk every tree had before stems were a
//! row, keyed and headed exactly as it was, so a table that leaves the row at
//! one grows the tree it always grew. More than one and they part at the
//! ground: laid out about a bearing the seed alone decides, `stem_divergence`
//! of bearing between neighbours, and tilted away from the centre by
//! `stem_lean` — the outermost by all of it, the ones between in proportion to
//! how far out they stand, so the middle stem of an odd clump stands upright.
//! `stem_lean_spread` carries that lean from the clump's centre to its order:
//! all of it and the first stem stands upright, the last leans by the whole
//! angle and the ones between by their place in the clump, which is a
//! dominant stem with the lesser ones pushed out beside it.
//! `stem_fork_height` moves where they part: none of it and every stem leaves
//! the root; some of it and the first stem grows alone up to that share of
//! the bole, and the later stems leave it there, one trunk below the fork.
use super::*;
use std::f64::consts::TAU;

/// The stream the one stem of a single-stemmed family has always drawn from.
const ROOT_STREAM: u32 = 0x742b_e831;
/// The stream the fan's own bearing is drawn from: one seed, one bearing.
const BEARING_STREAM: u32 = 0x3f6a_88c5;

/// One stem: the heading it leaves the root on, and the random stream its own
/// subtree draws from.
pub(in crate::branching) struct Stem {
    pub heading: Vec3,
    pub key: u32,
}

/// The bole's height: the crown's base, or the growth's own trunk height if
/// that stands higher.
fn bole(params: &SkeletonParams, config: &GrowthConfig) -> f64 {
    config
        .trunk_height
        .max(params.envelope.height * params.envelope.crown_base)
}

/// How far an order-zero axis runs: the leader's own rule, the bole plus the
/// share of the crown the apical dominance keeps for the leader.
pub(in crate::branching) fn top(params: &SkeletonParams, config: &GrowthConfig) -> f64 {
    let base = bole(params, config);
    base + (params.envelope.height - base) * params.habit.apical_dominance
}

/// The clump as the frontier first holds it. Every stem is an order-zero axis
/// with the leader's own length rule - the bole gates read order and not
/// birth, so a stem is a trunk all the way down and never a lateral in the
/// bole. With no fork height every stem leaves the root; with one, the first
/// leaves it alone and carries the rest until it stands at their height, and
/// each of them runs to the leader's top from there.
pub(in crate::branching) fn axes(params: &SkeletonParams, config: &GrowthConfig) -> VecDeque<Axis> {
    let top = top(params, config);
    if params.envelope.height <= 0.0 || top <= 0.0 {
        return VecDeque::new();
    }
    let fork = bole(params, config) * params.habit.stem_fork_height;
    let mut clump = stems(params)
        .into_iter()
        .map(|stem| Axis::new(0, stem.heading, top, 0, stem.key));
    if fork <= 0.0 {
        return clump.collect();
    }
    let Some(mut first) = clump.next() else {
        return VecDeque::new();
    };
    first.forks = clump
        .map(|stem| Axis {
            length: top - fork,
            ..stem
        })
        .collect();
    first.fork_height = fork;
    VecDeque::from([first])
}

impl Axis {
    /// Hands the stems held on this axis to `into`, born on node `at`, once
    /// that node stands at their height - or wherever the axis stopped, if it
    /// stops short of it, so a clump never loses a stem to a short leader.
    pub(super) fn part(&mut self, at: usize, height: f64, stopped: bool, into: &mut Vec<Axis>) {
        if self.forks.is_empty() || !(stopped || height + TOLERANCE >= self.fork_height) {
            return;
        }
        into.extend(self.forks.drain(..).map(|stem| Axis {
            at,
            tip: at,
            ..stem
        }));
    }
}

/// Every stem of a family, in order.
pub(in crate::branching) fn stems(params: &SkeletonParams) -> Vec<Stem> {
    let key = params.seed ^ ROOT_STREAM;
    let count = params.habit.stems.max(1);
    if count == 1 {
        return vec![Stem {
            heading: Vec3::Y,
            key,
        }];
    }
    let bearing = Rng::new(params.seed ^ BEARING_STREAM).range(0.0, TAU);
    let divergence = params.habit.stem_divergence.to_radians();
    let lean = params.habit.stem_lean.to_radians();
    let spread = params.habit.stem_lean_spread;
    let last = f64::from(count - 1);
    let span = last / 2.0;
    (0..count)
        .map(|k| {
            let offset = f64::from(k) - span;
            let azimuth = bearing + offset * divergence;
            // How far out it stands, walked toward where it comes in the
            // clump; at no spread the walk adds nothing, to the bit.
            let fanned = (offset / span).abs();
            let ordered = f64::from(k) / last;
            let tilt = lean * (fanned + (ordered - fanned) * spread);
            let out = Vec3::new(azimuth.cos_fixed(), 0.0, azimuth.sin_fixed());
            Stem {
                heading: (Vec3::Y * tilt.cos_fixed() + out * tilt.sin_fixed()).normalized(),
                key: axis_key(key, 0, k as usize),
            }
        })
        .collect()
}

/// Two headings this close are one heading: the stems would be the same stem
/// grown twice, node for node, rather than two standing beside each other.
const SAME_HEADING: f64 = 1e-9;

/// Refuses a clump of stems the tree cannot be built on, by the stem that is
/// wrong: one whose first growth unit is already outside the shell, and one
/// that leaves the root on a heading a neighbour has already taken — no
/// divergence between them and no spread to lean them unequally, or no lean to
/// carry them apart at all, which is the same stem twice and not two. A tree on
/// one stem has nothing to refuse.
///
/// The second test is degeneracy and not clearance on purpose. Any positive
/// clearance would make some point between two valid rows invalid — a walk
/// that opens a clump passes through every separation between nothing and the
/// one it ends on — and the blend's contract is that every point between two
/// valid families is itself a valid family. Stems that stand close enough to
/// touch are a tree with a narrow fork, which is a look and not an error.
pub(in crate::branching) fn placed(params: &SkeletonParams, config: &GrowthConfig) -> Result<()> {
    let stems = stems(params);
    if stems.len() < 2 {
        return Ok(());
    }
    let unit = growth_unit(params.habit, config, 0);
    for (k, stem) in stems.iter().enumerate() {
        if !first_edge_fits(params, stem.heading * unit) {
            return Err(Error::InvalidValue {
                field: "stem outside the crown envelope",
                value: format!("stem {k}"),
            });
        }
        if stems[..k]
            .iter()
            .any(|other| (stem.heading - other.heading).length() <= SAME_HEADING)
        {
            return Err(Error::InvalidValue {
                field: "stems pass through each other",
                value: format!("stem {k}"),
            });
        }
    }
    Ok(())
}

/// The scaffold's own gate on an edge, asked of the first one a stem would
/// take. The shell has no width at the root, so a stem leaves it the way a
/// leaning bole meets the crown - from outside, free to close on it - and what
/// is left to ask is whether the first unit is inside the tree's own height at
/// all. A shell that has no room for it is a shell the stem cannot start in.
fn first_edge_fits(params: &SkeletonParams, p: Vec3) -> bool {
    p.y >= -TOLERANCE && p.y <= params.envelope.height + TOLERANCE
}

#[cfg(test)]
mod tests {
    use super::*;

    fn habit(stems: u32, stem_divergence: f64, stem_lean: f64) -> SkeletonParams {
        let mut p = crate::presets::Preset::Ordinary.parameters().skeleton;
        p.habit.stems = stems;
        p.habit.stem_divergence = stem_divergence;
        p.habit.stem_lean = stem_lean;
        p
    }

    #[test]
    fn one_stem_is_the_upright_axis_the_tree_always_had() {
        let p = habit(1, 90.0, 30.0);
        let only = stems(&p);
        assert_eq!(only.len(), 1);
        assert_eq!(only[0].heading, Vec3::Y);
        assert_eq!(only[0].key, p.seed ^ ROOT_STREAM);
    }

    #[test]
    fn a_clump_is_spread_about_its_bearing_and_leans_out_of_it() {
        let p = habit(3, 60.0, 20.0);
        let fan = stems(&p);
        assert_eq!(fan.len(), 3);
        // The middle stem of an odd clump stands at the centre, so it stands up.
        assert!((fan[1].heading.y - 1.0).abs() < 1e-12);
        for outer in [&fan[0], &fan[2]] {
            assert!((outer.heading.y - 20.0_f64.to_radians().cos()).abs() < 1e-12);
        }
        // Two neighbours are one divergence apart in bearing.
        let bearing = |v: Vec3| v.z.atan2(v.x);
        let step = (bearing(fan[2].heading) - bearing(fan[0].heading)) / 2.0;
        assert!((step - 60.0_f64.to_radians()).abs() < 1e-12);
    }

    /// Degrees from vertical a stem leaves the root at.
    fn tilt(stem: &Stem) -> f64 {
        stem.heading.y.clamp(-1.0, 1.0).acos().to_degrees()
    }

    #[test]
    fn no_spread_is_the_even_lean_to_the_bit() {
        // The clump fn-38 grew, written out: every stem by how far out of the
        // clump's centre it stands, about the same bearing.
        for count in 2..=6 {
            let p = habit(count, 50.0, 30.0);
            let bearing = Rng::new(p.seed ^ BEARING_STREAM).range(0.0, TAU);
            let span = f64::from(count - 1) / 2.0;
            for (k, stem) in stems(&p).iter().enumerate() {
                let offset = k as f64 - span;
                let azimuth = bearing + offset * 50.0_f64.to_radians();
                let tilt = 30.0_f64.to_radians() * (offset / span).abs();
                let out = Vec3::new(azimuth.cos_fixed(), 0.0, azimuth.sin_fixed());
                let was = (Vec3::Y * tilt.cos_fixed() + out * tilt.sin_fixed()).normalized();
                assert_eq!(stem.heading, was, "{count} stems: stem {k} moved");
            }
        }
    }

    #[test]
    fn a_spread_leans_the_clump_in_its_order() {
        for count in 2..=6 {
            let even = stems(&habit(count, 50.0, 30.0));
            for spread in [0.25, 0.5, 0.8, 1.0] {
                let mut p = habit(count, 50.0, 30.0);
                p.habit.stem_lean_spread = spread;
                let fan = stems(&p);
                let leans: Vec<f64> = fan.iter().map(tilt).collect();
                let last = leans.len() - 1;
                // The first stem keeps what the spread leaves it, the last
                // all of it.
                assert!(
                    (leans[0] - 30.0 * (1.0 - spread)).abs() < 1e-9,
                    "{count} stems at {spread}: the first leans {}",
                    leans[0]
                );
                assert!((leans[last] - 30.0).abs() < 1e-9);
                // The bearings are the clump's own: only the lean moved.
                for (stem, was) in fan.iter().zip(&even) {
                    if tilt(stem) > 1e-6 && tilt(was) > 1e-6 {
                        let bearing = |v: Vec3| v.z.atan2(v.x);
                        let turn = bearing(stem.heading) - bearing(was.heading);
                        assert!(turn.sin().abs() < 1e-12, "{count} at {spread} turned");
                    }
                }
                // At the whole spread each stem leans by its place in the
                // clump, so they rise in order from upright.
                if spread == 1.0 {
                    for (k, lean) in leans.iter().enumerate() {
                        let place = 30.0 * k as f64 / last as f64;
                        assert!((lean - place).abs() < 1e-9, "stem {k} leans {lean}");
                    }
                }
                // Two stems lean in order at every spread.
                if count == 2 {
                    assert!(leans[0] < leans[1], "two stems at {spread}");
                }
            }
        }
    }

    #[test]
    fn stems_that_share_a_heading_are_refused_by_the_stem_that_is_wrong() {
        // A spread leaves stems with no lean on one heading still: all of them
        // upright, however unequally nothing is shared out.
        for (divergence, lean, spread) in [(0.0, 20.0, 0.0), (60.0, 0.0, 0.0), (60.0, 0.0, 1.0)] {
            let mut p = habit(2, divergence, lean);
            p.habit.stem_lean_spread = spread;
            let config = p.resolved_growth(0).unwrap();
            assert_eq!(
                placed(&p, &config),
                Err(Error::InvalidValue {
                    field: "stems pass through each other",
                    value: "stem 1".into(),
                }),
                "{divergence} deg apart at {lean} deg of lean was accepted"
            );
        }
        let p = habit(2, 60.0, 20.0);
        let config = p.resolved_growth(0).unwrap();
        assert_eq!(placed(&p, &config), Ok(()));
        // And it parts two stems on one bearing by leaning them unequally.
        let mut p = habit(2, 0.0, 20.0);
        p.habit.stem_lean_spread = 0.5;
        assert_eq!(placed(&p, &config), Ok(()));
    }
}
