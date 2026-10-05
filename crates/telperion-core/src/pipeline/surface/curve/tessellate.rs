//! The CPU reference tessellation of a curve (fn-208, host decisions 2, 3
//! and 5): the wood surfaced for one view at one error in pixels, the same
//! function the GPU passes run. Camera-dependent by definition; it reads
//! only the curve.
use super::{Curve, CurvePoint};
use crate::{
    math::{Transcendental, Vec3},
    Error, Result,
};

/// Where the wood is seen from: the eye, the unit direction it looks along,
/// the pixels a metre spans at one metre's depth (half the viewport's height
/// over the tangent of half the field of view), and the near plane.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewer {
    pub eye: Vec3,
    pub forward: Vec3,
    pub pixels_per_metre: f64,
    pub near: f64,
}

impl Viewer {
    /// Pixels a metre spans at `point`'s depth.
    pub fn pixels_at(&self, point: Vec3) -> f64 {
        self.pixels_per_metre / (point - self.eye).dot(self.forward).max(self.near)
    }
}

/// The wood's triangles for one view, in the renderer's own layout: three
/// floats a position and a normal, the bark's (along, angle) and the radius a
/// vertex, and the triangles' indices. `scale` is the error the budget let
/// it be drawn at, in multiples of the error asked for.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tessellation {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub coords: Vec<f32>,
    pub radii: Vec<f32>,
    pub indices: Vec<u32>,
    pub rings: usize,
    pub scale: f64,
}

/// One ring the surface passes through.
#[derive(Debug, Clone, Copy)]
struct Ring {
    point: CurvePoint,
    sides: u32,
}

impl Curve {
    /// The wood surfaced for `viewer` within `error` pixels. With a budget of
    /// triangles, a view that would need more is drawn at twice, four or
    /// eight times the error, the scale it took reported; past that it is
    /// refused by name, never truncated.
    pub fn tessellate(
        &self,
        viewer: &Viewer,
        error: f64,
        budget: Option<usize>,
    ) -> Result<Tessellation> {
        if !(error > 0.0) || !(viewer.pixels_per_metre > 0.0) || !(viewer.near > 0.0) {
            return Err(Error::InvalidInput("tessellation view or error"));
        }
        for scale in [1.0, 2.0, 4.0, 8.0] {
            let plans: Vec<Vec<Ring>> = (0..self.runs.len())
                .map(|run| self.plan(run, viewer, error * scale))
                .collect();
            let triangles: usize = plans.iter().map(|rings| triangles(rings)).sum();
            if budget.is_none_or(|b| triangles <= b) {
                let mut out = Tessellation {
                    scale,
                    ..Tessellation::default()
                };
                for rings in &plans {
                    self.emit(rings, &mut out);
                }
                return Ok(out);
            }
        }
        Err(Error::ResourceLimit("wood tessellation budget"))
    }

    /// The rings one run is drawn through: today's sweep, one ring a point
    /// at twelve sides.
    fn plan(&self, run: usize, _viewer: &Viewer, _error: f64) -> Vec<Ring> {
        self.run_points(&self.runs[run])
            .iter()
            .map(|&point| Ring { point, sides: 12 })
            .collect()
    }

    /// One run's vertices and triangles: its rings, the strips between them
    /// and a cap at each end.
    fn emit(&self, rings: &[Ring], out: &mut Tessellation) {
        let mut starts = Vec::with_capacity(rings.len());
        for (i, ring) in rings.iter().enumerate() {
            starts.push((out.positions.len() / 3) as u32);
            let slope = slope(rings, i);
            for k in 0..ring.sides {
                let angle = std::f64::consts::TAU * f64::from(k) / f64::from(ring.sides);
                let (position, normal) = self.vertex(&ring.point, angle, slope);
                push(
                    out,
                    position,
                    normal,
                    [ring.point.along, angle],
                    ring.point.radius,
                );
            }
        }
        for i in 1..rings.len() {
            zipper(
                out,
                starts[i - 1],
                rings[i - 1].sides,
                starts[i],
                rings[i].sides,
            );
        }
        for (end, away) in [(0, -1.0), (rings.len() - 1, 1.0)] {
            let p = &rings[end].point;
            let centre = (out.positions.len() / 3) as u32;
            push(
                out,
                p.centre,
                p.normal.cross(p.binormal) * away,
                [p.along, 0.0],
                p.radius,
            );
            let (start, sides) = (starts[end], rings[end].sides);
            for k in 0..sides {
                out.indices
                    .extend([centre, start + k, start + (k + 1) % sides]);
            }
        }
        out.rings += rings.len();
    }

    /// A ring's point at `angle`, with the tube's own normal there: the
    /// radial direction tilted by the lobes' turn and the radius's slope.
    fn vertex(&self, p: &CurvePoint, angle: f64, slope: f64) -> (Vec3, Vec3) {
        let (sin, cos) = angle.sin_cos_fixed();
        let radial = p.normal * cos + p.binormal * sin;
        let around = p.binormal * cos - p.normal * sin;
        let tangent = p.normal.cross(p.binormal);
        let phase = std::f64::consts::TAU * self.twist_rate * (p.along / self.height);
        let lobes = f64::from(self.lobes);
        let (profile, turn) = if self.lobes == 0 {
            (1.0, 0.0)
        } else {
            let a = lobes * (angle + phase);
            (
                1.0 + self.lobe_depth * a.cos_fixed(),
                -self.lobe_depth * lobes * a.sin_fixed(),
            )
        };
        let width = p.radius * profile;
        let normal =
            radial * width - around * (p.radius * turn) - tangent * (width * slope * profile);
        (p.centre + radial * width, normal.normalized())
    }
}

/// How fast the radius falls along the curve at ring `i`, from its neighbours.
fn slope(rings: &[Ring], i: usize) -> f64 {
    let (a, b) = (
        &rings[i.saturating_sub(1)].point,
        &rings[(i + 1).min(rings.len() - 1)].point,
    );
    let run = b.along - a.along;
    if run.abs() < 1e-12 {
        0.0
    } else {
        (b.radius - a.radius) / run
    }
}

fn push(out: &mut Tessellation, p: Vec3, n: Vec3, coord: [f64; 2], radius: f64) {
    out.positions.extend([p.x as f32, p.y as f32, p.z as f32]);
    out.normals.extend([n.x as f32, n.y as f32, n.z as f32]);
    out.coords.extend([coord[0] as f32, coord[1] as f32]);
    out.radii.push(radius as f32);
}

/// The triangles between two rings of any side counts, walked by angle so
/// each joins the vertices nearest round it: one a side of either ring.
fn zipper(out: &mut Tessellation, a: u32, na: u32, b: u32, nb: u32) {
    let (mut i, mut j) = (0u32, 0u32);
    while i < na || j < nb {
        let next_a = u64::from(i + 1) * u64::from(nb);
        let next_b = u64::from(j + 1) * u64::from(na);
        if j >= nb || (i < na && next_a <= next_b) {
            out.indices
                .extend([a + i % na, a + (i + 1) % na, b + j % nb]);
            i += 1;
        } else {
            out.indices
                .extend([a + i % na, b + (j + 1) % nb, b + j % nb]);
            j += 1;
        }
    }
}

/// Triangles a run's rings make: its strips and two caps.
fn triangles(rings: &[Ring]) -> usize {
    let strips: u32 = rings.windows(2).map(|w| w[0].sides + w[1].sides).sum();
    strips as usize + (rings[0].sides + rings[rings.len() - 1].sides) as usize
}

#[cfg(test)]
mod tests;
