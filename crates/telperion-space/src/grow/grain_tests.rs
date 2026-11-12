//! The grower does not grow what is under its grain (fn-356).
use super::cycle::{tree_digest, Paused, Retiring};
use super::{run, Options};
use crate::geometry::dominance;
use crate::presence::{Windows, GRAIN};
use crate::species::NodeLaw;
use crate::structure::Origin;
use crate::{oak, Light, Request, Species};
use std::sync::Arc;

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

/// The lit oak at 5 stands over the grain at every cycle (from its sixth
/// or eighth some tips fall under it): its every tip
/// reads more than it, and the tree is the one grown before the rule.
#[test]
fn a_tree_whose_every_tip_stands_over_the_grain_is_as_it_was() {
    let species = Arc::new(oak());
    let digests = [1, 7].map(|seed| {
        let request = Request {
            age: 5,
            seed,
            light: LIT,
        };
        let mut paused = Paused::begin(species.clone(), request, Retiring::AtAge, true).unwrap();
        paused.grow_to(5).unwrap();
        for c in &paused.census().unwrap().cycles {
            assert_eq!(c.under_grain, 0, "seed {seed} cycle {}", c.cycle);
        }
        tree_digest(&paused.settle().unwrap())
    });
    // Measured at d29f29cc5, before the rule.
    assert_eq!(digests, [14144581626978832127, 17731531182853926843]);
}

/// The oak with every draw that makes a tip whole, no death, abortion,
/// relay, shedding or sleeping bud, whole node counts, and its lateral
/// kinds' dominance whole, so a lateral of the trunk stands at its
/// dominance draw's fourth power: the trunk's laterals under the grain at
/// their first unit grow nothing, and those over it grow.
fn faint_laterals() -> Species {
    let mut species = oak();
    for (k, state) in species.states.iter_mut().enumerate() {
        state.viability = 1.0;
        (state.shade_hazard, state.ground_hazard) = (0.0, 0.0);
        (state.abortion, state.abortion_rise) = (0.0, 0.0);
        (state.relay_failed, state.balance_hazard) = (0.0, 0.0);
        state.shedding = f64::INFINITY;
        state.lifespan = state.lifespan.round();
        state.shade_short = Vec::new();
        state.form.dominance = if k > 0 { 1.0 } else { 0.0 };
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

#[test]
fn a_tip_under_the_grain_is_not_grown() {
    const AGE: u32 = 7;
    let species = faint_laterals();
    let windows = Windows::new(&species, AGE);
    let (mut under, mut over) = (0, 0);
    for seed in [1, 7] {
        let request = Request {
            age: AGE,
            seed,
            light: LIT,
        };
        let tree = run(&species, request, &mut |_| {}, true, Options::default()).unwrap();
        for axis in &tree.structure.axes {
            let Origin::Lateral { parent: 0, .. } = axis.origin else {
                continue;
            };
            if axis.birth >= AGE {
                continue;
            }
            let share = dominance(species.states[axis.pa].form.dominance, axis.lineage);
            let start = f64::from(windows.start(axis.pa, axis.birth + 1));
            let reads = axis.vigour * share * start;
            let at = format!("seed {seed} axis {}: {reads:e}", axis.lineage);
            if reads < 0.5 * GRAIN {
                under += 1;
                assert!(axis.phytomers.is_empty(), "{at} grew");
            } else if reads > 2.0 * GRAIN {
                over += 1;
                assert!(!axis.phytomers.is_empty(), "{at} grew nothing");
            }
        }
    }
    assert!(under > 0 && over > 0, "{under} under, {over} over");
}
