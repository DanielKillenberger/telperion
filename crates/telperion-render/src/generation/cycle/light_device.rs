//! The cycle's light recorded on the device (DESIGN.md section 2, step
//! 8): the box, the deposit, the densities, the sweep, the combine and
//! the read, each dispatch its own compute pass (`io::Pass::run`), so a
//! profile stamps each. A refused box or deposit is read back by name.
use super::super::io;
use super::columns::Cap;
use crate::buffer::Owned;
use crate::{Gpu, Result};

const ENTRIES: [&str; 9] = [
    "span_main",
    "grid_main",
    "deposit_main",
    "densities_main",
    "weights_main",
    "path_main",
    "sweep_main",
    "combine_main",
    "read_main",
];
/// The depth recurrence's entry, dispatched once a range of layers.
pub(super) const SWEEP: usize = 6;
/// Each pass's phase, in `ENTRIES`' order.
const PHASES: [&str; 9] = [
    "box",
    "grid",
    "deposit",
    "densities",
    "weights",
    "paths",
    "depths",
    "combine",
    "read",
];
/// The errors `light.wgsl` sets, by bit, as the CPU names them.
pub(super) const REFUSALS: [&str; 6] = [
    "positions inside the contract's domain",
    "a lattice inside the contract's domain",
    "a lattice inside the buffers' capacity",
    "a leaf's area inside the contract's bound",
    "light.deposit: a lattice node holds at most 2^32 quanta of 2^-20 m² of leaf area",
    "a leaf inside its box",
];

pub(super) struct Kernels {
    pub(super) pass: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let mut reads = vec![true; 1];
        reads.extend([false; 6]);
        Self {
            pass: io::Pass::new(gpu, super::light_source(), &reads, &ENTRIES),
        }
    }
}

/// A captured light's inputs as the device reads them.
pub(super) struct Lit {
    /// x, y, z and area a leaf; x, y, z and nothing a tip.
    pub leaves: Vec<f32>,
    pub tips: Vec<f32>,
    /// Each lit direction's x/z, y/z, extinction times its path, and its
    /// share, as f64 words.
    pub dirs: Vec<u32>,
    pub lit: bool,
    /// The nodes the buffers hold.
    pub capacity: u32,
    /// The lattice's size the sweep's dispatches are counted for.
    pub size: [u32; 3],
}

impl Lit {
    /// The capture's light, its lattice's buffers `capacity` nodes; with
    /// `extinction`, swept under it whatever the site's.
    pub(super) fn new(cap: &Cap, capacity: u32, extinction: Option<f64>) -> Self {
        let k = extinction.unwrap_or(f64::from_bits(cap.u64s("light_extinction")[0]));
        let mut dirs = Vec::new();
        for d in cap.u64s("light_directions").as_chunks::<4>().0 {
            // `extinction * path`, as `swept` takes it.
            let ep = k * f64::from_bits(d[2]);
            for w in [d[0], d[1], ep.to_bits(), d[3]] {
                dirs.extend([w as u32, (w >> 32) as u32]);
            }
        }
        let tips = cap.f32s("light_tips");
        let tips = tips
            .as_chunks::<3>()
            .0
            .iter()
            .flat_map(|t| [t[0], t[1], t[2], 0.0])
            .collect();
        Self {
            leaves: cap.f32s("light_leaves"),
            tips,
            dirs,
            lit: k > 0.0,
            capacity,
            size: std::array::from_fn(|a| cap.u32s("cpu_grid_size")[a]),
        }
    }

    pub(super) fn counts(&self) -> [u32; 3] {
        [
            self.leaves.len() as u32 / 4,
            self.tips.len() as u32 / 4,
            self.dirs.len() as u32 / 8,
        ]
    }
}

fn storage(gpu: &Gpu, label: &'static str, bytes: u64, words: &[u32]) -> Result<Owned> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    io::buffer(
        gpu,
        label,
        bytes.max(16),
        usage,
        bytemuck::cast_slice(words),
    )
}

/// The directions the uniform holds: the sky's nine at most.
pub(super) const DIRECTIONS: usize = 9;

/// The light's buffers on the device and its passes' bindings.
pub(super) struct Run {
    pub state: Owned,
    pub grid: Owned,
    pub quanta: Owned,
    /// The densities, then from the capacity on the lattice.
    pub field: Owned,
    pub reads: Owned,
    start: Owned,
    bind: wgpu::BindGroup,
    /// The sweep's dispatches, each its range of layers.
    sweeps: Vec<wgpu::BindGroup>,
    groups: [u32; 9],
    tips: usize,
    capacity: usize,
    /// Bytes resident (inputs, box, quanta, densities, lattice and reads)
    /// and scratch (each direction's light and its depths).
    pub resident: u64,
    pub scratch: u64,
    _held: Vec<Owned>,
}

