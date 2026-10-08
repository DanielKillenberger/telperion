//! Fine roots in the tree's frame, and the whole-tree folds over them, on
//! the device (fn-212 task 4; CONTRACT.md's probe, steps 2 and 5). After
//! the GPU executor's last phase each root's laid geometry is its own,
//! metres from its bearer's node: `dress.wgsl` draws it as stage 2's
//! conversion draws it, node for node, in the tree's frame, from each
//! root's attachment as the conversion drew its bearer (`Stub`). Then
//! `fold.wgsl` folds the drawn nodes into the summaries the pipeline
//! combines with its own (`executor::Folds`, the shoulder's bins and
//! front, the leaf box). Only those summaries are read back.
use super::super::io;
use super::device::Device;
use super::laid::{Wave, AL};
use crate::{Gpu, Result};
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor::{reach, Folds, BINS};

pub(super) const DRESS: &str = include_str!("dress.wgsl");
pub(super) const FOLD: &str = include_str!("fold.wgsl");
/// A drawn axis's and phytomer's floats (`dress.wgsl`).
const CA: u32 = 12;
pub(super) const CV: usize = 26;
pub(super) const CA_WORDS: usize = CA as usize;
pub(super) const MADE: u32 = 2;
pub(super) const LANDED: u32 = 4;
const TAKEN: u32 = 1;
const PREV: u32 = 0x8000_0000;
/// `fold.wgsl`'s sums: the folds, the widest node, the box, the kept
/// count, then the bins.
const SUMS: usize = 16 + BINS;
/// The most crown nodes the sift hands on: the spruce at 80 keeps about
/// 4,000 across the whole tree (CONTRACT.md §4).
pub(super) const CAP: u32 = 1 << 16;

/// A fine root's attachment: the node its bearer's phytomer is drawn as
/// (none where it is not drawn: the root is not drawn either), its
/// bearer's node as laid (its origin, z up) and whether it is laid on
/// the CPU (host decision 3).
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Stub {
    pub node: Option<StubNode>,
    pub origin: [f32; 3],
    pub landed: bool,
    /// Its bearer's node's distance along the wood from the tree's root.
    pub distance: f32,
}

/// A drawn node as the conversion reads it: its position and its
/// parent's (y up), its radius, start radius and base radius.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct StubNode {
    pub position: [f32; 3],
    pub parent: [f32; 3],
    pub radius: f32,
    pub start: f32,
    pub base: f32,
    /// Its wood's frame as laid: heading, then side (y up).
    pub frame: [f32; 6],
    /// Its branch (its branch's first node, by the pipeline's index) and
    /// its key (`Tree::key`).
    pub branch: u32,
    pub key: u32,
}

pub(super) struct Dresser {
    dress: io::Pass,
    fold: io::Pass,
}

impl Dresser {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let dress = [true, true, true, true, true, true, false, true];
        let fold = [true, false, true, false];
        let folds = ["extent", "widest", "crowns", "sift"];
        Self {
            dress: io::Pass::new(gpu, DRESS.to_string(), &dress, &["dress"]),
            fold: io::Pass::new(gpu, FOLD.to_string(), &fold, &folds),
        }
    }
}

/// A drawn wave, or a group's stubs: its buffer, its axes and records,
/// and its records' first id.
pub(super) struct Drawn {
    pub buffer: wgpu::Buffer,
    pub axes: u32,
    pub phyts: u32,
    pub base: u32,
    pub stubs: bool,
}

/// Every group's stubs and drawn waves, the ids they take, and the
/// folds' sums.
pub(super) struct Dressed {
    pub waves: Vec<Drawn>,
    pub ids: u32,
    sums: wgpu::Buffer,
}

/// Destroyed when let go: a browser frees a buffer only then (fn-212
/// task 10; wgpu's web backend drops nothing).
impl Drop for Dressed {
    fn drop(&mut self) {
        self.waves.iter().for_each(|w| w.buffer.destroy());
        self.sums.destroy();
    }
}

fn storage(gpu: &Gpu, label: &'static str, words: usize, data: &[u8]) -> wgpu::Buffer {
    let usage =
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    io::buffer(gpu, label, (words.max(4) * 4) as u64, usage, data).unwrap()
}

fn uniform(gpu: &Gpu, words: &[u32]) -> wgpu::Buffer {
    let usage = wgpu::BufferUsages::UNIFORM;
    io::buffer(
        gpu,
        "dress cfg",
        (words.len() * 4) as u64,
        usage,
        bytemuck::cast_slice(words),
    )
    .unwrap()
}

/// The stubs' drawn records, by the roots' places in their group, their
/// ids from `base`.
fn stub_records(stubs: &[Stub], base: u32) -> Vec<f32> {
    let n = stubs.len();
    let mut out = vec![0.0f32; n * CA as usize + n * CV];
    let at = n * CA as usize;
    for (k, s) in stubs.iter().enumerate() {
        let a = &mut out[k * CA as usize..][..CA as usize];
        a[3] = f32::from_bits(if s.landed { LANDED } else { 0 });
        a[4..7].copy_from_slice(&s.origin);
        a[7] = s.distance;
        let r = &mut out[at + k * CV..][..CV];
        r[9] = f32::from_bits(k as u32);
        r[14] = f32::from_bits(base + k as u32);
        r[16] = f32::from_bits(u32::MAX);
        let Some(node) = s.node else {
            continue;
        };
        r[..3].copy_from_slice(&node.position);
        (r[3], r[4], r[5]) = (node.radius, node.start, node.base);
        r[6] = f32::from_bits(TAKEN | MADE);
        r[7] = f32::from_bits(k as u32 | PREV);
        r[10..13].copy_from_slice(&node.parent);
        r[17..23].copy_from_slice(&node.frame);
        r[23] = f32::from_bits(node.branch | PREV);
        r[24] = f32::from_bits(node.key);
    }
    out
}

