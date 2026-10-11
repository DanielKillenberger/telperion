//! One cycle's balance and allotment recorded on the device over the
//! hierarchy (DESIGN.md section 2, steps 1 and 2), test-only: the living
//! apexes marked, the totals by block level from the deepest, the walk
//! from the root's, the shed apexes compacted out, the presences, the
//! subtrees' lit presence up and the split down, the normaliser's blocks
//! and the sizes. Each dispatch is its own stamped pass; the living
//! after the balance reach their kernels through the scan's total.
use super::super::io;
use super::hier_build::{Hierarchy, Kernels};
use super::hier_columns::Hier;
use super::hier_scan::{Scan, sums_words, uniform, widest, zeroed};
use crate::buffer::Owned;
use crate::{Gpu, Result};

pub(super) const ENTRIES: [&str; 11] = [
    "mark_main",
    "shed_main",
    "compact_main",
    "presence_main",
    "partials_main",
    "combine_main",
    "sizes_main",
    "totals_main",
    "walk_main",
    "sub_main",
    "split_main",
];
pub(super) const MARK: usize = 0;
pub(super) const SHED: usize = 1;
pub(super) const COMPACT: usize = 2;
pub(super) const PRESENCE: usize = 3;
pub(super) const PARTIALS: usize = 4;
pub(super) const COMBINE: usize = 5;
pub(super) const SIZES: usize = 6;
pub(super) const TOTALS: usize = 7;
pub(super) const WALK: usize = 8;
pub(super) const SUB: usize = 9;
pub(super) const SPLIT: usize = 10;

/// The sums' columns (`hier_cycle.wgsl`'s `S_` words), and those the
/// cycle starts from the tree's (memory, kept, size) or from none.
pub(super) const SUMS: u64 = 19;
pub(super) const S_LIGHT: u64 = 1;
pub(super) const S_COST: u64 = 2;
pub(super) const S_LIVING: u64 = 3;
pub(super) const S_MEMORY: u64 = 6;
pub(super) const S_KEPT: u64 = 7;
pub(super) const S_SIZE: u64 = 8;
pub(super) const S_SUB: u64 = 12;
pub(super) const S_INCOMING: u64 = 14;
pub(super) const S_BUD: u64 = 15;
pub(super) const S_GONE: u64 = 18;
pub(super) const FROM_TREE: [(u64, u64); 3] = [(12, S_MEMORY), (11, S_KEPT), (13, S_SIZE)];
pub(super) const CLEARED: [u64; 7] = [0, 9, 10, 11, 14, 15, 18];
/// The living's columns, and where the living after start.
pub(super) const LIVE: u64 = 7;
pub(super) const V_FLAG: u64 = 3;
pub(super) const V_OUT: u64 = 4;
/// The normaliser's words before the order, and the tally's.
const N_ORDER: u64 = 128;
const TALLY: u64 = 128;
pub(super) const BUD_BLOCK: u32 = 1024;

/// The balance and allotment's buffers on the device, and the plan of
/// one cycle's dispatches over them.
pub(super) struct Sums {
    pub sums: Owned,
    pub live: Owned,
    pub norm: Owned,
    pub tally: Owned,
    pub rows: Owned,
    pub scan_sums: Owned,
    pub live_stride: u32,
    stride: u32,
    live_n: u32,
    /// The bind of each level, the first serving the per-apex passes.
    binds: Vec<wgpu::BindGroup>,
    level_sizes: Vec<u32>,
    flags: Option<Scan>,
    _held: Vec<Owned>,
}

/// What one cycle's plan reads: the laid axes, the cycle, the PAs and
/// the living apexes.
#[derive(Debug, Clone, Copy)]
pub(super) struct Counts {
    pub n: u32,
    pub cycle: u32,
    pub pas: u32,
    pub live: u32,
}

impl Sums {
    /// A captured cycle's sums: its living and PA rows uploaded, planned.
    pub(super) fn new(
        gpu: &Gpu,
        k: &Kernels,
        hier: &Hier,
        h: &Hierarchy,
        tree: &Owned,
    ) -> Result<Self> {
        let live_n = hier.live();
        let mut sums = Self::alloc(gpu, hier.stride, live_n, hier.pas)?;
        let live_stride = sums.live_stride;
        let mut cols = vec![0u32; (LIVE * u64::from(live_stride)) as usize];
        cols[..live_n as usize].copy_from_slice(&hier.live_axis);
        for (k, t) in hier.live_spent.iter().enumerate() {
            cols[live_stride as usize + k] = *t as u32;
            cols[2 * live_stride as usize + k] = (*t >> 32) as u32;
        }
        gpu.queue.write_buffer(&sums.live, 0, bytemuck::cast_slice(&cols));
        gpu.queue.write_buffer(&sums.rows, 0, bytemuck::cast_slice(&hier.rows));
        let counts = Counts { n: hier.n, cycle: hier.cycle, pas: hier.pas, live: live_n };
        sums.plan(gpu, k, h, tree, counts)?;
        Ok(sums)
    }

