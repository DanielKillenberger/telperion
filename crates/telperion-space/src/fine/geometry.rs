//! The GPU executor's surface for a fine root's geometry (fn-209 F1, task
//! 4): a species point's continuous values as the presences, scale, girth
//! and lay read them (`presence.rs`, `geometry.rs`, `bounded/girth.rs`,
//! `geometry/lay.rs`, `sag.rs`), each in the form an f32 executor computes
//! it from without cancellation (logs and log-odds taken here in f64), and
//! each fine root's attachment as its record holds it. Geometry decides no
//! topology, so it is held to fn-125's screen, never to the bit.
use super::CommittedFine;
use crate::bounded::Expected;
use crate::error::Result;
use crate::grow::alone::rules;
use crate::lineage::threshold;
use crate::species::{Species, ALONG};
use crate::structure::{Structure, Vec3};

/// One PA's continuous values.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct PaGeometry {
    pub internode: f64,
    pub insertion: f64,
    pub divergence: f64,
    pub relay_at: f64,
    pub epitony: f64,
    pub straightening: f64,
    pub erection: f64,
    pub tropism: f64,
    pub elevation: f64,
    pub wander: f64,
    pub plane: f64,
    pub pipe: f64,
    pub exponent: f64,
    pub ripening: f64,
    pub dominance: f64,
    pub roll: f64,
    pub sag: f64,
    pub secondary: f64,
    pub bend_length: f64,
    pub retained: f64,
    /// The share of a bud of it that reaches the age it grows in
    /// (`Species::lived`).
    pub reach: f64,
    /// Its move's chance, the reach of its successor included; none (0)
    /// where it has none.
    pub go: f64,
    /// Its relays' chances (by abortion, end, failure).
    pub relay: [f64; 3],
    /// Its lifespan, in cycles on the tick grid.
    pub life: f64,
}

/// One zone of a PA's growth unit: its buds a node, and per PA a lateral
/// (or a sleeping bud) of it is drawn under, the thresholds of the running
/// sums below and above its run of draws (`lineage::bud`), which its lead
/// is read from; and, per cycle
/// from 0 to the age, the wood each PA's sleeping bud on a node grown then
/// is expected to grow.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ZoneGeometry {
    pub buds: f64,
    pub laterals: Vec<[u64; 2]>,
    pub dormant: Vec<[u64; 2]>,
    pub sleeping: Vec<Vec<f64>>,
}

/// A species point's continuous values for a fine root's geometry.
#[derive(Debug, Clone)]
pub struct GeometryTables {
    pub age: u32,
    pub pas: Vec<PaGeometry>,
    /// Per PA, the zones in their order along a unit (`Units`).
    pub zones: Vec<Vec<ZoneGeometry>>,
    /// Per PA, per cycle 0 to the age plus one, the wood a bud of it is
    /// expected to grow from then, as a share of the tree's
    /// (`Windows::wood`).
    pub woods: Vec<Vec<f64>>,
    /// The committed rows by `pa * (age + 1) + m` for a lateral `m` cycles
    /// from the tree's age: kept, s1, s2, grown1, grown2, mass, moment
    /// and e (none, 0, where no bud of the PA is borne).
    pub rows: Vec<[f64; 8]>,
}

/// The thresholds of the running sums below and above each PA's run.
fn bounds(probabilities: &[f64]) -> Vec<[u64; 2]> {
    let mut low = 0.0;
    probabilities
        .iter()
        .map(|&p| {
            let high = low + p;
            let out = [threshold(low), threshold(high)];
            low = high;
            out
        })
        .collect()
}

