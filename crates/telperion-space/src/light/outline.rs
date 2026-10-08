//! Light read from the crown's outline, per bud (fn-226 R3;
//! R4-INVENTORY.md sections 3 and 7). Once a cycle the structure above
//! the cut folds the outline of the cycle's leaves (`fold.rs`): a solid
//! of revolution about the seed's vertical, `r(z) = R quadrant(p, s)`.
//! Its volume is taken by a 64-point midpoint rule and the cycle's leaf
//! area spread evenly through it; its profile is sampled as 16 stacked
//! cone sections. A bud's light is Beer–Lambert's transmission along
//! each of the sky's directions, `exp(-k D L)`, `L` the ray's chords
//! through the sections (`chord.rs`), weighted by the direction's share.
//! Every value is contract A's Q32; a bud's read takes one `exp` per
//! direction and no other transcendental, and reads nothing but its own
//! place and the outline.
mod chord;
pub(crate) mod fold;
#[cfg(test)]
mod tests;

use super::Light;
use crate::fixed::feedback::vector::isqrt;
use crate::fixed::{exp, exp2, log2, mul, q32, INF, ONE32};
pub(crate) use chord::Section;

/// The outline's cone sections, and the heights of its volume's rule.
pub(crate) const SECTIONS: usize = 16;
const HEIGHTS: usize = 64;
/// The radii a height's mean transmission is read at: the middles, by
/// area, of four rings of equal area (host decision S5).
const RINGS: usize = 4;

/// The crown's outline of one cycle, as every bud reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Outline {
    /// Its base, height, widest reach, fullness and shoulder, Q32.
    pub shape: [i64; 5],
    pub sections: [Section; SECTIONS],
    /// Its volume (m³) and leaf area density (m² m⁻³), Q32.
    pub volume: i64,
    pub density: i64,
    /// Extinction times density, and the sky's directions and shares.
    kd: i64,
    sky: Vec<([i64; 3], i64)>,
    /// Its mean transmission over its volume (host decision S5).
    pub mean: i64,
}

/// `quadrant(p, s) = (1 - p^s)^(1/s)` of Q32 `p` in [0, 1] and shoulder
/// `s`, on contract A's `log2` and `exp2`: 1 at `p = 0`, 0 at `p = 1`.
pub(crate) fn quadrant(p: i64, s: i64) -> i64 {
    if p <= 0 {
        return ONE32;
    }
    if p >= ONE32 || s <= 0 {
        return 0;
    }
    let ln = |x: i64| log2(x as u64) - (32 << 32);
    let rest = ONE32 - exp2(mul(s, ln(p)));
    if rest <= 0 {
        return 0;
    }
    exp2(((i128::from(ln(rest)) << 32) / i128::from(s)) as i64)
}

/// The outline's radius at height `z`, Q32: none outside its span; at
/// an end, the profile's limit there (the widest reach where the
/// fullness puts it at that end).
pub(crate) fn radius([base, height, reach, full, s]: [i64; 5], z: i64) -> i64 {
    let span = height - base;
    if span <= 0 || z < base || z > height {
        return 0;
    }
    let t = ((i128::from(z - base) << 32) / i128::from(span)) as i64;
    let p = if t < full {
        ONE32 - ((i128::from(t) << 32) / i128::from(full)) as i64
    } else if full >= ONE32 {
        0
    } else {
        ((i128::from(t - full) << 32) / i128::from(ONE32 - full)) as i64
    };
    mul(reach, quadrant(p, s))
}

impl Outline {
    /// The outline of `shape` holding `area` square metres of leaf, under
    /// `light`.
    pub fn new(shape: [i64; 5], area: i64, light: &Light) -> Self {
        let [base, height, ..] = shape;
        let span = (height - base).max(0);
        let z = |k: usize, of: usize| base + (i128::from(span) * k as i128 / of as i128) as i64;
        let sections = std::array::from_fn(|k| {
            let z = [z(k, SECTIONS), z(k + 1, SECTIONS)];
            Section {
                z,
                r: z.map(|z| radius(shape, z).max(0)),
            }
        });
        // The midpoint rule's heights and their radii squared.
        let middles: Vec<(i64, i64)> = (0..HEIGHTS)
            .map(|j| {
                let z =
                    base + (i128::from(span) * (2 * j + 1) as i128 / (2 * HEIGHTS) as i128) as i64;
                let r = radius(shape, z);
                (z, mul(r, r))
            })
            .collect();
        let squares: i128 = middles.iter().map(|m| i128::from(m.1)).sum();
        let slice = i128::from(span) / HEIGHTS as i128;
        let pi = i128::from(q32(std::f64::consts::PI));
        let volume = ((((squares * slice) >> 32) * pi) >> 32) as i64;
        let density = match volume > 0 {
            true => ((i128::from(area) << 32) / i128::from(volume)).min(i128::from(INF)) as i64,
            false => 0,
        };
        let sky = light
            .directions()
            .into_iter()
            .filter(|d| d.1 > 0.0)
            .map(|(d, w)| ([q32(d.x), q32(d.y), q32(d.z)], q32(w)))
            .collect();
        let mut outline = Self {
            shape,
            sections,
            volume,
            density,
            kd: mul(q32(light.extinction), density),
            sky,
            mean: ONE32,
        };
        outline.mean = outline.mean_over(&middles);
        outline
    }

    /// The light at `p` (Q32 point), Q32 in [0, 1]: 1 where no leaf
    /// stands in the outline.
    pub fn at(&self, p: [i64; 3]) -> i64 {
        if self.kd <= 0 {
            return ONE32;
        }
        let lit: i64 = self
            .sky
            .iter()
            .map(|&(d, w)| {
                let path: i64 = self.sections.iter().map(|s| chord::chord(s, p, d)).sum();
                mul(w, exp(-mul(self.kd, path)))
            })
            .sum();
        lit.clamp(0, ONE32)
    }

    /// The mean transmission over the volume (host decision S5): at each
    /// of the rule's heights, four radii at the middles of rings of equal
    /// area and four azimuths, each height weighted by its slice's
    /// volume.
    fn mean_over(&self, middles: &[(i64, i64)]) -> i64 {
        if self.kd <= 0 {
            return ONE32;
        }
        // sqrt((2i + 1) / 8): the middle by area of ring i of four.
        let ring: [i64; RINGS] =
            std::array::from_fn(|i| isqrt((((2 * i + 1) as u128) << 64) / 8) as i64);
        let (mut num, mut den) = (0i128, 0i128);
        for &(z, square) in middles.iter().filter(|m| m.1 > 0) {
            let r = isqrt((square as u128) << 32) as i64;
            let mut sum = 0i128;
            for &share in &ring {
                let rho = mul(r, share);
                for p in [[rho, 0, z], [0, rho, z], [-rho, 0, z], [0, -rho, z]] {
                    sum += i128::from(self.at(p));
                }
            }
            num += i128::from(square) * sum;
            den += i128::from(square) * (RINGS * 4) as i128;
        }
        match den > 0 {
            true => (num / den) as i64,
            false => ONE32,
        }
    }
}
