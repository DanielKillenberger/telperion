//! A growth unit lays `min(1, size x presence)` of its nodes (fn-356).
use super::{run, Options};
use crate::species::{NodeLaw, ALONG};
use crate::{oak, Light, Request, Species};
use std::collections::BTreeMap;

/// The oak with whole units of a fixed count: no death, abortion, relay,
/// shedding or sleeping bud, whole lifespans and buds, and every zone's
/// count its law's mean rounded. A unit's nodes then all stand whole at
/// its presence, so the scale of its fullest node is that presence.
fn whole_units() -> Species {
    let mut species = oak();
    for state in &mut species.states {
        state.viability = 1.0;
        (state.shade_hazard, state.ground_hazard) = (0.0, 0.0);
        (state.abortion, state.abortion_rise) = (0.0, 0.0);
        state.relay_failed = 0.0;
        state.balance_hazard = 0.0;
        state.shedding = f64::INFINITY;
        state.lifespan = state.lifespan.round();
        for zone in &mut state.zones {
            let n = zone.nodes.mean().round();
            zone.nodes = NodeLaw::Uniform { min: n, max: n };
            zone.buds = zone.buds.round();
            zone.dormant.iter_mut().for_each(|p| *p = 0.0);
        }
    }
    species
}

/// Each unit of the lit fixture lays its whole count where its size times
/// its presence is 1 or more, as it did before the rule, and otherwise
/// that share of them, whole, with one more at most growing in.
#[test]
fn a_unit_lays_the_share_its_size_and_presence_give() {
    let species = whole_units();
    let light = Light {
        extinction: 0.5,
        sky: 0.5,
    };
    let (mut whole, mut shared) = (0, 0);
    for seed in [1, 7] {
        let request = Request {
            age: 12,
            seed,
            light,
        };
        let tree = run(&species, request, &mut |_| {}, true, Options::default()).unwrap();
        for axis in &tree.structure.axes {
            let state = &species.states[axis.pa];
            let along = ALONG.iter().filter(|&&z| z < state.zones.len());
            let count: f64 = along.map(|&z| state.zones[z].nodes.mean()).sum();
            let mut units: BTreeMap<u32, (usize, f64, f64)> = BTreeMap::new();
            for p in &axis.phytomers {
                let unit = units.entry(p.cycle).or_insert((0, 0.0, p.size));
                unit.0 += 1;
                unit.1 = unit.1.max(p.scale);
            }
            for (cycle, (laid, presence, size)) in units {
                let q = (size * presence).min(1.0);
                let nodes = q * count;
                // Clear of a whole count by more than the f32 they are
                // laid in moves it.
                if (nodes - nodes.round()).abs() < 1e-3 || (q - 1.0).abs() < 1e-4 {
                    continue;
                }
                let at = format!("seed {seed} axis {} cycle {cycle}", axis.lineage);
                if q >= 1.0 {
                    whole += 1;
                    assert_eq!(laid as f64, count, "{at}: a whole unit");
                } else {
                    shared += 1;
                    let most = nodes.floor() + 1.0;
                    assert!(laid as f64 <= most, "{at}: {laid} laid at {q} of {count}");
                }
            }
        }
    }
    assert!(whole > 0 && shared > 0, "{whole} whole, {shared} shared");
}
