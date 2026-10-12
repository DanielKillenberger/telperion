//! fn-256 task 2: the whole grower on the device from the seed, test-only
//! (DESIGN.md section 2). The state (`state.rs`) is seeded as
//! `Grower::seeded` seeds it, then each cycle runs fn-255's phases over it
//! in order: the balance and the allotment over the hierarchy, the
//! units, the wake, and, before the tree's age, the layout and the light,
//! which writes each living apex's light into its pencil. Every dispatch
//! is sized on the device (`build_plan.rs`); the host reads the counts a
//! cycle late, only to stop a build whose count passed a capacity or a
//! bound. Retirement is not run: it leaves the tree as it was (DESIGN.md
//! section 6).
use super::build_light::{Light, Lit};
use super::build_plan::{record, Phases};
use super::gate::{Limits, REC};
use super::given::{given, Pending, Span};
use super::census as c;
use super::columns::{packed, Cap};
use super::hier_build::Hierarchy;
use super::hier_device::Sums;
use super::hier_scan::zeroed;
use super::lay_device::Layout;
use super::plan::{PassId, Plan};
use super::state::{Caps, Held, State};
use super::{device, glue, hier_build, lay_device, light_device, vm};
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_space::{Capture, ChainTables};

/// Every kernel the build runs.
pub(super) struct Kit {
    pub units: device::Kernels,
    pub layout: lay_device::Kernels,
    pub hier: hier_build::Kernels,
    pub light: light_device::Kernels,
    pub glue: glue::Kernels,
    pub vm: vm::Kernels,
    pub retire: super::retire_device::Kernels,
}

impl Kit {
    /// Every module made side by side (`io::pair`).
    pub(super) fn new(gpu: &Gpu) -> Self {
        use super::super::io::pair;
        let ((units, layout), ((hier, light), (glue, (vm, retire)))) = pair(
            gpu,
            || pair(gpu, || device::Kernels::new(gpu), || lay_device::Kernels::new(gpu)),
            || {
                pair(
                    gpu,
                    || pair(gpu, || hier_build::Kernels::new(gpu), || light_device::Kernels::new(gpu)),
                    || pair(gpu, || glue::Kernels::new(gpu), || pair(gpu, || vm::Kernels::new(gpu), || super::retire_device::Kernels::new(gpu))),
                )
            },
        );
        Self { units, layout, hier, light, glue, vm, retire }
    }

    pub(super) fn pass(&self, id: PassId) -> &super::super::io::Pass {
        match id {
            PassId::Units => &self.units.cycle,
            PassId::Layout => &self.layout.layout,
            PassId::HierBuild => &self.hier.build,
            PassId::HierSums => &self.hier.sums,
            PassId::HierScan => &self.hier.scan,
            PassId::Light => &self.light.pass,
            PassId::Glue => &self.glue.pass,
            PassId::Retire => &self.retire.pass,
        }
    }
}

/// The species' tables and the request, as `grow_capture_tables` takes
/// them: the units' tables and offsets, the layout's form, each PA's
/// balance constants and its whole stake's window per cycle.
pub(super) struct Tables {
    pub age: u32,
    pub budget: u32,
    pub seed: u64,
    pub lights: u32,
    pub rows: u32,
    pub whorls: u32,
    pub pas: u32,
    /// The zones a unit has at most (`zones.wgsl`).
    pub zones: u32,
    /// Each PA's settle and retirement constants (`settle_pa`).
    pub settle_pa: Vec<u32>,
    /// Whether retirement logs its verdicts: a test's, never a build's.
    pub log: bool,
    pub table: Owned,
    pub words: Owned,
    pub form: Vec<f32>,
    pa: Vec<f32>,
    windows: Vec<f32>,
}

