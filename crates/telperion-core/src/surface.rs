//! Independent, closed swept shells over solved tree paths. Buffers are caller-owned.
use crate::math::Transcendental;
use crate::{math::Vec3, tree::Tree, Error, Result};
mod angular;
mod attachment;
#[doc(hidden)]
pub mod compact;
mod dependencies;
mod frames;
mod normals;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod parallel;
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

/// The elements a tree's wood buffers hold once it is swept. `build` reserves
/// by it and a prediction sizes the specimen by it, so the ring arithmetic is
/// written once and read twice.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WoodExtent {
    /// Floats in `positions`, and the same count again in `normals`.
    pub positions: usize,
    /// Floats in `coords`: two a vertex.
    pub coords: usize,
    /// Indices in `indices`.
    pub indices: usize,
}
impl WoodExtent {
    /// Bytes the four buffers keep, each count times the size of the type
    /// that holds it. `positions` is counted twice: `normals` is its equal.
    pub fn bytes(&self) -> usize {
        (self.positions * 2 + self.coords) * size_of::<f32>() + self.indices * size_of::<u32>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct SurfaceParams {
    /// How many sides each piece of wood is drawn with. Raising it makes
    /// the wood rounder and smoother, and costs triangles.
    pub radial_segments: u32,
    /// How many ridges run up around the trunk. Raising it gives the
    /// bark more flutes; zero is a plain round bole.
    pub lobes: u32,
    /// How deep the flutes between those ridges cut, as a share of the
    /// wood's own radius. Raising it makes the fluting more pronounced. At
    /// zero the bole is plainly round whatever the ridge count says, and any
    /// rise starts cutting the flutes.
    pub lobe_depth: f64,
    /// How many turns those ridges make over the tree's height. Raising
    /// it winds them more tightly around the trunk.
    pub twist_rate: f64,
    /// How much wider the trunk is where it meets the ground, as a
    /// multiple of its own radius. Raising it gives a broader buttress.
    pub flare_radius: f64,
    /// How far up the trunk that flare reaches, as a share of the
    /// height. Raising it carries the swelling further up the bole.
    pub flare_falloff: f64,
    /// How deep the trunk's base is sunk below the ground, as a share
    /// of the height. Raising it buries more of the flare.
    pub flare_depth: f64,
    /// How deeply a child branch is set into its parent at a fork.
    /// Raising it sinks the junction further in, so the two read as one
    /// piece of wood rather than two tubes meeting.
    pub fork_socket: f64,
    /// Fraction of the parent's inscribed radius available for a socket.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_socket_containment")
    )]
    pub socket_containment: f64,
    /// How much wood thickens at a fork. Raising it leaves a more
    /// pronounced collar where a branch leaves its parent.
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
/// Radial segments a ring is cut into: never fewer than four to a lobe.
fn segments(params: &SurfaceParams) -> usize {
    params.radial_segments.max(params.lobes * 4) as usize
}

/// The buffers these paths fill: one ring a path node, one more at the foot of
/// every trunk run the flare buries, two cap vertices a run, and six indices a
/// ring segment.
fn extent_of(paths: &paths::Paths, segments: usize, buried: bool) -> Result<WoodExtent> {
    let feet = if buried {
        paths.runs.iter().filter(|run| run.trunk).count()
    } else {
        0
    };
    let rings = paths
        .nodes
        .len()
        .checked_add(feet)
        .ok_or(Error::ResourceLimit("surface rings"))?;
    let ring_vertices = rings
        .checked_mul(segments)
        .ok_or(Error::ResourceLimit("surface vertices"))?;
    let vertices = paths
        .runs
        .len()
        .checked_mul(2)
        .and_then(|caps| ring_vertices.checked_add(caps))
        .filter(|&n| n <= u32::MAX as usize)
        .ok_or(Error::ResourceLimit("surface vertices"))?;
    Ok(WoodExtent {
        positions: vertices
            .checked_mul(3)
            .ok_or(Error::ResourceLimit("surface positions"))?,
        coords: vertices * 2,
        indices: ring_vertices
            .checked_mul(6)
            .ok_or(Error::ResourceLimit("surface indices"))?,
    })
}

/// What `build` would fill for this tree, without sweeping it: the same paths
/// pass and the same ring arithmetic, and no mesh.
pub fn extent(tree: &Tree, height: f64, params: &SurfaceParams) -> Result<WoodExtent> {
    tree.validate()?;
    params.validate()?;
    if !height.is_finite() || height <= 0.0 {
        return Err(Error::InvalidInput("surface height"));
    }
    let paths = paths(&tree.nodes)?;
    if paths.runs.is_empty() {
        return Ok(WoodExtent::default());
    }
    let burial = params.flare_depth * height.max(1e-6);
    extent_of(&paths, segments(params), burial > 0.0)
}

/// Builds only wood geometry. Invalid input or allocation failure returns no partial mesh.
pub fn build(tree: &Tree, height: f64, params: &SurfaceParams) -> Result<SurfaceMesh> {
    build_inner(tree, height, params, None, None)
}

fn build_inner(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    prepared: Option<&mut prepared::PreparedSurface>,
    contacts: Option<&mut Vec<Option<[usize; 4]>>>,
) -> Result<SurfaceMesh> {
    build_mode(tree, height, params, prepared, contacts, true)
}

fn build_mode(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    mut prepared: Option<&mut prepared::PreparedSurface>,
    mut contacts: Option<&mut Vec<Option<[usize; 4]>>>,
    parallel_allowed: bool,
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
    let segments = segments(params);
    let angular = angular::samples(segments, params)?;
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
    for (path_id, path) in paths.runs.iter().enumerate() {
        sample_path(tree, height, params, &paths, path, &distance, &mut samples);
        let radius = samples.iter().map(|s| s.r).fold(0.0, f64::max);
        ordered.push((path_id, radius));
    }
    ordered.sort_by(|a, b| b.1.total_cmp(&a.1));
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    if parallel_allowed && prepared.is_none() && contacts.is_none() {
        if let Some(workers) = parallel::admitted(
            &paths,
            &distance,
            &ordered,
            &angular,
            longest,
            vertices,
            indices_len,
        ) {
            drop((samples, frame, segments_scratch));
            return match parallel::build(
                tree,
                height,
                params,
                paths,
                distance,
                ordered,
                angular,
                longest,
                vertices,
                indices_len,
                workers,
            ) {
                Ok(mesh) => Ok(mesh),
                Err(_) => build_mode(tree, height, params, None, None, false),
            };
        }
    }
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    let _ = parallel_allowed;
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
    for (path_id, largest_radius) in ordered {
        let path = &paths.runs[path_id];
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
        if let Some(edges) = contacts.as_deref_mut() {
            let offset = usize::from(path.trunk && params.flare_depth > 0.0);
            for (i, &node) in paths.nodes[path.start..path.end].iter().enumerate().skip(1) {
                edges[node] = Some([
                    base as usize + (i - 1 + offset) * segments,
                    base as usize + (i + offset) * segments,
                    base as usize,
                    base as usize + (samples.len() - 1) * segments,
                ]);
            }
        }
        emit_run(&samples, &frame, &angular, params, height, |xyz, coord| {
            mesh.positions.extend(xyz);
            if prepared.is_none() {
                mesh.coords.extend(coord);
            }
        })?;
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

fn emit_run(
    samples: &[Sample],
    frame: &[(Vec3, Vec3)],
    angular: &[angular::Angular],
    params: &SurfaceParams,
    height: f64,
    mut emit: impl FnMut([f32; 3], [f32; 2]),
) -> Result<()> {
    let mut vertex = |p: Vec3, coord: [f32; 2]| {
        let xyz = [p.x as f32, p.y as f32, p.z as f32];
        if !xyz.iter().all(|v| v.is_finite()) {
            return Err(Error::InvalidInput("surface float32 position overflow"));
        }
        emit(xyz, coord);
        Ok(())
    };
    for (i, s) in samples.iter().enumerate() {
        let (normal, binormal) = frame[i];
        let phase = std::f64::consts::TAU * params.twist_rate * (s.d / height);
        for sample in angular {
            let width = s.r * sample.profile(params, phase);
            vertex(
                s.p + (normal * sample.cos + binormal * sample.sin) * width,
                [s.d as f32, sample.angle as f32],
            )?;
        }
    }
    vertex(samples[0].p, [samples[0].d as f32, 0.0])?;
    let last = samples.last().unwrap();
    vertex(last.p, [last.d as f32, 0.0])
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
