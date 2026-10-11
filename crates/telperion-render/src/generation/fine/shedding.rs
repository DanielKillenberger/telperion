//! Phase 1's shedding on the device (fn-209 F1 task 6; REALIZED.md, host
//! decision 8, step 3): each root's subtree's living carried up from the
//! deepest wave (`fades.wgsl`'s `up`), one record per root read back for
//! the structure above the cut to settle on, then each axis kept from the
//! first wave down (`down`), from what the structure above kept of each
//! root. Nothing per chain or per node crosses.
use super::super::io;
use super::roots::{Cost, Grown};
use super::steps::buffer;
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_space::ChainTables;

const FADES: &str = include_str!("fades.wgsl");

/// The fades' WGSL.
pub(super) fn source() -> String {
    format!("{}\n{FADES}", super::super::fine::contract())
}

pub(super) struct Fades {
    pass: io::Pass,
}

/// Each PA's fade delay (f32, two words) and whether it sheds, as `up`
/// reads them.
pub(super) fn table(gpu: &Gpu, chain: &ChainTables) -> Result<Owned> {
    let words: Vec<u32> = chain
        .pas
        .iter()
        .flat_map(|r| {
            [r.fade_delay.to_bits(), 0, u32::from(r.delay.is_some())]
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
    ) -> Result<()> {
        for run in &g.runs {
            let groups = run.n.div_ceil(64);
            let most = gpu.device.limits().max_compute_workgroups_per_dimension;
            let row = groups.min(most) * 64;
            let cfg = [run.n, row, age, 0];
            let uniform = wgpu::BufferUsages::UNIFORM;
            let data = bytemuck::cast_slice(&cfg);
            let cfg = io::buffer(gpu, "fades cfg", 16, uniform, data)?;
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
        Ok(())
    }

    /// Each wave's chains' living (2 words a chain), carried up from the
    /// deepest wave; the first wave's is one record per root.
    pub(super) fn living(
        &self,
        gpu: &Gpu,
        (waves, chains): (&[Grown], &[u32]),
        (table, age): (&wgpu::Buffer, u32),
    ) -> Result<Vec<Owned>> {
        let mut out: Vec<Owned> = chains
            .iter()
            .map(|&n| buffer(gpu, "wave living", u64::from(n) * 2))
            .collect::<Result<_>>()?;
        let none = buffer(gpu, "no living", 4)?;
        let mut encoder = gpu.encoder(&Default::default());
        for w in (0..waves.len()).rev() {
            let prior = out.get(w + 1).unwrap_or(&none);
            self.dispatch(
                gpu,
                &mut encoder,
                (0, age),
                &waves[w],
                (table, prior, &out[w]),
            )?;
        }
        gpu.submit([encoder.finish()]);
        out.truncate(waves.len());
        Ok(out)
    }

    /// Each wave's own axes kept (a word each, at their places), from
    /// `roots`, each root's stand-in kept by the structure above.
    pub(super) fn shed(
        &self,
        gpu: &Gpu,
        (waves, own): (&[Grown], &[u32]),
        roots: &wgpu::Buffer,
    ) -> Result<Vec<Owned>> {
        let mut out: Vec<Owned> = Vec::with_capacity(waves.len());
        let mut encoder = gpu.encoder(&Default::default());
        for (w, g) in waves.iter().enumerate() {
            let mine = buffer(gpu, "wave kept", u64::from(own[w]))?;
            let prior = out.last().map_or(roots, |b| b);
            self.dispatch(gpu, &mut encoder, (1, 0), g, (&g.meta, prior, &mine))?;
            out.push(mine);
        }
        gpu.submit([encoder.finish()]);
        Ok(out)
    }
}

/// The first wave's living read back: one record per root, by its place
/// in the order the roots were taken (lived, in cycles; alive).
pub(super) async fn report(
    gpu: &Gpu,
    living: &wgpu::Buffer,
    n: usize,
    cost: &mut Cost,
) -> crate::Result<Vec<(f32, f32)>> {
    cost.trips += 1;
    let bytes = io::read_async(gpu, living, (n.max(1) * 8) as u64).await?;
    let words: &[f32] = bytemuck::cast_slice(&bytes);
    Ok(words.chunks_exact(2).take(n).map(|w| (w[0], w[1])).collect())
}
