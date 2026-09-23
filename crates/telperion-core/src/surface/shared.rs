//! One sweep read twice: the wood surface emitting as its vertices the rings
//! leaf contacts query, and the rings read back from a wood already swept.
use super::*;

/// Builds the wood from a sweep leaf contacts already ran: the same vertices,
/// read from its rings rather than swept a second time.
pub(crate) fn build_swept(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    rings: &AttachmentSurface,
) -> Result<SurfaceMesh> {
    build_run(tree, height, params, None, None, true, Some(rings))
}

/// A sweep already run over every path, whose rings are the wood's vertices.
pub(super) type Swept<'a> = &'a AttachmentSurface;

impl AttachmentSurface {
    /// Path `k`'s first `rings` rings, as the vertices a build emits.
    pub(super) fn run_rings(&self, k: usize, rings: usize) -> &[Vec3] {
        let start = self.starts[k];
        &self.rings[start..start + rings * self.segments]
    }
}

/// A run's vertices as `emit_run` emits them, read from rings a sweep kept:
/// each ring point is already the float32 vertex, so the bytes are the same.
pub(super) fn emit_swept(
    rings: &[Vec3],
    samples: &[Sample],
    angular: &[angular::Angular],
    mut emit: impl FnMut([f32; 3], [f32; 2]),
) -> Result<()> {
    let mut vertex = |p: Vec3, coord: [f32; 2]| {
        emit(vertex32(p)?, coord);
        Ok(())
    };
    for (s, ring) in samples.iter().zip(rings.chunks_exact(angular.len())) {
        for (&p, sample) in ring.iter().zip(angular) {
            vertex(p, [s.d as f32, sample.angle as f32])?;
        }
    }
    vertex(samples[0].p, [samples[0].d as f32, 0.0])?;
    let last = samples.last().unwrap();
    vertex(last.p, [last.d as f32, 0.0])
}

/// Each path's first vertex in a wood surface `build` swept: its runs are
/// laid out largest radius first, ties in path order, each its rings and two
/// caps.
pub(super) fn wood_bases(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    paths: &paths::Paths,
    distance: &[f64],
    samples: &mut Vec<Sample>,
) -> Result<Vec<usize>> {
    let segments = segments(params);
    let mut ordered = reserved(paths.runs.len())?;
    for (k, path) in paths.runs.iter().enumerate() {
        sample_path(tree, height, params, paths, path, distance, samples);
        let radius = samples.iter().map(|s| s.r).fold(0.0, f64::max);
        ordered.push((k, radius, samples.len()));
    }
    ordered.sort_by(|a, b| b.1.total_cmp(&a.1));
    let mut bases = filled(paths.runs.len(), 0)?;
    let mut base = 0;
    for (k, _, rings) in ordered {
        bases[k] = base;
        base += rings * segments + 2;
    }
    Ok(bases)
}
