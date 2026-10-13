//! The tree's wood as curves on the device (fn-212 task 5; fn-256 task
//! 3): F3's points, runs, clusters, their bounds, radius extrema and error
//! ladders over the nodes the cycle's dress drew, the runs widest first. Only totals are read back:
//! the runs, then the points and clusters, to size what follows.
use super::super::io;
use super::fold::{Dressed, CA_WORDS};
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_core::surface::SurfaceParams;

const SWEEP: &str = include_str!("sweep.wgsl");
pub(super) const NW: u64 = 36;
const LW: u64 = 12;
/// A run's place in the table, a point's and a cluster's words.
pub(super) const TW: usize = 4;
pub(super) const PW: usize = 11;
pub(super) const KW: usize = 16;
const GATHER: usize = 0;
const LEAD: usize = 1;
const TIES: usize = 2;
const STARTS: usize = 3;
const WALK: usize = 4;
const BITONIC: usize = 5;
const BLOCK: usize = 6;
const BLOCKS: usize = 7;
const PLACE: usize = 8;
const EMIT: usize = 9;

pub(super) fn source() -> String {
    SWEEP.to_string()
}

pub(super) struct Sweeper {
    pass: io::Pass,
}

impl Sweeper {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let read = [true, false, false, false, false, false, false, false];
        let entries = [
            "gather", "lead", "ties", "starts", "walk", "bitonic", "block", "blocks", "place",
            "emit",
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

pub(super) struct Swept {
    pub runs: u32,
    pub points: u32,
    pub clusters: u32,
    pub table: Owned,
    pub point_buffer: Owned,
    pub cluster_buffer: Owned,
    /// Every drawn node by id (`sweep.wgsl`'s `NW` words), for the stations.
    pub nodes: Owned,
}

fn storage(gpu: &Gpu, label: &'static str, words: u64) -> Result<Owned> {
    let usage =
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    io::buffer(gpu, label, words.max(4) * 4, usage, &[])
}

/// The buffers every pass binds.
struct Bound<'a> {
    nodes: Owned,
    links: Owned,
    counts: Owned,
    keys: Owned,
    table: Owned,
    points: Owned,
    clusters: Owned,
    /// What every pass after `gather` binds in its records' place: the
    /// bases' cells, or one unread word.
    cells: &'a wgpu::Buffer,
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
        drawn: Option<&wgpu::Buffer>,
    ) -> Result<()> {
        let groups = groups.unwrap_or(n.div_ceil(64)).max(1);
        let most = gpu.device.limits().max_compute_workgroups_per_dimension;
        let mut words = cfg.to_vec();
        words[0] = n;
        words[1] = groups.min(most) * 64;
        words.extend(rows.iter().flatten().map(|f| f.to_bits()));
        let usage = wgpu::BufferUsages::UNIFORM;
        let cfg = io::buffer(gpu, "sweep cfg", 80, usage, bytemuck::cast_slice(&words))?;
        let all = [
            &cfg,
            drawn.unwrap_or(b.cells),
            &b.nodes,
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

    /// The passes recorded so far submitted, then the counts read back.
    async fn counts(gpu: &Gpu, encoder: &mut crate::Encoder, b: &Bound<'_>) -> Result<[u32; 4]> {
        let done = std::mem::replace(encoder, gpu.encoder(&Default::default()));
        gpu.submit([done.finish()]);
        let raw = io::read_async(gpu, &b.counts, 16).await?;
        let w: &[u32] = bytemuck::cast_slice(&raw);
        Ok([w[0], w[1], w[2], w[3]])
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
        let ids = u64::from(dressed.ids);
        let no_cells = storage(gpu, "sweep no cells", 4)?;
        let mut b = Bound {
            nodes: storage(gpu, "sweep nodes", ids * NW)?,
            links: storage(gpu, "sweep links", ids * LW)?,
            counts: storage(gpu, "sweep counts", 4)?,
            keys: storage(gpu, "sweep keys", 2 * ids.next_power_of_two())?,
            table: storage(gpu, "sweep table", 4)?,
            points: storage(gpu, "sweep points", 4)?,
            clusters: storage(gpu, "sweep clusters", 4)?,
            cells: cells.map_or(&*no_cells, |c| &**c),
        };
        let z = [0u32; 8];
        let mut e = gpu.encoder(&Default::default());
        for w in &dressed.waves {
            let records = w.axes * CA_WORDS as u32;
            let cfg = [0, 0, 0, 0, records, w.base, u32::from(w.root), 0];
            self.run(
                (gpu, &mut e),
                &b,
                (GATHER, w.phyts, None),
                cfg,
                rows,
                Some(&w.buffer),
            )?;
        }
        self.run((gpu, &mut e), &b, (LEAD, dressed.ids, None), z, rows, None)?;
        for stage in 0..4 {
            let cfg = [0, 0, stage, 0, 0, 0, 0, 0];
            self.run(
                (gpu, &mut e),
                &b,
                (TIES, dressed.ids, None),
                cfg,
                rows,
                None,
            )?;
        }
        self.run(
            (gpu, &mut e),
            &b,
            (STARTS, dressed.ids, None),
            z,
            rows,
            None,
        )?;
        let runs = Self::counts(gpu, &mut e, &b).await?[0];
        let wide = runs.next_power_of_two().max(2);
        let pad = vec![u32::MAX; 2 * (wide - runs) as usize];
        gpu.queue
            .write_buffer(&b.keys, u64::from(runs) * 8, bytemuck::cast_slice(&pad));
        self.run((gpu, &mut e), &b, (WALK, runs, None), z, rows, None)?;
        let mut k = 2;
        while k <= wide {
            let mut j = k / 2;
            while j > 0 {
                self.run(
                    (gpu, &mut e),
                    &b,
                    (BITONIC, wide, None),
                    [0, 0, j, k, 0, 0, 0, 0],
                    rows,
                    None,
                )?;
                j /= 2;
            }
            k *= 2;
        }
        let blocks = runs.div_ceil(256);
        b.table = storage(
            gpu,
            "sweep table",
            u64::from(runs) * TW as u64 + 2 * u64::from(blocks),
        )?;
        self.run(
            (gpu, &mut e),
            &b,
            (BLOCK, runs, Some(blocks)),
            z,
            rows,
            None,
        )?;
        self.run((gpu, &mut e), &b, (BLOCKS, runs, Some(1)), z, rows, None)?;
        self.run((gpu, &mut e), &b, (PLACE, runs, None), z, rows, None)?;
        let [_, points, clusters, _] = Self::counts(gpu, &mut e, &b).await?;
        b.points = storage(gpu, "sweep points", u64::from(points) * PW as u64)?;
        b.clusters = storage(gpu, "sweep clusters", u64::from(clusters) * KW as u64)?;
        self.run((gpu, &mut e), &b, (EMIT, runs, None), z, rows, None)?;
        gpu.submit([e.finish()]);
        io::complete(gpu).await?;
        for done in [&b.links, &b.counts, &b.keys] {
            done.destroy();
        }
        Ok(Swept {
            runs,
            points,
            clusters,
            table: b.table,
            point_buffer: b.points,
            cluster_buffer: b.clusters,
            nodes: b.nodes,
        })
    }
}
