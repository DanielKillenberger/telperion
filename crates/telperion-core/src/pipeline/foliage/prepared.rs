//! Compact regular twig stations, shared by accelerated consumers without a GPU dependency.
use super::{cluster, plan, station, CanopyParams, TwigPlacement};
use crate::math::Transcendental;
use crate::{
    envelope::Envelope,
    math::Vec3,
    pipeline::surface::{AttachmentSurface, SurfaceParams},
    tree::{Leafing, Tree},
    Error, Result,
};

/// A contiguous range of stations on one swept wood segment. Ordinals refer
/// to the uncullled stream, so scheduling never changes random draws.
/// `count` leaves stand on it from leaf `first`: its stations from
/// ordinal `run_station` that keep their leaves.
#[derive(Debug)]
pub struct StationSegment {
    pub first: u32,
    pub count: u32,
    pub run_station: u32,
    /// Where some of its stations shed their leaves (fn-248): the boundary
    /// along the run in whole micrometres and the retentions before and
    /// from it (`tree::Leafing`), which a placement walks its stations by
    /// to find each leaf's ordinal. `None` where every station keeps one.
    pub retention: Option<(u32, u32)>,
    /// Sine/cosine of the first internode turn, evaluated before f32 narrowing.
    pub phase: [f64; 2],
    pub endpoints: [Vec3; 2],
    pub radii: [f64; 2],
    pub along: f64,
    pub span: f64,
    pub frame: [Vec3; 3],
    /// Lower/upper ring, then first/last ring on the sweep path.
    pub contact: Option<[u32; 4]>,
    /// The key of its run's first node (`Tree::key`): with the family's
    /// seed, the stream its leaves draw from (`tree::leaf_stream`).
    pub key: u32,
    /// A short shoot's cluster (fn-223), its spread: `count` leaves at
    /// the tip `endpoints` names, fanned about the heading `frame[0]`,
    /// drawing on the tip's cluster stream (`key`).
    pub cluster: Option<f64>,
    /// Its leaves' share of their whole size where the tree is drawn
    /// within its year (fn-270 R5, `Tree::unfolding`): its distal node's.
    pub unfolding: f64,
}

/// No per-leaf positions or transforms are constructed by this preparation.
#[derive(Debug)]
pub struct PreparedStations<R = Vec<Vec3>> {
    pub segments: Vec<StationSegment>,
    pub count: u32,
    pub rings: R,
    pub ring_size: u32,
}

/// Capability check only; preparation still validates every parameter. A
/// rosette is planned frond by frond but has no stations to prepare.
pub fn supports_stations(p: CanopyParams, twig: Option<TwigPlacement>) -> bool {
    plan::supports(p, twig) && !super::rosette::bearing(&p)
}

/// `None` is an explicit capability fallback, after parameter validation.
/// A supported but over-budget request remains an error.
pub fn prepare_stations(
    tree: &Tree,
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
    surface: &SurfaceParams,
) -> Result<Option<PreparedStations>> {
    let result = prepare_inner(
        tree,
        envelope,
        p,
        twig,
        surface,
        || AttachmentSurface::new(tree, envelope.height, surface),
        |c, node| c.edges.get(node).copied().flatten(),
    )?;
    Ok(result.map(|(segments, count, contacts)| {
        let (rings, ring_size) =
            contacts.map_or((Vec::new(), 0), |c| (c.points(), c.segments as u32));
        PreparedStations {
            segments,
            count,
            rings,
            ring_size,
        }
    }))
}

/// Borrows canonical float32 contacts; no ring emission or spatial index is built.
pub fn prepare_shared_stations<'a>(
    shared: &'a crate::pipeline::surface::prepared::PreparedWithContacts<'_>,
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
) -> Result<Option<PreparedStations<&'a [f32]>>> {
    if envelope.height != shared.height {
        return Err(Error::InvalidInput("contact surface height mismatch"));
    }
    let result = prepare_inner(
        shared.tree,
        envelope,
        p,
        twig,
        shared.params,
        || Ok(shared),
        |c, node| c.edges.get(node).copied().flatten(),
    )?;
    Ok(result.map(|(segments, count, contacts)| PreparedStations {
        segments,
        count,
        rings: contacts.map_or(&[][..], |c| c.surface.positions.as_slice()),
        ring_size: contacts.map_or(0, |c| c.surface.segments),
    }))
}

/// Station indices tied to compact emitted vertices; no CPU positions are fabricated.
pub fn prepare_compact_stations(
    shared: &crate::pipeline::surface::compact::CompactWithContacts<'_>,
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
) -> Result<Option<PreparedStations<()>>> {
    if envelope.height != shared.height {
        return Err(Error::InvalidInput("contact surface height mismatch"));
    }
    let result = prepare_inner(
        shared.tree,
        envelope,
        p,
        twig,
        shared.params,
        || Ok(shared),
        |c, node| c.edges.get(node).copied().flatten(),
    )?;
    Ok(result.map(|(segments, count, contacts)| PreparedStations {
        segments,
        count,
        rings: (),
        ring_size: contacts.map_or(0, |c| c.surface.segments),
    }))
}

