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
const PACK: &str = include_str!("retire_pack.wgsl");
const UNITS: &str = include_str!("retire_units.wgsl");

pub(super) const ENTRIES: [&str; 7] = ["open_main", "close_main", "woken_main", "living_main", "verdict_main", "end_main", "inside_main"];
pub(super) const OPEN: usize = 0;
pub(super) const CLOSE: usize = 1;
pub(super) const WOKEN: usize = 2;
pub(super) const LIVING: usize = 3;
pub(super) const VERDICT: usize = 4;
pub(super) const END: usize = 5;
pub(super) const INSIDE: usize = 6;
/// `ret`'s columns a laid axis (`retire_cols.wgsl`'s `R_`), and the
/// header's words before the PA constants.
pub(super) const R_COLS: u64 = 14;
const HEAD: u64 = 64;
/// The log's words before its entries, and its entries' words.
pub(super) const L_AT: u64 = 4;
pub(super) const ENTRY: u64 = 5;

/// The modules, built under one arithmetic: retirement's, and the
/// compaction's two (the arena's rows, then the unit records).
pub(super) struct Kernels {
    pub pass: io::Pass,
    pub pack: io::Pass,
    pub units: io::Pass,
}

pub(super) const FLAG: usize = 0;
pub(super) const SCATTER: usize = 1;

impl Kernels {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let reads = [true, true, true, true, true, true, false, false];
        let pack = [true, true, true, true, false, false, false, false];
        Self {
            pass: io::Pass::new(gpu, source(), &reads, &ENTRIES),
            pack: io::Pass::new(gpu, PACK.into(), &pack, &["flag_main", "scatter_main"]),
            units: io::Pass::new(gpu, UNITS.into(), &[true, true, true, false, false], &["units_main"]),
        }
    }
}

#[cfg(test)]
/// The compaction's two modules' WGSL, for the shader checks.
pub(super) fn compaction_sources() -> [&'static str; 2] {
    [PACK, UNITS]
}

/// The module's WGSL: the contract, the reals, then retirement's columns,
/// per-axis passes and sweep.
pub(super) fn source() -> String {
    format!("{}\n{REAL}\n{COLS}\n{AXIS}\n{SWEEP}", super::contract_with(super::ARITH))
}

/// Retirement's buffers for `s` laid axes at most and `pas` PAs: its
/// columns and header (`ret`), and its verdicts' log.
pub(super) struct Retire {
    pub ret: Owned,
    pub verdicts: Owned,
    pub stride: u32,
    pub pa: Vec<u32>,
    /// The retired rows' grown columns, where a PA keeps them: the
    /// records, their stride and how many are held.
    pub recs: Owned,
    pub rec_cap: u32,
    pub rec_n: u32,
}

impl Retire {
    pub(super) fn new(gpu: &Gpu, s: u32, pa: &[u32]) -> Result<Self> {
        let r = Self {
            ret: zeroed_dst(gpu, "retire columns", Self::ret_words(s, pa))?,
            verdicts: zeroed_dst(gpu, "retire verdicts", L_AT + ENTRY * u64::from(Self::log_cap(s)))?,
            stride: s,
            pa: pa.to_vec(),
            recs: zeroed_dst(gpu, "retire records", 4)?,
            rec_cap: 0,
            rec_n: 0,
        };
        gpu.queue.write_buffer(&r.ret, 4 * (r.head() + HEAD), bytemuck::cast_slice(pa));
        Ok(r)
    }

    fn ret_words(s: u32, pa: &[u32]) -> u64 {
        R_COLS * u64::from(s) + HEAD + pa.len() as u64
    }

    /// Where the header stands in `ret`.
    pub(super) fn head(&self) -> u64 {
        R_COLS * u64::from(self.stride)
    }

    /// The most verdicts the log holds.
    pub(super) fn log_cap(s: u32) -> u32 {
        4 * s
    }

    /// The buffers at `s` rows a column, the columns, header and log
    /// copied into `e`.
    pub(super) fn moved(self, gpu: &Gpu, e: &mut wgpu::CommandEncoder, s: u32) -> Result<Retire> {
        if s == self.stride {
            return Ok(self);
        }
        let mut new = Retire::new(gpu, s, &self.pa)?;
        let (a, b) = (4 * u64::from(self.stride), 4 * u64::from(s));
        let head = 4 * self.head();
        for k in 0..R_COLS {
            e.copy_buffer_to_buffer(&self.ret, k * a, &new.ret, k * b, a);
        }
        e.copy_buffer_to_buffer(&self.ret, head, &new.ret, 4 * new.head(), 4 * HEAD);
        e.copy_buffer_to_buffer(&self.verdicts, 0, &new.verdicts, 0, self.verdicts.size());
        (new.recs, new.rec_cap, new.rec_n) = (self.recs, self.rec_cap, self.rec_n);
        Ok(new)
    }

    #[cfg(test)]
    pub(super) fn bytes(&self) -> u64 {
        self.ret.size() + self.verdicts.size() + self.recs.size()
    }

    /// Whether any PA keeps the retired rows (shed pipes or leaf girth).
    pub(super) fn keeps(&self) -> bool {
        self.pa.chunks(16).any(|r| r[7] & 6 != 0)
    }
}

/// A buffer the build copies into as well as reads back.
pub(super) fn zeroed_dst(gpu: &Gpu, label: &'static str, words: u64) -> Result<Owned> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    io::buffer(gpu, label, words.max(4) * 4, usage, &[])
}
