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
pub(crate) fn segments(params: &SurfaceParams) -> usize {
    params.radial_segments.max(params.lobes * 4) as usize
}

#[cfg(test)]
/// What `build` would fill for this tree, without sweeping it: the same paths
/// pass, the same ranking and the same layout, and no mesh.
pub fn extent(tree: &Tree, height: f64, params: &SurfaceParams) -> Result<WoodExtent> {
    tree.validate()?;
    params.validate()?;
    crate::pipeline::surface::height(height)?;
    let paths = paths(tree)?;
    if paths.runs.is_empty() {
        return Ok(WoodExtent::default());
    }
    let segments = segments(params);
    let distance = rings::distances(tree)?;
    let angular = angular::samples(segments, params)?;
    let at = rings::Swept {
        tree,
        height: height.max(1e-6),
        params,
        paths: &paths,
        distance: &distance,
        angular: &angular,
    };
    let ordered = rings::rank(at, &mut rings::Scratch::new(at.longest()?)?)?;
    let rings = ordered.iter().map(|o| at.rings(o.0));
    Ok(Layout::new(rings, segments, true, CHUNK_VERTICES)?.extent(segments))
}

/// Builds only wood geometry. Invalid input or allocation failure returns no partial mesh.
pub fn build(tree: &Tree, height: f64, params: &SurfaceParams) -> Result<SurfaceMesh> {
    let sweep = Sweep {
        drawn: true,
        edges: false,
        distances: false,
    };
    let rings = rings(tree, height, params, sweep)?;
    let faces = faces(&rings, tree, height, params)?;
    Ok(rings.into_mesh(faces))
}

/// What the mesh step adds around the rings: each chunk's normals and
/// indices, and the run table the rings hold where the step dropped no
/// triangle.
#[derive(Debug, Default)]
pub(crate) struct Faces {
    pub(super) chunks: Vec<FaceChunk>,
    /// The run table, where dropped triangles moved a piece's span.
    pub(super) run_table: Option<Vec<WoodRun>>,
    pub(super) bounds: Option<Bounds>,
    pub(super) dropped: usize,
}

/// One chunk's normals (`encode::normal`) and indices.
#[derive(Debug, Default)]
pub(crate) struct FaceChunk {
    pub(super) normals: Vec<i16>,
    pub(super) indices: Vec<u16>,
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
    let mut spans = reserved(rings.runs.len())?;
    let (chunks, dropped) = shaded(rings, &resweep, &rings.layout.pieces, &mut spans)?;
    let mut faces = Faces {
        chunks,
        run_table: None,
        bounds: Some(rings.bounds()),
        dropped,
    };
    if faces.dropped > DROPPED_RINGS * 2 * rings.segments {
        return Err(Error::InvalidValue {
            field: "surface triangles collapsed in float32",
            value: faces.dropped.to_string(),
        });
    }
    let swept = |r: &WoodRun| [r.span.first_index, r.span.index_count];
    let moved = spans.iter().zip(&rings.runs).any(|(s, r)| *s != swept(r));
    faces.run_table = moved.then(|| {
        let runs = rings.runs.iter().zip(spans);
        let span = |r: &WoodRun, [first_index, index_count]: [u32; 2]| SurfaceRun {
            first_index,
            index_count,
            ..r.span
        };
        runs.map(|(r, s)| WoodRun {
            span: span(r, s),
            ..*r
        })
        .collect()
    });
    Ok(faces)
}

