//! What moves between the phases over the one resident state
//! (`glue.wgsl`): the kernels, and one dispatch recorded with its words.
use super::super::io;
use super::state::State;
use crate::buffer::Owned;
use crate::{Gpu, Result};

const GLUE: &str = include_str!("glue.wgsl");

pub(super) const TREE: usize = 0;
pub(super) const SCATTER: usize = 1;
pub(super) const INPUTS: usize = 2;
pub(super) const BORN: usize = 3;
pub(super) const GREW_MARK: usize = 4;
pub(super) const GREW_GATHER: usize = 5;
pub(super) const POINTS: usize = 6;
pub(super) const LIT: usize = 7;
pub(super) const RELIVE: usize = 8;
pub(super) const FRESH: usize = 9;
const ENTRIES: [&str; 10] = [
    "tree_main",
    "scatter_main",
    "inputs_main",
    "born_main",
    "grew_mark_main",
    "grew_gather_main",
    "points_main",
    "lit_main",
    "relive_main",
    "fresh_main",
];

/// The glue's WGSL: the software f64, then its entries.
pub(super) fn source() -> String {
    format!("{}\n{GLUE}", super::SOFT64)
}

pub(super) struct Kernels {
    pass: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu) -> Self {
        Self {
            pass: io::Pass::new(gpu, source(), &[false; 7], &ENTRIES),
        }
    }
}

/// A glue dispatch's own words (`glue.wgsl`'s `Cfg`); the rest are the
/// state's.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Words {
    pub n: u32,
    pub cycle: u32,
    pub laid: u32,
    pub live_stride: u32,
    pub first: u32,
    pub leaves: u32,
    pub from_in: bool,
    pub live: u32,
}

/// The invocations a dispatch row holds for `n` invocations, and its
/// workgroups.
pub(super) fn row(gpu: &Gpu, n: u32) -> (u32, u32) {
    let groups = n.div_ceil(64).max(1);
    let most = gpu.device.limits().max_compute_workgroups_per_dimension;
    (groups, groups.min(most) * 64)
}

/// One glue dispatch over `w.n` invocations (`w.laid` for the entries
/// over the laid axes), `aux` and `aux2` bound beside the state.
#[allow(clippy::too_many_arguments)]
pub(super) fn run(
    gpu: &Gpu,
    k: &Kernels,
    e: &mut wgpu::CommandEncoder,
    (state, live): (&State, &Owned),
    (aux, aux2): (&Owned, &Owned),
    entry: usize,
    w: Words,
) -> Result<()> {
    let c = state.caps;
    let (groups, row) = row(gpu, w.n);
    let words = [
        w.n,
        row,
        w.cycle,
        w.laid,
        c.axes,
        c.units,
        c.phyts,
        c.phyt_at(),
        w.live_stride,
        c.next_at(),
        c.next,
        w.first,
        w.leaves,
        u32::from(w.from_in),
        w.live,
        0,
    ];
    let usage = wgpu::BufferUsages::UNIFORM;
    let cfg = io::buffer(gpu, "glue cfg", 64, usage, bytemuck::cast_slice(&words))?;
    let all = [
        &*cfg,
        &state.axes,
        &state.pencils,
        &state.arena,
        &state.lists,
        live,
        aux,
        aux2,
    ];
    let bind = k.pass.bind(gpu, &all);
    k.pass.run(e, &bind, entry, groups);
    Ok(())
}
