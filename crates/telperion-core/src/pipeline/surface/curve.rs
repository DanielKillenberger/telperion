//! The curve the wood is surfaced from (fn-208, host decision 1): every run
//! of the sweep as the points its rings stand on, before any vertex. A point
//! is a ring's centre, radius, distance along the wood and frame, exactly as
//! the sweep holds them; a run names its first point, its count and the cell
//! a shaped run is drawn as; clusters of up to `CLUSTER` points carry the
//! bounds a view culls by and the error each coarser ring level would make.
//! Every tree's wood is this curve, whatever grew it: the sweep's own rings
//! are drawn from it, so there is no second path to the surface.
use super::{angular, build::Ring, rings::Scratch, *};
use crate::tree::Section;

mod pack;
mod tessellate;
pub use pack::{PackedPoint, POINT_WORDS};
pub use tessellate::{Budget, Tessellation, Viewer, SCALES};

/// The most points a cluster spans, its two ends included; neighbouring
/// clusters of a run share their end point.
pub const CLUSTER: usize = 32;
/// The ring levels a cluster keeps an error for: level `k` keeps every
/// `2^k`-th point of the cluster and its last.
pub const LEVELS: usize = 6;

/// Where one ring stands, in the sweep's own float64.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurvePoint {
    pub centre: Vec3,
    /// The ring's radius: the girth with the fork's easing and the flare,
    /// or a shaped run's extent.
    pub radius: f64,
    /// Metres along the wood from the root: the bark's first coordinate.
    pub along: f64,
    /// The parallel-transported frame; `normal × binormal` is the tangent.
    pub normal: Vec3,
    pub binormal: Vec3,
}

/// One run of the wood: a chain of points, widest run first.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurveRun {
    pub first: u32,
    pub count: u32,
    /// A trunk run stands on the root rather than sunk into a socket.
    pub trunk: bool,
    /// The cell this run is drawn as, an index into `Curve::sections`.
    pub section: Option<u32>,
    pub largest_radius: f64,
}

/// Up to `CLUSTER` consecutive points of one run, with what a view reads
/// before it draws them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurveCluster {
    pub run: u32,
    /// The first point, as an index into `Curve::points`, and the count.
    pub first: u32,
    pub count: u32,
    /// A sphere round every ring of the cluster, lobes included.
    pub centre: Vec3,
    pub reach: f64,
    pub largest_radius: f64,
    pub smallest_radius: f64,
    /// For each ring level, the most a ring it leaves out stands off the
    /// line between the rings it keeps (centre and radius together), in
    /// metres; never less than the level below.
    pub errors: [f32; LEVELS],
}

/// The wood of one tree as curves.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Curve {
    pub points: Vec<CurvePoint>,
    pub runs: Vec<CurveRun>,
    pub clusters: Vec<CurveCluster>,
    pub sections: Vec<Section>,
    /// The section's rows every ring reads: its lobes, their depth and the
    /// twist over the tree's height.
    pub lobes: u32,
    pub lobe_depth: f64,
    pub twist_rate: f64,
    pub height: f64,
}

/// The curve of a solved tree: the sweep's samples and frames, run by run in
/// the order the wood draws them.
pub(crate) fn curve(tree: &Tree, height: f64, params: &SurfaceParams) -> Result<Curve> {
    tree.validate()?;
    params.validate()?;
    crate::pipeline::surface::height(height)?;
    let paths = paths(&tree.nodes)?;
    let height = height.max(1e-6);
    let mut out = Curve {
        lobes: params.lobes,
        lobe_depth: params.lobe_depth,
        twist_rate: params.twist_rate,
        height,
        ..Curve::default()
    };
    if paths.runs.is_empty() {
        return Ok(out);
    }
    tree.validate_solved()?;
    let angular = angular::samples(segments(params), params)?;
    let distance = rings::distances(tree)?;
    let at = rings::Swept {
        tree,
        height,
        params,
        paths: &paths,
        distance: &distance,
        angular: &angular,
    };
    let mut scratch = Scratch::new(at.longest()?)?;
    let ordered = rings::rank(at, &mut scratch)?;
    out.points = reserved(paths.nodes.len() + paths.runs.len())?;
    out.runs = reserved(ordered.len())?;
    for (path_id, largest_radius) in ordered {
        let section = at.section(path_id).map(|shape| {
            out.sections.push(*shape);
            out.sections.len() as u32 - 1
        });
        let (samples, frame) = scratch.sweep(at, path_id);
        let first =
            u32::try_from(out.points.len()).map_err(|_| Error::ResourceLimit("curve points"))?;
        for (s, &(normal, binormal)) in samples.iter().zip(frame) {
            out.points.push(CurvePoint {
                centre: s.p,
                radius: s.r,
                along: s.d,
                normal,
                binormal,
            });
        }
        let run = CurveRun {
            first,
            count: samples.len() as u32,
            trunk: paths.runs[path_id].trunk,
            section,
            largest_radius,
        };
        let mut made = std::mem::take(&mut out.clusters);
        clusters(&out, out.runs.len() as u32, &run, &mut made);
        out.clusters = made;
        out.runs.push(run);
    }
    Ok(out)
}

