//! Contract A's fine roots on the GPU inside the stepped executor (fn-211
//! task 10): each cycle the device deposits the items' leaves, samples
//! their buds' light from the CPU's one sweep, and walks their D16
//! (`telperion_space::Feedback`); the CPU runs the rest. Every phase of
//! every cycle, every grower's whole state and every lattice is the
//! all-CPU run's to the word, across cuts, the items' orders and
//! chunkings, the device's own deposit orders and chunkings, and a
//! forced buffer retry.
use super::super::io;
use crate::Gpu;
use telperion_space::fixed::feedback::lattice::Grid;
use telperion_space::fixed::feedback::shed::{Lateral, Walked};
use telperion_space::fixed::feedback::Fault;
use telperion_space::{Feedback, Laid, LayJob, LayPa, Species};

const STEPPED: &str = include_str!("feedback_stepped.wgsl");
const LAY: &str = include_str!("feedback_lay.wgsl");

pub(super) fn source() -> String {
    format!("{}\n{STEPPED}", super::feedback())
}

pub(super) fn lay_source() -> String {
    format!("{}\n{LAY}", super::feedback())
}

const STORAGE: wgpu::BufferUsages = wgpu::BufferUsages::STORAGE.union(wgpu::BufferUsages::COPY_SRC);

fn words(x: i64) -> [u32; 2] {
    [x as u32, (x as u64 >> 32) as u32]
}

/// The order the device takes a cycle's leaves in.
#[derive(Debug, Clone, Copy)]
pub(super) enum Taken {
    Forward,
    Reverse,
    Scrambled,
}

/// The device's side of the exchange, and how it takes its deposits.
pub(super) struct Device<'a> {
    gpu: &'a Gpu,
    pass: io::Pass,
    lay_pass: io::Pass,
    /// Leaves a dispatch, their order, and whether each cycle's first
    /// attempt is into a lattice half the box's size.
    pub chunk: usize,
    pub taken: Taken,
    pub cramped: bool,
    /// The attempts that overflowed, and the dispatches and bytes read
    /// back.
    pub retries: u32,
    pub dispatches: u32,
    pub read_bytes: u64,
    /// The leaves deposited, the buds read, and the laterals that decided
    /// and were shed.
    pub leaves: u64,
    pub reads: u64,
    pub decided: u64,
    pub shed: u64,
    /// The axes and phytomers laid, and those laid resting on the ground.
    pub axes: u64,
    pub phytomers: u64,
    pub grounded: u64,
}

impl<'a> Device<'a> {
    pub fn new(gpu: &'a Gpu, (chunk, taken, cramped): (usize, Taken, bool)) -> Self {
        let read_only = [true, true, true, false, false, false];
        let entries = ["deposit", "sample", "shed"];
        Self {
            gpu,
            pass: io::Pass::new(gpu, source(), &read_only, &entries),
            lay_pass: io::Pass::new(
                gpu,
                lay_source(),
                &[true, true, true, true, false, false],
                &["lay"],
            ),
            chunk,
            taken,
            cramped,
            retries: 0,
            dispatches: 0,
            read_bytes: 0,
            leaves: 0,
            reads: 0,
            decided: 0,
            shed: 0,
            axes: 0,
            phytomers: 0,
            grounded: 0,
        }
    }

    fn buffer(&self, label: &'static str, bytes: &[u8]) -> wgpu::Buffer {
        io::buffer(self.gpu, label, bytes.len() as u64, STORAGE, bytes).expect("a buffer")
    }

    fn zeros(&self, label: &'static str, bytes: usize) -> wgpu::Buffer {
        self.buffer(label, &vec![0u8; bytes.max(16)])
    }

