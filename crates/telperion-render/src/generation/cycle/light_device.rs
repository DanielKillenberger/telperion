//! The cycle's light recorded on the device (DESIGN.md sections 2, step
//! 8, and 19): the box, the deposit, the densities, the sweep and the
//! read, each dispatch its own compute pass (`io::Pass::run`), so a
//! profile stamps each. A refused box or deposit is read back by name.
use super::super::io;
#[cfg(test)]
use super::build_light::{cfg, sky, sweeps_of, LAYERS, START, TILE};
#[cfg(test)]
use super::columns::Cap;
#[cfg(test)]
use crate::buffer::Owned;
use crate::Gpu;
#[cfg(test)]
use crate::Result;

const ENTRIES: [&str; 6] = [
    "span_main",
    "grid_main",
    "deposit_main",
    "densities_main",
    "sweep_main",
    "read_main",
];
#[cfg(test)]
/// The sweep's entry, dispatched once a range of layers.
pub(super) const SWEEP: usize = 4;
#[cfg(test)]
/// Each pass's phase, in `ENTRIES`' order.
const PHASES: [&str; 6] = ["box", "grid", "deposit", "densities", "depths", "read"];
#[cfg(test)]
/// The deposit and the lattice's entries, which a dormant lattice does
/// not record (`plan_lay.rs`'s `plan_light`).
const LATTICE: std::ops::RangeInclusive<usize> = 2..=4;
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

#[cfg(test)]
/// A captured light's inputs as the device reads them.
pub(super) struct Lit {
    /// x, y, z and area a leaf; x, y, z and nothing a tip.
    pub leaves: Vec<f32>,
    pub tips: Vec<f32>,
    /// Each lit direction's words (`build_light::sky`), the most whole
    /// cells a ray crosses a layer, and the open sky's light.
    pub dirs: Vec<u32>,
    pub reach: u32,
    pub open: f32,
    pub lit: bool,
    /// Whether the lattice sleeps: its dispatches not recorded, every
    /// node's light read as one (`build_light.rs`).
    pub dormant: bool,
    /// The nodes the buffers hold.
    pub capacity: u32,
    /// The lattice's size the sweep's dispatches are counted for.
    pub size: [u32; 3],
}

#[cfg(test)]
impl Lit {
    /// The capture's light, its lattice's buffers `capacity` nodes; with
    /// `extinction`, swept under it whatever the site's.
    pub(super) fn new(cap: &Cap, capacity: u32, extinction: Option<f64>) -> Self {
        let k = extinction.unwrap_or(f64::from_bits(cap.u64s("light_extinction")[0]));
        let directions = cap.u64s("light_directions");
        let (dirs, reach) = sky(k, &directions);
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
            reach,
            open: super::build_light::open_light(&directions),
            lit: k > 0.0,
            dormant: false,
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

    /// A direction's real at word `w` of its eight.
    fn real(&self, d: usize, w: usize) -> f32 {
        f32::from_bits(self.dirs[8 * d + w])
    }

    /// Each node's light from the device's depths, as `read_main` takes
    /// it (`light_at`): the open sky's above the swept layers.
    fn lights(&self, depths: &[f32], (nodes, swept): (usize, usize)) -> Vec<u32> {
        let layer = (self.size[0] * self.size[1]) as usize;
        let cap = self.capacity as usize;
        let dirs = self.dirs.len() / 8;
        (0..nodes)
            .map(|i| match i / layer >= swept {
                true => self.open,
                false => (0..dirs)
                    .fold(0.0f32, |s, d| {
                        s + self.real(d, 7) * (-depths[d * cap + i]).exp()
                    })
                    .clamp(0.0, 1.0),
            })
            .map(f32::to_bits)
            .collect()
    }
}

#[cfg(test)]
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

#[cfg(test)]
/// The light's buffers on the device and its passes' bindings.
pub(super) struct Run {
    pub state: Owned,
    pub grid: Owned,
    pub quanta: Owned,
    /// The densities.
    pub field: Owned,
    /// Each direction's depth at every node.
    pub depths: Owned,
    pub reads: Owned,
    start: Owned,
    bind: wgpu::BindGroup,
    /// The sweep's dispatches, each its range of layers.
    sweeps: Vec<wgpu::BindGroup>,
    groups: [u32; 6],
    dormant: bool,
    tips: usize,
    capacity: usize,
    /// Bytes resident (inputs, box, quanta, densities and reads) and
    /// scratch (each direction's depths).
    pub resident: u64,
    pub scratch: u64,
    _held: Vec<Owned>,
}

#[cfg(test)]
impl Run {
    pub(super) fn new(gpu: &Gpu, k: &Kernels, lit: &Lit) -> Result<Self> {
        let [leaves, tips, dirs] = lit.counts();
        assert!(dirs as usize <= DIRECTIONS, "the sky's directions");
        let cap = u64::from(lit.capacity);
        let uniform = wgpu::BufferUsages::UNIFORM;
        let mut cfgs = Vec::new();
        for chunk in 0..sweeps_of(lit.size[2]).max(1) {
            let words = cfg(
                [leaves, tips],
                (dirs, lit.capacity),
                (lit.lit, lit.dormant),
                (chunk, 0),
                (lit.reach, lit.open),
                &lit.dirs,
            );
            cfgs.push(io::buffer(
                gpu,
                "light cfg",
                4 * words.len() as u64,
                uniform,
                bytemuck::cast_slice(&words),
            )?);
        }
        let points = [&lit.leaves[..], &lit.tips[..]].concat();
        let points = storage(
            gpu,
            "light points",
            4 * points.len() as u64,
            bytemuck::cast_slice(&points),
        )?;
        let start = storage(gpu, "light state start", 32, &START)?;
        let state = storage(gpu, "light state", 32, &[])?;
        let grid = storage(gpu, "light grid", 48, &[])?;
        let quanta = storage(gpu, "light quanta", 4 * cap, &[])?;
        let field = storage(gpu, "light field", 4 * cap, &[])?;
        let depths = storage(gpu, "light depths", 4 * u64::from(dirs) * cap, &[])?;
        let reads = storage(gpu, "light reads", 4 * u64::from(tips), &[])?;
        let bind_at = |cfg: &Owned| {
            k.pass.bind(
                gpu,
                &[
                    &**cfg, &points, &state, &grid, &quanta, &field, &depths, &reads,
                ],
            )
        };
        let sweeps: Vec<_> = cfgs.iter().map(bind_at).collect();
        let bind = bind_at(&cfgs[0]);
        let over = |n: u32| n.div_ceil(64).max(1);
        let side = |n: u32| (n + lit.reach * (LAYERS - 1)).div_ceil(TILE);
        let groups = [
            over(leaves + tips),
            1,
            over(leaves),
            over(lit.capacity),
            dirs * side(lit.size[0]) * side(lit.size[1]),
            over(tips),
        ];
        let size = |b: &[&Owned]| b.iter().map(|b| b.size()).sum::<u64>();
        let resident = size(&[&cfgs[0], &points, &state, &grid, &quanta, &field, &reads]);
        let scratch = depths.size();
        Ok(Self {
            state,
            grid,
            quanta,
            field,
            depths,
            reads,
            start,
            bind,
            sweeps,
            groups,
            dormant: lit.dormant,
            tips: tips as usize,
            capacity: cap as usize,
            resident,
            scratch,
            _held: cfgs.into_iter().chain([points]).collect(),
        })
    }

