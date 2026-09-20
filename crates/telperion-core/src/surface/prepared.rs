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
    let mut out = PreparedSurface::default();
    super::build_inner(tree, height, params, Some(&mut out))?;
    Ok((!out.fallback).then_some(out))
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
