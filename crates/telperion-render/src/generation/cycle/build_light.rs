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
    /// Whether the lattice sleeps: nothing shades (no extinction, or no PA
    /// with leaf area under a sky whose open light is one), so every node's
    /// light is one and the lattice's dispatches are not recorded.
    pub dormant: bool,
    pub n_dirs: u32,
    /// The points, tips, lattice nodes and lattice columns (a layer's
    /// nodes) the buffers hold.
    pub points_cap: u32,
    pub tips_cap: u32,
    pub nodes_cap: u32,
    pub columns_cap: u32,
}

/// A cycle's leaves and tips.
#[derive(Debug, Clone, Copy)]
pub(super) struct Lit {
    pub leaves: u32,
    pub tips: u32,
}

impl Light {
    /// Whether the site is lit (fn-256 task 7's roofline).
    #[cfg(test)]
    pub(super) fn lit(&self) -> bool {
        self.lit
    }

    /// The light of the site the tables name (`grow_capture_tables`).
    pub(super) fn new(gpu: &Gpu, tables: &Cap) -> Result<Self> {
        let k = f64::from_bits(tables.u64s("light_extinction")[0]);
        let mut dirs = Vec::new();
        let directions = tables.u64s("light_directions");
        for d in directions.as_chunks::<4>().0 {
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
            dormant: k <= 0.0 || (leafless(tables) && open(&directions)),
            n_dirs,
            points_cap: 0,
            tips_cap: 0,
            nodes_cap: 0,
            columns_cap: 0,
        };
        out.ensure(gpu, Lit { leaves: 1024, tips: 1024 }, (4096, 1024))?;
        Ok(out)
    }

    /// The buffers grown to hold `lit`'s points, and a lattice of `nodes`
    /// nodes whose layers hold `columns`.
    pub(super) fn ensure(&mut self, gpu: &Gpu, lit: Lit, (nodes, columns): (u32, u32)) -> Result<()> {
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
        let (cap, cols) = (more(self.nodes_cap, nodes), more(self.columns_cap, columns));
        if cap != self.nodes_cap {
            let c = u64::from(cap);
            self.quanta = storage(gpu, "light quanta", c, &[])?;
            self.field = storage(gpu, "light field", 2 * c, &[])?;
        }
        if (cap, cols) != (self.nodes_cap, self.columns_cap) {
            // Each direction's depths, an f64 a node, then each direction's
            // column records, five f64s a column of a layer (fn-256 task 5:
            // the records were sized a node each, 432 bytes a node).
            let words = 2 * u64::from(self.n_dirs) * (u64::from(cap) + 5 * u64::from(cols));
            self.scratch = storage(gpu, "light scratch", words, &[])?;
            (self.nodes_cap, self.columns_cap) = (cap, cols);
        }
        Ok(())
    }

    #[cfg(test)]
    /// The bytes the light holds resident and as scratch.
    pub(super) fn bytes(&self) -> (u64, u64) {
        let b = [&self.points, &self.reads, &self.state, &self.grid, &self.quanta, &self.field];
        (b.iter().map(|b| b.size()).sum(), self.scratch.size())
    }

    /// A dispatch's uniform as the host knows it (`light.wgsl`'s `Cfg`):
    /// the leaves and tips none (the device writes them), its sweep chunk
    /// and its range's first item, and the directions.
    pub(super) fn words(&self, (chunk, base): (u32, u32)) -> Vec<u32> {
        let (lit, dormant) = (u32::from(self.lit), u32::from(self.dormant));
        let mut cfg = vec![0, 0, self.n_dirs, self.nodes_cap, lit, super::plan::MAXX * 64, chunk, 0, base, 1, self.columns_cap, dormant];
        cfg.extend(&self.dirs);
        cfg.resize(12 + 8 * DIRECTIONS, 0);
        cfg
    }

    /// The buffers a dispatch binds after its uniform.
    pub(super) fn buffers(&self) -> [&wgpu::Buffer; 7] {
        [&self.points, &self.state, &self.grid, &self.quanta, &self.field, &self.scratch, &self.reads]
    }
}

/// A PA's form row and its leaf area's word (`layout.wgsl`'s `FORM`,
/// `F_LEAF_AREA`).
const FORM: usize = 19;
const F_LEAF_AREA: usize = 18;

/// Whether no PA deposits leaf area: every leaf's area is nothing.
fn leafless(tables: &Cap) -> bool {
    tables.f32s("lay_form").chunks(FORM).all(|row| row[F_LEAF_AREA] == 0.0)
}

/// Whether a node at no depth takes a light of one: the directions'
/// shares, each times `exp(-0)`, summed in order, held to one and
/// rounded to f32, as `combine_main` takes them.
pub(super) fn open(directions: &[u64]) -> bool {
    let sum = directions.as_chunks::<4>().0.iter().fold(0.0, |s, d| s + f64::from_bits(d[3]));
    sum.clamp(0.0, 1.0) as f32 == 1.0
}

/// The refusals in an errors word, by name.
pub(super) fn refusals(errors: u32) -> Vec<&'static str> {
    (0..REFUSALS.len()).filter(|b| errors >> b & 1 == 1).map(|b| REFUSALS[b]).collect()
}
