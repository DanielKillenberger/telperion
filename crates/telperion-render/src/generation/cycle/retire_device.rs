//! fn-256 task 2 (DESIGN.md section 6, as decided for the build):
//! retirement's kernels and buffers, test-only. One module marks the open
//! axes, flags and assigns the closed ones, sizes the woken buds by their
//! bearers' kept shares, carries living up the hierarchy's levels and
//! judges each candidate, its verdicts logged.
use super::super::io;
use super::Arith;
use crate::buffer::Owned;
use crate::{Gpu, Result};

const COLS: &str = include_str!("retire_cols.wgsl");
const AXIS: &str = include_str!("retire_axis.wgsl");
const SWEEP: &str = include_str!("retire_sweep.wgsl");
const REAL: &str = include_str!("real.wgsl");

pub(super) const ENTRIES: [&str; 6] = ["open_main", "close_main", "woken_main", "living_main", "verdict_main", "end_main"];
pub(super) const OPEN: usize = 0;
pub(super) const CLOSE: usize = 1;
pub(super) const WOKEN: usize = 2;
pub(super) const LIVING: usize = 3;
pub(super) const VERDICT: usize = 4;
pub(super) const END: usize = 5;
/// `ret`'s columns a laid axis (`retire_cols.wgsl`'s `R_`), the fate's
/// and a retired root's living's, and the header's words before the PA
/// constants.
pub(super) const R_COLS: u64 = 14;
pub(super) const R_FATE: u64 = 1;
pub(super) const R_ROOT_LIVED: u64 = 12;
const HEAD: u64 = 64;
/// The log's words before its entries, and its entries' words.
pub(super) const L_AT: u64 = 4;
pub(super) const ENTRY: u64 = 5;

/// The module, built under one arithmetic.
pub(super) struct Kernels {
    pub pass: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu, arith: Arith) -> Self {
        let reads = [true, true, true, true, true, true, false, false];
        Self { pass: io::Pass::new(gpu, source(arith), &reads, &ENTRIES) }
    }
}

/// The module's WGSL: the contract, the reals, then retirement's columns,
/// per-axis passes and sweep.
pub(super) fn source(arith: Arith) -> String {
    format!("{}\n{REAL}\n{COLS}\n{AXIS}\n{SWEEP}", super::super::fine::contract_with(arith.marks()))
}

/// Retirement's buffers for `s` laid axes at most and `pas` PAs: its
/// columns and header (`ret`), and its verdicts' log.
pub(super) struct Retire {
    pub ret: Owned,
    pub verdicts: Owned,
    pub stride: u32,
    pub pa: Vec<u32>,
}

impl Retire {
    pub(super) fn new(gpu: &Gpu, s: u32, pa: &[u32]) -> Result<Self> {
        let r = Self {
            ret: zeroed_dst(gpu, "retire columns", Self::ret_words(s, pa))?,
            verdicts: zeroed_dst(gpu, "retire verdicts", L_AT + ENTRY * u64::from(Self::log_cap(s)))?,
            stride: s,
            pa: pa.to_vec(),
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
        let new = Retire::new(gpu, s, &self.pa)?;
        let (a, b) = (4 * u64::from(self.stride), 4 * u64::from(s));
        for k in 0..R_COLS {
            e.copy_buffer_to_buffer(&self.ret, k * a, &new.ret, k * b, a);
        }
        e.copy_buffer_to_buffer(&self.ret, 4 * self.head(), &new.ret, 4 * new.head(), 4 * HEAD);
        e.copy_buffer_to_buffer(&self.verdicts, 0, &new.verdicts, 0, self.verdicts.size());
        Ok(new)
    }

    pub(super) fn bytes(&self) -> u64 {
        self.ret.size() + self.verdicts.size()
    }
}

/// A buffer the build copies into as well as reads back.
fn zeroed_dst(gpu: &Gpu, label: &'static str, words: u64) -> Result<Owned> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    io::buffer(gpu, label, words.max(4) * 4, usage, &[])
}
