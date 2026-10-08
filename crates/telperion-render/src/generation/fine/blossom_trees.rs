//! fn-234 R1's trees and the crown they are measured against
//! (`blossom_measure`): fn-217 task 3's flowering fixtures, restated.
use telperion_core::foliage::{BloomParams, Instances};
use telperion_core::math::Vec3;
use telperion_core::presets::{Family, Preset};
use telperion_core::supernatural::Supernatural;

/// A beech of 40 whose ages 12 and 14 flower whole (fn-217 `flowering`).
fn beech() -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    (f.age, f.skeleton.seed) = (40.0, 1);
    let species = f.space.species.as_mut().unwrap();
    for k in [12, 14] {
        species.states[k].flowering = 1.0;
    }
    f
}

/// fn-217's `cherry`: the corymb with its eye and stamens, six florets of
/// 16 mm petals.
pub(super) fn cherry() -> Family {
    let mut f = beech();
    f.bloom = BloomParams {
        florets: 6.0,
        rachis: 0.0,
        droop: 0.6,
        pedicel: 0.03,
        spread: 60.0,
        petals: 5.0,
        open: 75.0,
        petal_length: 0.016,
        petal_width: 0.014,
        petal_widest: 0.62,
        petal_cup: 0.35,
        scatter: 0.12,
        stamens: 20.0,
        stamen_length: 0.008,
        stamen_width: 0.0004,
        ..f.bloom
    };
    let m = &mut f.material;
    (m.bloom_front_red, m.bloom_front_green, m.bloom_front_blue) = (0.1, 0.097, 0.09);
    (m.bloom_back_red, m.bloom_back_green, m.bloom_back_blue) = (0.085, 0.082, 0.076);
    (m.bloom_eye_red, m.bloom_eye_green, m.bloom_eye_blue) = (0.07, 0.075, 0.015);
    m.bloom_eye_share = 0.22;
    f
}

/// fn-217's `dense`: every outer age flowering, twelve florets of 20 mm.
pub(super) fn dense() -> Family {
    let mut f = cherry();
    for k in 10..15 {
        f.space.species.as_mut().unwrap().states[k].flowering = 1.0;
    }
    (f.bloom.florets, f.bloom.petal_length, f.bloom.petal_width) = (12.0, 0.02, 0.018);
    f
}

/// fn-217's Laurelin test tree: the horn racemes (DESIGN.md 9.2) on the
/// group at scale 3 with the gold rim.
pub(super) fn laurelin() -> Family {
    let mut f = beech();
    f.bloom = BloomParams {
        florets: 24.0,
        rachis: 0.16,
        droop: 1.0,
        pedicel: 0.008,
        spread: 25.0,
        taper: 0.35,
        petals: 5.0,
        open: 20.0,
        petal_length: 0.012,
        petal_width: 0.008,
        petal_widest: 0.85,
        petal_cup: 0.6,
        petal_curl: 0.6,
        scatter: 0.1,
        ..f.bloom
    };
    let m = &mut f.material;
    (m.bloom_front_red, m.bloom_front_green, m.bloom_front_blue) = (0.13, 0.075, 0.008);
    (m.bloom_back_red, m.bloom_back_green, m.bloom_back_blue) = (0.11, 0.06, 0.006);
    f.supernatural = Supernatural {
        enabled: true,
        scale: 3.0,
        margin_reflectance: 0.9,
        roughness: 0.25,
        ..Supernatural::NONE
    };
    let s = &mut f.supernatural.material;
    (s.margin_red, s.margin_green, s.margin_blue) = (Some(0.6), Some(0.42), Some(0.03));
    f
}

pub(super) fn positions(i: &Instances) -> Vec<Vec3> {
    (0..i.len()).map(|k| i.position(k)).collect()
}

/// `xs`'s quantiles at `qs`, sorted in place.
pub(super) fn quantiles(xs: &mut [f64], qs: &[f64]) -> Vec<f64> {
    xs.sort_by(f64::total_cmp);
    let n = xs.len().max(1) - 1;
    qs.iter()
        .map(|q| xs.get((q * n as f64) as usize).copied().unwrap_or(f64::NAN))
        .collect()
}

/// Where each point stands in the crown: the shader's `depth_in_crown`
/// against the leaves' box, and metres under the shell, the 98th
/// percentile of the leaves' reach from the box's centre within the point's
/// bin of direction (24 by 12), or NaN in a bin of under 20 leaves.
pub(super) struct Crown {
    pub(super) centre: Vec3,
    pub(super) half: Vec3,
    shell: Vec<f64>,
}

const AZ: usize = 24;
const EL: usize = 12;

impl Crown {
    pub(super) fn of(leaves: &[Vec3]) -> Self {
        let (mut lo, mut hi) = (leaves[0], leaves[0]);
        for p in leaves {
            lo = Vec3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
            hi = Vec3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
        }
        let (centre, half) = ((lo + hi) * 0.5, (hi - lo) * 0.5);
        let mut bins = vec![Vec::new(); AZ * EL];
        for p in leaves {
            let d = *p - centre;
            bins[Self::bin(d)].push(d.length());
        }
        let shell = (bins.iter_mut())
            .map(|b| match b.len() {
                0..20 => f64::NAN,
                _ => quantiles(b, &[0.98])[0],
            })
            .collect();
        Self {
            centre,
            half,
            shell,
        }
    }

    fn bin(d: Vec3) -> usize {
        let az = (d.z.atan2(d.x) / std::f64::consts::TAU + 0.5) * AZ as f64;
        let el = ((d.y / d.length().max(1e-12)).clamp(-1.0, 1.0).asin() / std::f64::consts::PI
            + 0.5)
            * EL as f64;
        (az as usize).min(AZ - 1) + AZ * (el as usize).min(EL - 1)
    }

    /// The shader's depth: 1 at the centre, 0 on and past the box's ellipsoid.
    pub(super) fn ellipsoid(&self, p: Vec3) -> f64 {
        let o = p - self.centre;
        let q = Vec3::new(o.x / self.half.x, o.y / self.half.y, o.z / self.half.z);
        1.0 - q.length().clamp(0.0, 1.0)
    }

    /// Metres under the shell (negative: outside it).
    pub(super) fn under(&self, p: Vec3) -> f64 {
        let d = p - self.centre;
        self.shell[Self::bin(d)] - d.length()
    }
}
