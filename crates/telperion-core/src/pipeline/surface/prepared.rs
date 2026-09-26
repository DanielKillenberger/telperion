//! Canonical positions and compact inputs for resident surface expansion.
use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Run {
    pub base: u32,
    pub first_index: u32,
    pub ring_start: u32,
    pub rings: u32,
    pub index_count: u32,
}
impl Run {
    #[inline]
    pub(super) fn visit_triangles(
        &self,
        segments: u32,
        mut visit: impl FnMut([u32; 3]) -> Result<()>,
    ) -> Result<()> {
        for ring in 0..self.rings - 1 {
            let lower = self.base + ring * segments;
            let upper = lower + segments;
            for k in 0..segments {
                let next = (k + 1) % segments;
                visit([lower + k, lower + next, upper + k])?;
                visit([lower + next, upper + next, upper + k])?;
            }
        }
        let bottom = self.base + self.rings * segments;
        let top = bottom - segments;
        for k in 0..segments {
            let next = (k + 1) % segments;
            visit([bottom, self.base + next, self.base + k])?;
            visit([bottom + 1, top + k, top + next])?;
        }
        Ok(())
    }

    /// Triangle in the CPU builder's strip-then-interleaved-cap order.
    pub fn triangle(&self, face: u32, segments: u32) -> [u32; 3] {
        let strip = (self.rings - 1) * segments * 2;
        if face < strip {
            let pair = face / 2;
            let lower = self.base + pair / segments * segments;
            let k = pair % segments;
            let next = (k + 1) % segments;
            if face % 2 == 0 {
                [lower + k, lower + next, lower + segments + k]
            } else {
                [lower + next, lower + segments + next, lower + segments + k]
            }
        } else {
            let cap = face - strip;
            let k = cap / 2;
            let next = (k + 1) % segments;
            let bottom = self.base + self.rings * segments;
            if cap % 2 == 0 {
                [bottom, self.base + next, self.base + k]
            } else {
                let top = bottom - segments;
                [bottom + 1, top + k, top + next]
            }
        }
    }
}
#[derive(Debug, Default)]
pub struct PreparedSurface {
    pub positions: Vec<f32>,
    /// Along-distance and mean radius recovered from the rounded ring corners.
    pub rings: Vec<[f32; 2]>,
    pub angles: Vec<f32>,
    pub runs: Vec<Run>,
    pub run_table: Vec<SurfaceRun>,
    pub bounds: Option<Bounds>,
    pub segments: u32,
    pub index_count: u32,
    pub(super) fallback: bool,
}
/// None requests the ordinary CPU builder for a collapsed or unsupported surface.
pub fn prepare(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
) -> Result<Option<PreparedSurface>> {
    let out = prepared(tree, height, params, false)?.0;
    Ok((!out.fallback).then_some(out))
}
/// Canonical wood and its node contact ranges from the same solved tree.
/// Ring ranges exclude caps and use vertex offsets into `surface.positions`.
pub struct PreparedWithContacts<'a> {
    pub(crate) surface: PreparedSurface,
    pub(crate) tree: &'a Tree,
    pub(crate) params: &'a SurfaceParams,
    pub(crate) height: f64,
    pub(crate) edges: Vec<Option<[usize; 4]>>,
}
impl PreparedWithContacts<'_> {
    pub fn surface(&self) -> &PreparedSurface {
        &self.surface
    }
    pub fn into_surface(self) -> PreparedSurface {
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
) -> Result<Option<PreparedWithContacts<'a>>> {
    let (surface, edges) = prepared(tree, height, params, true)?;
    Ok((!surface.fallback).then_some(PreparedWithContacts {
        surface,
        tree,
        params,
        height,
        edges,
    }))
}

/// The prepared surface on the rings swept in turn: each ring's distance,
/// read from its coords, and its radius from its float32 corners, and each
/// run's span of the index buffer the resident expansion fills.
fn prepared(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    edges: bool,
) -> Result<(PreparedSurface, Vec<Option<[usize; 4]>>)> {
    tree.validate()?;
    let sweep = Sweep { drawn: true, edges };
    let rings = super::rings::rings_mode(tree, height, params, sweep, false)?;
    let mut p = PreparedSurface::default();
    if rings.runs.is_empty() {
        return Ok((p, rings.edges));
    }
    let segments = rings.segments;
    let seg = segments as u32;
    p.segments = seg;
    p.rings = reserved(rings.positions.len() / 3 / segments)?;
    p.runs = reserved(rings.runs.len())?;
    p.angles = reserved(segments)?;
    p.angles.extend(
        angular::samples(segments, params)?
            .iter()
            .map(|a| a.angle as f32),
    );
    let mut base = 0;
    for table in &rings.runs {
        let count = rings.count(table);
        let run = Run {
            base: u32::try_from(base).map_err(|_| Error::ResourceLimit("surface vertices"))?,
            first_index: table.first_index,
            ring_start: p.rings.len() as u32,
            rings: count as u32,
            index_count: table.index_count,
        };
        run.visit_triangles(seg, |triangle| {
            if !admitted(&rings.positions, triangle)? {
                p.fallback = true;
            }
            Ok(())
        })?;
        for i in 0..count {
            let start = base + i * segments;
            p.rings.push([
                rings.coords[start * 2],
                ring_radius(&rings.positions[start * 3..(start + segments) * 3]),
            ]);
        }
        p.runs.push(run);
        p.index_count = table.first_index + table.index_count;
        base += rings.vertices(table);
    }
    p.bounds = Some(bounds(&rings.positions));
    p.positions = rings.positions;
    p.run_table = rings.runs;
    Ok((p, rings.edges))
}
pub(super) fn admitted(positions: &[f32], t: [u32; 3]) -> Result<bool> {
    let point = |i: u32| {
        let p = &positions[i as usize * 3..][..3];
        Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)
    };
    let [a, b, c] = t.map(point);
    let n = (c - b).cross(a - b);
    if !n.is_finite() {
        return Err(Error::InvalidInput("surface float32 position overflow"));
    }
    let scale = n.x.abs().max(n.y.abs()).max(n.z.abs());
    Ok(n.length_squared() > 0.0
        && scale >= f32::MIN_POSITIVE as f64
        && scale < (f32::MAX as f64 / 256.0))
}
/// Renderer bark metric, calculated in f64 from float32 positions.
pub fn ring_radius(positions: &[f32]) -> f32 {
    let point = |p: &[f32; 3]| Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64);
    let points = positions.as_chunks::<3>().0;
    let centre = points.iter().map(point).fold(Vec3::ZERO, |sum, p| sum + p) / points.len() as f64;
    (points
        .iter()
        .map(|p| point(p).distance(centre))
        .sum::<f64>()
        / points.len() as f64) as f32
}

#[cfg(test)]
mod traversal_tests {
    use super::*;

    #[test]
    fn nested_faces_match_procedural_order() {
        for (base, rings, segments) in [(0, 2, 3), (17, 4, 8), (101, 1025, 17)] {
            let run = Run {
                base,
                rings,
                first_index: 33,
                ring_start: 7,
                index_count: rings * segments * 6,
            };
            let mut face = 0;
            run.visit_triangles(segments, |triangle| {
                assert_eq!(triangle, run.triangle(face, segments));
                face += 1;
                Ok(())
            })
            .unwrap();
            assert_eq!(face, run.index_count / 3);
        }
    }
}
