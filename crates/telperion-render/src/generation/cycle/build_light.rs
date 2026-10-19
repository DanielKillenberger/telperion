//! A build cycle's light (DESIGN.md sections 2, step 8, and 19) over
//! buffers kept for the build: the leaves and tips the glue gathers from
//! the state (`points`), the box and its size, the deposit, the sweep's
//! depths and the reads (`light.wgsl`, `light_sweep.wgsl`).
use super::super::io;
use super::columns::Cap;
use super::light_device::{DIRECTIONS, REFUSALS};
use super::vm::E;
use crate::buffer::Owned;
use crate::{Gpu, Result};

fn storage(gpu: &Gpu, label: &'static str, words: u64, init: &[u32]) -> Result<Owned> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    io::buffer(
        gpu,
        label,
        words.max(4) * 4,
        usage,
        bytemuck::cast_slice(init),
    )
}

/// The light's buffers, at capacities grown as the tree asks.
pub(super) struct Light {
    pub points: Owned,
    pub reads: Owned,
    pub state: Owned,
    pub grid: Owned,
    pub quanta: Owned,
    pub field: Owned,
    pub depths: Owned,
    pub start: Owned,
    /// Each lit direction's words (`light.wgsl`'s `Cfg::dir`), whether
    /// the site is lit, the whole cells a ray crosses a layer at most, the
    /// open sky's light, and the directions.
    dirs: Vec<u32>,
    lit: bool,
    reach: u32,
    open: f32,
    /// Whether the lattice sleeps: nothing shades (no extinction, or no PA
    /// with leaf area under a sky whose open light is one), so every node's
    /// light is one and neither the deposit nor the lattice's dispatches
    /// are recorded.
    pub dormant: bool,
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
    /// Whether the site is lit (fn-256 task 7's roofline).
    #[cfg(test)]
    pub(super) fn lit(&self) -> bool {
        self.lit
    }

    /// The light of the site the tables name (`grow_capture_tables`).
    pub(super) fn new(gpu: &Gpu, tables: &Cap) -> Result<Self> {
        let k = f64::from_bits(tables.u64s("light_extinction")[0]);
        let directions = tables.u64s("light_directions");
        let (dirs, reach) = sky(k, &directions);
        let n_dirs = (dirs.len() / 8) as u32;
        assert!(n_dirs as usize <= DIRECTIONS, "the sky's directions");
        let mut out = Self {
            points: storage(gpu, "light points", 4, &[])?,
            reads: storage(gpu, "light reads", 4, &[])?,
            state: storage(gpu, "light state", 8, &[])?,
            grid: storage(gpu, "light grid", GRID, &[])?,
            quanta: storage(gpu, "light quanta", 4, &[])?,
            field: storage(gpu, "light field", 4, &[])?,
            depths: storage(gpu, "light depths", 4, &[])?,
            start: storage(gpu, "light state start", 8, &START)?,
            dirs,
            lit: k > 0.0,
            reach,
            open: open_light(&directions),
            dormant: k <= 0.0 || (leafless(tables) && open(&directions)),
            n_dirs,
            points_cap: 0,
            tips_cap: 0,
            nodes_cap: 0,
        };
        out.ensure(
            gpu,
            Lit {
                leaves: 1024,
                tips: 1024,
            },
            4096,
        )?;
        Ok(out)
    }

    /// The buffers grown to hold `lit`'s points, and a lattice of `nodes`
    /// nodes.
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
        // A dormant lattice's deposit and own dispatches are never
        // recorded, so its quanta, densities and depths, which only they
        // touch, stay at their least (fn-256 tasks 11 and 12).
        if !self.dormant && cap != self.nodes_cap {
            self.quanta = storage(gpu, "light quanta", u64::from(cap), &[])?;
            self.field = storage(gpu, "light field", u64::from(cap), &[])?;
            // Each direction's depth, an f32 a node.
            self.depths = storage(
                gpu,
                "light depths",
                u64::from(self.n_dirs) * u64::from(cap),
                &[],
            )?;
        }
        self.nodes_cap = cap;
        Ok(())
    }

    #[cfg(test)]
    /// The bytes the light holds resident and as scratch.
    pub(super) fn bytes(&self) -> (u64, u64) {
        let b = [
            &self.points,
            &self.reads,
            &self.state,
            &self.grid,
            &self.quanta,
            &self.field,
        ];
        (b.iter().map(|b| b.size()).sum(), self.depths.size())
    }

    /// A dispatch's uniform as the host knows it (`light.wgsl`'s `Cfg`):
    /// the leaves and tips none (the device writes them), its sweep chunk
    /// and its range's first item, and the directions.
    pub(super) fn words(&self, (chunk, base): (u32, u32)) -> Vec<u32> {
        cfg(
            [0, 0],
            (self.n_dirs, self.nodes_cap),
            (self.lit, self.dormant),
            (chunk, base),
            (self.reach, self.open),
            &self.dirs,
        )
    }

    /// The workgroups a sweep's dispatch takes: a tile of every direction
    /// over the box's columns and the cells its rays cross in a dispatch's
    /// layers.
    pub(super) fn sweep_tiles(&self, (x, y): (E, E)) -> E {
        let more = E::C(self.reach * (LAYERS - 1));
        x.add(more.clone())
            .div(E::C(TILE))
            .mul(y.add(more).div(E::C(TILE)))
            .mul(E::C(self.n_dirs))
    }

    /// The buffers a dispatch binds after its uniform.
    pub(super) fn buffers(&self) -> [&wgpu::Buffer; 7] {
        [
            &self.points,
            &self.state,
            &self.grid,
            &self.quanta,
            &self.field,
            &self.depths,
            &self.reads,
        ]
    }
}