/// The faces of the chunks `pieces` fill, whole runs of them in order: each
/// chunk's buffers reserved at their exact size as its first piece is
/// shaded, each piece's span of its chunk's indices pushed to `spans`, and
/// the triangles dropped. A run's normals are summed in floats over the run
/// and then encoded.
pub(super) fn shaded(
    rings: &Rings,
    resweep: &Resweep,
    pieces: &[Piece],
    spans: &mut Vec<[u32; 2]>,
) -> Result<(Vec<FaceChunk>, usize)> {
    let Some(first) = pieces.first().map(|p| p.chunk) else {
        return Ok((Vec::new(), 0));
    };
    let segments = rings.segments;
    let indices = rings.layout.indices(segments, true)?;
    let count = pieces.last().map_or(0, |p| p.chunk + 1) - first;
    let mut chunks: Vec<FaceChunk> = reserved(count)?;
    let whole = pieces.iter().filter(|p| p.lead && p.tail);
    let longest = whole.map(|p| p.vertices(segments, true)).max();
    let mut normals = reserved(longest.unwrap_or(0) * 3)?;
    let mut dropped = 0;
    for run in pieces.chunk_by(|a, b| a.run == b.run) {
        for piece in run {
            if piece.chunk - first == chunks.len() {
                if rings.layout.chunks[piece.chunk] > CHUNK_VERTICES {
                    return Err(Error::ResourceLimit("surface chunk"));
                }
                chunks.push(FaceChunk {
                    normals: reserved(rings.layout.chunks[piece.chunk] * 2)?,
                    indices: reserved(indices[piece.chunk])?,
                });
            }
        }
        let [piece] = run else {
            let chunks = &mut chunks[run[0].chunk - first..];
            dropped += split(rings, resweep, run, chunks, spans)?;
            continue;
        };
        let chunk = &mut chunks[piece.chunk - first];
        let (base, count) = (piece.base, piece.rings);
        let vertices = count * segments + 2;
        let positions = &rings.chunks[piece.chunk].positions[base * 3..(base + vertices) * 3];
        normals.clear();
        normals.resize(vertices * 3, 0.0);
        let start = chunk.indices.len();
        let indices = &mut chunk.indices;
        let keep = |t: [u32; 3]| indices.extend(t.map(|i| (base + i as usize) as u16));
        dropped += shade(
            positions,
            count,
            segments,
            piece.run,
            resweep,
            &mut normals,
            keep,
        )?;
        encoded(&normals, &mut chunk.normals);
        spans.push(span(start, chunk.indices.len())?);
    }
    Ok((chunks, dropped))
}

/// Unit normals, three floats each, encoded onto the end of `out`.
fn encoded(normals: &[f32], out: &mut Vec<i16>) {
    for n in normals.as_chunks::<3>().0 {
        let [x, y, z] = n.map(f64::from);
        out.extend(encode::normal(Vec3::new(x, y, z)));
    }
}

/// A piece's span of its chunk's indices, from where it started to `end`.
fn span(start: usize, end: usize) -> Result<[u32; 2]> {
    let too_many = || Error::ResourceLimit("surface indices");
    let start = u32::try_from(start).map_err(|_| too_many())?;
    let end = u32::try_from(end).map_err(|_| too_many())?;
    Ok([start, end - start])
}

/// A run split across `chunks`, its pieces' chunks from the first: its
/// points gathered whole and shaded as one run, so the split moves no
/// normal and drops no other triangle; then each piece's normals copied
/// and each triangle handed, in draw order, to the piece that draws it: a
/// strip to the piece its pair belongs to, a cap to the last piece, which
/// repeats the first ring, so the caps draw after every strip.
fn split(
    rings: &Rings,
    resweep: &Resweep,
    run: &[Piece],
    chunks: &mut [FaceChunk],
    spans: &mut Vec<[u32; 2]>,
) -> Result<usize> {
    let segments = rings.segments;
    let (lead, tail) = (run[0], run[run.len() - 1]);
    let count = tail.first + tail.rings;
    let ring = count * segments;
    let point = |p: &Piece, v: usize| {
        let at = p.local(v, count, segments) * 3;
        &rings.chunks[p.chunk].positions[at..at + 3]
    };
    let mut positions = reserved((ring + 2) * 3)?;
    let mut next = 0;
    for piece in run {
        let end = (piece.first + piece.rings) * segments;
        positions.extend((next..end).flat_map(|v| point(piece, v)));
        next = end;
    }
    positions.extend(point(&lead, ring).iter().chain(point(&tail, ring + 1)));
    let mut normals = filled((ring + 2) * 3, 0.0)?;
    let mut starts = reserved(run.len())?;
    starts.extend(
        run.iter()
            .map(|p| chunks[p.chunk - lead.chunk].indices.len()),
    );
    let owner = |t: [u32; 3]| match t.iter().max().map(|&v| v as usize) {
        Some(v) if v >= ring => tail,
        _ => {
            let low = t.iter().min().map_or(0, |&v| v as usize) / segments;
            let mut pieces = run.iter();
            *pieces
                .find(|p| p.pairs().contains(&low))
                .expect("a pair has a piece")
        }
    };
    let dropped = shade(
        &positions,
        count,
        segments,
        lead.run,
        resweep,
        &mut normals,
        |t| {
            let piece = owner(t);
            let local = t.map(|v| piece.local(v as usize, count, segments) as u16);
            chunks[piece.chunk - lead.chunk].indices.extend(local);
        },
    )?;
    for (piece, &start) in run.iter().zip(&starts) {
        let chunk = &mut chunks[piece.chunk - lead.chunk];
        let held = piece.first * segments * 3..(piece.first + piece.rings) * segments * 3;
        encoded(&normals[held], &mut chunk.normals);
        if piece.repeats() {
            encoded(&normals[..segments * 3], &mut chunk.normals);
        }
        let bottom = piece.lead || piece.repeats();
        for (_, v) in [(bottom, ring), (piece.tail, ring + 1)]
            .into_iter()
            .filter(|c| c.0)
        {
            encoded(&normals[v * 3..v * 3 + 3], &mut chunk.normals);
        }
        spans.push(span(start, chunk.indices.len())?);
    }
    Ok(dropped)
}

