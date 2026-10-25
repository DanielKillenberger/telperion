//! fn-256 task 3 (DESIGN.md 12): a tree built resident, whole on the
//! device. The one cycle grows and settles it, the dress draws it as the
//! pipeline's nodes (`cycle/tree.rs`), and the device folds them into
//! the whole tree's summaries, from which the pipeline reads its rows
//! (`executor::resident`); a family that hangs retained leaf bases
//! hangs them between the dress and the folds (fn-266, `boots.rs`); then
//! it sweeps them into the renderer's own curve form, stations, rings and
//! places them. A flowering tree's florets are placed the same way
//! (fn-229, `florets.rs`). Only totals and summaries are read back. The
//! submit takes these buffers as they are.
use super::super::compute::{self, Placement};
use super::super::cycle::tree::{self, Kernels};
use super::super::{data, io, Clock, Generator, Metrics, Prepared, Resident};
use super::boots::Hanger;
use super::florets::Organ;
use super::fold::{Dressed, Dresser};
use super::rings::Ringer;
#[cfg(test)]
use super::stations::Stationed;
use super::stations::{station_rows, Stationer};
use super::sweep::{rows, Sweeper};
use crate::buffer::{Held, Owned};
use crate::curve::CurveGpu;
use crate::{Gpu, Result};
use telperion_core::foliage::Instances;
use telperion_core::math::Vec3;
use telperion_core::mesh::{Foliage, TreeMesh};
use telperion_core::pipeline::executor::{self, beyond, Summaries};
use telperion_core::presets::Family;
use telperion_core::surface::{Bounds, SurfaceMesh};
use telperion_core::tree::Year;
use telperion_core::Error;

/// The device's tools, made once.
pub(super) struct Tools {
    pub hanger: Hanger,
    pub dresser: Dresser,
    pub sweeper: Sweeper,
    pub stationer: Stationer,
    pub ringer: Ringer,
    pub fielder: super::field::Fielder,
}

impl Tools {
    /// Every tool made side by side (`io::pair`).
    pub(super) fn new(gpu: &Gpu) -> Self {
        use super::super::io::pair;
        let ((hanger, dresser), ((sweeper, stationer), ringer)) = pair(
            gpu,
            || pair(gpu, || Hanger::new(gpu), || Dresser::new(gpu)),
            || {
                pair(
                    gpu,
                    || pair(gpu, || Sweeper::new(gpu), || Stationer::new(gpu)),
                    || Ringer::new(gpu),
                )
            },
        );
        Self {
            hanger,
            dresser,
            sweeper,
            stationer,
            ringer,
            fielder: super::field::Fielder::new(gpu),
        }
    }
}

/// Each stage's milliseconds, each waited on: the tree grown, settled
/// and drawn (outside the 3x boundary), its retained leaf bases, the
/// device's folds and the
/// pipeline's rows from them, its sweep into the renderer's curve, its
/// stations, rings and ring positions, and the placement and the mass
/// grid.
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct Timed {
    pub settle: f64,
    pub bases: f64,
    pub folds: f64,
    pub sweep: f64,
    pub stations: f64,
    pub rings: f64,
    pub place: f64,
}

impl Timed {
    /// From the drawn tree to draw-ready: every stage but the growth.
    pub(super) fn inside(&self) -> f64 {
        let t = self;
        t.bases + t.folds + t.sweep + t.stations + t.rings + t.place
    }
}

/// A build: the prepared tree, and the stations a test reads beside the
/// buffers the submit takes.
pub(super) struct Built {
    pub prepared: Prepared,
    #[cfg(test)]
    pub device: Stationed,
}

/// `work` awaited and its device work waited for, its milliseconds in
/// `time`: each stage's own, natively and in a browser alike.
pub(super) async fn stage<T>(
    gpu: &Gpu,
    time: &mut f64,
    work: impl AsyncFnOnce() -> Result<T>,
) -> Result<T> {
    let t = Clock::now();
    let out = work().await?;
    io::complete(gpu).await?;
    *time = t.elapsed_ms();
    Ok(out)
}

fn refused(what: &'static str) -> crate::RenderError {
    Error::InvalidInput(what).into()
}

/// The leaves the stations counted as an instance count, refused by name
/// past the canopy's budget or past 32 bits, which no leaf buffer counts
/// (fn-297 R4 B.3).
pub(super) fn instances(stations: u64, budget: usize) -> Result<u32> {
    u32::try_from(stations)
        .ok()
        .filter(|&n| n as usize <= budget)
        .ok_or(Error::ResourceLimit("foliage instance budget").into())
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
    crowned(gpu, (gen, k, tools), family, 1.0).await
}

/// `build`, its frond crowns drawn `fraction` into the last year (fn-270
/// R5b): the whole year's for a build, part of it where a test draws
/// the crown within its year on the whole tree's wood.
pub(super) async fn crowned(
    gpu: &Gpu,
    (gen, k, tools): (&Generator, &Kernels, &Tools),
    family: &Family,
    fraction: f64,
) -> Result<Built> {
    // Every buffer the build makes is held to the budget before it is
    // made, a move's new tables beside its old (`Ledger::capped`).
    let _capped = gpu.ledger.capped(gen.device_budget());
    let growth = executor::growth(family)?;
    placeable(family)?;
    let bases = super::boots::most(family)?;
    let mut t = Timed::default();
    let drawn = stage(gpu, &mut t.settle, async || {
        tree::drawn(gpu, k, (&growth, bases)).await
    })
    .await?;
    placed(
        gpu,
        (gen, tools),
        (family, &growth),
        (drawn, growth.year(fraction)),
        t,
    )
    .await
}