/// Draws every group's fine roots from `stubs` (by the tree's roots) as
/// the pipeline's nodes, wave by wave.
pub(super) fn dress(gpu: &Gpu, d: &Dresser, dev: &Device, stubs: &[Stub]) -> Dressed {
    let mut waves = Vec::new();
    let mut ids = 0u32;
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    for (grown, laid, roots) in dev.groups() {
        let mine: Vec<Stub> = roots.iter().map(|&r| stubs[r]).collect();
        let records = stub_records(&mine, ids);
        let mut bases = vec![ids];
        ids += mine.len() as u32;
        for w in &laid[1..] {
            bases.push(ids);
            ids += w.phyts;
        }
        let mut bufs = vec![storage(
            gpu,
            "drawn stubs",
            records.len(),
            bytemuck::cast_slice(&records),
        )];
        for (w, wave) in grown.grown.iter().enumerate() {
            let (was, now) = (&laid[w], &laid[w + 1]);
            let words = now.axes as usize * CA as usize + now.phyts as usize * CV;
            bufs.push(storage(gpu, "drawn", words, &[]));
            for run in &wave.runs {
                let groups = run.n.div_ceil(64);
                let most = gpu.device.limits().max_compute_workgroups_per_dimension;
                let cfg = [
                    run.n,
                    groups.min(most) * 64,
                    now.axes * AL as u32,
                    was.axes * AL as u32,
                    now.axes * CA,
                    was.axes * CA,
                    bases[w + 1],
                    bases[w],
                ];
                let cfg = uniform(gpu, &cfg);
                let all = [
                    &cfg,
                    &run.items,
                    &run.axes,
                    &run.scratch,
                    &was.buffer,
                    &now.buffer,
                    &bufs[w],
                    &bufs[w + 1],
                    dev.table(),
                ];
                let bind = d.dress.bind(gpu, &all);
                d.dress.run(&mut encoder, &bind, 0, groups);
            }
        }
        let sized = laid.iter().map(|w: &Wave| (w.axes, w.phyts));
        let all = bufs.into_iter().zip(sized).zip(bases).enumerate();
        waves.extend(all.map(|(w, ((buffer, (axes, phyts)), base))| Drawn {
            buffer,
            axes,
            phyts,
            base,
            stubs: w == 0,
        }));
    }
    gpu.queue.submit([encoder.finish()]);
    let sums = storage(gpu, "fold sums", SUMS, bytemuck::cast_slice(&empty_sums()));
    Dressed { waves, ids, sums }
}

/// The sums before any node is folded.
fn empty_sums() -> Vec<u32> {
    let mut s = vec![0u32; SUMS];
    s[0] = 0x8000_0000;
    s[1] = u32::MAX;
    s[6..9].fill(u32::MAX);
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
    /// Runs `entry` over every drawn wave (not the stubs), under `shape`
    /// and `reach`, with `beyond` and `kept` bound.
    fn fold(
        &self,
        gpu: &Gpu,
        d: &Dresser,
        entry: usize,
        rows: [[f32; 4]; 2],
        bound: (&wgpu::Buffer, &wgpu::Buffer),
    ) {
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        for w in self.waves.iter().filter(|w| !w.stubs && w.phyts > 0) {
            let (buffer, axes, phyts) = (&w.buffer, &w.axes, &w.phyts);
            let groups = phyts.div_ceil(64);
            let most = gpu.device.limits().max_compute_workgroups_per_dimension;
            let mut cfg = vec![*phyts, axes * CA, CAP, groups.min(most) * 64];
            cfg.extend(rows.iter().flatten().map(|f| f.to_bits()));
            let cfg = uniform(gpu, &cfg);
            let bind = d
                .fold
                .bind(gpu, &[&cfg, buffer, &self.sums, bound.0, bound.1]);
            d.fold.run(&mut encoder, &bind, entry, groups);
        }
        gpu.queue.submit([encoder.finish()]);
    }

    async fn sums(&self, gpu: &Gpu) -> Result<Vec<u32>> {
        let raw = io::read_async(gpu, &self.sums, (SUMS * 4) as u64).await?;
        Ok(bytemuck::cast_slice(&raw).to_vec())
    }

    /// The device's folds: its nodes' highest, the crown's lowest and the
    /// widest (one round trip).
    pub(super) async fn extent(&self, gpu: &Gpu, d: &Dresser) -> Result<Folds> {
        let (past, kept) = (
            storage(gpu, "fold none", 4, &[]),
            storage(gpu, "fold none", 4, &[]),
        );
        for entry in [0, 1] {
            self.fold(gpu, d, entry, [[0.0; 4]; 2], (&past, &kept));
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
            storage(gpu, "fold none", 4, &[]),
            storage(gpu, "fold none", 4, &[]),
        );
        let rows = [shape.map(|v| v as f32), terms.map(|v| v as f32)];
        self.fold(gpu, d, 2, rows, (&past, &kept));
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
        let past = storage(gpu, "fold beyond", past.len(), bytemuck::cast_slice(&past));
        let kept = storage(gpu, "fold kept", 3 * CAP as usize, &[]);
        let rows = [shape.map(|v| v as f32), [0.0; 4]];
        self.fold(gpu, d, 3, rows, (&past, &kept));
        let n = self.sums(gpu).await?[12];
        let taken = n.min(CAP) as usize;
        let raw = io::read_async(gpu, &kept, (taken.max(1) * 12) as u64).await?;
        let floats: &[f32] = bytemuck::cast_slice(&raw);
        let points = floats.chunks(3).take(taken).map(|p| v3([p[0], p[1], p[2]]));
        Ok((points.collect(), n > CAP))
    }
}
