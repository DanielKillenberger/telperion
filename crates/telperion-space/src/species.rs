//! A species as the engine reads it: the reference axis, an ordered list of
//! physiological ages (PA), each a state of the bud automaton. Index 0 is the
//! youngest PA, the botanists' PA 1 (the trunk's). A bud's PA only ages: a
//! lateral bud carries its parent's PA or an older one, and an apex moves
//! along the reference axis towards older PAs (AmapSim's oriented automaton;
//! GreenLab's dual-scale automaton, de Reffye et al. 2021).
use crate::error::{refuse, Result};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// The most buds one node carries: a whorl of six.
pub const MAX_BUDS: u8 = 6;
pub(crate) const MAX_NODES_PER_ZONE: u32 = 1_000;
const MAX_MEAN_NODES: f64 = 500.0;
const MAX_STATES: usize = 64;
/// The longest internode, in metres: a budget of phytomers this long stays finite.
const MAX_INTERNODE: f64 = 100.0;
/// The fastest bend or wander, in radians (or shares) per metre.
const MAX_RATE: f64 = 100.0;
/// The widest pipe one phytomer adds, in metres.
const MAX_PIPE: f64 = 1.0;
/// The longest ripening of a phytomer's own wood, in years.
const MAX_RIPENING: f64 = 1_000.0;
/// The pipe model's exponents: below 1.5 a fork outgrows what bears it.
const MIN_EXPONENT: f64 = 1.5;
const MAX_EXPONENT: f64 = 4.0;
/// The most an axis bends per unit of its moment over its section's
/// stiffness.
const MAX_SAG: f64 = 1_000.0;
/// The steepest rise of the abortion hazard.
const MAX_RISE: f64 = 8.0;
/// The longest a bud sleeps before it can wake, in years.
const MAX_DELAY: f64 = 1_000.0;

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
    /// The probability that the apex aborts after each growth unit, so its
    /// laterals carry the axis on: sympodial growth. Neutral 0.
    pub abortion: f64,
    /// How the abortion probability rises with the growth units an axis
    /// has grown, k: 1 - (1 - abortion)^(k^rise), a hazard that gives
    /// modules a regular length. Neutral 0, a flat rate; dormant without
    /// abortion.
    pub abortion_rise: f64,
    /// The probability that a stopped apex is replaced by a relay bud of its
    /// own PA at its last node, as Troll's relays are. Neutral 0; dormant
    /// where no apex stops.
    pub relay: f64,
    /// Where along its stopped axis a relay bud stands, as a share of the
    /// nodes of the axis's last growth unit from their base: Troll's relay
    /// "in the curvature zone" of the module it takes over from. Neutral 1, the last node; dormant
    /// without relays.
    pub relay_at: f64,
    /// How far a relay bud turns from its phyllotactic side to the upper
    /// side of its parent (epitony). Neutral 0; dormant without relays.
    pub epitony: f64,
    /// Troll's secondary erection: how fast, per cycle of the axis's age,
    /// its base straightens further towards the vertical, on top of its
    /// straightening (and on the seed, which has none). Neutral 0.
    pub erection: f64,
    /// How ready the axis is to branch: its lateral probabilities scaled.
    /// Zero is Corner's unbranched stem. Neutral 1; dormant without laterals.
    pub readiness: f64,
    /// The strength of rhythmic growth: 1 keeps each zone's laterals, 0
    /// spreads the growth unit's laterals evenly over its nodes, continuous
    /// growth. Neutral 1; dormant in a growth unit of one zone.
    pub rhythm: f64,
    /// How far the base of a lateral axis of this PA straightens towards
    /// the vertical, as Troll's plagiotropic axes do. Neutral 0.
    pub straightening: f64,
    /// How an axis of this PA bends, wanders, turns its laterals' plane and
    /// thickens.
    pub form: Form,
}

