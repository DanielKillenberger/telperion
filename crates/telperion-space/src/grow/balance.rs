//! The carbon balance (fn-197 step 4; DESIGN-OPTIONS.md, section 5).
//! Before each cycle's growth, every lateral subtree's balance is its
//! buds' light against its wood's upkeep, (Q - C) / (Q + C), remembered as
//! a running average. A lateral is shed with a yearly hazard that rises
//! continuously as its remembered balance falls below its PA's tolerance
//! (host decision 5): a draw under its lineage and the cycle. Where it
//! fires, every apex in the subtree stops before it grows, and the idle
//! rule drops the subtree (`shed.rs`); where it does not, the draw's lead
//! is a presence that every unit the subtree grows this cycle carries,
//! so a lateral a setting is about to shed fades before it is shed. The
//! main line is never tested: the seed and what carries it on are no
//! laterals.
use super::Grower;
use crate::lineage::{above, Key, SHED};
use crate::structure::Origin;

/// The share of last year's remembered balance kept each year: a
/// half-life of a year. An estimate, unsourced.
pub(crate) const MEMORY: f64 = 0.5;

impl Grower<'_> {
    /// Updates every living lateral subtree's balance, sheds by it, and
    /// gives each living apex what its subtrees' survival makes of its
    /// unit this cycle.
    pub(super) fn balance(&mut self, cycle: u32) {
        let species = self.species;
        if species.states.iter().all(|s| s.balance_hazard == 0.0) {
            return;
        }
        let Some(sketch) = self.sketch.as_mut() else {
            return;
        };
        let n = sketch.pencils.len();
        let (mut light, mut cost, mut living) = (vec![0.0; n], vec![0.0; n], vec![false; n]);
        for i in 0..n {
            let p = &sketch.pencils[i];
            cost[i] = species.states[self.axes[i].pa].upkeep * p.wood;
        }
        for apex in self.live.iter().filter(|a| a.axis < n) {
            let p = &sketch.pencils[apex.axis];
            light[apex.axis] = p.base_scale * p.running * p.light;
            living[apex.axis] = true;
        }
        for i in (1..n).rev() {
            if let Some(parent) = self.axes[i].origin.parent() {
                light[parent] += light[i];
                cost[parent] += cost[i];
                living[parent] |= living[i];
            }
        }
        // Parents first: what each subtree's shedding makes of every axis
        // in it this cycle.
        let (mut shed, mut kept) = (vec![false; n], vec![1.0; n]);
        for i in 0..n {
            let (dead, keep) = match self.axes[i].origin.parent() {
                Some(parent) => (shed[parent], kept[parent]),
                None => (false, 1.0),
            };
            (shed[i], kept[i]) = (dead, keep);
            if dead || !living[i] || !matches!(self.axes[i].origin, Origin::Lateral { .. }) {
                continue;
            }
            let state = &species.states[self.axes[i].pa];
            let total = light[i] + cost[i];
            let now = if total > 0.0 {
                (light[i] - cost[i]) / total
            } else {
                1.0
            };
            let pencil = &mut sketch.pencils[i];
            pencil.memory = MEMORY * pencil.memory + (1.0 - MEMORY) * now;
            let hazard = state.balance_hazard * (state.tolerance - pencil.memory).max(0.0);
            if hazard <= 0.0 {
                continue;
            }
            let p = -(-hazard).exp_m1();
            let u = Key(self.axes[i].lineage)
                .child(SHED)
                .child(u64::from(cycle))
                .unit();
            if u < p {
                shed[i] = true;
            } else {
                let windows = &self.rules.windows;
                let wood = windows.wood(self.axes[i].pa, cycle);
                kept[i] = keep * windows.decided(above(u, p), wood, 1.0);
            }
        }
        let live = std::mem::take(&mut self.live);
        for apex in live {
            if apex.axis < n && shed[apex.axis] {
                self.axes[apex.axis].apex_end = Some(cycle - 1);
                continue;
            }
            if apex.axis < n {
                sketch.pencils[apex.axis].kept = kept[apex.axis];
            }
            self.live.push(apex);
        }
    }
}
