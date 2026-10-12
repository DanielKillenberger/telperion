//! The build's cycle recorded once (`plan.rs`): the seed, then a cycle's
//! phases in fn-255's order, each sized on the device. The host replays
//! them every cycle, choosing only by what it knows: the cycle, whether
//! any axis is laid yet (from the second cycle), the hierarchy's lists'
//! parity, and whether the cycle is the tree's age (no layout or light).
use super::build::{Build, Kit, Tables};
use super::census as c;
use super::device::SEED;
use super::glue;
use super::gate::Limits;
use super::plan::{Builder, Gated, PassId, Plan, Size, Step, MAXX};
use super::plan_hier::HierBounds;
use super::plan_lay::LayBounds;
use super::vm::E;
use crate::{Gpu, Result};

/// How far each loop with a count only the device knows is recorded,
/// and the ranges a list's dispatches are cut into.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Bounds {
    pub hier: HierBounds,
    pub lay: LayBounds,
    /// The units' dispatches a list takes at most, and the rounds a
    /// range's jobs run (DESIGN.md section 10).
    pub units: u32,
    pub rounds: u32,
    pub ranges: Ranges,
}

/// The most items a dispatch over a list takes (bounded work per
/// dispatch, DESIGN.md section 5): living apexes a units dispatch, grown
/// axes a draw, leaves a deposit. Counts, never timings; multiples of
/// the workgroup. Busiest cycle, Telperion's: about 30,000 apexes, 90,000
/// axes and 700,000 leaves a millisecond; the oak's lattice about 120,000
/// nodes combined and 750,000 directions' nodes pathed a millisecond.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Ranges {
    pub units: u32,
    pub draw: u32,
    pub deposit: u32,
    /// Lattice nodes a densities or combine dispatch takes, and
    /// directions' columns or nodes a weights or paths dispatch takes.
    pub nodes: u32,
    pub paths: u32,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            hier: HierBounds { gens: 4, passes: 4, levels: 8 },
            lay: LayBounds { generations: 4, sweeps: 2, deposits: 1, draws: 1, lights: 1 },
            units: 1,
            // Recorded up to eight, each cycle only as far as the rounds
            // the last used and a slack (`Cond::UnitRound`).
            rounds: 8,
            ranges: Ranges { units: super::device::RANGE, draw: 65536, deposit: 262_144, nodes: 65536, paths: 524_288 },
        }
    }
}

/// A cycle's recorded phases: the seed; the start; the hierarchy's step
/// and sums by the parity of its lists, and retirement's inside sweep
/// over them, replayed apart (`Build::inside`); the units; the layout and
/// light.
pub(super) struct Phases {
    pub seed: Vec<Gated>,
    pub start: Vec<Gated>,
    pub hier: [Vec<Gated>; 2],
    pub inside: [Vec<Gated>; 2],
    pub units: Vec<Gated>,
    pub sketch: Vec<Gated>,
    _held: Vec<crate::buffer::Owned>,
}

impl Build {
    /// A glue dispatch recorded: its static words from `w`, the device's
    /// from `dynamic` (word 0 its invocations).
    pub(super) fn glue_step(&self, b: &mut Builder, kit: &Kit, (entry, aux, aux2): (usize, &wgpu::Buffer, &wgpu::Buffer), w: glue::Words, dynamic: &[(usize, E)]) {
        let s = self.state.caps;
        let words = [
            w.n, MAXX * 64, w.cycle, w.laid, s.axes, s.units, s.phyts, 0, w.live_stride, s.next_at(), s.next, w.first, w.leaves,
            u32::from(w.from_in), w.live, 0,
        ];
        let slot = b.slot(&words);
        let mut n = E::C(w.n);
        for (k, e) in dynamic {
            if *k == 0 {
                n = e.clone();
            }
            b.set(slot, *k, e.clone());
        }
        let size = match dynamic.is_empty() {
            true => Size::Direct(w.n.div_ceil(64).max(1)),
            false => b.groups(n.div(E::C(64))),
        };
        let st = &self.state;
        let bind = b.bind(&kit.glue.pass, slot, &[&st.axes, &st.pencils, &st.units, &st.laid, &st.lists, &self.sums.live, aux, aux2]);
        b.dispatch((PassId::Glue, entry), bind, size);
    }

    /// The seed's axis and apex (`Grower::seeded`).
    fn plan_seed(&self, b: &mut Builder, kit: &Kit, t: &Tables) {
        let mut words = self.units_static(t, (0, 1, 0));
        words[0] = 1;
        let slot = b.slot(&words);
        let s = &self.state;
        let rest = [&*t.table, &t.words, &self.inputs, &s.axes, &s.units, &s.grown, &s.lists, &self.counts];
        let bind = b.bind(&kit.units.cycle, slot, &rest);
        b.dispatch((PassId::Units, SEED), bind, Size::Direct(1));
        let w = glue::Words { n: 1, live_stride: self.sums.live_stride, ..glue::Words::default() };
        self.glue_step(b, kit, (glue::BORN, &self.spare, &self.flags), w, &[]);
        b.tally(&s.lists, 4 * 20, c::Flag::Collapsed.word(), 4);
    }

