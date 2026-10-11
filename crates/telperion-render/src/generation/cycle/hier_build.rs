//! The hierarchy kept on the device as the tree grows (DESIGN.md sections
//! 3 and 4), test-only: each step takes in the axes laid since the last,
//! counts them, scans, places and merges, in `hier_build.wgsl` and
//! `hier_blocks.wgsl`. A step's generations are read back after its
//! depth pass (DESIGN.md section 5's small read), and its blocks per
//! level at its end, so the sweeps' dispatches are sized.
use super::super::io;
use crate::buffer::Owned;
use crate::{Gpu, Result};

const SCAN: &str = include_str!("../fine/scan.wgsl");

/// The build's entries.
const ENTRIES: [&str; 13] = [
    "begin_main",
    "depth_main",
    "kid_place_main",
    "kid_merge_main",
    "join_count_main",
    "join_place_main",
    "decide_main",
    "assign_main",
    "round_end_main",
    "member_count_main",
    "member_place_main",
    "member_merge_main",
    "level_merge_main",
];
const BEGIN: usize = 0;
const DEPTH: usize = 1;
const KID_PLACE: usize = 2;
const KID_MERGE: usize = 3;
const JOIN_COUNT: usize = 4;
const JOIN_PLACE: usize = 5;
const DECIDE: usize = 6;
const ASSIGN: usize = 7;
const ROUND_END: usize = 8;
const MEMBER_COUNT: usize = 9;
const MEMBER_PLACE: usize = 10;
const MEMBER_MERGE: usize = 11;
const LEVEL_MERGE: usize = 12;

/// The shape's, counts', lists' and buckets' columns, and the tally's
/// words (`hier_cols.wgsl`, `hier_build.wgsl`).
pub(super) const SHAPE: u64 = 5;
const COUNTS: u64 = 9;
pub(super) const LISTS: u64 = 6;
const BUCKETS: u64 = 3;
const TALLY: u64 = 384;
const C_KIDS: u64 = 0;
const C_KID_FILL: u64 = 1;
const C_JOIN: u64 = 2;
const C_JOIN_FILL: u64 = 3;
const C_OPEN: u64 = 4;
const C_MEMBERS: u64 = 6;
const C_MEMBER_FILL: u64 = 7;
const C_LEVEL: u64 = 8;
const X_BLOCKS: usize = 0;
const X_LEVELS: usize = 1;
const X_GENS: u64 = 2;
const X_FAULT: usize = 5;
const X_KIDS: u64 = 64;
const X_JOIN: u64 = 128;
const X_OPEN: u64 = 192;
const X_MEMBERS: u64 = 256;
const X_LEVEL: u64 = 320;
pub(super) const R_LVL_START: u64 = 4;

/// The build's, the sums' and the scan's modules.
pub(super) struct Kernels {
    pub build: io::Pass,
    pub sums: io::Pass,
    pub scan: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let build = [true, false, false, true, false, false, false];
        let sums = [true, true, true, true, false, false, false, false];
        Self {
            build: io::Pass::new(gpu, super::hier_build_source(), &build, &ENTRIES),
            sums: io::Pass::new(
                gpu,
                super::hier_cycle_source(),
                &sums,
                &super::hier_device::ENTRIES,
            ),
            scan: io::Pass::new(
                gpu,
                SCAN.into(),
                &[false; 3],
                &["blocks", "spine", "spread"],
            ),
        }
    }
}

pub(super) fn zeroed(gpu: &Gpu, label: &'static str, words: u64) -> Result<Owned> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    io::buffer(gpu, label, words.max(4) * 4, usage, &[])
}

pub(super) fn uniform(gpu: &Gpu, words: &[u32]) -> Result<Owned> {
    let bytes = (words.len() as u64 * 4).next_multiple_of(16);
    io::buffer(
        gpu,
        "hierarchy cfg",
        bytes,
        wgpu::BufferUsages::UNIFORM,
        bytemuck::cast_slice(words),
    )
}

/// The invocations a dispatch row holds, and the workgroups.
pub(super) fn widest(gpu: &Gpu) -> (u32, u32) {
    let most = gpu.device.limits().max_compute_workgroups_per_dimension;
    (most * 64, most)
}

/// `n` words of `b` from word `at`, through a copy.
pub(super) fn read_words(gpu: &Gpu, b: &wgpu::Buffer, at: u64, n: u64) -> Result<Vec<u32>> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let to = io::buffer(gpu, "hierarchy read", n.max(1) * 4, usage, &[])?;
    let mut e = gpu.encoder(&Default::default());
    e.copy_buffer_to_buffer(b, at * 4, &to, 0, n * 4);
    gpu.submit([e.finish()]);
    let raw = io::read(gpu, &to, n * 4)?;
    Ok(bytemuck::cast_slice(&raw).to_vec())
}

/// A bind of `size` bytes of `buffer` from `offset`.
fn slice(binding: u32, buffer: &wgpu::Buffer, offset: u64, size: u64) -> wgpu::BindGroupEntry<'_> {
    wgpu::BindGroupEntry {
        binding,
        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
            buffer,
            offset,
            size: std::num::NonZeroU64::new(size),
        }),
    }
}

