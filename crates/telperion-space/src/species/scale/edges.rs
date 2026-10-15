//! The table's bounds are the ones `validate` enforced before it read them
//! from the table (fn-284): `BEFORE` records each single-range check as it
//! was written, and each edge it names still admits on its side and still
//! refuses, by the setting's name, one step past it.
use super::{setting, settings, zone_settings, SETTINGS};
use crate::error::Error;
use crate::{beech, Species};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// One check as written before: its setting, `[low, high]`, whether `low`
/// itself was refused, and which edges the setting's own check enforced (a
/// zone's node counts hold one edge each by their relation).
type Before = (&'static str, [f64; 2], bool, [bool; 2]);
const BOTH: [bool; 2] = [true, true];
const INF: f64 = f64::INFINITY;

const BEFORE: &[Before] = &[
    ("lifespan", [0.0, 1e6], false, BOTH),
    ("continuation", [0.0, 1.0], false, BOTH),
    ("viability", [0.0, 1.0], false, BOTH),
    ("shedding", [0.0, INF], false, BOTH),
    ("internode", [0.0, 100.0], true, BOTH),
    ("insertion", [0.0, PI], false, BOTH),
    ("divergence", [-TAU, TAU], false, BOTH),
    ("abortion", [0.0, 1.0], false, BOTH),
    ("abortion_rise", [0.0, 8.0], false, BOTH),
    ("relay", [0.0, 1.0], false, BOTH),
    ("relay_ended", [0.0, 1.0], false, BOTH),
    ("relay_failed", [0.0, 1.0], false, BOTH),
    ("relay_at", [0.0, 1.0], false, BOTH),
    ("epitony", [0.0, 1.0], false, BOTH),
    ("takeover", [0.0, 1.0], false, BOTH),
    ("erection", [0.0, 100.0], false, BOTH),
    ("readiness", [0.0, 1.0], false, BOTH),
    ("rhythm", [0.0, 1.0], false, BOTH),
    ("straightening", [0.0, 1.0], false, BOTH),
    ("leaf_area", [0.0, 5.0], false, BOTH),
    ("flowering", [0.0, 1.0], false, BOTH),
    ("cluster_leaves", [0.0, 8.0], false, BOTH),
    ("cluster_spread", [0.0, FRAC_PI_2], false, BOTH),
    ("leaf_years", [0.0, INF], false, BOTH),
    ("elongation", [0.0, 365.0], false, BOTH),
    ("expansion", [0.0, 365.0], false, BOTH),
    ("leaf_life", [0.0, INF], false, BOTH),
    ("budburst", [0.0, 365.0], false, BOTH),
    ("senescence", [0.0, 365.0], false, BOTH),
    ("colouring", [0.0, 365.0], false, BOTH),
    ("leaf_fall", [0.0, 365.0], false, BOTH),
    ("shade_hazard", [0.0, 10.0], false, BOTH),
    ("ground_hazard", [0.0, 1.0], false, BOTH),
    ("shade_size", [0.0, 10.0], false, BOTH),
    ("apical_control", [0.0, 1.0], false, BOTH),
    ("upkeep", [0.0, 10.0], false, BOTH),
    ("balance_hazard", [0.0, 10.0], false, BOTH),
    ("tolerance", [-1.0, 1.0], false, BOTH),
    ("retained", [0.0, 1.0], false, BOTH),
    ("leaf_girth", [0.0, 10.0], false, BOTH),
    ("form.tropism", [0.0, 100.0], false, BOTH),
    ("form.elevation", [-FRAC_PI_2, FRAC_PI_2], false, BOTH),
    ("form.wander", [0.0, 100.0], false, BOTH),
    ("form.plane", [-TAU, TAU], false, BOTH),
    ("form.pipe", [0.0, 1.0], false, BOTH),
    ("form.exponent", [1.5, 4.0], false, BOTH),
    ("form.ripening", [0.0, 1000.0], false, BOTH),
    ("form.dominance", [0.0, 1.0], false, BOTH),
    ("form.roll", [0.0, PI], false, BOTH),
    ("form.sag", [0.0, 1000.0], false, BOTH),
    ("form.secondary", [0.0, 1.0], false, BOTH),
    ("form.bend_length", [0.0, 1000.0], false, BOTH),
    ("form.internode_spread", [0.0, 1.0], false, BOTH),
    ("form.insertion_spread", [0.0, FRAC_PI_2], false, BOTH),
    ("form.tropism_spread", [0.0, 1.0], false, BOTH),
    ("zone.nodes.min", [0.0, 1000.0], false, [true, false]),
    ("zone.nodes.max", [0.0, 1000.0], false, [false, true]),
    ("zone.buds", [1.0, 6.0], false, BOTH),
    ("zone.lateral", [0.0, 1.0], false, BOTH),
    ("zone.dormant", [0.0, 1.0], false, BOTH),
    ("zone.delay", [0.0, 1000.0], false, BOTH),
    ("zone.rate", [0.0, 100.0], false, BOTH),
];

/// The first age the beech grows in, with a zone.
fn age(species: &Species) -> usize {
    (species.states.iter())
        .position(|s| s.lifespan > 0.0 && !s.zones.is_empty())
        .expect("a grown age")
}

/// The beech with the setting `name` of its first grown age at `v` (a zone
/// setting in its first zone, a table's last entry), and its refusal.
fn refusal(name: &str, v: f64) -> Option<(String, &'static str)> {
    let mut species = beech();
    let k = age(&species);
    let state = &mut species.states[k];
    let slot = match name.strip_prefix("zone.") {
        Some(_) => (zone_settings(&mut state.zones[0]).expect("a uniform zone"))
            .into_iter()
            .filter(|(n, _)| *n == name)
            .last(),
        None => settings(state).into_iter().find(|(n, _)| *n == name),
    };
    *slot.expect("a setting").1 = v;
    match species.validate() {
        Err(Error::Refused { input, reason }) => Some((input, reason)),
        _ => None,
    }
}

/// Whether a refusal names the setting: `states[3].zones[0].lateral[14]`
/// names `zone.lateral`, its indices aside.
fn names(input: &str, name: &str) -> bool {
    let tail = name.strip_prefix("zone.").unwrap_or(name);
    let bare: String = input
        .split('[')
        .map(|p| p.split_once(']').map_or(p, |(_, r)| r))
        .collect();
    bare.ends_with(&format!(".{tail}"))
}

#[test]
fn the_table_holds_the_bounds_validate_enforced() {
    assert_eq!(BEFORE.len(), SETTINGS.len());
    for &(name, [low, high], low_open, _) in BEFORE {
        let s = setting(name).unwrap_or_else(|| panic!("{name} has no row"));
        assert_eq!((s.low, s.high, s.low_open), (low, high, low_open), "{name}");
    }
}

#[test]
fn validate_refuses_at_the_edges_it_always_did() {
    for &(name, [low, high], low_open, [at_low, at_high]) in BEFORE {
        let edges = [
            (at_low, low, low.next_down(), low_open),
            (at_high, high, high.next_up(), false),
        ];
        for (enforced, edge, past, open) in edges {
            if !enforced || !edge.is_finite() {
                continue;
            }
            let outside = refusal(name, past).unwrap_or_else(|| panic!("{name} admits {past}"));
            assert!(names(&outside.0, name), "{name} at {past}: {outside:?}");
            let inside = refusal(name, if open { edge.next_up() } else { edge });
            assert_ne!(inside, Some(outside), "{name} refuses its own edge {edge}");
        }
    }
}
