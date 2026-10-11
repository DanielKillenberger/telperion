//! The cycle's rough layout recorded on the device (DESIGN.md section
//! 2.3) over the one resident state: the grown axes laid, each relay set
//! at its node, the new axes' generations found, then per generation its
//! axes listed by a flag, a scan and a gather, framed and laid. Each
//! dispatch is its own compute pass (`io::Pass::run`), so a profile
//! stamps each; a generation's count moves by a copy from the scan's
//! total into the layout's tally, never to the host.
use super::super::io;
use super::device::{record_scans, row, scan_strided};
use super::state::State;
use crate::buffer::Owned;
use crate::{Gpu, Result};

const SCAN: &str = include_str!("../fine/scan.wgsl");

const DRAW: usize = 0;
const FRAME: usize = 1;
const RELAY: usize = 2;
const GENERATION: usize = 3;
const FLAG: usize = 4;
const GATHER: usize = 5;
/// The layout's tally (`layout.wgsl`'s `T_`): the generation listed and
/// the cycle's faults.
pub(super) const T_FAULT: u64 = 8;
const TALLY: u32 = 16;

pub(super) struct Kernels {
    layout: io::Pass,
    scan: io::Pass,
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
    form: Owned,
    pub lists: Owned,
    counts: Owned,
    cap: u32,
}

/// What a cycle's layout reads: the cycle, the axes laid before it, the
/// axes it grew, and the new axes.
#[derive(Debug, Clone, Copy)]
pub(super) struct Laying {
    pub cycle: u32,
    pub first_new: u32,
    pub grown: u32,
    pub new_axes: u32,
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
            cap: axes,
        })
    }

    /// The lists and flags grown to `axes` rows.
    pub(super) fn ensure(&mut self, gpu: &Gpu, axes: u32) -> Result<()> {
        if axes > self.cap {
            self.lists = storage(gpu, "layout lists", u64::from(TALLY + 3 * axes), &[])?;
            self.counts = storage(gpu, "layout flags", u64::from(axes), &[])?;
            self.cap = axes;
        }
        Ok(())
    }

    /// The bytes the layout holds beside the state.
    pub(super) fn bytes(&self) -> u64 {
        self.form.size() + self.lists.size() + self.counts.size()
    }

    fn bind(&self, gpu: &Gpu, k: &Kernels, state: &State, words: [u32; 16]) -> Result<wgpu::BindGroup> {
        let usage = wgpu::BufferUsages::UNIFORM;
        let c = io::buffer(gpu, "layout cfg", 64, usage, bytemuck::cast_slice(&words))?;
        let all = [&*c, &self.form, &state.arena, &state.axes, &state.pencils, &self.lists, &self.counts];
        Ok(k.layout.bind(gpu, &all))
    }

    /// The grown axes drawn (the glue listed them), each relay set at
    /// its node and each new axis's generation found.
    pub(super) fn record_first(&self, gpu: &Gpu, k: &Kernels, e: &mut wgpu::CommandEncoder, state: &State, l: Laying) -> Result<()> {
        let draw = self.bind(gpu, k, state, config(gpu, state, l, (l.grown, 0, 0)))?;
        k.layout.run(e, &draw, DRAW, row(gpu, l.grown).0);
        if l.new_axes == 0 {
            return Ok(());
        }
        let new = self.bind(gpu, k, state, config(gpu, state, l, (l.new_axes, 0, 0)))?;
        k.layout.run(e, &new, RELAY, row(gpu, l.new_axes).0);
        k.layout.run(e, &new, GENERATION, row(gpu, l.new_axes).0);
        Ok(())
    }

    /// Generation `g`'s new axes listed by a flag, a scan and a gather,
    /// framed and drawn.
    pub(super) fn record_generation(&self, gpu: &Gpu, k: &Kernels, e: &mut wgpu::CommandEncoder, state: &State, (l, g): (Laying, u32)) -> Result<()> {
        let n = l.new_axes;
        let bind = self.bind(gpu, k, state, config(gpu, state, l, (n, 1, g)))?;
        k.layout.run(e, &bind, FLAG, row(gpu, n).0);
        let s = scan_strided(gpu, &k.scan, (&self.counts, n, 1, 1), 0)?;
        record_scans(&k.scan, e, std::slice::from_ref(&s), &self.lists);
        k.layout.run(e, &bind, GATHER, row(gpu, n).0);
        k.layout.run(e, &bind, FRAME, row(gpu, n).0);
        k.layout.run(e, &bind, DRAW, row(gpu, n).0);
        Ok(())
    }
}

/// The layout's uniform (`layout.wgsl`'s `Cfg`) over `n` invocations,
/// drawing the grown axes or generation `generation`'s.
fn config(gpu: &Gpu, state: &State, l: Laying, (n, listed, generation): (u32, u32, u32)) -> [u32; 16] {
    let c = state.caps;
    [
        n,
        row(gpu, n).1,
        l.cycle,
        l.first_new,
        c.axes,
        c.phyts,
        c.units,
        l.new_axes,
        listed,
        generation,
        0,
        l.grown,
        c.phyt_at(),
        0,
        0,
        0,
    ]
}
