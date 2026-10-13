//! The whole-tree folds over the drawn nodes, on the device (fn-212 task
//! 4; fn-256 task 3): `fold.wgsl` folds the records the cycle's dress
//! drew (`Dressed`) into the summaries the pipeline reads its rows from
//! (`executor::Folds`, the shoulder's bins and front, the leaf box). Only
//! those summaries are read back.
use super::super::io;
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor::{reach, Folds, BINS};

pub(super) const FOLD: &str = include_str!("fold.wgsl");
/// A drawn axis's and phytomer's words (`cycle/dress.wgsl`).
const CA: u32 = 12;
pub(in crate::generation) const CV: usize = 38;
pub(in crate::generation) const CA_WORDS: usize = CA as usize;
/// `fold.wgsl`'s sums: the folds, the widest node, the box, the kept
/// count, the widest node's id, then the bins.
const SUMS: usize = 16 + BINS;
/// The most crown nodes the sift hands on: the spruce at 80 keeps about
/// 4,000 across the whole tree (CONTRACT.md §4).
pub(super) const CAP: u32 = 1 << 16;

pub(super) struct Dresser {
    fold: io::Pass,
}

impl Dresser {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let fold = [true, false, true, false];
        let folds = ["extent", "widest", "crowns", "sift", "publish"];
        Self {
            fold: io::Pass::new(gpu, FOLD.to_string(), &fold, &folds),
        }
    }
}

/// A drawn wave, or the root: its buffer, its axes and records (`phyts`:
/// its laid phytomers' and, past the root, one join record per laid axis,
/// fn-244), and its records' first id.
pub(in crate::generation) struct Drawn {
    pub buffer: Owned,
    pub axes: u32,
    pub phyts: u32,
    pub base: u32,
    pub root: bool,
}

/// The root and the drawn waves, the ids they take, and the folds' sums.
pub(in crate::generation) struct Dressed {
    pub waves: Vec<Drawn>,
    pub ids: u32,
    sums: Owned,
}

impl Dressed {
    /// Drawn `waves` (the root's first) and the ids they take, nothing
    /// folded yet.
    pub(in crate::generation) fn new(gpu: &Gpu, waves: Vec<Drawn>, ids: u32) -> Result<Self> {
        let sums = storage(gpu, "fold sums", SUMS, bytemuck::cast_slice(&empty_sums()))?;
        Ok(Self { waves, ids, sums })
    }
}

fn storage(gpu: &Gpu, label: &'static str, words: usize, data: &[u8]) -> Result<Owned> {
    let usage =
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    io::buffer(gpu, label, (words.max(4) * 4) as u64, usage, data)
}

fn uniform(gpu: &Gpu, words: &[u32]) -> Result<Owned> {
    let usage = wgpu::BufferUsages::UNIFORM;
    io::buffer(
        gpu,
        "dress cfg",
        (words.len() * 4) as u64,
        usage,
        bytemuck::cast_slice(words),
    )
}

/// The sums before any node is folded.
fn empty_sums() -> Vec<u32> {
    let mut s = vec![0u32; SUMS];
    s[0] = 0x8000_0000;
    s[1] = u32::MAX;
    s[6..9].fill(u32::MAX);
    s[13] = u32::MAX;
    s
}

/// A float from its ordered bits (`fold.wgsl`'s `ord`).
fn unord(u: u32) -> f32 {
    f32::from_bits(if u & 0x8000_0000 != 0 {
        u ^ 0x8000_0000
    } else {
        !u
    })
}

fn v3(v: [f32; 3]) -> Vec3 {
    Vec3::new(f64::from(v[0]), f64::from(v[1]), f64::from(v[2]))
}

