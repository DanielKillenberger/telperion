//! How far a bud's wood can reach (fn-209, host decisions 16): the
//! longest path from its base to any wood its subtree can grow, with every
//! bud made, every unit grown that the schedule allows, every relay and
//! continuation taken, and each zone's node count at its 1 − 1e−9
//! quantile. The envelope's reach is a root mean square, never a bound;
//! this is one, with its tail named.
//!
//! **Tail:** the bound holds unless some zone's node count passes its
//! quantile. Each zone's draw does so with chance at most `TAIL` (1e−9),
//! so over a subtree whose units draw Z zones it holds with chance at
//! least 1 − Z · 1e−9. A uniform node law is bounded outright. A straight
//! path is the farthest wood can stand from its base, whatever bends it.
use super::{Kept, Table};
use crate::closed_form::along::Count;
use crate::closed_form::events::Event;
use crate::closed_form::steps::Step;
use crate::dormant::First;
use crate::schedule::Carried;

/// The chance a zone's node count passes the quantile the bound takes.
pub(crate) const TAIL: f64 = 1e-9;

impl Kept<'_> {
    /// The longest path from the base of a bud of PA `k` made `m` cycles
    /// before the tree's age to any wood it can grow, per unit base scale,
    /// within the tail `TAIL` a zone: worked out with its table.
    pub fn reach_bound(
        &mut self,
        k: usize,
        m: usize,
        carried: Carried,
        first: Option<First>,
    ) -> f64 {
        self.of(k, m, carried, first).reach
    }

    /// The bound of a bud of the living PA `k` from its first unit `step`
    /// and the rest of its axis `rest`, which stands on the unit's tip.
    pub(super) fn bound(
        &mut self,
        (k, m): (usize, usize),
        step: Option<&Step>,
        rest: Option<&Table>,
    ) -> f64 {
        let Some(step) = step else {
            return 0.0;
        };
        let species = self.species;
        let state = &species.states[k];
        // Each zone's node count at its quantile.
        let most: f64 = state
            .zones
            .iter()
            .map(|z| quantile(&Count::pmf(z.nodes)))
            .sum();
        // Each internode at its longest (fn-222).
        let longest = state.internode * (1.0 + state.form.internode_spread);
        let path = most * longest * step.size;
        let mut best = path + rest.map_or(0.0, |r| r.reach);
        // A failed unit's relay stands on the unit before it, the base.
        if let Some(r) = step.failed {
            best = best.max(self.reach_bound(k, m - r.at, r.carried, None));
        }
        // Laterals stand at most at the unit's tip, no larger than its
        // size (a place's share and a dominance share are at most 1).
        let laterals = self.laterals.clone();
        for lateral in &laterals[k] {
            for (j, _) in lateral.iter().enumerate().filter(|(_, &p)| p > 0.0) {
                let child = self.reach_bound(j, m - step.i, Carried::default(), None);
                best = best.max(path + step.size * child);
            }
        }
        for event in step.sleeping.iter().flatten() {
            if let Event::Bud {
                at,
                pa,
                carried,
                first,
                unit,
                ..
            } = *event
            {
                best = best.max(path + unit * self.reach_bound(pa, m - at, carried, first));
            }
        }
        if let Some(r) = step.aborted {
            best = best.max(path + self.reach_bound(k, m - r.at, r.carried, None));
        }
        if let Some(end) = step.end {
            if let Some((at, pa, onward, _)) = end.continuation {
                best = best.max(path + self.reach_bound(pa, m - at, onward, None));
            }
            if let Some(r) = end.relay {
                best = best.max(path + self.reach_bound(k, m - r.at, r.carried, None));
            }
        }
        best
    }
}

/// The least count whose cumulative chance reaches 1 − `TAIL`.
fn quantile(pmf: &[f64]) -> f64 {
    let mut total = 0.0;
    for (n, &p) in pmf.iter().enumerate() {
        total += p;
        if total >= 1.0 - TAIL {
            return n as f64;
        }
    }
    (pmf.len() - 1) as f64
}
