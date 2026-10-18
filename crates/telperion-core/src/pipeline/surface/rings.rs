//! The ring step: the rings swept over every solved path, as the float32
//! points the wood draws and seated leaves read in place. It is a Plan
//! artifact; the mesh step and the leaves both read it, and the wood takes
//! its positions, coords and run table once the mesh step is done.
use super::curve::{Curve, Traced};
use super::*;
use crate::tree::Section;
use std::sync::OnceLock;

/// What a sweep is for.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Sweep {
    /// A drawn wood's layout: the runs widest first, each capped, with every
    /// vertex's coords and the run table. Bare rings are only the ring
    /// points, in path order, all that seated leaves read without a wood.
    pub(crate) drawn: bool,
    /// Each node's contact rings, which only seated leaves read.
    pub(crate) edges: bool,
    /// Each ring's distance along in place of a drawn wood's bark coords:
    /// all the prepared surface reads of them.
    pub(crate) distances: bool,
}

pub(crate) struct Rings {
    /// Each chunk's points: three floats a vertex, every ring of a piece,
    /// then its caps where drawn; and two floats a vertex of coords, where
    /// drawn.
    pub(super) chunks: Vec<RingChunk>,
    /// Each node's lower and upper ring, then the first and last ring of the
    /// piece that answers for it, as ring addresses (`chunks::address`),
    /// where the sweep kept them.
    pub(super) edges: Vec<Option<[usize; 4]>>,
    /// The runs and their pieces, in the sweep's order.
    pub(super) layout: Layout,
    /// The run table as swept, one entry a piece, where drawn: widest run
    /// first, ties in path order, each span before the mesh step drops any
    /// triangle.
    pub(super) runs: Vec<WoodRun>,
    pub(super) segments: usize,
    /// The longest distance along the wood, which coords are quantised over.
    pub(super) along: f64,
    /// The workers the sweep ran on, where it ran in parallel.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub(super) workers: Option<usize>,
}

/// One chunk's points, and its coords (`encode::coord`) or its rings'
/// distances along.
#[derive(Debug, Default, Clone)]
pub(crate) struct RingChunk {
    pub(super) positions: Vec<f32>,
    pub(super) coords: Vec<u16>,
    pub(super) distances: Vec<f32>,
}

/// What a sweep keeps beside each point.
#[derive(Debug, Clone, Copy)]
pub(super) enum Coords {
    /// Nothing: bare rings.
    None,
    /// Bark coords, the distance along quantised over the wood's longest.
    Bark(f64),
    /// One distance along a ring, as a float.
    Distances,
}

impl Rings {
    /// The wood: these rings' chunks and run table, moved, around the faces
    /// the mesh step built on them.
    pub(crate) fn into_mesh(self, faces: Faces) -> SurfaceMesh {
        let run_table = faces.run_table.unwrap_or(self.runs);
        let chunks = self.chunks.into_iter().zip(faces.chunks);
        SurfaceMesh {
            chunks: chunks
                .map(|(rings, faces)| SurfaceChunk {
                    positions: rings.positions,
                    normals: faces.normals,
                    coords: rings.coords,
                    indices: faces.indices,
                })
                .collect(),
            bounds: faces.bounds,
            along: self.along,
            runs: run_table.len(),
            run_table,
            dropped: faces.dropped,
        }
    }

    /// The box around every point.
    pub(super) fn bounds(&self) -> Bounds {
        bounds(self.chunks.iter().map(|c| &c.positions[..]))
    }
}

/// Sweeps the rings of every solved path: a drawn wood in chunks of at most
/// `CHUNK_VERTICES` vertices, bare rings in one.
pub(crate) fn rings(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    sweep: Sweep,
) -> Result<Rings> {
    let limit = if sweep.drawn {
        CHUNK_VERTICES
    } else {
        usize::MAX
    };
    swept(tree, height, params, sweep, None, true, limit)
}

