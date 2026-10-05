//! Every input the engine cannot draw, refused by name: each age's
//! settings, its form and its zones, each on its rail.
use super::*;
use crate::error::{refuse, Result};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

impl PaState {
    pub(super) fn validate(&self, at: &str, pa: usize, count: usize) -> Result<()> {
        if !(0.0..=1.0).contains(&self.viability) {
            return refuse(format!("{at}.viability"), "a probability lies in 0 to 1");
        }
        if self.zones.is_empty() || self.zones.len() > ZONES {
            return refuse(
                format!("{at}.zones"),
                "a growth unit holds 1 to 4 zones, one for each role",
            );
        }
        for (z, zone) in self.zones.iter().enumerate() {
            zone.validate(&format!("{at}.zones[{z}]"), pa, count)?;
        }
        if !(0.0..=MAX_LIFESPAN).contains(&self.lifespan) {
            return refuse(
                format!("{at}.lifespan"),
                "a lifespan lies in 0 to 1000000 cycles",
            );
        }
        // An age of no lifespan is passed through: it moves on, as its
        // limit of a lifespan all but none decides by its draw (Codex
        // round 7 on fn-206).
        if self.lifespan == 0.0 && self.continuation < 1.0 {
            return refuse(
                format!("{at}.continuation"),
                "an age of no lifespan is passed through and moves on",
            );
        }
        if self.shedding.is_nan() || self.shedding < 0.0 {
            return refuse(format!("{at}.shedding"), "a delay is 0 or more cycles");
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
            ("leaf_area", self.leaf_area, MAX_LEAF_AREA),
            ("shade_hazard", self.shade_hazard, MAX_SHADE),
            ("shade_size", self.shade_size, MAX_SHADE),
            ("upkeep", self.upkeep, MAX_SHADE),
            ("balance_hazard", self.balance_hazard, MAX_SHADE),
            ("leaf_girth", self.leaf_girth, MAX_SHADE),
        ];
        for (name, value, most) in rates {
            if !(0.0..=most).contains(&value) {
                return refuse(format!("{at}.{name}"), "a rate lies in its bounded range");
            }
        }
        let shares = [
            ("continuation", self.continuation),
            ("abortion", self.abortion),
            ("relay", self.relay),
            ("relay_ended", self.relay_ended),
            ("relay_failed", self.relay_failed),
            ("relay_at", self.relay_at),
            ("epitony", self.epitony),
            ("readiness", self.readiness),
            ("rhythm", self.rhythm),
            ("straightening", self.straightening),
            ("apical_control", self.apical_control),
            ("retained", self.retained),
        ];
        for (name, value) in shares {
            if !(0.0..=1.0).contains(&value) {
                return refuse(format!("{at}.{name}"), "a share lies in 0 to 1");
            }
        }
        if !(-1.0..=1.0).contains(&self.tolerance) {
            return refuse(format!("{at}.tolerance"), "a balance lies in -1 to 1");
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
            ("bend_length", self.bend_length, MAX_BEND_LENGTH),
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

impl Zone {
    fn validate(&self, at: &str, pa: usize, count: usize) -> Result<()> {
        match self.nodes {
            NodeLaw::Uniform { min, max } if !(0.0..=max).contains(&min) => {
                return refuse(
                    format!("{at}.nodes.min"),
                    "the least node count exceeds the most",
                );
            }
            NodeLaw::Uniform { max, .. } if max.is_nan() || max > f64::from(MAX_NODES_PER_ZONE) => {
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
        if !(1.0..=f64::from(MAX_BUDS)).contains(&self.buds) {
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
