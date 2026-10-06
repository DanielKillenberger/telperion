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
use super::sketch::Rows;
use super::Grower;
use crate::allocation::TIP;
use crate::lineage::{above, Key, SHED};

/// The share of last year's remembered balance kept each year: a
/// half-life of a year. An estimate, unsourced.
pub(crate) const MEMORY: f64 = 0.5;

/// Each laid axis's subtree's collected light, its wood's upkeep and
/// whether anything in it lives, summed child to parent in reverse index
/// order: what a fine root reports its bearer (PHASES.md, phase (i)).
pub(super) struct Totals {
    pub light: Vec<f64>,
    pub cost: Vec<f64>,
    pub living: Vec<bool>,
}

impl Grower<'_> {
    /// Whether the balance runs this cycle: where any PA's hazard is set
    /// and a rough layout is grown.
    pub(super) fn balances(&self) -> bool {
        self.sketch.is_some() && self.species.states.iter().any(|s| s.balance_hazard != 0.0)
    }

    /// Updates every living lateral subtree's balance, sheds by it, and
    /// gives each living apex what its subtrees' survival makes of its
    /// unit this cycle.
    pub(super) fn balance(&mut self, cycle: u32, rows: Option<&Rows>) {
        let Some(rows) = rows.filter(|_| self.balances()) else {
            return;
        };
        let sketch = self.sketch.as_ref().expect("sketching");
        // A fine root's subtree as its aggregate holds it (fn-211.4).
        let fine = self.fine_balance(sketch);
        let totals = self.totals(rows, &fine);
        // A fine subtree's root: its own shedding is the pass's record.
        let root = self
            .fine
            .as_ref()
            .map(|r| (r.records.shed == Some(cycle), r.records.kept(cycle)));
        let (shed, kept) = self.shed_by(cycle, rows, &totals, root);
        self.fine_kept(cycle, &shed, &kept);
    }

    /// Each subtree's totals, `given` (axis, light, upkeep, living) taking
    /// the place of what axis holds before they are summed.
    pub(super) fn totals(&self, rows: &Rows, given: &[(usize, f64, f64, bool)]) -> Totals {
        let (species, pa) = (self.species, &rows.pa);
        let sketch = self.sketch.as_ref().expect("sketching");
        let n = sketch.pencils.len();
        let (mut light, mut cost, mut living) = (vec![0.0; n], vec![0.0; n], vec![false; n]);
        let pencils = &sketch.pencils;
        crate::cores::fill(&mut cost, 1 << 15, self.threads, &|i| {
            species.states[pa[i]].upkeep * pencils[i].wood
        });
        for apex in self.live.iter().filter(|a| a.axis < n) {
            let p = &sketch.pencils[apex.axis];
            light[apex.axis] = p.base_scale * p.running * p.light;
            living[apex.axis] = true;
        }
        for &(i, q, c, alive) in given {
            (light[i], cost[i], living[i]) = (q, c, alive);
        }
        for i in (1..n).rev() {
            if let Some(parent) = rows.links[i].parent {
                light[parent] += light[i];
                cost[parent] += cost[i];
                living[parent] |= living[i];
            }
        }
        Totals {
            light,
            cost,
            living,
        }
    }

    /// Parents first, what each subtree's shedding makes of every axis in
    /// it this cycle (axis 1's given as `root`, where it is a subtree's
    /// root whose bearer decides it); every apex shed leaves the living.
    /// Each axis's shedding and kept share.
    pub(super) fn shed_by(
        &mut self,
        cycle: u32,
        Rows { links, pa }: &Rows,
        Totals {
            light,
            cost,
            living,
        }: &Totals,
        root: Option<(bool, f64)>,
    ) -> (Vec<bool>, Vec<f64>) {
        let species = self.species;
        let mut held = self.sketch.take().expect("sketching");
        let sketch = &mut held;
        let n = sketch.pencils.len();
        let (mut shed, mut kept) = (vec![false; n], vec![1.0; n]);
        for i in 0..n {
            let (dead, keep) = match links[i].parent {
                Some(parent) => (shed[parent], kept[parent]),
                None => (false, 1.0),
            };
            (shed[i], kept[i]) = (dead, keep);
            if let (1, Some((when, k))) = (i, root) {
                (shed[i], kept[i]) = (when, k);
                continue;
            }
            let lateral = links[i].parent.is_some() && links[i].node != TIP;
            if dead || !living[i] || !lateral {
                continue;
            }
            let state = &species.states[pa[i]];
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
            let d = Key(self.axes[i].lineage)
                .child(SHED)
                .child(u64::from(cycle))
                .draw();
            let u = d.unit();
            if d.under(p) {
                shed[i] = true;
            } else {
                let windows = &self.rules.windows;
                let wood = windows.wood(pa[i], cycle);
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
        self.sketch = Some(held);
        (shed, kept)
    }
}
