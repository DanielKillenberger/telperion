//! A field asked: a batch of closed cubes answered on the device, only
//! the answers read back. A cell met by more limb systems than one tally
//! holds is asked again, those cells alone, a window of systems at a
//! time, and given the system with the most leaves over every window.
use super::super::super::io;
use super::super::build::Tools;
use super::{storage, Answer, DeviceField, Rec, OVER, TALLY};
use crate::{Gpu, Result};
use telperion_core::Error;

/// No window: every system a cell meets is tallied (`cfg.flip` in the
/// query).
const NONE: u32 = u32::MAX;

impl DeviceField {
    /// The field's answers for `cells`, four numbers each: a closed cube's
    /// centre and half extent, in metres. A batch whose length is not a
    /// multiple of four, or with a non-finite number or a negative half
    /// extent, is refused whole.
    pub(in super::super) async fn query(
        &self,
        gpu: &Gpu,
        tools: &Tools,
        cells: &[f32],
    ) -> Result<Vec<Answer>> {
        let refused = || Error::InvalidInput("field query");
        if cells.len() % 4 != 0 || cells.chunks(4).any(|c| !c.iter().all(|v| v.is_finite())) {
            return Err(refused().into());
        }
        if cells.chunks(4).any(|c| c[3] < 0.0) {
            return Err(refused().into());
        }
        if cells.is_empty() {
            return Ok(Vec::new());
        }
        let words = self.ask(gpu, tools, cells, NONE).await?;
        let mut out: Vec<Answer> = words.chunks(4).map(answer).collect();
        let crowded: Vec<usize> = (words.chunks(4).enumerate())
            .filter(|(_, w)| w[0] & OVER != 0)
            .map(|(i, _)| i)
            .collect();
        if !crowded.is_empty() {
            let limbs = self.resolve(gpu, tools, cells, &crowded).await?;
            for (&i, limb) in crowded.iter().zip(limbs) {
                out[i].limb = limb;
            }
        }
        Ok(out)
    }

    /// The crowded `cells`' limbs, a window of `TALLY` systems at a time:
    /// each window's best system and its leaves, the most over every
    /// window, the lower name on a tie.
    async fn resolve(
        &self,
        gpu: &Gpu,
        tools: &Tools,
        cells: &[f32],
        crowded: &[usize],
    ) -> Result<Vec<Option<u32>>> {
        let some: Vec<f32> = crowded.iter().flat_map(|&i| cells[4 * i..4 * i + 4].to_vec()).collect();
        let mut best = vec![(None::<u32>, 0f32); crowded.len()];
        for window in (0..self.systems).step_by(TALLY as usize) {
            let words = self.ask(gpu, tools, &some, window).await?;
            for (b, w) in best.iter_mut().zip(words.chunks(4)) {
                let (system, leaves) = ((w[3] != u32::MAX).then_some(w[3]), f32::from_bits(w[2]));
                let better = match (system, b.0) {
                    (None, _) => false,
                    (Some(_), None) => true,
                    (Some(s), Some(held)) => leaves > b.1 || (leaves == b.1 && s < held),
                };
                if better {
                    *b = (system, leaves);
                }
            }
        }
        Ok(best.into_iter().map(|b| b.0).collect())
    }

    /// One query of `cells` at `window`, its four words a cell read back.
    async fn ask(&self, gpu: &Gpu, tools: &Tools, cells: &[f32], window: u32) -> Result<Vec<u32>> {
        let n = (cells.len() / 4) as u32;
        let bits: Vec<u32> = cells.iter().map(|v| v.to_bits()).collect();
        let at = storage(gpu, "field cells", bits.len() as u64, &bits)?;
        let answers = storage(gpu, "field answers", 4 * u64::from(n), &[])?;
        let mut r = Rec {
            gpu,
            held: Vec::new(),
        };
        let cfg = super::Cfg {
            flip: window,
            ..self.cfg
        };
        let mut e = gpu.encoder(&Default::default());
        let bufs = [&*self.prims, &*self.owner, &*self.index, &*at, &*answers];
        r.run(&mut e, (&tools.fielder.query, 0, n, &cfg), &bufs)?;
        gpu.submit([e.finish()]);
        let raw = io::read_async(gpu, &answers, 16 * u64::from(n)).await?;
        Ok(bytemuck::cast_slice(&raw).to_vec())
    }
}

fn answer(w: &[u32]) -> Answer {
    Answer {
        wood: w[0] & 1 != 0,
        wood_radius: f32::from_bits(w[1]),
        foliage: w[0] & 2 != 0,
        leaves: f32::from_bits(w[2]),
        limb: (w[3] != u32::MAX).then_some(w[3]),
    }
}
