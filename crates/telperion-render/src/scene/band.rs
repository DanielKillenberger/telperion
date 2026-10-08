//! The margin band's share of a leaf's area (fn-217 DESIGN.md 8.2): what a
//! leaf under a pixel shows of its band. The band is where the edge
//! coordinate `max(|across|, along)` passes `1 - width`; that coordinate
//! does not run with the blade's area, so its share is measured on the
//! element's own finest triangles, once per submitted element.
use telperion_core::foliage::Element;

/// The band widths the table holds, from none to the row's bound of half.
const STEPS: usize = 32;
const WIDEST: f64 = 0.5;
/// Each triangle is cut this many times along each side to integrate it.
const CUTS: usize = 16;

/// The area share of the band at each of `STEPS + 1` widths from zero to
/// `WIDEST`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BandShares([f32; STEPS + 1]);

impl Default for BandShares {
    /// A band as wide as its coordinate's share: the element's own
    /// measure replaces it on every submission.
    fn default() -> Self {
        Self(std::array::from_fn(|i| {
            (WIDEST * i as f64 / STEPS as f64) as f32
        }))
    }
}

impl BandShares {
    /// Measured on `e`'s finest triangles, the coordinate across signed by
    /// the side of the midrib the vertex stands on, as the vertex stage
    /// restores it before interpolation.
    pub fn of(e: &Element) -> Self {
        if e.coords.is_empty() {
            return Self::default();
        }
        let vertex = |i: u32| {
            let (p, at) = (e.positions[i as usize], 2 * i as usize);
            let across = f64::from(e.coords[at + 1]) * p.x.signum();
            (p, f64::from(e.coords[at]), across)
        };
        let mut bins = [0.0f64; STEPS + 1];
        let mut total = 0.0;
        for t in e.indices.chunks_exact(3) {
            let [a, b, c] = [t[0], t[1], t[2]].map(vertex);
            let area = (b.0 - a.0).cross(c.0 - a.0).length() * 0.5;
            let piece = area / (CUTS * CUTS) as f64;
            for (u, v) in centroids() {
                let w = 1.0 - u - v;
                let along = a.1 * w + b.1 * u + c.1 * v;
                let across = a.2 * w + b.2 * u + c.2 * v;
                let edge = across.abs().max(along);
                // Every width whose band this piece falls in.
                let from = ((1.0 - edge) / WIDEST * STEPS as f64).max(0.0).ceil() as usize;
                for bin in bins.iter_mut().skip(from) {
                    *bin += piece;
                }
                total += piece;
            }
        }
        Self(bins.map(|a| (a / total.max(1e-30)) as f32))
    }

    /// The band's area share at `width`, read between the table's widths.
    pub fn at(&self, width: f64) -> f32 {
        let x = (width / WIDEST).clamp(0.0, 1.0) * STEPS as f64;
        let i = (x as usize).min(STEPS - 1);
        let t = (x - i as f64) as f32;
        self.0[i] * (1.0 - t) + self.0[i + 1] * t
    }
}

/// The centroids of a triangle cut `CUTS` times along each side, in the
/// barycentric weights of its second and third corners.
fn centroids() -> impl Iterator<Item = (f64, f64)> {
    let n = CUTS as f64;
    (0..CUTS).flat_map(move |i| {
        (0..CUTS - i).flat_map(move |j| {
            let (i, j) = (i as f64, j as f64);
            let up = ((i + 1.0 / 3.0) / n, (j + 1.0 / 3.0) / n);
            let down = ((i + 2.0 / 3.0) / n, (j + 2.0 / 3.0) / n);
            let inside = i + j + 1.0 < n;
            std::iter::once(up).chain(inside.then_some(down))
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use telperion_core::math::Vec3;

    /// A square blade, the across coordinate running with its width and
    /// the along coordinate with its length, has its band's area exactly
    /// where its coordinate says, the tip strip included.
    #[test]
    fn a_square_blade_s_band_is_its_coordinate_s_share() {
        let e = Element {
            positions: vec![
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(-1.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ],
            coords: vec![0., 1., 0., 1., 1., 1., 1., 1., 0., 0., 1., 0.],
            indices: vec![4, 1, 2, 4, 2, 5, 0, 4, 5, 0, 5, 3],
            ..Element::default()
        };
        let s = BandShares::of(&e);
        // The band is |across| > 1 - w or along > 1 - w: 1 - (1 - w)^2.
        for w in [0.0, 0.1, 0.25, 0.5] {
            let expected = 1.0 - (1.0 - w) * (1.0 - w);
            assert!(
                (f64::from(s.at(w)) - expected).abs() < 0.02,
                "w {w}: {}",
                s.at(w)
            );
        }
    }
}
