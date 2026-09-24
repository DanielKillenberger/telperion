//! The sweep itself: one closed shell over every solved path.
use super::*;
/// The most triangles a tree may drop: two rings' worth, the strips on both
/// sides of each ring collapsed to a point. Past it the collapse is a fault
/// upstream, and the build fails naming the count.
pub(super) const DROPPED_RINGS: usize = 2;
#[derive(Clone, Copy)]
pub(super) struct Sample {
    pub(super) p: Vec3,
    pub(super) r: f64,
    pub(super) d: f64,
}
/// Radial segments a ring is cut into: never fewer than four to a lobe.
pub(super) fn segments(params: &SurfaceParams) -> usize {
    params.radial_segments.max(params.lobes * 4) as usize
}

/// The buffers these paths fill: one ring a path node, one more at the foot of
/// every trunk run the flare buries, two cap vertices a run, and six indices a
/// ring segment.
pub(super) fn extent_of(paths: &paths::Paths, segments: usize, buried: bool) -> Result<WoodExtent> {
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
    crate::surface::height(height)?;
    let paths = paths(&tree.nodes)?;
    if paths.runs.is_empty() {
        return Ok(WoodExtent::default());
    }
    let burial = params.flare_depth * height.max(1e-6);
    extent_of(&paths, segments(params), burial > 0.0)
}

/// Builds only wood geometry. Invalid input or allocation failure returns no partial mesh.
pub fn build(tree: &Tree, height: f64, params: &SurfaceParams) -> Result<SurfaceMesh> {
    let sweep = Sweep {
        drawn: true,
        edges: false,
    };
    let rings = rings(tree, height, params, sweep)?;
    let faces = faces(&rings, tree, height, params)?;
    Ok(rings.into_mesh(faces))
}

/// What the mesh step adds around the rings: everything the wood holds but
/// its positions and coords, and the run table the rings hold where the step
/// dropped no triangle.
#[derive(Debug, Default)]
pub(crate) struct Faces {
    pub(super) normals: Vec<f32>,
    pub(super) indices: Vec<u32>,
    /// The run table, where dropped triangles moved a run's span.
    pub(super) run_table: Option<Vec<SurfaceRun>>,
    pub(super) bounds: Option<Bounds>,
    pub(super) dropped: usize,
}

/// The mesh step: indices, normals and bounds around rings swept for a
/// drawn wood, on the sweep's workers where it had them.
pub(crate) fn faces(
    rings: &Rings,
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
) -> Result<Faces> {
    if rings.runs.is_empty() {
        return Ok(Faces::default());
    }
    let resweep = Resweep::new(tree, height, params);
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    if let Some(workers) = rings.workers {
        if let Ok(faces) = parallel::faces(rings, &resweep, workers) {
            return Ok(faces);
        }
    }
    let ring_vertices = rings.positions.len() / 3 - rings.runs.len() * 2;
    let mut faces = Faces {
        normals: reserved(rings.positions.len())?,
        indices: reserved(ring_vertices * 6)?,
        ..Faces::default()
    };
    let mut base = 0;
    for (i, run) in rings.runs.iter().enumerate() {
        let first_index = u32::try_from(faces.indices.len())
            .map_err(|_| Error::ResourceLimit("surface indices"))?;
        faces
            .normals
            .resize(faces.normals.len() + rings.vertices(run) * 3, 0.0);
        let normals = &mut faces.normals[base * 3..];
        let indices = &mut faces.indices;
        let keep = |t: [u32; 3]| indices.extend(t);
        faces.dropped += shade(rings, i, base, &resweep, normals, keep)?;
        let index_count = u32::try_from(faces.indices.len())
            .map_err(|_| Error::ResourceLimit("surface indices"))?
            - first_index;
        if (first_index, index_count) != (run.first_index, run.index_count) {
            let table = match &mut faces.run_table {
                Some(table) => table,
                None => faces.run_table.insert(copied(&rings.runs)?),
            };
            table[i] = SurfaceRun {
                first_index,
                index_count,
                largest_radius: run.largest_radius,
            };
        }
        base += rings.vertices(run);
    }
    if faces.dropped > DROPPED_RINGS * 2 * rings.segments {
        return Err(Error::InvalidValue {
            field: "surface triangles collapsed in float32",
            value: faces.dropped.to_string(),
        });
    }
    faces.bounds = Some(bounds(&rings.positions));
    Ok(faces)
}

