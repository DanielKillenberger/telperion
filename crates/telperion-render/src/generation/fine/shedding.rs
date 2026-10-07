//! Phase 1's shedding on the device (fn-209 F1 task 6; REALIZED.md, host
//! decision 8, step 3): each root's subtree's living carried up from the
//! deepest wave (`fades.wgsl`'s `up`), one record per root read back for
//! the structure above the cut to settle on, then each axis kept from the
//! first wave down (`down`), from what the structure above kept of each
//! root. Nothing per chain or per node crosses.
use super::super::io;
use super::roots::{Cost, Grown};
use super::steps::buffer;
use crate::Gpu;
use telperion_space::ChainTables;

const FADES: &str = include_str!("fades.wgsl");

/// The fades' WGSL.
pub(super) fn source() -> String {
    format!("{}\n{FADES}", super::super::fine::contract())
}

pub(super) struct Fades {
    pass: io::Pass,
}

/// Each PA's fade delay (Q32) and whether it sheds, as `up` reads them.
pub(super) fn table(gpu: &Gpu, chain: &ChainTables) -> wgpu::Buffer {
    let words: Vec<u32> = chain
        .pas
        .iter()
        .flat_map(|r| {
            let d = r.fade_delay;
            [d as u32, (d >> 32) as u32, u32::from(r.delay.is_some())]
        })
        .collect();
    let usage = wgpu::BufferUsages::STORAGE;
    let bytes = (words.len().max(4) * 4) as u64;
    io::buffer(
        gpu,
        "fade table",
        bytes,
        usage,
        bytemuck::cast_slice(&words),
    )
    .unwrap()
}

impl Fades {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let read = [true, true, false, true, true, true, false];
        Self {
            pass: io::Pass::new(gpu, source(), &read, &["up", "down"]),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn dispatch(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        (entry, age): (usize, u32),
        g: &Grown,
        (info, prior, mine): (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
    ) {
        for run in &g.runs {
            let groups = run.n.div_ceil(64);
            let most = gpu.device.limits().max_compute_workgroups_per_dimension;
            let row = groups.min(most) * 64;
            let cfg = [run.n, row, age, 0];
            let uniform = wgpu::BufferUsages::UNIFORM;
            let data = bytemuck::cast_slice(&cfg);
            let cfg = io::buffer(gpu, "fades cfg", 16, uniform, data).unwrap();
            let all = [
                &cfg,
                &run.runoff,
                &g.places,
                &run.axes,
                &run.phyts,
                info,
                prior,
                mine,
            ];
            let bind = self.pass.bind(gpu, &all);
            self.pass.run(encoder, &bind, entry, groups);
        }
    }

    /// Each wave's chains' living (4 words a chain), carried up from the
    /// deepest wave; the first wave's is one record per root.
    pub(super) fn living(
        &self,
        gpu: &Gpu,
        (waves, chains): (&[Grown], &[u32]),
        (table, age): (&wgpu::Buffer, u32),
    ) -> Vec<wgpu::Buffer> {
        let mut out: Vec<wgpu::Buffer> = chains
            .iter()
            .map(|&n| buffer(gpu, "wave living", u64::from(n) * 4))
            .collect();
        let none = buffer(gpu, "no living", 4);
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        for w in (0..waves.len()).rev() {
            let prior = out.get(w + 1).unwrap_or(&none);
            self.dispatch(
                gpu,
                &mut encoder,
                (0, age),
                &waves[w],
                (table, prior, &out[w]),
            );
        }
        gpu.queue.submit([encoder.finish()]);
        out.truncate(waves.len());
        out
    }

    /// Each wave's own axes kept (a word each, at their places), from
    /// `roots`, each root's stand-in kept by the structure above.
    pub(super) fn shed(
        &self,
        gpu: &Gpu,
        (waves, own): (&[Grown], &[u32]),
        roots: &wgpu::Buffer,
    ) -> Vec<wgpu::Buffer> {
        let mut out: Vec<wgpu::Buffer> = Vec::with_capacity(waves.len());
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        for (w, g) in waves.iter().enumerate() {
            let mine = buffer(gpu, "wave kept", u64::from(own[w]));
            let prior = out.last().unwrap_or(roots);
            self.dispatch(gpu, &mut encoder, (1, 0), g, (&g.meta, prior, &mine));
            out.push(mine);
        }
        gpu.queue.submit([encoder.finish()]);
        out
    }
}

/// The first wave's living read back: one record per root, by its place
/// in the order the roots were taken (lived, Q32.32; alive, Q32).
pub(super) fn report(
    gpu: &Gpu,
    living: &wgpu::Buffer,
    n: usize,
    cost: &mut Cost,
) -> Vec<(u64, u64)> {
    cost.trips += 1;
    let bytes = io::read(gpu, living, (n.max(1) * 16) as u64).unwrap();
    let words: &[u32] = bytemuck::cast_slice(&bytes);
    words
        .chunks_exact(4)
        .take(n)
        .map(|w| {
            let wide = |lo: u32, hi: u32| u64::from(lo) | u64::from(hi) << 32;
            (wide(w[0], w[1]), wide(w[2], w[3]))
        })
        .collect()
}
