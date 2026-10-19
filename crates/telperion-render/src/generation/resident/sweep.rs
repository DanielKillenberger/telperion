//! The tree's wood as curves on the device (fn-212 task 5; fn-256 task
//! 3): F3's points, runs, clusters, their bounds, radius extrema and error
//! ladders over the nodes the cycle's dress drew, read in its records by
//! id, the runs widest first, written in the renderer's own curve form
//! (fn-293: no pass repacks them). Only totals are read back: the runs,
//! then the points and clusters, to size what follows, then the curve's
//! demand and its box.
use super::super::{compute, io};
use super::fold::Dressed;
use super::stations::Laid;
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_core::surface::{Bounds, Demand, SurfaceParams, CLUSTER_WORDS, POINT_WORDS};

const SWEEP: &str = include_str!("sweep.wgsl");
/// The renderer's packed point (`CurvePoint::pack`) written and read on
/// the device.
pub(super) const PACK: &str = include_str!("pack.wgsl");
const LW: u64 = 12;
/// A run's place in the table.
pub(super) const TW: usize = 4;
const UNLINKED: usize = 0;
const LEAD: usize = 1;
const TIES: usize = 2;
const STARTS: usize = 3;
const WALK: usize = 4;
const BITONIC: usize = 5;
const BLOCK: usize = 6;
const BLOCKS: usize = 7;
const PLACE: usize = 8;
const EMIT: usize = 9;
const TOTAL: usize = 10;
/// The curve's sums (`sweep.wgsl`'s `counts` in `emit`): the stretches,
/// the box's six corners, a pad and 256 lanes' partial sums of the sags;
/// each cluster's sag follows.
const SUMS: usize = 8 + 256;

pub(super) fn source() -> String {
    format!("{PACK}\n{SWEEP}")
}

pub(super) struct Sweeper {
    pass: io::Pass,
}

impl Sweeper {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let read = [true, true, false, false, false, false, false, false];
        let entries = [
            "unlinked", "lead", "ties", "starts", "walk", "bitonic", "block", "blocks", "place",
            "emit", "total",
        ];
        Self {
            pass: io::Pass::new(gpu, source(), &read, &entries),
        }
    }
}

/// The surface's rows as `sweep.wgsl` reads them, at the wood's `height`.
pub(super) fn rows(p: &SurfaceParams, height: f64) -> [[f32; 4]; 3] {
    let segments = p.radial_segments.max(p.lobes * 4);
    let cos = (std::f64::consts::PI / f64::from(segments)).cos();
    let twisting = p.lobes > 0 && p.lobe_depth != 0.0 && p.twist_rate != 0.0;
    let rate = std::f64::consts::TAU * p.twist_rate / height;
    let (twist, depth) = match twisting {
        true => (f64::from(p.lobes) * rate.abs(), p.lobe_depth.abs()),
        false => (0.0, 0.0),
    };
    let lobe = 1.0 + p.lobe_depth.abs() * f64::from(u8::from(p.lobes > 0));
    [
        [p.lobe_depth, cos, p.fork_swell, p.fork_socket],
        [
            p.socket_containment,
            p.flare_radius,
            p.flare_falloff,
            height,
        ],
        [twist, depth, lobe, p.flare_depth * height],
    ]
    .map(|r| r.map(|v| v as f32))
}

/// The tree's curve as the renderer takes it, and what the stations and
/// rings read of the sweep.
pub(super) struct Swept {
    pub runs: u32,
    pub points: u32,
    pub clusters: u32,
    /// Each run's place (`TW` words), each block's totals, then from word
    /// `contacts` each node's place along its run and the run's.
    pub table: Owned,
    pub contacts: u32,
    /// The packed points and clusters (`POINT_WORDS`, `CLUSTER_WORDS`).
    pub point_buffer: Owned,
    pub cluster_buffer: Owned,
    /// What the most any view could ask of the curve rests on, and the
    /// box round every ring; none where no run was swept.
    pub demand: Demand,
    pub bounds: Option<Bounds>,
}

