//! A curve point as the GPU reads it: 32 bytes (host decision 8). The
//! centre, radius and distance along are float32; the frame's two unit
//! vectors are octahedral, two 24-bit components each in the last three
//! words, so a ring stands off by at most about 1e-6 of its radius (measured
//! in `tests.rs`) and a metres-wide trunk is exact at 5 cm. The twist's
//! phase is the curve's own function of the distance along, so it takes no
//! word.
use super::{Curve, CurvePoint};
use crate::math::Vec3;

/// Words in one packed point.
pub const POINT_WORDS: usize = 8;

/// One packed point.
pub type PackedPoint = [u32; POINT_WORDS];

impl CurvePoint {
    /// This point in 32 bytes.
    pub fn pack(&self) -> PackedPoint {
        let c = self.centre;
        let frame = frame_words(octahedral(self.normal), octahedral(self.binormal));
        [
            (c.x as f32).to_bits(),
            (c.y as f32).to_bits(),
            (c.z as f32).to_bits(),
            (self.radius as f32).to_bits(),
            (self.along as f32).to_bits(),
            frame[0],
            frame[1],
            frame[2],
        ]
    }

    /// The point a packed one stands for, widened back to float64.
    pub fn unpack(words: &PackedPoint) -> Self {
        let f = |w: u32| f64::from(f32::from_bits(w));
        let (normal, binormal) = frame_vectors([words[5], words[6], words[7]]);
        Self {
            centre: Vec3::new(f(words[0]), f(words[1]), f(words[2])),
            radius: f(words[3]),
            along: f(words[4]),
            normal: unoctahedral(normal),
            binormal: unoctahedral(binormal),
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

/// Bits in one octahedral component.
const BITS: u32 = 24;
const MASK: u64 = (1 << BITS) - 1;

/// Two vectors' two components each, 24 bits apiece, into three words.
fn frame_words(normal: [u32; 2], binormal: [u32; 2]) -> [u32; 3] {
    let low = u64::from(normal[0]) | (u64::from(normal[1]) << BITS);
    let high = u64::from(binormal[0]) | (u64::from(binormal[1]) << BITS);
    let all = u128::from(low) | (u128::from(high) << (2 * BITS));
    [all as u32, (all >> 32) as u32, (all >> 64) as u32]
}

fn frame_vectors(words: [u32; 3]) -> ([u32; 2], [u32; 2]) {
    let all = u128::from(words[0]) | (u128::from(words[1]) << 32) | (u128::from(words[2]) << 64);
    let part = |k: u32| ((all >> (k * BITS)) as u64 & MASK) as u32;
    ([part(0), part(1)], [part(2), part(3)])
}

/// A unit vector folded onto the octahedron and quantised to two snorm24.
fn octahedral(v: Vec3) -> [u32; 2] {
    let l1 = v.x.abs() + v.y.abs() + v.z.abs();
    let (mut x, mut y) = (v.x / l1, v.y / l1);
    if v.z < 0.0 {
        let (ox, oy) = (x, y);
        x = (1.0 - oy.abs()) * ox.signum();
        y = (1.0 - ox.abs()) * oy.signum();
    }
    let top = f64::from((1u32 << (BITS - 1)) - 1);
    let q = |c: f64| ((c.clamp(-1.0, 1.0) * top).round() as i32 as u32) & MASK as u32;
    [q(x), q(y)]
}

fn unoctahedral(c: [u32; 2]) -> Vec3 {
    let top = f64::from((1u32 << (BITS - 1)) - 1);
    // Sign-extend the 24-bit two's complement.
    let d = |w: u32| f64::from(((w << (32 - BITS)) as i32) >> (32 - BITS)) / top;
    let (x, y) = (d(c[0]), d(c[1]));
    let z = 1.0 - x.abs() - y.abs();
    let (x, y) = if z < 0.0 {
        ((1.0 - y.abs()) * x.signum(), (1.0 - x.abs()) * y.signum())
    } else {
        (x, y)
    };
    Vec3::new(x, y, z).normalized()
}
