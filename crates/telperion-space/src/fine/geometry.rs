//! The GPU executor's surface for a fine root's geometry (fn-209 F1,
//! tasks 4 and 6): a species point's continuous values as scale, girth
//! and lay read them (`geometry.rs`, `girth.rs`, `geometry/lay.rs`,
//! `sag.rs`), the zones' thresholds the presences' leads are taken from,
//! and a bearer's laid frame at a fine root's node. Geometry decides no
//! topology, so it is held to fn-125's screen, never to the bit.
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
}

/// One zone of a PA's growth unit: its buds a node, and per PA a lateral
/// (or a sleeping bud) of it is drawn under, the thresholds of the running
/// sums below and above its run of draws (`lineage::bud`), which its lead
/// is read from.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ZoneGeometry {
    pub buds: f64,
    pub laterals: Vec<[u64; 2]>,
    pub dormant: Vec<[u64; 2]>,
}

/// A species point's continuous values for a fine root's geometry.
#[derive(Debug, Clone)]
pub struct GeometryTables {
    pub age: u32,
    pub pas: Vec<PaGeometry>,
    /// Per PA, the zones in their order along a unit (`Units`).
    pub zones: Vec<Vec<ZoneGeometry>>,
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
    pub fn new(species: &Species, age: u32) -> Result<Self> {
        let rules = rules(species, age, 0)?;
        let pas = (0..species.states.len())
            .map(|pa| {
                let s = &species.states[pa];
                let f = s.form;
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
                }
            })
            .collect();
        let zones = (0..species.states.len())
            .map(|pa| {
                let state = &species.states[pa];
                let along = ALONG.iter().filter(|&&z| z < state.zones.len());
                along
                    .map(|&z| {
                        let zone = &state.zones[z];
                        ZoneGeometry {
                            buds: zone.buds,
                            laterals: bounds(&rules.laterals[pa][z]),
                            dormant: bounds(&zone.dormant),
                        }
                    })
                    .collect()
            })
            .collect();
        Ok(Self { age, pas, zones })
    }
}

/// The bearer's laid frame at a fine root's node: the point below the
/// node, its tip, heading and side, its phytomer's rank, scale and size,
/// and the phase of its axis's phyllotaxis (`geometry::phase`, fn-199).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BearerFrame {
    pub below: Vec3,
    pub tip: Vec3,
    pub heading: Vec3,
    pub side: Vec3,
    pub rank: f64,
    pub scale: f64,
    pub size: f64,
    pub phase: f64,
}

/// A fine root's attachment as its geometry reads it: its bud's vigour as
/// made and the buds its node holds a whorl, its bearer's PA, and its
/// bearer's frame before bending and as laid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RootGeometry {
    pub vigour: f64,
    pub whorl: f64,
    pub bearer_pa: usize,
    pub unbent: BearerFrame,
    pub bent: BearerFrame,
}

/// The laid frame of axis `bearer` of `tree` at its node `node`, as a
/// fine root standing there reads it (realized mechanics), each PA's
/// divergence `divergence`.
pub fn bearer_frame(
    tree: &Structure,
    divergence: &[f64],
    bearer: usize,
    node: usize,
) -> BearerFrame {
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
        phase: crate::geometry::phase_by(&tree.axes, &|pa| divergence[pa], bearer),
    }
}
