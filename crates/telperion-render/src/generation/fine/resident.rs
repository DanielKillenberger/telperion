//! fn-212 task 7: a tree built resident with its fine roots on the device
//! (CONTRACT.md's probe, last part; host decisions on the handoff,
//! 2026-10-07). The pipeline converts the structure above the cut and the
//! roots laid on the CPU alone, taken from the device settle's structure
//! (decision 1, D3(c)); the device draws its roots from
//! `Tree::attachments`, folds them into the whole tree's envelope and leaf
//! box, sweeps, stations and places them, and packs its curve after the
//! CPU block's in the renderer's own form (decision 2); both blocks of
//! leaves are placed into one leaf buffer, the device's at an offset, with
//! one mass grid over both (decision 3). Only totals and summaries are
//! read back. The submit takes these buffers as they are.
use super::super::{compute, data, io, Backend, Generator, Metrics, Prepared, Resident};
use super::device::Device;
use super::dress::{dress, Dresser};
use super::fixture::stubs;
use super::handoff::{union, Handoff};
use super::rings::Ringer;
use super::roots::{Kernels, Tables};
use super::stations::{station_rows, Stationed, Stationer};
use super::sweep::{rows, Sweeper};
use crate::buffer::Held;
use crate::{Gpu, Result};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;
use telperion_core::foliage::prepared::{PreparedStations, StationSegment};
use telperion_core::foliage::Instances;
use telperion_core::mesh::{Foliage, TreeMesh};
use telperion_core::pipeline::executor::{self, beyond, bins, Expansion};
use telperion_core::presets::Family;
use telperion_core::surface::SurfaceMesh;
use telperion_core::Error;
use telperion_space::{ChainTables, GeometryTables};

/// The device's tools, made once.
pub(super) struct Tools {
    pub dresser: Dresser,
    pub sweeper: Sweeper,
    pub stationer: Stationer,
    pub ringer: Ringer,
    pub handoff: Handoff,
}

impl Tools {
    pub(super) fn new(gpu: &Gpu) -> Self {
        Self {
            dresser: Dresser::new(gpu),
            sweeper: Sweeper::new(gpu),
            stationer: Stationer::new(gpu),
            ringer: Ringer::new(gpu),
            handoff: Handoff::new(gpu),
        }
    }
}

/// Each stage's milliseconds, each waited on: the settle (growth, outside
/// the 3x boundary), the partial conversion, the device's dress and
/// folds, the pipeline's expansion of its part, its curve, stations and
/// ring positions (`cpu`), the device's sweep, stations, rings and ring
/// positions, both placements and the mass grid, and the curve's handoff.
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct Timed {
    pub settle: f64,
    pub convert: f64,
    pub dress: f64,
    pub folds: f64,
    pub expand: f64,
    pub cpu: f64,
    pub sweep: f64,
    pub stations: f64,
    pub rings: f64,
    pub place: f64,
    pub handoff: f64,
}

impl Timed {
    /// From the settled tree to draw-ready: every stage but the settle.
    pub(super) fn inside(&self) -> f64 {
        let t = self;
        t.convert
            + t.dress
            + t.folds
            + t.expand
            + t.cpu
            + t.sweep
            + t.stations
            + t.rings
            + t.place
            + t.handoff
    }
}

/// A build on the real path: the prepared tree, its timings, and what the
/// parity probe reads beside the buffers the submit takes (both blocks'
/// station segments: the CPU's as prepared, the device's buffer).
pub(super) struct Built {
    pub prepared: Prepared,
    pub timed: Timed,
    pub expansion: Expansion,
    pub cpu_segments: Vec<StationSegment>,
    pub device: Stationed,
}

fn stage<T>(gpu: &Gpu, time: &mut f64, work: impl FnOnce() -> T) -> T {
    let t = Instant::now();
    let out = work();
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .unwrap();
    *time = t.elapsed().as_secs_f64() * 1e3;
    out
}

fn refused(what: &'static str) -> crate::RenderError {
    Error::InvalidInput(what).into()
}

