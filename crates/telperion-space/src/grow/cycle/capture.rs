//! fn-256's spike (COUNCIL.md early test 3): one busy cycle of the one
//! cycle captured, every phase's inputs and the CPU's outputs, for a
//! replay on the device. The light at the last cycle that reads it (the
//! tree's age less one); the hierarchy, normalisation and fate at the
//! tree's age, the busiest. An experiment entry point, wired into nothing.
use super::super::{start, Grower, Options, Request};
use crate::allocation::TIP;
use crate::contract::{decide, ticks};
use crate::error::{refuse, Result};
use crate::light::Field;
use crate::lineage::{Key, ENDED, FAILED, VIABILITY};
use crate::schedule::Carried;
use crate::species::Species;
use crate::structure::Origin;

/// The captured arrays, by name, each a kind and its little-endian bytes.
pub struct Capture {
    pub arrays: Vec<(String, u8, Vec<u8>)>,
}

macro_rules! put {
    ($c:expr, $name:expr, $kind:expr, $v:expr) => {
        $c.arrays.push((
            $name.to_string(),
            $kind,
            $v.iter().flat_map(|x| x.to_le_bytes()).collect(),
        ))
    };
}

impl Capture {
    fn u32s(&mut self, name: &str, v: &[u32]) {
        put!(self, name, 1, v);
    }
    fn u64s(&mut self, name: &str, v: &[u64]) {
        put!(self, name, 2, v);
    }
    fn f32s(&mut self, name: &str, v: &[f32]) {
        put!(self, name, 3, v);
    }
    fn i32s(&mut self, name: &str, v: &[i32]) {
        put!(self, name, 4, v);
    }

    /// The capture as one file's bytes: per array its name's length and
    /// name, its kind, its byte count and its bytes.
    pub fn bytes(&self) -> Vec<u8> {
        let mut out = b"FN256CAP".to_vec();
        for (name, kind, data) in &self.arrays {
            out.extend((name.len() as u32).to_le_bytes());
            out.extend(name.as_bytes());
            out.push(*kind);
            out.extend((data.len() as u64).to_le_bytes());
            out.extend(data);
        }
        out
    }
}

/// Grows `species` through the one cycle as `grow_cycle` does and
/// captures its busy cycle, the tree's age; refused below age 2.
pub fn grow_capture(species: &Species, request: Request) -> Result<Capture> {
    if request.age < 2 {
        return refuse("age", "a capture reads the light of the cycle before");
    }
    let (mut g, _) = start(species, request, true, Options::default())?;
    (g.one, g.rules.one) = (true, true);
    g.working = Some(Box::default());
    g.sketch
        .get_or_insert_with(|| super::super::sketch::Sketch::new(request.light));
    g.rules.lasting = vec![None; species.states.len()].into();
    let mut c = Capture { arrays: Vec::new() };
    let mut advanced = Vec::new();
    let last = request.age;
    // Each cycle's axes held and living apexes at its start: what the
    // whole build's device time scales with.
    let (mut axes, mut live) = (Vec::new(), Vec::new());
    for cycle in 1..last - 1 {
        axes.push(g.axes.len() as u32);
        live.push(g.live.len() as u32);
        g.year(cycle, true, &mut advanced, &mut |_| {})?;
    }
    axes.push(g.axes.len() as u32);
    live.push(g.live.len() as u32);
    // The cycle before: its growth and layout, then its light captured.
    advanced.clear();
    advanced.extend(g.live.iter().map(|a| a.axis));
    g.step(last - 1, &mut |_| {})?;
    g.sketch(last - 1, &advanced)?;
    g.capture_light(&mut c)?;
    g.shade(last - 1, &mut |_| {})?;
    axes.push(g.axes.len() as u32);
    live.push(g.live.len() as u32);
    c.u32s("per_cycle_axes", &axes);
    c.u32s("per_cycle_live", &live);
    g.capture_cycle(last, &mut c)?;
    Ok(c)
}