impl GeometryTables {
    /// The tables of `species` grown to `age`, for a tree no bud of which
    /// reads light.
    pub fn new(species: &Species, age: u32, expected: &Expected) -> Result<Self> {
        let rules = rules(species, age, 0)?;
        let windows = &rules.windows;
        let pas = (0..species.states.len())
            .map(|pa| {
                let s = &species.states[pa];
                let f = s.form;
                let go = species.successor(pa).map_or(0.0, |(_, go)| go);
                PaGeometry {
                    internode: s.internode,
                    insertion: s.insertion,
                    divergence: s.divergence,
                    relay_at: s.relay_at,
                    epitony: s.epitony,
                    straightening: s.straightening,
                    erection: s.erection,
                    tropism: f.tropism,
                    elevation: f.elevation,
                    wander: f.wander,
                    plane: f.plane,
                    pipe: f.pipe,
                    exponent: f.exponent,
                    ripening: f.ripening,
                    dominance: f.dominance,
                    roll: f.roll,
                    sag: f.sag,
                    secondary: f.secondary,
                    bend_length: f.bend_length,
                    retained: s.retained,
                    reach: species.lived(pa).map_or(0.0, |l| l.1),
                    go,
                    relay: [s.relay, s.relay_ended, s.relay_failed],
                    life: s.life(),
                }
            })
            .collect();
        let mut woods = Vec::new();
        let zones = (0..species.states.len())
            .map(|pa| {
                let state = &species.states[pa];
                let along = ALONG.iter().filter(|&&z| z < state.zones.len());
                along
                    .map(|&z| {
                        let zone = &state.zones[z];
                        let sleeping = (0..=age)
                            .map(|cycle| {
                                windows.sleeping(zone, cycle, &mut woods);
                                woods.clone()
                            })
                            .collect();
                        ZoneGeometry {
                            buds: zone.buds,
                            laterals: bounds(&rules.laterals[pa][z]),
                            dormant: bounds(&zone.dormant),
                            sleeping,
                        }
                    })
                    .collect()
            })
            .collect();
        let woods = (0..species.states.len())
            .map(|pa| {
                // Cycle 0 grows nothing and reads none.
                (0..=age + 1)
                    .map(|c| if c == 0 { 0.0 } else { windows.wood(pa, c) })
                    .collect()
            })
            .collect();
        let held = expected.rows();
        let rows = (0..species.states.len())
            .flat_map(|pa| (0..=age).map(move |m| (pa, m)))
            .map(|(pa, m)| match held.row_at(pa, m as usize) {
                Some(r) => [
                    r.kept, r.s1, r.s2, r.grown1, r.grown2, r.mass, r.moment, r.e,
                ],
                None => [0.0; 8],
            })
            .collect();
        Ok(Self {
            age,
            pas,
            zones,
            woods,
            rows,
        })
    }
}

/// The bearer's laid frame at a fine root's node: the point below the
/// node, its tip, heading and side, and its phytomer's rank, scale and
/// size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BearerFrame {
    pub below: Vec3,
    pub tip: Vec3,
    pub heading: Vec3,
    pub side: Vec3,
    pub rank: f64,
    pub scale: f64,
    pub size: f64,
}

/// A fine root's attachment as its geometry reads it: its bud's vigour as
/// made and the buds its node holds a whorl, its bearer's PA, and its
/// bearer's frame before bending and as laid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RootGeometry {
    /// Grown on the CPU reference executor (host decision 3).
    pub grounded: bool,
    pub vigour: f64,
    pub whorl: f64,
    pub bearer_pa: usize,
    pub unbent: BearerFrame,
    pub bent: BearerFrame,
}

/// Each fine root's attachment, in record order.
pub fn roots_geometry(fine: &CommittedFine) -> Vec<RootGeometry> {
    fine.roots
        .iter()
        .map(|(root, ..)| {
            let a = &root.attachment;
            let whorl = match root.record.origin {
                crate::structure::Origin::Lateral { whorl, .. } => whorl,
                _ => unreachable!("a fine root is a lateral bud"),
            };
            let frame = |f: &super::record::Frame| BearerFrame {
                below: f.below,
                tip: f.at.tip,
                heading: f.at.heading,
                side: f.at.side,
                rank: f.at.rank,
                scale: f.at.scale,
                size: f.at.size,
            };
            RootGeometry {
                grounded: root.grounded,
                vigour: root.record.vigour,
                whorl,
                bearer_pa: a.pa,
                unbent: frame(&a.unbent),
                bent: frame(&a.bent),
            }
        })
        .collect()
}

/// The laid frame of axis `bearer` of `tree` at its node `node`, as a
/// fine root standing there reads it (realized mechanics).
pub fn bearer_frame(tree: &Structure, bearer: usize, node: usize) -> BearerFrame {
    let axis = &tree.axes[bearer];
    let at = &axis.phytomers[node];
    BearerFrame {
        below: node
            .checked_sub(1)
            .map_or(axis.base, |n| axis.phytomers[n].tip),
        tip: at.tip,
        heading: at.heading,
        side: at.side,
        rank: at.rank,
        scale: at.scale,
        size: at.size,
    }
}