impl Tables {
    pub(super) fn new(gpu: &Gpu, c: &Capture, chain: &ChainTables) -> Result<Self> {
        let cap = Cap(c);
        let [_, age, budget] = cap.u32s("cycle")[..] else {
            panic!("cycle, age, budget")
        };
        let (table, words, rows, whorls) = packed(&cap, chain, age);
        let zones = (0..table[0] as usize / 2).map(|p| table[2 * p + 1]).max().unwrap_or(0);
        let mut form = cap.f32s("lay_globals");
        form.extend(cap.f32s("lay_form"));
        let pa = cap.f32s("hier_pa");
        let pas = (pa.len() / 5) as u32;
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let upload = |label, bytes: &[u8]| super::super::io::buffer(gpu, label, bytes.len() as u64, usage, bytes);
        Ok(Self {
            age,
            budget,
            seed: cap.u64s("seed")[0],
            lights: cap.u32s("lights")[0],
            rows,
            whorls,
            pas,
            zones,
            settle_pa: cap.u32s("settle_pa"),
            log: false,
            table: upload("cycle table", bytemuck::cast_slice(&table))?,
            words: upload("cycle words", bytemuck::cast_slice(&words))?,
            form,
            pa,
            windows: cap.f32s("pa_windows"),
        })
    }

    /// Each PA's row the balance reads in `cycle` (`hier_cycle.wgsl`'s
    /// `P_`): upkeep, hazard, tolerance, the whole stake's window, apical
    /// control and ψ. Species data the host knows, written before the
    /// cycle is submitted, as its number is.
    pub(super) fn rows_at(&self, cycle: u32) -> Vec<f32> {
        let span = self.age as usize + 2;
        (0..self.pas as usize)
            .flat_map(|p| {
                let r = &self.pa[5 * p..5 * p + 5];
                [r[0], r[1], r[2], self.windows[p * span + cycle as usize], r[3], r[4], 0.0, 0.0]
            })
            .collect()
    }
}

/// The build on the device: the state, the hierarchy's and the light's
/// buffers, the recorded cycle, and the counts its last cycle left.
pub(super) struct Build {
    pub state: State,
    pub hier: Hierarchy,
    pub sums: Sums,
    pub tree: Owned,
    pub light: Light,
    pub layout: Layout,
    /// Retirement's columns and verdicts (DESIGN.md section 6).
    pub retire: super::retire_device::Retire,
    /// The in-place compaction's scratch and scans (`compact.rs`).
    pub compactor: super::compact::Compactor,
    /// The units' inputs and counts, the glue's flags, a spare binding,
    /// and the grown axes' scan's total and sums.
    pub inputs: Owned,
    pub counts: Owned,
    pub flags: Owned,
    pub spare: Owned,
    pub scan_tally: Owned,
    pub scan_sums: Owned,
    pub held: Held,
    pub laid: u32,
    pub grown: u64,
    pub census: Vec<u32>,
}

impl Build {
    /// The buffers for a tree of `caps`, the rows of every cycle uploaded.
    pub(super) fn new(gpu: &Gpu, t: &Tables, (c, caps, block_cap, records): (&Capture, Caps, u32, u32), (lit, nodes): (Lit, (u32, u32))) -> Result<Self> {
        let state = State::new(gpu, caps)?;
        let mut light = Light::new(gpu, &Cap(c))?;
        light.ensure(gpu, lit, nodes)?;
        let s = caps.axes;

        let build = Self {
            hier: Hierarchy::new(gpu, s, block_cap)?,
            sums: Sums::alloc(gpu, s, caps.next, t.pas)?,
            tree: zeroed(gpu, "cycle tree", u64::from(super::hier_columns::TREE as u32 * s))?,
            light,
            layout: Layout::new(gpu, &t.form, s)?,
            retire: super::retire_device::Retire::new(gpu, (s, records), &t.settle_pa, t.log)?,
            compactor: super::compact::Compactor::new(gpu, caps, s)?,
            inputs: zeroed(gpu, "cycle inputs", 2 * u64::from(caps.next) + 6 * u64::from(s))?,
            counts: zeroed(gpu, "cycle counts", device::counts(caps, t.zones).words)?,
            flags: zeroed(gpu, "cycle flags", 2 * u64::from(s))?,
            spare: zeroed(gpu, "cycle spare", 4)?,
            scan_tally: zeroed(gpu, "cycle scan tally", 64)?,
            scan_sums: zeroed(gpu, "cycle scan sums", super::hier_scan::sums_words(s))?,
            state,
            held: Held::default(),
            laid: 0,
            grown: 0,
            census: Vec::new(),
        };
        Ok(build)
    }