    fn cfg(&self, grid: &Grid, (n, start, capacity): (u32, u32, u32)) -> wgpu::Buffer {
        let limit = self
            .gpu
            .device
            .limits()
            .max_compute_workgroups_per_dimension;
        let row = n.div_ceil(64).min(limit) * 64;
        let low = grid.low.map(|l| l as i32 as u32);
        let size = grid.size.map(|s| s as u32);
        let cfg = [
            n, start, capacity, row, low[0], low[1], low[2], 0, size[0], size[1], size[2], 0,
        ];
        let bytes: &[u8] = bytemuck::cast_slice(&cfg);
        let uniform = wgpu::BufferUsages::UNIFORM;
        io::buffer(self.gpu, "stepped cfg", 48, uniform, bytes).expect("a buffer")
    }

    /// One dispatch of `entry` over `n`, with `bound` at bindings 1 to 6
    /// and a buffer of its own wherever the entry reads none.
    fn run(&mut self, entry: usize, cfg: &wgpu::Buffer, n: u32, bound: [Option<&wgpu::Buffer>; 6]) {
        let unused: Vec<wgpu::Buffer> = (0..6).map(|_| self.zeros("stepped unused", 16)).collect();
        let mut all = vec![cfg];
        all.extend(bound.iter().zip(&unused).map(|(b, u)| b.unwrap_or(u)));
        let bind = self.pass.bind(self.gpu, &all);
        let mut encoder = self.gpu.device.create_command_encoder(&Default::default());
        self.pass.run(&mut encoder, &bind, entry, n.div_ceil(64));
        self.gpu.queue.submit([encoder.finish()]);
        self.dispatches += 1;
    }

    fn read(&mut self, source: &wgpu::Buffer, bytes: usize) -> Vec<u8> {
        self.read_bytes += bytes as u64;
        io::read(self.gpu, source, bytes as u64).expect("a read back")
    }

    fn flag(&mut self, flag: &wgpu::Buffer) -> u32 {
        let bytes = self.read(flag, 4);
        u32::from_le_bytes(bytes[..4].try_into().expect("a word"))
    }

    fn order(&self, n: usize) -> Vec<u32> {
        let n32 = n as u32;
        match self.taken {
            Taken::Forward => (0..n32).collect(),
            Taken::Reverse => (0..n32).rev().collect(),
            Taken::Scrambled => {
                let mut o: Vec<u32> = (0..n32).collect();
                o.sort_by_key(|&k| (u64::from(k) * 2_654_435_761) % (n as u64).max(1));
                o
            }
        }
    }
}

/// Each PA's constants as the lay shader reads them.
fn pa_words(species: &Species) -> Vec<u32> {
    let (pas, cone) = LayPa::table(species);
    let row = |p: &LayPa| {
        let flags = i64::from(p.wanders) | i64::from(p.bends) << 1;
        [
            p.internode,
            p.tropism,
            p.elevation,
            p.wander,
            p.bend_length,
            flags,
            cone,
            0,
        ]
    };
    pas.iter().flat_map(row).flat_map(words).collect()
}

/// The jobs' words: their headers, their units and lifts, their nodes.
fn job_words(jobs: &[LayJob]) -> [Vec<u32>; 3] {
    let (mut head, mut tables, mut nodes) = (Vec::new(), Vec::new(), Vec::new());
    let mut at = 0i64;
    for job in jobs {
        let units = tables.len() as i64 / 2;
        tables.extend(job.units.iter().flatten().copied().flat_map(words));
        let lifts = tables.len() as i64 / 2;
        tables.extend(job.lifts.iter().flatten().copied().flat_map(words));
        let pair = |a: i64, b: i64| a | b << 32;
        let mut row = vec![pair(job.pa as i64, i64::from(job.trunk)), job.bend];
        row.extend(job.layer);
        row.extend([job.base_scale, job.running, job.rank, job.wood]);
        row.push(pair(units, job.units.len() as i64));
        row.push(pair(at, job.nodes.len() as i64));
        row.push(pair(lifts, job.lifts.len() as i64));
        row.push(0);
        head.extend(row.into_iter().flat_map(words));
        for &(key, node, radius) in &job.nodes {
            nodes.extend([key as i64, node, radius].into_iter().flat_map(words));
        }
        at += job.nodes.len() as i64;
    }
    [head, tables, nodes]
}