impl Dressed {
    /// Runs `entry` over every drawn wave (not the root), under `shape`
    /// and `reach`, with `beyond` and `kept` bound.
    fn fold(
        &self,
        gpu: &Gpu,
        d: &Dresser,
        entry: usize,
        rows: [[f32; 4]; 2],
        bound: (&wgpu::Buffer, &wgpu::Buffer),
    ) -> Result<()> {
        let mut encoder = gpu.encoder(&Default::default());
        for w in self.waves.iter().filter(|w| !w.root && w.phyts > 0) {
            let (buffer, axes, phyts) = (&w.buffer, &w.axes, &w.phyts);
            let groups = phyts.div_ceil(64);
            let most = gpu.device.limits().max_compute_workgroups_per_dimension;
            let mut cfg = vec![*phyts, axes * CA, CAP, groups.min(most) * 64];
            cfg.extend(rows.iter().flatten().map(|f| f.to_bits()));
            let cfg = uniform(gpu, &cfg)?;
            let bind = d
                .fold
                .bind(gpu, &[&cfg, buffer, &self.sums, bound.0, bound.1]);
            d.fold.run(&mut encoder, &bind, entry, groups);
        }
        gpu.submit([encoder.finish()]);
        Ok(())
    }

    async fn sums(&self, gpu: &Gpu) -> Result<Vec<u32>> {
        let raw = io::read_async(gpu, &self.sums, (SUMS * 4) as u64).await?;
        Ok(bytemuck::cast_slice(&raw).to_vec())
    }

    /// The device's folds: its nodes' highest, the crown's lowest and the
    /// widest (one round trip).
    pub(super) async fn extent(&self, gpu: &Gpu, d: &Dresser) -> Result<Folds> {
        let (past, kept) = (
            storage(gpu, "fold none", 4, &[])?,
            storage(gpu, "fold none", 4, &[])?,
        );
        for entry in [0, 1, 4] {
            self.fold(gpu, d, entry, [[0.0; 4]; 2], (&past, &kept))?;
        }
        let s = self.sums(gpu).await?;
        let widest = (s[2] != 0).then(|| {
            let at = v3([s[3], s[4], s[5]].map(f32::from_bits));
            (reach(at), at.y)
        });
        Ok(Folds {
            height: f64::from(unord(s[0])),
            low: if s[1] == u32::MAX {
                f64::INFINITY
            } else {
                f64::from(unord(s[1]))
            },
            widest,
        })
    }

    /// Under the whole tree's `shape` and a station's `reach` terms: the
    /// device's bins of the shoulder's way and its nodes' leaf box (one
    /// round trip).
    pub(super) async fn crowns(
        &self,
        gpu: &Gpu,
        d: &Dresser,
        shape: [f64; 4],
        terms: [f64; 4],
    ) -> Result<(Vec<f64>, (Vec3, Vec3))> {
        let (past, kept) = (
            storage(gpu, "fold none", 4, &[])?,
            storage(gpu, "fold none", 4, &[])?,
        );
        let rows = [shape.map(|v| v as f32), terms.map(|v| v as f32)];
        self.fold(gpu, d, 2, rows, (&past, &kept))?;
        let s = self.sums(gpu).await?;
        let bins = s[16..]
            .iter()
            .map(|&u| if u == 0 { 0.0 } else { f64::from(unord(u)) });
        let corner = |at: usize| v3([unord(s[at]), unord(s[at + 1]), unord(s[at + 2])]);
        Ok((bins.collect(), (corner(6), corner(9))))
    }

    /// The device's crown nodes no node beyond them (`beyond`, the whole
    /// tree's) passes, sifted: positions, y up (one round trip); and
    /// whether more were kept than handed on.
    pub(super) async fn sift(
        &self,
        gpu: &Gpu,
        d: &Dresser,
        shape: [f64; 4],
        beyond: &[f64],
    ) -> Result<(Vec<Vec3>, bool)> {
        let past: Vec<f32> = beyond.iter().map(|&b| b as f32).collect();
        let past = storage(gpu, "fold beyond", past.len(), bytemuck::cast_slice(&past))?;
        let kept = storage(gpu, "fold kept", 3 * CAP as usize, &[])?;
        let rows = [shape.map(|v| v as f32), [0.0; 4]];
        self.fold(gpu, d, 3, rows, (&past, &kept))?;
        let n = self.sums(gpu).await?[12];
        let taken = n.min(CAP) as usize;
        let raw = io::read_async(gpu, &kept, (taken.max(1) * 12) as u64).await?;
        let floats: &[f32] = bytemuck::cast_slice(&raw);
        let points = floats.chunks(3).take(taken).map(|p| v3([p[0], p[1], p[2]]));
        Ok((points.collect(), n > CAP))
    }
}

