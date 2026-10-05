//! The CPU reference tessellation of a curve (fn-208, host decisions 2 to 5
//! and 14): the wood surfaced for one view at one error in pixels, cluster
//! by cluster, in the order and layout the GPU passes write. Camera-dependent
//! by definition; it reads only the curve.
use super::{Curve, CurvePoint};
use crate::{
    math::{Transcendental, Vec3},
    Error, Result,
};
use plan::tangent;

mod plan;
pub use plan::RIBBON;

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
    /// Whether it stands at least `RIBBON` pixels in radius.
    tube: bool,
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
            let mut out = Tessellation {
                scale,
                ..Tessellation::default()
            };
            for plan in &plans {
                self.emit(plan, viewer, &mut out);
            }
            let fits = budget.is_none_or(|b| {
                out.radii.len() <= b.vertices
                    && out.indices.len() <= b.tube_indices
                    && out.ribbons.len() <= b.ribbon_indices
            });
            if fits {
                return Ok(out);
            }
        }
        Err(Error::ResourceLimit("wood tessellation budget"))
    }

    /// One cluster's vertices and triangles, ring by ring. A stretch between
    /// two tube rings is a tube's strip; any other a ribbon's quad. A ring
    /// keeps the forms its stretches need: its polygon where a tube stretch
    /// meets it, closed by a cap where its tube ends, and its two ribbon
    /// vertices where a ribbon stretch meets it.
    fn emit(&self, plan: &plan::Plan, viewer: &Viewer, out: &mut Tessellation) {
        let rings = &plan.rings;
        out.rings += rings.len();
        let stretch = |i: usize| rings[i].tube && rings[i + 1].tube;
        let mut previous = (0u32, 0u32);
        for (i, ring) in rings.iter().enumerate() {
            let before = i > 0 && stretch(i - 1);
            let after = i + 1 < rings.len() && stretch(i);
            let ribbon = (i > 0 && !before) || (i + 1 < rings.len() && !after);
            let mut tube_start = 0;
            if before || after {
                tube_start = vertices(out);
                self.polygon(ring, out);
                if before {
                    zipper(out, previous.0, rings[i - 1].sides, tube_start, ring.sides);
                }
            }
            let mut ribbon_start = 0;
            if ribbon {
                ribbon_start = vertices(out);
                self.across(&ring.point, viewer, out);
                if i > 0 && !before {
                    let (a, b) = (previous.1, ribbon_start);
                    out.ribbons.extend([a, a + 1, b, b, a + 1, b + 1]);
                    out.ribbons
                        .extend([a + 1, a + 2, b + 1, b + 1, a + 2, b + 2]);
                }
            }
            if before || after {
                for (open, away) in [(!before, -1.0), (!after, 1.0)] {
                    if open {
                        cap(out, &ring.point, tube_start, ring.sides, away);
                    }
                }
            }
            previous = (tube_start, ribbon_start);
        }
    }

    /// A tube ring's vertices.
    fn polygon(&self, ring: &Ring, out: &mut Tessellation) {
        for k in 0..ring.sides {
            let angle = std::f64::consts::TAU * f64::from(k) / f64::from(ring.sides);
            let (position, normal) = match ring.shape {
                Some(shape) => self.cell_vertex(shape, angle),
                None => self.vertex(&ring.point, angle, ring.slope),
            };
            push(
                out,
                position,
                normal,
                [ring.point.along, angle],
                ring.point.radius,
                1.0,
            );
        }
    }

    /// A ribbon ring's three vertices, across the wood as the eye sees it:
    /// its two silhouette edges and its middle, each with the normal the
    /// tube has there and the bark's angle round it, so a ribbon shades as
    /// the half of the tube facing the eye does, as wide as the wood.
    fn across(&self, p: &CurvePoint, viewer: &Viewer, out: &mut Tessellation) {
        let view = viewer.towards(p.centre);
        let t = tangent(p);
        let across = t.cross(view);
        let side = if across.length_squared() > 1e-12 {
            across.normalized()
        } else {
            p.binormal
        };
        let facing = -view - t * (-view).dot(t);
        let facing = if facing.length_squared() > 1e-12 {
            facing.normalized()
        } else {
            p.normal
        };
        // At the wood's true width (fn-208 step 5: a pixel-wide ribbon with
        // its share as alpha-to-coverage read pale beside today's crown; the
        // multisampled rasteriser's own coverage of the true width does not).
        let half = p.radius;
        let coverage = 1.0;
        for (offset, normal) in [(-1.0, -side), (0.0, facing), (1.0, side)] {
            let angle = normal.dot(p.binormal).atan2_fixed(normal.dot(p.normal));
            let at = p.centre + side * (half * offset);
            push(out, at, normal, [p.along, angle], p.radius, coverage);
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

/// A tube's end closed by a fan about its centre, facing `away` along it.
fn cap(out: &mut Tessellation, p: &CurvePoint, start: u32, sides: u32, away: f64) {
    let centre = vertices(out);
    push(
        out,
        p.centre,
        tangent(p) * away,
        [p.along, 0.0],
        p.radius,
        1.0,
    );
    for k in 0..sides {
        out.indices
            .extend([centre, start + k, start + (k + 1) % sides]);
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
