//! The one cycle's balance and allotment over its working set
//! (`working.rs`): the balance, and the same arithmetic as `sketch.rs`'s
//! allotment, each in the whole tree's order, visiting only the open axes.
use super::working::sizes;
use super::Grower;
use crate::contract::shed::{walk, Bud, Lateral};
use crate::lineage::{Key, SHED};

impl Grower {
    /// The cycle's working set: the axes laid so far taken in, and those
    /// open on the way to a living apex or a sleeping bud.
    pub(in crate::grow) fn open(&mut self, cycle: u32) {
        let n = self.sketch.as_ref().expect("sketching").pencils.len();
        let mut w = self.working.take().unwrap_or_default();
        let laid = w.links.len();
        w.extend(&self.axes, n);
        let sleeping =
            (self.asleep.iter().skip(cycle as usize)).flat_map(|a| a.iter().map(|s| s.parent));
        w.open_from(self.live.iter().map(|a| a.axis).chain(sleeping), laid);
        self.working = Some(w);
    }

    /// Whether the balance runs this cycle: where a rough layout is grown
    /// and any PA's hazard is set, or always in the one cycle.
    pub(in crate::grow) fn balances(&self) -> bool {
        self.sketch.is_some()
            && (self.one || self.species.states.iter().any(|s| s.balance_hazard != 0.0))
    }

    /// The carbon balance (fn-197 step 4, fn-349) over the working set.
    /// Each living tip collects its size times the light at its tip and is
    /// charged its size times the square of its path from the root collar
    /// over its species' potential height (`charge`); wood that has died
    /// is charged nothing. Each open part earns its own tips' light and
    /// what its paying child laterals have left over, and costs its own
    /// tips' charge (`Working::totals`), and every
    /// living subtree but the seed's own axis is tested on its balance,
    /// (Q - C) / (Q + C), remembered as a running average: D16's walk,
    /// each memory kept, every apex shed leaving the living and every other
    /// given its kept share.
    pub(in crate::grow) fn balance_one(&mut self, cycle: u32) {
        let mut w = self.working.take().expect("a working set");
        let (species, sketch) = (&*self.species, self.sketch.as_mut().expect("sketching"));
        let n = sketch.pencils.len();
        let mut lit = vec![false; n];
        // Each axis's time spent in its PA: its living apex's, or where it
        // stopped, which its draw's window looks on from (fn-355).
        let mut spent = self.units.clone();
        for a in self.live.iter().filter(|a| a.axis < n) {
            (lit[a.axis], spent[a.axis]) = (true, a.spent);
        }
        let pencils = &sketch.pencils;
        let own = |i: usize| {
            if !lit[i] {
                return (0.0, 0.0, false);
            }
            let p = &pencils[i];
            let size = p.base_scale as f32 * p.running as f32;
            let inverse = species.states[self.axes[i].pa].inverse_height as f32;
            (
                size * p.light as f32,
                charge(size, p.path_end(), inverse),
                true,
            )
        };
        w.totals(&own);
        let mut laterals = vec![Lateral::default(); w.list.len()];
        let (ws, axes, rules) = (&w, &self.axes, &self.rules);
        crate::cores::fill(&mut laterals, 1 << 12, self.threads, &|k| {
            let i = ws.list[k];
            let link = ws.links[i];
            let parent = link.parent.map(|p| ws.at[p]);
            if !(parent.is_some() && ws.living[i]) {
                return Lateral {
                    parent,
                    ..Lateral::default()
                };
            }
            let state = &species.states[ws.pa[i]];
            let d = Key(axes[i].lineage)
                .child(SHED)
                .child(u64::from(cycle))
                .draw();
            let window = rules.windows.window((ws.pa[i], spent[i], cycle, None), 1.0);
            let bud = Bud {
                q: ws.light[i],
                c: ws.cost[i],
                memory: pencils[i].memory as f32,
                hazard: state.balance_hazard as f32,
                tolerance: state.tolerance as f32,
                draw: d.0,
                window,
                keep: 0.0,
            };
            Lateral {
                parent,
                decides: true,
                bud,
            }
        });
        let walked = walk(&laterals, None);
        for (&i, wk) in w.list.iter().zip(&walked) {
            if let Some(memory) = wk.memory {
                sketch.pencils[i].memory = f64::from(memory);
            }
        }
        let live = std::mem::take(&mut self.live);
        for apex in live {
            if apex.axis < n {
                let wk = &walked[w.at[apex.axis] as usize];
                if wk.shed {
                    self.axes[apex.axis].apex_end = Some(cycle - 1);
                    continue;
                }
                sketch.pencils[apex.axis].kept = f64::from(wk.kept);
            }
            self.live.push(apex);
        }
        self.working = Some(w);
    }

    /// The allotment over the working set: each living apex's unit's size
    /// this cycle (`Grower::allot`).
    pub(in crate::grow) fn allot_one(&mut self) {
        let mut w = self.working.take().expect("a working set");
        let (species, sketch) = (&*self.species, self.sketch.as_mut().expect("sketching"));
        let n = sketch.pencils.len();
        let m = w.list.len();
        let (mut presence, mut psi, mut lit) = (vec![0.0; m], vec![0.0; m], vec![0.0f32; m]);
        for apex in self.live.iter().filter(|a| a.axis < n) {
            let (k, p) = (w.at[apex.axis] as usize, &sketch.pencils[apex.axis]);
            presence[k] = p.base_scale * p.running * p.kept;
            lit[k] = (presence[k] * p.light) as f32;
            psi[k] = species.states[w.pa[apex.axis]].shade_size;
        }
        let lambda: Vec<f32> = (w.list.iter())
            .map(|&i| species.states[w.pa[i]].apical_control as f32)
            .collect();
        w.split(&lit, &lambda);
        let got: Vec<f64> = w.list.iter().map(|&i| f64::from(w.bud[i])).collect();
        let groups: Vec<usize> = w.list.iter().map(|&i| w.pa[i]).collect();
        let sized = sizes(&got, &presence, &psi, &groups, &w.order());
        for apex in self.live.iter().filter(|a| a.axis < n) {
            sketch.pencils[apex.axis].size = sized[w.at[apex.axis] as usize];
        }
        self.working = Some(w);
    }
}

/// A living tip's charge: its size times the square of its path from the
/// root collar times its species' inverse height; none at an inverse
/// height of 0.
pub(super) fn charge(size: f32, path: f32, inverse_height: f32) -> f32 {
    let reach = path * inverse_height;
    size * reach * reach
}
