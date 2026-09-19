//! One leaf in twelve bytes, the same three words on the CPU and on the GPU.
//!
//! A leaf transform is three orthonormal right-handed basis vectors, one
//! uniform scale and a point: a rotation, a number and a translation, which is
//! 3 + 1 + 3 pieces of information written until now as sixteen floats of
//! which four were the constants `Instances::validate` enforced. Word 0 holds
//! the rotation as a smallest-three quaternion - two bits naming the dropped
//! largest component, ten bits each for the other three over plus or minus one
//! over root two. Word 1 holds x and y as unsigned normals over the reference
//! box, word 2 holds z in its low half and the scale as a half float in its
//! high half. WGSL reads word 1 and the low half of word 2 with
//! `unpack2x16unorm` and the scale with `unpack2x16float`.
use crate::math::Vec3;

/// Words one stored leaf occupies. Three, and the same three on both sides of
/// the wire: the storage buffer is the bytes the generator wrote.
pub const WORDS: usize = 3;

/// One stored leaf: rotation, position, scale.
pub type Leaf = [u32; WORDS];

/// The largest a dropped quaternion component leaves the other three, and the
/// codes the range is cut into.
const RANGE: f64 = std::f64::consts::FRAC_1_SQRT_2;
const CODES: f64 = 1023.0;

/// The box a crown's leaf positions are quantised against: per axis a minimum
/// and an extent. It is derived from a family's parameters rather than from
/// the tree it grew, because `timeline::Placement` caches leaves per shoot
/// while the tree's own bounds grow: words quantised against one age's box
/// would decode against a different one at the next.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct Reference {
    pub min: Vec3,
    pub extent: Vec3,
}

impl Default for Reference {
    /// A box of no extent: every position decodes to its corner. What an
    /// empty crown carries, and what a caller who places nothing needs.
    fn default() -> Self {
        Self {
            min: Vec3::ZERO,
            extent: Vec3::ZERO,
        }
    }
}

impl Reference {
    /// The box spanning two corners, in either order, with a negative or
    /// unmeasurable side flattened to none.
    pub fn spanning(a: Vec3, b: Vec3) -> Self {
        let lo = |a: f64, b: f64| a.min(b);
        let hi = |a: f64, b: f64| a.max(b);
        let min = Vec3::new(lo(a.x, b.x), lo(a.y, b.y), lo(a.z, b.z));
        let max = Vec3::new(hi(a.x, b.x), hi(a.y, b.y), hi(a.z, b.z));
        Self {
            min,
            extent: Vec3::new(
                (max.x - min.x).max(0.0),
                (max.y - min.y).max(0.0),
                (max.z - min.z).max(0.0),
            ),
        }
    }

    pub fn max(&self) -> Vec3 {
        self.min + self.extent
    }

    pub fn is_finite(&self) -> bool {
        self.min.is_finite()
            && self.extent.is_finite()
            && self.extent.x >= 0.0
            && self.extent.y >= 0.0
            && self.extent.z >= 0.0
    }

    /// Whether a point stands in the box, which every station does by
    /// construction: the box is what the parameters allow the crown to reach.
    pub fn contains(&self, p: Vec3) -> bool {
        let max = self.max();
        p.x >= self.min.x
            && p.y >= self.min.y
            && p.z >= self.min.z
            && p.x <= max.x
            && p.y <= max.y
            && p.z <= max.z
    }

    /// The step one position code spans on each axis. What the round trip's
    /// error is half of.
    pub fn step(&self) -> Vec3 {
        self.extent / 65535.0
    }

    /// The three words for one column-major transform: its columns' rotation,
    /// its translation over this box and the length of its columns.
    ///
    /// A station outside the box cannot occur by construction. The clamp is
    /// what the release build does if one ever did - a leaf on the wall
    /// rather than a leaf wrapped to the far side - and the assertion is what
    /// says so while the tests run.
    pub fn pack(&self, m: &[f32; 16]) -> Leaf {
        let at = Vec3::new(f64::from(m[12]), f64::from(m[13]), f64::from(m[14]));
        debug_assert!(
            self.contains(at),
            "leaf station outside the reference box: {at:?} not in {self:?}"
        );
        let column = |c: usize| {
            Vec3::new(
                f64::from(m[c * 4]),
                f64::from(m[c * 4 + 1]),
                f64::from(m[c * 4 + 2]),
            )
        };
        let columns = [column(0), column(1), column(2)];
        let scale = (columns[0].length() + columns[1].length() + columns[2].length()) / 3.0;
        let unorm = |v: f64, lo: f64, extent: f64| {
            if extent <= 0.0 {
                return 0u32;
            }
            (((v - lo) / extent).clamp(0.0, 1.0) * 65535.0).round() as u32
        };
        [
            rotation_word(&columns),
            unorm(at.x, self.min.x, self.extent.x) | unorm(at.y, self.min.y, self.extent.y) << 16,
            unorm(at.z, self.min.z, self.extent.z) | u32::from(half_bits(scale)) << 16,
        ]
    }

