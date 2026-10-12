//! The hierarchy kept on the device as the tree grows (DESIGN.md sections
//! 3 and 4), test-only: each step takes in the axes laid since the last,
//! counts them, scans, places and merges (`hier_build.wgsl`,
//! `hier_blocks.wgsl`, `hier_members.wgsl`). A step's generations are read
//! back after its depth pass and its waiting axes after each placing pass
//! (DESIGN.md section 5's small reads), and its blocks per level at its
//! end, so its dispatches and the sweeps' are sized.
use super::super::io;
#[cfg(test)]
use super::hier_plan::Plan;
#[cfg(test)]
use super::hier_scan::read_words;
use super::hier_scan::{sums_words, zeroed};
use crate::buffer::Owned;
use crate::{Gpu, Result};

/// The build's entries.
pub(super) const ENTRIES: [&str; 17] = [
    "begin_main",
    "depth_main",
    "kid_place_main",
    "kid_merge_main",
    "target_main",
    "join_place_main",
    "decide_main",
    "assign_main",
    "follow_main",
    "round_end_main",
    "member_count_main",
    "member_place_main",
    "member_rank_main",
    "member_old_main",
    "member_new_main",
    "member_start_main",
    "level_merge_main",
];

/// The shape's, counts', lists' and buckets' columns, and the tally's
/// words (`hier_cols.wgsl`, `hier_build.wgsl`).
pub(super) const SHAPE: u64 = 6;
pub(super) const COUNTS: u64 = 12;
pub(super) const LISTS: u64 = 6;
const BUCKETS: u64 = 5;
const TALLY: u64 = 384;
#[cfg(test)]
pub(super) const X_BLOCKS: usize = 0;
pub(super) const X_LEVELS: usize = 1;
pub(super) const X_GENS: u64 = 2;
pub(super) const X_FAULT: usize = 5;
pub(super) const X_UNPLACED: u64 = 6;
pub(super) const R_LVL_START: u64 = 4;
/// The partition's defaults: the most axes a block holds, and the lanes
/// a sweep's workgroup has (HIERARCHY.md's grid). Speed only: no
/// partition changes a word.
pub(super) const CAP: u32 = 128;
pub(super) const WIDTH: u32 = 64;
#[cfg(test)]
/// The most placing passes a generation may take.
const PASSES: u32 = 64;

/// The build's, the sums' and the scan's modules.
pub(super) struct Kernels {
    pub build: io::Pass,
    pub sums: io::Pass,
    pub scan: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu) -> Self {
        Self::with_width(gpu, WIDTH)
    }

    /// The kernels, the sweeps' workgroups `width` lanes wide.
    pub(super) fn with_width(gpu: &Gpu, width: u32) -> Self {
        let build = [true, false, false, true, false, false, false];
        let sums = [true, true, true, true, false, false, false, false];
        let entries = &super::hier_device::ENTRIES;
        Self {
            build: io::Pass::new(gpu, super::hier_build_source(), &build, &ENTRIES),
            sums: io::Pass::new(gpu, super::hier_cycle_source_at(width), &sums, entries),
            scan: io::Pass::new(
                gpu,
                super::hier_scan_source(),
                &[false; 3],
                &["scan_up", "scan_down"],
            ),
        }
    }
}

/// The hierarchy as built so far: its state on the device and what the
/// host knows of it.
#[cfg_attr(not(test), allow(dead_code))]
pub(super) struct Hierarchy {
    pub stride: u32,
    pub cap: u32,
    pub shape: Owned,
    pub counts: Owned,
    pub lists: [Owned; 2],
    pub bucket: Owned,
    pub tally: Owned,
    pub sums: Owned,
    /// Which of `lists` is current.
    pub parity: usize,
    pub built: u32,
    pub blocks: u32,
    pub levels: u32,
    /// Each level's blocks, from the root's.
    pub level_sizes: Vec<u32>,
    /// Each step's placing passes, per generation.
    pub passes: Vec<Vec<u32>>,
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
            sums: zeroed(gpu, "hierarchy scan sums", sums_words(stride + 64))?,
            parity: 0,
            built: 0,
            blocks: 0,
            levels: 0,
            level_sizes: Vec::new(),
            passes: Vec::new(),
        })
    }

    /// The step taking in the axes up to `n_new`: its depth pass and its
    /// generations read, its children, each generation placed pass by
    /// pass until none waits, its members and levels, its blocks read.
    #[cfg(test)]
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
        plan.children(k, &mut e);
        gpu.submit([e.finish()]);
        let mut passes = Vec::new();
        for round in 0..gens as usize {
            let mut taken = 0;
            loop {
                let mut e = gpu.encoder(&Default::default());
                plan.pass(self, k, &mut e, round);
                gpu.submit([e.finish()]);
                taken += 1;
                // cap: a generation's passes are bounded by `PASSES`.
                if read_words(gpu, &self.tally, X_UNPLACED, 1)?[0] == 0 || taken == PASSES {
                    break;
                }
            }
            assert!(taken < PASSES, "a generation that never placed");
            passes.push(taken);
        }
        let mut e = gpu.encoder(&Default::default());
        plan.members(self, k, &mut e);
        gpu.submit([e.finish()]);
        self.passes.push(passes);
        self.ended(gpu, n_new)
    }

    /// After a step: the lists swapped, its blocks and levels read.
    #[cfg(test)]
    pub(super) fn ended(&mut self, gpu: &Gpu, n_new: u32) -> Result<()> {
        self.parity = 1 - self.parity;
        self.built = n_new;
        let t = read_words(gpu, &self.tally, 0, 8)?;
        assert_eq!(t[X_FAULT], 0, "a hierarchy fault");
        (self.blocks, self.levels) = (t[X_BLOCKS], t[X_LEVELS]);
        let at = R_LVL_START * (u64::from(self.stride) + 64);
        let n = u64::from(self.levels) + 1;
        let starts = read_words(gpu, &self.lists[self.parity], at, n)?;
        self.level_sizes = starts.windows(2).map(|w| w[1] - w[0]).collect();
        Ok(())
    }

    #[cfg(test)]
    /// The bytes of the build's scratch: counts, buckets, a second list,
    /// the scan's sums and the tally.
    pub(super) fn scratch(&self) -> u64 {
        let lists = self.lists[1].size();
        self.counts.size() + self.bucket.size() + lists + self.sums.size() + self.tally.size()
    }
}
