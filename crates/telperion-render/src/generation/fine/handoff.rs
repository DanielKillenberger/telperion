//! The fine roots' curve handed to the renderer (fn-212 task 7, host
//! decision 2): the device packs the fine block into the renderer's own
//! curve form, after the CPU block's points and clusters (D5's order), so
//! the renderer reads one block and learns nothing new. Read back: the
//! fine block's stretches, bounds and 256 partial sums of its sags, which
//! the curve's demand and the tree's bounds fold in.
use super::super::{compute, io};
use super::sweep::Swept;
use crate::buffer::Owned;
use crate::curve::CurveGpu;
use crate::{Gpu, Result};
use telperion_core::math::Vec3;
use telperion_core::surface::{Bounds, Curve, CLUSTER_WORDS, POINT_WORDS};

const HANDOFF: &str = include_str!("handoff.wgsl");
const PACK: usize = 0;
const CLUSTERS: usize = 1;
const TOTAL: usize = 2;
/// The stretches, six corners, a pad, and 256 partial sums.
const SUMS: usize = 8 + 256;

pub(super) fn source() -> String {
    HANDOFF.to_string()
}

pub(super) struct Handoff {
    pass: io::Pass,
}

/// The whole tree's curve on the device, and the fine block's bounds.
pub(super) struct Handed {
    pub curve: CurveGpu,
    pub fine: Option<Bounds>,
}

const USAGE: wgpu::BufferUsages = wgpu::BufferUsages::STORAGE
    .union(wgpu::BufferUsages::COPY_SRC)
    .union(wgpu::BufferUsages::COPY_DST);

impl Handoff {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let read = [true, true, true, false, false, false, false];
        let entries = ["pack", "pack_clusters", "total"];
        Self {
            pass: io::Pass::new(gpu, source(), &read, &entries),
        }
    }

    /// `cpu`'s block and `swept`'s on the device as one curve, the fine
    /// block's after the CPU's.
    pub(super) async fn hand(&self, gpu: &Gpu, cpu: &Curve, swept: &Swept) -> Result<Handed> {
        let (p0, k0) = (cpu.points.len() as u64, cpu.clusters.len() as u64);
        let (p, k) = (p0 + u64::from(swept.points), k0 + u64::from(swept.clusters));
        // Bytes on 256-byte boundaries, as the curve's own buffers are: its
        // views bind slices of at least that (`curve/target.rs`).
        let words = |n: u64, w: usize| (n * w as u64 * 4).max(256).next_multiple_of(256);
        let points = io::buffer(gpu, "curve points", words(p, POINT_WORDS), USAGE, &[]);
        let clusters = io::buffer(gpu, "curve clusters", words(k, CLUSTER_WORDS), USAGE, &[]);
        let (points, clusters) = (points?, clusters?);
        let packed = cpu.packed();
        gpu.queue
            .write_buffer(&points, 0, bytemuck::cast_slice(&packed));
        drop(packed);
        let packed = cpu.packed_clusters();
        gpu.queue
            .write_buffer(&clusters, 0, bytemuck::cast_slice(&packed));
        drop(packed);
        let mut start = [0u32; SUMS];
        start[1..4].fill(u32::MAX);
        let sums = io::buffer(
            gpu,
            "handoff sums",
            (SUMS * 4) as u64,
            USAGE,
            bytemuck::cast_slice(&start),
        );
        let sags = io::buffer(
            gpu,
            "handoff sags",
            words(u64::from(swept.clusters), 1),
            USAGE,
            &[],
        );
        let (sums, sags) = (sums?, sags?);
        let (twist, depth) = twisting(cpu);
        let offsets = (p0 as u32, k0 as u32, twist, depth);
        let all: [&wgpu::Buffer; 7] = [
            &swept.point_buffer,
            &swept.cluster_buffer,
            &swept.table,
            &points,
            &clusters,
            &sums,
            &sags,
        ];
        let mut e = gpu.encoder(&Default::default());
        let runs = [
            (PACK, swept.points),
            (CLUSTERS, swept.clusters),
            (TOTAL, swept.clusters),
        ];
        for (entry, n) in runs {
            let groups = if entry == TOTAL {
                1
            } else {
                n.div_ceil(64).max(1)
            };
            let cfg = self.cfg(gpu, n, groups, offsets);
            let mut bound: Vec<&wgpu::Buffer> = vec![&cfg];
            bound.extend(all);
            let bind = self.pass.bind(gpu, &bound);
            self.pass.run(&mut e, &bind, entry, groups);
        }
        gpu.submit([e.finish()]);
        let raw = io::read_async(gpu, &sums, (SUMS * 4) as u64).await?;
        let s: &[u32] = bytemuck::cast_slice(&raw);
        let fine = (swept.clusters > 0).then(|| compute::bounds(s, 1));
        let mut demand = cpu.demand();
        demand.clusters += u64::from(swept.clusters);
        demand.stretches += u64::from(s[0]);
        demand.root_sag += s[8..]
            .iter()
            .map(|&w| f64::from(f32::from_bits(w)))
            .sum::<f64>();
        let curve = CurveGpu::on_device(gpu, cpu, (points, clusters), (p, k as u32), demand);
        Ok(Handed { curve, fine })
    }

    fn cfg(
        &self,
        gpu: &Gpu,
        n: u32,
        groups: u32,
        (p0, k0, twist, depth): (u32, u32, f64, f64),
    ) -> Owned {
        let most = gpu.device.limits().max_compute_workgroups_per_dimension;
        let row = groups.min(most) * 64;
        let words = [
            n,
            row,
            p0,
            k0,
            (twist as f32).to_bits(),
            (depth as f32).to_bits(),
            0,
            0,
        ];
        let usage = wgpu::BufferUsages::UNIFORM;
        io::buffer(gpu, "handoff cfg", 32, usage, bytemuck::cast_slice(&words)).unwrap()
    }
}

/// The lobes' twist (radians a metre) and depth, as `Curve::twisting`
/// reads them: none where the wood does not twist its lobes.
fn twisting(c: &Curve) -> (f64, f64) {
    if c.lobes == 0 || c.lobe_depth == 0.0 || c.twist_rate == 0.0 {
        return (0.0, 0.0);
    }
    let rate = std::f64::consts::TAU * c.twist_rate / c.height;
    (f64::from(c.lobes) * rate.abs(), c.lobe_depth.abs())
}

/// The union of two boxes, either absent.
pub(super) fn union(a: Option<Bounds>, b: Option<Bounds>) -> Option<Bounds> {
    let lo = |a: Vec3, b: Vec3| Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let hi = |a: Vec3, b: Vec3| Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
    match (a, b) {
        (Some(a), Some(b)) => Some(Bounds {
            min: lo(a.min, b.min),
            max: hi(a.max, b.max),
        }),
        (a, b) => a.or(b),
    }
}