fn copied<T: Copy>(values: &[T]) -> Result<Vec<T>> {
    let mut out = reserved(values.len())?;
    out.extend_from_slice(values);
    Ok(out)
}

/// Run `run` of the table's faces, its vertices starting at `base`: each
/// triangle with area handed to `keep` in draw order, and the run's unit
/// normals summed into `normals`, which starts at zero. A vertex no triangle
/// shades faces the way its ring does. Returns the triangles dropped.
pub(super) fn shade(
    rings: &Rings,
    run: usize,
    base: usize,
    resweep: &Resweep,
    normals: &mut [f32],
    keep: impl FnMut([u32; 3]),
) -> Result<usize> {
    let segments = rings.segments;
    let count = rings.count(&rings.runs[run]);
    let vertices = count * segments + 2;
    let positions = &rings.positions[base * 3..(base + vertices) * 3];
    let at = prepared::Run {
        base: base as u32,
        first_index: 0,
        ring_start: 0,
        rings: count as u32,
        index_count: 0,
    };
    let normals = &mut normals[..vertices * 3];
    let dropped = normals::accumulate(positions, &at, segments as u32, normals, keep)?;
    let mut frame = None;
    normals::normalize(normals, |j| {
        if frame.is_none() {
            frame = Some(resweep.frames(run)?);
        }
        let frame = frame.as_deref().expect("swept on first use");
        Ok(facing(frame, segments, j))
    })?;
    Ok(dropped)
}

/// Records where each node of one run meets the rings: its lower and upper
/// ring, then the run's first and last, as vertex offsets from `base`.
pub(super) fn record_edges(
    edges: &mut [Option<[usize; 4]>],
    nodes: &[usize],
    base: usize,
    rings: usize,
    segments: usize,
    offset: usize,
) {
    for (i, &node) in nodes.iter().enumerate().skip(1) {
        edges[node] = Some([
            base + (i - 1 + offset) * segments,
            base + (i + offset) * segments,
            base,
            base + (rings - 1) * segments,
        ]);
    }
}

/// A run's vertices, ring by ring, then its two caps where `caps` asks.
pub(super) fn emit_run(
    samples: &[Sample],
    frame: &[(Vec3, Vec3)],
    at: Swept,
    caps: bool,
    mut emit: impl FnMut([f32; 3], [f32; 2]),
) -> Result<()> {
    let (params, height) = (at.params, at.height);
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
        for sample in at.angular {
            let width = s.r * sample.profile(params, phase);
            vertex(
                s.p + (normal * sample.cos + binormal * sample.sin) * width,
                [s.d as f32, sample.angle as f32],
            )?;
        }
    }
    if !caps {
        return Ok(());
    }
    vertex(samples[0].p, [samples[0].d as f32, 0.0])?;
    let last = samples.last().unwrap();
    vertex(last.p, [last.d as f32, 0.0])
}

/// The way run vertex `j` faces when no triangle is left to say: out from the
/// axis for a ring vertex, along it for the two caps that sit on it.
pub(super) fn facing(frame: &[(Vec3, Vec3)], segments: usize, j: usize) -> Vec3 {
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

/// The box around every vertex, widened from float32.
pub(super) fn bounds(positions: &[f32]) -> Bounds {
    let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for p in positions.as_chunks::<3>().0 {
        min.x = min.x.min(p[0] as f64);
        min.y = min.y.min(p[1] as f64);
        min.z = min.z.min(p[2] as f64);
        max.x = max.x.max(p[0] as f64);
        max.y = max.y.max(p[1] as f64);
        max.z = max.z.max(p[2] as f64);
    }
    Bounds { min, max }
}
