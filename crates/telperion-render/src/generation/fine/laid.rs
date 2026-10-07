//! A fine root's geometry on the GPU executor (fn-209 F1, task 4; host
//! decision 5): each wave's chains, scaled, thickened and laid by
//! `geometry.wgsl` from their packed regions and their bearers in the wave
//! before, into the wave's laid buffer on the device. The first wave's
//! bearers are the roots' attachments (`stubs`). Positions are each root's
//! own, metres from its bearer's node as laid before bending and as laid;
//! `origins` holds those nodes' heights, which the ground is read from.
use super::super::io;
use super::pool::Pool;
use crate::Gpu;
use telperion_space::{BearerFrame, GeometryTables, RootGeometry, Vec3};

const CONTRACT_LAY: &str = include_str!("lay.wgsl");
const GEOMETRY: &str = include_str!("geometry.wgsl");
const RESOLVE: &str = include_str!("resolve.wgsl");
const WALK: &str = include_str!("walk.wgsl");
const SETTLE: &str = include_str!("settle.wgsl");
/// A laid axis's and phytomer's floats (`geometry.wgsl`).
pub(super) const AL: usize = 8;
pub(super) const PL: usize = 22;
/// A chain's scratch: per region axis, per region phytomer, then a status.
pub(super) const SA: u32 = 66;
pub(super) const SP: u32 = 12;
const GG: usize = 24;
/// `settle.wgsl`'s passes.
pub(super) const SCALES: usize = 0;
pub(super) const GIRTHS: usize = 1;
pub(super) const LAYS: usize = 2;
pub(super) const LEVERS: usize = 3;
pub(super) const BENDS: usize = 4;

/// The geometry kernel's WGSL.
pub(super) fn source() -> String {
    format!(
        "{}\n{CONTRACT_LAY}\n{GEOMETRY}\n{RESOLVE}\n{WALK}\n{SETTLE}",
        super::contract()
    )
}

/// The species point's values as `geometry.wgsl` reads them: the PAs,
/// the age, where the blocks end and whether any PA sags; each PA's
/// block.
pub(super) fn table(g: &GeometryTables) -> (Vec<f32>, bool) {
    let npas = g.pas.len();
    let rows = 4 + GG * npas;
    let mut out = vec![0.0f32; rows];
    out[0] = f32::from_bits(npas as u32);
    out[1] = f32::from_bits(g.age);
    out[2] = f32::from_bits(rows as u32);
    for (pa, p) in g.pas.iter().enumerate() {
        let block = [
            p.internode,
            p.insertion,
            p.divergence,
            p.relay_at,
            p.epitony,
            p.straightening,
            p.erection,
            p.tropism,
            p.elevation,
            p.wander,
            p.plane,
            p.pipe,
            p.exponent,
            p.ripening,
            p.dominance,
            p.roll,
            p.sag,
            p.secondary,
            p.bend_length,
            p.retained,
        ];
        for (k, v) in block.iter().enumerate() {
            out[4 + GG * pa + k] = *v as f32;
        }
        let [lo, hi] = q(p.divergence / std::f64::consts::TAU, 60);
        out[4 + GG * pa + 20] = f32::from_bits(lo);
        out[4 + GG * pa + 21] = f32::from_bits(hi);
    }
    let sags = g.pas.iter().any(|p| p.sag > 0.0);
    (out, sags)
}

/// `x` in fixed point of `bits` fraction bits, as two words.
fn q(x: f64, bits: i32) -> [u32; 2] {
    let v = (x * 2f64.powi(bits)).round() as u64;
    [v as u32, (v >> 32) as u32]
}

/// An f32 for the tables, in a word's low half.
fn f(x: f64) -> u64 {
    u64::from((x as f32).to_bits())
}

/// The zones' values as `presence.wgsl` and `assign.wgsl` read them, a
/// word each: the PAs and where the zones' blocks start, then each
/// zone's block (its buds a node, and where its runs' thresholds of
/// laterals and sleeping buds start) and those thresholds.
pub(super) fn presences(g: &GeometryTables) -> Vec<u64> {
    let npas = g.pas.len();
    let zones: usize = g.zones.iter().map(Vec::len).sum();
    let mut out = vec![npas as u64, 2];
    out.resize(2 + 4 * zones, 0);
    for (z, zone) in g.zones.iter().flatten().enumerate() {
        let at = 2 + 4 * z;
        out[at] = f(zone.buds);
        out[at + 1] = out.len() as u64;
        out.extend(zone.laterals.iter().flatten());
        out[at + 2] = out.len() as u64;
        out.extend(zone.dormant.iter().flatten());
    }
    out
}

