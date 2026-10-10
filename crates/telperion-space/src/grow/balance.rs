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
use crate::contract::shed::{walk, Bud, Lateral, Walked};
use crate::lineage::{Key, SHED};

/// Each laid axis's subtree's collected light, its wood's upkeep and
/// whether anything in it lives, summed child to parent, the last child
/// first: what a fine root reports its bearer (PHASES.md, phase (i)).
pub(super) struct Totals {
    pub light: Vec<f32>,
    pub cost: Vec<f32>,
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
    pub(super) fn totals(&self, rows: &Rows, given: &[(usize, f32, f32, bool)]) -> Totals {
        let (species, pa) = (self.species, &rows.pa);
        let sketch = self.sketch.as_ref().expect("sketching");
        let n = sketch.pencils.len();
        let (mut light, mut cost, mut living) = (vec![0.0f32; n], vec![0.0f32; n], vec![false; n]);
        let pencils = &sketch.pencils;
        crate::cores::fill(&mut cost, 1 << 15, self.threads, &|i| {
            species.states[pa[i]].upkeep as f32 * pencils[i].wood as f32
        });
        for apex in self.live.iter().filter(|a| a.axis < n) {
            let p = &sketch.pencils[apex.axis];
            light[apex.axis] = p.base_scale as f32 * p.running as f32 * p.light as f32;
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
        rows: &Rows,
        totals: &Totals,
        root: Option<(bool, f64)>,
    ) -> (Vec<bool>, Vec<f64>) {
        let laterals = self.laterals(cycle, rows, totals);
        let root = root.map(|(shed, kept)| (shed, kept as f32));
        let walked = walk(&laterals, root);
        self.shed_walked(cycle, &walked)
    }

    /// What D16's walk reads of each laid axis (contract A, 2.7): its
    /// bearer, whether it is a living lateral, and its balance's inputs.
    pub(super) fn laterals(
        &self,
        cycle: u32,
        Rows { links, pa }: &Rows,
        Totals {
            light,
            cost,
            living,
        }: &Totals,
    ) -> Vec<Lateral> {
        let species = self.species;
        let sketch = self.sketch.as_ref().expect("sketching");
        let n = sketch.pencils.len();
        let mut out = vec![Lateral::default(); n];
        crate::cores::fill(&mut out, 1 << 12, self.threads, &|i| {
            let parent = links[i].parent.map(|p| p as u32);
            let decides = parent.is_some() && links[i].node != TIP && living[i];
            if !decides {
                return Lateral {
                    parent,
                    ..Lateral::default()
                };
            }
            let state = &species.states[pa[i]];
            let d = Key(self.axes[i].lineage)
                .child(SHED)
                .child(u64::from(cycle))
                .draw();
            // D16 (CONTRACT-A.md 2.7): the window is the presences' at
            // the cycle, stake one.
            let window = self.rules.windows.window_q((pa[i], cycle, None), 1.0);
            let bud = Bud {
                q: light[i],
                c: cost[i],
                memory: sketch.pencils[i].memory as f32,
                hazard: state.balance_hazard as f32,
                tolerance: state.tolerance as f32,
                draw: d.0,
                window,
                keep: 0.0,
            };
            Lateral {
                parent,
                decides,
                bud,
            }
        });
        out
    }

    /// The walk's decisions taken: each memory kept, every apex shed
    /// leaving the living, every other given its kept share.
    pub(super) fn shed_walked(&mut self, cycle: u32, walked: &[Walked]) -> (Vec<bool>, Vec<f64>) {
        let sketch = self.sketch.as_mut().expect("sketching");
        let n = sketch.pencils.len();
        for (pencil, w) in sketch.pencils.iter_mut().zip(walked) {
            if let Some(memory) = w.memory {
                pencil.memory = f64::from(memory);
            }
        }
        let shed: Vec<bool> = walked.iter().map(|w| w.shed).collect();
        let kept: Vec<f64> = walked.iter().map(|w| f64::from(w.kept)).collect();
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
        (shed, kept)
    }
}
