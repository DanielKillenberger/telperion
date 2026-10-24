//! fn-256 task 2 (DESIGN.md section 6, as decided for the build):
//! retirement's kernels and buffers, test-only. One module marks the open
//! axes, flags and assigns the closed ones, sizes the woken buds by their
//! bearers' kept shares, carries living up the hierarchy's levels and
//! judges each candidate, its verdicts logged.
use super::super::io;
use crate::buffer::Owned;
use crate::{Gpu, Result};

const COLS: &str = include_str!("retire_cols.wgsl");
const AXIS: &str = include_str!("retire_axis.wgsl");
const SWEEP: &str = include_str!("retire_sweep.wgsl");
const REAL: &str = include_str!("real.wgsl");
const COMPACT: &str = include_str!("compact.wgsl");
const UNITS: &str = include_str!("retire_units.wgsl");

pub(super) const ENTRIES: [&str; 12] = [
    "open_main",
    "close_main",
    "woken_main",
    "living_main",
    "verdict_main",
    "end_main",
    "inside_main",
    "living_fold_main",
    "inside_fold_main",
    "reused_main",
    "terms_main",
    "fading_main",
];
pub(super) const OPEN: usize = 0;
pub(super) const CLOSE: usize = 1;
pub(super) const WOKEN: usize = 2;
pub(super) const LIVING: usize = 3;
pub(super) const VERDICT: usize = 4;
pub(super) const END: usize = 5;
pub(super) const INSIDE: usize = 6;
/// Each sweep's narrow levels folded into one workgroup (`fold.wgsl`).
pub(super) const FOLDS: [usize; 2] = [7, 8];
/// A measurement's count of what the cycle reused (`Tables::counted`).
pub(super) const REUSED: usize = 9;
/// Each child's part in the living its closing bearer carries up.
pub(super) const TERMS: usize = 10;
/// Each kept lateral retired once its fade reaches none (fn-270 R3).
pub(super) const FADING: usize = 11;
/// `ret`'s columns a laid axis (`retire_cols.wgsl`'s `R_`), and the
/// header's words before the PA constants.
pub(super) const R_COLS: u64 = 13;
/// The cycle's own columns a laid axis (`retire_cols.wgsl`'s `C_`), in
/// `verdicts` past the blocks' stamps: let go between calls (fn-270 R4).
pub(super) const C_COLS: u64 = 10;
const HEAD: u64 = 64;
/// The log's count of what the cycle reused, its words; the log's words
/// before its entries, and its entries' words. The blocks' stamps follow
/// the entries.
pub(super) const L_REUSE: u64 = 2;
pub(super) const REUSE_WORDS: u64 = 3;
pub(super) const L_AT: u64 = 8;
pub(super) const ENTRY: u64 = 6;

/// The modules, built under one arithmetic: retirement's, and the
/// compaction's two (the arena in place, `compact.rs`; the unit records'
/// first phytomers).
pub(super) struct Kernels {
    pub pass: io::Pass,
    pub compact: io::Pass,
    pub units: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let reads = [true, true, true, true, true, true, false, false];
        Self {
            pass: io::Pass::new(gpu, source(), &reads, &ENTRIES),
            compact: io::Pass::new(
                gpu,
                COMPACT.into(),
                &[false, false, true, false],
                &super::compact::ENTRIES,
            ),
            units: io::Pass::new(
                gpu,
                UNITS.into(),
                &[true, true, true, false, false],
                &["units_main"],
            ),
        }
    }
}

#[cfg(test)]
/// The compaction's two modules' WGSL, for the shader checks.
pub(super) fn compaction_sources() -> [&'static str; 2] {
    [COMPACT, UNITS]
}

/// The module's WGSL: the contract, the reals, then retirement's columns,
/// per-axis passes and sweep.
pub(super) fn source() -> String {
    format!(
        "{}\n{REAL}\n{COLS}\n{AXIS}\n{SWEEP}\n{}",
        super::contract_with(super::ARITH),
        super::FOLD
    )
}

/// Retirement's buffers for `s` laid axes at most and `pas` PAs: its
/// columns and header (`ret`), and its verdicts' log.
pub(super) struct Retire {
    pub ret: Owned,
    pub verdicts: Owned,
    pub stride: u32,
    pub pa: Vec<u32>,
    /// The verdicts the log holds at most: none unless a test asks
    /// (`Tables::log`), as nothing in a build reads it.
    pub log_cap: u32,
}

impl Retire {
    /// Retirement for `s` laid axes.
    pub(super) fn new(gpu: &Gpu, s: u32, pa: &[u32], log: bool) -> Result<Self> {
        let log_cap = if log { 4 * s } else { 0 };
        let r = Self {
            ret: zeroed_dst(gpu, "retire columns", Self::ret_words(s, pa))?,
            verdicts: zeroed_dst(gpu, "retire verdicts", Self::verdict_words(log_cap, s))?,
            stride: s,
            pa: pa.to_vec(),
            log_cap,
        };
        gpu.queue
            .write_buffer(&r.ret, 4 * (r.head() + HEAD), bytemuck::cast_slice(pa));
        Ok(r)
    }

    /// The log's words for `log_cap` entries, then the stamps and the
    /// cycle's columns of a cycle over `s` laid axes (none while paused).
    pub(super) fn verdict_words(log_cap: u32, s: u32) -> u64 {
        L_AT + ENTRY * u64::from(log_cap) + (1 + C_COLS) * u64::from(s)
    }

    fn ret_words(s: u32, pa: &[u32]) -> u64 {
        R_COLS * u64::from(s) + HEAD + pa.len() as u64
    }

    /// Where the header stands in `ret`.
    pub(super) fn head(&self) -> u64 {
        R_COLS * u64::from(self.stride)
    }

    /// The buffers at `s` rows a column, the columns, header and log
    /// copied into `e`.
    pub(super) fn moved(self, gpu: &Gpu, e: &mut wgpu::CommandEncoder, s: u32) -> Result<Retire> {
        if s == self.stride {
            return Ok(self);
        }
        let new = Retire::new(gpu, s, &self.pa, self.log_cap > 0)?;
        let (a, b) = (4 * u64::from(self.stride), 4 * u64::from(s));
        let head = 4 * self.head();
        for k in 0..R_COLS {
            e.copy_buffer_to_buffer(&self.ret, k * a, &new.ret, k * b, a);
        }
        e.copy_buffer_to_buffer(&self.ret, head, &new.ret, 4 * new.head(), 4 * HEAD);
        e.copy_buffer_to_buffer(
            &self.verdicts,
            0,
            &new.verdicts,
            0,
            self.verdicts.size().min(new.verdicts.size()),
        );
        Ok(new)
    }

    #[cfg(test)]
    pub(super) fn bytes(&self) -> u64 {
        self.ret.size() + self.verdicts.size()
    }
}

/// A buffer the build copies into as well as reads back.
pub(super) fn zeroed_dst(gpu: &Gpu, label: &'static str, words: u64) -> Result<Owned> {
    let usage =
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    io::buffer(gpu, label, words.max(4) * 4, usage, &[])
}