impl Swept {
    /// What the stations read of `dressed` swept: its records, and the
    /// table with the nodes' contacts.
    pub(super) fn laid<'a>(&'a self, dressed: &'a Dressed) -> Laid<'a> {
        Laid {
            records: &dressed.records,
            table: &self.table,
            contacts: self.contacts,
        }
    }
}

fn storage(gpu: &Gpu, label: &'static str, words: u64) -> Result<Owned> {
    io::buffer(gpu, label, words.max(4) * 4, USAGE, &[])
}

const USAGE: wgpu::BufferUsages = wgpu::BufferUsages::STORAGE
    .union(wgpu::BufferUsages::COPY_SRC)
    .union(wgpu::BufferUsages::COPY_DST);

/// A block of the renderer's curve: `n` items of `w` words, its bytes on
/// 256-byte boundaries as the curve's views bind them (`curve/target.rs`).
fn curve_block(gpu: &Gpu, label: &'static str, n: u32, w: usize) -> Result<Owned> {
    let bytes = (u64::from(n) * w as u64 * 4).max(256).next_multiple_of(256);
    io::buffer(gpu, label, bytes, USAGE, &[])
}

/// The buffers every pass binds.
struct Bound<'a> {
    drawn: &'a Dressed,
    cells: &'a wgpu::Buffer,
    links: Owned,
    counts: Owned,
    keys: Owned,
    table: Owned,
    points: Owned,
    clusters: Owned,
}

impl Sweeper {
    /// Runs `entry` over `n` items (`groups` workgroups where given).
    fn run(
        &self,
        (gpu, encoder): (&Gpu, &mut wgpu::CommandEncoder),
        b: &Bound<'_>,
        (entry, n, groups): (usize, u32, Option<u32>),
        cfg: [u32; 8],
        rows: [[f32; 4]; 3],
    ) -> Result<()> {
        let groups = groups.unwrap_or(n.div_ceil(64)).max(1);
        let most = gpu.device.limits().max_compute_workgroups_per_dimension;
        let mut words = cfg.to_vec();
        words[0] = n;
        words[1] = groups.min(most) * 64;
        words[4] = b.drawn.axes;
        words[6] = b.drawn.ids;
        words.extend(rows.iter().flatten().map(|f| f.to_bits()));
        let usage = wgpu::BufferUsages::UNIFORM;
        let cfg = io::buffer(gpu, "sweep cfg", 80, usage, bytemuck::cast_slice(&words))?;
        let all = [
            &cfg,
            &b.drawn.records,
            b.cells,
            &b.links,
            &b.counts,
            &b.keys,
            &b.table,
            &b.points,
            &b.clusters,
        ];
        let bind = self.pass.bind(gpu, &all);
        self.pass.run(encoder, &bind, entry, groups);
        Ok(())
    }

    /// The passes recorded so far submitted, then `words` of `counts` read
    /// back.
    async fn read(
        gpu: &Gpu,
        encoder: &mut crate::Encoder,
        counts: &wgpu::Buffer,
        words: usize,
    ) -> Result<Vec<u32>> {
        let done = std::mem::replace(encoder, gpu.encoder(&Default::default()));
        gpu.submit([done.finish()]);
        let raw = io::read_async(gpu, counts, (words * 4) as u64).await?;
        Ok(bytemuck::cast_slice(&raw).to_vec())
    }

