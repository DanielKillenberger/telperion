//! The cycle's rough layout recorded on the device (DESIGN.md section
//! 2.3): the grown axes laid, each relay set at its node, the new axes'
//! generations found, then per generation its axes listed by a flag, a
//! scan and a gather, framed and laid. Each dispatch is its own compute
//! pass (`io::Pass::run`), so a profile stamps each; a generation's count
//! moves by a copy from the scan's total into the tally, never to the
//! host.
use super::super::io;
use super::lay_columns::Laid;
use crate::buffer::Owned;
use crate::{Gpu, Result};

const SCAN: &str = include_str!("../fine/scan.wgsl");

const DRAW: usize = 0;
const FRAME: usize = 1;
const RELAY: usize = 2;
const GENERATION: usize = 3;
const FLAG: usize = 4;
const GATHER: usize = 5;

pub(super) struct Kernels {
    layout: io::Pass,
    scan: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let mut reads = vec![true; 2];
        reads.extend([false; 5]);
        let entries = [
            "draw_main",
            "frame_main",
            "relay_main",
            "generation_main",
            "flag_main",
            "gather_main",
        ];
        Self {
            layout: io::Pass::new(gpu, super::layout_source(), &reads, &entries),
            scan: io::Pass::new(gpu, SCAN.into(), &[false; 3], &["blocks", "spine", "spread"]),
        }
    }
}

fn storage(gpu: &Gpu, label: &'static str, words: &[u32]) -> Result<Owned> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    io::buffer(gpu, label, (words.len() as u64 * 4).max(16), usage, bytemuck::cast_slice(words))
}

/// Workgroups over `n` invocations, and the row a dispatch spreads them in.
fn row(gpu: &Gpu, n: u32) -> (u32, u32) {
    let groups = n.div_ceil(64).max(1);
    let most = gpu.device.limits().max_compute_workgroups_per_dimension;
    (groups, groups.min(most) * 64)
}

/// A captured layout on the device: its tables, the state it starts from,
/// kept to start each run again, and the working buffers.
pub(super) struct Run {
    pub axes: Owned,
    pub phyts: Owned,
    pub pencils: Owned,
    pub lists: Owned,
    start: [Owned; 4],
    /// The grown axes' draw, the new axes' (relay and generation), then
    /// each generation's.
    binds: Vec<(u32, wgpu::BindGroup)>,
    scan: (wgpu::BindGroup, u32, u32),
    totals: Owned,
    generations: usize,
    /// The bytes the layout holds resident (its state and tables) and its
    /// scratch (the lists, flags and scan sums), the copies the runs
    /// restart from aside.
    pub resident: u64,
    pub scratch: u64,
    _held: Vec<Owned>,
}

