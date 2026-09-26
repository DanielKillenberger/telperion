//! One point between two families. Every parameter of a family is a number, so
//! a blend is per-field linear interpolation at one seed: no field is a switch,
//! every point between two valid rows is itself a valid family, and a
//! transition between two presets has no frame where the tree changes kind.
use crate::{
    bias::SupernaturalParams,
    catalogue::{linear, walk},
    presets::Family,
    Error, Result,
};

/// The family at `t` between `a` and `b`, where 0 is `a` and 1 is `b`, both
/// returned unchanged. The two rows are read at one seed — the result carries
/// `a`'s, and the caller sets the seed it wants on both before it asks.
///
/// A spacing whose zero grows nothing walks as the density it stands for, so
/// a walk from none thins in from nothing rather than arriving at its closest.
/// Angles take the shortest path between their two bearings. Counts are
/// rounded last, and the leaf's counts are rounded the way its own rule
/// demands: lobes down and sections up, so a margin that is legal at both ends
/// is legal in between. A supernatural field that is off carries no amplitude,
/// which is how the bias itself reads it, so a family that switches the terms
/// on ramps up from nothing.
pub fn families(a: &Family, b: &Family, t: f64) -> Result<Family> {
    if !t.is_finite() || !(0.0..=1.0).contains(&t) {
        return Err(Error::InvalidInput("blend parameter"));
    }
    if t == 0.0 {
        return Ok(a.clone());
    }
    if t == 1.0 {
        return Ok(b.clone());
    }
    let mut f = a.clone();
    // Every row whose entry names its own walk moves by it; the rest are
    // coupled to each other and walk below.
    walk(a, b, t, &mut f);
    // A card is the blade with no anatomy at all, and it carries neither lobes
    // nor roundness: a walk from a card to a leaf is a leaf.
    f.element.card = a.element.card && b.element.card;
    f.skeleton.bias.supernatural = supernatural(
        a.skeleton.bias.supernatural,
        b.skeleton.bias.supernatural,
        t,
    );
    let (ga, gb) = (
        a.skeleton.resolved_growth(a.skeleton.attractors)?,
        b.skeleton.resolved_growth(b.skeleton.attractors)?,
    );
    let (oa, ob) = (a.skeleton.growth, b.skeleton.growth);
    let g = &mut f.skeleton.growth;
    let set = |from: Option<f64>, to: Option<f64>, ra, rb| overridden(from, to, ra, rb, t);
    g.influence_radius = set(
        oa.influence_radius,
        ob.influence_radius,
        ga.influence_radius,
        gb.influence_radius,
    );
    g.kill_distance = set(
        oa.kill_distance,
        ob.kill_distance,
        ga.kill_distance,
        gb.kill_distance,
    );
    g.step_distance = set(
        oa.step_distance,
        ob.step_distance,
        ga.step_distance,
        gb.step_distance,
    );
    g.trunk_height = set(
        oa.trunk_height,
        ob.trunk_height,
        ga.trunk_height,
        gb.trunk_height,
    );
    g.max_turn_per_step = set(
        oa.max_turn_per_step,
        ob.max_turn_per_step,
        ga.max_turn_per_step,
        gb.max_turn_per_step,
    );
    g.max_nodes = (oa.max_nodes.is_some() || ob.max_nodes.is_some())
        .then(|| linear(ga.max_nodes as f64, gb.max_nodes as f64, t).round() as usize);
    Ok(f)
}

/// A growth override neither row states is left to the envelope, as it was on
/// both; one either row states walks between the two resolved distances, so the
/// walk is continuous where one row overrides and the other does not.
fn overridden(from: Option<f64>, to: Option<f64>, a: f64, b: f64, t: f64) -> Option<f64> {
    (from.is_some() || to.is_some()).then(|| linear(a, b, t))
}

