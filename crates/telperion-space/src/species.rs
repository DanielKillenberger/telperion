//! A species as the engine reads it: the reference axis, an ordered list of
//! physiological ages (PA), each a state of the bud automaton. Index 0 is the
//! youngest PA, the botanists' PA 1 (the trunk's). A bud's PA only ages: a
//! lateral bud carries its parent's PA or an older one, and an apex moves
//! along the reference axis towards older PAs (AmapSim's oriented automaton;
//! GreenLab's dual-scale automaton, de Reffye et al. 2021).
use crate::error::{refuse, Result};
use crate::transcendental::Transcendental;

pub(crate) mod scale;
mod validate;

/// The roles a growth unit's zones play, by index, so two species' zones
/// of one role are walked into each other (host, 2026-10-05): its bare
/// base, its medial nodes, the whorl at its top, and a spare. A unit may
/// leave its last roles out; they have no nodes.
pub const BARE: usize = 0;
pub const MEDIAL: usize = 1;
pub const TOP: usize = 2;
pub const SPARE: usize = 3;
pub const ZONES: usize = 4;
/// The order the roles' nodes stand in along the unit, base to top: the
/// spare between the medial nodes and the top whorl.
pub(crate) const ALONG: [usize; ZONES] = [BARE, MEDIAL, SPARE, TOP];

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
/// The longest lifespan of an age, in cycles.
const MAX_LIFESPAN: f64 = 1_000_000.0;
/// The longest a bud sleeps before it can wake, in years.
const MAX_DELAY: f64 = 1_000.0;
/// The longest a wander's bend is remembered, in metres.
const MAX_BEND_LENGTH: f64 = 1_000.0;
/// The largest leaf area one node bears, in square metres: the date
/// palm's frond, 2.81 m² a node, is the largest shipped leaf (fn-264, host
/// 2026-10-12).
pub const MAX_LEAF_AREA: f64 = 5.0;
/// The steepest response of a hazard or a size to shade.
const MAX_SHADE: f64 = 10.0;
/// The most leaves one short shoot's cluster carries (fn-223).
pub const MAX_CLUSTER_LEAVES: f64 = 8.0;

#[derive(Debug, Clone, PartialEq)]
pub struct Species {
    /// The reference axis, youngest PA first.
    pub states: Vec<PaState>,
}