impl Run {
    pub(super) fn new(gpu: &Gpu, k: &Kernels, laid: &Laid) -> Result<Self> {
        let start = [
            storage(gpu, "layout axes start", &laid.in_axes)?,
            storage(gpu, "layout phytomers start", &laid.in_phyts)?,
            storage(gpu, "layout pencils start", &laid.in_pencils)?,
            storage(gpu, "layout lists start", &laid.lists())?,
        ];
        let work = |b: &Owned, label| {
            let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
            io::buffer(gpu, label, b.size(), usage, &[])
        };
        let axes = work(&start[0], "layout axes")?;
        let phyts = work(&start[1], "layout phytomers")?;
        let pencils = work(&start[2], "layout pencils")?;
        let lists = work(&start[3], "layout lists")?;
        let form = storage(gpu, "layout form", bytemuck::cast_slice(&laid.form))?;
        let units = storage(gpu, "layout units", &laid.in_units)?;
        let new = laid.new_axes();
        let counts = storage(gpu, "layout flags", &vec![0; new.max(1) as usize])?;
        let generations = laid.generations.len();
        let mut held = Vec::new();
        let mut binds = Vec::new();
        let mut jobs = vec![(laid.grown(), 0, 0), (new, 0, 0)];
        jobs.extend((0..generations as u32).map(|g| (new, 1, g)));
        for (n, listed, generation) in jobs {
            let words = config(gpu, laid, (n, listed, generation));
            let c = io::buffer(gpu, "layout cfg", 64, wgpu::BufferUsages::UNIFORM, bytemuck::cast_slice(&words))?;
            let all = [&*c, &form, &units, &axes, &phyts, &pencils, &lists, &counts];
            binds.push((n, k.layout.bind(gpu, &all)));
            held.push(c);
        }
        let blocks = new.div_ceil(256);
        let scan_cfg = [new, 1, 1, blocks, row(gpu, new).1, 0, 0, 0];
        let cfg = io::buffer(gpu, "scan cfg", 32, wgpu::BufferUsages::UNIFORM, bytemuck::cast_slice(&scan_cfg))?;
        let sums = storage(gpu, "scan sums", &vec![0; 4 * blocks.max(1) as usize])?;
        let totals = storage(gpu, "scan totals", &[0; 4])?;
        let scan = k.scan.bind(gpu, &[&cfg, &counts, &sums, &totals]);
        let size = |b: &[&Owned]| b.iter().map(|b| b.size()).sum::<u64>();
        let resident = size(&[&axes, &phyts, &pencils, &form, &units]);
        let scratch = size(&[&lists, &counts, &sums, &totals]);
        held.extend([form, units, counts, cfg, sums]);
        Ok(Self {
            axes,
            phyts,
            pencils,
            lists,
            start,
            binds,
            scan: (scan, blocks, row(gpu, new).0),
            totals,
            generations,
            resident,
            scratch,
            _held: held,
        })
    }

    /// The layout recorded into `encoder` from the captured state; each
    /// pass's phase, in order.
    pub(super) fn record(&self, gpu: &Gpu, k: &Kernels, e: &mut wgpu::CommandEncoder) -> Vec<&'static str> {
        let working = [&self.axes, &self.phyts, &self.pencils, &self.lists];
        for (from, to) in self.start.iter().zip(working) {
            e.copy_buffer_to_buffer(from, 0, to, 0, from.size());
        }
        let mut phases = Vec::new();
        let run = |e: &mut wgpu::CommandEncoder, entry: usize, job: usize| {
            let (n, bind) = &self.binds[job];
            k.layout.run(e, bind, entry, row(gpu, *n).0);
        };
        run(e, DRAW, 0);
        phases.push("draw grown");
        if self.generations == 0 {
            return phases;
        }
        run(e, RELAY, 1);
        run(e, GENERATION, 1);
        phases.extend(["relay", "generation"]);
        for g in 0..self.generations {
            run(e, FLAG, 2 + g);
            let (bind, blocks, groups) = &self.scan;
            k.scan.run(e, bind, 0, blocks.div_ceil(64).max(1));
            k.scan.run(e, bind, 1, 1);
            k.scan.run(e, bind, 2, *groups);
            e.copy_buffer_to_buffer(&self.totals, 0, &self.lists, 0, 4);
            run(e, GATHER, 2 + g);
            run(e, FRAME, 2 + g);
            run(e, DRAW, 2 + g);
            phases.extend(["list"; 5]);
            phases.extend(["frame", "draw new"]);
        }
        phases
    }
}

/// The kernels' uniform (`layout.wgsl`'s `Cfg`) over `n` invocations,
/// drawing the grown axes or generation `generation`'s.
fn config(gpu: &Gpu, laid: &Laid, (n, listed, generation): (u32, u32, u32)) -> [u32; 16] {
    [
        n,
        row(gpu, n).1,
        laid.cycle,
        laid.first_new,
        laid.axes,
        laid.phyts,
        laid.units,
        laid.new_axes(),
        listed,
        generation,
        0,
        laid.grown(),
        0,
        0,
        0,
        0,
    ]
}