impl Grower<'_> {
    fn capture_light(&self, c: &mut Capture) -> Result<()> {
        let light = self.sketch.as_ref().expect("sketching").light;
        let tips: Vec<_> = self.live.iter().map(|a| self.tip(a.axis)).collect();
        let (field, quanta) = Field::quantised_with(&light, &self.leaves, &tips, self.threads)?;
        let (grid, density, lit) = field.parts();
        let f = |x: f64| x as f32;
        let leaves: Vec<f32> = (self.leaves.iter())
            .flat_map(|(p, a)| [f(p.x), f(p.y), f(p.z), f(*a)])
            .collect();
        let at: Vec<f32> = tips.iter().flat_map(|p| [f(p.x), f(p.y), f(p.z)]).collect();
        let reads: Vec<f32> = tips.iter().map(|&t| field.at(t) as f32).collect();
        let dirs: Vec<f32> = (light.directions().into_iter())
            .filter(|d| d.1 > 0.0)
            .flat_map(|(t, s)| [t.x / t.z, t.y / t.z, crate::light::CELL / t.z, s].map(f))
            .collect();
        c.f32s("leaves", &leaves);
        c.f32s("tips", &at);
        c.i32s("grid_low", &grid.low);
        c.u32s("grid_size", &grid.size);
        c.u32s("quanta", &quanta);
        c.f32s("density", density);
        c.f32s("lattice", lit);
        c.f32s("reads", &reads);
        c.f32s("directions", &dirs);
        c.f32s("extinction", &[light.extinction as f32]);
        Ok(())
    }

    /// The tree's age through fn-255's passes: the working set opened and
    /// retired, its balance (totals, shedding, memory, kept shares), its
    /// allotment (split and sizes), then each surviving apex's survival
    /// and the axes it bears, as `step` takes them.
    fn capture_cycle(&mut self, cycle: u32, c: &mut Capture) -> Result<()> {
        assert!(self.fine.is_none() && self.couple.is_none() && self.oracle.is_none());
        self.open(cycle);
        self.retire(cycle);
        let w = self.working.as_ref().expect("a working set");
        let n = w.links.len();
        let none = u32::MAX;
        let parent: Vec<u32> = w.links.iter().map(|l| l.parent.map_or(none, |p| p as u32)).collect();
        let node: Vec<u32> = (w.links.iter())
            .map(|l| if l.node == TIP { none } else { l.node as u32 })
            .collect();
        c.u32s("parent", &parent);
        c.u32s("node", &node);
        c.u32s("pa", &w.pa.iter().map(|&x| x as u32).collect::<Vec<_>>());
        let list: Vec<u32> = w.list.iter().map(|&i| i as u32).collect();
        c.u32s("open", &list);
        let order = w.order();
        c.u32s("open_order", &order);
        let sketch = self.sketch.as_ref().expect("sketching");
        let p = |g: &dyn Fn(&super::super::sketch::Pencil) -> f64| -> Vec<f32> {
            sketch.pencils[..n].iter().map(|x| g(x) as f32).collect()
        };
        c.f32s("base_scale", &p(&|x| x.base_scale));
        c.f32s("running", &p(&|x| x.running));
        c.f32s("light", &p(&|x| x.light));
        c.f32s("memory", &p(&|x| x.memory));
        c.f32s("wood", &p(&|x| x.wood));
        let lineage: Vec<u64> = self.axes[..n].iter().map(|a| a.lineage).collect();
        c.u64s("lineage", &lineage);
        let states = &self.species.states;
        let st = |g: &dyn Fn(&crate::species::PaState) -> f64| -> Vec<f32> {
            states.iter().map(|s| g(s) as f32).collect()
        };
        c.f32s("pa_upkeep", &st(&|s| s.upkeep));
        c.f32s("pa_hazard", &st(&|s| s.balance_hazard));
        c.f32s("pa_tolerance", &st(&|s| s.tolerance));
        c.f32s("pa_lambda", &st(&|s| s.apical_control));
        c.f32s("pa_psi", &st(&|s| s.shade_size));
        c.f32s("pa_viability", &st(&|s| s.viability));
        c.f32s("pa_shade_hazard", &st(&|s| s.shade_hazard));
        let windows: Vec<f32> = (0..states.len())
            .map(|pa| self.rules.windows.window((pa, cycle, None), 1.0))
            .collect();
        c.f32s("pa_window", &windows);
        c.u32s("cycle", &[cycle]);
        let live: Vec<u32> = (self.live.iter()).filter(|a| a.axis < n).map(|a| a.axis as u32).collect();
        c.u32s("live", &live);
        // The balance over the working set: totals, the walk, memory and
        // each surviving apex's kept share.
        self.balance_one(cycle);
        let w = self.working.as_ref().expect("a working set");
        let at = |v: &[f32]| -> Vec<f32> { w.list.iter().map(|&i| v[i]).collect() };
        c.f32s("cpu_total_light", &at(&w.light));
        c.f32s("cpu_total_cost", &at(&w.cost));
        let living: Vec<u32> = w.list.iter().map(|&i| w.living[i] as u32).collect();
        c.u32s("cpu_living", &living);
        let sketch = self.sketch.as_ref().expect("sketching");
        let survived: std::collections::HashSet<usize> = self.live.iter().map(|a| a.axis).collect();
        let shed: Vec<u32> = live.iter().map(|&i| !survived.contains(&(i as usize)) as u32).collect();
        c.u32s("cpu_shed", &shed);
        let kept: Vec<f32> = live.iter().map(|&i| sketch.pencils[i as usize].kept as f32).collect();
        c.f32s("cpu_kept", &kept);
        let memory: Vec<f32> = w.list.iter().map(|&i| sketch.pencils[i].memory as f32).collect();
        c.f32s("cpu_memory", &memory);
        // The allotment: the split over the open axes, then the sizes.
        self.allot_one();
        let w = self.working.as_ref().expect("a working set");
        let at = |v: &[f32]| -> Vec<f32> { w.list.iter().map(|&i| v[i]).collect() };
        c.f32s("cpu_sub", &at(&w.sub));
        c.f32s("cpu_incoming", &at(&w.incoming));
        c.f32s("cpu_bud", &at(&w.bud));
        let sketch = self.sketch.as_ref().expect("sketching");
        let sizes: Vec<f32> = (self.live.iter())
            .filter(|a| a.axis < n)
            .map(|a| sketch.pencils[a.axis].size as f32)
            .collect();
        c.f32s("cpu_sizes", &sizes);
        self.capture_fate(cycle, c)
    }

    /// Each surviving apex's survival decision as `advance` takes it, then
    /// the axes each bears this cycle, in the live order.
    fn capture_fate(&mut self, cycle: u32, c: &mut Capture) -> Result<()> {
        let (mut axis, mut key, mut share, mut light) = (vec![], vec![], vec![], vec![]);
        let (mut threshold, mut draw, mut pas) = (vec![], vec![], vec![]);
        for apex in &self.live {
            let i = apex.axis;
            let state = &self.species.states[self.axes[i].pa];
            let d = &self.draws[i];
            let unit = Key(self.axes[i].lineage).child(u64::from(cycle));
            let first = d.units.is_empty();
            let unit = if first && d.rest {
                unit.child(FAILED)
            } else {
                unit
            };
            let decide_key = match d.ended_relays {
                0 => unit,
                k => unit.child(ENDED).child(u64::from(k)),
            };
            let carried = Carried {
                spent: apex.spent,
                aged: 0.0,
                gone: if first { d.sleep } else { 0.0 },
                relay: first && d.ended,
                rest: first && d.rest,
            };
            let s = carried.unit(state.life(), 1).share;
            let below = if first { d.below } else { 0 };
            let l = self
                .sketch
                .as_ref()
                .and_then(|k| k.pencils.get(i))
                .map_or(1.0, |p| p.light);
            // The table's f32, shaded as the device shades it (`advance`).
            let viability = decide::shaded(state.viability as f32, state.shade_hazard as f32, l as f32);
            axis.push(i as u32);
            pas.push(self.axes[i].pa as u32);
            key.push(decide_key.0);
            share.push(if s >= 1.0 {
                u64::MAX
            } else {
                (ticks(s) << 12) + below
            });
            light.push(l as f32);
            threshold.push(decide::survival32(viability, s, below));
            draw.push(decide_key.child(VIABILITY).draw().0);
        }
        c.u32s("fate_axis", &axis);
        c.u32s("fate_pa", &pas);
        c.u64s("fate_key", &key);
        c.u64s("fate_share", &share);
        c.f32s("fate_light", &light);
        c.u64s("cpu_fate_threshold", &threshold);
        c.u64s("cpu_fate_draw", &draw);
        // The births: every axis grown in this cycle's units, by the apex
        // that bore it.
        let before = self.axes.len();
        let live = self.live.clone();
        let index: std::collections::HashMap<usize, usize> =
            live.iter().enumerate().map(|(k, a)| (a.axis, k)).collect();
        self.grow_live(live.clone(), cycle, &mut |_| {})?;
        let (mut births, mut first) = (vec![0u32; live.len()], vec![u32::MAX; live.len()]);
        // An axis born of an axis born this cycle (a continuation carried
        // on within it) is its first bearer's.
        let mut owner = Vec::with_capacity(self.axes.len() - before);
        for (j, a) in self.axes[before..].iter().enumerate() {
            let bearer = match a.origin {
                Origin::Lateral { parent, .. }
                | Origin::Continuation { parent }
                | Origin::Relay { parent, .. } => parent,
                Origin::Seed => unreachable!("one seed"),
            };
            let k = if bearer < before {
                index[&bearer]
            } else {
                owner[bearer - before]
            };
            owner.push(k);
            births[k] += 1;
            first[k] = first[k].min((before + j) as u32);
        }
        c.u32s("cpu_births", &births);
        c.u32s("cpu_first_birth", &first);
        c.u32s("axes_before", &[before as u32]);
        Ok(())
    }
}
