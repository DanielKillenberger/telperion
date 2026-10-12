//! fn-256 task 3 (DESIGN.md 12): a tree built resident, whole on the
//! device. The one cycle grows and settles it, the dress draws it as the
//! pipeline's nodes (`cycle/tree.rs`), and the device folds them into
//! the whole tree's summaries, from which the pipeline reads its rows
//! (`executor::resident`); a family that hangs retained leaf bases
//! hangs them between the dress and the folds (fn-266, `boots.rs`); then
//! it sweeps, stations, rings and places them, and packs its curve in the renderer's own form. A flowering
//! tree's florets are placed the same way (fn-229, `florets.rs`). Only
//! totals and summaries are read back. The submit takes these buffers as
//! they are.
use super::super::compute::{self, Placement};
use super::super::cycle::tree::{self, Kernels};
use super::super::{data, io, Backend, Clock, Generator, Metrics, Prepared, Resident};
use super::boots::Hanger;
use super::fold::Dresser;
use super::florets::Organ;
use super::handoff::{union, Handoff};
use super::rings::Ringer;
use super::stations::{station_rows, Stationed, Stationer};
use super::sweep::{rows, Sweeper};
use crate::buffer::{Held, Owned};
use crate::{Gpu, Result};
use telperion_core::foliage::Instances;
use telperion_core::mesh::{Foliage, TreeMesh};
use telperion_core::pipeline::executor::{self, beyond, Summaries};
use telperion_core::presets::Family;
use telperion_core::surface::SurfaceMesh;
use telperion_core::Error;

/// The device's tools, made once.
pub(super) struct Tools {
    pub hanger: Hanger,
    pub dresser: Dresser,
    pub sweeper: Sweeper,
    pub stationer: Stationer,
    pub ringer: Ringer,
    pub handoff: Handoff,
}

impl Tools {
    /// Every tool made side by side (`io::pair`).
    pub(super) fn new(gpu: &Gpu) -> Self {
        use super::super::io::pair;
        let ((hanger, dresser), ((sweeper, stationer), (ringer, handoff))) = pair(
            gpu,
            || pair(gpu, || Hanger::new(gpu), || Dresser::new(gpu)),
            || pair(gpu, || pair(gpu, || Sweeper::new(gpu), || Stationer::new(gpu)), || pair(gpu, || Ringer::new(gpu), || Handoff::new(gpu))),
        );
        Self { hanger, dresser, sweeper, stationer, ringer, handoff }
    }
}

/// Each stage's milliseconds, each waited on: the tree grown, settled
/// and drawn (outside the 3x boundary), its retained leaf bases, the
/// device's folds and the
/// pipeline's rows from them, its sweep, stations, rings and ring
/// positions, the placement and the mass grid, and the curve's handoff.
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct Timed {
    pub settle: f64,
    pub bases: f64,
    pub folds: f64,
    pub sweep: f64,
    pub stations: f64,
    pub rings: f64,
    pub place: f64,
    pub handoff: f64,
}

impl Timed {
    /// From the drawn tree to draw-ready: every stage but the growth.
    pub(super) fn inside(&self) -> f64 {
        let t = self;
        t.bases + t.folds + t.sweep + t.stations + t.rings + t.place + t.handoff
    }
}

/// A build: the prepared tree, and the stations a test reads beside the
/// buffers the submit takes.
pub(super) struct Built {
    pub prepared: Prepared,
    #[cfg_attr(not(test), allow(dead_code))]
    pub device: Stationed,
}

/// `work` awaited and its device work waited for, its milliseconds in
/// `time`: each stage's own, natively and in a browser alike.
async fn stage<T>(gpu: &Gpu, time: &mut f64, work: impl AsyncFnOnce() -> Result<T>) -> Result<T> {
    let t = Clock::now();
    let out = work().await?;
    io::complete(gpu).await?;
    *time = t.elapsed_ms();
    Ok(out)
}

fn refused(what: &'static str) -> crate::RenderError {
    Error::InvalidInput(what).into()
}

