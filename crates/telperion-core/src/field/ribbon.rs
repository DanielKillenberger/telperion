//! The plan's flat primitive: the rectangle a segment sweeps from `-side` to
//! `side`, pushed out by `thickness` either way along its normal, a flat box.
//! A frond's leaflets fan in one plane, and a ribbon holds them the way a
//! capsule holds a shoot's.
use super::index::checked;
use crate::{foliage::Bounds, math::Vec3, Result};

#[derive(Debug, Clone, Copy)]
pub(super) struct Ribbon {
    pub(super) a: Vec3,
    pub(super) b: Vec3,
    /// Half the width, square to `b - a`.
    pub(super) side: Vec3,
    pub(super) thickness: f64,
}
impl Ribbon {
    /// The box's centre, its three unit axes and its half extent along each.
    fn frame(&self) -> (Vec3, [Vec3; 3], [f64; 3]) {
        let d = self.b - self.a;
        let (length, width) = (d.length(), self.side.length());
        let side = self.side / width;
        let along = if length > 0. {
            d / length
        } else {
            side.perpendicular()
        };
        let axes = [along, side, along.cross(side)];
        let centre = (self.a + self.b) * 0.5;
        (centre, axes, [length / 2., width, self.thickness])
    }
    /// Whether the closed cube of `half` extent about `p` meets the box. The
    /// cube's axes and the box's are tried as separating axes; where neither
    /// separates, the cell counts, so a cell only an edge pair would part is
    /// kept and the answer stays conservative.
    pub(super) fn meets(&self, p: Vec3, half: f64) -> bool {
        let (centre, axes, extent) = self.frame();
        let q = p - centre;
        let reach = |u: Vec3| half * (u.x.abs() + u.y.abs() + u.z.abs());
        let box_meets = axes
            .iter()
            .zip(extent)
            .all(|(u, e)| q.dot(*u).abs() <= e + reach(*u));
        box_meets && (0..3).all(|k| component(q, k).abs() <= half + self.reach(axes, extent, k))
    }
    /// The box's half extent along world axis `k`.
    fn reach(&self, axes: [Vec3; 3], extent: [f64; 3], k: usize) -> f64 {
        axes.iter()
            .zip(extent)
            .map(|(u, e)| component(*u, k).abs() * e)
            .sum()
    }
    /// The world box around the ribbon.
    pub(super) fn bounds(&self) -> Result<Bounds> {
        let (centre, axes, extent) = self.frame();
        let r = Vec3::new(
            self.reach(axes, extent, 0),
            self.reach(axes, extent, 1),
            self.reach(axes, extent, 2),
        );
        let b = Bounds {
            min: centre - r,
            max: centre + r,
        };
        checked(b)?;
        Ok(b)
    }
}

/// One of a vector's components, by world axis.
fn component(v: Vec3, k: usize) -> f64 {
    match k {
        0 => v.x,
        1 => v.y,
        _ => v.z,
    }
}