impl Run {
    pub(super) fn new(gpu: &Gpu, k: &Kernels, lit: &Lit) -> Result<Self> {
        let [leaves, tips, dirs] = lit.counts();
        assert!(dirs as usize <= DIRECTIONS, "the sky's directions");
        let cap = u64::from(lit.capacity);
        let most = gpu.device.limits().max_compute_workgroups_per_dimension;
        let mut cfg = vec![
            leaves,
            tips,
            dirs,
            lit.capacity,
            u32::from(lit.lit),
            most * 64,
            0,
            0,
            0,
            1,
            0,
            0,
        ];
        cfg.extend(&lit.dirs);
        cfg.resize(12 + 8 * DIRECTIONS, 0);
        let uniform = wgpu::BufferUsages::UNIFORM;
        let chunks = super::light_program::sweep_dispatches(lit.size);
        let mut cfgs = Vec::new();
        for chunk in 0..chunks {
            cfg[6] = chunk;
            let words = bytemuck::cast_slice(&cfg);
            cfgs.push(io::buffer(
                gpu,
                "light cfg",
                4 * cfg.len() as u64,
                uniform,
                words,
            )?);
        }
        let points = [&lit.leaves[..], &lit.tips[..]].concat();
        let points = storage(
            gpu,
            "light points",
            4 * points.len() as u64,
            bytemuck::cast_slice(&points),
        )?;
        let low = i32::MAX as u32;
        let high = i32::MIN as u32;
        let start = storage(
            gpu,
            "light state start",
            32,
            &[low, low, low, high, high, high, 0, 0],
        )?;
        let state = storage(gpu, "light state", 32, &[])?;
        let grid = storage(gpu, "light grid", 32, &[])?;
        let quanta = storage(gpu, "light quanta", 4 * cap, &[])?;
        let field = storage(gpu, "light field", 8 * cap, &[])?;
        let scratch = storage(gpu, "light scratch", 48 * u64::from(dirs) * cap, &[])?;
        let reads = storage(gpu, "light reads", 4 * u64::from(tips), &[])?;
        let bind_at = |cfg: &Owned| {
            let all = [
                &**cfg, &points, &state, &grid, &quanta, &field, &scratch, &reads,
            ];
            k.pass.bind(gpu, &all)
        };
        let sweeps: Vec<_> = cfgs.iter().map(bind_at).collect();
        let bind = bind_at(&cfgs[0]);
        let over = |n: u32| n.div_ceil(64).max(1);
        let groups = [
            over(leaves + tips),
            1,
            over(leaves),
            over(lit.capacity),
            over(dirs * lit.capacity),
            over(dirs * lit.capacity),
            dirs.max(1),
            over(lit.capacity),
            over(tips),
        ];
        let size = |b: &[&Owned]| b.iter().map(|b| b.size()).sum::<u64>();
        let resident = size(&[&cfgs[0], &points, &state, &grid, &quanta, &field, &reads]);
        let scratch_bytes = scratch.size();
        Ok(Self {
            state,
            grid,
            quanta,
            field,
            reads,
            start,
            bind,
            sweeps,
            groups,
            tips: tips as usize,
            capacity: cap as usize,
            resident,
            scratch: scratch_bytes,
            _held: cfgs.into_iter().chain([points, scratch]).collect(),
        })
    }

    /// The light recorded into `encoder`, its box and quanta first reset
    /// by unstamped copies; each pass's phase, in order.
    pub(super) fn record(&self, k: &Kernels, e: &mut wgpu::CommandEncoder) -> Vec<&'static str> {
        e.copy_buffer_to_buffer(&self.start, 0, &self.state, 0, 32);
        e.clear_buffer(&self.quanta, 0, None);
        let mut phases = Vec::new();
        for (entry, groups) in self.groups.iter().enumerate() {
            let binds = if entry == SWEEP {
                &self.sweeps[..]
            } else {
                std::slice::from_ref(&self.bind)
            };
            for bind in binds {
                k.pass.run(e, bind, entry, *groups);
                phases.push(PHASES[entry]);
            }
        }
        phases
    }

    pub(super) fn read(&self, gpu: &Gpu) -> Result<Out> {
        let words = |b: &Owned| -> Result<Vec<u32>> {
            Ok(bytemuck::cast_slice(&io::read(gpu, b, b.size())?).to_vec())
        };
        let grid = words(&self.grid)?;
        let field = words(&self.field)?;
        let n = (grid[6] as usize).min(self.capacity);
        let cut = |mut v: Vec<u32>, len: usize| {
            v.truncate(len);
            v
        };
        Ok(Out {
            errors: words(&self.state)?[6],
            low: [0, 1, 2].map(|a| grid[a]),
            size: [3, 4, 5].map(|a| grid[a]),
            quanta: cut(words(&self.quanta)?, n),
            density: field[..n].to_vec(),
            lattice: field[self.capacity..self.capacity + n].to_vec(),
            reads: cut(words(&self.reads)?, self.tips),
        })
    }
}

/// The light as the device left it, every value by its word.
pub(super) struct Out {
    pub errors: u32,
    pub low: [u32; 3],
    pub size: [u32; 3],
    pub quanta: Vec<u32>,
    pub density: Vec<u32>,
    pub lattice: Vec<u32>,
    pub reads: Vec<u32>,
}

impl Out {
    /// The refusals the device made, by name.
    pub(super) fn refusals(&self) -> Vec<&'static str> {
        (0..REFUSALS.len())
            .filter(|b| self.errors >> b & 1 == 1)
            .map(|b| REFUSALS[b])
            .collect()
    }
}
