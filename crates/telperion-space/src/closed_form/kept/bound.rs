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
use super::{sub, Kept};
use crate::closed_form::along::Count;
use crate::closed_form::steps::{first_step, rest};
use crate::dormant::First;
use crate::schedule::Carried;

/// The chance a zone's node count passes the quantile the bound takes.
pub(crate) const TAIL: f64 = 1e-9;

impl Kept<'_> {
    /// The longest path from the base of a bud of PA `k` made `m` cycles
    /// before the tree's age to any wood it can grow, per unit base scale,
    /// within the tail `TAIL` a zone.
    pub fn reach_bound(
        &mut self,
        k: usize,
        m: usize,
        carried: Carried,
        first: Option<First>,
    ) -> f64 {
        let Some((k, reach)) = self.species.lived(k) else {
            return 0.0;
        };
        if m == 0 {
            return 0.0;
        }
        let carried = {
            let state = &self.species.states[k];
            carried.canonical(state.lifespan, state.rises(), m)
        };
        let key = sub(self.species, k, m, carried, first);
        if let Some(&b) = self.bounds.get(&key) {
            return reach * b;
        }
        let species = self.species;
        let state = &species.states[k];
        // Each zone's node count at its quantile.
        let most: Vec<f64> = state
            .zones
            .iter()
            .map(|z| quantile(&Count::pmf(z.nodes)))
            .collect();
        let mut path = 0.0f64;
        let mut best = 0.0f64;
        let step = first_step(species, &mut self.living, (k, m, carried), first);
        // The rest of its axis stands on the first unit's tip: the bud one
        // cycle younger (fn-209, P1).
        if let Some(s) = step
            .as_ref()
            .filter(|s| s.end.is_none() && s.after > 0.0 && m > 1)
        {
            let length = most.iter().sum::<f64>() * state.internode * s.size;
            let rest = self.reach_bound(k, m - 1, rest(carried, state.lifespan), None);
            best = best.max(length + rest);
        }
        if let Some(step) = step {
            // A failed unit's relay stands on the unit before it.
            if let Some(r) = step.failed {
                best = best.max(path + self.reach_bound(k, m - r.at, r.carried, None));
            }
            path += most.iter().sum::<f64>() * state.internode * step.size;
            best = best.max(path);
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
                if let crate::closed_form::events::Event::Bud {
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
        }
        self.bounds.insert(key, best);
        reach * best
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