/// The light's grid words: low node (3), size (3), nodes, whether the box
/// stands, the swept layers.
const GRID: u64 = 12;
/// The box's state before a cycle: its least and most nodes none yet, no
/// error, no leaf with area.
pub(super) const START: [u32; 8] = [
    i32::MAX as u32,
    i32::MAX as u32,
    i32::MAX as u32,
    i32::MIN as u32,
    i32::MIN as u32,
    i32::MIN as u32,
    0,
    i32::MIN as u32,
];

/// The sweep's tile, a side's cells, and the layers a dispatch takes
/// (`light_sweep.wgsl`): its workgroup's memory holds two layers of
/// (TILE + LAYERS)^2 cells.
pub(super) const TILE: u32 = 16;
pub(super) const LAYERS: u32 = 16;
/// The sweep's workgroup.
pub(super) const SWEEP_WG: u32 = 256;

/// The sweep's constants as WGSL.
pub(super) fn wgsl() -> String {
    format!("const TILE: u32 = {TILE}u;\nconst LAYERS: u32 = {LAYERS}u;\nconst SWEEP_WG: u32 = {SWEEP_WG}u;\n")
}

/// The dispatches the sweep of a lattice of `layers` layers takes.
pub(super) fn sweeps_of(layers: u32) -> u32 {
    layers.div_ceil(LAYERS)
}

/// A dispatch's uniform words (`light.wgsl`'s `Cfg`).
pub(super) fn cfg(
    [leaves, tips]: [u32; 2],
    (dirs, capacity): (u32, u32),
    (lit, dormant): (bool, bool),
    (chunk, base): (u32, u32),
    (reach, open): (u32, f32),
    sky: &[u32],
) -> Vec<u32> {
    let row = super::plan::MAXX * 64;
    let mut words = vec![
        leaves,
        tips,
        dirs,
        capacity,
        u32::from(lit),
        row,
        chunk,
        reach,
        base,
        1,
        open.to_bits(),
        u32::from(dormant),
    ];
    words.extend(sky);
    words.resize(12 + 8 * DIRECTIONS, 0);
    words
}

/// Each lit direction's words under extinction `k` (`light.wgsl`'s
/// `Cfg::dir`): the whole cells its ray crosses a layer on x and y, its
/// bilinear weights of the fractions left (`bilinear`'s, in its order),
/// `extinction * path` as `swept` takes it, and its share, the reals
/// taken in f64 and rounded once; and the most whole cells any crosses.
pub(super) fn sky(k: f64, directions: &[u64]) -> (Vec<u32>, u32) {
    let mut words = Vec::new();
    let mut reach = 0;
    for d in directions.as_chunks::<4>().0 {
        let [ox, oy, path, share] = d.map(f64::from_bits);
        let (a, b) = (ox.floor(), oy.floor());
        let (tx, ty) = (ox - a, oy - b);
        let w = [
            (1.0 - tx) * (1.0 - ty),
            (1.0 - tx) * ty,
            tx * (1.0 - ty),
            tx * ty,
        ];
        reach = reach
            .max((a as i32).unsigned_abs())
            .max((b as i32).unsigned_abs());
        words.extend([a as i32 as u32, b as i32 as u32]);
        words.extend(w.map(|w| (w as f32).to_bits()));
        words.extend([((k * path) as f32).to_bits(), (share as f32).to_bits()]);
    }
    (words, reach)
}

/// A PA's form row and its leaf area's word (`layout.wgsl`'s `FORM`,
/// `F_LEAF_AREA`).
const FORM: usize = 20;
const F_LEAF_AREA: usize = 18;

/// Whether no PA deposits leaf area: every leaf's area is nothing.
fn leafless(tables: &Cap) -> bool {
    tables
        .f32s("lay_form")
        .chunks(FORM)
        .all(|row| row[F_LEAF_AREA] == 0.0)
}

/// A node's light at no depth: the directions' shares, each times
/// `exp(-0)`, summed in order, held to one and rounded to f32, as the CPU
/// takes them.
pub(super) fn open_light(directions: &[u64]) -> f32 {
    let sum = directions
        .as_chunks::<4>()
        .0
        .iter()
        .fold(0.0, |s, d| s + f64::from_bits(d[3]));
    sum.clamp(0.0, 1.0) as f32
}

/// Whether a node at no depth takes a light of one.
pub(super) fn open(directions: &[u64]) -> bool {
    open_light(directions) == 1.0
}

/// The refusals in an errors word, by name.
pub(super) fn refusals(errors: u32) -> Vec<&'static str> {
    (0..REFUSALS.len())
        .filter(|b| errors >> b & 1 == 1)
        .map(|b| REFUSALS[b])
        .collect()
}