    /// The seed recorded, the census set as `Grower::seeded` leaves it.
    /// The census the seed leaves: one axis, living.
    pub(super) fn seed_census() -> Vec<u32> {
        let mut census = vec![0u32; c::WORDS];
        (census[c::AXES], census[c::NEXT]) = (1, 1);
        census
    }

    pub(super) fn seeded(&self, gpu: &Gpu, kit: &Kit, (plan, phases): (&Plan, &Phases)) -> Vec<&'static str> {
        gpu.queue.write_buffer(&plan.census, 0, bytemuck::cast_slice(&Self::seed_census()));
        let mut e = gpu.encoder(&Default::default());
        let mut stamped = Vec::new();
        let all = Limits::full(&super::build_plan::Bounds::default());
        let given = vec![u32::MAX; plan.sized.len()];
        record(kit, (plan, &all, &given), &mut e, &phases.seed, &mut stamped);
        gpu.submit([e.finish()]);
        stamped
    }

    /// Cycle `cycle` recorded as far as `limits` allow, each bounded
    /// dispatch given the most `span` allows, and submitted, its census
    /// sent back late; its dispatches' phases, and its bounded ones.
    pub(super) fn submit(&self, gpu: &Gpu, (kit, t): (&Kit, &Tables), (plan, phases, limits, span): (&Plan, &Phases, &Limits, &Span), (late, cycle): (&mut Late, u32)) -> (Vec<&'static str>, Pending) {
        let steps = phases.cycle(cycle, t.age);
        let given = given(plan, span, (&steps, limits));
        gpu.queue.write_buffer(&plan.census, 4 * c::CYCLE as u64, bytemuck::bytes_of(&cycle));
        gpu.queue.write_buffer(&plan.census, 4 * REC as u64, bytemuck::cast_slice(&limits.words()));
        gpu.queue.write_buffer(&plan.given, 0, bytemuck::cast_slice(&given));
        gpu.queue.write_buffer(&self.sums.rows, 0, bytemuck::cast_slice(&t.rows_at(cycle)));
        let mut e = gpu.encoder(&Default::default());
        let mut stamped = Vec::new();
        for s in &steps {
            record(kit, (plan, limits, &given), &mut e, s, &mut stamped);
        }
        late.copy(&mut e, plan, cycle);
        gpu.submit([e.finish()]);
        late.send(gpu, cycle);
        (stamped, Pending::new(plan, &steps, (limits, &given)))
    }

    #[cfg(test)]
    /// The bytes the build holds resident (the state, the hierarchy's
    /// lists and shape, the tables) and as scratch (every buffer a cycle
    /// writes before it reads).
    pub(super) fn bytes(&self, t: &Tables) -> (u64, u64) {
        let size = |b: &[&Owned]| b.iter().map(|b| b.size()).sum::<u64>();
        let h = &self.hier;
        let resident = self.state.bytes() + self.retire.bytes() + size(&[&h.shape, &h.counts, &h.lists[0], &h.lists[1], &h.tally, &t.table, &t.words]);
        let m = &self.sums;
        let mut scratch = size(&[&h.bucket, &h.sums, &m.sums, &m.live, &m.norm, &m.tally, &m.rows, &m.scan_sums, &self.tree]);
        scratch += size(&[&self.inputs, &self.counts, &self.flags, &self.spare, &self.scan_tally, &self.scan_sums]);
        let (lit, swept) = self.light.bytes();
        (resident, scratch + lit + swept + self.layout.bytes())
    }

    /// Where the units' second count records start in `counts`.
    pub(super) fn b_at(&self) -> u32 {
        device::b_at(self.state.caps.next.max(self.state.caps.sleepers))
    }

    /// The state's counts from the census.
    pub(super) fn counted(&mut self, read: &[u32]) {
        self.held = Held {
            axes: read[c::AXES],
            units: read[c::UNITS],
            phyts: read[c::PHYTS],
            sleepers: read[c::SLEEPERS],
            next: read[c::NEXT],
            woken: read[c::WOKEN],
        };
        (self.laid, self.grown, self.census) = (read[c::LAID], u64::from(read[c::GROWN]), read.to_vec());
    }
}

