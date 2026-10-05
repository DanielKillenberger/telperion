//! A curve point as the GPU reads it: 28 bytes (host decision 1). The
//! centre, radius and distance along are float32; the frame's two unit
//! vectors are octahedral, two 16-bit components each, so a ring stands off
//! by at most about 1e-4 of its radius (measured in `tests.rs`). The twist's
//! phase is the curve's own function of the distance along, so it takes no
//! word.
use super::{Curve, CurvePoint};
use crate::math::Vec3;

/// Words in one packed point.
pub const POINT_WORDS: usize = 7;

/// One packed point.
pub type PackedPoint = [u32; POINT_WORDS];

impl CurvePoint {
    /// This point in 28 bytes.
    pub fn pack(&self) -> PackedPoint {
        let c = self.centre;
        [
            (c.x as f32).to_bits(),
            (c.y as f32).to_bits(),
            (c.z as f32).to_bits(),
            (self.radius as f32).to_bits(),
            (self.along as f32).to_bits(),
            octahedral(self.normal),
            octahedral(self.binormal),
        ]
    }

    /// The point a packed one stands for, widened back to float64.
    pub fn unpack(words: &PackedPoint) -> Self {
        let f = |w: u32| f64::from(f32::from_bits(w));
        Self {
            centre: Vec3::new(f(words[0]), f(words[1]), f(words[2])),
            radius: f(words[3]),
            along: f(words[4]),
            normal: unoctahedral(words[5]),
            binormal: unoctahedral(words[6]),
        }
    }
}

impl Curve {
    /// Every point packed, in order.
    pub fn packed(&self) -> Vec<PackedPoint> {
        self.points.iter().map(CurvePoint::pack).collect()
    }

    /// The curve as the GPU would read it: each point packed and widened
    /// back, runs, clusters and sections as they are.
    pub fn as_packed(&self) -> Curve {
        Curve {
            points: self
                .points
                .iter()
                .map(|p| CurvePoint::unpack(&p.pack()))
                .collect(),
            ..self.clone()
        }
    }
}

/// A unit vector folded onto the octahedron and quantised to two snorm16.
fn octahedral(v: Vec3) -> u32 {
    let l1 = v.x.abs() + v.y.abs() + v.z.abs();
    let (mut x, mut y) = (v.x / l1, v.y / l1);
    if v.z < 0.0 {
        let (ox, oy) = (x, y);
        x = (1.0 - oy.abs()) * ox.signum();
        y = (1.0 - ox.abs()) * oy.signum();
    }
    let q = |c: f64| ((c.clamp(-1.0, 1.0) * 32767.0).round() as i16) as u16 as u32;
    q(x) | (q(y) << 16)
}

fn unoctahedral(w: u32) -> Vec3 {
    let d = |c: u32| f64::from(c as u16 as i16) / 32767.0;
    let (x, y) = (d(w & 0xffff), d(w >> 16));
    let z = 1.0 - x.abs() - y.abs();
    let (x, y) = if z < 0.0 {
        ((1.0 - y.abs()) * x.signum(), (1.0 - x.abs()) * y.signum())
    } else {
        (x, y)
    };
    Vec3::new(x, y, z).normalized()
}