impl Feedback for Device<'_> {
    fn lay(&mut self, species: &Species, jobs: &[LayJob]) -> Vec<Laid> {
        if jobs.is_empty() {
            return Vec::new();
        }
        let n: usize = jobs.iter().map(|j| j.nodes.len()).sum();
        (self.axes, self.phytomers) = (self.axes + jobs.len() as u64, self.phytomers + n as u64);
        let [head, tables, nodes] = job_words(jobs);
        let pas = self.buffer("lay pas", bytemuck::cast_slice(&pa_words(species)));
        let head = self.buffer("lay jobs", bytemuck::cast_slice(&head));
        let tables = self.buffer("lay tables", bytemuck::cast_slice(&tables));
        let nodes = self.buffer("lay nodes", bytemuck::cast_slice(&nodes));
        let out = self.zeros("lay out", jobs.len() * 19 * 8);
        let laid = self.zeros("lay phytomers", n * 11 * 8);
        let limit = self
            .gpu
            .device
            .limits()
            .max_compute_workgroups_per_dimension;
        let count = jobs.len() as u32;
        let cfg = [count, count.div_ceil(64).min(limit) * 64, 0, 0];
        let uniform = wgpu::BufferUsages::UNIFORM;
        let cfg = io::buffer(self.gpu, "lay cfg", 16, uniform, bytemuck::cast_slice(&cfg))
            .expect("a buffer");
        let bind =
            (self.lay_pass).bind(self.gpu, &[&cfg, &pas, &head, &tables, &nodes, &out, &laid]);
        let mut encoder = self.gpu.device.create_command_encoder(&Default::default());
        self.lay_pass
            .run(&mut encoder, &bind, 0, count.div_ceil(64));
        self.gpu.queue.submit([encoder.finish()]);
        self.dispatches += 1;
        let out = self.read(&out, jobs.len() * 19 * 8);
        let out = &bytemuck::pod_collect_to_vec::<u8, i64>(&out);
        let rows = self.read(&laid, n * 11 * 8);
        let rows = &bytemuck::pod_collect_to_vec::<u8, i64>(&rows);
        self.grounded += rows.chunks(11).filter(|r| r[4] == 0).count() as u64;
        let mut at = 0;
        (jobs.iter().enumerate())
            .map(|(k, job)| {
                let o = &out[k * 19..k * 19 + 19];
                let phytomers = (at..at + job.nodes.len())
                    .map(|p| rows[p * 11..p * 11 + 11].try_into().expect("eleven"))
                    .collect();
                at += job.nodes.len();
                Laid {
                    status: o[0] as u32,
                    z: o[1],
                    layer: o[2..16].try_into().expect("fourteen"),
                    rank: o[16],
                    running: o[17],
                    wood: o[18],
                    phytomers,
                }
            })
            .collect()
    }

    fn shed(&mut self, items: &[(Vec<Lateral>, (bool, u64))]) -> Result<Vec<Vec<Walked>>, Fault> {
        if items.is_empty() {
            return Ok(Vec::new());
        }
        let (mut axes, mut heads) = (Vec::new(), Vec::new());
        let mut first = 0u32;
        for (laterals, (shed, kept)) in items {
            let n = laterals.len() as u32;
            heads.extend([first, n, u32::from(*shed), 0]);
            heads.extend(words(*kept as i64));
            first += n;
            for l in laterals {
                let b = &l.bud;
                axes.extend([l.parent.unwrap_or(u32::MAX), u32::from(l.decides)]);
                let row = [
                    b.q,
                    b.c,
                    b.memory,
                    b.hazard,
                    b.tolerance,
                    b.draw as i64,
                    b.window,
                ];
                axes.extend(row.into_iter().flat_map(words));
            }
        }
        let total = first as usize;
        let axes = self.buffer("stepped axes", bytemuck::cast_slice(&axes));
        let heads = self.buffer("stepped items", bytemuck::cast_slice(&heads));
        let flag = self.zeros("stepped flag", 4);
        let out = self.zeros("stepped walked", total * 24);
        let grid = Grid {
            low: [0; 3],
            size: [1; 3],
        };
        let cfg = self.cfg(&grid, (items.len() as u32, 0, 0));
        let bound = [
            Some(&axes),
            Some(&heads),
            None,
            None,
            Some(&flag),
            Some(&out),
        ];
        self.run(2, &cfg, items.len() as u32, bound);
        if self.flag(&flag) & 8 != 0 {
            return Err(Fault::Range);
        }
        let bytes = self.read(&out, total * 24);
        let got = &bytemuck::pod_collect_to_vec::<u8, u32>(&bytes);
        let walked = |k: usize| {
            let w = &got[k * 6..k * 6 + 6];
            let wide = |lo: u32, hi: u32| u64::from(lo) | u64::from(hi) << 32;
            Walked {
                shed: w[0] != 0,
                kept: wide(w[2], w[3]),
                memory: (w[1] != 0).then_some(wide(w[4], w[5]) as i64),
            }
        };
        let all: Vec<Walked> = (0..total).map(walked).collect();
        self.decided += all.iter().filter(|w| w.memory.is_some()).count() as u64;
        self.shed += all.iter().filter(|w| w.memory.is_some() && w.shed).count() as u64;
        let mut at = 0;
        Ok(items
            .iter()
            .map(|(laterals, _)| {
                let one = (at..at + laterals.len()).map(walked).collect();
                at += laterals.len();
                one
            })
            .collect())
    }

    fn deposit(&mut self, grid: &Grid, leaves: &[([i64; 3], i64)]) -> Result<Vec<u128>, Fault> {
        let nodes = grid.nodes();
        if leaves.is_empty() {
            return Ok(vec![0; nodes]);
        }
        self.leaves += leaves.len() as u64;
        let rows = leaves.iter().flat_map(|(p, a)| [p[0], p[1], p[2], *a]);
        let words: Vec<u32> = rows.flat_map(words).collect();
        let leaves_buffer = self.buffer("stepped leaves", bytemuck::cast_slice(&words));
        let order = self.order(leaves.len());
        let order = self.buffer("stepped order", bytemuck::cast_slice(&order));
        let mut capacity = if self.cramped {
            (nodes / 2).max(1)
        } else {
            nodes
        };
        loop {
            let lattice = self.zeros("stepped lattice", capacity * 16);
            let flag = self.zeros("stepped flag", 4);
            for start in (0..leaves.len()).step_by(self.chunk.max(1)) {
                let n = self.chunk.min(leaves.len() - start) as u32;
                let cfg = self.cfg(grid, (n, start as u32, capacity as u32));
                let (l, o) = (Some(&leaves_buffer), Some(&order));
                let bound = [l, o, None, Some(&lattice), Some(&flag), None];
                self.run(0, &cfg, n, bound);
            }
            let flag = self.flag(&flag);
            if flag & 6 != 0 {
                return Err(Fault::Range);
            }
            if flag & 1 != 0 {
                self.retries += 1;
                capacity = nodes;
                continue;
            }
            let bytes = self.read(&lattice, capacity * 16);
            return Ok(bytemuck::pod_collect_to_vec::<u8, u128>(
                &bytes[..nodes * 16],
            ));
        }
    }

    fn sample(&mut self, grid: &Grid, light: &[i64], at: &[[i64; 3]]) -> Vec<i64> {
        if at.is_empty() {
            return Vec::new();
        }
        self.reads += at.len() as u64;
        let points: Vec<u32> = at.iter().flatten().copied().flat_map(words).collect();
        let points = self.buffer("stepped points", bytemuck::cast_slice(&points));
        let light = self.buffer("stepped light", bytemuck::cast_slice(light));
        let out = self.zeros("stepped reads", at.len() * 8);
        let cfg = self.cfg(grid, (at.len() as u32, 0, 0));
        let (p, l) = (Some(&points), Some(&light));
        let bound = [p, None, l, None, None, Some(&out)];
        self.run(1, &cfg, at.len() as u32, bound);
        let bytes = self.read(&out, at.len() * 8);
        bytemuck::pod_collect_to_vec::<u8, i64>(&bytes)
    }
}
