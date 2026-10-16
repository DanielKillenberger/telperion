//! fn-257 R1': a tree's field grown on the device and kept there. The one
//! cycle grows and draws the tree, the resident stages hang, fold, sweep
//! and station it as a harness build does (`build::readied`), and
//! `field.wgsl` makes the field's primitives from the swept nodes and the
//! station segments: each node's wood, each segment's leaves on its limb
//! system. They stay on the device in a grid of buckets; a query answers
//! any batch of cubes there and reads back only the answers. The CPU's
//! field (`pipeline/field.rs`) is the reference it is measured against.
use super::build::{placeable, readied, Readied, Timed, Tools};
use super::stations::station_rows;
use super::super::cycle::tree::{self, Kernels};
use super::super::{io, Generator};
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_core::pipeline::executor;
use telperion_core::presets::Family;
use telperion_core::Error;

pub(super) const FIELD: &str = include_str!("field.wgsl");
const BUILD: &str = include_str!("field_build.wgsl");
const QUERY_SOURCE: &str = include_str!("field_query.wgsl");
const ENTRIES: [&str; 13] = [
    "clear", "widest", "carrier", "opens", "order", "own", "inherit", "wood", "leaves", "leaflets",
    "count", "fill", "sorted",
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
const SORTED: usize = 12;
/// A node's work words (`field.wgsl`'s `WORDS`), and the box's six after
/// every node's.
const WORDS: u64 = 7;
/// The primitives a bucket holds on average, and the most buckets a grid
/// takes: fewer, fuller buckets walk faster than many near-empty ones
/// under a cell's reach (the spruce's 36-cell query 84 ms at 4, 62 at 16,
/// 66 at 64), and the grid's words stay small.
const PER_BUCKET: f64 = 16.0;
const MOST_BUCKETS: f64 = (1u32 << 22) as f64;
/// An answer's flag past `field.wgsl`'s tally of limb systems in one cell.
const OVER: u32 = 4;

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
            build: io::Pass::new(gpu, build, &[true, true, false, false, false, false], &ENTRIES),
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

/// Three empty buffers for the bindings a pass does not read, apart, as
/// WebGPU binds a written buffer once.
fn unbound(gpu: &Gpu) -> Result<[Owned; 3]> {
    let one = || storage(gpu, "field unbound", 4, &[]);
    Ok([one()?, one()?, one()?])
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
/// station segment's leaves), their limb systems, and the grid of buckets
/// they are listed in.
pub struct DeviceField {
    prims: Owned,
    owner: Owned,
    index: Owned,
    cfg: Cfg,
    /// The field's box: its least and its most corner, in metres.
    pub bounds: [f32; 6],
    /// The wood's primitives (the drawn nodes) and the leaves'.
    pub wood: u32,
    pub leaves: u32,
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

/// Jumps enough for a path of `nodes` nodes: each doubles the reach.
fn jumps(nodes: u32) -> u32 {
    32 - nodes.max(1).leading_zeros() + 1
}

/// A float from its ordered bits (`field.wgsl`'s `ord`).
fn unord(w: u32) -> f32 {
    f32::from_bits(if w & 0x8000_0000 != 0 { w & 0x7fff_ffff } else { !w })
}

/// `family` grown on the device and its field made there, its limb
/// systems opened by laterals of `limb_order` or less (the family's
/// `clumpSystemOrder` where none is given).
pub(super) async fn grow(
    gpu: &Gpu,
    (gen, kernels, tools): (&Generator, &Kernels, &Tools),
    family: &Family,
    limb_order: Option<u32>,
) -> Result<DeviceField> {
    let _capped = gpu.ledger.capped(gen.device_budget());
    let growth = executor::growth(family)?;
    placeable(family)?;
    let mut t = Timed::default();
    let clock = super::super::Clock::now();
    let drawn = tree::drawn(gpu, kernels, &growth).await?;
    let at_drawn = clock.elapsed_ms();
    let Readied { x, swept, ids, .. } = readied(gpu, tools, family, drawn, &mut t).await?;
    let section = x.section()?;
    let rows = station_rows(&x, section.segments, None, growth.year(1.0));
    let device = tools.stationer.stations(gpu, &swept, ids, rows).await?;
    // A frond's leaflets stand as primitives of their own.
    let fronds = rows.rosette[0][0] > 0.0;
    let at_stations = clock.elapsed_ms();
    let [seat, rachis, blade] = x.leaf_reach();
    let canopy = x.leaves().canopy;
    let mut cfg = Cfg {
        nodes: ids,
        root: 0,
        most: limb_order.unwrap_or(canopy.clump_system_order),
        segs: device.segments,
        reach: [seat, rachis, blade, canopy.rachis_length].map(|v| v as f32),
        bound: [
            x.bearing_bounds()[0] as f32,
            canopy.size_variation as f32,
            canopy.scatter.to_radians() as f32,
            f32::from_bits(x.leaves().seed),
        ],
        ..Cfg::default()
    };
    (cfg.leaf_centre, cfg.leaf_half) = leaf_box(x.element(), canopy.size);
    let f = &tools.fielder.build;
    let mut r = Rec {
        gpu,
        held: Vec::new(),
    };
    let leaflets = if fronds { u64::from(device.stations) } else { 0 };
    let prims = u64::from(ids) + u64::from(device.segments) + leaflets;
    let mut lows = vec![0u32; (WORDS * u64::from(ids)) as usize];
    lows.extend([u32::MAX; 3]);
    lows.extend([0u32; 3]);
    let work = storage(gpu, "field work", lows.len() as u64, &lows)?;
    let prim = storage(gpu, "field primitives", prims * 8, &[])?;
    let owner = storage(gpu, "field systems", prims, &[])?;
    let none = unbound(gpu)?;
    let seg = &*device.buffer;
    let mut e = gpu.encoder(&Default::default());
    let bufs: [&wgpu::Buffer; 6] = [&swept.nodes, seg, &work, &prim, &owner, &none[0]];
    for entry in [CLEAR, WIDEST, CARRIER, OPENS] {
        r.run(&mut e, (f, entry, ids, &cfg), &bufs)?;
    }
    for j in 0..jumps(ids) {
        cfg.flip = j % 2;
        r.run(&mut e, (f, ORDER, ids, &cfg), &bufs)?;
    }
    cfg.flip = jumps(ids) % 2;
    r.run(&mut e, (f, OWN, ids, &cfg), &bufs)?;
    for j in 0..jumps(ids) {
        cfg.flip = j % 2;
        r.run(&mut e, (f, INHERIT, ids, &cfg), &bufs)?;
    }
    cfg.flip = jumps(ids) % 2;
    r.run(&mut e, (f, WOOD, ids, &cfg), &bufs)?;
    r.run(&mut e, (f, LEAVES, device.segments, &cfg), &bufs)?;
    if fronds {
        r.run(&mut e, (f, LEAFLETS, device.segments, &cfg), &bufs)?;
    }
    gpu.submit([e.finish()]);
    let at = WORDS * u64::from(ids);
    #[cfg(test)]
    if std::env::var("FIELD_DEBUG").is_ok() {
        debug(gpu, (&swept.nodes, &work), &cfg).await?;
    }
    let corners = read_words(gpu, &work, at, 6).await?;
    let bounds: [f32; 6] = std::array::from_fn(|k| unord(corners[k]));
    drop((swept, device, work));
    if !(bounds[0] <= bounds[3] && bounds.iter().all(|v| v.is_finite())) {
        return Err(Error::InvalidInput("the device's field is empty").into());
    }
    grid(&mut cfg, &bounds, prims);
    cfg.dims[3] = prims as u32;
    let at_made = clock.elapsed_ms();
    let index = index(gpu, (&mut r, tools), &cfg, (&prim, &owner)).await?;
    drop(none);
    if std::env::var("CYCLE_TRACE").is_ok() {
        eprintln!(
            "TRACE field: drawn {at_drawn:.1} ms, readied {t:?}, stations at {at_stations:.1}, made at {at_made:.1}, indexed at {:.1}; {} buckets",
            clock.elapsed_ms(),
            cfg.buckets
        );
    }
    Ok(DeviceField {
        prims: prim,
        owner,
        index,
        cfg,
        bounds,
        wood: ids,
        leaves: (prims - u64::from(ids)) as u32,
    })
}

/// A frond leaflet's box in its own frame (`fronds.rs`'s `Shape`): its
/// centre with the leaf's size, and its half extents.
fn leaf_box(element: &telperion_core::foliage::Element, size: f64) -> ([f32; 4], [f32; 4]) {
    let p = &element.positions;
    let first = p.first().copied().unwrap_or(telperion_core::math::Vec3::ZERO);
    let (lo, hi) = p.iter().fold((first, first), |(lo, hi), v| {
        (
            telperion_core::math::Vec3::new(lo.x.min(v.x), lo.y.min(v.y), lo.z.min(v.z)),
            telperion_core::math::Vec3::new(hi.x.max(v.x), hi.y.max(v.y), hi.z.max(v.z)),
        )
    });
    let (c, h) = ((lo + hi) * 0.5, (hi - lo) * 0.5);
    (
        [c.x, c.y, c.z, size].map(|v| v as f32),
        [h.x, h.y, h.z, 0.0].map(|v| v as f32),
    )
}

/// The bucket grid over the field's box: about `PER_BUCKET` primitives
/// a bucket, at most `MOST_BUCKETS`.
fn grid(cfg: &mut Cfg, b: &[f32; 6], prims: u64) {
    let extent: [f64; 3] = std::array::from_fn(|k| f64::from(b[k + 3] - b[k]).max(1e-3));
    let volume: f64 = extent.iter().product();
    let buckets = (prims as f64 / PER_BUCKET).clamp(1.0, MOST_BUCKETS);
    let mut side = (volume / buckets).cbrt();
    let dims = |side: f64| extent.map(|e| ((e / side).ceil() as u32).max(1));
    while dims(side).iter().map(|&d| f64::from(d)).product::<f64>() > MOST_BUCKETS {
        side *= 1.05;
    }
    let d = dims(side);
    cfg.dims = [d[0], d[1], d[2], 0];
    cfg.buckets = d[0] * d[1] * d[2];
    cfg.lo = [b[0], b[1], b[2], side as f32];
}

/// Each primitive listed in the buckets its box meets: counted, scanned
/// into each bucket's first place, filled and put in order. The index's
/// words: the buckets' firsts and the total, their fills, the lists.
async fn index(
    gpu: &Gpu,
    (r, tools): (&mut Rec<'_>, &Tools),
    cfg: &Cfg,
    (prim, owner): (&Owned, &Owned),
) -> Result<Owned> {
    let b = cfg.buckets;
    let n = unbound(gpu)?;
    let counts = storage(gpu, "field bucket counts", u64::from(b) + 1, &[])?;
    let mut e = gpu.encoder(&Default::default());
    let prims = cfg.dims[3];
    let f = &tools.fielder.build;
    let on = [&*n[0], &n[1], &n[2], prim, owner, &counts];
    r.run(&mut e, (f, COUNT, prims, cfg), &on)?;
    let total = storage(gpu, "field bucket total", 4, &[])?;
    tools.stationer.scan(gpu, &mut e, (&counts, b, 1, 1), &total)?;
    gpu.submit([e.finish()]);
    let listed = read_words(gpu, &total, 0, 1).await?[0];
    let words = 2 * u64::from(b) + 1 + u64::from(listed);
    let index = storage(gpu, "field index", words, &[])?;
    let mut e = gpu.encoder(&Default::default());
    e.copy_buffer_to_buffer(&counts, 0, &index, 0, u64::from(b) * 4);
    e.copy_buffer_to_buffer(&total, 0, &index, u64::from(b) * 4, 4);
    let on = [&*n[0], &n[1], &n[2], prim, owner, &index];
    for (entry, items) in [(FILL, prims), (SORTED, b)] {
        r.run(&mut e, (f, entry, items, cfg), &on)?;
    }
    gpu.submit([e.finish()]);
    io::complete(gpu).await?;
    Ok(index)
}

impl DeviceField {
    /// The field's answers for `cells`, four words each: a closed cube's
    /// centre and half extent, in metres. Only the answers are read back.
    pub(super) async fn query(
        &self,
        gpu: &Gpu,
        tools: &Tools,
        cells: &[f32],
    ) -> Result<Vec<Answer>> {
        if cells.len() % 4 != 0
            || cells.chunks(4).any(|c| !c.iter().all(|v| v.is_finite()) || c[3] < 0.0)
        {
            return Err(Error::InvalidInput("field query").into());
        }
        let n = (cells.len() / 4) as u32;
        if n == 0 {
            return Ok(Vec::new());
        }
        let bits: Vec<u32> = cells.iter().map(|v| v.to_bits()).collect();
        let at = storage(gpu, "field cells", bits.len() as u64, &bits)?;
        let answers = storage(gpu, "field answers", 4 * u64::from(n), &[])?;
        let mut r = Rec {
            gpu,
            held: Vec::new(),
        };
        let mut e = gpu.encoder(&Default::default());
        let bufs = [&*self.prims, &*self.owner, &*self.index, &*at, &*answers];
        r.run(&mut e, (&tools.fielder.query, 0, n, &self.cfg), &bufs)?;
        gpu.submit([e.finish()]);
        let raw = io::read_async(gpu, &answers, 16 * u64::from(n)).await?;
        let words: &[u32] = bytemuck::cast_slice(&raw);
        let mut out = Vec::with_capacity(n as usize);
        for w in words.chunks(4) {
            if w[0] & OVER != 0 {
                return Err(Error::ResourceLimit("a field cell meets more than 128 limb systems").into());
            }
            out.push(Answer {
                wood: w[0] & 1 != 0,
                wood_radius: f32::from_bits(w[1]),
                foliage: w[0] & 2 != 0,
                leaves: f32::from_bits(w[2]),
                limb: (w[3] != u32::MAX).then_some(w[3]),
            });
        }
        Ok(out)
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

    /// The leaves the field's segments carry, read back: the tests'.
    #[cfg(test)]
    pub(super) fn leaf_total(&self, gpu: &Gpu) -> f64 {
        let at = 8 * u64::from(self.wood);
        let n = 8 * u64::from(self.leaves).max(1);
        let w = pollster::block_on(read_words(gpu, &self.prims, at, n)).unwrap();
        w.chunks(8).map(|p| f64::from(f32::from_bits(p[7]))).sum()
    }

    /// The device bytes the field holds.
    pub fn bytes(&self) -> u64 {
        self.prims.size() + self.owner.size() + self.index.size()
    }
}

/// The systems recomputed on the host from the swept nodes, against the
/// device's: a test's trace (`FIELD_DEBUG`).
#[cfg(test)]
async fn debug(gpu: &Gpu, (nodes, work): (&wgpu::Buffer, &wgpu::Buffer), cfg: &Cfg) -> Result<()> {
    const NW: usize = 39;
    let n = cfg.nodes as usize;
    let words = read_words(gpu, nodes, 0, (n * NW) as u64).await?;
    let w = read_words(gpu, work, 0, WORDS * n as u64).await?;
    let f = |i: usize, k: usize| f32::from_bits(words[i * NW + k]);
    let u = |i: usize, k: usize| words[i * NW + k];
    let made = |i: usize| i == 0 || u(i, 9) & 2 != 0;
    let parent = |i: usize| u(i, 8) as usize;
    let structural = |i: usize| (i == 0 || f(i, 3) >= cfg.bound[0]) && u(i, 9) & 0x4000_0000 == 0;
    let mut carrier = vec![u32::MAX; n];
    for i in 1..n {
        let p = parent(i);
        if p >= n || !made(i) || !structural(i) || f(i, 5) != 0.0 {
            continue;
        }
        let best = carrier[p];
        if best == u32::MAX || f(i, 4) > f(best as usize, 4) {
            carrier[p] = i as u32;
        }
    }
    // Parents before children: walk from the root.
    let mut kids = vec![Vec::new(); n];
    for i in 1..n {
        if made(i) && parent(i) < n {
            kids[parent(i)].push(i);
        }
    }
    let (mut order, mut system) = (vec![0u32; n], vec![0u32; n]);
    let mut stack = vec![0usize];
    let mut opened = 0;
    while let Some(p) = stack.pop() {
        for &i in &kids[p] {
            let stem = p == 0;
            let sibling = f(i, 5) != 0.0;
            let opens = structural(i) && (stem || carrier[p] != i as u32);
            order[i] = order[p] + u32::from(opens && !stem && !sibling);
            system[i] = if opens && order[i] <= cfg.most { opened += 1; i as u32 } else { system[p] };
            stack.push(i);
        }
    }
    let half = (jumps(cfg.nodes) % 2) as usize;
    let dev_order = |i: usize| w[(5 + half) * n + i];
    let made_n = (0..n).filter(|&i| made(i)).count();
    let order_diff = (0..n).filter(|&i| made(i) && dev_order(i) != order[i]).count();
    let carrier_diff = (0..n).filter(|&i| made(i) && w[n + i] != carrier[i] && carrier[i] != u32::MAX).count();
    let most_order = order.iter().max().copied().unwrap_or(0);
    let stems = (1..n).filter(|&i| made(i) && parent(i) == 0).count();
    let siblings = (1..n).filter(|&i| made(i) && f(i, 5) != 0.0).count();
    let unparented = (1..n).filter(|&i| made(i) && parent(i) >= n).count();
    let multi = (0..n).filter(|&p| kids[p].len() > 1).count();
    let mut hist = [0usize; 12];
    for i in 0..n { if made(i) { hist[(order[i] as usize).min(11)] += 1; } }
    eprintln!("FIELD_DEBUG stems {stems}, siblings {siblings}, unparented {unparented}, parents of more than one {multi}, orders {hist:?}");
    eprintln!("FIELD_DEBUG nodes {n}, made {made_n}, host systems {opened}, host deepest order {most_order}, order differs at {order_diff}, carrier differs at {carrier_diff}, most {}", cfg.most);
    Ok(())
}