/// One exclusive scan of `rows` words of a buffer from word `at`, its
/// total to a tally's word `to`.
pub(super) struct Scan {
    bind: wgpu::BindGroup,
    rows: u32,
    _cfg: Owned,
}

impl Scan {
    pub(super) fn new(
        gpu: &Gpu,
        k: &Kernels,
        (data, at, rows): (&Owned, u64, u32),
        sums: &Owned,
        (tally, to): (&Owned, u64),
    ) -> Result<Self> {
        let cfg = uniform(
            gpu,
            &[rows, 1, 1, rows.div_ceil(256), widest(gpu).0, 0, 0, 0],
        )?;
        let entries = [
            slice(0, &cfg, 0, cfg.size()),
            slice(1, data, at * 4, u64::from(rows.max(1)) * 4),
            slice(2, sums, 0, sums.size()),
            slice(3, tally, to * 4, 256),
        ];
        let bind = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("hierarchy scan"),
            layout: &k.scan.layout,
            entries: &entries,
        });
        Ok(Self {
            bind,
            rows,
            _cfg: cfg,
        })
    }

    pub(super) fn record(&self, k: &Kernels, e: &mut wgpu::CommandEncoder) {
        k.scan
            .run(e, &self.bind, 0, self.rows.div_ceil(256).div_ceil(64));
        k.scan.run(e, &self.bind, 1, 1);
        k.scan.run(e, &self.bind, 2, self.rows.div_ceil(64));
    }
}

/// The hierarchy as built so far: its state on the device and what the
/// host knows of it.
pub(super) struct Hierarchy {
    pub stride: u32,
    pub cap: u32,
    pub shape: Owned,
    pub counts: Owned,
    pub lists: [Owned; 2],
    pub bucket: Owned,
    pub tally: Owned,
    sums: Owned,
    /// Which of `lists` is current.
    pub parity: usize,
    pub built: u32,
    pub blocks: u32,
    pub levels: u32,
    /// Each level's blocks, from the root's.
    pub level_sizes: Vec<u32>,
    /// Each step's generations.
    pub gens: Vec<u32>,
}

impl Hierarchy {
    pub(super) fn new(gpu: &Gpu, stride: u32, cap: u32) -> Result<Self> {
        let s = u64::from(stride);
        Ok(Self {
            stride,
            cap,
            shape: zeroed(gpu, "hierarchy shape", SHAPE * s)?,
            counts: zeroed(gpu, "hierarchy counts", COUNTS * s)?,
            lists: [
                zeroed(gpu, "hierarchy lists a", LISTS * (s + 64))?,
                zeroed(gpu, "hierarchy lists b", LISTS * (s + 64))?,
            ],
            bucket: zeroed(gpu, "hierarchy buckets", BUCKETS * s)?,
            tally: zeroed(gpu, "hierarchy tally", TALLY)?,
            sums: zeroed(gpu, "hierarchy scan sums", 4 * s.div_ceil(256))?,
            parity: 0,
            built: 0,
            blocks: 0,
            levels: 0,
            level_sizes: Vec::new(),
            gens: Vec::new(),
        })
    }

    /// The step taking in the axes up to `n_new`: its depth pass, its
    /// generations read, the rest, then its blocks per level read.
    pub(super) fn grow(&mut self, gpu: &Gpu, k: &Kernels, tree: &Owned, n_new: u32) -> Result<()> {
        if n_new <= self.built {
            return Ok(());
        }
        let head = Plan::new(gpu, k, self, tree, (n_new, 0))?;
        let mut e = gpu.encoder(&Default::default());
        head.head(self, k, &mut e);
        gpu.submit([e.finish()]);
        let gens = read_words(gpu, &self.tally, X_GENS, 1)?[0];
        let plan = Plan::new(gpu, k, self, tree, (n_new, gens))?;
        let mut e = gpu.encoder(&Default::default());
        plan.tail(self, k, &mut e);
        gpu.submit([e.finish()]);
        self.gens.push(gens);
        self.ended(gpu, n_new)
    }

    /// After a step: the lists swapped, its blocks and levels read.
    pub(super) fn ended(&mut self, gpu: &Gpu, n_new: u32) -> Result<()> {
        self.parity = 1 - self.parity;
        self.built = n_new;
        let t = read_words(gpu, &self.tally, 0, 8)?;
        assert_eq!(t[X_FAULT], 0, "a hierarchy fault");
        (self.blocks, self.levels) = (t[X_BLOCKS], t[X_LEVELS]);
        let at = R_LVL_START * (u64::from(self.stride) + 64);
        let starts = read_words(
            gpu,
            &self.lists[self.parity],
            at,
            u64::from(self.levels) + 1,
        )?;
        self.level_sizes = starts.windows(2).map(|w| w[1] - w[0]).collect();
        Ok(())
    }

