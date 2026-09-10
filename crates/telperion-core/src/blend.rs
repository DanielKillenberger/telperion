//! One point between two families. Every parameter of a family is a number, so
//! a blend is per-field linear interpolation at one seed: no field is a switch,
//! every point between two valid rows is itself a valid family, and a
//! transition between two presets has no frame where the tree changes kind.
use crate::{bias::SupernaturalParams, presets::Family, Error, Result};

/// The family at `t` between `a` and `b`, where 0 is `a` and 1 is `b`, both
/// returned unchanged. The two rows are read at one seed — the result carries
/// `a`'s, and the caller sets the seed it wants on both before it asks.
///
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
    /// Assigns each named field the walk of the two rows' values through `$op`.
    macro_rules! walk {
        ($op:ident: $($($p:ident).+),+ $(,)?) => {
            $( f.$($p).+ = $op(a.$($p).+, b.$($p).+, t); )+
        };
    }
    walk!(linear:
        skeleton.habit.apical_dominance, skeleton.habit.whorl_strength,
        skeleton.habit.leader_internode, skeleton.habit.rise_primary,
        skeleton.habit.rise_secondary, skeleton.habit.lateral_spacing,
        skeleton.habit.lateral_length_ratio, skeleton.habit.attractor_weight,
        skeleton.habit.twig_tip_taper, skeleton.habit.shedding_threshold,
        skeleton.step,
        skeleton.envelope.height, skeleton.envelope.crown_base,
        skeleton.envelope.spread, skeleton.envelope.fullness,
        skeleton.envelope.shoulder,
        skeleton.bias.gravitropism, skeleton.bias.lean,
        skeleton.twigs.twig.diameter, skeleton.twigs.twig.length,
        skeleton.twigs.twig.internode_length, skeleton.twigs.twig.bearing_diameter,
        skeleton.twigs.length_ratio, skeleton.twigs.ratio_power,
        skeleton.twigs.internode_factor, skeleton.twigs.limb_radius,
        skeleton.twigs.reach, skeleton.twigs.vigour_variation,
        radii.trunk_radius, radii.fork_exponent, radii.length_taper,
        surface.lobe_depth, surface.twist_rate, surface.flare_radius,
        surface.flare_falloff, surface.flare_depth, surface.fork_socket,
        surface.fork_swell,
        canopy.shoot_radius, canopy.spacing, canopy.clump_span, canopy.outward,
        canopy.upward, canopy.forward_lean, canopy.lean_rise,
        canopy.surface_contact, canopy.size, canopy.size_variation,
        element.connector_length, element.length, element.width,
        element.widest_at, element.base_fullness, element.tip_sharpness,
        element.cup, element.curl, element.lobe_depth, element.section_roundness,
        shell_depth,
    );
    walk!(degrees:
        skeleton.habit.lateral_pitch, skeleton.habit.pitch_variation,
        skeleton.habit.crookedness,
        skeleton.twigs.angle, skeleton.twigs.angle_variation,
        skeleton.twigs.divergence,
        canopy.divergence, canopy.scatter,
    );
    walk!(count:
        skeleton.habit.laterals_per_station, skeleton.habit.lateral_orders,
        skeleton.twigs.twig.stations_per_internode, skeleton.twigs.laterals,
        surface.radial_segments, surface.lobes,
        canopy.clump, element.cross_segments,
    );
    walk!(many: skeleton.attractors, canopy.max_instances);
    // The leaf's own rounding rule: a lobed margin needs a crest and a sinus
    // section per lobe plus the base and the tip, and both sides of that are
    // linear, so lobes round down and sections up and no step of a walk
    // between two legal margins is illegal.
    f.element.lobe_count = down(a.element.lobe_count, b.element.lobe_count, t);
    f.element.axial_segments = up(a.element.axial_segments, b.element.axial_segments, t);
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
        .then(|| many(ga.max_nodes, gb.max_nodes, t));
    Ok(f)
}

fn linear(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Degrees along the shorter arc, so two bearings either side of the wrap meet
/// across it rather than sweeping the long way round. A walk between two
/// bearings that are themselves positive stays positive, which is the range
/// every angle in a family is validated against.
fn degrees(a: f64, b: f64, t: f64) -> f64 {
    let delta = (b - a + 180.0).rem_euclid(360.0) - 180.0;
    let turned = a + delta * t;
    if turned < 0.0 && a >= 0.0 && b >= 0.0 {
        turned + 360.0
    } else {
        turned
    }
}

/// A growth override neither row states is left to the envelope, as it was on
/// both; one either row states walks between the two resolved distances, so the
/// walk is continuous where one row overrides and the other does not.
fn overridden(from: Option<f64>, to: Option<f64>, a: f64, b: f64, t: f64) -> Option<f64> {
    (from.is_some() || to.is_some()).then(|| linear(a, b, t))
}

fn count(a: u32, b: u32, t: f64) -> u32 {
    linear(f64::from(a), f64::from(b), t).round() as u32
}

fn down(a: u32, b: u32, t: f64) -> u32 {
    linear(f64::from(a), f64::from(b), t).floor() as u32
}

fn up(a: u32, b: u32, t: f64) -> u32 {
    linear(f64::from(a), f64::from(b), t).ceil() as u32
}

fn many(a: usize, b: usize, t: f64) -> usize {
    linear(a as f64, b as f64, t).round() as usize
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
        spiral_rate: linear(from.spiral_rate, to.spiral_rate, t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_walk_rounds_each_kind_of_number_the_way_its_rule_demands() {
        assert_eq!(linear(2.0, 4.0, 0.25), 2.5);
        // The shorter arc across the wrap, not the long way round, and a walk
        // between two positive bearings stays positive.
        assert_eq!(degrees(10.0, 350.0, 0.5), 0.0);
        assert_eq!(degrees(350.0, 10.0, 0.5), 360.0);
        assert_eq!(degrees(55.0, 88.0, 0.5), 71.5);
        // Counts round to the nearest, lobes down and sections up.
        assert_eq!((count(1, 4, 0.5), count(1, 4, 0.1)), (3, 1));
        assert_eq!((down(0, 5, 0.99), up(20, 40, 0.01)), (4, 21));
        assert_eq!(many(0, 1000, 0.4), 400);
        // An override neither row states stays the envelope's to answer.
        assert_eq!(overridden(None, None, 1.0, 2.0, 0.5), None);
        assert_eq!(overridden(None, Some(2.0), 1.0, 2.0, 0.5), Some(1.5));
    }

    #[test]
    fn a_field_that_is_off_walks_up_from_nothing() {
        let off = SupernaturalParams {
            enabled: false,
            writhe_amplitude: 9.0,
            writhe_wavelength: 9.0,
            spiral_rate: 9.0,
        };
        let on = SupernaturalParams {
            enabled: true,
            writhe_amplitude: 0.1,
            writhe_wavelength: 0.5,
            spiral_rate: 2.0,
        };
        let half = supernatural(off, on, 0.5);
        assert!(half.enabled);
        assert_eq!(half.writhe_amplitude, 0.05);
        assert_eq!(half.spiral_rate, 1.0);
        assert_eq!(supernatural(off, off, 0.5), SupernaturalParams::NONE);
    }
}