/// A drawn wood's rings read from `traced`, the curve of the same tree:
/// its run order, and each run's samples and frames to the bit, never
/// swept again.
pub(crate) fn drawn_from(
    traced: &Traced,
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    edges: bool,
) -> Result<Rings> {
    let sweep = Sweep {
        drawn: true,
        edges,
        distances: false,
    };
    swept(tree, height, params, sweep, Some(traced), true, CHUNK_VERTICES)
}

pub(super) fn rings_mode(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    sweep: Sweep,
    parallel_allowed: bool,
    limit: usize,
) -> Result<Rings> {
    swept(tree, height, params, sweep, None, parallel_allowed, limit)
}

/// The rings, each run swept here or, for a drawn wood, read from the
/// curve `traced` where one is given.
fn swept(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    sweep: Sweep,
    traced: Option<&Traced>,
    parallel_allowed: bool,
    limit: usize,
) -> Result<Rings> {
    tree.validate()?;
    params.validate()?;
    crate::pipeline::surface::height(height)?;
    let segments = segments(params);
    let edges = if sweep.edges {
        filled(tree.nodes.len(), None)?
    } else {
        Vec::new()
    };
    let owned;
    let paths = match traced {
        Some(traced) => &traced.paths,
        None => {
            owned = paths(tree)?;
            &owned
        }
    };
    let mut out = Rings {
        chunks: Vec::new(),
        edges,
        layout: Layout::default(),
        runs: Vec::new(),
        segments,
        along: 0.0,
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        workers: None,
    };
    if paths.runs.is_empty() {
        return Ok(out);
    }
    tree.validate_solved()?;
    let height = height.max(1e-6);
    let angular = angular::samples(segments, params)?;
    let distance = distances(tree)?;
    out.along = distance.iter().copied().fold(0.0, f64::max);
    let at = Swept {
        tree,
        height,
        params,
        paths,
        distance: &distance,
        angular: &angular,
    };
    let longest = at.longest()?;
    let mut scratch = Scratch::new(longest)?;
    let curve = traced.filter(|_| sweep.drawn).map(|t| &t.curve);
    let ordered = if let Some(traced) = traced.filter(|_| sweep.drawn) {
        let mut order = reserved(traced.order.len())?;
        let runs = traced.order.iter().zip(&traced.curve.runs);
        order.extend(runs.map(|(&path_id, run)| (path_id, run.largest_radius)));
        order
    } else if sweep.drawn {
        rank(at, &mut scratch)?
    } else {
        let mut order = reserved(paths.runs.len())?;
        order.extend((0..paths.runs.len()).map(|path_id| (path_id, 0.0)));
        order
    };
    let rings = ordered.iter().map(|o| at.rings(o.0));
    out.layout = Layout::new(rings, segments, sweep.drawn, limit)?;
    let coords = match (sweep.drawn, sweep.distances) {
        (false, _) => Coords::None,
        (true, false) => Coords::Bark(out.along),
        (true, true) => Coords::Distances,
    };
    if sweep.drawn {
        super::budget::check(&out.layout, segments, coords)?;
    }
    if sweep.edges {
        at.record(&mut out.edges, &ordered, &out.layout);
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    if parallel_allowed && sweep.drawn {
        let admitted = parallel::admitted(at, &ordered, longest, &out.layout);
        if let Some(workers) = admitted {
            drop(scratch);
            return match parallel::rings(at, curve, &ordered, &mut out, longest, workers) {
                Ok(()) => table(ordered, out),
                Err(_) => swept(tree, height, params, sweep, traced, false, limit),
            };
        }
    }
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    let _ = parallel_allowed;
    let pieces = &out.layout.pieces;
    out.chunks = written(at, curve, &ordered, &out.layout, pieces, coords, &mut scratch)?;
    if !sweep.drawn {
        return Ok(out);
    }
    table(ordered, out)
}

/// The chunks `pieces` fill, whole runs of them in order, each chunk
/// reserved at its exact size as its first piece is written. Each run is
/// read from `curve` where one is given, else swept.
pub(super) fn written(
    at: Swept,
    curve: Option<&Curve>,
    ordered: &[(usize, f64)],
    layout: &Layout,
    pieces: &[Piece],
    coords: Coords,
    scratch: &mut Scratch,
) -> Result<Vec<RingChunk>> {
    let drawn = !matches!(coords, Coords::None);
    let Some(first) = pieces.first().map(|p| p.chunk) else {
        return Ok(Vec::new());
    };
    let count = pieces.last().map_or(0, |p| p.chunk + 1) - first;
    let mut chunks: Vec<RingChunk> = reserved(count)?;
    for run in pieces.chunk_by(|a, b| a.run == b.run) {
        for piece in run {
            if piece.chunk - first == chunks.len() {
                let vertices = layout.chunks[piece.chunk];
                let per = |n: usize| match coords {
                    Coords::Bark(_) => n * 2,
                    _ => 0,
                };
                let rings = |n: usize| match coords {
                    Coords::Distances => n / at.segments(),
                    _ => 0,
                };
                chunks.push(RingChunk {
                    positions: reserved(vertices * 3)?,
                    coords: reserved(per(vertices))?,
                    distances: reserved(rings(vertices))?,
                });
            }
        }
        let path_id = ordered[run[0].run].0;
        let (samples, frame) = match curve {
            Some(curve) => scratch.read(curve, run[0].run),
            None => scratch.sweep(at, path_id),
        };
        let ring = samples.len() * at.segments();
        let shape = at.section(path_id);
        let mut v = 0;
        emit_run(samples, frame, at.ring(), shape, drawn, |xyz, coord| {
            for piece in run {
                // Run vertex `v`: a ring the piece holds, or the cap it holds;
                // a split run's last piece takes its first ring again from
                // the first piece before its bottom cap.
                if matches!(coords, Coords::Bark(_)) && v == ring && piece.repeats() {
                    let (lead, segments) = (run[0], at.segments());
                    let (held, rest) = chunks.split_at_mut(piece.chunk - first);
                    let (from, to) = (&held[lead.chunk - first], &mut rest[0]);
                    let at = |n: usize| lead.base * n..(lead.base + segments) * n;
                    to.positions.extend_from_slice(&from.positions[at(3)]);
                    to.coords.extend_from_slice(&from.coords[at(2)]);
                }
                let holds = run.len() == 1
                    || if v < ring {
                        (piece.first..piece.first + piece.rings).contains(&(v / at.segments()))
                    } else {
                        (v == ring && (piece.lead || piece.repeats())) || (v > ring && piece.tail)
                    };
                if holds {
                    let chunk = &mut chunks[piece.chunk - first];
                    chunk.positions.extend(xyz);
                    match coords {
                        Coords::Bark(along) => {
                            let [d, angle] = coord.map(f64::from);
                            chunk.coords.extend(encode::coord(d, angle, along));
                        }
                        Coords::Distances if v < ring && v % at.segments() == 0 => {
                            chunk.distances.push(coord[0]);
                        }
                        _ => {}
                    }
                }
            }
            v += 1;
        })?;
    }
    Ok(chunks)
}

/// One run's samples and frames, reused run after run.
pub(super) struct Scratch {
    samples: Vec<Sample>,
    frame: Vec<(Vec3, Vec3)>,
    /// The frames' own scratch: each segment's direction.
    directions: Vec<Vec3>,
}
impl Scratch {
    pub(super) fn new(longest: usize) -> Result<Self> {
        Ok(Self {
            samples: reserved(longest)?,
            frame: reserved(longest)?,
            directions: reserved(longest)?,
        })
    }

    /// Samples path `path_id`'s run and frames it.
    pub(super) fn sweep(&mut self, at: Swept, path_id: usize) -> (&[Sample], &[(Vec3, Vec3)]) {
        at.sample(path_id, &mut self.samples);
        let seed = seed(at.tree, at.paths, &at.paths.runs[path_id]);
        frames(&self.samples, &mut self.directions, &mut self.frame, seed);
        if let Some(shape) = at.section(path_id) {
            section::square(shape, &mut self.frame);
        }
        (&self.samples, &self.frame)
    }

    /// Run `run` of `curve`: the samples and frames the sweep traced it
    /// from, as its points hold them.
    pub(super) fn read(&mut self, curve: &Curve, run: usize) -> (&[Sample], &[(Vec3, Vec3)]) {
        self.samples.clear();
        self.frame.clear();
        for p in curve.run_points(&curve.runs[run]) {
            self.samples.push(Sample {
                p: p.centre,
                r: p.radius,
                d: p.along,
            });
            self.frame.push((p.normal, p.binormal));
        }
        (&self.samples, &self.frame)
    }
}

/// What a sweep reads: the tree and its paths, sampled around the ring.
#[derive(Clone, Copy)]
pub(super) struct Swept<'a> {
    pub(super) tree: &'a Tree,
    pub(super) height: f64,
    pub(super) params: &'a SurfaceParams,
    pub(super) paths: &'a paths::Paths,
    pub(super) distance: &'a [f64],
    pub(super) angular: &'a [angular::Angular],
}
impl Swept<'_> {
    /// Whether the flare buries a ring at the foot of every trunk, as
    /// `sample_path` sinks it.
    pub(super) fn buried(&self) -> bool {
        self.params.flare_depth * self.height > 0.0
    }

    /// The most samples a run takes: one a node of the longest path, and one
    /// more for a buried foot.
    pub(super) fn longest(&self) -> Result<usize> {
        let runs = self.paths.runs.iter();
        let longest = runs.map(|p| p.end - p.start).max().unwrap_or(0);
        longest
            .checked_add(1)
            .ok_or(Error::ResourceLimit("surface samples"))
    }

    /// What every ring of this sweep is drawn with.
    pub(super) fn ring(&self) -> Ring<'_> {
        Ring {
            params: self.params,
            height: self.height,
            angular: self.angular,
        }
    }

    /// The rings a path's run is swept in: one a node, and a buried foot.
    pub(super) fn rings(&self, path_id: usize) -> usize {
        let path = &self.paths.runs[path_id];
        path.end - path.start + usize::from(path.trunk && self.buried())
    }

    pub(super) fn sample(&self, path_id: usize, samples: &mut Vec<Sample>) {
        let (tree, height, params) = (self.tree, self.height, self.params);
        let path = &self.paths.runs[path_id];
        sample_path(
            tree,
            height,
            params,
            self.paths,
            path,
            self.distance,
            samples,
        );
    }

    /// The cell a path's run is drawn as, if the tree shapes it.
    pub(super) fn section(&self, path_id: usize) -> Option<&Section> {
        section::of(self.tree, self.paths, &self.paths.runs[path_id])
    }

    /// Points round a ring.
    pub(super) fn segments(&self) -> usize {
        self.angular.len()
    }

    /// Records every node's contact rings over the layout of `ordered`.
    pub(super) fn record(
        &self,
        edges: &mut [Option<[usize; 4]>],
        ordered: &[(usize, f64)],
        layout: &Layout,
    ) {
        for pieces in layout.runs() {
            let path = &self.paths.runs[ordered[pieces[0].run].0];
            let nodes = &self.paths.nodes[path.start..path.end];
            let offset = usize::from(path.trunk && self.buried());
            chunks::record(edges, nodes, pieces, self.segments(), offset, layout.shift);
        }
    }
}

