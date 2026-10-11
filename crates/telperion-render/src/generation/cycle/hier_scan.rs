//! The hierarchy's buffers and its scan (`hier_scan.wgsl`): an exclusive
//! prefix sum of a column slice in levels of 256 words, up then down,
//! its total to a tally's word.
use super::super::io;
use crate::buffer::Owned;
use crate::{Gpu, Result};

const UP: usize = 0;
const DOWN: usize = 1;
const SPAN: u32 = 256;

pub(super) fn zeroed(gpu: &Gpu, label: &'static str, words: u64) -> Result<Owned> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    io::buffer(gpu, label, words.max(4) * 4, usage, &[])
}

pub(super) fn uniform(gpu: &Gpu, words: &[u32]) -> Result<Owned> {
    let bytes = (words.len() as u64 * 4).next_multiple_of(16);
    let usage = wgpu::BufferUsages::UNIFORM;
    io::buffer(
        gpu,
        "hierarchy cfg",
        bytes,
        usage,
        bytemuck::cast_slice(words),
    )
}

/// The invocations a dispatch row holds, and the workgroups.
pub(super) fn widest(gpu: &Gpu) -> (u32, u32) {
    let most = gpu.device.limits().max_compute_workgroups_per_dimension;
    (most * 64, most)
}

/// `n` words of `b` from word `at`, through a copy.
pub(super) fn read_words(gpu: &Gpu, b: &wgpu::Buffer, at: u64, n: u64) -> Result<Vec<u32>> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let to = io::buffer(gpu, "hierarchy read", n.max(1) * 4, usage, &[])?;
    let mut e = gpu.encoder(&Default::default());
    e.copy_buffer_to_buffer(b, at * 4, &to, 0, n * 4);
    gpu.submit([e.finish()]);
    let raw = io::read(gpu, &to, n * 4)?;
    Ok(bytemuck::cast_slice(&raw).to_vec())
}

/// A bind of `size` bytes of `buffer` from `offset`.
fn slice(binding: u32, buffer: &wgpu::Buffer, offset: u64, size: u64) -> wgpu::BindGroupEntry<'_> {
    wgpu::BindGroupEntry {
        binding,
        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
            buffer,
            offset,
            size: std::num::NonZeroU64::new(size),
        }),
    }
}

/// Each level's words, from the column's.
fn levels(rows: u32) -> Vec<u32> {
    let mut out = vec![rows];
    while *out.last().expect("a level") > SPAN {
        out.push(out.last().expect("a level").div_ceil(SPAN));
    }
    out
}

/// The words the levels above a column of `rows` take in the sums.
pub(super) fn sums_words(rows: u32) -> u64 {
    levels(rows)[1..].iter().map(|&n| u64::from(n)).sum::<u64>() + 4
}

/// One exclusive scan of `rows` words of a buffer from word `at`, its
/// total to a tally's word `to`: a bind and a size per level.
pub(super) struct Scan {
    levels: Vec<(wgpu::BindGroup, u32)>,
    _cfgs: Vec<Owned>,
}

impl Scan {
    pub(super) fn new(
        gpu: &Gpu,
        pass: &io::Pass,
        (data, at, rows): (&Owned, u64, u32),
        sums: &Owned,
        (tally, to): (&Owned, u64),
    ) -> Result<Self> {
        let sizes = levels(rows);
        let top = sizes.len() - 1;
        let (mut starts, mut next) = (Vec::new(), 0);
        for &n in &sizes {
            starts.push(next);
            next += if starts.len() == 1 { 0 } else { n };
        }
        let (mut out, mut cfgs) = (Vec::new(), Vec::new());
        for (l, &n) in sizes.iter().enumerate() {
            let above = starts.get(l + 1).copied().unwrap_or(0);
            let words = [
                n,
                starts[l],
                above,
                u32::from(l == 0),
                u32::from(l == top),
                widest(gpu).1,
                1,
                1,
                0,
                0,
                0,
                0,
            ];
            let cfg = uniform(gpu, &words)?;
            let entries = [
                slice(0, &cfg, 0, cfg.size()),
                slice(1, data, at * 4, u64::from(rows.max(1)) * 4),
                slice(2, sums, 0, sums.size()),
                slice(3, tally, to * 4, 256),
            ];
            let bind = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("hierarchy scan"),
                layout: &pass.layout,
                entries: &entries,
            });
            out.push((bind, n));
            cfgs.push(cfg);
        }
        Ok(Self {
            levels: out,
            _cfgs: cfgs,
        })
    }

    pub(super) fn record(&self, pass: &io::Pass, e: &mut wgpu::CommandEncoder) {
        for (bind, n) in &self.levels {
            pass.run(e, bind, UP, n.div_ceil(SPAN).max(1));
        }
        for (bind, n) in self.levels.iter().rev().skip(1) {
            pass.run(e, bind, DOWN, n.div_ceil(64));
        }
    }

    pub(super) fn dispatches(&self) -> usize {
        2 * self.levels.len() - 1
    }
}
