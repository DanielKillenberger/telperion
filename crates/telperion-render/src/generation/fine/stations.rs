//! The fine roots' leaf stations on the device (fn-212 task 6;
//! CONTRACT.md's probe, step 6): their bearing runs, station counts and
//! the station segments placement reads, `prepare_compact_stations`'s
//! layout (`data.rs`), from the nodes and curve `sweep.wgsl` left. Only
//! totals are read back: the runs, then the stations and segments.
use super::super::io;
use super::sweep::Swept;
use crate::{Gpu, Result};
use telperion_core::pipeline::executor::Expansion;

const STATIONS: &str = include_str!("stations.wgsl");
const SW: u64 = 4;
pub(super) const RW: u64 = 6;
/// A segment's words (`data.rs`'s `Segment`).
pub(super) const GW: usize = 32;
const KIDS: usize = 0;
const BEAR: usize = 1;
const STARTS: usize = 2;
const MEASURE: usize = 3;
const BASES: usize = 4;
const EMIT: usize = 5;
const BLOCKS: usize = 6;
const SPREAD: usize = 7;
/// Runs a workgroup of the bases' scan takes.
const LANES: u32 = 256;

pub(super) fn source() -> String {
    STATIONS.to_string()
}

/// The rows the stations read: the stations per internode, the ring's
/// segments, `STRUCTURAL` of the root's radius, the slender bound, the
/// twig layer's internode, the divergence (radians) and whether leaves
/// are seated.
#[derive(Debug, Clone, Copy)]
pub(super) struct Rows {
    pub spi: u32,
    pub segments: u32,
    pub structural: f32,
    pub slender: f32,
    pub internode: f32,
    pub divergence: f32,
    pub contact: bool,
}

/// The fine roots' station segments on the device, their count and the
/// stations they hold.
/// Destroyed when let go: a browser frees a buffer only then (fn-212
/// task 10; wgpu's web backend drops nothing).
impl Drop for Stationed {
    fn drop(&mut self) {
        self.buffer.destroy();
        self.run_words.destroy();
    }
}

pub(super) struct Stationed {
    #[cfg_attr(not(test), allow(dead_code))]
    pub runs: u32,
    pub stations: u32,
    pub segments: u32,
    pub buffer: wgpu::Buffer,
    /// The runs' words (`RW` a run): first node, stations, segments,
    /// nodes, first station and first segment.
    #[cfg_attr(not(test), allow(dead_code))]
    pub run_words: wgpu::Buffer,
}

pub(super) struct Stationer {
    pass: io::Pass,
}

fn storage(gpu: &Gpu, label: &'static str, words: u64) -> wgpu::Buffer {
    let usage =
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    io::buffer(gpu, label, words.max(4) * 4, usage, &[]).unwrap()
}

impl Stationer {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let read = [true, true, false, false, false, false];
        let entries = [
            "kids", "bear", "starts", "measure", "bases", "emit", "blocks", "spread",
        ];
        Self {
            pass: io::Pass::new(gpu, source(), &read, &entries),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn run(
        &self,
        (gpu, e): (&Gpu, &mut wgpu::CommandEncoder),
        (entry, n, groups): (usize, u32, Option<u32>),
        rows: Rows,
        bufs: [&wgpu::Buffer; 6],
    ) {
        let groups = groups.unwrap_or(n.div_ceil(64)).max(1);
        let most = gpu.device.limits().max_compute_workgroups_per_dimension;
        let words = [
            n,
            groups.min(most) * 64,
            rows.spi,
            rows.segments,
            rows.structural.to_bits(),
            rows.slender.to_bits(),
            rows.internode.to_bits(),
            rows.divergence.to_bits(),
            u32::from(rows.contact),
            0,
            0,
            0,
        ];
        let usage = wgpu::BufferUsages::UNIFORM;
        let cfg = io::buffer(gpu, "stations cfg", 48, usage, bytemuck::cast_slice(&words));
        let cfg = cfg.unwrap();
        let mut all = vec![&cfg];
        all.extend(bufs);
        let bind = self.pass.bind(gpu, &all);
        self.pass.run(e, &bind, entry, groups);
    }

    async fn counts(
        gpu: &Gpu,
        e: &mut wgpu::CommandEncoder,
        counts: &wgpu::Buffer,
    ) -> Result<[u32; 4]> {
        let fresh = gpu.device.create_command_encoder(&Default::default());
        gpu.queue.submit([std::mem::replace(e, fresh).finish()]);
        let raw = io::read_async(gpu, counts, 16).await?;
        let w: &[u32] = bytemuck::cast_slice(&raw);
        Ok([w[0], w[1], w[2], w[3]])
    }

    /// The stations of the `ids` nodes `swept` laid out.
    pub(super) async fn stations(
        &self,
        gpu: &Gpu,
        swept: &Swept,
        ids: u32,
        rows: Rows,
    ) -> Result<Stationed> {
        let st = storage(gpu, "station words", u64::from(ids) * SW);
        let counts = storage(gpu, "station counts", 4);
        let none = storage(gpu, "stations none", 4);
        let mut e = gpu.device.create_command_encoder(&Default::default());
        let none_runs = storage(gpu, "station runs none", 4);
        for entry in [KIDS, BEAR] {
            self.run(
                (gpu, &mut e),
                (entry, ids, None),
                rows,
                [&swept.nodes, &swept.table, &st, &none_runs, &counts, &none],
            );
        }
        let runs = storage(gpu, "station runs", u64::from(ids) * RW);
        self.run(
            (gpu, &mut e),
            (STARTS, ids, None),
            rows,
            [&swept.nodes, &swept.table, &st, &runs, &counts, &none],
        );
        let n = Self::counts(gpu, &mut e, &counts).await?[0];
        self.run(
            (gpu, &mut e),
            (MEASURE, n, None),
            rows,
            [&swept.nodes, &swept.table, &st, &runs, &counts, &none],
        );
        // The runs' first stations and segments, scanned in blocks.
        let blocks = n.div_ceil(LANES).max(1);
        let totals = storage(gpu, "station blocks", u64::from(blocks) * 2);
        for (entry, groups) in [(BASES, blocks), (BLOCKS, 1), (SPREAD, blocks)] {
            self.run(
                (gpu, &mut e),
                (entry, n, Some(groups)),
                rows,
                [&swept.nodes, &swept.table, &st, &runs, &counts, &totals],
            );
        }
        let [_, stations, segments, _] = Self::counts(gpu, &mut e, &counts).await?;
        let buffer = storage(gpu, "station segments", u64::from(segments) * GW as u64);
        let late = [&swept.nodes, &swept.table, &st, &runs, &counts, &buffer];
        self.run((gpu, &mut e), (EMIT, n, None), rows, late);
        gpu.queue.submit([e.finish()]);
        io::complete(gpu).await?;
        for done in [&st, &counts, &none, &none_runs, &totals] {
            done.destroy();
        }
        Ok(Stationed {
            runs: n,
            stations,
            segments,
            buffer,
            run_words: runs,
        })
    }
}

/// The rows the device's stations read from the pipeline's expansion, its
/// rings `segments` vertices round.
pub(super) fn station_rows(x: &Expansion, segments: u32) -> Rows {
    let [structural, slender] = x.bearing_bounds();
    let (twig, canopy) = (x.twig(), x.leaves().canopy);
    Rows {
        spi: twig.stations_per_internode,
        segments,
        structural: structural as f32,
        slender: slender as f32,
        internode: twig.internode_length as f32,
        divergence: canopy.divergence.to_radians() as f32,
        contact: x.seats(),
    }
}