    /// The cycle's start: its counts kept, the living taken in.
    fn plan_start(&self, b: &mut Builder, kit: &Kit) {
        b.point();
        let snap = [
            (c::AXES0, c::AXES),
            (c::UNITS0, c::UNITS),
            (c::PHYTS0, c::PHYTS),
            (c::SLEEPERS0, c::SLEEPERS),
            (c::WOKEN0, c::WOKEN),
            (c::LIVE0, c::NEXT),
            (c::LIVE, c::NEXT),
            (c::LAID0, c::LAID),
        ];
        for (to, from) in snap {
            b.count(to, E::At(from));
        }
        // Words only a recorded round or pass writes, none until it does.
        for w in (0..c::ROUNDS).map(|r| c::UNPLACED + r).chain((0..c::ROUNDS * c::PASSES).map(|k| c::PASS_LEFT + k)) {
            b.count(w, E::C(0));
        }
        b.point();
        let w = glue::Words { live_stride: self.sums.live_stride, ..glue::Words::default() };
        self.glue_step(b, kit, (glue::RELIVE, &self.spare, &self.flags), w, &[(0, E::At(c::LIVE0))]);
    }

    /// The whole cycle recorded under `bounds`.
    pub(super) fn plan(&self, gpu: &Gpu, kit: &Kit, t: &Tables, bounds: Bounds) -> Result<(Plan, Phases)> {
        let mut b = Builder::new(gpu)?;
        self.plan_seed(&mut b, kit, t);
        let seed = b.take();
        self.plan_start(&mut b, kit);
        let start = b.take();
        let (mut hier, mut inside): ([Vec<Gated>; 2], [Vec<Gated>; 2]) = Default::default();
        b.phase = "balance and allotment";
        for (parity, (phase, marks)) in hier.iter_mut().zip(&mut inside).enumerate() {
            self.plan_step(&mut b, kit, bounds.hier, parity);
            b.phase = "retirement";
            let sweep = self.plan_retire(&mut b, kit, t, bounds.hier, 1 - parity);
            b.phase = "balance and allotment";
            self.plan_sums(&mut b, kit, t, bounds.hier, 1 - parity);
            *phase = b.take();
            b.phase = "retirement";
            sweep.plan(&mut b);
            *marks = b.take();
            b.phase = "balance and allotment";
        }
        b.phase = "units and wake";
        let mut held = self.plan_units(&mut b, kit, t, (bounds.units, bounds.ranges.units, bounds.rounds))?;
        let units = b.take();
        b.phase = "layout";
        held.extend(self.plan_layout(&mut b, kit, (bounds.lay, bounds.ranges.draw))?);
        b.phase = "light";
        self.plan_light(&mut b, kit, (bounds.lay, bounds.ranges));
        let sketch = b.take();
        let plan = b.finish(&kit.vm)?;
        Ok((plan, Phases { seed, start, hier, inside, units, sketch, _held: held }))
    }
}

impl Phases {
    /// Cycle `cycle` of a tree of `age`, as recorded.
    pub(super) fn cycle(&self, cycle: u32, age: u32) -> Vec<&[Gated]> {
        let mut out: Vec<&[Gated]> = vec![&self.start];
        if cycle > 1 {
            out.push(&self.hier[(cycle as usize) % 2]);
        }
        out.push(&self.units);
        if cycle < age {
            out.push(&self.sketch);
        }
        out
    }
}

/// The workgroups `size` takes this cycle, from those `given` each
/// bounded size.
fn groups(size: Size, given: &[u32]) -> u32 {
    match size {
        Size::Direct(n) => n,
        Size::Bounded(k) => given[k as usize],
    }
}

/// `steps` the cycle's `limits` allow recorded into `e`, each bounded
/// dispatch with the workgroups `given` it (none skipped), each
/// dispatch's phase onto `phases`. Unprofiled, consecutive dispatches
/// share a compute pass.
pub(super) fn record(kit: &Kit, (plan, limits, given): (&Plan, &Limits, &[u32]), e: &mut wgpu::CommandEncoder, steps: &[Gated], phases: &mut Vec<&'static str>) {
    let live = |s: &Step| !matches!(s, Step::Dispatch { size, .. } | Step::Zeroed { size, .. } if groups(*size, given) == 0);
    let allowed: Vec<&Step> = steps.iter().filter(|g| g.gate.iter().all(|c| limits.allows(c))).map(|g| &g.step).filter(|s| live(s)).collect();
    #[cfg(test)]
    if super::super::profile::on() {
        return stamped(kit, (plan, given), e, &allowed, phases);
    }
    let mut k = 0;
    while k < allowed.len() {
        let run = allowed[k..].iter().take_while(|s| matches!(s, Step::Dispatch { .. })).count();
        if run == 0 {
            stamped(kit, (plan, given), e, &allowed[k..k + 1], phases);
            k += 1;
            continue;
        }
        let mut pass = e.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("cycle"), timestamp_writes: None });
        for step in &allowed[k..k + run] {
            if let Step::Dispatch { pass: id, entry, bind, size, phase } = step {
                kit.pass(*id).encode(&mut pass, (bind, *entry), groups(*size, given));
                phases.push(phase);
            }
        }
        drop(pass);
        k += run;
    }
}

/// `steps` recorded each in its own stamped pass.
fn stamped(kit: &Kit, (plan, given): (&Plan, &[u32]), e: &mut wgpu::CommandEncoder, steps: &[&Step], phases: &mut Vec<&'static str>) {
    for step in steps {
        match step {
            Step::Dispatch { pass, entry, bind, size, phase } => {
                kit.pass(*pass).run(e, bind, *entry, groups(*size, given));
                phases.push(phase);
            }
            Step::Copy { from, at, to, to_at, bytes } => e.copy_buffer_to_buffer(from, *at, to, *to_at, *bytes),
            Step::Clear { buffer, at, bytes } => e.clear_buffer(buffer, *at, *bytes),
            Step::Zeroed { buffer, at, size, most } => e.clear_buffer(buffer, *at, Some((4 * u64::from(groups(*size, given))).min(*most))),
            Step::Point(k) => {
                if plan.run_point(&kit.vm, e, *k) {
                    phases.push("sizing");
                }
            }
        }
    }
}
