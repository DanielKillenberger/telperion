//! The plan's second primitive beside the capsule: an oriented box, the
//! rectangle a segment sweeps from `-side` to `side` pushed out by a
//! thickness either way along its normal. A leaflet that stands where the
//! wood does not, a frond's, is planned as one.
use super::index::checked;
use crate::{math::Vec3, pipeline::foliage::Bounds, Error, Result};

#[derive(Debug, Clone, Copy)]
pub(super) struct Cuboid {
    pub(super) side: Vec3,
    /// Its three unit axes with its extent along each; the world axes are
    /// its bounds.
    axes: [(Vec3, [f64; 2]); 3],
    pub(super) bounds: Bounds,
}
impl Cuboid {
    /// The box about the segment `a`, `b`, `side` its half-width square to
    /// the segment and `thickness` its half-thickness square to both.
    pub(super) fn new(a: Vec3, b: Vec3, side: Vec3, thickness: f64) -> Result<Self> {
        let breadth = side.length_squared();
        if breadth.is_nan() || breadth <= 0. {
            return Err(Error::InvalidInput("box width"));
        }
        let across = side.normalized();
        let run = (b - a) - across * (b - a).dot(across);
        let along = if run.length_squared() > 0. {
            run.normalized()
        } else {
            across.perpendicular()
        };
        let normal = along.cross(across);
        let centre = (a + b) * 0.5;
        let extent = [(b - a).dot(along) * 0.5, side.length(), thickness];
        let axes = [along, across, normal];
        let span = |u: Vec3| {
            let r: f64 = axes
                .iter()
                .zip(extent)
                .map(|(v, e)| v.dot(u).abs() * e)
                .sum();
            let c = centre.dot(u);
            [c - r, c + r]
        };
        let [x, y, z] = [Vec3::X, Vec3::Y, Vec3::Z].map(span);
        let bounds = Bounds {
            min: Vec3::new(x[0], y[0], z[0]),
            max: Vec3::new(x[1], y[1], z[1]),
        };
        checked(bounds)?;
        Ok(Self {
            side,
            axes: axes.map(|u| (u, span(u))),
            bounds,
        })
    }
    /// Whether the closed cube of `half` extent about `p` meets the box. The
    /// cube's axes and the box's are tried as separating axes; where none
    /// separates, the cell counts, so a cell only an edge pair would part is
    /// kept and the answer stays conservative.
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