/// One physiological age: how an apex in it grows, lives and branches.
#[derive(Debug, Clone, PartialEq)]
pub struct PaState {
    /// The cycles an apex spends in this PA before it moves on, one growth
    /// unit a cycle. A lifespan between whole cycles grows its last unit at
    /// the share left of it, and the next age's first unit grows in the
    /// same cycle at the rest, as a woken bud's does (host decision 7); at
    /// 0 the age is passed through, its apex moving on to the next age, by
    /// its `continuation`, without growing in it.
    pub lifespan: f64,
    /// The probability that an apex which has spent its lifespan moves on
    /// to the next age on the reference axis rather than stops (its relay
    /// then may take over): 1 where an age moves on, 0 where it ends.
    /// Drawn, grown in by its lead. Dormant at the axis's last age, which
    /// has no next.
    pub continuation: f64,
    /// The probability that the apex survives each cycle, tested before it grows.
    pub viability: f64,
    /// The growth unit's zones of nodes and their lateral buds, by role
    /// (`BARE`, `MEDIAL`, `TOP`, `SPARE`); they stand along the unit in
    /// the order `ALONG` gives.
    pub zones: Vec<Zone>,
    /// Cycles a lateral axis of this PA keeps with no living apex before it
    /// is shed, fading out over the cycle past it (`shed.rs`), so a delay
    /// between whole cycles keeps it at that share; infinite, it is kept.
    pub shedding: f64,
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
    /// The probability that an apex that aborted is replaced by a relay bud
    /// of its own PA at its last node, as Troll's relays are. Neutral 0;
    /// dormant where no apex aborts. With `abortion` it is one outcome of
    /// three, carrying on, relaying or dying, which a walk between species
    /// mixes as shares (host decision 12).
    pub relay: f64,
    /// The same for an apex whose age ends and does not move on (with
    /// `continuation`).
    pub relay_ended: f64,
    /// The same for an apex whose growth unit failed (with `viability`).
    pub relay_failed: f64,
    /// Where along its stopped axis a relay bud stands, as a share of the
    /// nodes of the axis's last growth unit from their base: Troll's relay
    /// "in the curvature zone" of the module it takes over from. Neutral 1, the last node; dormant
    /// without relays. Where the relay joins its parent, and so where its
    /// wood and load enter and the drawing's join stands, follows from it
    /// (`geometry::attach`, fn-244).
    pub relay_at: f64,
    /// How far a relay bud turns from its phyllotactic side to the upper
    /// side of its parent (epitony). Neutral 0; dormant without relays.
    pub epitony: f64,
    /// Troll's take-over (fn-286): how far a relay's course runs back onto
    /// the line of the axis it carries on, the parent's running direction
    /// at its tip, at its base, fading over the scaled length its
    /// straightening fades over; as far as the relay has left that line
    /// (its blend). Neutral 0; dormant without relays.
    pub takeover: f64,
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
    /// The leaf area a node of this PA bears in its year, in square
    /// metres: what its leaves lend the light's lattice (`light.rs`).
    /// Neutral 0, leafless; dormant without extinction.
    pub leaf_area: f64,
    /// The share of this PA's kept nodes that bear an inflorescence: where
    /// a species flowers, by its architectural model (fn-216 host decision
    /// 1). The engine grows nothing of it; the pipeline's expansion draws
    /// the flowers. Neutral 0, flowerless.
    pub flowering: f64,
    /// Leaves at the tip of each axis of this PA, which then bears no leaf
    /// along its length: a short shoot's cluster (fn-223). A fraction is
    /// split by a keyed draw at each axis, which bears the whole count
    /// either side of it, a short shoot of none bearing along its length
    /// (`telperion_space::cluster`, fn-277). The engine grows nothing of it; the
    /// pipeline's expansion seats the leaves. Neutral 0, leaves along the
    /// shoot as before.
    pub cluster_leaves: f64,
    /// Radians either side of its tip's level bearing a cluster's leaves
    /// fan across: pi / 2 is a half circle, 0 stacks them. Dormant without
    /// `cluster_leaves`.
    pub cluster_spread: f64,
    /// How many years a phytomer of this PA bears its leaf: a station on
    /// wood `age` years old (0 the current year's) keeps its leaf by its
    /// retention clamp(leaf_years - age, 0, 1), on top of the slender
    /// wood the canopy bears on (fn-248). Fractional, the one age it
    /// splits keeps that share of its stations by a keyed draw. The
    /// engine grows nothing of it; the pipeline's expansion reads it.
    /// Neutral infinite, every age bears; a short shoot's cluster bears
    /// while its shoot lives.
    pub leaf_years: f64,
    /// The days a growth unit of this PA takes to elongate from bud burst,
    /// its shoot extending from its base along the logistic course
    /// (`unfold::course`) and its nodes unfolding as it reaches them
    /// (fn-270 R5). The engine grows nothing of it; the presentation
    /// within the year reads it. Neutral 0, at once.
    pub elongation: f64,
    /// The days a leaf of this PA takes to expand from its node's
    /// unfolding to its whole size, along the same course (fn-270 R5).
    /// Read as `elongation` is. Neutral 0, at once.
    pub expansion: f64,
    /// The years a leaf of this PA lives (fn-270 R5c): a leaf older than
    /// this at autumn colours and falls, by degree (a fraction falls that
    /// share of its cohort), and where its wood still bears one it grows
    /// again from its bud at budburst. Apart from `leaf_years`, how long
    /// wood bears leaves, so whole ages are unchanged. Read as
    /// `elongation` is. Neutral infinite: no leaf falls.
    pub leaf_life: f64,
    /// The day of the calendar year its buds burst: with `elongation` and
    /// `expansion` it places a whole age, which stands at full leaf, on the
    /// calendar, and so `senescence` on the presentation's year (R5c).
    /// Dormant while no leaf falls.
    pub budburst: f64,
    /// The day of the calendar year its falling leaves start to colour,
    /// the days they take to colour, and then the days they take to fall
    /// (R5c). Dormant while no leaf falls.
    pub senescence: f64,
    pub colouring: f64,
    pub leaf_fall: f64,
    /// How shade raises an apex's yearly death hazard: its survival is
    /// its viability to the power light^-shade_hazard, so ln survival =
    /// ln viability times light^-φ (host, 2026-10-05). Neutral 0; dormant
    /// where the apex never dies (viability 1) or no leaf shades it.
    pub shade_hazard: f64,
    /// How the ground ends an apex that reaches it (fn-252 R2): while
    /// its tip, as the rough layout grown with the tree lays it, rests on
    /// the ground, its yearly survival is its viability times
    /// (1 - ground_hazard). Neutral 0, today's resting and running along
    /// the ground (fn-200); at 1 a shoot that reaches the ground stops
    /// there. Read through the rough layout, so only where light shades.
    pub ground_hazard: f64,
    /// How a bud's growth unit follows the vigour the tree's light gives
    /// it (`allocation.rs`; host decisions 11 and 14): its own internodes
    /// and girth scale by its vigour per presence against the mean of the
    /// growing buds of its PA, to ψ, normalised so each PA's mean unit
    /// stays whole (host decision 23); what it bears does not inherit
    /// that. Neutral 0; dormant where no
    /// leaf shades it.
    pub shade_size: f64,
    /// Apical control λ at this PA's branching points: the share of its
    /// vigour the continuing axis keeps against its laterals, weighted by
    /// their light (Borchert–Honda, Pałubicki 2009). 0.5, the unbiased
    /// split, gives every bud its own light's share; dormant while
    /// `shade_size` is 0 or no leaf shades.
    pub apical_control: f64,
    /// The carbon balance's upkeep (fn-197 step 4): what a metre of this
    /// PA's present wood costs, in the units of the light a bud of whole
    /// presence collects in full light. Neutral 0, free.
    pub upkeep: f64,
    /// How fast a lateral of this PA is shed as its remembered balance,
    /// (light - upkeep) / (light + upkeep) over its subtree, falls below
    /// `tolerance`: a yearly hazard of balance_hazard x (tolerance -
    /// balance) where the balance lies below it. Neutral 0, never.
    pub balance_hazard: f64,
    /// The remembered balance, from -1 to 1, below which a lateral of
    /// this PA starts to be shed: its shade tolerance. Dormant without
    /// `balance_hazard`.
    pub tolerance: f64,
    /// The share of a shed branch of this PA's pipe that stays in its
    /// bearer's girth: Shinozaki's disused pipes, which Pałubicki (2009)
    /// keeps whole. Neutral 0, today's.
    pub retained: f64,
    /// How a phytomer of this PA's own pipe follows its leaves' light:
    /// (light / the tree's pipe-weighted mean)^χ, so limbs whose leaves
    /// catch more light thicken and the tree's own pipe is conserved.
    /// Neutral 0; dormant where no leaf shades.
    pub leaf_girth: f64,
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
    /// Secondary growth: how far the girth follows the load the wood
    /// carries. The radius is `secondary` times the pipe model's radius
    /// plus the rest times the radius the phytomer was established with,
    /// its own pipe as its apex laid it down. Neutral 1, the pipe model;
    /// at 0 the axis keeps its established width for life, as a palm's
    /// stem does.
    pub secondary: f64,
    /// How far, in metres, the axis's wander remembers its bend: the
    /// correlation length of a curvature that relaxes towards none and is
    /// kicked by each node's keyed draw (fn-207). 0 draws an independent
    /// turn at every node, as wander always has; a few metres turn a limb
    /// in a few slow arcs of the same spread per metre. Neutral 0;
    /// dormant without wander.
    pub bend_length: f64,
    /// How far each internode's length departs from its PA's, node to
    /// node: scaled by 1 + spread × (2u − 1), u its node's keyed draw, so
    /// a unit's expected length is unchanged (fn-222). Neutral 0, every
    /// internode alike; at most 1.
    pub internode_spread: f64,
    /// How far each lateral's or relay's insertion angle departs from its
    /// PA's, axis to axis: offset by spread × (2u − 1) radians, u its
    /// lineage's keyed draw, so the mean angle is unchanged (fn-224).
    /// Neutral 0, every axis at its PA's angle; at most pi / 2.
    pub insertion_spread: f64,
    /// How far each axis's tropism departs from its PA's, axis to axis:
    /// scaled by 1 + spread × (2u − 1), u its lineage's keyed draw, so its
    /// mean is unchanged (fn-224). Neutral 0, every axis alike; at most 1.
    pub tropism_spread: f64,
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
            secondary: 1.0,
            bend_length: 0.0,
            internode_spread: 0.0,
            insertion_spread: 0.0,
            tropism_spread: 0.0,
        }
    }
}

