//! A group of fine roots on the device (fn-209 F1 task 9): the roots
//! grown together in phase 1, as the driver sowed them while the
//! structure above the cut grew, or as the rest at its end. Each group
//! keeps its own waves, laid buffers and records, and settles through the
//! later phases on its own; a root's work reads only its own chains, so
//! how the roots are grouped never changes what any of them becomes.
use super::super::{io, Clock};
use super::laid::{stubs, Wave};
use super::realized::{grow_roots, RootBud, Waves};
use super::roots::{Cost, Kernels, Tables};
use super::shedding::{report, table};
use super::steps::{buffer, DESCRIBE};
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_space::{BearerFrame, Lived, RootGeometry, Vec3};

/// A chain's record (`settle.wgsl`'s `CR`).
pub(super) const CR: u32 = 5;
/// A root's record before them: its origins' heights and its first
/// axis's vigour (`settle.wgsl`'s `RR`).
const ROOT: u32 = 4;

/// A group as the device keeps it between the phases.
pub(super) struct Group {
    pub waves: Waves,
    /// The stubs (the first wave's bearers), then each wave's laid buffer.
    pub laid: Vec<Wave>,
    /// Each root's origins' heights and first axis's vigour, then each
    /// wave's chains' records.
    records: Owned,
    at: Vec<u32>,
    /// Each root's attachment as the stubs take it, and its node.
    pub attached: Vec<RootGeometry>,
    pub nodes: Vec<usize>,
    /// Each root's first axis's vigour, by its place: its seed's as
    /// `grow` hands it (task 9).
    pub made: Vec<f32>,
    /// The group's roots in the order the device took them (by their
    /// place in the group), and each one's living, by its place.
    order: Vec<usize>,
    pub lived: Vec<Lived>,
}

/// A root's attachment before its bearer is laid.
pub(super) fn unlaid(scale: f64) -> BearerFrame {
    BearerFrame {
        below: Vec3::default(),
        tip: Vec3::default(),
        heading: Vec3::default(),
        side: Vec3::default(),
        rank: 0.0,
        scale,
        size: 1.0,
        phase: 0.0,
    }
}

impl Group {
    /// Phase 1 for `roots` (each with its node), taken in `order`,
    /// `batch` chains a run: grown whole, their living carried up and
    /// read back, and the buffers the later phases fill sized.
    pub(super) async fn grow(
        (gpu, k): (&Gpu, &Kernels),
        (roots, nodes): (&[RootBud], Vec<usize>),
        (order, batch): (Vec<usize>, usize),
        (t, cost): (&Tables, &mut Cost),
    ) -> Result<Self> {
        let (waves, grown) = grow_roots((gpu, k), (roots, &order, batch), t).await?;
        cost.add(&grown);
        let fading = Clock::now();
        let fades = table(gpu, &t.chain)?;
        let living = k
            .fades
            .living(gpu, (&waves.grown, &waves.chains), (&fades, t.chain.age))?;
        let read = report(gpu, &living[0], roots.len(), cost).await?;
        cost.fade += fading.elapsed_ms();
        let mut lived = vec![(0.0, 0.0); roots.len()];
        for (wi, &r) in order.iter().enumerate() {
            lived[r] = read[wi];
        }
        // The laid buffers and the records, sized now: nothing more is
        // read back to size them.
        let n = roots.len() as u32;
        let mut laid = vec![Wave::taken(&k.pool, gpu, (n, n))?];
        let mut at = Vec::with_capacity(waves.chains.len());
        let mut floats = ROOT * n;
        for (w, &chains) in waves.chains.iter().enumerate() {
            laid.push(Wave::taken(&k.pool, gpu, (waves.own[w], waves.phyts[w]))?);
            at.push(floats);
            floats += CR * chains;
        }
        let records = buffer(gpu, "settle records", u64::from(floats))?;
        let attached = roots
            .iter()
            .map(|r| RootGeometry {
                vigour: f64::from(r.1[0]),
                whorl: f64::from(r.1[1]),
                bearer_pa: 0,
                unbent: unlaid(0.0),
                bent: unlaid(0.0),
            })
            .collect();
        Ok(Self {
            waves,
            laid,
            records,
            at,
            attached,
            nodes,
            made: vec![0.0; roots.len()],
            order,
            lived,
        })
    }