/// The shape an axis takes as it is laid, beyond its angles: each a
/// continuous setting, neutral where it changes nothing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Form {
    /// How fast the axis bends towards `elevation`: the share of the gap
    /// closed is 1 - exp(-tropism × metres grown). Neutral 0.
    pub tropism: f64,
    /// The elevation the axis bends towards, in radians above the
    /// horizontal: plagiotropic near 0, orthotropic at pi / 2.
    pub elevation: f64,
    /// Sinuosity: each node turns the axis by up to this many radians per
    /// metre of its internode, in a direction its draw keys. Neutral 0.
    pub wander: f64,
    /// The turn, in radians about its heading, of the plane a lateral of
    /// this axis branches in. Neutral 0: the plane holds its parent.
    pub plane: f64,
    /// The pipe each phytomer of this PA adds below it, as a radius in
    /// metres (the pipe model: a section is the sum of the sections it bears).
    pub pipe: f64,
    /// The pipe model's exponent at this PA's wood: its radius to this
    /// power is the sum of the radii it carries to this power. 2 sums
    /// areas; a larger one keeps what it carries thicker beside it, so
    /// its limbs taper less abruptly from it. Neutral 2.
    pub exponent: f64,
    /// The years over which a phytomer lays down its own pipe, from a
    /// share at its first year to all of it: a limb stays heavy where it
    /// is old and ends in fine young tips. Neutral 0, at once.
    pub ripening: f64,
    /// How unequal laterals of this PA are among their siblings: each
    /// keeps a share of its size, from all of it (0) towards a keyed share
    /// few hold whole (1), so a few dominate and the rest stay small.
    /// Neutral 0.
    pub dominance: f64,
    /// How far a lateral of this PA turns about its parent, by up to this
    /// many radians either way as its lineage keys, so laterals of one
    /// parent do not stack in one plane. Neutral 0.
    pub roll: f64,
    /// How far the axis bends under the load it carries, as a beam does:
    /// each phytomer turns down by `sag` times the bending moment of the
    /// wood and foliage beyond it over its radius to the fourth, per metre
    /// of its length (`sag.rs`). Its tip carries nothing and keeps its
    /// tropism. Neutral 0.
    pub sag: f64,
}

impl Default for Form {
    /// Straight axes in their parent's plane, each phytomer a 5 mm pipe.
    fn default() -> Self {
        Self {
            tropism: 0.0,
            elevation: 0.0,
            wander: 0.0,
            plane: 0.0,
            pipe: 0.005,
            exponent: 2.0,
            ripening: 0.0,
            dominance: 0.0,
            roll: 0.0,
            sag: 0.0,
        }
    }
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
    /// The probability that each bud's place also holds a sleeping bud of
    /// each PA, which wakes years later (`dormant.rs`). Neutral all 0.
    pub dormant: Vec<f64>,
    /// The years before a sleeping bud can wake. Dormant without sleeping
    /// buds.
    pub delay: f64,
    /// A sleeping bud's yearly waking hazard after the delay; 0, it never
    /// wakes. Dormant without sleeping buds.
    pub rate: f64,
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
        if !(self.internode > 0.0 && self.internode <= MAX_INTERNODE) {
            return refuse(
                format!("{at}.internode"),
                "an internode is longer than 0 and at most 100 m",
            );
        }
        if !(0.0..=PI).contains(&self.insertion) {
            return refuse(
                format!("{at}.insertion"),
                "an insertion angle lies in 0 to pi",
            );
        }
        let rates = [
            ("abortion_rise", self.abortion_rise, MAX_RISE),
            ("erection", self.erection, MAX_RATE),
        ];
        for (name, value, most) in rates {
            if !(0.0..=most).contains(&value) {
                return refuse(format!("{at}.{name}"), "a rate lies in its bounded range");
            }
        }
        let shares = [
            ("abortion", self.abortion),
            ("relay", self.relay),
            ("relay_at", self.relay_at),
            ("epitony", self.epitony),
            ("readiness", self.readiness),
            ("rhythm", self.rhythm),
            ("straightening", self.straightening),
        ];
        for (name, value) in shares {
            if !(0.0..=1.0).contains(&value) {
                return refuse(format!("{at}.{name}"), "a share lies in 0 to 1");
            }
        }
        if !(-TAU..=TAU).contains(&self.divergence) {
            return refuse(
                format!("{at}.divergence"),
                "a divergence angle lies in -2 pi to 2 pi",
            );
        }
        self.form.validate(&format!("{at}.form"))
    }
}