/// A zone of a growth unit: its node count and the PA of each node's buds.
/// Within a zone the nodes stand acrotonically: the youngest lateral PA on
/// top, bare nodes at the base.
#[derive(Debug, Clone, PartialEq)]
pub struct Zone {
    pub nodes: NodeLaw,
    /// Lateral buds per node, each drawn independently (1 alternate, 2
    /// opposite): a place for each whole bud, and one more at the share of
    /// the fraction, what it bears grown at that share, as a lifespan's
    /// last unit is; the whorl's places turn by a full turn over `buds`,
    /// so a fraction spaces them by degree (fn-206).
    pub buds: f64,
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
    /// Uniform on `min..=max` for whole bounds, `min == max` being
    /// deterministic; node j stands as the draw passes its share of the
    /// span, (j - min + 1) / (max - min + 1), grown in by its lead, so a
    /// bound between whole numbers grows its last node in by degree
    /// (fn-206).
    Uniform {
        min: f64,
        max: f64,
    },
    Poisson {
        mean: f64,
    },
}

impl NodeLaw {
    pub fn mean(self) -> f64 {
        match self {
            // Each node stands with chance (max - j) / (max - min + 1),
            // within 0 and 1: (min + max) / 2 for whole bounds.
            NodeLaw::Uniform { min, max } => {
                let span = max - min + 1.0;
                (0..max.ceil() as u32)
                    .map(|j| ((max - f64::from(j)) / span).clamp(0.0, 1.0))
                    .sum()
            }
            NodeLaw::Poisson { mean } => mean,
        }
    }
}

