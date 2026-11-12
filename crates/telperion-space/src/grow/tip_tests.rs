//! A tip whose unit is less than one node ends (fn-356).
use super::{run, Grown, Options};
use crate::geometry::dominance;
use crate::species::{NodeLaw, ALONG};
use crate::structure::Origin;
use crate::{oak, Light, Request, Species};

/// Young enough that a tree of certain buds and no death stays small.
const AGE: u32 = 7;

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

/// The oak with whole units of a fixed count, lit: no death, abortion,
/// relay, shedding, sleeping bud or turn under shade, whole lifespans and
/// buds, each zone's count its law's mean rounded, and each zone's buds
/// all of its first kind: every draw that makes a tip is whole.
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

fn grown(species: &Species, seed: u64) -> Grown {
    let request = Request {
        age: AGE,
        seed,
        light: LIT,
    };
    run(species, request, &mut |_| {}, true, Options::default()).unwrap()
}

/// The nodes a unit of `pa` lays in `species`, whole.
fn nodes(species: &Species, pa: usize) -> f64 {
    let state = &species.states[pa];
    let along = ALONG.iter().filter(|&&z| z < state.zones.len());
    along.map(|&z| state.zones[z].nodes.mean()).sum()
}

/// Where every tip stands whole, every unit at a node or more, the tree
/// is the one grown before the rule, to the bit.
#[test]
fn a_tree_whose_every_tip_lays_a_node_is_as_it_was() {
    let species = whole();
    let digests = [1, 7].map(|seed| {
        let tree = grown(&species, seed).structure;
        let laterals = tree.axes.iter().filter(|a| a.origin.parent().is_some());
        assert!(laterals.clone().count() > 0);
        assert!(laterals.clone().all(|a| a.vigour == 1.0), "seed {seed}");
        super::cycle::tree_digest(&tree)
    });
    // Measured at 6da2ff9a3, before the rule.
    assert_eq!(digests, [8086766086517289291, 16931195750158535152]);
}

/// The lateral kinds' dominance at 0.99, so most laterals the trunk bears
/// stand at a few hundredths: one that lays under a quarter of a node
/// ends within a unit or two, and one at a node or more grows every unit
/// its age gives it.
#[test]
fn a_tip_under_a_node_ends() {
    let mut species = whole();
    for state in &mut species.states[1..] {
        state.form.dominance = 0.99;
    }
    let (mut faint, mut whole) = ([0.0; 2], [0.0; 2]);
    for seed in [1, 7] {
        let tree = grown(&species, seed).structure;
        for axis in &tree.axes {
            let Origin::Lateral { parent: 0, .. } = axis.origin else {
                continue;
            };
            let state = &species.states[axis.pa];
            let share = dominance(state.form.dominance, axis.lineage);
            let lays = axis.vigour * share * nodes(&species, axis.pa);
            let could = state.lifespan.min(f64::from(AGE - axis.birth));
            let mut cycles: Vec<u32> = axis.phytomers.iter().map(|p| p.cycle).collect();
            cycles.dedup();
            let units = [cycles.len() as f64, could];
            match lays {
                l if l < 0.25 => (0..2).for_each(|k| faint[k] += units[k]),
                l if l >= 1.0 => (0..2).for_each(|k| whole[k] += units[k]),
                _ => {}
            }
        }
    }
    assert!(faint[1] > 20.0 && whole[1] > 20.0, "{faint:?} {whole:?}");
    assert_eq!(whole[0], whole[1], "every unit of a whole tip grows");
    assert!(faint[0] < 0.5 * faint[1], "{faint:?}: faint tips grow on");
}