/// A run's clusters: consecutive points, neighbours sharing an end.
fn clusters(curve: &Curve, index: u32, run: &CurveRun, out: &mut Vec<CurveCluster>) {
    let points = &curve.points[run.first as usize..(run.first + run.count) as usize];
    let lobe = 1.0 + curve.lobe_depth.abs() * f64::from(u8::from(curve.lobes > 0));
    let mut start = 0;
    loop {
        let end = (start + CLUSTER - 1).min(points.len() - 1);
        let span = &points[start..=end];
        let (mut lo, mut hi) = (span[0].centre, span[0].centre);
        for p in span {
            lo = Vec3::new(
                lo.x.min(p.centre.x),
                lo.y.min(p.centre.y),
                lo.z.min(p.centre.z),
            );
            hi = Vec3::new(
                hi.x.max(p.centre.x),
                hi.y.max(p.centre.y),
                hi.z.max(p.centre.z),
            );
        }
        let centre = (lo + hi) * 0.5;
        let largest = span.iter().map(|p| p.radius).fold(0.0, f64::max);
        let smallest = span.iter().map(|p| p.radius).fold(f64::INFINITY, f64::min);
        let reach = span
            .iter()
            .map(|p| p.centre.distance(centre) + p.radius * lobe)
            .fold(0.0, f64::max);
        out.push(CurveCluster {
            run: index,
            first: run.first + start as u32,
            count: span.len() as u32,
            centre,
            reach,
            largest_radius: largest,
            smallest_radius: smallest,
            errors: ladder(span),
        });
        if end + 1 >= points.len() {
            break;
        }
        start = end;
    }
}

/// The error of each ring level of a cluster: level `k` keeps every
/// `2^k`-th point and the last, and a point it leaves out is measured from
/// the line between its kept neighbours at its share of the distance along,
/// in centre and radius together.
fn ladder(span: &[CurvePoint]) -> [f32; LEVELS] {
    let mut errors = [0.0f32; LEVELS];
    let last = span.len() - 1;
    for k in 1..LEVELS {
        let step = 1usize << k;
        let mut worst = 0.0f64;
        let mut a = 0;
        while a < last {
            let b = (a + step).min(last);
            let (pa, pb) = (&span[a], &span[b]);
            let length = (pb.along - pa.along).max(1e-12);
            for p in &span[a + 1..b] {
                let t = ((p.along - pa.along) / length).clamp(0.0, 1.0);
                let line = pa.centre + (pb.centre - pa.centre) * t;
                let radius = pa.radius + (pb.radius - pa.radius) * t;
                worst = worst.max(p.centre.distance(line) + (p.radius - radius).abs());
            }
            a = b;
        }
        errors[k] = (worst as f32).max(errors[k - 1]);
    }
    errors
}

impl Curve {
    /// The section's rows a ring reads, with every other row at its default.
    pub fn surface_params(&self) -> SurfaceParams {
        SurfaceParams {
            lobes: self.lobes,
            lobe_depth: self.lobe_depth,
            twist_rate: self.twist_rate,
            ..SurfaceParams::default()
        }
    }

    /// The run's points.
    pub fn run_points(&self, run: &CurveRun) -> &[CurvePoint] {
        &self.points[run.first as usize..(run.first + run.count) as usize]
    }

    /// The sweep's rings drawn from the curve alone at `segments` sides:
    /// every run's ring vertices and its two caps, with their coords, in the
    /// order and to the bit the sweep draws them. What proves the curve
    /// carries all the wood is that these are the sweep's own.
    pub fn rings(&self, segments: usize) -> Result<(Vec<f32>, Vec<f32>)> {
        let params = self.surface_params();
        let angular = angular::samples(segments, &params)?;
        let ring = Ring {
            params: &params,
            height: self.height,
            angular: &angular,
        };
        let (mut positions, mut coords) = (Vec::new(), Vec::new());
        let mut samples = Vec::new();
        let mut frame = Vec::new();
        for run in &self.runs {
            samples.clear();
            frame.clear();
            for p in self.run_points(run) {
                samples.push(Sample {
                    p: p.centre,
                    r: p.radius,
                    d: p.along,
                });
                frame.push((p.normal, p.binormal));
            }
            let shape = run.section.map(|s| &self.sections[s as usize]);
            emit_run(&samples, &frame, ring, shape, true, |xyz, coord| {
                positions.extend(xyz);
                coords.extend(coord);
            })?;
        }
        Ok((positions, coords))
    }
}

#[cfg(test)]
mod tests;
