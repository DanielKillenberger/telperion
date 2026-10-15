//! Every input the engine cannot draw, refused by name: each age's
//! settings, its form and its zones, each on its rail. A single range is
//! the setting's row in the table (`scale::SETTINGS`); a check across
//! settings is written here.
use super::*;
use crate::error::{refuse, Result};

/// Whether the setting `name` admits `v` on its own.
fn admits(name: &str, v: f64) -> bool {
    scale::setting(name).is_some_and(|s| s.admits(v))
}

impl PaState {
    pub(super) fn validate(&self, at: &str, pa: usize, count: usize) -> Result<()> {
        if !admits("viability", self.viability) {
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
        if !admits("lifespan", self.lifespan) {
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
        if !admits("shedding", self.shedding) {
            return refuse(format!("{at}.shedding"), "a delay is 0 or more cycles");
        }
        if !admits("internode", self.internode) {
            return refuse(
                format!("{at}.internode"),
                "an internode is longer than 0 and at most 100 m",
            );
        }
        if !admits("insertion", self.insertion) {
            return refuse(
                format!("{at}.insertion"),
                "an insertion angle lies in 0 to pi",
            );
        }
        let rates = [
            ("abortion_rise", self.abortion_rise),
            ("erection", self.erection),
            ("leaf_area", self.leaf_area),
            ("shade_hazard", self.shade_hazard),
            ("shade_size", self.shade_size),
            ("upkeep", self.upkeep),
            ("balance_hazard", self.balance_hazard),
            ("leaf_girth", self.leaf_girth),
        ];
        for (name, value) in rates {
            if !admits(name, value) {
                return refuse(format!("{at}.{name}"), "a rate lies in its bounded range");
            }
        }
        let shares = [
            ("ground_hazard", self.ground_hazard),
            ("flowering", self.flowering),
            ("continuation", self.continuation),
            ("abortion", self.abortion),
            ("relay", self.relay),
            ("relay_ended", self.relay_ended),
            ("relay_failed", self.relay_failed),
            ("relay_at", self.relay_at),
            ("epitony", self.epitony),
            ("takeover", self.takeover),
            ("readiness", self.readiness),
            ("rhythm", self.rhythm),
            ("straightening", self.straightening),
            ("apical_control", self.apical_control),
            ("retained", self.retained),
        ];
        for (name, value) in shares {
            if !admits(name, value) {
                return refuse(format!("{at}.{name}"), "a share lies in 0 to 1");
            }
        }
        let leaves = self.cluster_leaves;
        if !admits("cluster_leaves", leaves) {
            return refuse(
                format!("{at}.cluster_leaves"),
                "a cluster holds 0 to 8 leaves",
            );
        }
        if !admits("cluster_spread", self.cluster_spread) {
            return refuse(
                format!("{at}.cluster_spread"),
                "a cluster's spread lies in 0 to pi / 2",
            );
        }
        if !admits("leaf_years", self.leaf_years) {
            return refuse(
                format!("{at}.leaf_years"),
                "a leaf's years lie in 0 to never",
            );
        }
        // A shoot and the leaf at its last node are whole within the year
        // (fn-270 R5), so a whole year draws them whole.
        let days = [
            ("elongation", self.elongation),
            ("expansion", self.expansion),
        ];
        for (name, value) in days {
            if !admits(name, value) {
                return refuse(format!("{at}.{name}"), "a span of days lies in 0 to 365");
            }
        }
        if self.elongation + self.expansion > crate::unfold::YEAR {
            return refuse(
                format!("{at}.expansion"),
                "a shoot and its last leaf unfold within the year",
            );
        }
        // R5c: a leaf's lifespan, and its season's days on the calendar.
        if !admits("leaf_life", self.leaf_life) {
            return refuse(
                format!("{at}.leaf_life"),
                "a leaf's years lie in 0 to never",
            );
        }
        let calendar = [
            ("budburst", self.budburst),
            ("senescence", self.senescence),
            ("colouring", self.colouring),
            ("leaf_fall", self.leaf_fall),
        ];
        for (name, value) in calendar {
            if !admits(name, value) {
                return refuse(format!("{at}.{name}"), "a day of the year lies in 0 to 365");
            }
        }
        if !admits("tolerance", self.tolerance) {
            return refuse(format!("{at}.tolerance"), "a balance lies in -1 to 1");
        }
        if !admits("divergence", self.divergence) {
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
            ("tropism", self.tropism),
            ("wander", self.wander),
            ("pipe", self.pipe),
            ("ripening", self.ripening),
            ("sag", self.sag),
            ("bend_length", self.bend_length),
            ("internode_spread", self.internode_spread),
            ("insertion_spread", self.insertion_spread),
            ("tropism_spread", self.tropism_spread),
        ];
        for (name, value) in rates {
            if !admits(&format!("form.{name}"), value) {
                return refuse(format!("{at}.{name}"), "a rate lies in its bounded range");
            }
        }
        if !admits("form.elevation", self.elevation) {
            return refuse(
                format!("{at}.elevation"),
                "an elevation lies in -pi / 2 to pi / 2",
            );
        }
        if !admits("form.exponent", self.exponent) {
            return refuse(format!("{at}.exponent"), "a pipe exponent lies in 1.5 to 4");
        }
        for (name, value) in [("dominance", self.dominance), ("secondary", self.secondary)] {
            if !admits(&format!("form.{name}"), value) {
                return refuse(format!("{at}.{name}"), "a share lies in 0 to 1");
            }
        }
        if !admits("form.roll", self.roll) {
            return refuse(format!("{at}.roll"), "a roll lies in 0 to pi");
        }
        if !admits("form.plane", self.plane) {
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
            NodeLaw::Uniform { max, .. } if !admits("zone.nodes.max", max) => {
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
        if !admits("zone.buds", self.buds) {
            return refuse(format!("{at}.buds"), "a node carries 1 to 6 buds");
        }
        table(
            &format!("{at}.lateral"),
            &self.lateral,
            pa,
            count,
            "zone.lateral",
        )?;
        table(
            &format!("{at}.dormant"),
            &self.dormant,
            pa,
            count,
            "zone.dormant",
        )?;
        if !admits("zone.delay", self.delay) {
            return refuse(format!("{at}.delay"), "a delay lies in 0 to 1000 years");
        }
        if !admits("zone.rate", self.rate) {
            return refuse(format!("{at}.rate"), "a rate lies in its bounded range");
        }
        Ok(())
    }
}

/// Refuses a table of bud probabilities, one per PA, that is not one.
fn table(at: &str, table: &[f64], pa: usize, count: usize, name: &str) -> Result<()> {
    if table.len() != count {
        return refuse(at, "one probability per physiological age");
    }
    for (j, &p) in table.iter().enumerate() {
        if !admits(name, p) {
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