    /// The buffers for a tree of `stride` axes' columns, at most `live`
    /// living apexes and `pas` PAs.
    pub(super) fn alloc(gpu: &Gpu, stride: u32, live: u32, pas: u32) -> Result<Self> {
        let (s, live_stride) = (u64::from(stride), super::hier_columns::stride(live));
        let blocks = u64::from(live.div_ceil(BUD_BLOCK).max(1));
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let rows = io::buffer(gpu, "hierarchy rows", 4 * 8 * u64::from(pas.max(1)), usage, &[])?;
        Ok(Self {
            sums: zeroed(gpu, "hierarchy sums", SUMS * s)?,
            live: zeroed(gpu, "hierarchy living", LIVE * u64::from(live_stride))?,
            norm: zeroed(gpu, "hierarchy normaliser", N_ORDER + s + 2 * blocks * u64::from(pas))?,
            tally: zeroed(gpu, "hierarchy sums tally", TALLY)?,
            rows,
            scan_sums: zeroed(gpu, "hierarchy sums scan", sums_words(live))?,
            live_stride,
            stride,
            live_n: 0,
            binds: Vec::new(),
            level_sizes: Vec::new(),
            flags: None,
            _held: Vec::new(),
        })
    }

    /// One cycle's dispatches planned over the hierarchy as `h` holds it.
    pub(super) fn plan(&mut self, gpu: &Gpu, k: &Kernels, h: &Hierarchy, tree: &Owned, c: Counts) -> Result<()> {
        let live_stride = self.live_stride;
        self.flags = Some(Scan::new(
            gpu,
            &k.scan,
            (&self.live, V_FLAG * u64::from(live_stride), c.live),
            &self.scan_sums,
            (&self.tally, 0),
        )?);
        let cur = &h.lists[h.parity];
        let (mut binds, mut held) = (Vec::new(), Vec::new());
        for level in 0..h.levels.max(1) {
            let words = [
                c.n,
                self.stride,
                c.live,
                live_stride,
                c.cycle,
                c.pas,
                level,
                widest(gpu).0,
                widest(gpu).1,
                0,
                0,
                0,
            ];
            let cfg = uniform(gpu, &words)?;
            let all = [
                &*cfg, tree, &self.rows, cur, &h.shape, &self.sums, &self.live, &self.norm, &self.tally,
            ];
            binds.push(k.sums.bind(gpu, &all));
            held.push(cfg);
        }
        (self.binds, self._held, self.live_n) = (binds, held, c.live);
        self.level_sizes = h.level_sizes.clone();
        Ok(())
    }

    /// The cycle's columns set as it starts them: memory, kept and size
    /// from the tree, the marks and the allotment's none, the tally none.
    fn reset(&self, tree: &Owned, e: &mut wgpu::CommandEncoder) {
        let col = 4 * u64::from(self.stride);
        for (from, to) in FROM_TREE {
            e.copy_buffer_to_buffer(tree, from * col, &self.sums, to * col, col);
        }
        for c in CLEARED {
            e.clear_buffer(&self.sums, c * col, Some(col));
        }
        e.clear_buffer(&self.tally, 0, None);
    }

    /// The balance and the allotment recorded into `e`.
    pub(super) fn record(&self, k: &Kernels, tree: &Owned, e: &mut wgpu::CommandEncoder) {
        self.reset(tree, e);
        let (base, apexes) = (&self.binds[0], self.live_n.div_ceil(64));
        let levels = self.level_sizes.len();
        k.sums.run(e, base, MARK, apexes);
        for l in (0..levels).rev() {
            k.sums.run(e, &self.binds[l], TOTALS, self.level_sizes[l]);
        }
        for l in 0..levels {
            k.sums.run(e, &self.binds[l], WALK, self.level_sizes[l]);
        }
        k.sums.run(e, base, SHED, apexes);
        self.flags.as_ref().expect("planned").record(&k.scan, e);
        k.sums.run(e, base, COMPACT, apexes);
        k.sums.run(e, base, PRESENCE, apexes);
        for l in (0..levels).rev() {
            k.sums.run(e, &self.binds[l], SUB, self.level_sizes[l]);
        }
        for l in 0..levels {
            k.sums.run(e, &self.binds[l], SPLIT, self.level_sizes[l]);
        }
        k.sums
            .run(e, base, PARTIALS, self.live_n.div_ceil(BUD_BLOCK).max(1));
        k.sums.run(e, base, COMBINE, 1);
        k.sums.run(e, base, SIZES, apexes);
    }

    /// The dispatches `record` makes.
    pub(super) fn dispatches(&self) -> usize {
        4 * self.level_sizes.len() + 7 + self.flags.as_ref().map_or(0, Scan::dispatches)
    }

    /// The bytes the sums hold beyond the tree: their columns, the living,
    /// the normaliser and the tally.
    pub(super) fn scratch(&self) -> u64 {
        let held = [&self.sums, &self.live, &self.norm, &self.tally, &self.scan_sums];
        held.iter().map(|b| b.size()).sum()
    }
}