fn prepare_inner<C>(
    tree: &Tree,
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
    surface: &SurfaceParams,
    make_contacts: impl FnOnce() -> Result<C>,
    edge: impl Fn(&C, usize) -> Option<[usize; 4]>,
) -> Result<Option<(Vec<StationSegment>, u32, Option<C>)>> {
    super::canopy::validate(tree, envelope, p, twig)?;
    surface.validate()?;
    if !supports_stations(p, twig) {
        return Ok(None);
    }
    let twig = twig.unwrap();
    let mut segments = Vec::new();
    let mut total_count = 0;
    if tree.nodes.len() < 2 || p.size == 0.0 {
        return Ok(Some((segments, total_count, None)));
    }
    if tree.nodes.iter().any(|n| {
        [
            n.position.x,
            n.position.y,
            n.position.z,
            n.radius,
            n.start_radius,
        ]
        .iter()
        .any(|v| v.abs() > f32::MAX as f64 / 4.0)
    }) {
        return Err(Error::ResourceLimit("foliage coordinate range"));
    }
    let contacts = if p.surface_contact > 0.0 {
        Some(make_contacts()?)
    } else {
        None
    };
    let runs = plan::runs(tree, envelope, p, Some(twig))?.unwrap_or_default();
    let (mut points, mut chord, mut along) = (Vec::new(), Vec::new(), Vec::new());
    for run in &runs {
        // The leaves this run's tiles before the next one keep.
        let mut seen = 0u32;
        // The chord along the run, which its frames read; its stations
        // stand by the grown wood (`run.along`).
        plan::walk(
            tree,
            &run.nodes,
            Some(twig),
            &mut points,
            (&mut chord, &mut along),
        );
        // Eight f64 rounding units cover the original multiply/divide angle
        // chain. Keep periodic tile reconstruction below 0.00025 radians of
        // extra uncertainty; larger valid requests use the reference CPU path.
        let phase_error = f64::from(run.internodes) * p.divergence.abs() * std::f64::consts::PI
            / 180.0
            * f64::EPSILON
            * 8.0;
        if phase_error > 0.00025 {
            return Ok(None);
        }
        for (segment, (tangent, normal, binormal)) in station::station_frame_iter(
            &points,
            &chord,
            tree.seed_side(run.nodes[0]),
        )
        .enumerate()
        {
            let (first, last) = run.segment_stations(segment, twig);
            if run.kept_between(tree, twig, (first, last)) == 0 {
                continue;
            }
            let nodes = &run.nodes;
            let distal = &tree.nodes[nodes[segment + 1]];
            let contact = contacts
                .as_ref()
                .map(|c| {
                    let [a, b, start, end] = edge(c, nodes[segment + 1]).ok_or(
                        Error::InvalidInput("foliage surface contact projection missed"),
                    )?;
                    let mut indices = [0u32; 4];
                    for (target, value) in indices.iter_mut().zip([a, b, start, end]) {
                        *target = u32::try_from(value)
                            .map_err(|_| Error::ResourceLimit("attachment rings"))?;
                    }
                    Ok(indices)
                })
                .transpose()?;
            let leafing = tree.leafing(nodes[segment + 1]);
            let retention = (leafing != Leafing::ALL).then(|| {
                let boundary = run.along[segment] + plan::micrometres(leafing.older);
                (boundary as u32, leafing.keep)
            });
            let mut tile_first = first;
            while tile_first < last {
                let tile_count = (last - tile_first).min(256);
                let kept = run.kept_between(tree, twig, (tile_first, tile_first + tile_count));
                let before = seen;
                seen += kept;
                if kept == 0 {
                    tile_first += tile_count;
                    continue;
                }
                let turn = (tile_first / twig.stations_per_internode) as f64
                    * p.divergence
                    * std::f64::consts::PI
                    / 180.0;
                let (sin, cos) = turn.sin_cos_fixed();
                segments.push(StationSegment {
                    first: total_count + before,
                    count: kept,
                    run_station: tile_first,
                    retention,
                    phase: [sin, cos],
                    endpoints: [points[segment], points[segment + 1]],
                    radii: [distal.start_radius, distal.radius],
                    along: run.along[segment] * 1e-6,
                    span: (run.along[segment + 1] - run.along[segment]) * 1e-6,
                    frame: [tangent, normal, binormal],
                    contact,
                    key: tree.key(run.nodes[1]),
                    cluster: None,
                    unfolding: tree.unfolding(nodes[segment + 1]),
                });
                tile_first += tile_count;
            }
        }
        total_count += run.kept;
    }
    // Each short shoot's cluster after every run (fn-223): one segment at
    // its tip.
    for c in cluster::of(tree, &p) {
        let tip = c.node as usize;
        let at = tree.nodes[tip].position;
        let radius = tree.nodes[tip].radius;
        segments.push(StationSegment {
            first: total_count,
            count: c.leaves,
            run_station: 0,
            retention: None,
            phase: [0.0, 1.0],
            endpoints: [at; 2],
            radii: [radius; 2],
            along: 0.0,
            span: 0.0,
            frame: [cluster::heading(tree, tip), Vec3::ZERO, Vec3::ZERO],
            contact: None,
            key: crate::tree::cluster_key(tree.key(tip)),
            cluster: Some(c.spread),
            unfolding: tree.unfolding(tip),
        });
        total_count += c.leaves;
    }
    Ok(Some((segments, total_count, contacts)))
}
