//! Compact ring inputs. Consumers must admit emitted geometry before use.
//! This does not admit geometry: the consumer must validate emitted triangles.
//!
//! The rings are the curve's points (fn-212 R2): the same sweep, the same
//! run order, every word equal to the bit (measured on the beech and the
//! spruce at 80), so they are read from the curve, never swept again.
use super::curve::{traced, Traced};
use super::*;

#[derive(Debug, Default)]
pub struct CompactSurface {
    /// centre/radius, normal/along, binormal/phase, then vertex/cap offsets.
    pub rings: Vec<[u32; 16]>,
    /// angle, cosine, sine, unused.
    pub angular: Vec<[f32; 4]>,
    pub runs: Vec<prepared::Run>,
    pub run_table: Vec<SurfaceRun>,
    pub vertices: u32,
    pub indices: u32,
    pub segments: u32,
    /// The section's lobes and their depth, which the positions pass lays
    /// each ring's outline with (fn-212 task 11).
    pub lobes: u32,
    pub depth: f32,
    /// Whether any run is drawn as a lattice cell. The ring words carry a round
    /// section only, so a shaped surface is never qualified for them.
    pub shaped: bool,
}

pub fn prepare(tree: &Tree, height: f64, params: &SurfaceParams) -> Result<CompactSurface> {
    read(&traced(tree, height, params)?, params, None)
}

/// Conservative capability bound for the GPU position path, not an engine limit.
pub fn qualified_ring(centre: [f32; 3], radius: f32) -> bool {
    let maximum = centre.iter().map(|v| v.abs()).fold(1.0_f32, f32::max);
    centre.iter().all(|v| v.is_finite() && v.abs() <= 64.0)
        && radius.is_finite()
        && radius <= 32.0
        && radius >= maximum / 131072.0
}
impl CompactSurface {
    pub fn qualified(&self) -> bool {
        !self.shaped
            && self.rings.iter().all(|r| {
                qualified_ring([r[0], r[1], r[2]].map(f32::from_bits), f32::from_bits(r[3]))
            })
    }
}

/// Contact ranges index the packed GPU vertices emitted from this exact preparation.
pub struct CompactWithContacts<'a> {
    pub(crate) surface: CompactSurface,
    pub(crate) tree: &'a Tree,
    pub(crate) params: &'a SurfaceParams,
    pub(crate) height: f64,
    pub(crate) edges: Vec<Option<[usize; 4]>>,
}
impl CompactWithContacts<'_> {
    pub fn surface(&self) -> &CompactSurface {
        &self.surface
    }
    pub fn into_surface(self) -> CompactSurface {
        self.surface
    }
    pub fn contact_bytes(&self) -> usize {
        self.edges.capacity() * size_of::<Option<[usize; 4]>>()
    }
}
pub fn prepare_with_contacts<'a>(
    tree: &'a Tree,
    height: f64,
    params: &'a SurfaceParams,
) -> Result<CompactWithContacts<'a>> {
    with_contacts(&traced(tree, height, params)?, tree, height, params)
}

/// The compact rings of `traced`, the curve of `tree`, with the node
/// contacts seated leaves read.
pub(crate) fn with_contacts<'a>(
    traced: &Traced,
    tree: &'a Tree,
    height: f64,
    params: &'a SurfaceParams,
) -> Result<CompactWithContacts<'a>> {
    let mut edges = filled(tree.nodes.len(), None)?;
    let surface = read(traced, params, Some(&mut edges))?;
    Ok(CompactWithContacts {
        surface,
        tree,
        params,
        height,
        edges,
    })
}

/// Each curve point as a ring's words, run by run in the curve's order,
/// with the vertex and cap offsets the GPU emits them at.
fn read(
    traced: &Traced,
    params: &SurfaceParams,
    mut contacts: Option<&mut Vec<Option<[usize; 4]>>>,
) -> Result<CompactSurface> {
    let (curve, paths) = (&traced.curve, &traced.paths);
    if paths.runs.is_empty() {
        return Ok(CompactSurface::default());
    }
    let segments = params.radial_segments.max(params.lobes * 4);
    let rings = curve.points.len();
    let vertices = rings
        .checked_mul(segments as usize)
        .and_then(|n| n.checked_add(paths.runs.len().checked_mul(2)?))
        .and_then(|n| u32::try_from(n).ok())
        .ok_or(Error::ResourceLimit("surface vertices"))?;
    let indices = rings
        .checked_mul(segments as usize)
        .and_then(|n| n.checked_mul(6))
        .and_then(|n| u32::try_from(n).ok())
        .ok_or(Error::ResourceLimit("surface indices"))?;
    let mut out = CompactSurface {
        rings: reserved(rings)?,
        angular: reserved(segments as usize)?,
        runs: reserved(paths.runs.len())?,
        run_table: reserved(paths.runs.len())?,
        vertices,
        indices,
        segments,
        lobes: params.lobes,
        depth: params.lobe_depth as f32,
        shaped: !curve.sections.is_empty(),
    };
    out.angular.extend(
        angular::samples(segments as usize, params)?
            .iter()
            .map(|a| [a.angle as f32, a.cos as f32, a.sin as f32, 0.0]),
    );
    let mut base = 0;
    let mut first_index = 0;
    for (run, &path_id) in curve.runs.iter().zip(&traced.order) {
        let path = &paths.runs[path_id];
        let points = &curve.points[run.first as usize..(run.first + run.count) as usize];
        let count = run.count;
        let cap = base + count * segments;
        let ring_run = prepared::Run {
            base,
            first_index,
            ring_start: out.rings.len() as u32,
            rings: count,
            index_count: count * segments * 6,
        };
        if let Some(edges) = contacts.as_deref_mut() {
            let offset = usize::from(path.trunk && params.flare_depth > 0.0);
            for (i, &node) in paths.nodes[path.start..path.end].iter().enumerate().skip(1) {
                edges[node] = Some([
                    base as usize + (i - 1 + offset) * segments as usize,
                    base as usize + (i + offset) * segments as usize,
                    base as usize,
                    base as usize + (points.len() - 1) * segments as usize,
                ]);
            }
        }
        for (i, p) in points.iter().enumerate() {
            let floats = [
                p.centre.x,
                p.centre.y,
                p.centre.z,
                p.radius,
                p.normal.x,
                p.normal.y,
                p.normal.z,
                p.along,
                p.binormal.x,
                p.binormal.y,
                p.binormal.z,
                std::f64::consts::TAU * params.twist_rate * (p.along / curve.height),
            ]
            .map(|v| v as f32);
            if !floats.iter().all(|v| v.is_finite()) {
                return Err(Error::InvalidInput("compact surface float32 overflow"));
            }
            let mut ring = [0; 16];
            for (word, value) in ring.iter_mut().zip(floats) {
                *word = value.to_bits();
            }
            ring[12] = base + i as u32 * segments;
            ring[13] = if i == 0 { cap } else { u32::MAX };
            ring[14] = if i + 1 == points.len() {
                cap + 1
            } else {
                u32::MAX
            };
            out.rings.push(ring);
        }
        out.runs.push(ring_run);
        out.run_table.push(SurfaceRun {
            first_index,
            index_count: ring_run.index_count,
            largest_radius: run.largest_radius,
        });
        first_index += ring_run.index_count;
        base = cap + 2;
    }
    Ok(out)
}