/// The census read back a cycle late: a copy into one of two mappable
/// buffers at each cycle's end, mapped once its cycle is submitted.
pub(super) struct Late {
    staging: [Owned; 2],
    pending: [Option<(wgpu::SubmissionIndex, Mapped)>; 2],
}

/// A mapping's result, once the device has delivered it.
type Mapped = futures_channel::oneshot::Receiver<std::result::Result<(), wgpu::BufferAsyncError>>;

impl Late {
    pub(super) fn new(gpu: &Gpu) -> Result<Self> {
        let usage = wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST;
        let bytes = 4 * c::WORDS as u64;
        let one = || super::super::io::buffer(gpu, "census read", bytes, usage, &[]);
        Ok(Self { staging: [one()?, one()?], pending: [None, None] })
    }

    fn copy(&self, e: &mut wgpu::CommandEncoder, plan: &Plan, cycle: u32) {
        let to = &self.staging[cycle as usize % 2];
        e.copy_buffer_to_buffer(&plan.census, 0, to, 0, 4 * c::WORDS as u64);
    }

    fn send(&mut self, gpu: &Gpu, cycle: u32) {
        let at = cycle as usize % 2;
        let index = gpu.queue.submit([]);
        let (send, receive) = futures_channel::oneshot::channel();
        self.staging[at].slice(..).map_async(wgpu::MapMode::Read, move |r| {
            let _ = send.send(r);
        });
        self.pending[at] = Some((index, receive));
    }

    pub(super) async fn take(&mut self, gpu: &Gpu, cycle: u32) -> Result<Vec<u32>> {
        let at = cycle as usize % 2;
        let (index, receive) = self.pending[at].take().expect("a cycle's census sent");
        mapped(gpu, (index, receive)).await?;
        let words = {
            let view = self.staging[at].slice(..).get_mapped_range().map_err(|e| super::super::io::error(e.to_string()))?;
            bytemuck::cast_slice(&view).to_vec()
        };
        self.staging[at].unmap();
        Ok(words)
    }
}

/// A mapping's result. Natively it is delivered by the poll that waits
/// for its cycle, or by the next that waits for every submission; a
/// browser delivers it on its own.
async fn mapped(gpu: &Gpu, (index, mut r): (wgpu::SubmissionIndex, Mapped)) -> Result<()> {
    let error = |e: String| super::super::io::error(e);
    #[cfg(not(target_arch = "wasm32"))]
    {
        let wait = wgpu::PollType::Wait { submission_index: Some(index), timeout: None };
        gpu.device.poll(wait).map_err(|e| error(e.to_string()))?;
        match r.try_recv() {
            Ok(Some(done)) => return done.map_err(|e| error(e.to_string())),
            Ok(None) => gpu.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| error(e.to_string())).map(drop)?,
            Err(e) => return Err(error(e.to_string())),
        }
    }
    #[cfg(target_arch = "wasm32")]
    let _ = (gpu, index);
    r.await.map_err(|e| error(e.to_string()))?.map_err(|e| error(e.to_string()))
}
