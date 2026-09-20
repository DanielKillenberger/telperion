//! Compact regular twig stations, shared by accelerated consumers without a GPU dependency.
use super::{placement, station, CanopyParams, TwigPlacement};
use crate::math::Transcendental;
use crate::{
    envelope::Envelope,
    math::Vec3,
    surface::{AttachmentSurface, SurfaceParams},
    tree::Tree,
    Error, Result,
};

/// A contiguous range of stations on one swept wood segment. Ordinals refer
/// to the uncullled stream, so scheduling never changes random draws.
#[derive(Debug)]
pub struct StationSegment {
    pub first: u32,
    pub count: u32,
    pub run_station: u32,
    /// Sine/cosine of the first internode turn, evaluated before f32 narrowing.
    pub phase: [f64; 2],
    pub endpoints: [Vec3; 2],
    pub radii: [f64; 2],
    pub along: f64,
    pub span: f64,
    pub frame: [Vec3; 3],
    /// Lower/upper ring, then first/last ring on the sweep path.
    pub contact: Option<[u32; 4]>,
}

/// No per-leaf positions or transforms are constructed by this preparation.
#[derive(Debug)]
pub struct PreparedStations {
    pub segments: Vec<StationSegment>,
    pub count: u32,
    pub rings: Vec<Vec3>,
    pub ring_size: u32,
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
    placement::validate(tree, envelope, p, twig)?;
    surface.validate()?;
    let Some(twig) = twig else { return Ok(None) };
    if p.short_shoot_spacing != 0.0 || p.limb_clumping != 0.0 {
        return Ok(None);
    }
    let mut out = PreparedStations {
        segments: Vec::new(),
        count: 0,
        rings: Vec::new(),
        ring_size: 0,
    };
    if tree.nodes.len() < 2 || p.size == 0.0 {
        return Ok(Some(out));
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
        Some(AttachmentSurface::new(tree, envelope.height, surface)?)
    } else {
        None
    };
    for nodes in placement::bearing_runs(tree, p) {
        let points: Vec<_> = nodes.iter().map(|&i| tree.nodes[i].position).collect();
        let mut along = vec![0.0];
        for pair in points.windows(2) {
            along.push(along.last().unwrap() + pair[0].distance(pair[1]));
        }
        let length = *along.last().unwrap();
        if length == 0.0 {
            continue;
        }
        if !length.is_finite() {
            return Err(Error::ResourceLimit("shoot length overflow"));
        }
        let internodes = (length / twig.internode_length - 1e-9).ceil().max(1.0);
        let count = internodes * f64::from(twig.stations_per_internode);
        // Eight f64 rounding units cover the original multiply/divide angle
        // chain. Keep periodic tile reconstruction below 0.00025 radians of
        // extra uncertainty; larger valid requests use the reference CPU path.
        let phase_error =
            internodes * p.divergence.abs() * std::f64::consts::PI / 180.0 * f64::EPSILON * 8.0;

        if !count.is_finite()
            || count > p.max_instances as f64
            || count >= (isize::MAX as usize / size_of::<f64>()) as f64
            || count > u32::MAX as f64
        {
            return Err(Error::ResourceLimit("foliage instance budget"));
        }
        if phase_error > 0.00025 {
            return Ok(None);
        }
        let count = count as u32;
        let end = out
            .count
            .checked_add(count)
            .filter(|&n| n as usize <= p.max_instances)
            .ok_or(Error::ResourceLimit("foliage instance budget"))?;
        let frames = station::station_frames(&points, &along);
        // The original reverse search chooses the last segment starting at or
        // below a distance. Integer lower bounds retain that tie convention,
        // including repeated zero-length segments, without enumerating leaves.
        let lower = |distance: f64| {
            let (mut lo, mut hi) = (0u32, internodes as u32);
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if f64::from(mid) * twig.internode_length < distance {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            lo * twig.stations_per_internode
        };
        for segment in 0..points.len() - 1 {
            let first = if segment == 0 {
                0
            } else {
                lower(along[segment])
            };
            let last = if segment + 2 == points.len() {
                count
            } else {
                lower(along[segment + 1])
            };
            if first == last {
                continue;
            }
            let distal = &tree.nodes[nodes[segment + 1]];
            let contact = contacts
                .as_ref()
                .map(|c| {
                    let (a, b, start, end) = c.edges[nodes[segment + 1]].ok_or(
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
            let (tangent, normal, binormal) = frames[segment];
            let mut tile_first = first;
            while tile_first < last {
                let tile_count = (last - tile_first).min(256);
                let turn = (tile_first / twig.stations_per_internode) as f64
                    * p.divergence
                    * std::f64::consts::PI
                    / 180.0;
                let (sin, cos) = turn.sin_cos_fixed();
                out.segments.push(StationSegment {
                    first: out.count + tile_first,
                    count: tile_count,
                    run_station: tile_first,
                    phase: [sin, cos],
                    endpoints: [points[segment], points[segment + 1]],
                    radii: [distal.start_radius, distal.radius],
                    along: along[segment],
                    span: along[segment + 1] - along[segment],
                    frame: [tangent, normal, binormal],
                    contact,
                });
                tile_first += tile_count;
            }
        }
        out.count = end;
    }
    if let Some(c) = contacts {
        out.rings = c.rings;
        out.ring_size = c.segments as u32;
    }
    Ok(Some(out))
}
