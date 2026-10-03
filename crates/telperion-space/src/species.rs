//! A species as the engine reads it: the reference axis, an ordered list of
//! physiological ages (PA), each a state of the bud automaton. Index 0 is the
//! youngest PA, the botanists' PA 1 (the trunk's). A bud's PA only ages: a
//! lateral bud carries its parent's PA or an older one, and an apex moves
//! along the reference axis towards older PAs (AmapSim's oriented automaton;
//! GreenLab's dual-scale automaton, de Reffye et al. 2021).
use crate::error::{refuse, Result};
use std::f64::consts::PI;

/// The most buds one node carries: a whorl of six.
pub const MAX_BUDS: u8 = 6;
const MAX_NODES_PER_ZONE: u32 = 1_000;
const MAX_MEAN_NODES: f64 = 500.0;
const MAX_STATES: usize = 64;

#[derive(Debug, Clone, PartialEq)]
pub struct Species {
    /// The reference axis, youngest PA first.
    pub states: Vec<PaState>,
}

/// One physiological age: how an apex in it grows, lives and branches.
#[derive(Debug, Clone, PartialEq)]
pub struct PaState {
    /// Growth units (one per cycle) an apex spends in this PA before it moves on.
    pub lifespan: u32,
    /// The PA the apex turns into after `lifespan` growth units; none, it stops.
    pub next: Option<usize>,
    /// The probability that the apex survives each cycle, tested before it grows.
    pub viability: f64,
    /// The growth unit, base to top: its zones of nodes and their lateral buds.
    pub zones: Vec<Zone>,
    /// Cycles a lateral axis of this PA keeps with no living apex before it is shed;
    /// none, it is kept.
    pub shedding: Option<u32>,
    /// The internode length in metres.
    pub internode: f64,
    /// The angle in radians between an axis of this PA and its parent at insertion.
    pub insertion: f64,
    /// The angle in radians between successive nodes of this PA (phyllotaxis).
    pub divergence: f64,
}

/// A zone of a growth unit: its node count and the PA of each node's buds.
/// Within a zone the nodes stand acrotonically: the youngest lateral PA on
/// top, bare nodes at the base.
#[derive(Debug, Clone, PartialEq)]
pub struct Zone {
    pub nodes: NodeLaw,
    /// Lateral buds per node, each drawn independently (1 alternate, 2 opposite).
    pub buds: u8,
    /// The probability that a bud carries each PA; the remainder is bare.
    pub lateral: Vec<f64>,
}

/// The law of a zone's node count per growth unit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeLaw {
    /// Uniform on `min..=max`; `min == max` is deterministic.
    Uniform {
        min: u32,
        max: u32,
    },
    Poisson {
        mean: f64,
    },
}

impl NodeLaw {
    pub fn mean(self) -> f64 {
        match self {
            NodeLaw::Uniform { min, max } => (min as f64 + max as f64) / 2.0,
            NodeLaw::Poisson { mean } => mean,
        }
    }
}

impl Species {
    /// Refuses, by name, every input the engine cannot draw.
    pub fn validate(&self) -> Result<()> {
        let count = self.states.len();
        if count == 0 || count > MAX_STATES {
            return refuse(
                "states",
                "the reference axis holds 1 to 64 physiological ages",
            );
        }
        for (pa, state) in self.states.iter().enumerate() {
            state.validate(&format!("states[{pa}]"), pa, count)?;
        }
        Ok(())
    }
}

impl PaState {
    fn validate(&self, at: &str, pa: usize, count: usize) -> Result<()> {
        if self.lifespan == 0 {
            return refuse(
                format!("{at}.lifespan"),
                "an apex grows at least one growth unit",
            );
        }
        if let Some(next) = self.next {
            if next < pa || next >= count {
                return refuse(
                    format!("{at}.next"),
                    "an apex ages along the reference axis",
                );
            }
        }
        if !(0.0..=1.0).contains(&self.viability) {
            return refuse(format!("{at}.viability"), "a probability lies in 0 to 1");
        }
        if self.zones.is_empty() {
            return refuse(
                format!("{at}.zones"),
                "a growth unit holds at least one zone",
            );
        }
        for (z, zone) in self.zones.iter().enumerate() {
            zone.validate(&format!("{at}.zones[{z}]"), pa, count)?;
        }
        if !(self.internode.is_finite() && self.internode > 0.0) {
            return refuse(
                format!("{at}.internode"),
                "an internode has a positive length",
            );
        }
        if !(0.0..=PI).contains(&self.insertion) {
            return refuse(
                format!("{at}.insertion"),
                "an insertion angle lies in 0 to pi",
            );
        }
        if !self.divergence.is_finite() {
            return refuse(format!("{at}.divergence"), "a divergence angle is finite");
        }
        Ok(())
    }
}

impl Zone {
    fn validate(&self, at: &str, pa: usize, count: usize) -> Result<()> {
        match self.nodes {
            NodeLaw::Uniform { min, max } if min > max => {
                return refuse(
                    format!("{at}.nodes.min"),
                    "the least node count exceeds the most",
                );
            }
            NodeLaw::Uniform { max, .. } if max > MAX_NODES_PER_ZONE => {
                return refuse(format!("{at}.nodes.max"), "a zone holds at most 1000 nodes");
            }
            NodeLaw::Poisson { mean } if !(0.0..=MAX_MEAN_NODES).contains(&mean) => {
                return refuse(
                    format!("{at}.nodes.mean"),
                    "a mean node count lies in 0 to 500",
                );
            }
            _ => {}
        }
        if self.buds == 0 || self.buds > MAX_BUDS {
            return refuse(format!("{at}.buds"), "a node carries 1 to 6 buds");
        }
        if self.lateral.len() != count {
            return refuse(
                format!("{at}.lateral"),
                "one probability per physiological age",
            );
        }
        for (j, &p) in self.lateral.iter().enumerate() {
            if !(0.0..=1.0).contains(&p) {
                return refuse(format!("{at}.lateral[{j}]"), "a probability lies in 0 to 1");
            }
            if j < pa && p > 0.0 {
                return refuse(
                    format!("{at}.lateral[{j}]"),
                    "a lateral bud is never younger than its parent",
                );
            }
        }
        if self.lateral.iter().sum::<f64>() > 1.0 + 1e-12 {
            return refuse(
                format!("{at}.lateral"),
                "the lateral probabilities sum above one",
            );
        }
        Ok(())
    }
}
