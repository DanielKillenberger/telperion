//! A growth unit's tables (fn-209 F1, task 2): what the GPU executor's
//! unit kernel reads, computed once per species point on the CPU from the
//! grower's own rules and handed to it as integers (FINE-CONTRACT.md D2,
//! D3 and D18's node presences), so the kernel draws a unit's nodes, their
//! order and their buds as `grow_unit` does.
use crate::error::Result;
use crate::fixed::presence::q;
use crate::fixed::{grid, q32, q62, ticks};
use crate::grow::alone::rules;
use crate::grow::node_wood;
use crate::lineage::{threshold, ONE};
use crate::presence::{FLOOR, RATE, SPAN};
use crate::species::{NodeLaw, Species, ALONG, MAX_NODES_PER_ZONE};

/// Every PA's unit tables for a tree of one age.
#[derive(Debug, Clone)]
pub struct Units {
    /// Per PA, the zones a unit draws, in their order along it.
    pub zones: Vec<Vec<UnitZone>>,
    pub age: u32,
}

/// One zone of a PA's growth unit.
#[derive(Debug, Clone)]
pub struct UnitZone {
    /// Its index, which keys its draws (`ZONE + zone`).
    pub zone: usize,
    pub poisson: bool,
    /// Node `j` stands while the zone's draw is at or above
    /// `thresholds[j]` and every one before it (D2).
    pub thresholds: Vec<u64>,
    /// Node `j`'s bound, Q62, its lead's (D18).
    pub bounds: Vec<u64>,
    /// A node's bud places.
    pub places: usize,
    /// Per PA, the threshold of the running sum of the zone's lateral
    /// probabilities, or none where the PA's own probability is none (D3).
    pub laterals: Vec<Option<u64>>,
    /// Per cycle, from 0 to the tree's age, a node's window, Q32.
    pub windows: Vec<i64>,
    /// Per PA, the threshold of the running sum of the zone's sleeping-bud
    /// probabilities, or none (D3, `dormant::draw`).
    pub dormant: Vec<Option<u64>>,
    /// A sleeping bud's delay in ticks and its years a nat (Q32), where
    /// it wakes at all (D11).
    pub wake: Option<(u64, i64)>,
    /// Per cycle, whether a node grown then may hold a sleeping bud that
    /// is expected to grow: `grow_unit`'s gate on `sleep`.
    pub sleeps: Vec<bool>,
    /// Host decision 8: each bud place's share, Q32 (`Zone::share`), and
    /// per cycle and PA a sleeping bud's window, Q32 (`decide::node`).
    pub placed: Vec<u64>,
    pub sleep_windows: Vec<Vec<i64>>,
}

impl Units {
    /// The tables of `species` grown to `age`, for a tree no bud of which
    /// reads light (the fine path's).
    pub fn new(species: &Species, age: u32) -> Result<Self> {
        let rules = rules(species, age, 0)?;
        let mut sleeping = Vec::new();
        let zones = (0..species.states.len())
            .map(|pa| {
                let state = &species.states[pa];
                let along = ALONG.iter().filter(|&&z| z < state.zones.len());
                along
                    .map(|&z| {
                        let zone = &state.zones[z];
                        let (poisson, thresholds, bounds) = law(zone.nodes);
                        let mut low = 0.0;
                        let laterals = rules.laterals[pa][z]
                            .iter()
                            .map(|&p| {
                                let high = low + p;
                                low = high;
                                (p > 0.0).then(|| threshold(high))
                            })
                            .collect();
                        let window = |wood: f64| q32((RATE * wood).clamp(FLOOR, SPAN));
                        let mut sleep_windows = Vec::new();
                        let (windows, sleeps) = (0..=age)
                            .map(|cycle| {
                                let wood = node_wood(&rules, (pa, z), cycle, &mut sleeping);
                                let sleeps = sleeping.iter().any(|&wood| wood > 0.0);
                                sleep_windows.push(sleeping.iter().map(|&w| window(w)).collect());
                                (window(wood), sleeps)
                            })
                            .unzip();
                        let mut low = 0.0;
                        let dormant = zone
                            .dormant
                            .iter()
                            .map(|&p| {
                                let high = low + p;
                                low = high;
                                (p > 0.0).then(|| threshold(high))
                            })
                            .collect();
                        let wake = (zone.rate > 0.0)
                            .then(|| (ticks(grid(zone.delay)), q32(1.0 / zone.rate)));
                        UnitZone {
                            zone: z,
                            poisson,
                            thresholds,
                            bounds,
                            places: zone.places(),
                            laterals,
                            windows,
                            dormant,
                            wake,
                            sleeps,
                            placed: (0..zone.places()).map(|k| q(zone.share(k))).collect(),
                            sleep_windows,
                        }
                    })
                    .collect()
            })
            .collect();
        Ok(Self { zones, age })
    }
}

/// A node law's thresholds and bounds, as `lineage::nodes` walks them.
fn law(law: NodeLaw) -> (bool, Vec<u64>, Vec<u64>) {
    let (mut thresholds, mut bounds) = (Vec::new(), Vec::new());
    match law {
        NodeLaw::Uniform { min, max } => {
            let span = max - min + 1.0;
            for j in 0..(max.ceil() as u32).min(MAX_NODES_PER_ZONE) {
                let bound = (f64::from(j) - min + 1.0) / span;
                thresholds.push(threshold(bound));
                bounds.push(q62(bound));
            }
            (false, thresholds, bounds)
        }
        NodeLaw::Poisson { mean } => {
            let mut mass = (-mean).exp();
            let mut cumulative = mass;
            for k in 0..MAX_NODES_PER_ZONE {
                let t = threshold(cumulative);
                thresholds.push(t);
                bounds.push(q62(cumulative));
                // No draw passes a certain bound: the count ends here.
                if t >= ONE {
                    break;
                }
                mass *= mean / f64::from(k + 1);
                cumulative += mass;
            }
            (true, thresholds, bounds)
        }
    }
}
