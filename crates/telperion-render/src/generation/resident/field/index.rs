//! The grid of buckets a field's primitives are listed in: each
//! primitive in every bucket its box meets, each bucket's list in its
//! primitives' order.
use super::super::super::io;
use super::super::build::Tools;
use super::{read_words, storage, unbound, Cfg, Rec};
use super::{COUNT, FILL, SORTED};
use crate::buffer::Owned;
use crate::{Gpu, Result};

/// The primitives a bucket holds on average, and the most buckets a grid
/// takes: fewer, fuller buckets walk faster than many near-empty ones
/// under a cell's reach (the spruce's 36-cell query 84 ms at 4, 62 at 16,
/// 66 at 64), and the grid's words stay small.
const PER_BUCKET: f64 = 16.0;
const MOST_BUCKETS: f64 = (1u32 << 22) as f64;

/// The bucket grid over the field's box `b`: about `PER_BUCKET`
/// primitives a bucket, at most `MOST_BUCKETS`.
pub(super) fn grid(cfg: &mut Cfg, b: &[f32; 6], prims: u64) {
    let extent: [f64; 3] = std::array::from_fn(|k| f64::from(b[k + 3] - b[k]).max(1e-3));
    let volume: f64 = extent.iter().product();
    let buckets = (prims as f64 / PER_BUCKET).clamp(1.0, MOST_BUCKETS);
    let mut side = (volume / buckets).cbrt();
    let dims = |side: f64| extent.map(|e| ((e / side).ceil() as u32).max(1));
    while dims(side).iter().map(|&d| f64::from(d)).product::<f64>() > MOST_BUCKETS {
        side *= 1.05;
    }
    let d = dims(side);
    cfg.dims = [d[0], d[1], d[2], prims as u32];
    cfg.buckets = d[0] * d[1] * d[2];
    cfg.lo = [b[0], b[1], b[2], side as f32];
}

/// Each primitive listed in the buckets its box meets: counted, scanned
/// into each bucket's first place, filled and put in order. The index's
/// words: the buckets' firsts and the total, their fills, the lists.
pub(super) async fn index(
    gpu: &Gpu,
    tools: &Tools,
    cfg: &Cfg,
    (prim, owner): (&Owned, &Owned),
) -> Result<Owned> {
    let mut r = Rec {
        gpu,
        held: Vec::new(),
    };
    let b = cfg.buckets;
    let n = unbound(gpu)?;
    let counts = storage(gpu, "field bucket counts", u64::from(b) + 1, &[])?;
    let mut e = gpu.encoder(&Default::default());
    let prims = cfg.dims[3];
    let f = &tools.fielder.build;
    let on = [&*n[0], &n[1], &n[2], prim, owner, &counts, &n[3]];
    r.run(&mut e, (f, COUNT, prims, cfg), &on)?;
    let total = storage(gpu, "field bucket total", 4, &[])?;
    tools.stationer.scan(gpu, &mut e, (&counts, b, 1, 1), &total)?;
    gpu.submit([e.finish()]);
    let listed = read_words(gpu, &total, 0, 1).await?[0];
    let words = 2 * u64::from(b) + 1 + u64::from(listed);
    let index = storage(gpu, "field index", words, &[])?;
    let mut e = gpu.encoder(&Default::default());
    e.copy_buffer_to_buffer(&counts, 0, &index, 0, u64::from(b) * 4);
    e.copy_buffer_to_buffer(&total, 0, &index, u64::from(b) * 4, 4);
    let on = [&*n[0], &n[1], &n[2], prim, owner, &index, &n[3]];
    for (entry, items) in [(FILL, prims), (SORTED, b)] {
        r.run(&mut e, (f, entry, items, cfg), &on)?;
    }
    gpu.submit([e.finish()]);
    io::complete(gpu).await?;
    Ok(index)
}