/// A run's faces, its `count` rings and two caps from the start of
/// `positions`: each triangle with area handed to `keep` in draw order, as
/// run vertices, and the run's unit normals summed into `normals`, which
/// starts at zero. A vertex no triangle shades faces the way its ring does.
/// `run` is its place in the table. Returns the triangles dropped.
pub(super) fn shade(
    positions: &[f32],
    count: usize,
    segments: usize,
    run: usize,
    resweep: &Resweep,
    normals: &mut [f32],
    keep: impl FnMut([u32; 3]),
) -> Result<usize> {
    let vertices = count * segments + 2;
    let positions = &positions[..vertices * 3];
    let at = prepared::Run {
        base: 0,
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

/// What every ring of a sweep is drawn with: the section's rows, the
/// tree's height (the twist runs over it) and the points round a ring.
#[derive(Clone, Copy)]
pub(super) struct Ring<'a> {
    pub(super) params: &'a SurfaceParams,
    pub(super) height: f64,
    pub(super) angular: &'a [angular::Angular],
}

/// A run's vertices, ring by ring, then its two caps where `caps` asks.
pub(super) fn emit_run(
    samples: &[Sample],
    frame: &[(Vec3, Vec3)],
    ring: Ring,
    shape: Option<&crate::tree::Section>,
    caps: bool,
    mut emit: impl FnMut([f32; 3], [f32; 2]),
) -> Result<()> {
    let Ring {
        params,
        height,
        angular,
    } = ring;
    let mut vertex = |p: Vec3, coord: [f32; 2]| {
        let xyz = [p.x as f32, p.y as f32, p.z as f32];
        if !xyz.iter().all(|v| v.is_finite()) {
            return Err(Error::InvalidInput("surface float32 position overflow"));
        }
        emit(xyz, coord);
        Ok(())
    };
    for (i, s) in samples.iter().enumerate() {
        if let Some(shape) = shape {
            let ring = shape.ring(i, samples.len());
            for sample in angular {
                let p = shape.vertex(ring, sample.cos, sample.sin);
                vertex(p, [s.d as f32, sample.angle as f32])?;
            }
            continue;
        }
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

/// The box around every vertex of every chunk, widened from float32.
pub(super) fn bounds<'a>(chunks: impl Iterator<Item = &'a [f32]>) -> Bounds {
    let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for p in chunks.flat_map(|c| c.as_chunks::<3>().0) {
        min.x = min.x.min(p[0] as f64);
        min.y = min.y.min(p[1] as f64);
        min.z = min.z.min(p[2] as f64);
        max.x = max.x.max(p[0] as f64);
        max.y = max.y.max(p[1] as f64);
        max.z = max.z.max(p[2] as f64);
    }
    Bounds { min, max }
}
