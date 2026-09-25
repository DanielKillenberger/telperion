//! The plan's flat primitive: the trapezoid a segment sweeps from `-side` to
//! `side`, its half-width varying linearly from one end to the other, pushed
//! out either way along its normal by a thickness that varies the same way. A frond's leaflets fan in
//! one plane and shorten toward its tip, and a ribbon holds them the way a
//! capsule holds a shoot's.
use super::index::checked;
use crate::{foliage::Bounds, math::Vec3, Error, Result};

#[derive(Debug, Clone, Copy)]
pub(super) struct Ribbon {
    pub(super) sides: [Vec3; 2],
    /// The slab's own face and edge normals with its extent along each; the
    /// world axes are its bounds.
    axes: [(Vec3, [f64; 2]); 5],
    pub(super) bounds: Bounds,
}
impl Ribbon {
    /// The slab over the segment `a`, `b` with half-widths `sides` and
    /// half-thicknesses `thickness` at its ends, the sides square to the run
    /// and parallel: the convex hull of its two end rectangles.
    pub(super) fn new(a: Vec3, b: Vec3, sides: [Vec3; 2], thickness: [f64; 2]) -> Result<Self> {
        let width = sides[0] + sides[1];
        let breadth = width.length_squared();
        if breadth.is_nan() || breadth <= 0. {
            return Err(Error::InvalidInput("ribbon width"));
        }
        let side = width.normalized();
        let run = (b - a) - side * (b - a).dot(side);
        let along = if run.length_squared() > 0. {
            run.normalized()
        } else {
            side.perpendicular()
        };
        let normal = along.cross(side);
        // Corner `[end][s][n]`: `s` and `n` pick the side and face.
        let corner = |end: usize, s: f64, n: f64| {
            let (at, half, out) = if end == 0 {
                (a, sides[0], thickness[0])
            } else {
                (b, sides[1], thickness[1])
            };
            at + half * s + normal * (out * n)
        };
        let corners: Vec<Vec3> = [0, 1]
            .into_iter()
            .flat_map(|e| {
                [(1., 1.), (1., -1.), (-1., 1.), (-1., -1.)].map(|(s, n)| corner(e, s, n))
            })
            .collect();
        let span = |u: Vec3| {
            corners
                .iter()
                .fold([f64::INFINITY, f64::NEG_INFINITY], |r, c| {
                    let x = c.dot(u);
                    [r[0].min(x), r[1].max(x)]
                })
        };
        // Each long face through three of its corners: two at one end, one
        // at the other; each is a plane, since its edge runs linearly.
        let face = |s: f64, n: f64, flip: bool| {
            let (p, q, r) = if flip {
                (corner(0, s, n), corner(0, s, -n), corner(1, s, n))
            } else {
                (corner(0, s, n), corner(0, -s, n), corner(1, s, n))
            };
            (q - p).cross(r - p).normalized()
        };
        let axes = [
            along,
            face(1., 1., true),
            face(-1., 1., true),
            face(1., 1., false),
            face(1., -1., false),
        ]
        .map(|u| (u, span(u)));
        let [x, y, z] = [Vec3::X, Vec3::Y, Vec3::Z].map(span);
        let bounds = Bounds {
            min: Vec3::new(x[0], y[0], z[0]),
            max: Vec3::new(x[1], y[1], z[1]),
        };
        checked(bounds)?;
        Ok(Self {
            sides,
            axes,
            bounds,
        })
    }
    /// Whether the closed cube of `half` extent about `p` meets the slab. The
    /// cube's axes and the slab's faces are tried as separating axes; where
    /// none separates, the cell counts, so a cell only an edge pair would
    /// part is kept and the answer stays conservative.
    pub(super) fn meets(&self, p: Vec3, half: f64) -> bool {
        let b = &self.bounds;
        let world = p.x + half >= b.min.x
            && p.x - half <= b.max.x
            && p.y + half >= b.min.y
            && p.y - half <= b.max.y
            && p.z + half >= b.min.z
            && p.z - half <= b.max.z;
        world
            && self.axes.iter().all(|(u, [lo, hi])| {
                let (c, r) = (p.dot(*u), half * (u.x.abs() + u.y.abs() + u.z.abs()));
                c + r >= *lo && c - r <= *hi
            })
    }
}
