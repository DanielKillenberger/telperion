//! fn-257 R8: a tree's field read out on the device from the drawn
//! stage's output, a consumer of it as the renderer is. The one cycle
//! grows and draws the tree; the resident stages hang, fold and station
//! it as a harness build does (`build::folded`), its drawn records read
//! as the dress wrote them and no curve swept; `field.wgsl` finds each node's limb
//! system and the box every record stands in, and `field/splat.wgsl`
//! writes every drawn node and leaf-bearing segment into the cells of the
//! requested grid it reaches. Only the grid is read back, and nothing is
//! held on the device after it. The CPU's field (`pipeline/field.rs`) is
//! the reference it is measured against.
use super::super::io;
use crate::buffer::Owned;
use crate::{Gpu, Result};

mod make;
pub(super) use make::{grow, Ask};

const FIELD: &str = include_str!("field.wgsl");
const SPLAT: &str = include_str!("field/splat.wgsl");
const ENTRIES: [&str; 19] = [
    "clear",
    "widest",
    "carrier",
    "opens",
    "order",
    "own",
    "inherit",
    "box_wood",
    "box_leaves",
    "grid_clear",
    "table_clear",
    "bound_leaves",
    "splat_wood",
    "splat_leaves",
    "splat_leaflets",
    "splat_best",
    "splat_best_low",
    "splat_limb",
    "splat_finish",
];
const CLEAR: usize = 0;
const WIDEST: usize = 1;
const CARRIER: usize = 2;
const OPENS: usize = 3;
const ORDER: usize = 4;
const OWN: usize = 5;
const INHERIT: usize = 6;
const BOX_WOOD: usize = 7;
const BOX_LEAVES: usize = 8;
const GRID_CLEAR: usize = 9;
const TABLE_CLEAR: usize = 10;
const BOUND_LEAVES: usize = 11;
const SPLAT_WOOD: usize = 12;
const SPLAT_LEAVES: usize = 13;
const SPLAT_LEAFLETS: usize = 14;
const SPLAT_BEST: usize = 15;
const SPLAT_BEST_LOW: usize = 16;
const SPLAT_LIMB: usize = 17;
const SPLAT_FINISH: usize = 18;

/// The field's kernels, compiled with the rest of a generator's.
pub(super) struct Fielder(io::Pass);

impl Fielder {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let read_only = [true, true, false, false, false, false];
        Self(io::Pass::new(gpu, source(), &read_only, &ENTRIES))
    }
}

fn source() -> String {
    let sources = (
        super::super::sources::rachis(),
        super::super::sources::rest(),
    );
    format!("{}\n{}\n{FIELD}\n{SPLAT}", sources.0, sources.1)
}

/// Every source the field compiles, by name: the shader tests'.
#[cfg(test)]
pub(super) fn sources() -> [(&'static str, String); 1] {
    [("resident field", source())]
}

/// `field.wgsl`'s `Cfg` past its first two words (what a dispatch covers
/// and its row, set per dispatch).
#[derive(Debug, Clone, Copy, Default)]
struct Cfg {
    nodes: u32,
    root: u32,
    most: u32,
    segs: u32,
    flip: u32,
    slots: u32,
    dims: [u32; 4],
    lo: [f32; 4],
    reach: [f32; 4],
    bound: [f32; 4],
    leaf_centre: [f32; 4],
    leaf_half: [f32; 4],
    /// The leaf's wedge it rests on the ground by (`rest.wgsl`).
    wedge: [[f32; 4]; 2],
}

impl Cfg {
    fn words(&self, n: u32, row: u32) -> Vec<u32> {
        let c = self;
        let mut w = vec![n, row, c.nodes, c.root, c.most, c.segs, c.flip, c.slots];
        w.extend(c.dims);
        for v in [
            c.lo,
            c.reach,
            c.bound,
            c.leaf_centre,
            c.leaf_half,
            c.wedge[0],
            c.wedge[1],
        ] {
            w.extend(v.map(f32::to_bits));
        }
        w
    }
}

fn storage(gpu: &Gpu, label: &'static str, words: u64, initial: &[u32]) -> Result<Owned> {
    let usage =
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    io::buffer(
        gpu,
        label,
        words.max(4) * 4,
        usage,
        bytemuck::cast_slice(initial),
    )
}

/// `n` words of `b` from word `at`, read back.
async fn read_words(gpu: &Gpu, b: &wgpu::Buffer, at: u64, n: u64) -> Result<Vec<u32>> {
    let to = storage(gpu, "field read", n, &[])?;
    let mut e = gpu.encoder(&Default::default());
    e.copy_buffer_to_buffer(b, at * 4, &to, 0, n * 4);
    gpu.submit([e.finish()]);
    let raw = io::read_async(gpu, &to, n * 4).await?;
    Ok(bytemuck::cast_slice(&raw).to_vec())
}

/// One recording of the field's kernels, holding each dispatch's uniform
/// until it is submitted.
struct Rec<'a> {
    gpu: &'a Gpu,
    pass: &'a io::Pass,
    held: Vec<Owned>,
}

impl Rec<'_> {
    /// `entry` over `n` items under `cfg`, on `bufs` (its storage
    /// bindings).
    fn run(
        &mut self,
        e: &mut wgpu::CommandEncoder,
        (entry, n, cfg): (usize, u32, &Cfg),
        bufs: &[&wgpu::Buffer],
    ) -> Result<()> {
        let groups = n.div_ceil(64).max(1);
        let limits = self.gpu.device.limits();
        let most = limits.max_compute_workgroups_per_dimension;
        let mut words = cfg.words(n, groups.min(most) * 64);
        words.resize(words.len().next_multiple_of(4), 0);
        let bytes = words.len() as u64 * 4;
        let usage = wgpu::BufferUsages::UNIFORM;
        let uniform = io::buffer(
            self.gpu,
            "field cfg",
            bytes,
            usage,
            bytemuck::cast_slice(&words),
        )?;
        let mut all: Vec<&wgpu::Buffer> = vec![&uniform];
        all.extend(bufs);
        let bind = self.pass.bind(self.gpu, &all);
        self.pass.run(e, &bind, entry, groups);
        self.held.push(uniform);
        Ok(())
    }
}