/// `family` built resident on the device, whole (fn-256 task 3,
/// DESIGN.md 12): grown through the one cycle, settled, drawn as the
/// pipeline's nodes, its leaf bases hung (DESIGN.md 12.8), folded,
/// swept, stationed, ringed and placed. The pipeline reads only the
/// summaries back. Every readback is awaited,
/// so the same build runs natively, blocked on, and in a browser.
pub(super) async fn build(
    gpu: &Gpu,
    gen: &Generator,
    k: &Kernels,
    tools: &Tools,
    family: &Family,
) -> Result<Built> {
    // Every buffer the build makes is held to the page's budget before it
    // is made, a move's new tables beside its old (`Ledger::capped`).
    let _capped = gen.device_budget().map(|b| gpu.ledger.capped(b));
    let growth = executor::growth(family)?;
    if !executor::stations(family) {
        return Err(refused("leaves the device places on neither twig stations nor a rosette"));
    }
    let bases = executor::bases(family)?;
    let flowers = growth.species.states.iter().any(|s| s.flowering > 0.0);
    let mut t = Timed::default();
    let (mut dressed, trunk) = stage(gpu, &mut t.settle, async || tree::drawn(gpu, k, &growth).await).await?;
    let hung = stage(gpu, &mut t.bases, async || {
        tools.hanger.hang(gpu, &mut dressed, bases.as_ref()).await
    })
    .await?;
    let d = &tools.dresser;
    let x = stage(gpu, &mut t.folds, async || {
        let folds = dressed.extent(gpu, d).await?;
        let shape = folds.shape();
        let terms = executor::reach_terms(family, shape[1])?;
        let (bins, corners) = dressed.crowns(gpu, d, shape, terms).await?;
        let (crown, over) = dressed.sift(gpu, d, shape, &beyond(&bins)).await?;
        if over {
            return Err(Error::ResourceLimit("the device's crown front").into());
        }
        let (root, stem) = (trunk.root, trunk.stem);
        Ok(executor::resident(family, Summaries { folds, crown, corners, root, stem })?)
    })
    .await?;
    let seats = x.seats();
    let mut m = Metrics::default();
    let section = x.section()?;
    let segments = section.segments;
    let surface = &family.surface;
    let height = x.leaves().envelope.height;
    let sweeper = &tools.sweeper;
    let rows = rows(surface, height);
    let swept = stage(gpu, &mut t.sweep, async || {
        sweeper.sweep(gpu, &dressed, rows, hung.cells.as_ref()).await
    })
    .await?;
    let ids = dressed.ids;
    drop(dressed);
    // A flowering tree's organ, its rows checked before the stations read
    // them (`bloom_element`'s check).
    let organ = match flowers {
        true => Some(Organ::of(&x)?),
        false => None,
    };
    let station = station_rows(&x, segments, organ.as_ref().map(Organ::rows));
    let stationer = &tools.stationer;
    let device = stage(gpu, &mut t.stations, async || {
        stationer.stations(gpu, &swept, ids, station).await
    })
    .await?;
    let count = Some(device.stations)
        .filter(|&n| n as usize <= x.leaves().canopy.max_instances)
        .ok_or(Error::ResourceLimit("foliage instance budget"))?;
    swept.nodes.destroy();
    let positions = match seats {
        true => Some(
            stage(gpu, &mut t.rings, async || {
                let twist = std::f64::consts::TAU * surface.twist_rate / height;
                let rung = tools.ringer.rings(gpu, &swept, segments, twist as f32)?;
                let rings = (&*rung.buffer, rung.rings, rung.vertices);
                let at = (section.segments, section.lobes, section.depth);
                gen.ring_vertices(rings, at, &section.angular)
            })
            .await?,
        ),
        false => None,
    };
    // The leaves into one leaf buffer and its mass grid; the florets
    // likewise, in the submission of the leaves.
    let reference = x.reference();
    let (resident, florets) = stage(gpu, &mut t.place, async || {
        let leaves = gen.leaf_buffer(count)?;
        let (l, e) = (x.leaves(), x.element());
        let usage = wgpu::BufferUsages::STORAGE;
        // Leaves that seat on nothing read no ring positions.
        let none = io::buffer(gpu, "no ring positions", 16, usage, &[])?;
        let mut encoder = gpu.encoder(&Default::default());
        let mut summaries = Vec::new();
        if device.stations > 0 {
            let counts = (device.stations, device.segments, segments);
            let config = data::config(&l, x.twig(), counts, e, reference);
            let at = (device.stations, &*leaves);
            let bufs = (&*device.buffer, positions.as_deref().unwrap_or(&none));
            let placed = gen.record_place(&mut encoder, Placement::Leaves, config, bufs, e, at)?;
            summaries.push(placed.0);
        }
        let placed = match &organ {
            Some(o) => o.place(gen, &mut encoder, &device)?,
            None => None,
        };
        let floret = placed.as_ref().and_then(|p| p.summary());
        let placed_florets = floret.is_some();
        let words = read(gpu, encoder, summaries.iter().chain(floret)).await?;
        let (mut crown, mut full) = (None, None);
        for s in words.chunks(16).take(summaries.len()) {
            if s[1] != 0 {
                return Err(refused("GPU foliage transform/contact/reference"));
            }
            crown = union(crown, Some(compute::bounds(s, 2)));
            full = union(full, Some(compute::bounds(s, 8)));
        }
        let mut encoder = gpu.encoder(&Default::default());
        let at = (&*leaves, count, crown, reference);
        let masses = gen.record_mass(&mut encoder, at, 0, &mut m)?;
        let florets = match (&organ, placed) {
            (Some(o), Some(p)) => {
                let summary = placed_florets.then(|| &words[16 * summaries.len()..]);
                Some(o.resident(gen, &mut encoder, (p, summary), &mut m)?)
            }
            _ => None,
        };
        gpu.submit([encoder.finish()]);
        let resident = Resident {
            leaves: Held::resident(leaves, u64::from(count) * 12, "generated foliage"),
            masses: Held::resident(masses.0, masses.1, "generated foliage masses"),
            count,
            crown,
            full,
        };
        Ok((resident, florets))
    })
    .await?;
    // The ring positions served the placements alone.
    drop(positions);
    let curve = x.curve_rows()?;
    let handoff = &tools.handoff;
    let handed = stage(gpu, &mut t.handoff, async || {
        handoff.hand(gpu, &curve, &swept, hung.cells).await
    })
    .await?;
    for b in [&swept.point_buffer, &swept.cluster_buffer, &swept.table] {
        b.destroy();
    }
    let bounds = union(handed.fine, resident.full);
    let bounds = union(bounds, florets.as_ref().and_then(|f| f.full));
    let bounds = bounds.ok_or_else(|| refused("mesh has no geometry"))?;
    let organ = match (&organ, &florets) {
        (Some(o), Some(_)) => Some(o.foliage()),
        _ => None,
    };
    let mesh = TreeMesh {
        wood: SurfaceMesh::default(),
        curve,
        foliage: Foliage {
            element: x.element().clone(),
            instances: Instances::new(reference),
        },
        organ,
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
        organ: florets,
        wood: Some(handed.curve),
        backend: Backend::Gpu,
        metrics: m,
    };
    if std::env::var("CYCLE_TRACE").is_ok() {
        eprintln!("TRACE stages {t:?}");
    }
    Ok(Built { prepared, device })
}

/// The 16-word placement `summaries`, copied into one buffer in `encoder`
/// and read back in one readback, in their order.
async fn read<'a>(
    gpu: &Gpu,
    mut encoder: crate::Encoder,
    summaries: impl Iterator<Item = &'a Owned>,
) -> Result<Vec<u32>> {
    let summaries: Vec<&Owned> = summaries.collect();
    let bytes = (summaries.len() * 64) as u64;
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let all = io::buffer(gpu, "generation summaries", bytes, usage, &[])?;
    for (k, s) in summaries.iter().enumerate() {
        encoder.copy_buffer_to_buffer(s, 0, &all, k as u64 * 64, 64);
    }
    gpu.submit([encoder.finish()]);
    let raw = io::read_async(gpu, &all, bytes).await?;
    Ok(bytemuck::pod_collect_to_vec(&raw))
}
