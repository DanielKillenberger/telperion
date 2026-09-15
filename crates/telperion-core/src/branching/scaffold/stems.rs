//! The stems a family is born with: the order-zero axes that leave the root.
//!
//! One stem is the single upright trunk every tree had before stems were a
//! row, keyed and headed exactly as it was, so a table that leaves the row at
//! one grows the tree it always grew. More than one and they part at the
//! ground: laid out about a bearing the seed alone decides, `stem_divergence`
//! of bearing between neighbours, and tilted away from the centre by
//! `stem_lean` — the outermost by all of it, the ones between in proportion to
//! how far out they stand, so the middle stem of an odd clump stands upright.
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

/// How far an order-zero axis runs: the leader's own rule, the bole plus the
/// share of the crown the apical dominance keeps for the leader.
pub(in crate::branching) fn top(params: &SkeletonParams, config: &GrowthConfig) -> f64 {
    let base = config
        .trunk_height
        .max(params.envelope.height * params.envelope.crown_base);
    base + (params.envelope.height - base) * params.habit.apical_dominance
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
    let span = f64::from(count - 1) / 2.0;
    (0..count)
        .map(|k| {
            let offset = f64::from(k) - span;
            let azimuth = bearing + offset * divergence;
            let tilt = lean * (offset / span).abs();
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
/// divergence between them, or no lean to carry them apart, which is the same
/// stem twice and not two. A tree on one stem has nothing to refuse.
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
        if !first_edge_fits(params, config, stem.heading * unit) {
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
/// take: the envelope has no width below the crown base, so it constrains the
/// bole's height and nothing else there.
fn first_edge_fits(params: &SkeletonParams, config: &GrowthConfig, p: Vec3) -> bool {
    p.y >= -TOLERANCE
        && p.y <= params.envelope.height + TOLERANCE
        && (p.y < config.trunk_height || params.envelope.contains(p, TOLERANCE, params.seed))
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

    #[test]
    fn stems_that_share_a_heading_are_refused_by_the_stem_that_is_wrong() {
        for (divergence, lean) in [(0.0, 20.0), (60.0, 0.0)] {
            let p = habit(2, divergence, lean);
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
    }
}
