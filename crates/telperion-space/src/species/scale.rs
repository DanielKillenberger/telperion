//! The space's metric (fn-206, host decision 9): how each setting of a
//! physiological age mixes between two species, in one table beside the
//! settings it names. Every positive magnitude mixes on a log scale, above
//! a floor (the least magnitude that matters), so a midpoint of a twig's
//! internode and a trunk's is a geometric one and 0 stays reachable;
//! angles, probabilities, shares and counts mix by value; a delay that may
//! be infinite by the share of a cycle it keeps. A response's exponent (as
//! light^-phi) mixes by value too: it acts as a power, so its effect is
//! already geometric (host decision 19). `settings` lists every
//! setting of an age by name, its fields named one by one, so a setting
//! added to `PaState`, `Form` or `Zone` does not compile until it is listed
//! there, and one listed with no scale here is refused.
use super::{NodeLaw, PaState, Zone};

/// How a setting mixes between two species.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scale {
    /// By its value: an angle, a probability, a share or a count.
    Linear,
    /// By the log of its value above `floor`: a positive magnitude.
    Log { floor: f64 },
    /// By the share 1 / (1 + value) it keeps: a delay that may be infinite.
    Kept,
    /// As one share of a stop's three outcomes, carrying on, relaying or
    /// dying, the three mixed by value (host decision 12): `OUTCOMES`.
    Outcome,
}

use Scale::{Kept, Linear, Log, Outcome};

/// A stop's outcomes, by the settings that give them: the chance it
/// stops (abortion) or carries on (the others), and the relay's share of
/// a stop; the rest dies. A species reads best as these; a walk mixes the
/// three shares.
pub const OUTCOMES: &[(&str, &str)] = &[
    ("abortion", "relay"),
    ("continuation", "relay_ended"),
    ("viability", "relay_failed"),
];

/// Every setting's scale, by the name `settings` gives it.
pub const SCALES: &[(&str, Scale)] = &[
    ("lifespan", Log { floor: 1.0 }),
    ("continuation", Outcome),
    ("viability", Outcome),
    ("shedding", Kept),
    ("internode", Log { floor: 1e-3 }),
    ("insertion", Linear),
    ("divergence", Linear),
    ("abortion", Outcome),
    ("abortion_rise", Linear),
    ("relay", Outcome),
    ("relay_ended", Outcome),
    ("relay_failed", Outcome),
    ("relay_at", Linear),
    ("epitony", Linear),
    ("erection", Log { floor: 0.01 }),
    ("readiness", Linear),
    ("rhythm", Linear),
    ("straightening", Linear),
    // Phase E (fn-197): an area, a cost and a rate are magnitudes; a
    // response's exponent, a share and a balance mix by value.
    ("leaf_area", Log { floor: 1e-4 }),
    ("shade_hazard", Linear),
    ("shade_size", Linear),
    ("apical_control", Linear),
    ("upkeep", Log { floor: 1e-3 }),
    ("balance_hazard", Log { floor: 0.01 }),
    ("tolerance", Linear),
    ("retained", Linear),
    ("leaf_girth", Linear),
    ("form.tropism", Log { floor: 0.1 }),
    ("form.elevation", Linear),
    ("form.wander", Log { floor: 0.05 }),
    ("form.plane", Linear),
    ("form.pipe", Log { floor: 1e-5 }),
    ("form.exponent", Linear),
    ("form.ripening", Log { floor: 1.0 }),
    ("form.dominance", Linear),
    ("form.roll", Linear),
    ("form.sag", Log { floor: 1e-5 }),
    ("form.secondary", Linear),
    ("form.bend_length", Log { floor: 0.1 }),
    ("zone.nodes.min", Linear),
    ("zone.nodes.max", Linear),
    ("zone.buds", Linear),
    ("zone.lateral", Linear),
    ("zone.dormant", Linear),
    ("zone.delay", Log { floor: 0.1 }),
    ("zone.rate", Log { floor: 0.01 }),
];

/// The scale of the setting named `name`, if the table holds one.
pub fn scale(name: &str) -> Option<Scale> {
    SCALES.iter().find(|(n, _)| *n == name).map(|&(_, s)| s)
}

impl Scale {
    /// The value a share `t` of the way from `a` to `b` on this scale.
    pub fn mix(self, a: f64, b: f64, t: f64) -> f64 {
        // Each end is its species' own value, exactly.
        if t <= 0.0 || a == b {
            return a;
        }
        if t >= 1.0 {
            return b;
        }
        let lerp = |x: f64, y: f64| x + (y - x) * t;
        match self {
            Linear | Outcome => lerp(a, b),
            // Never below the lesser end, which rounding could leave.
            Log { floor } => {
                let mixed = lerp((a + floor).ln(), (b + floor).ln()).exp() - floor;
                mixed.max(a.min(b))
            }
            Kept => {
                let kept = |d: f64| 1.0 / (1.0 + d);
                let d = 1.0 / lerp(kept(a), kept(b)) - 1.0;
                if d.is_finite() {
                    d
                } else {
                    f64::INFINITY
                }
            }
        }
    }
}