/// A field that is off has no amplitude — the bias reads a disabled row as the
/// zero one — so the walk interpolates what each side actually applies, and the
/// terms rise from nothing rather than arriving at one frame.
fn supernatural(a: SupernaturalParams, b: SupernaturalParams, t: f64) -> SupernaturalParams {
    let applied = |p: SupernaturalParams| {
        if p.enabled {
            p
        } else {
            SupernaturalParams::NONE
        }
    };
    let (from, to) = (applied(a), applied(b));
    SupernaturalParams {
        enabled: a.enabled || b.enabled,
        writhe_amplitude: linear(from.writhe_amplitude, to.writhe_amplitude, t),
        writhe_wavelength: linear(from.writhe_wavelength, to.writhe_wavelength, t),
        max_writhe_magnitude: linear(a.max_writhe_magnitude, b.max_writhe_magnitude, t),
        spiral_rate: linear(from.spiral_rate, to.spiral_rate, t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::catalogue::{degrees, density};

    #[test]
    fn a_walk_rounds_each_kind_of_number_the_way_its_rule_demands() {
        assert_eq!(linear(2.0, 4.0, 0.25), 2.5);
        // The shorter arc across the wrap, not the long way round, and a walk
        // between two positive bearings stays positive.
        assert_eq!(degrees(10.0, 350.0, 0.5), 0.0);
        assert_eq!(degrees(350.0, 10.0, 0.5), 360.0);
        assert_eq!(degrees(55.0, 88.0, 0.5), 71.5);
        // Counts round to the nearest, lobes down and sections up, on the
        // rows that carry them.
        let (mut a, mut b) = (Family::default(), Family::default());
        (
            a.skeleton.habit.laterals_per_station,
            b.skeleton.habit.laterals_per_station,
        ) = (1, 4);
        (a.element.lobe_count, b.element.lobe_count) = (0, 5);
        (a.element.axial_segments, b.element.axial_segments) = (20, 40);
        (a.skeleton.attractors, b.skeleton.attractors) = (0, 1000);
        let at = |t| families(&a, &b, t).unwrap();
        assert_eq!(
            (
                at(0.5).skeleton.habit.laterals_per_station,
                at(0.1).skeleton.habit.laterals_per_station
            ),
            (3, 1)
        );
        assert_eq!(
            (at(0.99).element.lobe_count, at(0.01).element.axial_segments),
            (4, 21)
        );
        assert_eq!(at(0.4).skeleton.attractors, 400);
        // A spacing walks as its density: halfway from none to a shoot every
        // 10 cm is one every 20, and between two spacings the harmonic mean.
        assert_eq!(density(0.0, 0.1, 0.5), 0.2);
        assert_eq!(density(0.1, 0.0, 1.0), 0.0);
        assert!((density(0.1, 0.3, 0.5) - 0.15).abs() < 1e-12);
        assert_eq!(density(0.0, 0.1, 1e-9), 1000.0);
        // An override neither row states stays the envelope's to answer.
        assert_eq!(overridden(None, None, 1.0, 2.0, 0.5), None);
        assert_eq!(overridden(None, Some(2.0), 1.0, 2.0, 0.5), Some(1.5));
    }

    #[test]
    fn a_walk_between_two_material_rows_stays_a_row_a_leaf_can_be_drawn_from() {
        // Both ends of a range interpolate linearly, so a point between two
        // valid rows is valid: no walk needs re-validating, and none of it has
        // a frame where the leaves have no colour to take.
        let mut from = crate::presets::Preset::OregonWhiteOak.parameters();
        let mut to = crate::presets::Preset::NorwaySpruce.parameters();
        // The two rows' ranges are deliberately unequal at both ends, which is
        // what a walk between them has to survive.
        (from.material.hue_range_low, from.material.hue_range_high) = (-0.4, 0.1);
        (to.material.hue_range_low, to.material.hue_range_high) = (-0.05, 0.45);
        for step in 0..=10 {
            let t = f64::from(step) / 10.0;
            let walked = families(&from, &to, t).unwrap().material;
            assert_eq!(walked.validate(), Ok(()), "the row at {t} is not a row");
        }
        let half = families(&from, &to, 0.5).unwrap().material;
        assert!(
            (half.bark_red - (from.material.bark_red + to.material.bark_red) / 2.0).abs() < 1e-12
        );
        assert!((half.interior_darkening - 0.625).abs() < 1e-12);
    }

    #[test]
    fn a_field_that_is_off_walks_up_from_nothing() {
        let off = SupernaturalParams {
            enabled: false,
            writhe_amplitude: 9.0,
            writhe_wavelength: 9.0,
            spiral_rate: 9.0,
            max_writhe_magnitude: 0.9,
        };
        let on = SupernaturalParams {
            enabled: true,
            writhe_amplitude: 0.1,
            writhe_wavelength: 0.5,
            spiral_rate: 2.0,
            max_writhe_magnitude: 0.9,
        };
        let half = supernatural(off, on, 0.5);
        assert!(half.enabled);
        assert_eq!(half.writhe_amplitude, 0.05);
        assert_eq!(half.spiral_rate, 1.0);
        assert_eq!(supernatural(off, off, 0.5), SupernaturalParams::NONE);
    }
}