    /// The light recorded into `encoder`, its box and quanta first reset
    /// by unstamped copies; each pass's phase, in order.
    pub(super) fn record(&self, k: &Kernels, e: &mut wgpu::CommandEncoder) -> Vec<&'static str> {
        e.copy_buffer_to_buffer(&self.start, 0, &self.state, 0, 32);
        e.clear_buffer(&self.quanta, 0, None);
        let mut phases = Vec::new();
        for (entry, groups) in self.groups.iter().enumerate() {
            if self.dormant && LATTICE.contains(&entry) {
                continue;
            }
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

    /// The light as the device left it, every value by its word, each
    /// node's light taken from its depths as the read takes it.
    pub(super) fn read(&self, gpu: &Gpu, lit: &Lit) -> Result<Out> {
        let words = |b: &Owned| -> Result<Vec<u32>> {
            Ok(bytemuck::cast_slice(&io::read(gpu, b, b.size())?).to_vec())
        };
        let grid = words(&self.grid)?;
        let n = (grid[6] as usize).min(self.capacity);
        let cut = |mut v: Vec<u32>, len: usize| {
            v.truncate(len);
            v
        };
        let depths: Vec<f32> = words(&self.depths)?
            .into_iter()
            .map(f32::from_bits)
            .collect();
        Ok(Out {
            errors: words(&self.state)?[6],
            low: [0, 1, 2].map(|a| grid[a]),
            size: [3, 4, 5].map(|a| grid[a]),
            swept: grid[8],
            quanta: cut(words(&self.quanta)?, n),
            density: cut(words(&self.field)?, n),
            lattice: lit.lights(&depths, (n, grid[8] as usize)),
            reads: cut(words(&self.reads)?, self.tips),
        })
    }
}

#[cfg(test)]
/// The light as the device left it, every value by its word.
pub(super) struct Out {
    pub errors: u32,
    pub low: [u32; 3],
    pub size: [u32; 3],
    /// The layers the sweep took, from the bottom.
    pub swept: u32,
    pub quanta: Vec<u32>,
    pub density: Vec<u32>,
    pub lattice: Vec<u32>,
    pub reads: Vec<u32>,
}

#[cfg(test)]
impl Out {
    /// The refusals the device made, by name.
    pub(super) fn refusals(&self) -> Vec<&'static str> {
        (0..REFUSALS.len())
            .filter(|b| self.errors >> b & 1 == 1)
            .map(|b| REFUSALS[b])
            .collect()
    }
}
