//! The hierarchy's buffers and its scan (`hier_scan.wgsl`): an exclusive
//! prefix sum of a column slice in one dispatch, or two past a block of
//! `SPAN` words, its total to a tally's word.
use super::super::io;
use crate::buffer::Owned;
use crate::{Gpu, Result};

const UP: usize = 0;
const DOWN: usize = 1;
/// The records a workgroup of `hier_scan.wgsl` scans.
pub(super) const SPAN: u32 = 1024;

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

/// `n` words of `b` from word `at`, through a copy, awaited.
pub(super) async fn words(gpu: &Gpu, b: &wgpu::Buffer, at: u64, n: u64) -> Result<Vec<u32>> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let to = io::buffer(gpu, "hierarchy read", n.max(1) * 4, usage, &[])?;
    let mut e = gpu.encoder(&Default::default());
    e.copy_buffer_to_buffer(b, at * 4, &to, 0, n * 4);
    gpu.submit([e.finish()]);
    let raw = io::read_async(gpu, &to, n * 4).await?;
    Ok(bytemuck::cast_slice(&raw).to_vec())
}

/// `words`, blocked on: the tests' readbacks.
#[cfg(test)]
pub(super) fn read_words(gpu: &Gpu, b: &wgpu::Buffer, at: u64, n: u64) -> Result<Vec<u32>> {
    pollster::block_on(words(gpu, b, at, n))
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

/// The words a scan of `rows` records of `fields` words keeps in its
/// sums: each block's total, a field's after another's.
pub(super) fn sums_words(rows: u32, fields: u32) -> u64 {
    u64::from(rows.div_ceil(SPAN) * fields) + 4
}

/// One exclusive scan of `rows` words of a buffer from word `at`, its
/// total to a tally's word `to`: one dispatch, or two past a block.
pub(super) struct Scan {
    bind: wgpu::BindGroup,
    rows: u32,
    _cfg: Owned,
}

impl Scan {
    pub(super) fn new(
        gpu: &Gpu,
        pass: &io::Pass,
        (data, at, rows): (&Owned, u64, u32),
        sums: &Owned,
        (tally, to): (&Owned, u64),
    ) -> Result<Self> {
        let cfg = uniform(gpu, &[rows, 0, widest(gpu).1, 1, 1, 0, 1, 0])?;
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
        Ok(Self { bind, rows, _cfg: cfg })
    }

    pub(super) fn record(&self, pass: &io::Pass, e: &mut wgpu::CommandEncoder) {
        let blocks = self.rows.div_ceil(SPAN);
        pass.run(e, &self.bind, UP, blocks.max(1));
        if self.rows > SPAN {
            pass.run(e, &self.bind, DOWN, blocks);
        }
    }

    pub(super) fn dispatches(&self) -> usize {
        1 + usize::from(self.rows > SPAN)
    }
}
