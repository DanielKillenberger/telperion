//! Fine roots under realized mechanics on the GPU (fn-209 F1 task 6;
//! REALIZED.md, "Host decision 8"): each root's subtree grown whole on
//! the device, wave by wave, every axis kept, its shedding left to the
//! fades its whole subtree realizes. Per run and per wave only the scans'
//! totals cross to the host (host decision 6).
use super::super::Clock;
use super::chains::bud_words;
use super::roots::{grow_wave, totals, Cost, Grown, Kernels, Tables};
use super::steps::{buffer, BW2, CW, REALIZED};
use crate::{Gpu, Result};
use telperion_space::ChainBud;

/// A root as the device takes it: its bud, its first axis's vigour as
/// made and its node's buds a whorl (f32), and its making draws'
/// presences.
pub(super) type RootBud = (ChainBud, [f32; 2], [f32; 2]);

/// The waves as the device keeps them, and each one's chains, own axes
/// and their phytomers.
pub(super) struct Waves {
    pub grown: Vec<Grown>,
    pub chains: Vec<u32>,
    pub own: Vec<u32>,
    pub phyts: Vec<u32>,
}

/// Phase 1's growth: every root of `roots`, taken in `order`, grown
/// whole, `batch` chains a run; every wave as the device keeps it.
pub(super) async fn grow_roots(
    (gpu, k): (&Gpu, &Kernels),
    (roots, order, batch): (&[RootBud], &[usize], usize),
    tables: &Tables,
) -> Result<(Waves, Cost)> {
    let mut cost = Cost::default();
    let words: Vec<u64> = order
        .iter()
        .flat_map(|&r| bud_words(&roots[r].0, roots[r].1, roots[r].2))
        .collect();
    let meta: Vec<[u32; 2]> = order.iter().map(|&r| [r as u32, r as u32]).collect();
    let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let up = |label, data: &[u8]| {
        let bytes = data.len().max(16) as u64;
        super::super::io::buffer(gpu, label, bytes, storage, data)
    };
    let mut wave_buds = up("wave buds", bytemuck::cast_slice(&words))?;
    let mut wave_meta = up("wave meta", bytemuck::cast_slice(&meta))?;
    let mut n = order.len() as u32;
    let mut waves = Waves {
        grown: Vec::new(),
        chains: Vec::new(),
        own: Vec::new(),
        phyts: Vec::new(),
    };
    while n > 0 {
        cost.waves += 1;
        let start = Clock::now();
        let places = buffer(gpu, "wave places", u64::from(n * CW))?;
        let at = (&*wave_buds, n, batch);
        let runs = grow_wave(gpu, k, at, (&*places, tables), &mut cost).await?;
        let mut encoder = gpu.encoder(&Default::default());
        let sums = buffer(gpu, "wave totals", 4)?;
        k.steps
            .scan(gpu, &mut encoder, (&*places, n, CW, CW), &sums)?;
        gpu.submit([encoder.finish()]);
        let t = totals(gpu, &sums, 3, &mut cost).await?;
        let next_buds = buffer(gpu, "wave buds", u64::from(t[2] * BW2))?;
        let next_meta = buffer(gpu, "wave meta", u64::from(t[2] * 2))?;
        let mut encoder = gpu.encoder(&Default::default());
        for run in &runs {
            let lay = (&*run.runoff, &*places, &*wave_meta, &*run.items);
            let hand = (&*run.axes, &*run.scratch, &*next_buds, &*next_meta);
            k.steps
                .hand(gpu, &mut encoder, (run.n, false, REALIZED), lay, hand)?;
        }
        gpu.submit([encoder.finish()]);
        cost.grow += start.elapsed_ms();
        waves.grown.push(Grown {
            meta: wave_meta,
            places,
            runs,
        });
        waves.chains.push(n);
        waves.own.push(t[0]);
        waves.phyts.push(t[1]);
        // This wave's buds are spent once its passes are submitted.
        wave_buds.destroy();
        (wave_buds, wave_meta, n) = (next_buds, next_meta, t[2]);
    }
    super::super::io::complete(gpu).await?;
    Ok((waves, cost))
}