    /// Sweeps the curve of the nodes `dressed` drew, under the surface's
    /// `rows`, its retained leaf bases on their `cells` where they pack.
    pub(super) async fn sweep(
        &self,
        gpu: &Gpu,
        dressed: &Dressed,
        rows: [[f32; 4]; 3],
        cells: Option<&Owned>,
    ) -> Result<Swept> {
        let no_cells = storage(gpu, "sweep no cells", 4)?;
        let ids = u64::from(dressed.ids);
        let mut b = Bound {
            drawn: dressed,
            cells: cells.map_or(&*no_cells, |c| &**c),
            links: storage(gpu, "sweep links", ids * LW)?,
            counts: storage(gpu, "sweep counts", 4)?,
            keys: storage(gpu, "sweep keys", 2 * ids.next_power_of_two())?,
            table: storage(gpu, "sweep table", 4)?,
            points: storage(gpu, "sweep points", 4)?,
            clusters: storage(gpu, "sweep clusters", 4)?,
        };
        let z = [0u32; 8];
        let mut e = gpu.encoder(&Default::default());
        let n = dressed.ids;
        self.run((gpu, &mut e), &b, (UNLINKED, n, None), z, rows)?;
        self.run((gpu, &mut e), &b, (LEAD, n, None), z, rows)?;
        for stage in 0..4 {
            let cfg = [0, 0, stage, 0, 0, 0, 0, 0];
            self.run((gpu, &mut e), &b, (TIES, n, None), cfg, rows)?;
        }
        self.run((gpu, &mut e), &b, (STARTS, n, None), z, rows)?;
        let runs = Self::read(gpu, &mut e, &b.counts, 4).await?[0];
        let wide = runs.next_power_of_two().max(2);
        let pad = vec![u32::MAX; 2 * (wide - runs) as usize];
        gpu.queue
            .write_buffer(&b.keys, u64::from(runs) * 8, bytemuck::cast_slice(&pad));
        self.run((gpu, &mut e), &b, (WALK, runs, None), z, rows)?;
        let mut k = 2;
        while k <= wide {
            let mut j = k / 2;
            while j > 0 {
                let cfg = [0, 0, j, k, 0, 0, 0, 0];
                self.run((gpu, &mut e), &b, (BITONIC, wide, None), cfg, rows)?;
                j /= 2;
            }
            k *= 2;
        }
        let blocks = runs.div_ceil(256);
        let contacts = runs * TW as u32 + 2 * blocks;
        b.table = storage(gpu, "sweep table", u64::from(contacts) + 2 * ids)?;
        self.run((gpu, &mut e), &b, (BLOCK, runs, Some(blocks)), z, rows)?;
        self.run((gpu, &mut e), &b, (BLOCKS, runs, Some(1)), z, rows)?;
        self.run((gpu, &mut e), &b, (PLACE, runs, None), z, rows)?;
        let counted = Self::read(gpu, &mut e, &b.counts, 4).await?;
        let (points, clusters) = (counted[1], counted[2]);
        b.points = curve_block(gpu, "curve points", points, POINT_WORDS)?;
        b.clusters = curve_block(gpu, "curve clusters", clusters, CLUSTER_WORDS)?;
        let mut start = [0u32; SUMS];
        start[1..4].fill(u32::MAX);
        let words = (SUMS as u64 + u64::from(clusters)) * 4;
        b.counts = io::buffer(
            gpu,
            "curve sums",
            words,
            USAGE,
            bytemuck::cast_slice(&start),
        )?;
        let cfg = [0, 0, 0, 0, 0, contacts, 0, 0];
        self.run((gpu, &mut e), &b, (EMIT, runs, None), cfg, rows)?;
        self.run((gpu, &mut e), &b, (TOTAL, clusters, Some(1)), z, rows)?;
        let s = Self::read(gpu, &mut e, &b.counts, SUMS).await?;
        for done in [&b.links, &b.counts, &b.keys] {
            done.destroy();
        }
        let mut demand = Demand::default();
        demand.clusters += u64::from(clusters);
        demand.stretches += u64::from(s[0]);
        demand.root_sag += s[8..]
            .iter()
            .map(|&w| f64::from(f32::from_bits(w)))
            .sum::<f64>();
        Ok(Swept {
            runs,
            points,
            clusters,
            table: b.table,
            contacts,
            point_buffer: b.points,
            cluster_buffer: b.clusters,
            demand,
            bounds: (clusters > 0).then(|| compute::bounds(&s, 1)),
        })
    }
}
