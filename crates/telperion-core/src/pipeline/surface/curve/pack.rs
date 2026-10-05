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

/// Words in one packed cluster, as the GPU passes read it: its first point
/// and count, its run's first point and count, its flags (1 the run's first
/// cluster, 2 its last, 4 a shaped run, the section's index from bit 8), its
/// sphere's centre and reach, its largest radius and its six ring-level
/// errors (fn-208).
pub const CLUSTER_WORDS: usize = 16;
/// Floats in one packed section: origin, axis, radial, across, the two
/// corners, the flatness and three rings of reach, rise and scale, padded.
pub const SECTION_FLOATS: usize = 28;

impl Curve {
    /// Every cluster packed for the GPU, in order.
    pub fn packed_clusters(&self) -> Vec<[u32; CLUSTER_WORDS]> {
        let mut out = Vec::with_capacity(self.clusters.len());
        for (i, c) in self.clusters.iter().enumerate() {
            let run = &self.runs[c.run as usize];
            let first = i == 0 || self.clusters[i - 1].run != c.run;
            let last = self.clusters.get(i + 1).is_none_or(|n| n.run != c.run);
            let flags =
                u32::from(first) | (u32::from(last) << 1) | run.section.map_or(0, |s| 4 | (s << 8));
            let f = |v: f64| (v as f32).to_bits();
            let mut words = [
                c.first,
                c.count,
                run.first,
                run.count,
                flags,
                f(c.centre.x),
                f(c.centre.y),
                f(c.centre.z),
                f(c.reach),
                f(c.largest_radius),
                0,
                0,
                0,
                0,
                0,
                0,
            ];
            for (k, e) in c.errors.iter().enumerate() {
                words[10 + k] = e.to_bits();
            }
            out.push(words);
        }
        out
    }

    /// Every section packed for the GPU, in order.
    pub fn packed_sections(&self) -> Vec<[f32; SECTION_FLOATS]> {
        self.sections
            .iter()
            .map(|s| {
                let mut out = [0.0f32; SECTION_FLOATS];
                let v = [s.origin, s.axis, s.radial, s.across];
                for (k, v) in v.iter().enumerate() {
                    out[3 * k..3 * k + 3].copy_from_slice(&[v.x as f32, v.y as f32, v.z as f32]);
                }
                let c = s.corners;
                out[12..16]
                    .copy_from_slice(&[c[0][0], c[0][1], c[1][0], c[1][1]].map(|v| v as f32));
                out[16] = s.flatness as f32;
                for (k, r) in s.rings.iter().enumerate() {
                    out[17 + 3 * k..20 + 3 * k].copy_from_slice(&[
                        r.reach as f32,
                        r.rise as f32,
                        r.scale as f32,
                    ]);
                }
                out
            })
            .collect()
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
