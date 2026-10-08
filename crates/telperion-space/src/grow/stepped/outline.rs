//! R3 and R4 beside today's model, for the head-to-head only (fn-226 task
//! 3; R4-INVENTORY.md sections 3, 4 and 7). The structure above the cut
//! is grown first, through every cycle. Each fine root stands in it as
//! fn-211.4's `Aggregate`: in its bearer's balance by its light, upkeep
//! and living, and in the vigour split by its lit presence. Each cycle
//! the structure above folds the crown's outline from its own leaves and
//! each root's expected leaves (S7), and records it with each root's
//! shedding, kept share, memory and incoming vigour. Every bud's light is
//! read from the last cycle's outline at its own tip, and every size is
//! normalised by the mean the outline implies, `T̄^ψ` (S5), not by the
//! tree's sums. Then every root is grown ahead alone (`ahead.rs`), reading
//! the history at its own cycle: nothing it grows reaches the structure
//! above.
use super::{Counts, Executor, Keep};
use crate::allocation::{size_against, split};
use crate::closed_form::boundary::Rows;
use crate::closed_form::predictor::Predictor;
use crate::couple::{Aggregate, Inputs as Reads};
use crate::error::{Error, Result};
use crate::fine::Plan;
use crate::fixed::{q32, ONE32};
use crate::grow::partition::{Grouping, Order};
use crate::grow::sketch::Inputs;
use crate::grow::Grower;
use crate::grow::{start, Options, Request};
use crate::light::field::q;
use crate::light::outline::fold::Fold;
use crate::light::outline::Outline;
use crate::species::Species;
use crate::structure::Vec3;
use crate::transcendental::Transcendental;
use std::sync::Arc;

/// Per cycle and PA, the growing buds' presence and presence times size:
/// what host decision 23's drift reads.
pub(super) type Growth = Vec<Vec<[f64; 2]>>;

/// What the structure above records for the roots, one way.
pub(super) struct History {
    /// Each cycle's outline, after its growth, by cycle.
    pub outlines: Vec<Option<Outline>>,
    /// Each root's record, by item.
    pub ledgers: Vec<Ledger>,
}

impl History {
    /// The outline of `cycle`: none before the first.
    pub fn outline(&self, cycle: u32) -> Option<&Outline> {
        self.outlines.get(cycle as usize)?.as_ref()
    }
}

/// A fine root's record by cycle: what its bearer decided and handed it.
#[derive(Clone)]
pub(super) struct Ledger {
    pub shed: Vec<bool>,
    pub kept: Vec<f64>,
    pub memory: Vec<f64>,
    pub vigour: Vec<f64>,
}

impl Ledger {
    fn new(age: u32) -> Self {
        let n = age as usize + 2;
        Self {
            shed: vec![false; n],
            kept: vec![1.0; n],
            memory: vec![1.0; n],
            vigour: vec![0.0; n],
        }
    }
}

/// A fine root as the structure above holds it: its placeholder, PA,
/// birth, base scale and base, its expectation, the light it read and
/// its vigour per presence this cycle.
struct Standin {
    at: usize,
    pa: usize,
    birth: u32,
    scale: f64,
    base: Vec3,
    agg: Aggregate,
    light: f64,
    ratio: f64,
    dead: bool,
}

/// The structure above's own state beside the executor's.
pub(super) struct Above {
    stands: Vec<Standin>,
    pub history: History,
    pub growth: Growth,
    rows: Rows,
}

/// A Q32 as its exact real.
fn real(x: i64) -> f64 {
    x as f64 / ONE32 as f64
}

/// The light at `p`: 1 before the first outline.
pub(super) fn light_at(o: Option<&Outline>, p: Vec3) -> f64 {
    o.map_or(1.0, |o| real(o.at(q(p))))
}