impl Form {
    fn validate(&self, at: &str) -> Result<()> {
        let rates = [
            ("tropism", self.tropism, MAX_RATE),
            ("wander", self.wander, MAX_RATE),
            ("pipe", self.pipe, MAX_PIPE),
            ("ripening", self.ripening, MAX_RIPENING),
            ("sag", self.sag, MAX_SAG),
        ];
        for (name, value, most) in rates {
            if !(0.0..=most).contains(&value) {
                return refuse(format!("{at}.{name}"), "a rate lies in its bounded range");
            }
        }
        if !(-FRAC_PI_2..=FRAC_PI_2).contains(&self.elevation) {
            return refuse(
                format!("{at}.elevation"),
                "an elevation lies in -pi / 2 to pi / 2",
            );
        }
        if !(MIN_EXPONENT..=MAX_EXPONENT).contains(&self.exponent) {
            return refuse(format!("{at}.exponent"), "a pipe exponent lies in 1.5 to 4");
        }
        for (name, value) in [("dominance", self.dominance), ("secondary", self.secondary)] {
            if !(0.0..=1.0).contains(&value) {
                return refuse(format!("{at}.{name}"), "a share lies in 0 to 1");
            }
        }
        if !(0.0..=PI).contains(&self.roll) {
            return refuse(format!("{at}.roll"), "a roll lies in 0 to pi");
        }
        if !(-TAU..=TAU).contains(&self.plane) {
            return refuse(
                format!("{at}.plane"),
                "a plane's turn lies in -2 pi to 2 pi",
            );
        }
        Ok(())
    }
}

impl PaState {
    /// The probability the apex aborts after an axis's `k`th growth unit.
    pub fn abortion_at(&self, k: usize) -> f64 {
        if self.abortion_rise == 0.0 || self.abortion <= 0.0 {
            return self.abortion;
        }
        1.0 - (1.0 - self.abortion).powf((k as f64).powf(self.abortion_rise))
    }

    /// Each zone's lateral probabilities as the axis draws them: spread
    /// towards the growth unit's mean by the rhythm, scaled by readiness.
    pub(crate) fn laterals(&self) -> Vec<Vec<f64>> {
        let weights: Vec<f64> = self.zones.iter().map(|z| z.nodes.mean()).collect();
        let total: f64 = weights.iter().sum();
        let width = self.zones[0].lateral.len();
        let mean: Vec<f64> = (0..width)
            .map(|j| {
                let sum: f64 = self
                    .zones
                    .iter()
                    .zip(&weights)
                    .map(|(z, w)| w * z.lateral[j])
                    .sum();
                if total > 0.0 {
                    sum / total
                } else {
                    0.0
                }
            })
            .collect();
        // Dormant, exactly, in a growth unit of one zone or of no nodes.
        let rhythm = if total > 0.0 && self.zones.len() > 1 {
            self.rhythm
        } else {
            1.0
        };
        self.zones
            .iter()
            .map(|zone| {
                zone.lateral
                    .iter()
                    .zip(&mean)
                    .map(|(&p, &m)| self.readiness * (rhythm * p + (1.0 - rhythm) * m))
                    .collect()
            })
            .collect()
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
        table(&format!("{at}.lateral"), &self.lateral, pa, count)?;
        table(&format!("{at}.dormant"), &self.dormant, pa, count)?;
        if !(0.0..=MAX_DELAY).contains(&self.delay) {
            return refuse(format!("{at}.delay"), "a delay lies in 0 to 1000 years");
        }
        if !(0.0..=MAX_RATE).contains(&self.rate) {
            return refuse(format!("{at}.rate"), "a rate lies in its bounded range");
        }
        Ok(())
    }
}

/// Refuses a table of bud probabilities, one per PA, that is not one.
fn table(at: &str, table: &[f64], pa: usize, count: usize) -> Result<()> {
    if table.len() != count {
        return refuse(at, "one probability per physiological age");
    }
    for (j, &p) in table.iter().enumerate() {
        if !(0.0..=1.0).contains(&p) {
            return refuse(format!("{at}[{j}]"), "a probability lies in 0 to 1");
        }
        if j < pa && p > 0.0 {
            return refuse(
                format!("{at}[{j}]"),
                "a lateral bud is never younger than its parent",
            );
        }
    }
    if table.iter().sum::<f64>() > 1.0 + 1e-12 {
        return refuse(at, "the bud probabilities sum above one");
    }
    Ok(())
}
