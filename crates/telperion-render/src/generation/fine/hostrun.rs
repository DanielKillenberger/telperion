//! The chain grower run with its regions read back, for the chain tests
//! (fn-209 F1, task 2): each chain's topology and words, the buds' place
//! offsets reckoned on the host. The executor itself uses `roots.rs`'s
//! device-resident runs.
use super::super::io;
use super::chains::{bud_words, topology, upload, Caps, Grower, Packed, Region, AXW, PW};
use crate::Gpu;
use std::time::Instant;
use telperion_space::{ChainAxis, ChainBud};

/// One dispatch's chains as grown: each chain's axes, or none where it
/// outgrew its region.
pub(super) type Grown = Vec<Option<Grew>>;

/// A chain as grown: its topology, and its axes' words, which its buds'
/// own chains are read from.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Grew {
    pub axes: Vec<ChainAxis>,
    pub words: Vec<u32>,
}

impl Grower {
    /// Grows `buds` in regions of `caps`: each chain's axes, or none where
    /// it outgrew its region; the milliseconds from upload to the device's
    /// finishing the growth, and those packing and reading it back.
    pub(super) fn run(
        &self,
        gpu: &Gpu,
        buds: (&[ChainBud], &[[f32; 2]]),
        t: &Packed,
        at: (i64, u32),
        caps: Caps,
    ) -> (Grown, f64, f64) {
        let run = self.run_kept(gpu, buds, t, at, caps);
        (run.grown, run.grow_ms, run.read_ms)
    }

    /// `run`, as a record.
    pub(super) fn run_kept(
        &self,
        gpu: &Gpu,
        (buds, made): (&[ChainBud], &[[f32; 2]]),
        t: &Packed,
        (span, age): (i64, u32),
        caps: Caps,
    ) -> Run {
        let start = Instant::now();
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let n = buds.len() as u32;
        let input: Vec<u64> = buds
            .iter()
            .zip(made)
            .flat_map(|(b, &m)| bud_words(b, m, [1 << 32; 2]))
            .collect();
        let b =
            |label, bytes: u64, data: &[u8]| io::buffer(gpu, label, bytes, storage, data).unwrap();
        let input = b(
            "chain buds",
            input.len() as u64 * 8,
            bytemuck::cast_slice(&input),
        );
        let tables = upload(gpu, t);
        let mut encoder = gpu.encoder(&Default::default());
        let region = self.region(
            gpu,
            &mut encoder,
            (&input, 0, n),
            (&tables, t, span, age),
            caps,
        );
        let Region {
            axes,
            phyts,
            status,
            row,
            groups,
        } = region;
        gpu.submit([encoder.finish()]);
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();
        let grown_ms = start.elapsed().as_secs_f64() * 1e3;
        let start = Instant::now();
        let read = io::read(gpu, &status, u64::from(n) * 16).unwrap();
        let status_words: Vec<[u32; 4]> = bytemuck::cast_slice::<u8, u32>(&read)
            .as_chunks::<4>()
            .0
            .to_vec();
        // Each chain's place in the packed axes and phytomers.
        let mut offsets = Vec::with_capacity(buds.len());
        let (mut at_axes, mut at_phyts) = (0u32, 0u32);
        for &[count, grown, overflow, _] in &status_words {
            offsets.push([at_axes, at_phyts]);
            if overflow == 0 {
                (at_axes, at_phyts) = (at_axes + count, at_phyts + grown);
            }
        }
        let offsets_buf = b(
            "pack offsets",
            offsets.len() as u64 * 8,
            bytemuck::cast_slice(&offsets),
        );
        let axes_bytes = u64::from(at_axes.max(1)) * AXW as u64 * 4;
        let phyt_bytes = u64::from(at_phyts.max(1)) * PW as u64 * 4;
        let packed_axes = b("packed axes", axes_bytes, &[]);
        let packed_phyts = b("packed phytomers", phyt_bytes, &[]);
        let region = Region {
            axes,
            phyts,
            status,
            row,
            groups,
        };
        let mut encoder = gpu.encoder(&Default::default());
        self.pack(
            gpu,
            &mut encoder,
            (&region, n, caps),
            (&offsets_buf, 2),
            (&packed_axes, &packed_phyts),
        );
        gpu.submit([encoder.finish()]);
        let axes = io::read(gpu, &packed_axes, axes_bytes).unwrap();
        let axes: &[u32] = bytemuck::cast_slice(&axes);
        let phyts = io::read(gpu, &packed_phyts, phyt_bytes).unwrap();
        let phyts: &[u32] = bytemuck::cast_slice(&phyts);
        let grown = buds
            .iter()
            .enumerate()
            .map(|(c, bud)| {
                let [count, grown, overflow, _] = status_words[c];
                let [a, p] = offsets[c].map(|o| o as usize);
                (overflow == 0).then(|| {
                    let a = &axes[a * AXW..(a + count as usize) * AXW];
                    Grew {
                        axes: topology(bud, a, &phyts[p * PW..(p + grown as usize) * PW]),
                        words: a.to_vec(),
                    }
                })
            })
            .collect();
        Run {
            grown,
            grow_ms: grown_ms,
            read_ms: start.elapsed().as_secs_f64() * 1e3,
        }
    }
}

/// One dispatch of chains as grown, read back for the chain tests.
pub(super) struct Run {
    pub grown: Grown,
    pub grow_ms: f64,
    pub read_ms: f64,
}
