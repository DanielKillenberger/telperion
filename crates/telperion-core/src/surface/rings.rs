//! The ring step: the rings swept over every solved path, as the float32
//! points the wood draws and seated leaves read in place. It is a Plan
//! artifact; the mesh step and the leaves both read it, and the wood takes
//! its positions, coords and run table once the mesh step is done.
use super::*;
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
}

pub(crate) struct Rings {
    /// Three floats a vertex: every ring of a run, then its two caps where
    /// drawn.
    pub(super) positions: Vec<f32>,
    /// Two floats a vertex, where drawn.
    pub(super) coords: Vec<f32>,
    /// Each node's lower and upper ring, then its run's first and last, as
    /// vertex offsets into `positions`, where the sweep kept them.
    pub(super) edges: Vec<Option<[usize; 4]>>,
    /// The run table as swept, where drawn: widest run first, ties in path
    /// order, each run's span before the mesh step drops any triangle.
    pub(super) runs: Vec<SurfaceRun>,
    pub(super) segments: usize,
    /// The workers the sweep ran on, where it ran in parallel.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub(super) workers: Option<usize>,
}

impl Rings {
    /// The rings in a run of the table.
    pub(super) fn count(&self, run: &SurfaceRun) -> usize {
        run.index_count as usize / (self.segments * 6)
    }

    /// The vertices of a run of the table: its rings and two caps.
    pub(super) fn vertices(&self, run: &SurfaceRun) -> usize {
        self.count(run) * self.segments + 2
    }

    /// The wood: these rings' positions, coords and run table, moved, around
    /// the faces the mesh step built on them.
    pub(crate) fn into_mesh(self, faces: Faces) -> SurfaceMesh {
        let run_table = faces.run_table.unwrap_or(self.runs);
        SurfaceMesh {
            positions: self.positions,
            indices: faces.indices,
            normals: faces.normals,
            coords: self.coords,
            bounds: faces.bounds,
            runs: run_table.len(),
            run_table,
            dropped: faces.dropped,
        }
    }
}

/// Sweeps the rings of every solved path.
pub(crate) fn rings(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    sweep: Sweep,
) -> Result<Rings> {
    rings_mode(tree, height, params, sweep, true)
}

pub(super) fn rings_mode(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    sweep: Sweep,
    parallel_allowed: bool,
) -> Result<Rings> {
    tree.validate()?;
    params.validate()?;
    crate::surface::height(height)?;
    let segments = segments(params);
    let edges = if sweep.edges {
        filled(tree.nodes.len(), None)?
    } else {
        Vec::new()
    };
    let paths = paths(&tree.nodes)?;
    let mut out = Rings {
        positions: Vec::new(),
        coords: Vec::new(),
        edges,
        runs: Vec::new(),
        segments,
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
    let at = Swept {
        tree,
        height,
        params,
        paths: &paths,
        distance: &distance,
        angular: &angular,
    };
    let extent = extent_of(&paths, segments, at.buried())?;
    let longest = at.longest()?;
    let mut scratch = Scratch::new(longest)?;
    if !sweep.drawn {
        out.positions = reserved(extent.positions - paths.runs.len() * 6)?;
        return in_turn(at, out, sweep, scratch, 0..paths.runs.len());
    }
    let ordered = rank(at, &mut scratch)?;
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    if parallel_allowed {
        let vertices = extent.positions / 3;
        let admitted = parallel::admitted(at, &ordered, longest, vertices, extent.indices);
        if let Some(workers) = admitted {
            drop(scratch);
            return match parallel::rings(at, &ordered, &mut out, sweep, longest, workers) {
                Ok(()) => table(at, ordered, out),
                Err(_) => rings_mode(tree, height, params, sweep, false),
            };
        }
    }
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    let _ = parallel_allowed;
    out.positions = reserved(extent.positions)?;
    out.coords = reserved(extent.coords)?;
    let out = in_turn(at, out, sweep, scratch, ordered.iter().map(|o| o.0))?;
    table(at, ordered, out)
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
        frames(&self.samples, &mut self.directions, &mut self.frame);
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

    /// Records a path's contact rings, its run starting at vertex `base`.
    pub(super) fn record(&self, edges: &mut [Option<[usize; 4]>], path_id: usize, base: usize) {
        let path = &self.paths.runs[path_id];
        let nodes = &self.paths.nodes[path.start..path.end];
        let offset = usize::from(path.trunk && self.buried());
        let segments = self.angular.len();
        record_edges(edges, nodes, base, self.rings(path_id), segments, offset);
    }
}

/// The ring step in turn: each path's rings in the given order, after the
/// ones before it.
fn in_turn(
    at: Swept,
    mut out: Rings,
    sweep: Sweep,
    mut scratch: Scratch,
    order: impl Iterator<Item = usize>,
) -> Result<Rings> {
    for path_id in order {
        if sweep.edges {
            at.record(&mut out.edges, path_id, out.positions.len() / 3);
        }
        let (samples, frame) = scratch.sweep(at, path_id);
        let (positions, coords) = (&mut out.positions, &mut out.coords);
        emit_run(samples, frame, at, sweep.drawn, |xyz, coord| {
            positions.extend(xyz);
            if sweep.drawn {
                coords.extend(coord);
            }
        })?;
    }
    Ok(out)
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

/// The ranked runs as the run table. It is allocated after the rings, so
/// the ranking's space joins the sweep's freed scratch below them.
pub(super) fn table(at: Swept, ordered: Vec<(usize, f64)>, mut out: Rings) -> Result<Rings> {
    let segments = at.angular.len();
    let too_many = || Error::ResourceLimit("surface indices");
    let mut runs = reserved(ordered.len())?;
    let mut first_index = 0u32;
    for (path_id, largest_radius) in ordered {
        let count = at.rings(path_id) * segments * 6;
        let index_count = u32::try_from(count).map_err(|_| too_many())?;
        runs.push(SurfaceRun {
            first_index,
            index_count,
            largest_radius,
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
            paths: paths(&self.tree.nodes)?,
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