/// The mean of `(v/p)^ψ` the outline implies, `T̄^ψ` (S5), Q32.
fn mean(o: Option<&Outline>, psi: f64) -> i64 {
    let tbar = o.map_or(ONE32, |o| o.mean);
    crate::allocation::drawn(tbar, ONE32, q32(psi))
}

/// Every living apex's light, read at its tip from `o` on `threads`
/// cores: inline where the bud grows, nothing stored but its own read.
pub(super) fn read(g: &mut Grower, o: Option<&Outline>, threads: usize) {
    let Some(n) = g.sketch.as_ref().map(|s| s.pencils.len()) else {
        return;
    };
    let at: Vec<usize> = g.live.iter().map(|a| a.axis).filter(|&i| i < n).collect();
    let tips: Vec<Vec3> = at.iter().map(|&i| g.tip(i)).collect();
    let mut lit = vec![1.0; at.len()];
    crate::cores::fill(&mut lit, 1 << 10, threads, &|k| light_at(o, tips[k]));
    let sketch = g.sketch.as_mut().expect("sketching");
    for (&i, &l) in at.iter().zip(&lit) {
        sketch.pencils[i].light = l;
    }
}

/// Every growing bud of `g` sized against the outline's means (R4), its
/// presence and size added to `growth`.
pub(super) fn size(
    g: &mut Grower,
    (got, inputs, pa): (&[f64], &Inputs, &[usize]),
    o: Option<&Outline>,
    (cycle, growth): (u32, &mut Growth),
) {
    let species = g.species;
    let means: Vec<i64> = (species.states.iter())
        .map(|s| mean(o, s.shade_size))
        .collect();
    let sized: Vec<f64> = (0..got.len())
        .map(|i| {
            let x = [got[i], inputs.presence[i], inputs.psi[i]].map(q32);
            real(size_against(x[0], x[1], x[2], means[pa[i]]))
        })
        .collect();
    for apex in &g.live {
        let i = apex.axis;
        if i < got.len() && inputs.presence[i] > 0.0 {
            crate::grow::subtree::tally(growth, cycle, pa[i], inputs.presence[i], sized[i]);
        }
    }
    g.set_sizes(&sized);
}

impl<'a> Executor<'a> {
    /// The build's executor (`build`'s cut, order and chunking) about to
    /// grow R3 and R4's structure above; none where no bud reads light.
    pub(super) fn outlined(species: &'a Species, request: Request) -> Result<Option<Self>> {
        let plan = Plan::default();
        let (coarse, _) = start(species, request, true, Options::default())?;
        if coarse.sketch.is_none() {
            return Ok(None);
        }
        let predictor = Predictor::new(species, &coarse.rules.windows, request.age);
        let counts: Counts =
            Arc::new(move |pa, birth| predictor.count(pa, birth).unwrap_or(f64::INFINITY));
        let grouping = Grouping {
            order: Order::Forward,
            chunk: plan.chunk,
            sizes: true,
            leaves: true,
        };
        let mut x = Executor::new(coarse, (counts, plan.cut, grouping), request, Keep::Tree);
        x.plan = plan;
        Ok(Some(x))
    }
}