    /// Writes each root's stub and origins from its attachment.
    pub(super) fn attach(&self, gpu: &Gpu) {
        let (data, origins) = stubs(&self.attached, &self.made);
        let q = &gpu.queue;
        q.write_buffer(&self.laid[0].buffer, 0, bytemuck::cast_slice(&data));
        q.write_buffer(&self.records, 0, bytemuck::cast_slice(&origins));
    }

    /// Phase 2's start: each axis kept from what the structure above kept
    /// of each root (`on`, by its place), and each chain described for
    /// geometry, in its wave.
    pub(super) fn shed(&self, (gpu, k): (&Gpu, &Kernels), on: &[u32]) -> Result<()> {
        let usage = wgpu::BufferUsages::STORAGE;
        let bytes = (on.len().max(4) * 4) as u64;
        let on = io::buffer(gpu, "roots kept", bytes, usage, bytemuck::cast_slice(on))?;
        k.fades
            .shed(gpu, (&self.waves.grown, &self.waves.own), &on)?;
        let mut encoder = gpu.encoder(&Default::default());
        for (w, g) in self.waves.grown.iter().enumerate() {
            // Two buffers: WebGPU refuses one buffer at two writable bindings.
            let (buds, meta) = (buffer(gpu, "no buds", 4)?, buffer(gpu, "no meta", 4)?);
            for run in &g.runs {
                let lay = (&*run.runoff, &*g.places, &*g.meta, &*run.items);
                let hand = (&*run.axes, &*run.scratch, &*buds, &*meta);
                k.steps
                    .hand(gpu, &mut encoder, (run.n, w == 0, DESCRIBE), lay, hand)?;
            }
        }
        gpu.submit([encoder.finish()]);
        Ok(())
    }

    /// Runs `entry` over every wave, down from the first (`down`) or up
    /// from the deepest.
    pub(super) fn pass(
        &self,
        (gpu, k): (&Gpu, &Kernels),
        t: &Tables,
        entry: usize,
        down: bool,
    ) -> Result<()> {
        let mut encoder = gpu.encoder(&Default::default());
        let n = self.waves.grown.len();
        let waves: Vec<usize> = match down {
            true => (0..n).collect(),
            false => (0..n).rev().collect(),
        };
        for w in waves {
            let prior = self.at.get(w + 1).copied().unwrap_or(0);
            for run in &self.waves.grown[w].runs {
                let packed = (&*run.axes, &*run.phyts);
                let item = (&*run.items, run.n, packed, &*run.scratch);
                let at = (entry, prior, self.at[w]);
                let laid = (&self.laid[w], &self.laid[w + 1]);
                let bufs = (&*t.table, &*self.records);
                let how = (t.chain.age, t.sags);
                k.resolver
                    .settle(gpu, &mut encoder, item, bufs, laid, how, at)?;
            }
        }
        gpu.submit([encoder.finish()]);
        Ok(())
    }

    /// The first wave's records copied into `into` at `at` bytes; the
    /// bytes they take.
    pub(super) fn gather(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        into: &wgpu::Buffer,
        at: u64,
    ) -> u64 {
        let bytes = (self.order.len() * CR as usize * 4) as u64;
        if bytes > 0 {
            let from = u64::from(self.at[0]) * 4;
            encoder.copy_buffer_to_buffer(&self.records, from, into, at, bytes);
        }
        bytes
    }

    /// The first wave's records as gathered (`floats`): one record per
    /// root, by its place in the group.
    pub(super) fn records(&self, floats: &[f32]) -> Vec<[f32; CR as usize]> {
        let cr = CR as usize;
        let mut out = vec![[0.0; CR as usize]; self.order.len()];
        for (wi, &r) in self.order.iter().enumerate() {
            out[r].copy_from_slice(&floats[wi * cr..][..cr]);
        }
        out
    }

    /// Its pooled buffers, for the next build (`pool.rs`).
    pub(super) fn buffers(self) -> impl Iterator<Item = Owned> {
        let runs = self.waves.grown.into_iter().flat_map(|g| g.runs);
        let packed = runs.flat_map(|r| [r.axes, r.phyts, r.scratch]);
        packed.chain(self.laid.into_iter().map(|w| w.buffer))
    }
}
