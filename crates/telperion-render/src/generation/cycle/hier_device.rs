//! One cycle's balance and allotment recorded on the device over the
//! hierarchy (DESIGN.md section 2, steps 1 and 2), test-only: the living
//! apexes marked, the totals by block level from the deepest, the walk
//! from the root's, the shed apexes compacted out, the presences, the
//! subtrees' lit presence up and the split down, the normaliser's blocks
//! and the sizes. Each dispatch is its own stamped pass; the living
//! after the balance reach their kernels through the scan's total.
use super::super::io;
use super::hier_build::{Hierarchy, Kernels, Scan, uniform, widest, zeroed};
use super::hier_columns::{Hier, stride};
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
const MARK: usize = 0;
const SHED: usize = 1;
const COMPACT: usize = 2;
const PRESENCE: usize = 3;
const PARTIALS: usize = 4;
const COMBINE: usize = 5;
const SIZES: usize = 6;
const TOTALS: usize = 7;
const WALK: usize = 8;
const SUB: usize = 9;
const SPLIT: usize = 10;

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
const FROM_TREE: [(u64, u64); 3] = [(12, S_MEMORY), (11, S_KEPT), (13, S_SIZE)];
const CLEARED: [u64; 7] = [0, 9, 10, 11, 14, 15, 18];
/// The living's columns, and where the living after start.
pub(super) const LIVE: u64 = 7;
const V_FLAG: u64 = 3;
pub(super) const V_OUT: u64 = 4;
/// The normaliser's words before the order, and the tally's.
const N_ORDER: u64 = 128;
const TALLY: u64 = 128;
const BUD_BLOCK: u32 = 1024;

/// A captured cycle's balance and allotment on the device.
pub(super) struct Sums {
    pub sums: Owned,
    pub live: Owned,
    pub norm: Owned,
    pub tally: Owned,
    pub live_stride: u32,
    stride: u32,
    live_n: u32,
    /// The bind of each level, the first serving the per-apex passes.
    binds: Vec<wgpu::BindGroup>,
    level_sizes: Vec<u32>,
    flags: Scan,
    _held: Vec<Owned>,
}

impl Sums {
    pub(super) fn new(
        gpu: &Gpu,
        k: &Kernels,
        hier: &Hier,
        h: &Hierarchy,
        tree: &Owned,
    ) -> Result<Self> {
        let (s, live_n) = (u64::from(hier.stride), hier.live());
        let live_stride = stride(live_n);
        let mut cols = vec![0u32; (LIVE * u64::from(live_stride)) as usize];
        cols[..live_n as usize].copy_from_slice(&hier.live_axis);
        for (k, t) in hier.live_spent.iter().enumerate() {
            cols[live_stride as usize + k] = *t as u32;
            cols[2 * live_stride as usize + k] = (*t >> 32) as u32;
        }
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let live = io::buffer(
            gpu,
            "hierarchy living",
            cols.len() as u64 * 4,
            usage,
            bytemuck::cast_slice(&cols),
        )?;
        let rows = io::buffer(
            gpu,
            "hierarchy rows",
            hier.rows.len() as u64 * 4,
            usage,
            bytemuck::cast_slice(&hier.rows),
        )?;
        let sums = zeroed(gpu, "hierarchy sums", SUMS * s)?;
        let blocks = u64::from(live_n.div_ceil(BUD_BLOCK).max(1));
        let norm = zeroed(
            gpu,
            "hierarchy normaliser",
            N_ORDER + s + 2 * blocks * u64::from(hier.pas),
        )?;
        let tally = zeroed(gpu, "hierarchy sums tally", TALLY)?;
        let scan_sums = zeroed(
            gpu,
            "hierarchy sums scan",
            4 * u64::from(live_n.div_ceil(256)),
        )?;
        let flags = Scan::new(
            gpu,
            k,
            (&live, V_FLAG * u64::from(live_stride), live_n),
            &scan_sums,
            (&tally, 0),
        )?;
        let cur = &h.lists[h.parity];
        let (mut binds, mut held) = (Vec::new(), vec![rows, scan_sums]);
        for level in 0..h.levels.max(1) {
            let words = [
                hier.n,
                hier.stride,
                live_n,
                live_stride,
                hier.cycle,
                hier.pas,
                level,
                widest(gpu).0,
                widest(gpu).1,
                0,
                0,
                0,
            ];
            let cfg = uniform(gpu, &words)?;
            let all = [
                &*cfg, tree, &held[0], cur, &h.shape, &sums, &live, &norm, &tally,
            ];
            binds.push(k.sums.bind(gpu, &all));
            held.push(cfg);
        }
        Ok(Self {
            sums,
            live,
            norm,
            tally,
            live_stride,
            stride: hier.stride,
            live_n,
            binds,
            level_sizes: h.level_sizes.clone(),
            flags,
            _held: held,
        })
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
        self.flags.record(k, e);
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
        4 * self.level_sizes.len() + 10
    }

    /// The bytes the sums hold beyond the tree: their columns, the living,
    /// the normaliser and the tally.
    pub(super) fn scratch(&self) -> u64 {
        self.sums.size() + self.live.size() + self.norm.size() + self.tally.size()
    }
}
