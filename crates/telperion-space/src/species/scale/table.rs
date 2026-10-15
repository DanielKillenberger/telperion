//! Every setting of an age, once: its scale, the values the engine admits
//! (`validate` reads its single-range checks here), its unit and a line of
//! meaning (fn-284, host ruling 2026-10-15). The doc comments on `PaState`,
//! `Form` and `Zone` stay the full account; a meaning here is the line a
//! control shows.
use super::Scale::{self, Kept, Linear, Log, Outcome};
use crate::species::{
    MAX_BEND_LENGTH, MAX_BUDS, MAX_CLUSTER_LEAVES, MAX_DELAY, MAX_EXPONENT, MAX_INTERNODE,
    MAX_LEAF_AREA, MAX_LIFESPAN, MAX_NODES_PER_ZONE, MAX_PIPE, MAX_RATE, MAX_RIPENING, MAX_RISE,
    MAX_SAG, MAX_SHADE, MIN_EXPONENT,
};
use crate::unfold::YEAR;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// One setting of an age.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Setting {
    /// Its name as `settings` and `zone_settings` give it.
    pub name: &'static str,
    pub scale: Scale,
    pub low: f64,
    pub high: f64,
    /// `low` itself is refused.
    pub low_open: bool,
    pub unit: &'static str,
    pub meaning: &'static str,
}

impl Setting {
    /// Whether the engine admits `v` here, its cross-setting checks apart;
    /// a NaN never.
    pub fn admits(&self, v: f64) -> bool {
        let low = if self.low_open {
            v > self.low
        } else {
            v >= self.low
        };
        low && v <= self.high
    }
}

const fn row(
    name: &'static str,
    scale: Scale,
    range: [f64; 2],
    unit: &'static str,
    meaning: &'static str,
) -> Setting {
    Setting {
        name,
        scale,
        low: range[0],
        high: range[1],
        low_open: false,
        unit,
        meaning,
    }
}

const SHARE: [f64; 2] = [0.0, 1.0];
const DAYS: [f64; 2] = [0.0, YEAR];
const EVER: [f64; 2] = [0.0, f64::INFINITY];
const SHADE: [f64; 2] = [0.0, MAX_SHADE];
const RATE: [f64; 2] = [0.0, MAX_RATE];

