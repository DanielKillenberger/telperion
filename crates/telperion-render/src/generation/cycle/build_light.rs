//! A build cycle's light (DESIGN.md section 2, step 8) over buffers kept
//! for the build: the leaves and tips the glue gathers from the state
//! (`points`), the box and its size, the deposit, the sweep in software
//! f64, the combine and the reads (`light.wgsl`, `light_sweep.wgsl`).
use super::super::io;
use super::columns::Cap;
use super::light_device::{DIRECTIONS, REFUSALS};
use crate::buffer::Owned;
use crate::{Gpu, Result};

fn storage(gpu: &Gpu, label: &'static str, words: u64, init: &[u32]) -> Result<Owned> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    io::buffer(gpu, label, words.max(4) * 4, usage, bytemuck::cast_slice(init))
}

/// The light's buffers, at capacities grown as the tree asks.
pub(super) struct Light {
    pub points: Owned,
    pub reads: Owned,
    pub state: Owned,
    pub grid: Owned,
    pub quanta: Owned,
    pub field: Owned,
    pub scratch: Owned,
    pub start: Owned,
    /// Each lit direction's words (`light.wgsl`'s `Cfg::dir`), whether
    /// the site is lit, and the directions.
    dirs: Vec<u32>,
    lit: bool,
    pub n_dirs: u32,
    /// The points, tips and lattice nodes the buffers hold.
    pub points_cap: u32,
    pub tips_cap: u32,
    pub nodes_cap: u32,
}

/// A cycle's leaves and tips.
#[derive(Debug, Clone, Copy)]
pub(super) struct Lit {
    pub leaves: u32,
    pub tips: u32,
}

impl Light {
    /// The light of the site the tables name (`grow_capture_tables`).
    pub(super) fn new(gpu: &Gpu, tables: &Cap) -> Result<Self> {
        let k = f64::from_bits(tables.u64s("light_extinction")[0]);
        let mut dirs = Vec::new();
        for d in tables.u64s("light_directions").as_chunks::<4>().0 {
            // `extinction * path`, as `swept` takes it.
            let ep = k * f64::from_bits(d[2]);
            for w in [d[0], d[1], ep.to_bits(), d[3]] {
                dirs.extend([w as u32, (w >> 32) as u32]);
            }
        }
        let n_dirs = (dirs.len() / 8) as u32;
        assert!(n_dirs as usize <= DIRECTIONS, "the sky's directions");
        let (low, high) = (i32::MAX as u32, i32::MIN as u32);
        let mut out = Self {
            points: storage(gpu, "light points", 4, &[])?,
            reads: storage(gpu, "light reads", 4, &[])?,
            state: storage(gpu, "light state", 8, &[])?,
            grid: storage(gpu, "light grid", 8, &[])?,
            quanta: storage(gpu, "light quanta", 4, &[])?,
            field: storage(gpu, "light field", 4, &[])?,
            scratch: storage(gpu, "light scratch", 4, &[])?,
            start: storage(gpu, "light state start", 8, &[low, low, low, high, high, high, 0, 0])?,
            dirs,
            lit: k > 0.0,
            n_dirs,
            points_cap: 0,
            tips_cap: 0,
            nodes_cap: 0,
        };
        out.ensure(gpu, Lit { leaves: 1024, tips: 1024 }, 4096)?;
        Ok(out)
    }

    /// The buffers grown to hold `lit`'s points and `nodes` nodes.
    pub(super) fn ensure(&mut self, gpu: &Gpu, lit: Lit, nodes: u32) -> Result<()> {
        let more = |have: u32, want: u32| if want <= have { have } else { want + want / 2 };
        let points = more(self.points_cap, lit.leaves + lit.tips);
        if points != self.points_cap {
            self.points = storage(gpu, "light points", 4 * u64::from(points), &[])?;
            self.points_cap = points;
        }
        let tips = more(self.tips_cap, lit.tips);
        if tips != self.tips_cap {
            self.reads = storage(gpu, "light reads", u64::from(tips), &[])?;
            self.tips_cap = tips;
        }
        let cap = more(self.nodes_cap, nodes);
        if cap != self.nodes_cap {
            let c = u64::from(cap);
            self.quanta = storage(gpu, "light quanta", c, &[])?;
            self.field = storage(gpu, "light field", 2 * c, &[])?;
            self.scratch = storage(gpu, "light scratch", 12 * u64::from(self.n_dirs) * c, &[])?;
            self.nodes_cap = cap;
        }
        Ok(())
    }

    /// The bytes the light holds resident and as scratch.
    pub(super) fn bytes(&self) -> (u64, u64) {
        let b = [&self.points, &self.reads, &self.state, &self.grid, &self.quanta, &self.field];
        (b.iter().map(|b| b.size()).sum(), self.scratch.size())
    }

    /// A dispatch's uniform as the host knows it (`light.wgsl`'s `Cfg`):
    /// the leaves and tips none (the device writes them), its sweep chunk
    /// and its range's first item, and the directions.
    pub(super) fn words(&self, (chunk, base): (u32, u32)) -> Vec<u32> {
        let mut cfg = vec![0, 0, self.n_dirs, self.nodes_cap, u32::from(self.lit), super::plan::MAXX * 64, chunk, 0, base, 0, 0, 0];
        cfg.extend(&self.dirs);
        cfg.resize(12 + 8 * DIRECTIONS, 0);
        cfg
    }

    /// The buffers a dispatch binds after its uniform.
    pub(super) fn buffers(&self) -> [&wgpu::Buffer; 7] {
        [&self.points, &self.state, &self.grid, &self.quanta, &self.field, &self.scratch, &self.reads]
    }
}

/// The refusals in an errors word, by name.
pub(super) fn refusals(errors: u32) -> Vec<&'static str> {
    (0..REFUSALS.len()).filter(|b| errors >> b & 1 == 1).map(|b| REFUSALS[b]).collect()
}
