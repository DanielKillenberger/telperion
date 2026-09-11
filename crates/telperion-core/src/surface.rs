//! Independent, closed swept shells over solved tree paths. Buffers are caller-owned.
use crate::{math::Vec3, tree::Tree, Error, Result};
mod attachment;
mod frames;
mod paths;
pub(crate) use attachment::AttachmentSurface;
use frames::frames;
use paths::paths;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceParams {
    pub radial_segments: u32,
    pub lobes: u32,
    pub lobe_depth: f64,
    pub twist_rate: f64,
    pub flare_radius: f64,
    pub flare_falloff: f64,
    pub flare_depth: f64,
    pub fork_socket: f64,
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
            fork_swell: 1.35,
        }
    }
}
impl SurfaceParams {
    pub fn validate(&self) -> Result<()> {
        if !(3..=64).contains(&self.radial_segments)
            || self.lobes > 16
            || ![
                (self.lobe_depth, 0.0, 0.9),
                (self.twist_rate, -64.0, 64.0),
                (self.flare_radius, 1.0, 8.0),
                (self.flare_falloff, 1e-4, 1.0),
                (self.flare_depth, 0.0, 1.0),
                (self.fork_socket, 0.0, 0.9),
                (self.fork_swell, 1.0, 4.0),
            ]
            .iter()
            .all(|&(v, lo, hi)| v.is_finite() && v >= lo && v <= hi)
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
}
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

fn sample_path(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    path_nodes: &[usize],
    trunk: bool,
    distance: &[f64],
    samples: &mut Vec<Sample>,
) {
    let nodes = &tree.nodes;
    let depth = params.lobe_depth;
    let segments = params.radial_segments.max(params.lobes * 4) as usize;
    let burial = params.flare_depth * height;
    let socket = params.fork_socket;
    let swell = params.fork_swell;
    let flare = |y: f64| {
        1.0 + (params.flare_radius - 1.0) * (-y.max(0.0) / (params.flare_falloff * height)).exp()
    };
    samples.clear();
    if trunk {
        let root = &nodes[path_nodes[0]];
        if burial > 0.0 {
            samples.push(Sample {
                p: Vec3::new(root.position.x, root.position.y - burial, root.position.z),
                r: root.radius * flare(root.position.y),
                d: 0.0,
            });
        }
        for &i in path_nodes {
            samples.push(Sample {
                p: nodes[i].position,
                r: nodes[i].radius * flare(nodes[i].position.y),
                d: distance[i],
            });
        }
    } else {
        let attach = path_nodes[0];
        let first = path_nodes[1];
        let pr = nodes[attach].radius;
        let away = (nodes[first].position - nodes[attach].position).normalized();
        let inscribed = pr * (1.0 - depth) * (std::f64::consts::PI / segments as f64).cos();
        let sink = (socket * pr).min(0.9 * inscribed);
        let contained = (inscribed * inscribed - sink * sink).max(0.0).sqrt() / (1.0 + depth);
        samples.push(Sample {
            p: nodes[attach].position + away * (-sink),
            r: (nodes[first].start_radius * swell).min(contained) * flare(nodes[attach].position.y),
            d: distance[attach],
        });
        for &i in &path_nodes[1..] {
            let swelling =
                1.0 + (swell - 1.0) * (-(distance[i] - distance[attach]) / pr.max(1e-9)).exp();
            samples.push(Sample {
                p: nodes[i].position,
                r: nodes[i].radius * swelling * flare(nodes[i].position.y),
                d: distance[i],
            });
        }
    }
}
/// Builds only wood geometry. Invalid input or allocation failure returns no partial mesh.
pub fn build(tree: &Tree, height: f64, params: &SurfaceParams) -> Result<SurfaceMesh> {
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
    let lobes = params.lobes as f64;
    let segments = params.radial_segments.max(params.lobes * 4) as usize;
    let depth = params.lobe_depth;
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
        .checked_add(usize::from(burial > 0.0))
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
        normals: reserved(positions_len)?,
        coords: reserved(vertices * 2)?,
        indices: reserved(indices_len)?,
        bounds: None,
        runs: paths.runs.len(),
    };
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
    for path in &paths.runs {
        let path_nodes = &paths.nodes[path.start..path.end];
        sample_path(
            tree,
            height,
            params,
            path_nodes,
            path.trunk,
            &distance,
            &mut samples,
        );
        frames(&samples, &mut segments_scratch, &mut frame);
        let base = (mesh.positions.len() / 3) as u32;
        let seg = segments as u32;
        for (i, s) in samples.iter().enumerate() {
            let (normal, binormal) = frame[i];
            let phase = std::f64::consts::TAU * twist * (s.d / height);
            for k in 0..segments {
                let angle = (k as f64 / segments as f64) * std::f64::consts::TAU;
                let profile = if lobes == 0.0 {
                    1.0
                } else {
                    1.0 + depth * (lobes * (angle + phase)).cos()
                };
                let width = s.r * profile;
                vertex(
                    &mut mesh.positions,
                    s.p + (normal * (angle.cos()) + binormal * (angle.sin())) * (width),
                )?;
                mesh.coords.extend([s.d as f32, angle as f32]);
            }
        }
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
        let bottom = (mesh.positions.len() / 3) as u32;
        vertex(&mut mesh.positions, samples[0].p)?;
        // A cap sits on the axis, where the angle around it is undefined.
        mesh.coords.extend([samples[0].d as f32, 0.0]);
        let top = bottom + 1;
        let last = *samples.last().unwrap();
        vertex(&mut mesh.positions, last.p)?;
        mesh.coords.extend([last.d as f32, 0.0]);
        let top_ring = base + (samples.len() as u32 - 1) * seg;
        for k in 0..seg {
            let next = (k + 1) % seg;
            mesh.indices.extend([
                bottom,
                base + next,
                base + k,
                top,
                top_ring + k,
                top_ring + next,
            ]);
        }
    }
    finish(mesh)
}