/// Every setting, in the order `settings` and `zone_settings` list them.
pub const SETTINGS: &[Setting] = &[
    row("lifespan", Log { floor: 1.0 }, [0.0, MAX_LIFESPAN], "years",
        "The years an apex grows in this age before it moves on. Dormant while the tree is younger than it."),
    row("continuation", Outcome, SHARE, "probability",
        "The chance an apex that has spent its lifespan moves on to the next age. Dormant at the last age."),
    row("viability", Outcome, SHARE, "probability",
        "The chance the apex survives each year."),
    row("shedding", Kept, EVER, "years",
        "Years a lateral with no living apex is kept before it is shed; never keeps it. Dormant on an axis that bears no laterals."),
    Setting { low_open: true, ..row("internode", Log { floor: 1e-3 }, [0.0, MAX_INTERNODE], "m",
        "The internode length.") },
    row("insertion", Linear, [0.0, PI], "rad",
        "The angle between an axis of this age and its parent. Dormant on an axis that bears no laterals."),
    row("divergence", Linear, [-TAU, TAU], "rad",
        "The angle between successive nodes around the axis (phyllotaxis). Dormant on an axis that bears no laterals."),
    row("abortion", Outcome, SHARE, "probability",
        "The chance the apex aborts after each growth unit, so its laterals carry on."),
    row("abortion_rise", Linear, [0.0, MAX_RISE], "-",
        "How the abortion chance rises with the units an axis has grown. Dormant without abortion."),
    row("relay", Outcome, SHARE, "probability",
        "The chance an aborted apex is replaced by a relay bud of its own age. Dormant without abortion."),
    row("relay_ended", Outcome, SHARE, "probability",
        "The same, for an apex whose age ends and does not move on. Dormant where every age moves on."),
    row("relay_failed", Outcome, SHARE, "probability",
        "The same, for an apex whose growth unit failed. Dormant where every apex survives."),
    row("relay_at", Linear, SHARE, "share of the unit",
        "Where along the stopped axis's last unit a relay bud stands. Dormant without relays."),
    row("epitony", Linear, SHARE, "share",
        "How far a relay bud turns to the upper side of its parent. Dormant without relays."),
    row("erection", Log { floor: 0.01 }, [0.0, MAX_RATE], "per year",
        "How fast an axis's base straightens further towards the vertical with age."),
    row("readiness", Linear, SHARE, "share",
        "How ready the axis is to branch; zero is an unbranched stem. Dormant without laterals."),
    row("rhythm", Linear, SHARE, "share",
        "Rhythmic growth: 1 keeps each zone's laterals, 0 spreads them evenly. Dormant in a unit of one zone."),
    row("straightening", Linear, SHARE, "share",
        "How far a lateral's base straightens towards the vertical. Dormant on an axis that bears no laterals."),
    row("leaf_area", Log { floor: 1e-4 }, [0.0, MAX_LEAF_AREA], "m²",
        "The leaf area a node bears in its year, which shades the light. Dormant where no leaf shades another."),
    row("flowering", Linear, SHARE, "share of nodes",
        "The share of nodes that bear an inflorescence."),
    row("cluster_leaves", Linear, [0.0, MAX_CLUSTER_LEAVES], "leaves",
        "Leaves clustered at each axis's tip, as a short shoot bears them."),
    row("cluster_spread", Linear, [0.0, FRAC_PI_2], "rad",
        "How far a tip cluster's leaves fan either side of its level. Dormant without tip clusters."),
    row("leaf_years", Kept, EVER, "years",
        "How many years wood of this age bears its leaves; never, every age bears."),
    row("elongation", Log { floor: 1.0 }, DAYS, "days",
        "Days a growth unit takes to elongate from bud burst."),
    row("expansion", Log { floor: 1.0 }, DAYS, "days",
        "Days a leaf takes to expand to its whole size."),
    row("leaf_life", Kept, EVER, "years",
        "The years a leaf lives before it colours and falls; never, none falls. Dormant on the grown tree; drawn as the year plays."),
    row("budburst", Linear, DAYS, "day of year", "The day of the year the buds burst. Dormant on the grown tree, and while no leaf falls."),
    row("senescence", Linear, DAYS, "day of year",
        "The day of the year falling leaves start to colour. Dormant on the grown tree, and while no leaf falls."),
    row("colouring", Log { floor: 1.0 }, DAYS, "days", "Days the falling leaves take to colour. Dormant on the grown tree, and while no leaf falls."),
    row("leaf_fall", Log { floor: 1.0 }, DAYS, "days", "Days the coloured leaves take to fall. Dormant on the grown tree, and while no leaf falls."),
    row("shade_hazard", Linear, SHADE, "exponent",
        "How shade raises an apex's yearly death hazard. Dormant where every apex survives or no leaf shades."),
    row("ground_hazard", Linear, SHARE, "share",
        "How the ground ends an apex whose tip rests on it. Dormant where no tip reaches the ground."),
    row("shade_size", Linear, SHADE, "exponent",
        "How a growth unit's size follows the light its bud gets. Dormant where no leaf shades."),
    row("apical_control", Linear, SHARE, "share",
        "The share of vigour the continuing axis keeps against its laterals. Dormant without shade size."),
    row("upkeep", Log { floor: 1e-3 }, SHADE, "light per metre",
        "What a metre of present wood costs in light. Dormant without a balance hazard."),
    row("balance_hazard", Log { floor: 0.01 }, SHADE, "per year",
        "How fast a lateral whose carbon balance falls below tolerance is shed. Dormant without upkeep, and on an axis that bears no laterals."),
    row("tolerance", Linear, [-1.0, 1.0], "balance",
        "The carbon balance below which a lateral starts to be shed: shade tolerance. Dormant without a balance hazard."),
    row("retained", Linear, SHARE, "share",
        "The share of a shed branch's pipe that stays in its bearer's girth. Dormant where nothing is shed."),
    row("leaf_girth", Linear, SHADE, "exponent",
        "How a phytomer's own pipe follows its leaves' light. Dormant where no leaf shades."),
    row("form.tropism", Log { floor: 0.1 }, RATE, "per m",
        "How fast the axis bends towards its elevation."),
    row("form.elevation", Linear, [-FRAC_PI_2, FRAC_PI_2], "rad",
        "The elevation the axis bends towards, above the horizontal."),
    row("form.wander", Log { floor: 0.05 }, RATE, "rad per m",
        "How far each node turns the axis at random: sinuosity."),
    row("form.plane", Linear, [-TAU, TAU], "rad",
        "The turn of the plane a lateral of this axis branches in. Dormant on an axis that bears no laterals."),
    row("form.pipe", Log { floor: 1e-5 }, [0.0, MAX_PIPE], "m",
        "The pipe radius each phytomer adds below it: how thick the wood grows."),
    row("form.exponent", Linear, [MIN_EXPONENT, MAX_EXPONENT], "-",
        "The pipe model's exponent; larger keeps limbs thicker beside their bearer. Dormant on an axis that bears no laterals."),
    row("form.ripening", Log { floor: 1.0 }, [0.0, MAX_RIPENING], "years",
        "The years a phytomer takes to lay down its own pipe. Dormant where the girth keeps its established width (secondary 0)."),
    row("form.dominance", Linear, SHARE, "share",
        "How unequal sibling laterals are, so a few dominate. Dormant on an axis that bears no laterals."),
    row("form.roll", Linear, [0.0, PI], "rad",
        "How far a lateral turns about its parent, so siblings do not stack. Dormant on an axis that bears no laterals."),
    row("form.sag", Log { floor: 1e-5 }, [0.0, MAX_SAG], "-",
        "How far the axis bends under the load it carries."),
    row("form.secondary", Linear, SHARE, "share",
        "How far the girth follows the load; 0 keeps the established width."),
    row("form.bend_length", Log { floor: 0.1 }, [0.0, MAX_BEND_LENGTH], "m",
        "How far the wander remembers its bend: slow arcs rather than kinks. Dormant without wander."),
    row("form.internode_spread", Linear, SHARE, "share",
        "How far each internode's length departs from its age's."),
    row("form.insertion_spread", Linear, [0.0, FRAC_PI_2], "rad",
        "How far each lateral's insertion angle departs from its age's. Dormant on an axis that bears no laterals."),
    row("form.tropism_spread", Linear, SHARE, "share",
        "How far each axis's tropism departs from its age's."),
    row("zone.nodes.min", Linear, [0.0, MAX_NODES_PER_ZONE as f64], "nodes",
        "The fewest nodes a zone of the growth unit holds."),
    row("zone.nodes.max", Linear, [0.0, MAX_NODES_PER_ZONE as f64], "nodes",
        "The most nodes a zone of the growth unit holds."),
    row("zone.buds", Linear, [1.0, MAX_BUDS as f64], "buds",
        "Lateral buds per node: 1 alternate, 2 opposite, more a whorl. Dormant on an axis that bears no laterals."),
    row("zone.lateral", Linear, SHARE, "probability",
        "The chance a bud carries each age; the rest is bare."),
    row("zone.dormant", Linear, SHARE, "probability",
        "The chance a bud's place also holds a sleeping bud of each age."),
    row("zone.delay", Log { floor: 0.1 }, [0.0, MAX_DELAY], "years",
        "The years before a sleeping bud can wake. Dormant without sleeping buds."),
    row("zone.rate", Log { floor: 0.01 }, RATE, "per year",
        "A sleeping bud's yearly waking hazard after its delay. Dormant without sleeping buds."),
];
