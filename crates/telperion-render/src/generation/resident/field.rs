//! fn-257 R1': a tree's field grown on the device and kept there. The one
//! cycle grows and draws the tree, the resident stages hang, fold, sweep
//! and station it as a harness build does (`build::readied`), and
//! `field.wgsl` makes the field's primitives from the swept nodes and the
//! station segments: each node's wood, each segment's leaves on its limb
//! system. They stay on the device in a grid of buckets; a query answers
//! any batch of cubes there and reads back only the answers. The CPU's
//! field (`pipeline/field.rs`) is the reference it is measured against.
use super::super::io;
use crate::buffer::Owned;
use crate::{Gpu, Result};

mod ask;
mod index;
mod make;
pub(super) use make::grow;

pub(super) const FIELD: &str = include_str!("field.wgsl");
const BUILD: &str = include_str!("field_build.wgsl");
const QUERY_SOURCE: &str = include_str!("field_query.wgsl");
const ENTRIES: [&str; 13] = [
    "clear", "widest", "carrier", "opens", "order", "own", "inherit", "wood", "leaves", "leaflets",
    "count", "fill", "marks",
];
const CLEAR: usize = 0;
const WIDEST: usize = 1;
const CARRIER: usize = 2;
const OPENS: usize = 3;
const ORDER: usize = 4;
const OWN: usize = 5;
const INHERIT: usize = 6;
const WOOD: usize = 7;
const LEAVES: usize = 8;
const LEAFLETS: usize = 9;
const COUNT: usize = 10;
const FILL: usize = 11;
const MARKS: usize = 12;
/// A node's work words (`field.wgsl`'s `WORDS`), and the box's six after
/// every node's.
const WORDS: u64 = 7;
/// An answer's flag past `field.wgsl`'s tally of limb systems in one
/// cell, and the systems a tally holds (`TALLY`).
const OVER: u32 = 4;
pub(super) const TALLY: u32 = 128;

/// The field's kernels: its build's, and its query's.
pub(super) struct Fielder {
    build: io::Pass,
    query: io::Pass,
}

impl Fielder {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let build = format!("{FIELD}\n{BUILD}");
        let query = format!("{FIELD}\n{QUERY_SOURCE}");
        Self {
            build: io::Pass::new(gpu, build, &[true, true, false, false, false, false, false], &ENTRIES),
            query: io::Pass::new(gpu, query, &[true, true, true, true, false], &["query"]),
        }
    }
}

/// Every source the field compiles, by name: the shader tests'.
#[cfg(test)]
pub(super) fn sources() -> [(&'static str, String); 2] {
    [
        ("resident field build", format!("{FIELD}\n{BUILD}")),
        ("resident field query", format!("{FIELD}\n{QUERY_SOURCE}")),
    ]
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
    buckets: u32,
    dims: [u32; 4],
    lo: [f32; 4],
    reach: [f32; 4],
    bound: [f32; 4],
    leaf_centre: [f32; 4],
    leaf_half: [f32; 4],
}

impl Cfg {
    fn words(&self, n: u32, row: u32) -> Vec<u32> {
        let c = self;
        let mut w = vec![n, row, c.nodes, c.root, c.most, c.segs, c.flip, c.buckets];
        w.extend(c.dims);
        w.extend(c.lo.map(f32::to_bits));
        w.extend(c.reach.map(f32::to_bits));
        w.extend(c.bound.map(f32::to_bits));
        w.extend(c.leaf_centre.map(f32::to_bits));
        w.extend(c.leaf_half.map(f32::to_bits));
        w
    }
}

fn storage(gpu: &Gpu, label: &'static str, words: u64, initial: &[u32]) -> Result<Owned> {
    let usage =
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    io::buffer(gpu, label, words.max(4) * 4, usage, bytemuck::cast_slice(initial))
}

/// Four empty buffers for the bindings a pass does not read, apart, as
/// WebGPU binds a written buffer once.
fn unbound(gpu: &Gpu) -> Result<[Owned; 4]> {
    let one = || storage(gpu, "field unbound", 4, &[]);
    Ok([one()?, one()?, one()?, one()?])
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

/// A field on the device: its primitives (each node's wood, then each
/// station segment's leaves), their limb systems, named 0 to `systems`,
/// and the grid of buckets they are listed in.
pub struct DeviceField {
    prims: Owned,
    owner: Owned,
    index: Owned,
    cfg: Cfg,
    systems: u32,
    /// The field's box: its least and its most corner, in metres.
    pub bounds: [f32; 6],
    /// The wood's primitives (the drawn nodes) and the leaves'.
    pub wood: u32,
    pub leaves: u32,
}

impl DeviceField {
    /// The device bytes the field holds.
    pub fn bytes(&self) -> u64 {
        self.prims.size() + self.owner.size() + self.index.size()
    }

    /// The limb systems the field names: the tests'.
    #[cfg(test)]
    pub(super) fn named(&self) -> u32 {
        self.systems
    }

    /// The leaves' limb systems, read back: the tests'.
    #[cfg(test)]
    pub(super) fn systems(&self, gpu: &Gpu) -> Vec<u32> {
        let at = u64::from(self.wood);
        pollster::block_on(read_words(gpu, &self.owner, at, u64::from(self.leaves).max(1))).unwrap()
    }

    /// The primitives from `at`, `n` of them, read back: the tests'.
    #[cfg(test)]
    pub(super) fn prims(&self, gpu: &Gpu, at: u64, n: u64) -> Vec<[f32; 8]> {
        let w = pollster::block_on(read_words(gpu, &self.prims, 8 * at, 8 * n.max(1))).unwrap();
        w.chunks(8).map(|p| std::array::from_fn(|k| f32::from_bits(p[k]))).collect()
    }
}

/// One cell's answer (`pipeline::field::Occupancy`, in single
/// precision): wood, its thickest radius, foliage, its leaves and the
/// limb system that holds the most of them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Answer {
    pub wood: bool,
    pub wood_radius: f32,
    pub foliage: bool,
    pub leaves: f32,
    pub limb: Option<u32>,
}

/// One recording of the field's kernels.
struct Rec<'a> {
    gpu: &'a Gpu,
    held: Vec<Owned>,
}

impl Rec<'_> {
    /// `pass`'s `entry` over `n` items under `cfg`, on `bufs` (its storage
    /// bindings).
    fn run(
        &mut self,
        e: &mut wgpu::CommandEncoder,
        (pass, entry, n, cfg): (&io::Pass, usize, u32, &Cfg),
        bufs: &[&wgpu::Buffer],
    ) -> Result<()> {
        let groups = n.div_ceil(64).max(1);
        let most = self.gpu.device.limits().max_compute_workgroups_per_dimension;
        let mut words = cfg.words(n, groups.min(most) * 64);
        words.resize(words.len().next_multiple_of(4), 0);
        let usage = wgpu::BufferUsages::UNIFORM;
        let bytes = words.len() as u64 * 4;
        let uniform = io::buffer(self.gpu, "field cfg", bytes, usage, bytemuck::cast_slice(&words))?;
        let mut all: Vec<&wgpu::Buffer> = vec![&uniform];
        all.extend(bufs);
        let bind = pass.bind(self.gpu, &all);
        pass.run(e, &bind, entry, groups);
        self.held.push(uniform);
        Ok(())
    }
}

