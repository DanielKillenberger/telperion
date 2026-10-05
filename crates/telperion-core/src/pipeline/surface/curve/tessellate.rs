//! The CPU reference tessellation of a curve (fn-208, host decisions 2 to 5
//! and 14): the wood surfaced for one view at one error in pixels, cluster
//! by cluster, in the order and layout the GPU passes write. Camera-dependent
//! by definition; it reads only the curve.
use super::{Curve, CurvePoint};
use crate::{
    math::{Transcendental, Vec3},
    Error, Result,
};
use plan::{tangent, Mode};

mod plan;

/// Where the wood is seen from: the eye, the unit direction it looks along,
/// the pixels a metre spans at one metre's depth (half the viewport's height
/// over the tangent of half the field of view; for an orthographic view, at
/// every depth), the near plane, and the frustum's six planes (`a·x + b·y +
/// c·z + d ≥ 0` inside, normals of unit length) where the view culls.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewer {
    pub eye: Vec3,
    pub forward: Vec3,
    pub pixels_per_metre: f64,
    pub near: f64,
    pub orthographic: bool,
    pub planes: Option<[[f64; 4]; 6]>,
}

impl Viewer {
    /// Pixels a metre spans at `point`'s depth.
    pub fn pixels_at(&self, point: Vec3) -> f64 {
        if self.orthographic {
            return self.pixels_per_metre;
        }
        self.pixels_per_metre / (point - self.eye).dot(self.forward).max(self.near)
    }

    /// The direction from the eye towards `point`.
    fn towards(&self, point: Vec3) -> Vec3 {
        if self.orthographic {
            return self.forward;
        }
        let v = point - self.eye;
        if v.length_squared() > 0.0 {
            v.normalized()
        } else {
            self.forward
        }
    }
}

/// The most one view may write: vertices, tube indices and ribbon indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Budget {
    pub vertices: usize,
    pub tube_indices: usize,
    pub ribbon_indices: usize,
}

/// The wood for one view, in the renderer's layout: three floats a position
/// and a normal, the bark's (along, angle), the radius and the coverage a
/// vertex (1 on a tube, a coverage ribbon's true share of its pixel width),
/// the tubes' triangles and the ribbons'. `scale` is the error the budget let
/// it be drawn at, in multiples of the error asked for.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tessellation {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub coords: Vec<f32>,
    pub radii: Vec<f32>,
    pub coverage: Vec<f32>,
    pub indices: Vec<u32>,
    pub ribbons: Vec<u32>,
    pub rings: usize,
    pub scale: f64,
}

/// One ring the surface passes through: its point, the radius's slope along
/// the curve there, its sides, and for a shaped run its cell's ring (the
/// section, the point's index in its run and the run's count).
#[derive(Debug, Clone, Copy)]
struct Ring {
    point: CurvePoint,
    slope: f64,
    sides: u32,
    shape: Option<(u32, usize, usize)>,
}

/// The error scales a budget may draw at, finest first.
pub const SCALES: [f64; 4] = [1.0, 2.0, 4.0, 8.0];

impl Curve {
    /// The wood surfaced for `viewer` within `error` pixels. With a budget,
    /// a view that would need more is drawn at two, four or eight times the
    /// error, the scale it took reported; past that it is refused by name,
    /// never truncated.
    pub fn tessellate(
        &self,
        viewer: &Viewer,
        error: f64,
        budget: Option<Budget>,
    ) -> Result<Tessellation> {
        let positive = |v: f64| v.is_finite() && v > 0.0;
        if !positive(error) || !positive(viewer.pixels_per_metre) || !positive(viewer.near) {
            return Err(Error::InvalidInput("tessellation view or error"));
        }
        for scale in SCALES {
            let plans: Vec<_> = (0..self.clusters.len())
                .filter_map(|c| self.plan(c, viewer, error * scale))
                .collect();
            let fits = budget.is_none_or(|b| {
                let count = |f: fn(&plan::Plan) -> [usize; 3]| {
                    plans
                        .iter()
                        .map(f)
                        .fold([0; 3], |a, n| [a[0] + n[0], a[1] + n[1], a[2] + n[2]])
                };
                let [v, t, r] = count(counts);
                v <= b.vertices && t <= b.tube_indices && r <= b.ribbon_indices
            });
            if fits {
                let mut out = Tessellation {
                    scale,
                    ..Tessellation::default()
                };
                for plan in &plans {
                    self.emit(plan, viewer, &mut out);
                }
                return Ok(out);
            }
        }
        Err(Error::ResourceLimit("wood tessellation budget"))
    }

    /// One cluster's vertices and triangles.
    fn emit(&self, plan: &plan::Plan, viewer: &Viewer, out: &mut Tessellation) {
        out.rings += plan.rings.len();
        match plan.mode {
            Mode::Ribbon => self.ribbon(&plan.rings, viewer, out),
            Mode::Tube => self.tube(plan, out),
        }
    }

