//! The cycle's rough layout recorded on the device (DESIGN.md section
//! 2.3) over the one resident state: the grown axes laid, each relay set
//! at its node, the new axes' generations found, then per generation its
//! axes listed by a flag, a scan and a gather, framed and laid. Each
//! dispatch is its own compute pass (`io::Pass::run`), so a profile
//! stamps each; a generation's count moves by a copy from the scan's
//! total into the layout's tally, never to the host.
use super::super::io;
use crate::buffer::Owned;
use crate::{Gpu, Result};

const SCAN: &str = include_str!("../fine/scan.wgsl");

pub(super) const DRAW: usize = 0;
pub(super) const FRAME: usize = 1;
pub(super) const RELAY: usize = 2;
pub(super) const GENERATION: usize = 3;
pub(super) const FLAG: usize = 4;
pub(super) const GATHER: usize = 5;
/// The layout's tally (`layout.wgsl`'s `T_`): the generation listed and
/// the cycle's faults.
pub(super) const T_FAULT: u64 = 8;
/// Each generation reached, marked (`layout.wgsl`'s `T_DEEP`).
pub(super) const T_DEEP: u64 = 16;
pub(super) const DEEP: u32 = 16;
pub(super) const TALLY: u32 = 32;

pub(super) struct Kernels {
    pub(super) layout: io::Pass,
    pub(super) scan: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu, arith: super::Arith) -> Self {
        let mut reads = vec![true];
        reads.extend([false; 5]);
        let entries = [
            "draw_main",
            "frame_main",
            "relay_main",
            "generation_main",
            "flag_main",
            "gather_main",
        ];
        Self {
            layout: io::Pass::new(gpu, super::layout_source(arith), &reads, &entries),
            scan: io::Pass::new(gpu, SCAN.into(), &[false; 3], &["blocks", "spine", "spread"]),
        }
    }
}

/// The layout's own buffers beside the state: each PA's form, its lists
/// (the tally, the grown axes, the new axes' generations and a
/// generation's list) and its flags.
pub(super) struct Layout {
    pub form: Owned,
    pub lists: Owned,
    pub counts: Owned,
}

fn storage(gpu: &Gpu, label: &'static str, words: u64, init: &[u32]) -> Result<Owned> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    io::buffer(gpu, label, words.max(4) * 4, usage, bytemuck::cast_slice(init))
}

impl Layout {
    pub(super) fn new(gpu: &Gpu, form: &[f32], axes: u32) -> Result<Self> {
        Ok(Self {
            form: storage(gpu, "layout form", form.len() as u64, bytemuck::cast_slice(form))?,
            lists: storage(gpu, "layout lists", u64::from(TALLY + 3 * axes), &[])?,
            counts: storage(gpu, "layout flags", u64::from(axes), &[])?,
        })
    }

    /// The bytes the layout holds beside the state.
    pub(super) fn bytes(&self) -> u64 {
        self.form.size() + self.lists.size() + self.counts.size()
    }
}
