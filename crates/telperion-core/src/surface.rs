//! Independent, closed swept shells over solved tree paths. Buffers are caller-owned.
use crate::math::Transcendental;
use crate::{math::Vec3, tree::Tree, Error, Result};
mod angular;
mod attachment;
mod dependencies;
mod frames;
mod normals;
mod paths;
pub mod prepared;
mod samples;
pub(crate) use attachment::AttachmentSurface;
pub(crate) use dependencies::affected as affected_contacts;
use frames::frames;
use paths::paths;
pub(crate) use paths::straightest;
use samples::sample_path;
/// One complete surface run, in descending order of its largest sample radius.
/// The spans tile the wood index buffer; a caster can draw a single prefix.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceRun {
    pub first_index: u32,
    pub index_count: u32,
    pub largest_radius: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct SurfaceParams {
    pub radial_segments: u32,
    pub lobes: u32,
    pub lobe_depth: f64,
    pub twist_rate: f64,
    pub flare_radius: f64,
    pub flare_falloff: f64,
    pub flare_depth: f64,
    pub fork_socket: f64,
    /// Fraction of the parent's inscribed radius available for a socket.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_socket_containment")
    )]
    pub socket_containment: f64,
    pub fork_swell: f64,
}
impl Default for SurfaceParams {
    fn default() -> Self {
        Self {
            radial_segments: 12,
            lobes: 5,
            lobe_depth: 0.16,
            twist_rate: 1.5,
            flare_radius: 2.1,
            flare_falloff: 0.022,
            flare_depth: 0.004,
            fork_socket: 0.5,
            socket_containment: crate::ranges::default_socket_containment(),
            fork_swell: 1.35,
        }
    }
}
impl SurfaceParams {
    pub fn validate(&self) -> Result<()> {
        use crate::ranges;
        ranges::UNIT.check(self.socket_containment, "socketContainment")?;
        if ![
            (self.radial_segments as f64, ranges::SURFACE_RADIAL_SEGMENTS),
            (self.lobes as f64, ranges::SURFACE_LOBES),
            (self.lobe_depth, ranges::SURFACE_LOBE_DEPTH),
            (self.twist_rate, ranges::SURFACE_TWIST_RATE),
            (self.flare_radius, ranges::SURFACE_FLARE_RADIUS),
            (self.flare_falloff, ranges::SURFACE_FLARE_FALLOFF),
            (self.flare_depth, ranges::UNIT),
            (self.fork_socket, ranges::SURFACE_FORK_SOCKET),
            (self.fork_swell, ranges::SURFACE_FORK_SWELL),
        ]
        .iter()
        .all(|&(v, range)| v.is_finite() && v >= range.0 && v <= range.1)
        {
            return Err(Error::InvalidInput("surface parameters"));
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub min: Vec3,
    pub max: Vec3,
}
#[derive(Debug, Default, PartialEq)]
pub struct SurfaceMesh {
    pub positions: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,
    /// Two floats per vertex: metres along the branch from the root, and the
    /// angle around it in radians. Bark is drawn along them; no geometry here
    /// reads them back.
    pub coords: Vec<f32>,
    pub bounds: Option<Bounds>,
    pub runs: usize,
    pub run_table: Vec<SurfaceRun>,
    /// Triangles dropped because their float32 corners span no area; each
    /// run's span in the index buffer already leaves them out.
    pub dropped: usize,
}
/// The most triangles a tree may drop: two rings' worth, the strips on both
/// sides of each ring collapsed to a point. Past it the collapse is a fault
/// upstream, and the build fails naming the count.
const DROPPED_RINGS: usize = 2;
#[derive(Clone, Copy)]
struct Sample {
    p: Vec3,
    r: f64,
    d: f64,
}
fn reserved<T>(n: usize) -> Result<Vec<T>> {
    let mut out = Vec::new();
    out.try_reserve_exact(n)
        .map_err(|_| Error::ResourceLimit("surface allocation"))?;
    Ok(out)
}
fn filled<T: Clone>(n: usize, value: T) -> Result<Vec<T>> {
    let mut out = reserved(n)?;
    out.resize(n, value);
    Ok(out)
}
fn vertex(out: &mut Vec<f32>, p: Vec3) -> Result<()> {
    let xyz = [p.x as f32, p.y as f32, p.z as f32];
    if !xyz.iter().all(|v| v.is_finite()) {
        return Err(Error::InvalidInput("surface float32 position overflow"));
    }
    out.extend(xyz);
    Ok(())
}

/// Builds only wood geometry. Invalid input or allocation failure returns no partial mesh.
pub fn build(tree: &Tree, height: f64, params: &SurfaceParams) -> Result<SurfaceMesh> {
    build_inner(tree, height, params, None)
}

fn build_inner(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    mut prepared: Option<&mut prepared::PreparedSurface>,
) -> Result<SurfaceMesh> {
    tree.validate()?;
    params.validate()?;
    if !height.is_finite() || height <= 0.0 {
        return Err(Error::InvalidInput("surface height"));
    }
    let nodes = &tree.nodes;
    let paths = paths(nodes)?;
    if paths.runs.is_empty() {
        return Ok(SurfaceMesh::default());
    }
    tree.validate_solved()?;
    let height = height.max(1e-6);
    let segments = params.radial_segments.max(params.lobes * 4) as usize;
    let angular = angular::samples(segments, params)?;
    let twist = params.twist_rate;
    let burial = params.flare_depth * height;
    let mut distance = filled(nodes.len(), 0.0)?;
    for i in 1..nodes.len() {
        let p = nodes[i].parent.unwrap() as usize;
        distance[i] = distance[p] + nodes[p].position.distance(nodes[i].position);
        if !distance[i].is_finite() {
            return Err(Error::InvalidInput("surface path length overflow"));
        }
    }
    let rings = paths
        .nodes
        .len()
        .checked_add(if burial > 0.0 {
            paths.runs.iter().filter(|r| r.trunk).count()
        } else {
            0
        })
        .ok_or(Error::ResourceLimit("surface rings"))?;
    let vertices = rings
        .checked_mul(segments)
        .and_then(|n| n.checked_add(paths.runs.len().checked_mul(2)?))
        .filter(|&n| n <= u32::MAX as usize)
        .ok_or(Error::ResourceLimit("surface vertices"))?;
    let positions_len = vertices
        .checked_mul(3)
        .ok_or(Error::ResourceLimit("surface positions"))?;
    let indices_len = rings
        .checked_mul(segments)
        .and_then(|n| n.checked_mul(6))
        .ok_or(Error::ResourceLimit("surface indices"))?;
    let mut mesh = SurfaceMesh {
        positions: reserved(positions_len)?,
        normals: reserved(if prepared.is_some() { 0 } else { positions_len })?,
        coords: reserved(if prepared.is_some() { 0 } else { vertices * 2 })?,
        indices: reserved(if prepared.is_some() { 0 } else { indices_len })?,
        bounds: None,
        runs: paths.runs.len(),
        run_table: reserved(paths.runs.len())?,
        dropped: 0,
    };
    if let Some(p) = prepared.as_deref_mut() {
        p.segments = segments as u32;
        p.rings = reserved(rings)?;
        p.runs = reserved(paths.runs.len())?;
        p.angles = reserved(segments)?;
        p.angles.extend(angular.iter().map(|a| a.angle as f32));
    }
    let longest = paths
        .runs
        .iter()
        .map(|p| p.end - p.start)
        .max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or(Error::ResourceLimit("surface samples"))?;
    let mut samples = reserved(longest)?;
    let mut frame = reserved(longest)?;
    let mut segments_scratch = reserved(longest)?;
    // Sample once to rank the runs, then reuse the same scratch for emission.
    // Ties retain path order, so the permutation is deterministic.
    let mut ordered = reserved(paths.runs.len())?;
    for path in &paths.runs {
        sample_path(tree, height, params, &paths, path, &distance, &mut samples);
        let radius = samples.iter().map(|s| s.r).fold(0.0, f64::max);
        ordered.push((path, radius));
    }
    ordered.sort_by(|a, b| b.1.total_cmp(&a.1));
    for (path, largest_radius) in ordered {
        let first_index = u32::try_from(
            prepared
                .as_ref()
                .map_or(mesh.indices.len(), |p| p.index_count as usize),
        )
        .map_err(|_| Error::ResourceLimit("surface indices"))?;
        sample_path(tree, height, params, &paths, path, &distance, &mut samples);
        frames(&samples, &mut segments_scratch, &mut frame);
        let base = (mesh.positions.len() / 3) as u32;
        let seg = segments as u32;
        for (i, s) in samples.iter().enumerate() {
            let (normal, binormal) = frame[i];
            let phase = std::f64::consts::TAU * twist * (s.d / height);
            for sample in &angular {
                let angle = sample.angle;
                let profile = sample.profile(params, phase);
                let width = s.r * profile;
                vertex(
                    &mut mesh.positions,
                    s.p + (normal * sample.cos + binormal * sample.sin) * (width),
                )?;
                if prepared.is_none() {
                    mesh.coords.extend([s.d as f32, angle as f32]);
                }
            }
        }
        vertex(&mut mesh.positions, samples[0].p)?;
        // A cap sits on the axis, where the angle around it is undefined.
        if prepared.is_none() {
            mesh.coords.extend([samples[0].d as f32, 0.0]);
        }
        let last = *samples.last().unwrap();
        vertex(&mut mesh.positions, last.p)?;
        if prepared.is_none() {
            mesh.coords.extend([last.d as f32, 0.0]);
        }
        let run = prepared::Run {
            base,
            first_index,
            ring_start: prepared.as_ref().map_or(0, |p| p.rings.len() as u32),
            rings: samples.len() as u32,
            index_count: u32::try_from(samples.len() * segments * 6)
                .map_err(|_| Error::ResourceLimit("surface indices"))?,
        };
        let end = first_index
            .checked_add(run.index_count)
            .ok_or(Error::ResourceLimit("surface indices"))?;
        if let Some(p) = prepared.as_deref_mut() {
            run.visit_triangles(seg, |triangle| {
                if !prepared::admitted(&mesh.positions, triangle)? {
                    p.fallback = true;
                }
                Ok(())
            })?;
            for (i, sample) in samples.iter().enumerate() {
                let start = base as usize + i * segments;
                p.rings.push([
                    sample.d as f32,
                    prepared::ring_radius(&mesh.positions[start * 3..(start + segments) * 3]),
                ]);
            }
            p.runs.push(run);
            p.index_count = end;
        } else {
            for i in 0..samples.len() - 1 {
                let lower = base + i as u32 * seg;
                let upper = lower + seg;
                for k in 0..seg {
                    let next = (k + 1) % seg;
                    mesh.indices.extend([
                        lower + k,
                        lower + next,
                        upper + k,
                        lower + next,
                        upper + next,
                        upper + k,
                    ]);
                }
            }
            let bottom = base + samples.len() as u32 * seg;
            let top_ring = bottom - seg;
            for k in 0..seg {
                let next = (k + 1) % seg;
                mesh.indices.extend([
                    bottom,
                    base + next,
                    base + k,
                    bottom + 1,
                    top_ring + k,
                    top_ring + next,
                ]);
            }
            mesh.normals.resize(mesh.positions.len(), 0.0);
            mesh.dropped += normals::shade(
                &mesh.positions,
                &mut mesh.indices,
                &mut mesh.normals,
                (first_index as usize, base as usize),
                |j| facing(&frame, segments, j),
            )?;
        }
        let end = prepared.as_ref().map_or_else(
            || {
                u32::try_from(mesh.indices.len())
                    .map_err(|_| Error::ResourceLimit("surface indices"))
            },
            |_| Ok(end),
        )?;
        mesh.run_table.push(SurfaceRun {
            first_index,
            index_count: end - first_index,
            largest_radius,
        });
    }
    let mut mesh = finish(mesh, segments)?;
    if let Some(p) = prepared {
        p.positions = std::mem::take(&mut mesh.positions);
        p.run_table = std::mem::take(&mut mesh.run_table);
        p.bounds = mesh.bounds;
    }
    Ok(mesh)
}

/// The way run vertex `j` faces when no triangle is left to say: out from the
/// axis for a ring vertex, along it for the two caps that sit on it.
fn facing(frame: &[(Vec3, Vec3)], segments: usize, j: usize) -> Vec3 {
    let ring = frame.len() * segments;
    if j < ring {
        let (normal, binormal) = frame[j / segments];
        let angle = ((j % segments) as f64 / segments as f64) * std::f64::consts::TAU;
        return normal * angle.cos_fixed() + binormal * angle.sin_fixed();
    }
    let (at, away) = if j == ring {
        (0, -1.0)
    } else {
        (frame.len() - 1, 1.0)
    };
    let (normal, binormal) = frame[at];
    normal.cross(binormal) * away
}

fn finish(mut mesh: SurfaceMesh, segments: usize) -> Result<SurfaceMesh> {
    if mesh.dropped > DROPPED_RINGS * 2 * segments {
        return Err(Error::InvalidValue {
            field: "surface triangles collapsed in float32",
            value: mesh.dropped.to_string(),
        });
    }
    let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for p in mesh.positions.as_chunks::<3>().0 {
        min.x = min.x.min(p[0] as f64);
        min.y = min.y.min(p[1] as f64);
        min.z = min.z.min(p[2] as f64);
        max.x = max.x.max(p[0] as f64);
        max.y = max.y.max(p[1] as f64);
        max.z = max.z.max(p[2] as f64);
    }
    mesh.bounds = Some(Bounds { min, max });
    Ok(mesh)
}

#[cfg(test)]
mod fork_tests;
#[cfg(test)]
mod tests {
    #[test]
    fn allocation_failure_is_explicit() {
        assert!(matches!(
            super::reserved::<f32>(usize::MAX),
            Err(crate::Error::ResourceLimit(_))
        ));
    }
}
