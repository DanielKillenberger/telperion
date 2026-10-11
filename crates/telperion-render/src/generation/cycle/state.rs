//! The cycle's one resident state (DESIGN.md section 1), test-only:
//! every phase reads and writes these buffers, each a set of columns of
//! one word a row, `cap` rows apart, an index the CPU's.
//!
//! - **axes:** the units' columns (`arena.wgsl`'s `A_`, then its head),
//!   then the layout's (`lay_state.wgsl`): whorl, blend and birth draws as
//!   the f32s it reads, phase, rank, base, heading, side and faults.
//! - **units:** the units' records (`U_`).
//! - **grown:** the phytomers' columns the units write: key, axis, cycle,
//!   node presence.
//! - **laid:** the phytomers' columns the layout writes: tip, heading,
//!   side, scale, grown, size, rank and leaf area.
//! - **pencils:** a pencil an axis (`layout.wgsl`'s `P_`), its light,
//!   size, memory and kept share last.
//! - **lists:** the units' tally, the sleeping buds, the living apexes
//!   the cycle leaves, the woken buds and the cycle's waking list.
//!
//! The binding count WebGPU allows a stage (8) keeps the columns in these
//! few buffers rather than one buffer a column; each column is still
//! contiguous. The phytomers' columns are two buffers so that neither
//! passes the adapter's largest storage binding (2 GiB on this device).
use super::super::io;
use crate::buffer::Owned;
use crate::{Gpu, Result};

/// The columns of each table.
pub(super) const AXES: u64 = 51;
pub(super) const UNITS: u64 = 13;
pub(super) const GROWN: u64 = 5;
pub(super) const LAID: u64 = 14;
pub(super) const PENCILS: u64 = 59;
pub(super) const SLEEPERS: u64 = 14;
pub(super) const WOKEN: u64 = 5;
/// The units' tally at the head of `lists`.
pub(super) const TALLY: u64 = 64;

/// The axes' columns the build reads by name.
pub(super) const A_HEAD: u64 = 34;
pub(super) const A_LAYOUT: u64 = 35;
/// A unit record's links and phytomers.
pub(super) const U_NEXT: u64 = 10;
pub(super) const U_FIRST: u64 = 11;
pub(super) const U_COUNT: u64 = 12;

/// How many rows each table holds: axes (and pencils), units,
/// phytomers, sleeping buds, living apexes after a cycle and woken buds.
/// The axes' rows are a multiple of 64, so the hierarchy's columns at
/// their stride bind at 256 bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Caps {
    pub axes: u32,
    pub units: u32,
    pub phyts: u32,
    pub sleepers: u32,
    pub next: u32,
    pub woken: u32,
}

impl Caps {
    pub(super) fn sleep_at(&self) -> u32 {
        TALLY as u32
    }

    pub(super) fn next_at(&self) -> u32 {
        self.sleep_at() + (SLEEPERS * u64::from(self.sleepers)) as u32
    }

    pub(super) fn woken_at(&self) -> u32 {
        self.next_at() + 2 * self.next
    }

    pub(super) fn list_at(&self) -> u32 {
        self.woken_at() + (WOKEN * u64::from(self.woken)) as u32
    }

    fn list_words(&self) -> u64 {
        u64::from(self.list_at()) + u64::from(self.sleepers)
    }
}

/// What the state holds: the rows in use of each table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(super) struct Held {
    pub axes: u32,
    pub units: u32,
    pub phyts: u32,
    pub sleepers: u32,
    pub next: u32,
    pub woken: u32,
}

/// The resident buffers.
pub(super) struct State {
    pub caps: Caps,
    pub axes: Owned,
    pub units: Owned,
    pub grown: Owned,
    pub laid: Owned,
    pub pencils: Owned,
    pub lists: Owned,
}

fn zeroed(gpu: &Gpu, label: &'static str, words: u64) -> Result<Owned> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    io::buffer(gpu, label, words.max(4) * 4, usage, &[])
}

impl State {
    pub(super) fn new(gpu: &Gpu, caps: Caps) -> Result<Self> {
        let w = Self::words(caps);
        Ok(Self {
            caps,
            axes: zeroed(gpu, "cycle state axes", w[0])?,
            pencils: zeroed(gpu, "cycle state pencils", w[1])?,
            units: zeroed(gpu, "cycle state units", w[2])?,
            grown: zeroed(gpu, "cycle state phytomers grown", w[3])?,
            laid: zeroed(gpu, "cycle state phytomers laid", w[4])?,
            lists: zeroed(gpu, "cycle state lists", w[5])?,
        })
    }

    /// The words of each buffer at `caps`: axes, pencils, units, the
    /// phytomers' grown and laid columns, the lists.
    pub(super) fn words(caps: Caps) -> [u64; 6] {
        let (a, p) = (u64::from(caps.axes), u64::from(caps.phyts));
        [AXES * a, PENCILS * a, UNITS * u64::from(caps.units), GROWN * p, LAID * p, caps.list_words()]
    }

    /// The bytes the state holds.
    pub(super) fn bytes(&self) -> u64 {
        [&self.axes, &self.units, &self.grown, &self.laid, &self.pencils, &self.lists]
            .iter()
            .map(|b| b.size())
            .sum()
    }
}