impl Executor<'_> {
    /// The structure above, before its first cycle.
    pub(super) fn above(&self) -> Above {
        let (species, age) = (self.coarse.species, self.request.age);
        Above {
            stands: Vec::new(),
            history: History {
                outlines: vec![None; age as usize + 1],
                ledgers: Vec::new(),
            },
            growth: Vec::new(),
            rows: Rows::new(species, age, self.coarse.threads),
        }
    }

    /// One cycle of the structure above: its buds' and roots' light, the
    /// balance and the allotment with each root's expectation, the roots'
    /// expectations advanced, its growth and layout, the roots made
    /// handed over, and the cycle's outline folded and recorded.
    pub(super) fn above_cycle(&mut self, cycle: u32, a: &mut Above) -> Result<()> {
        let age = self.request.age;
        let o = a.history.outlines[cycle as usize - 1].clone();
        let threads = self.coarse.threads;
        read(&mut self.coarse, o.as_ref(), threads);
        for s in a.stands.iter_mut().filter(|s| !s.dead) {
            s.light = light_at(o.as_ref(), s.base);
        }
        self.above_balance(cycle, a);
        self.above_allot(cycle, a, o.as_ref());
        self.advance_stands(a, o.as_ref());
        let first_new = self.coarse.axes.len();
        let advanced: Vec<usize> = self.coarse.live.iter().map(|x| x.axis).collect();
        let live = std::mem::take(&mut self.coarse.live);
        self.coarse.grow_live(live, cycle, &mut |_| {})?;
        self.coarse.wake(cycle)?;
        if self.coarse.rules.grown > u64::from(self.request.budget) {
            return Err(Error::Budget {
                limit: self.request.budget,
            });
        }
        if cycle == age {
            return Ok(());
        }
        self.coarse.sketch(cycle, &advanced)?;
        let (mut fold, mut area) = self.fold_own(cycle, &advanced, first_new);
        let made = self.items.len();
        self.hand_over(first_new, cycle + 1);
        self.stand_in(made, a);
        for s in a.stands.iter().filter(|s| !s.dead && s.agg.leaves > 0.0) {
            area += s.scale * s.agg.leaves;
            let m = cycle.saturating_sub(s.birth).max(1);
            let reach = a.rows.lateral(s.pa, age.saturating_sub(m)).reach * s.scale;
            let b = s.base;
            let rho = b.x.hypot_fixed(b.y);
            let out = match rho > 0.0 {
                true => Vec3::new(b.x, b.y, 0.0) * ((rho + reach) / rho),
                false => Vec3::new(reach, 0.0, 0.0),
            };
            fold.add(b + Vec3::new(0.0, 0.0, reach), true);
            fold.add(b - Vec3::new(0.0, 0.0, reach), true);
            fold.add(Vec3::new(out.x, out.y, b.z), true);
        }
        let light = self.coarse.sketch.as_ref().expect("sketching").light;
        let outline = Outline::new(fold.shape(), q32(area), &light);
        a.history.outlines[cycle as usize] = Some(outline);
        Ok(())
    }

    /// The structure above's own leaves of `cycle`, folded, and their
    /// area: each on the stem where its axis carries the seed on.
    fn fold_own(&self, cycle: u32, advanced: &[usize], first_new: usize) -> (Fold, f64) {
        let c = &self.coarse;
        let pencils = &c.sketch.as_ref().expect("sketching").pencils;
        let mut fold = Fold::default();
        for i in advanced.iter().copied().chain(first_new..c.axes.len()) {
            let crown = !pencils[i].trunk;
            let grown = c.axes[i].phytomers.iter().rev();
            for p in grown.take_while(|p| p.cycle == cycle) {
                fold.add(p.tip, crown);
            }
        }
        (fold, c.leaves.iter().map(|l| l.1).sum())
    }

    /// Each item made since `made` stood in by its root's expectation.
    fn stand_in(&mut self, made: usize, a: &mut Above) {
        let pencils = &self.coarse.sketch.as_ref().expect("sketching").pencils;
        for item in &self.items[made..] {
            let root = &item.g.axes[1];
            let spent = (item.g.live.iter())
                .find(|x| x.axis == 1)
                .map_or(0.0, |x| x.spent);
            a.stands.push(Standin {
                at: item.at,
                pa: root.pa,
                birth: root.birth,
                scale: pencils[item.at].base_scale,
                base: root.base,
                agg: Aggregate::new(root.pa, spent),
                light: 1.0,
                ratio: 0.0,
                dead: false,
            });
            a.history.ledgers.push(Ledger::new(self.request.age));
        }
    }

    /// Phase (i) above the cut, each root by its expectation (S1, S2).
    fn above_balance(&mut self, cycle: u32, a: &mut Above) {
        if !self.coarse.balances() {
            return;
        }
        let species = self.coarse.species;
        let rows = self.coarse.rows().expect("laid");
        let given: Vec<_> = (a.stands.iter())
            .map(|s| {
                let living = if s.dead { 0.0 } else { s.agg.living() };
                let upkeep = species.states[s.pa].upkeep;
                let light = q32(s.scale * living * s.light);
                (
                    s.at,
                    light,
                    q32(upkeep * s.scale * s.agg.wood),
                    living > 0.0,
                )
            })
            .collect();
        let totals = self.coarse.totals(&rows, &given);
        let (shed, kept) = self.coarse.shed_by(cycle, &rows, &totals, None);
        let pencils = &mut self.coarse.sketch.as_mut().expect("sketching").pencils;
        let c = cycle as usize;
        for (s, l) in a.stands.iter_mut().zip(&mut a.history.ledgers) {
            (l.shed[c], l.kept[c], l.memory[c]) = (shed[s.at], kept[s.at], pencils[s.at].memory);
            s.dead |= shed[s.at];
            pencils[s.at].kept = kept[s.at];
        }
    }

    /// Phases (ii) and (iii) above the cut: the split with each root's
    /// expected lit presence, each root's incoming vigour recorded, and
    /// every bud sized against the outline's means.
    fn above_allot(&mut self, cycle: u32, a: &mut Above, o: Option<&Outline>) {
        if !self.coarse.allots() {
            return;
        }
        let rows = self.coarse.rows().expect("laid");
        let mut inputs = self.coarse.inputs(&rows);
        let pencils = &self.coarse.sketch.as_ref().expect("sketching").pencils;
        let presence: Vec<f64> = (a.stands.iter())
            .map(|s| match s.dead {
                true => 0.0,
                false => s.scale * s.agg.living() * pencils[s.at].kept,
            })
            .collect();
        for (s, &p) in a.stands.iter().zip(&presence) {
            (inputs.presence[s.at], inputs.lit[s.at]) = (p, p * s.light);
        }
        let (incoming, got) = split(&rows.links, &inputs.lit, &inputs.lambda, None);
        let c = cycle as usize;
        for ((s, l), &p) in a
            .stands
            .iter_mut()
            .zip(&mut a.history.ledgers)
            .zip(&presence)
        {
            l.vigour[c] = incoming[s.at];
            s.ratio = if p > 0.0 { got[s.at] / p } else { 0.0 };
            inputs.presence[s.at] = 0.0;
        }
        size(
            &mut self.coarse,
            (&got, &inputs, &rows.pa),
            o,
            (cycle, &mut a.growth),
        );
    }

    /// Every living root's expectation grows its unit of the cycle under
    /// its light, kept share and sizes (S1, S6).
    fn advance_stands(&mut self, a: &mut Above, o: Option<&Outline>) {
        let c = &self.coarse;
        let (species, laterals, allots) = (c.species, &c.rules.laterals, c.allots());
        let pencils = &c.sketch.as_ref().expect("sketching").pencils;
        let means: Vec<f64> = (species.states.iter())
            .map(|s| real(mean(o, s.shade_size)))
            .collect();
        let live: Vec<&mut Standin> = a.stands.iter_mut().filter(|s| !s.dead).collect();
        crate::cores::map(live, crate::cores::PIECE, c.threads, &|_, s| {
            let size: Vec<f64> = (species.states.iter().zip(&means))
                .map(|(st, &m)| match (allots, st.shade_size, m > 0.0) {
                    (true, psi, true) if psi != 0.0 => s.ratio.powf_fixed(psi) / m,
                    _ => 1.0,
                })
                .collect();
            let at = Reads {
                light: s.light,
                kept: pencils[s.at].kept,
                size: &size,
            };
            s.agg.advance(species, laterals, at);
        });
    }
}

#[cfg(test)]
mod tests;