impl Species {
    /// The species in canonical form (host decision 8): every age it does
    /// not use (no lifespan) carries the settings of its nearest age it
    /// does use, the earlier on a tie, with no lifespan, moving on, and no
    /// laterals or sleeping buds, so it is passed through and grows nothing,
    /// and a walk to a species that uses it mixes like with like. Ages a
    /// species uses are left as they are; a species that uses none is
    /// returned as it is.
    pub fn canonical(mut self) -> Species {
        let used: Vec<usize> = (0..self.states.len())
            .filter(|&k| self.states[k].lifespan > 0.0)
            .collect();
        for k in 0..self.states.len() {
            if self.states[k].lifespan > 0.0 {
                continue;
            }
            let Some(&near) = used.iter().min_by_key(|&&u| (u.abs_diff(k), u > k)) else {
                return self;
            };
            let mut state = self.states[near].clone();
            state.lifespan = 0.0;
            state.continuation = 1.0;
            for zone in &mut state.zones {
                zone.lateral.iter_mut().for_each(|p| *p = 0.0);
                zone.dormant.iter_mut().for_each(|p| *p = 0.0);
            }
            self.states[k] = state;
        }
        self
    }

    /// The age a bud of PA `pa` grows in, `pa` itself or the first after
    /// it with a lifespan, each age of none passed through by its
    /// continuation, and the product of those continuations: the share of
    /// the bud that reaches it. None where nothing does.
    pub fn lived(&self, mut pa: usize) -> Option<(usize, f64)> {
        let mut reach = 1.0;
        loop {
            let state = self.states.get(pa)?;
            if state.lifespan > 0.0 {
                return Some((pa, reach));
            }
            reach *= state.continuation;
            pa += 1;
            if reach <= 0.0 {
                return None;
            }
        }
    }

    /// The age an apex of `pa` that has spent its lifespan moves on to,
    /// and the probability that it does: its own continuation times those
    /// of the ages it passes through. None where it never moves on.
    pub fn successor(&self, pa: usize) -> Option<(usize, f64)> {
        let go = self.states[pa].continuation;
        if go <= 0.0 {
            return None;
        }
        self.lived(pa + 1).map(|(next, reach)| (next, go * reach))
    }

    /// Whether any bud reads the light or the rough layout: its survival
    /// (by its light or the ground its tip reaches), its unit's size,
    /// its subtree's carbon balance or its pipe. Without one the light
    /// changes nothing and the tree grows no rough layout.
    pub(crate) fn reads_light(&self) -> bool {
        self.states.iter().any(|s| {
            s.shade_hazard != 0.0
                || s.ground_hazard != 0.0
                || s.shade_size != 0.0
                || s.balance_hazard != 0.0
                || s.leaf_girth != 0.0
        })
    }

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
    /// Whether an apex's abortion hazard changes with the time it grew.
    pub(crate) fn rises(&self) -> bool {
        self.abortion_rise != 0.0 && self.abortion > 0.0
    }

    /// Its lifespan as the grower and the closed form read it: on the
    /// tick grid (`contract.rs`, FINE-CONTRACT.md D7).
    pub(crate) fn life(&self) -> f64 {
        crate::contract::grid(self.lifespan)
    }

    /// The probability the apex aborts after a growth unit that ends `t`
    /// cycles of its axis's growth: whole cycles for whole units, the real
    /// time where a unit is partial (host decision 7).
    pub fn abortion_at(&self, t: f64) -> f64 {
        if self.abortion_rise == 0.0 || self.abortion <= 0.0 {
            return self.abortion;
        }
        1.0 - (1.0 - self.abortion).powf_fixed(t.powf_fixed(self.abortion_rise))
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
    /// The bud places a node has, the last of them at a share
    /// (`share`); none of them past `MAX_BUDS`.
    pub(crate) fn places(&self) -> usize {
        self.buds.ceil() as usize
    }

    /// The chance bud place `slot` stands: 1 for a whole one, the fraction
    /// of `buds` for the last, a making draw (fn-231).
    pub(crate) fn share(&self, slot: usize) -> f64 {
        (self.buds - slot as f64).min(1.0)
    }

    /// Its whole bud places a node, and the chance the last, a fraction,
    /// stands: none where every place is whole.
    pub(crate) fn whole_places(&self) -> (i32, f64) {
        let whole = self.buds.floor();
        (whole as i32, self.buds - whole)
    }
}