/// Every scalar setting of an age by name, `PaState` and its `Form`: each
/// field named, so a new one must be listed to compile.
pub fn settings(state: &mut PaState) -> Vec<(&'static str, &mut f64)> {
    let PaState {
        lifespan,
        continuation,
        viability,
        zones: _,
        shedding,
        internode,
        insertion,
        divergence,
        abortion,
        abortion_rise,
        relay,
        relay_ended,
        relay_failed,
        relay_at,
        epitony,
        erection,
        readiness,
        rhythm,
        straightening,
        leaf_area,
        shade_hazard,
        shade_size,
        apical_control,
        upkeep,
        balance_hazard,
        tolerance,
        retained,
        leaf_girth,
        form,
    } = state;
    let super::Form {
        tropism,
        elevation,
        wander,
        plane,
        pipe,
        exponent,
        ripening,
        dominance,
        roll,
        sag,
        secondary,
        bend_length,
    } = form;
    vec![
        ("lifespan", lifespan),
        ("continuation", continuation),
        ("viability", viability),
        ("shedding", shedding),
        ("internode", internode),
        ("insertion", insertion),
        ("divergence", divergence),
        ("abortion", abortion),
        ("abortion_rise", abortion_rise),
        ("relay", relay),
        ("relay_ended", relay_ended),
        ("relay_failed", relay_failed),
        ("relay_at", relay_at),
        ("epitony", epitony),
        ("erection", erection),
        ("readiness", readiness),
        ("rhythm", rhythm),
        ("straightening", straightening),
        ("leaf_area", leaf_area),
        ("shade_hazard", shade_hazard),
        ("shade_size", shade_size),
        ("apical_control", apical_control),
        ("upkeep", upkeep),
        ("balance_hazard", balance_hazard),
        ("tolerance", tolerance),
        ("retained", retained),
        ("leaf_girth", leaf_girth),
        ("form.tropism", tropism),
        ("form.elevation", elevation),
        ("form.wander", wander),
        ("form.plane", plane),
        ("form.pipe", pipe),
        ("form.exponent", exponent),
        ("form.ripening", ripening),
        ("form.dominance", dominance),
        ("form.roll", roll),
        ("form.sag", sag),
        ("form.secondary", secondary),
        ("form.bend_length", bend_length),
    ]
}

/// Every scalar setting of a zone by name, its tables' entries each under
/// the table's name; none for a node law that is not uniform.
pub fn zone_settings(zone: &mut Zone) -> Option<Vec<(&'static str, &mut f64)>> {
    let Zone {
        nodes,
        buds,
        lateral,
        dormant,
        delay,
        rate,
    } = zone;
    let NodeLaw::Uniform { min, max } = nodes else {
        return None;
    };
    let mut out: Vec<(&'static str, &mut f64)> = vec![
        ("zone.nodes.min", min),
        ("zone.nodes.max", max),
        ("zone.buds", buds),
        ("zone.delay", delay),
        ("zone.rate", rate),
    ];
    out.extend(lateral.iter_mut().map(|p| ("zone.lateral", p)));
    out.extend(dormant.iter_mut().map(|p| ("zone.dormant", p)));
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{beech, palm};

    /// Every setting an age lists has a scale in the table, and the table
    /// names nothing an age does not list.
    #[test]
    fn every_setting_declares_its_scale() {
        let mut state = beech().states[9].clone();
        let mut names: Vec<&str> = settings(&mut state).into_iter().map(|(n, _)| n).collect();
        names.extend(
            zone_settings(&mut state.zones[0])
                .unwrap()
                .into_iter()
                .map(|(n, _)| n),
        );
        for name in &names {
            assert!(scale(name).is_some(), "{name} has no scale");
        }
        for (name, _) in SCALES {
            assert!(names.contains(name), "{name} is no setting");
        }
    }

    /// A magnitude mixes geometrically: the midpoint of the palm's stem
    /// pipe and the beech's branch pipe is their geometric mean, nearly.
    #[test]
    fn a_magnitude_mixes_on_its_log() {
        let (a, b) = (palm().states[2].form.pipe, beech().states[9].form.pipe);
        let mid = scale("form.pipe").unwrap().mix(a, b, 0.5);
        assert!((mid - ((a + 1e-5) * (b + 1e-5)).sqrt() + 1e-5).abs() < 1e-12);
        assert!(mid < (a + b) / 2.0 / 5.0, "{mid}");
    }
}