    /// The bytes of the build's scratch: counts, buckets, a second list
    /// and the scan's sums.
    pub(super) fn scratch(&self) -> u64 {
        self.counts.size()
            + self.bucket.size()
            + self.lists[1].size()
            + self.sums.size()
            + self.tally.size()
    }
}

/// One step's uniforms, binds and scans: `(n_new, gens)` from the
/// hierarchy as it stands.
pub(super) struct Plan {
    n_old: u32,
    new: u32,
    n_new: u32,
    block_rows: u32,
    level_rows: u32,
    /// A bind per round (the first serves the rest of the step).
    binds: Vec<wgpu::BindGroup>,
    scans: [Scan; 5],
    _cfgs: Vec<Owned>,
}

impl Plan {
    pub(super) fn new(
        gpu: &Gpu,
        k: &Kernels,
        h: &Hierarchy,
        tree: &Owned,
        (n_new, gens): (u32, u32),
    ) -> Result<Self> {
        let n_old = h.built;
        let new = n_new - n_old;
        let block_rows = h.blocks + new;
        let level_rows = h.levels + gens;
        let (cur, next) = (&h.lists[h.parity], &h.lists[1 - h.parity]);
        let (mut binds, mut cfgs) = (Vec::new(), Vec::new());
        for round in 0..gens.max(1) {
            let words = [
                n_old,
                n_new,
                h.cap,
                round,
                h.stride,
                widest(gpu).0,
                block_rows,
                level_rows,
            ];
            let cfg = uniform(gpu, &words)?;
            let all = [
                &*cfg, tree, &h.shape, &h.counts, cur, next, &h.bucket, &h.tally,
            ];
            binds.push(k.build.bind(gpu, &all));
            cfgs.push(cfg);
        }
        let s = u64::from(h.stride);
        let scan = |col: u64, rows: u32, to: u64| {
            Scan::new(gpu, k, (&h.counts, col * s, rows), &h.sums, (&h.tally, to))
        };
        let scans = [
            scan(C_KIDS, n_new, X_KIDS)?,
            scan(C_JOIN, block_rows, X_JOIN)?,
            scan(C_OPEN, new, X_OPEN)?,
            scan(C_MEMBERS, block_rows, X_MEMBERS)?,
            scan(C_LEVEL, level_rows, X_LEVEL)?,
        ];
        Ok(Self {
            n_old,
            new,
            n_new,
            block_rows,
            level_rows,
            binds,
            scans,
            _cfgs: cfgs,
        })
    }

    fn clear(&self, h: &Hierarchy, e: &mut wgpu::CommandEncoder, col: u64, rows: u32) {
        let at = col * u64::from(h.stride) * 4;
        e.clear_buffer(&h.counts, at, Some(u64::from(rows.max(1)) * 4));
    }

    /// The step's start: its depths and generations, its children counted.
    pub(super) fn head(&self, h: &Hierarchy, k: &Kernels, e: &mut wgpu::CommandEncoder) {
        self.clear(h, e, C_KIDS, self.n_new);
        self.clear(h, e, C_KID_FILL, self.n_new);
        e.clear_buffer(&h.tally, X_GENS * 4, Some(4));
        let b = &self.binds[0];
        k.build.run(e, b, BEGIN, 1);
        k.build.run(e, b, DEPTH, self.new.div_ceil(64));
    }

    /// The rest: the children merged, the blocks placed round by round,
    /// the members and levels merged.
    pub(super) fn tail(&self, h: &Hierarchy, k: &Kernels, e: &mut wgpu::CommandEncoder) {
        let (b, groups) = (&self.binds[0], self.new.div_ceil(64));
        self.scans[0].record(k, e);
        k.build.run(e, b, KID_PLACE, groups);
        k.build.run(e, b, KID_MERGE, self.n_new.div_ceil(64));
        for round in &self.binds {
            self.clear(h, e, C_JOIN, self.block_rows);
            self.clear(h, e, C_JOIN_FILL, self.block_rows);
            k.build.run(e, round, JOIN_COUNT, groups);
            self.scans[1].record(k, e);
            k.build.run(e, round, JOIN_PLACE, groups);
            k.build.run(e, round, DECIDE, groups);
            self.scans[2].record(k, e);
            k.build.run(e, round, ASSIGN, groups);
            k.build.run(e, round, ROUND_END, 1);
        }
        self.clear(h, e, C_MEMBERS, self.block_rows);
        self.clear(h, e, C_MEMBER_FILL, self.block_rows);
        self.clear(h, e, C_LEVEL, self.level_rows);
        k.build.run(e, b, MEMBER_COUNT, groups);
        self.scans[3].record(k, e);
        self.scans[4].record(k, e);
        k.build.run(e, b, MEMBER_PLACE, groups);
        k.build
            .run(e, b, MEMBER_MERGE, self.block_rows.div_ceil(64));
        k.build.run(e, b, LEVEL_MERGE, self.level_rows);
    }

    /// The dispatches the step records.
    pub(super) fn dispatches(&self) -> usize {
        2 + 3 + 2 + self.binds.len() * 11 + 1 + 6 + 3
    }

    pub(super) fn n_old(&self) -> u32 {
        self.n_old
    }
}