    /// The column-major transform three words stand for: the rotation's three
    /// columns at the stored scale, and the stored position.
    pub fn unpack(&self, leaf: Leaf) -> [f32; 16] {
        let columns = rotation_columns(leaf[0]);
        let scale = self.scale(leaf);
        let at = self.position(leaf);
        let mut m = [0.0f32; 16];
        for (c, column) in columns.iter().enumerate() {
            m[c * 4] = (column.x * scale) as f32;
            m[c * 4 + 1] = (column.y * scale) as f32;
            m[c * 4 + 2] = (column.z * scale) as f32;
        }
        m[12] = at.x as f32;
        m[13] = at.y as f32;
        m[14] = at.z as f32;
        m[15] = 1.0;
        m
    }

    /// Where a stored leaf stands, without rebuilding the rest of it. The
    /// crown's own bounds, the mass grid and the clumping pass read nothing
    /// else, so none of them pays for a rotation.
    pub fn position(&self, leaf: Leaf) -> Vec3 {
        let unorm = |word: u32, shift: u32| f64::from((word >> shift) & 0xffff) / 65535.0;
        Vec3::new(
            self.min.x + self.extent.x * unorm(leaf[1], 0),
            self.min.y + self.extent.y * unorm(leaf[1], 16),
            self.min.z + self.extent.z * unorm(leaf[2], 0),
        )
    }

    /// The uniform scale a stored leaf carries.
    pub fn scale(&self, leaf: Leaf) -> f64 {
        f64::from(half_value((leaf[2] >> 16) as u16))
    }
}

/// The rotation three orthonormal columns stand for, as a smallest-three
/// quaternion: the index of the largest component in the top two bits, and
/// the other three below it, ten bits each, lowest index first.
fn rotation_word(columns: &[Vec3; 3]) -> u32 {
    let q = quaternion(columns);
    let mut largest = 0;
    for i in 1..4 {
        if q[i].abs() > q[largest].abs() {
            largest = i;
        }
    }
    // A quaternion and its negation are the same rotation, so the dropped
    // component is always taken positive and never has to carry a sign.
    let sign = if q[largest] < 0.0 { -1.0 } else { 1.0 };
    let mut kept = [0.0; 3];
    let mut exact = [0.0; 3];
    let mut slot = 0;
    for (i, &v) in q.iter().enumerate() {
        if i == largest {
            continue;
        }
        kept[slot] = v * sign;
        exact[slot] = ((v * sign + RANGE) / (2.0 * RANGE)).clamp(0.0, 1.0) * CODES;
        slot += 1;
    }
    // Rounding each component on its own is not the nearest rotation. What
    // the dropped component reconstructs to depends on all three, and near
    // half a turn it is small enough that its own error dominates: a leaf
    // rounded component-wise can land three thousandths of a radian off,
    // past the bound the encoding is held to. The cell the exact triple
    // falls in has eight corners, so all eight are tried and the one whose
    // rotation is actually nearest is stored. The decoder is untouched: this
    // is a better choice of code, not a different code.
    let mut best = (f64::NEG_INFINITY, 0u32);
    for corner in 0..8u32 {
        let mut word = (largest as u32) << 30;
        let mut codes = [0.0; 3];
        for slot in 0..3 {
            let up = f64::from((corner >> slot) & 1);
            let code = (exact[slot].floor() + up).clamp(0.0, CODES);
            codes[slot] = code;
            word |= (code as u32) << (slot * 10);
        }
        let v: Vec<f64> = codes
            .iter()
            .map(|c| c / CODES * (2.0 * RANGE) - RANGE)
            .collect();
        let dropped = (1.0 - v[0] * v[0] - v[1] * v[1] - v[2] * v[2])
            .max(0.0)
            .sqrt();
        let exact_dropped = q[largest] * sign;
        let alignment = v[0] * kept[0] + v[1] * kept[1] + v[2] * kept[2] + dropped * exact_dropped;
        if alignment > best.0 {
            best = (alignment, word);
        }
    }
    best.1
}

/// The three columns one rotation word stands for, in the order the transform
/// keeps them: side, leaf axis, face.
fn rotation_columns(word: u32) -> [Vec3; 3] {
    let code = |slot: u32| f64::from((word >> (slot * 10)) & 1023) / CODES * (2.0 * RANGE) - RANGE;
    let (a, b, c) = (code(0), code(1), code(2));
    let dropped = (1.0 - a * a - b * b - c * c).max(0.0).sqrt();
    let q = match word >> 30 {
        0 => [dropped, a, b, c],
        1 => [a, dropped, b, c],
        2 => [a, b, dropped, c],
        _ => [a, b, c, dropped],
    };
    columns(&q)
}