/// `family`'s leaves refused where the device places them on neither
/// twig stations nor a rosette.
pub(super) fn placeable(family: &Family) -> Result<()> {
    match executor::stations(family) {
        true => Ok(()),
        false => Err(refused(
            "leaves the device places on neither twig stations nor a rosette",
        )),
    }
}

/// A tree the cycle drew (`dressed`, its `trunk`) hung, folded, swept,
/// stationed, ringed and placed as `family`'s, its fronds drawn in
/// `year`: what every resident tree is prepared by, built at once or
/// grown on (`growing.rs`). `t` holds the draw's milliseconds.
pub(super) async fn placed(
    gpu: &Gpu,
    (gen, tools): (&Generator, &Tools),
    (family, growth): (&Family, &executor::Growth),
    (drawn, year): ((Dressed, tree::Trunk), Option<Year>),
    mut t: Timed,
) -> Result<Built> {
    let flowers = growth.species.states.iter().any(|s| s.flowering > 0.0);
    let Readied {
        x,
        swept,
        hung,
        dressed,
    } = readied(gpu, tools, (family, year), drawn, &mut t).await?;
    let seats = x.seats();
    let mut m = Metrics::default();
    let section = x.section()?;
    let segments = section.segments;
    let surface = &family.surface;
    let height = x.leaves().envelope.height;
    // A flowering tree's organ, its rows checked before the stations read
    // them (`bloom_element`'s check).
    let organ = match flowers {
        true => Some(Organ::of(&x)?),
        false => None,
    };
    let station = station_rows(&x, segments, organ.as_ref().map(Organ::rows), year);
    let stationer = &tools.stationer;
    let device = stage(gpu, &mut t.stations, async || {
        let laid = swept.laid(&dressed, hung.fronds.as_deref());
        stationer.stations(gpu, laid, dressed.ids, station).await
    })
    .await?;
    let count = instances(device.stations, x.leaves().canopy.max_instances)?;
    drop(dressed);
    // The device frees the drawn records, and the ledger their bytes,
    // before the rings are made beside the stations.
    io::complete(gpu).await?;
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
        if count > 0 {
            let counts = (count, device.segments, segments);
            let config = data::config(&l, x.twig(), counts, e, reference);
            let at = (count, &*leaves);
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
    // The ring positions served the placements alone, and the sweep's
    // table the stations and rings.
    drop(positions);
    let curve = x.curve_rows()?;
    let super::sweep::Swept {
        point_buffer,
        cluster_buffer,
        points,
        clusters,
        demand,
        bounds: fine,
        ..
    } = swept;
    // The sections the renderer draws shaped runs by: the retained leaf
    // bases' cells where they pack, else the curve's own (none).
    let sections = match hung.cells {
        Some(cells) => cells,
        None => crate::curve::sections(gpu, &curve),
    };
    let blocks = (point_buffer, cluster_buffer, sections);
    let wood = CurveGpu::on_device(gpu, &curve, blocks, (u64::from(points), clusters), demand);
    let bounds = union(fine, resident.full);
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
    let prepared = Prepared {
        identity: gen.identity.clone(),
        mesh,
        resident: Some(resident),
        organ: florets,
        wood: Some(wood),
        metrics: m,
        colouring: 1.0,
    };
    if std::env::var("CYCLE_TRACE").is_ok() {
        eprintln!("TRACE stages {t:?}");
    }
    Ok(Built {
        prepared,
        #[cfg(test)]
        device,
    })
}

/// A drawn tree (`dressed`, its `trunk`) hung, folded into the
/// pipeline's expansion and swept as `family`'s, its fronds drawn in
/// `year`: what a resident tree is placed from. `t` holds each stage's
/// milliseconds.
pub(super) struct Readied {
    pub x: executor::Expansion,
    pub swept: super::sweep::Swept,
    pub hung: super::boots::Hung,
    pub dressed: Dressed,
}

pub(super) async fn readied(
    gpu: &Gpu,
    tools: &Tools,
    (family, year): (&Family, Option<Year>),
    drawn: (Dressed, tree::Trunk),
    t: &mut Timed,
) -> Result<Readied> {
    let Folded { x, dressed, hung } = folded(gpu, tools, (family, year), drawn, t).await?;
    let rows = rows(&family.surface, x.leaves().envelope.height);
    let sweeper = &tools.sweeper;
    let swept = stage(gpu, &mut t.sweep, async || {
        sweeper
            .sweep(gpu, &dressed, rows, hung.cells.as_ref())
            .await
    })
    .await?;
    Ok(Readied {
        x,
        swept,
        hung,
        dressed,
    })
}

/// A drawn tree hung, its fronds drawn in `year`, and folded into the
/// pipeline's expansion: what it is swept from, and what a field lays its
/// nodes from (fn-257).
pub(super) struct Folded {
    pub x: executor::Expansion,
    pub dressed: Dressed,
    pub hung: super::boots::Hung,
}

pub(super) async fn folded(
    gpu: &Gpu,
    tools: &Tools,
    (family, year): (&Family, Option<Year>),
    (mut dressed, trunk): (Dressed, tree::Trunk),
    t: &mut Timed,
) -> Result<Folded> {
    let bases = executor::bases(family)?;
    let crown = super::boots::frond_rows(family, year)?;
    let hung = stage(gpu, &mut t.bases, async || {
        let rows = (bases.as_ref(), crown.as_ref());
        tools.hanger.hang(gpu, &mut dressed, rows).await
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
        Ok(executor::resident(
            family,
            Summaries {
                folds,
                crown,
                corners,
                root,
                stem,
            },
        )?)
    })
    .await?;
    Ok(Folded { x, dressed, hung })
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