/// The first wave's bearers: each root's attachment as a laid axis of one
/// phytomer, in its own positions; and each root's origins' heights.
pub(super) fn stubs(roots: &[RootGeometry]) -> (Vec<f32>, Vec<f32>) {
    let mut out = vec![0.0f32; roots.len() * (AL + PL)];
    let phyts = roots.len() * AL;
    let mut origins = Vec::with_capacity(2 * roots.len());
    let v = |v: Vec3| [v.x as f32, v.y as f32, v.z as f32];
    for (k, r) in roots.iter().enumerate() {
        let rel = |f: &BearerFrame| v(f.below - f.tip);
        let a = &mut out[k * AL..(k + 1) * AL];
        a[..3].copy_from_slice(&rel(&r.unbent));
        a[3..6].copy_from_slice(&rel(&r.bent));
        a[6] = f32::from_bits(r.bearer_pa as u32);
        a[7] = f32::from_bits(k as u32);
        let p = &mut out[phyts + k * PL..phyts + (k + 1) * PL];
        p[3..6].copy_from_slice(&v(r.bent.heading));
        p[6..9].copy_from_slice(&v(r.bent.side));
        p[12..15].copy_from_slice(&v(r.unbent.heading));
        p[15..18].copy_from_slice(&v(r.unbent.side));
        p[19] = r.unbent.scale as f32;
        let [lo, hi] = q(r.unbent.rank, 52);
        p[20] = f32::from_bits(lo);
        p[21] = f32::from_bits(hi);
        origins.extend([r.unbent.tip.z as f32, r.bent.tip.z as f32]);
    }
    (out, origins)
}

/// A wave's laid buffer: its axes, then its phytomers.
pub(super) struct Wave {
    pub buffer: wgpu::Buffer,
    pub axes: u32,
    pub phyts: u32,
}

impl Wave {
    /// A wave of `axes` and `phyts` in a buffer from `pool`.
    pub(super) fn taken(pool: &Pool, gpu: &Gpu, (axes, phyts): (u32, u32)) -> Self {
        let floats = axes as usize * AL + phyts as usize * PL;
        Self {
            buffer: pool.take(gpu, "fine laid", floats as u64),
            axes,
            phyts,
        }
    }

    /// The word its phytomers start at.
    fn phyts_at(&self) -> u32 {
        self.axes * AL as u32
    }

    /// Read back, for the tests.
    pub(super) fn read(&self, gpu: &Gpu) -> Vec<f32> {
        let bytes = (self.axes as usize * AL + self.phyts as usize * PL) as u64 * 4;
        bytemuck::cast_slice(&io::read(gpu, &self.buffer, bytes.max(16)).unwrap()).to_vec()
    }
}

pub(super) struct Resolver {
    pass: io::Pass,
}

impl Resolver {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let read = [true, true, true, true, true, false, false, false];
        let entries = ["scales", "girths", "lays", "levers", "bends"];
        Self {
            pass: io::Pass::new(gpu, source(), &read, &entries),
        }
    }

    /// Runs `settle.wgsl`'s `entry` (0 `scales` to 4 `bends`) over a run's
    /// `n` chains, described by `items`, their regions packed in `packed`
    /// and their scratch in `scratch`, from `prev` into `out`, the next
    /// wave's records at `prior` and this wave's at `mine` in `records`,
    /// which holds the roots' origins first; recorded into `encoder`.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn settle(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        (items, n, packed, scratch): (
            &wgpu::Buffer,
            u32,
            (&wgpu::Buffer, &wgpu::Buffer),
            &wgpu::Buffer,
        ),
        (table, records): (&wgpu::Buffer, &wgpu::Buffer),
        (prev, out): (&Wave, &Wave),
        (age, sags): (u32, bool),
        (entry, prior, mine): (usize, u32, u32),
    ) {
        let groups = n.div_ceil(64);
        let row = groups.min(gpu.device.limits().max_compute_workgroups_per_dimension) * 64;
        let cfg = [
            n,
            row,
            out.phyts_at(),
            prev.phyts_at(),
            age,
            u32::from(sags),
            prior,
            mine,
        ];
        let uniform = wgpu::BufferUsages::UNIFORM;
        let cfg = io::buffer(gpu, "laid cfg", 32, uniform, bytemuck::cast_slice(&cfg)).unwrap();
        let all = [
            &cfg,
            items,
            table,
            packed.0,
            packed.1,
            &prev.buffer,
            &out.buffer,
            scratch,
            records,
        ];
        let bind = self.pass.bind(gpu, &all);
        self.pass.run(encoder, &bind, entry, groups);
    }
}