/// Each node's distance along the wood from the root.
pub(super) fn distances(tree: &Tree) -> Result<Vec<f64>> {
    let nodes = &tree.nodes;
    let mut distance = filled(nodes.len(), 0.0)?;
    for i in 1..nodes.len() {
        let p = nodes[i].parent.unwrap() as usize;
        distance[i] = distance[p] + nodes[p].position.distance(nodes[i].position);
        if !distance[i].is_finite() {
            return Err(Error::InvalidInput("surface path length overflow"));
        }
    }
    Ok(distance)
}

/// Every path with its run's largest sample radius, widest first. Ties keep
/// path order, so the order is deterministic.
pub(super) fn rank(at: Swept, scratch: &mut Scratch) -> Result<Vec<(usize, f64)>> {
    let mut ordered = reserved(at.paths.runs.len())?;
    for path_id in 0..at.paths.runs.len() {
        at.sample(path_id, &mut scratch.samples);
        let radius = scratch.samples.iter().map(|s| s.r).fold(0.0, f64::max);
        ordered.push((path_id, radius));
    }
    ordered.sort_by(|a, b| b.1.total_cmp(&a.1));
    Ok(ordered)
}

/// The ranked runs as the run table, a piece an entry. It is allocated
/// after the rings, so the ranking's space joins the sweep's freed scratch
/// below them.
pub(super) fn table(ordered: Vec<(usize, f64)>, mut out: Rings) -> Result<Rings> {
    let too_many = || Error::ResourceLimit("surface indices");
    let mut runs = reserved(out.layout.pieces.len())?;
    let mut first_index = 0u32;
    let mut chunk = 0;
    for piece in &out.layout.pieces {
        if piece.chunk != chunk {
            (chunk, first_index) = (piece.chunk, 0);
        }
        let count = piece.indices(out.segments, true);
        let index_count = u32::try_from(count).map_err(|_| too_many())?;
        runs.push(WoodRun {
            chunk: chunk as u32,
            span: SurfaceRun {
                first_index,
                index_count,
                largest_radius: ordered[piece.run].1,
            },
        });
        first_index = first_index.checked_add(index_count).ok_or_else(too_many)?;
    }
    out.runs = runs;
    Ok(out)
}