/// The quaternion (w, x, y, z) of a rotation whose columns are these, each
/// taken as a direction so a scaled transform reads the same as a bare one.
fn quaternion(columns: &[Vec3; 3]) -> [f64; 4] {
    let axis = |v: Vec3| {
        if v.length_squared() > 0.0 {
            v.normalized()
        } else {
            Vec3::ZERO
        }
    };
    let (x, y, z) = (axis(columns[0]), axis(columns[1]), axis(columns[2]));
    // Row-major reading of the matrix whose columns are x, y and z.
    let m = [[x.x, y.x, z.x], [x.y, y.y, z.y], [x.z, y.z, z.z]];
    let trace = m[0][0] + m[1][1] + m[2][2];
    // Shepperd's choice: take the branch whose divisor is largest, so no
    // rotation is read through a near-zero one.
    let q = if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0;
        [
            0.25 * s,
            (m[2][1] - m[1][2]) / s,
            (m[0][2] - m[2][0]) / s,
            (m[1][0] - m[0][1]) / s,
        ]
    } else if m[0][0] > m[1][1] && m[0][0] > m[2][2] {
        let s = (1.0 + m[0][0] - m[1][1] - m[2][2]).sqrt() * 2.0;
        [
            (m[2][1] - m[1][2]) / s,
            0.25 * s,
            (m[0][1] + m[1][0]) / s,
            (m[0][2] + m[2][0]) / s,
        ]
    } else if m[1][1] > m[2][2] {
        let s = (1.0 + m[1][1] - m[0][0] - m[2][2]).sqrt() * 2.0;
        [
            (m[0][2] - m[2][0]) / s,
            (m[0][1] + m[1][0]) / s,
            0.25 * s,
            (m[1][2] + m[2][1]) / s,
        ]
    } else {
        let s = (1.0 + m[2][2] - m[0][0] - m[1][1]).sqrt() * 2.0;
        [
            (m[1][0] - m[0][1]) / s,
            (m[0][2] + m[2][0]) / s,
            (m[1][2] + m[2][1]) / s,
            0.25 * s,
        ]
    };
    let length = q.iter().map(|v| v * v).sum::<f64>().sqrt();
    if length > 0.0 {
        q.map(|v| v / length)
    } else {
        [1.0, 0.0, 0.0, 0.0]
    }
}

/// The three columns of the rotation a quaternion (w, x, y, z) stands for.
fn columns(q: &[f64; 4]) -> [Vec3; 3] {
    let (w, x, y, z) = (q[0], q[1], q[2], q[3]);
    [
        Vec3::new(
            1.0 - 2.0 * (y * y + z * z),
            2.0 * (x * y + z * w),
            2.0 * (x * z - y * w),
        ),
        Vec3::new(
            2.0 * (x * y - z * w),
            1.0 - 2.0 * (x * x + z * z),
            2.0 * (y * z + x * w),
        ),
        Vec3::new(
            2.0 * (x * z + y * w),
            2.0 * (y * z - x * w),
            1.0 - 2.0 * (x * x + y * y),
        ),
    ]
}

/// A half float's bits, rounded to nearest with ties to even, saturating at
/// the format's largest finite value rather than reaching infinity.
fn half_bits(v: f64) -> u16 {
    let v = v as f32;
    if !v.is_finite() {
        return if v.is_nan() { 0x7e00 } else { 0x7bff };
    }
    let bits = v.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let exponent = ((bits >> 23) & 0xff) as i32 - 127;
    let mantissa = bits & 0x007f_ffff;
    if exponent > 15 {
        return sign | 0x7bff;
    }
    if exponent < -24 {
        return sign;
    }
    // Subnormal halves carry the leading one explicitly, so the mantissa is
    // shifted by however far the exponent falls short of the format's floor.
    let (shift, mantissa) = if exponent < -14 {
        ((-14 - exponent) as u32, mantissa | 0x0080_0000)
    } else {
        (0, mantissa)
    };
    let low = 13 + shift;
    let kept = mantissa >> low;
    let half = 1u32 << (low - 1);
    let remainder = mantissa & ((1u32 << low) - 1);
    let round = u32::from(remainder > half || (remainder == half && kept & 1 == 1));
    let stored = kept + round;
    let exponent = if exponent < -14 { 0 } else { exponent + 15 } as u32;
    // A mantissa that rounded up out of its width carries into the exponent,
    // which is exactly what the next value up is; saturate rather than reach
    // infinity when it carries out of the largest exponent.
    let packed = (exponent << 10) + stored;
    sign | (packed.min(0x7bff) as u16)
}

/// The value a half float's bits stand for.
fn half_value(bits: u16) -> f32 {
    let sign = u32::from(bits & 0x8000) << 16;
    let exponent = i32::from((bits >> 10) & 0x1f);
    let mantissa = u32::from(bits & 0x03ff);
    if exponent == 0 {
        if mantissa == 0 {
            return f32::from_bits(sign);
        }
        // A subnormal half is a normal single: shift the leading one up out
        // of the mantissa and pay for it in the exponent.
        let shift = mantissa.leading_zeros() - 21;
        let exponent = 127 - 15 - shift;
        let mantissa = (mantissa << (shift + 1)) & 0x03ff;
        return f32::from_bits(sign | (exponent << 23) | (mantissa << 13));
    }
    if exponent == 0x1f {
        return f32::from_bits(sign | 0x7f80_0000 | (mantissa << 13));
    }
    f32::from_bits(sign | ((exponent + 112) as u32) << 23 | (mantissa << 13))
}

#[cfg(test)]
mod tests;
