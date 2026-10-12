//! One build step's uniforms, binds and scans, and its dispatches in
//! order: the head (depths), the children, a placing pass of a
//! generation, the members and levels.
#[cfg(test)]
use super::hier_build::{Hierarchy, Kernels, X_GENS, X_UNPLACED};
#[cfg(test)]
use super::hier_scan::{Scan, uniform, widest};
#[cfg(test)]
use crate::buffer::Owned;
#[cfg(test)]
use crate::{Gpu, Result};

pub(super) const BEGIN: usize = 0;
pub(super) const DEPTH: usize = 1;
pub(super) const KID_PLACE: usize = 2;
pub(super) const KID_MERGE: usize = 3;
pub(super) const TARGET: usize = 4;
pub(super) const JOIN_PLACE: usize = 5;
pub(super) const DECIDE: usize = 6;
pub(super) const ASSIGN: usize = 7;
pub(super) const FOLLOW: usize = 8;
pub(super) const ROUND_END: usize = 9;
pub(super) const MEMBER_COUNT: usize = 10;
pub(super) const MEMBER_PLACE: usize = 11;
pub(super) const MEMBER_RANK: usize = 12;
pub(super) const MEMBER_OLD: usize = 13;
pub(super) const MEMBER_NEW: usize = 14;
pub(super) const MEMBER_START: usize = 15;
pub(super) const LEVEL_MERGE: usize = 16;

pub(super) const C_KIDS: u64 = 0;
pub(super) const C_KID_FILL: u64 = 1;
pub(super) const C_JOIN: u64 = 2;
pub(super) const C_JOIN_FILL: u64 = 3;
pub(super) const C_OPEN: u64 = 4;
pub(super) const C_MEMBERS: u64 = 6;
pub(super) const C_MEMBER_FILL: u64 = 7;
pub(super) const C_LEVEL: u64 = 8;
pub(super) const X_KIDS: u64 = 64;
pub(super) const X_JOIN: u64 = 128;
pub(super) const X_OPEN: u64 = 192;
pub(super) const X_MEMBERS: u64 = 256;
pub(super) const X_LEVEL: u64 = 320;

/// One step from the hierarchy as it stands to `n_new` axes, of `gens`
/// generations.
#[cfg(test)]
pub(super) struct Plan {
    n_old: u32,
    new: u32,
    n_new: u32,
    block_rows: u32,
    level_rows: u32,
    stride: u64,
    /// A bind per generation (the first serves the rest of the step).
    binds: Vec<wgpu::BindGroup>,
    scans: [Scan; 5],
    _cfgs: Vec<Owned>,
}

#[cfg(test)]
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
            Scan::new(
                gpu,
                &k.scan,
                (&h.counts, col * s, rows),
                &h.sums,
                (&h.tally, to),
            )
        };
        let scans = [
            scan(C_KIDS, n_new, X_KIDS)?,
            scan(C_JOIN, block_rows, X_JOIN)?,
            scan(C_OPEN, new, X_OPEN)?,
            scan(C_MEMBERS, block_rows, X_MEMBERS)?,
            scan(C_LEVEL, level_rows, X_LEVEL)?,
        ];
        let stride = s;
        Ok(Self {
            n_old,
            new,
            n_new,
            block_rows,
            level_rows,
            stride,
            binds,
            scans,
            _cfgs: cfgs,
        })
    }

    fn clear(&self, h: &Hierarchy, e: &mut wgpu::CommandEncoder, col: u64, rows: u32) {
        e.clear_buffer(
            &h.counts,
            col * self.stride * 4,
            Some(u64::from(rows.max(1)) * 4),
        );
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

    /// The children placed and merged.
    pub(super) fn children(&self, k: &Kernels, e: &mut wgpu::CommandEncoder) {
        let b = &self.binds[0];
        self.scans[0].record(&k.scan, e);
        k.build.run(e, b, KID_PLACE, self.new.div_ceil(64));
        k.build.run(e, b, KID_MERGE, self.n_new.div_ceil(64));
    }

    /// One placing pass of generation `round`.
    pub(super) fn pass(
        &self,
        h: &Hierarchy,
        k: &Kernels,
        e: &mut wgpu::CommandEncoder,
        round: usize,
    ) {
        let (b, groups) = (&self.binds[round], self.new.div_ceil(64));
        for col in [C_JOIN, C_JOIN_FILL] {
            self.clear(h, e, col, self.block_rows);
        }
        e.clear_buffer(&h.tally, X_UNPLACED * 4, Some(4));
        k.build.run(e, b, TARGET, groups);
        self.scans[1].record(&k.scan, e);
        k.build.run(e, b, JOIN_PLACE, groups);
        k.build.run(e, b, DECIDE, groups);
        self.scans[2].record(&k.scan, e);
        k.build.run(e, b, ASSIGN, groups);
        k.build.run(e, b, FOLLOW, groups);
        k.build.run(e, b, ROUND_END, 1);
    }

    /// The members ranked and merged, the levels merged.
    pub(super) fn members(&self, h: &Hierarchy, k: &Kernels, e: &mut wgpu::CommandEncoder) {
        let (b, groups) = (&self.binds[0], self.new.div_ceil(64));
        self.clear(h, e, C_MEMBERS, self.block_rows);
        self.clear(h, e, C_MEMBER_FILL, self.block_rows);
        self.clear(h, e, C_LEVEL, self.level_rows);
        k.build.run(e, b, MEMBER_COUNT, groups);
        self.scans[3].record(&k.scan, e);
        self.scans[4].record(&k.scan, e);
        k.build.run(e, b, MEMBER_PLACE, groups);
        k.build.run(e, b, MEMBER_RANK, groups);
        k.build.run(e, b, MEMBER_OLD, self.n_old.div_ceil(64));
        k.build.run(e, b, MEMBER_NEW, groups);
        k.build
            .run(e, b, MEMBER_START, self.block_rows.div_ceil(64));
        k.build.run(e, b, LEVEL_MERGE, self.level_rows);
    }

    /// The whole step, its passes per generation as a first run found.
    pub(super) fn record(
        &self,
        h: &Hierarchy,
        k: &Kernels,
        e: &mut wgpu::CommandEncoder,
        passes: &[u32],
    ) {
        self.head(h, k, e);
        self.children(k, e);
        for (round, &n) in passes.iter().enumerate() {
            for _ in 0..n {
                self.pass(h, k, e, round);
            }
        }
        self.members(h, k, e);
    }

    /// The dispatches `record` makes.
    pub(super) fn dispatches(&self, passes: &[u32]) -> usize {
        let s = |k: usize| self.scans[k].dispatches();
        let pass = 6 + s(1) + s(2);
        let all: u32 = passes.iter().sum();
        2 + s(0) + 2 + all as usize * pass + 8 + s(3) + s(4)
    }
}
