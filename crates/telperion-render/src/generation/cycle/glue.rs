//! What moves between the phases over the one resident state
//! (`glue.wgsl`): the kernels, and one dispatch recorded with its words.
use super::super::io;
use crate::Gpu;

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
    pub(super) pass: io::Pass,
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