/// The frames of one run of the table, swept again for the rare vertex no
/// triangle shades: the paths, distances and ranking once a build, then
/// only that run.
pub(super) struct Resweep<'t> {
    tree: &'t Tree,
    height: f64,
    params: &'t SurfaceParams,
    ranked: OnceLock<Result<Ranked>>,
}
struct Ranked {
    paths: paths::Paths,
    distance: Vec<f64>,
    angular: Vec<angular::Angular>,
    ordered: Vec<(usize, f64)>,
}
impl<'t> Resweep<'t> {
    pub(super) fn new(tree: &'t Tree, height: f64, params: &'t SurfaceParams) -> Self {
        Self {
            tree,
            height: height.max(1e-6),
            params,
            ranked: OnceLock::new(),
        }
    }

    fn at<'a>(&'a self, r: &'a Ranked) -> Swept<'a> {
        Swept {
            tree: self.tree,
            height: self.height,
            params: self.params,
            paths: &r.paths,
            distance: &r.distance,
            angular: &r.angular,
        }
    }

    fn rank(&self) -> Result<Ranked> {
        let mut r = Ranked {
            paths: paths(self.tree)?,
            distance: distances(self.tree)?,
            angular: angular::samples(segments(self.params), self.params)?,
            ordered: Vec::new(),
        };
        let at = self.at(&r);
        let ordered = rank(at, &mut Scratch::new(at.longest()?)?)?;
        r.ordered = ordered;
        Ok(r)
    }

    /// The frames of run `run` of the table.
    pub(super) fn frames(&self, run: usize) -> Result<Vec<(Vec3, Vec3)>> {
        let ranked = self.ranked.get_or_init(|| self.rank());
        let ranked = ranked.as_ref().map_err(Clone::clone)?;
        let at = self.at(ranked);
        let mut scratch = Scratch::new(at.longest()?)?;
        scratch.sweep(at, ranked.ordered[run].0);
        Ok(scratch.frame)
    }
}
