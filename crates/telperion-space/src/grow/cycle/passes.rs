//! The one cycle's balance and allotment over its working set
//! (`working.rs`): the same arithmetic as `balance.rs` and `sketch.rs`'s
//! allotment, each in the whole tree's order, visiting only the open axes.
use super::working::sizes;
use super::Grower;
use crate::allocation::TIP;
use crate::contract::shed::{walk, Bud, Lateral};
use crate::lineage::{Key, SHED};

impl Grower<'_> {
    /// The cycle's working set: the axes laid so far taken in, and those
    /// open on the way to a living apex or a sleeping bud.
    pub(in crate::grow) fn open(&mut self, cycle: u32) {
        let n = self.sketch.as_ref().expect("sketching").pencils.len();
        let mut w = self.working.take().unwrap_or_default();
        let laid = w.links.len();
        w.extend(&self.axes, n);
        let sleeping = (self.asleep.iter().skip(cycle as usize)).flat_map(|a| a.iter().map(|s| s.parent));
        w.open_from(self.live.iter().map(|a| a.axis).chain(sleeping), laid);
        self.working = Some(w);
    }

    /// `balance` over the working set: each open subtree's totals, D16's
    /// walk over the open axes, each memory kept, every apex shed leaving
    /// the living and every other given its kept share.
    pub(in crate::grow) fn balance_one(&mut self, cycle: u32) {
        let mut w = self.working.take().expect("a working set");
        let (species, sketch) = (self.species, self.sketch.as_mut().expect("sketching"));
        let n = sketch.pencils.len();
        let mut lit = vec![false; n];
        for a in self.live.iter().filter(|a| a.axis < n) {
            lit[a.axis] = true;
        }
        let pencils = &sketch.pencils;
        let upkeep = |i: usize| species.states[self.axes[i].pa].upkeep as f32 * pencils[i].wood as f32;
        let own = |i: usize| {
            let p = &pencils[i];
            let light = match lit[i] {
                true => p.base_scale as f32 * p.running as f32 * p.light as f32,
                false => 0.0,
            };
            (light, upkeep(i), lit[i])
        };
        w.totals(&own, &upkeep);
        let mut laterals = vec![Lateral::default(); w.list.len()];
        let (ws, axes, rules) = (&w, &self.axes, &self.rules);
        crate::cores::fill(&mut laterals, 1 << 12, self.threads, &|k| {
            let i = ws.list[k];
            let link = ws.links[i];
            let parent = link.parent.map(|p| ws.at[p]);
            if !(parent.is_some() && link.node != TIP && ws.living[i]) {
                return Lateral { parent, ..Lateral::default() };
            }
            let state = &species.states[ws.pa[i]];
            let d = Key(axes[i].lineage).child(SHED).child(u64::from(cycle)).draw();
            let window = rules.windows.window((ws.pa[i], cycle, None), 1.0);
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
            Lateral { parent, decides: true, bud }
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
        let (species, sketch) = (self.species, self.sketch.as_mut().expect("sketching"));
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
