//! The device steps between growth and geometry (fn-209 F1, task 5; host
//! decision 6): a run's chains recorded (`record.wgsl`), counts scanned
//! into places (`scan.wgsl`), the chains a region could not hold gathered
//! for a larger one (`gather.wgsl`), and each wave described for geometry
//! and handed on as the next wave's buds (`emit.wgsl`). Only the scans'
//! totals cross to the host.
use super::super::io;
use crate::Gpu;

const RECORD: &str = include_str!("record.wgsl");
const SCAN: &str = include_str!("scan.wgsl");
const GATHER: &str = include_str!("gather.wgsl");
const EMIT: &str = include_str!("emit.wgsl");
/// A run's record of a chain, and a wave's counts of one.
pub(super) const RW: u32 = 10;
pub(super) const CW: u32 = 3;
/// A bud's words (`chain.wgsl`), as u32s.
pub(super) const BW2: u32 = 24;
/// An item's words (`geometry.wgsl`).
pub(super) const IW: u32 = 10;

pub(super) fn sources() -> Vec<(&'static str, String)> {
    vec![
        ("fine record", RECORD.to_string()),
        ("fine scan", SCAN.to_string()),
        ("fine gather", GATHER.to_string()),
        ("fine emit", EMIT.to_string()),
    ]
}

pub(super) struct Steps {
    record: io::Pass,
    scan: io::Pass,
    gather: io::Pass,
    emit: io::Pass,
}

/// A buffer for the device's own use, readable for its totals.
pub(super) fn buffer(gpu: &Gpu, label: &'static str, words: u64) -> wgpu::Buffer {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    io::buffer(gpu, label, words.max(4) * 4, usage, &[]).unwrap()
}

fn uniform(gpu: &Gpu, data: &[u32; 8]) -> wgpu::Buffer {
    let usage = wgpu::BufferUsages::UNIFORM;
    io::buffer(gpu, "steps cfg", 32, usage, bytemuck::cast_slice(data)).unwrap()
}

fn row(gpu: &Gpu, n: u32) -> (u32, u32) {
    let groups = n.div_ceil(64);
    let most = gpu.device.limits().max_compute_workgroups_per_dimension;
    (groups, groups.min(most) * 64)
}

impl Steps {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let rw = |n: usize, reads: &[usize]| -> Vec<bool> {
            (1..=n).map(|b| reads.contains(&b)).collect()
        };
        Self {
            record: io::Pass::new(gpu, RECORD.into(), &rw(7, &[1, 2, 3]), &["record"]),
            scan: io::Pass::new(
                gpu,
                SCAN.into(),
                &rw(3, &[]),
                &["blocks", "spine", "spread"],
            ),
            gather: io::Pass::new(gpu, GATHER.into(), &rw(3, &[1, 2]), &["gather"]),
            emit: io::Pass::new(
                gpu,
                EMIT.into(),
                &rw(8, &[1, 2, 3, 5, 6]),
                &["describe", "emit"],
            ),
        }
    }

    /// Records a run's `n` chains: `level` 0 reads chain `c` as the
    /// wave's `first + c`, a later level as `list[first + c]`.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn record(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        (n, first, level, axes_cap): (u32, u32, u32, u32),
        (status, axes, list): (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
        (counts, queue, queued, runoff): (
            &wgpu::Buffer,
            &wgpu::Buffer,
            &wgpu::Buffer,
            &wgpu::Buffer,
        ),
    ) {
        let (groups, row) = row(gpu, n);
        let cfg = uniform(gpu, &[n, row, first, level, axes_cap, 0, 0, 0]);
        let all = [&cfg, status, axes, list, counts, queue, queued, runoff];
        let bind = self.record.bind(gpu, &all);
        self.record.run(encoder, &bind, 0, groups);
    }

    /// Scans the first `fields` words of `n` records `stride` apart into
    /// places, their totals into `totals`.
    pub(super) fn scan(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        (data, n, stride, fields): (&wgpu::Buffer, u32, u32, u32),
        totals: &wgpu::Buffer,
    ) {
        let blocks = n.div_ceil(256);
        let (groups, row) = row(gpu, n);
        let cfg = uniform(gpu, &[n, stride, fields, blocks, row, 0, 0, 0]);
        let sums = buffer(gpu, "scan sums", u64::from(blocks.max(1)) * 4);
        let bind = self.scan.bind(gpu, &[&cfg, data, &sums, totals]);
        self.scan.run(encoder, &bind, 0, blocks.div_ceil(64));
        self.scan.run(encoder, &bind, 1, 1);
        self.scan.run(encoder, &bind, 2, groups);
    }

    /// Gathers the `n` queued chains' buds from the wave's into `gathered`.
    pub(super) fn gather(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        n: u32,
        (queue, wave, gathered): (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
    ) {
        let (groups, row) = row(gpu, n);
        let cfg = uniform(gpu, &[n, row, 0, 0, 0, 0, 0, 0]);
        let bind = self.gather.bind(gpu, &[&cfg, queue, wave, gathered]);
        self.gather.run(encoder, &bind, 0, groups);
    }

    /// Describes a run's `n` chains for geometry into `items` (`hand`
    /// false), or hands their kept axes' buds on (`hand` true).
    #[allow(clippy::too_many_arguments)]
    pub(super) fn hand(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        (n, stub, hand): (u32, bool, bool),
        (runoff, places, meta, items): (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
        (axes, scratch, buds, next_meta): (
            &wgpu::Buffer,
            &wgpu::Buffer,
            &wgpu::Buffer,
            &wgpu::Buffer,
        ),
    ) {
        let (groups, row) = row(gpu, n);
        let cfg = uniform(gpu, &[n, row, u32::from(stub), 0, 0, 0, 0, 0]);
        let all = [
            &cfg, runoff, places, meta, items, axes, scratch, buds, next_meta,
        ];
        let bind = self.emit.bind(gpu, &all);
        self.emit.run(encoder, &bind, usize::from(hand), groups);
    }
}