    /// A tube: its rings, the strips between them and a cap at an open end.
    fn tube(&self, plan: &plan::Plan, out: &mut Tessellation) {
        let rings = &plan.rings;
        let mut starts = Vec::with_capacity(rings.len());
        for ring in rings {
            starts.push(vertices(out));
            for k in 0..ring.sides {
                let angle = std::f64::consts::TAU * f64::from(k) / f64::from(ring.sides);
                let (position, normal) = match ring.shape {
                    Some(shape) => self.cell_vertex(shape, angle),
                    None => self.vertex(&ring.point, angle, ring.slope),
                };
                let coord = [ring.point.along, angle];
                push(out, position, normal, coord, ring.point.radius, 1.0);
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
            if !plan.caps[usize::from(away > 0.0)] {
                continue;
            }
            let p = &rings[end].point;
            let centre = vertices(out);
            push(
                out,
                p.centre,
                tangent(p) * away,
                [p.along, 0.0],
                p.radius,
                1.0,
            );
            let (start, sides) = (starts[end], rings[end].sides);
            for k in 0..sides {
                out.indices
                    .extend([centre, start + k, start + (k + 1) % sides]);
            }
        }
    }

    /// A ribbon facing the eye: two vertices a ring, as wide as the wood or,
    /// under a pixel, a pixel wide with the wood's true share as coverage.
    fn ribbon(&self, rings: &[Ring], viewer: &Viewer, out: &mut Tessellation) {
        let start = vertices(out);
        for ring in rings {
            let p = &ring.point;
            let view = viewer.towards(p.centre);
            let across = tangent(p).cross(view);
            let side = if across.length_squared() > 1e-12 {
                across.normalized()
            } else {
                p.binormal
            };
            let half = p.radius.max(0.5 / viewer.pixels_at(p.centre));
            let coverage = p.radius / half;
            for sign in [-1.0, 1.0] {
                push(
                    out,
                    p.centre + side * (half * sign),
                    -view,
                    [p.along, 0.0],
                    p.radius,
                    coverage,
                );
            }
        }
        for i in 1..rings.len() as u32 {
            let (a, b) = (start + 2 * (i - 1), start + 2 * i);
            out.ribbons.extend([a, a + 1, b, b, a + 1, b + 1]);
        }
    }

    /// A shaped run's ring point at `angle`: the cell's own, facing out from
    /// the ring's centre.
    fn cell_vertex(
        &self,
        (section, index, count): (u32, usize, usize),
        angle: f64,
    ) -> (Vec3, Vec3) {
        let cell = &self.sections[section as usize];
        let ring = cell.ring(index, count);
        let (sin, cos) = angle.sin_cos_fixed();
        let p = cell.vertex(ring, cos, sin);
        (p, (p - cell.centre(ring)).normalized())
    }

    /// A ring's point at `angle`, with the tube's own normal there: the
    /// radial direction tilted by the lobes' turn and the radius's slope.
    fn vertex(&self, p: &CurvePoint, angle: f64, slope: f64) -> (Vec3, Vec3) {
        let (sin, cos) = angle.sin_cos_fixed();
        let radial = p.normal * cos + p.binormal * sin;
        let around = p.binormal * cos - p.normal * sin;
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
            radial * width - around * (p.radius * turn) - tangent(p) * (width * slope * profile);
        (p.centre + radial * width, normal.normalized())
    }
}

/// What one cluster writes: vertices, tube indices and ribbon indices.
fn counts(plan: &plan::Plan) -> [usize; 3] {
    let rings = &plan.rings;
    match plan.mode {
        Mode::Ribbon => [2 * rings.len(), 0, 6 * (rings.len() - 1)],
        Mode::Tube => {
            let sides: usize = rings.iter().map(|r| r.sides as usize).sum();
            let strips: usize = rings
                .windows(2)
                .map(|w| (w[0].sides + w[1].sides) as usize)
                .sum();
            let ends = [rings[0].sides, rings[rings.len() - 1].sides];
            let caps: usize = (0..2)
                .filter(|&e| plan.caps[e])
                .map(|e| ends[e] as usize)
                .sum();
            let capped = plan.caps.iter().filter(|&&c| c).count();
            [sides + capped, 3 * (strips + caps), 0]
        }
    }
}

fn vertices(out: &Tessellation) -> u32 {
    (out.positions.len() / 3) as u32
}

fn push(out: &mut Tessellation, p: Vec3, n: Vec3, coord: [f64; 2], radius: f64, coverage: f64) {
    out.positions.extend([p.x as f32, p.y as f32, p.z as f32]);
    out.normals.extend([n.x as f32, n.y as f32, n.z as f32]);
    out.coords.extend([coord[0] as f32, coord[1] as f32]);
    out.radii.push(radius as f32);
    out.coverage.push(coverage as f32);
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

#[cfg(test)]
mod tests;