fn finish(mut mesh: SurfaceMesh) -> Result<SurfaceMesh> {
    mesh.normals.resize(mesh.positions.len(), 0.0);
    let point = |index: u32| -> Result<Vec3> {
        let offset = (index as usize)
            .checked_mul(3)
            .ok_or(Error::ResourceLimit("surface index"))?;
        let p = mesh
            .positions
            .get(offset..offset + 3)
            .ok_or(Error::InvalidInput("surface index"))?;
        Ok(Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64))
    };
    for t in mesh.indices.as_chunks::<3>().0 {
        let (a, b, c) = (point(t[0])?, point(t[1])?, point(t[2])?);
        let normal = (c - b).cross(a - b);
        if !normal.is_finite() || normal.length_squared() == 0.0 {
            return Err(Error::InvalidInput("surface triangle collapsed in float32"));
        }
        for &index in t {
            let offset = index as usize * 3;
            for (k, v) in [normal.x, normal.y, normal.z].iter().enumerate() {
                mesh.normals[offset + k] = (mesh.normals[offset + k] as f64 + v) as f32;
            }
        }
    }
    for n in mesh.normals.as_chunks_mut::<3>().0 {
        let v = Vec3::new(n[0] as f64, n[1] as f64, n[2] as f64);
        if !v.is_finite() || v.length_squared() == 0.0 {
            return Err(Error::InvalidInput(
                "surface normal overflow or cancellation",
            ));
        }
        let v = v.normalized();
        n.copy_from_slice(&[v.x as f32, v.y as f32, v.z as f32]);
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
mod tests {
    #[test]
    fn allocation_failure_is_explicit() {
        assert!(matches!(
            super::reserved::<f32>(usize::MAX),
            Err(crate::Error::ResourceLimit(_))
        ));
    }
}
