//! Under its floor an element's faintness is rounded, not carried
//! (fn-356).
use super::cycle::{tree_digest, Paused, Retiring};
use crate::lineage::Key;
use crate::presence::rounded;
use crate::species::NodeLaw;
use crate::{oak, Light, Request, Species};
use std::sync::Arc;

/// The oak with every element whole: no death, abortion, relay,
/// shedding, sleeping bud or turn under shade, whole lifespans, buds and
/// node counts, and each zone's buds all of its first kind.
fn whole() -> Species {
    let mut species = oak();
    for state in &mut species.states {
        state.viability = 1.0;
        (state.shade_hazard, state.ground_hazard) = (0.0, 0.0);
        (state.abortion, state.abortion_rise) = (0.0, 0.0);
        (state.relay_failed, state.balance_hazard) = (0.0, 0.0);
        state.shedding = f64::INFINITY;
        state.lifespan = state.lifespan.round();
        state.shade_short = Vec::new();
        for zone in &mut state.zones {
            let n = zone.nodes.mean().round();
            zone.nodes = NodeLaw::Uniform { min: n, max: n };
            zone.buds = zone.buds.round();
            zone.dormant.iter_mut().for_each(|p| *p = 0.0);
            let first = zone.lateral.iter().position(|&p| p > 0.0);
            for (pa, p) in zone.lateral.iter_mut().enumerate() {
                *p = if Some(pa) == first { 1.0 } else { 0.0 };
            }
        }
    }
    species
}

/// A tree of whole elements, its every faintness 1, is the tree grown
/// before the rule, to the bit, lit, at 7.
#[test]
fn a_tree_of_whole_elements_is_as_it_was() {
    let species = Arc::new(whole());
    let light = Light {
        extinction: 0.5,
        sky: 0.5,
    };
    let digests = [1, 7].map(|seed| {
        let request = Request {
            age: 7,
            seed,
            light,
        };
        let mut paused = Paused::begin(species.clone(), request, Retiring::AtAge, true).unwrap();
        paused.grow_to(7).unwrap();
        let rounded = paused.census().unwrap().cycles.iter().map(|c| c.rounded);
        assert!(rounded.clone().all(|r| r == [0, 0]), "seed {seed}");
        let tree = paused.settle().unwrap();
        let laterals = tree.axes.iter().filter(|a| a.origin.parent().is_some());
        assert!(laterals.clone().all(|a| a.vigour == 1.0), "seed {seed}");
        tree_digest(&tree)
    });
    // Measured at 53c2baa62, before the rule.
    assert_eq!(digests, [5471803321526034208, 12893453816766107291]);
}

/// Over many keys an element under its floor is kept in the share
/// `faint / floor`, within four standard errors, and then stands at the
/// floor: never under it, and over it by an f32's step at most.
#[test]
fn a_faint_element_is_kept_in_its_share_and_stands_at_its_floor() {
    const KEYS: u64 = 200_000;
    for (faint, floor) in [(1e-3f32, 0.01), (0.2, 0.5), (0.049, 0.05), (1e-6, 0.9)] {
        let mut kept = 0u64;
        for k in 0..KEYS {
            if let Some(lift) = rounded(faint, floor, Key(k).child(7).draw()) {
                kept += 1;
                let stands = f64::from(faint * lift);
                assert!(
                    stands >= floor as f32 as f64,
                    "{faint} at {floor}: {stands}"
                );
                assert!(
                    stands <= floor * (1.0 + 1e-6),
                    "{faint} at {floor}: {stands}"
                );
            }
        }
        let p = f64::from(faint) / floor;
        let error = (p * (1.0 - p) / KEYS as f64).sqrt();
        let share = kept as f64 / KEYS as f64;
        assert!(
            (share - p).abs() < 4.0 * error,
            "{faint} at {floor}: {share} vs {p}"
        );
    }
}