/// `family` built resident, its fine roots on the device.
pub(super) fn build(
    gpu: &Gpu,
    gen: &Generator,
    k: &Kernels,
    tools: &Tools,
    family: &Family,
) -> Result<Built> {
    let species = (family.space.species.clone()).ok_or_else(|| refused("no engine species"))?;
    let age = family.age as u32;
    let chain = ChainTables::new(&species, age).map_err(|_| refused("chain tables"))?;
    let geo = GeometryTables::new(&species, age).map_err(|_| refused("geometry tables"))?;
    let tables = Tables::new(gpu, Arc::new(chain), &geo);
    let mut dev = Device::new(gpu, k, &tables, 1 << 16).sown_from(64);
    let mut t = Timed::default();
    let settling = stage(gpu, &mut t.settle, || executor::settle(family, &mut dev))?;
    let part = stage(gpu, &mut t.convert, || settling.part());
    let landed: HashSet<usize> = part.grounded.iter().copied().collect();
    let d = &tools.dresser;
    let dressed = stage(gpu, &mut t.dress, || {
        dress(
            gpu,
            d,
            &dev,
            &stubs(part.tree(), &dev, &part.lineages, &landed),
        )
    });
    drop(dev);
    let (folds, crown, corners) = stage(gpu, &mut t.folds, || {
        let folds = part.folds().with(dressed.extent(gpu, d));
        let shape = folds.shape();
        let terms = part.wood_reach(family, shape[1])?;
        let (theirs, corners) = dressed.crowns(gpu, d, shape, terms);
        let own = part.tree().nodes.iter().map(|n| n.position);
        let most: Vec<f64> = (bins(own, shape).into_iter().zip(theirs))
            .map(|(a, b)| a.max(b))
            .collect();
        let (crown, over) = dressed.sift(gpu, d, shape, &beyond(&most));
        if over {
            return Err(Error::ResourceLimit("the device's crown front").into());
        }
        Ok::<_, crate::RenderError>((folds, crown, corners))
    })?;
    let x = stage(gpu, &mut t.expand, || {
        part.expansion(family, folds, &crown, corners)
    })?;
    // The pipeline's part: its curve, stations and ring positions.
    let mut m = Metrics::default();
    let (curve, cpu, wood, angular, segments) = stage(gpu, &mut t.cpu, || {
        let (curve, shared) = x.curve_with_contacts()?;
        let stations = x.compact_stations(&shared)?;
        let stations = stations.ok_or_else(|| refused("the fine path needs regular stations"))?;
        let surface = shared.surface();
        let (angular, segments) = (surface.angular.clone(), surface.segments);
        let wood = pollster::block_on(gen.emit_positions(shared.into_surface(), &mut m))?;
        let wood = wood.ok_or_else(|| refused("ring positions"))?;
        Ok::<_, crate::RenderError>((curve, stations, wood, angular, segments))
    })?;
    // The device's: its curve, stations and ring positions.
    let surface = &family.surface;
    let height = x.leaves().envelope.height;
    let swept = stage(gpu, &mut t.sweep, || {
        tools.sweeper.sweep(gpu, &dressed, rows(surface, height))
    });
    let ids = dressed.ids;
    drop(dressed);
    let device = stage(gpu, &mut t.stations, || {
        tools
            .stationer
            .stations(gpu, &swept, ids, station_rows(&x, segments))
    });
    swept.nodes.destroy();
    let positions = stage(gpu, &mut t.rings, || {
        let twist = std::f64::consts::TAU * surface.twist_rate / height;
        let rung = tools.ringer.rings(gpu, &swept, segments, twist as f32);
        let rings = (&rung.buffer, rung.rings, rung.vertices);
        let positions = gen.ring_vertices(rings, segments, &angular);
        rung.buffer.destroy();
        positions
    })?;
    // Both blocks into one leaf buffer, one mass grid over both.
    let count = cpu.count + device.stations;
    let reference = x.reference();
    let resident = stage(gpu, &mut t.place, || {
        let leaves = gen.leaf_buffer(count)?;
        let (l, e) = (x.leaves(), x.element());
        let counts = (cpu.count, cpu.segments.len() as u32, cpu.ring_size);
        let config = data::config(&l, x.twig(), counts, e, reference);
        let segs = data::segments(&cpu);
        let usage = wgpu::BufferUsages::STORAGE;
        let segs = io::buffer(
            gpu,
            "generation segments",
            (segs.len().max(1) * 128) as u64,
            usage,
            bytemuck::cast_slice(&segs),
        )?;
        let mut summaries = Vec::new();
        if cpu.count > 0 {
            let at = (cpu.count, &leaves);
            summaries.push(gen.place_into(config, (&segs, &wood.positions), e, at)?.0);
        }
        if device.stations > 0 {
            let counts = (device.stations, device.segments, segments);
            let config = data::config(&l, x.twig(), counts, e, reference).at(cpu.count);
            let at = (device.stations, &leaves);
            summaries.push(
                gen.place_into(config, (&device.buffer, &positions), e, at)?
                    .0,
            );
        }
        let (mut crown, mut full) = (None, None);
        for s in &summaries {
            let raw = io::read(gpu, s, 64)?;
            let s: &[u32] = bytemuck::cast_slice(&raw);
            if s[1] != 0 {
                return Err(refused("GPU foliage transform/contact/reference"));
            }
            crown = union(crown, Some(compute::bounds(s, 2)));
            full = union(full, Some(compute::bounds(s, 8)));
        }
        let masses = pollster::block_on(gen.mass(&leaves, count, crown, reference, 0, &mut m))?;
        Ok(Resident {
            leaves: Held::resident(leaves, u64::from(count) * 12, "generated foliage"),
            masses: Held::resident(masses.0, masses.1, "generated foliage masses"),
            count,
            crown,
            full,
        })
    })?;
    drop((wood, positions));
    let handed = stage(gpu, &mut t.handoff, || {
        tools.handoff.hand(gpu, &curve, &swept)
    });
    for b in [&swept.point_buffer, &swept.cluster_buffer, &swept.table] {
        b.destroy();
    }
    let bounds = union(union(curve.bounds(), handed.fine), resident.full);
    let bounds = bounds.ok_or_else(|| refused("mesh has no geometry"))?;
    let mesh = TreeMesh {
        wood: SurfaceMesh::default(),
        curve,
        foliage: Foliage {
            element: x.element().clone(),
            instances: Instances::new(reference),
        },
        bounds,
    };
    crate::submit::fits_count(&gpu.device.limits(), &mesh, count as usize)?;
    m.instances = count;
    m.total_ms = t.inside();
    m.wood_backend = Some(Backend::Gpu);
    let prepared = Prepared {
        identity: gen.identity.clone(),
        mesh,
        resident: Some(resident),
        wood: Some(handed.curve),
        backend: Backend::Gpu,
        metrics: m,
    };
    let PreparedStations {
        segments: cpu_segments,
        ..
    } = cpu;
    Ok(Built {
        prepared,
        timed: t,
        expansion: x,
        cpu_segments,
        device,
    })
}
