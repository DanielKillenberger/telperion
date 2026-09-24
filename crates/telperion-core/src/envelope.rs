use crate::math::Transcendental;
use crate::{math::Vec3, noise, rng::Rng, Error, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct Envelope {
    /// The tree's height in metres. Everything else in the crown is
    /// measured against it: the crown base sits at `crown_base` of it and
    /// the widest radius is `spread` times it.
    pub height: f64,
    /// Where the crown starts, as a share of the height: below it the
    /// crown has no radius at all. Raising it lifts the crown and leaves a
    /// longer bare trunk.
    pub crown_base: f64,
    /// The crown's widest radius as a share of the height, so raising it
    /// widens the crown without making the tree taller.
    pub spread: f64,
    /// Where the crown is widest, as a share of the way from the crown
    /// base to the top. Raising it carries the widest part higher, so the
    /// crown reads top-heavy.
    pub fullness: f64,
    /// How square the crown's outline is. Raising it holds the crown near
    /// its full width further toward the top and the base, so the profile
    /// reads boxier; lowering it tapers the outline to a point.
    pub shoulder: f64,
    /// How far the outline departs from the smooth shell, as a fraction of the
    /// radius there. 0 is the axisymmetric superellipse every tree was before,
    /// and every shipped table that leaves it there is untouched.
    pub irregularity: f64,
    /// The wavelength of that departure over the shell's own surface, as a
    /// fraction of the tree's height: small is many small lumps, 1 is a lobe
    /// as long as the tree is tall.
    pub lobe_scale: f64,
}
impl Default for Envelope {
    fn default() -> Self {
        Self {
            height: 24.0,
            crown_base: 0.3,
            spread: 0.3,
            fullness: 0.45,
            shoulder: 2.2,
            irregularity: 0.0,
            lobe_scale: 0.5,
        }
    }
}
impl Envelope {
    pub fn validate(&self) -> Result<()> {
        let values = [
            self.height,
            self.crown_base,
            self.spread,
            self.fullness,
            self.shoulder,
        ];
        if !values.iter().all(|v| v.is_finite())
            || self.height < 0.0
            || self.spread < 0.0
            || !(0.0..=1.0).contains(&self.crown_base)
            || !(0.0..=1.0).contains(&self.fullness)
            || self.shoulder <= 0.0
            || !(self.height * self.spread).is_finite()
        {
            return Err(Error::InvalidInput("envelope"));
        }
        // The outline's two rows are refused rather than clamped, and by name:
        // a table that asks for an amplitude or a wavelength off its rail is a
        // table with a mistake in it.
        for (value, low, high, row) in [
            (self.irregularity, 0.0, 0.5, "envelope irregularity"),
            (self.lobe_scale, 0.05, 1.0, "envelope lobe scale"),
        ] {
            if !value.is_finite() || !(low..=high).contains(&value) {
                return Err(Error::InvalidInput(row));
            }
        }
        Ok(())
    }
    pub fn max_radius(&self) -> f64 {
        self.height * self.spread
    }
    pub fn radius_at(&self, y: f64) -> f64 {
        let base = self.height * self.crown_base;
        let span = self.height - base;
        if span <= 0.0 {
            return 0.0;
        }
        let t = (y - base) / span;
        if t <= 0.0 || t >= 1.0 {
            return 0.0;
        }
        let fullness = self.fullness;
        let shoulder = self.shoulder;
        let p = if t < fullness {
            1.0 - t / fullness
        } else {
            (t - fullness) / (1.0 - fullness)
        };
        self.max_radius() * quadrant(p, shoulder)
    }
    /// The shell's radius at a height and a bearing: the smooth radius shaped
    /// by the seed's own lobes. The perturbation is multiplicative, so the
    /// crown base and the bole are as authored and the radius stays within
    /// `radius_at` times one plus or minus the amplitude. At amplitude zero
    /// this is `radius_at` to the byte, whatever the seed.
    pub fn radius_at_bearing(&self, y: f64, azimuth: f64, seed: u32) -> f64 {
        self.lobed(y, azimuth.cos_fixed(), azimuth.sin_fixed(), seed)
    }
    /// `radius_at_bearing` for a point that already knows its own bearing, as
    /// every containment query does: the horizontal direction is the cosine
    /// and sine, and no angle is formed to take them back apart.
    pub fn radius_toward(&self, p: Vec3, seed: u32) -> f64 {
        // An outline without lobes has no bearing to find.
        if self.irregularity == 0.0 {
            return self.radius_at(p.y);
        }
        let radial = p.x.hypot_fixed(p.z);
        if radial <= 0.0 {
            return self.lobed(p.y, 1.0, 0.0, seed);
        }
        self.lobed(p.y, p.x / radial, p.z / radial, seed)
    }
    /// One wavelength of the noise spans `lobe_scale` of the height, up the
    /// shell and around it alike, so the outline is the same shape whatever
    /// size the tree is and the lobes close exactly on themselves at every
    /// full turn.
    fn lobed(&self, y: f64, cos: f64, sin: f64, seed: u32) -> f64 {
        let radius = self.radius_at(y);
        if self.irregularity == 0.0 || radius <= 0.0 {
            return radius;
        }
        let wavelength = (self.lobe_scale * self.height).max(1e-6);
        let around = self.max_radius() / wavelength;
        let at = Vec3::new(cos * around, y / wavelength, sin * around);
        radius * (1.0 + self.irregularity * noise::seeded(seed, at).clamp(-1.0, 1.0))
    }
    /// Includes the bare trunk axis within the tree's vertical extent. The
    /// seed is the family's: it is what the outline's lobes are keyed by, so
    /// two seeds of one family fill two different shells.
    pub fn contains(&self, p: Vec3, tolerance: f64, seed: u32) -> bool {
        p.is_finite()
            && tolerance.is_finite()
            && tolerance >= 0.0
            && p.y >= -tolerance
            && p.y <= self.height + tolerance
            && p.x.hypot_fixed(p.z) <= self.radius_toward(p, seed) + tolerance
    }
    pub fn sample(&self, count: usize, rng: &mut Rng, seed: u32) -> Result<Vec<Vec3>> {
        self.sample_with_attempts(
            count,
            rng,
            seed,
            crate::ranges::default_sampling_attempts_per_attractor(),
        )
    }
    pub fn sample_with_attempts(
        &self,
        count: usize,
        rng: &mut Rng,
        seed: u32,
        attempts_per_attractor: u32,
    ) -> Result<Vec<Vec3>> {
        self.validate()?;
        crate::ranges::POSITIVE_COUNT.check(
            attempts_per_attractor as f64,
            "samplingAttemptsPerAttractor",
        )?;
        if count > crate::ranges::MAX_ATTRACTORS {
            return Err(Error::InvalidInput("attractors"));
        }
        let r = self.max_radius();
        let base = self.height * self.crown_base;
        if r <= 0.0 || self.height <= base {
            return Ok(Vec::new());
        }
        let attempts =
            count
                .checked_mul(attempts_per_attractor as usize)
                .ok_or(Error::ResourceLimit(
                    "samplingAttemptsPerAttractor overflow",
                ))?;
        let mut out = Vec::new();
        out.try_reserve_exact(count)
            .map_err(|_| Error::ResourceLimit("attractor allocation"))?;
        for _ in 0..attempts {
            if out.len() == count {
                return Ok(out);
            }
            let p = Vec3::new(
                rng.range(-r, r),
                rng.range(base, self.height),
                rng.range(-r, r),
            );
            if self.contains(p, 0.0, seed) {
                out.push(p);
            }
        }
        if out.len() == count {
            Ok(out)
        } else {
            Err(Error::ResourceLimit("samplingAttemptsPerAttractor"))
        }
    }
    /// The smooth two-dimensional outline, and deliberately smooth: shedding
    /// and the crown index read one polyline for depth and exposure, and the
    /// irregularity says where growth may go, not how retention is judged. A
    /// bearing-aware retention is a spec of its own if the pairs ask for one.
    pub fn profile(&self) -> Vec<[f64; 2]> {
        let base = self.height * self.crown_base;
        (0..=128)
            .map(|i| {
                let y = base + (self.height - base) * i as f64 / 128.0;
                [self.radius_at(y), y]
            })
            .collect()
    }
    pub fn distance_to_profile(&self, r: f64, y: f64) -> f64 {
        distance_to_profile(&self.profile(), r, y)
    }
}
/// One quarter of the crown's outline, `(1 - p^shoulder)^(1 / shoulder)`: the
/// share of the widest radius left at `p` of the way from the widest height
/// to either end. It is written as logarithms and exponentials because
/// `libm`'s `pow` rounds to the last bit and costs about twice as much, and
/// `expm1` keeps `1 - p^shoulder` exact where it vanishes at the ends, which
/// the `pow` form cannot. A shoulder of 1 is the straight line it always was.
/// NaN and anything outside the quadrant read as no width.
pub(crate) fn quadrant(p: f64, shoulder: f64) -> f64 {
    if shoulder == 1.0 {
        return (1.0 - p).max(0.0);
    }
    let rest = -(shoulder * p.ln_fixed()).exp_m1_fixed();
    if !(rest > 0.0) {
        return 0.0;
    }
    (rest.ln_fixed() / shoulder).exp_fixed()
}
pub fn distance_to_profile(profile: &[[f64; 2]], r: f64, y: f64) -> f64 {
    let mut exceptional = f64::INFINITY;
    let squared = profile
        .windows(2)
        .map(|pair| {
            let [ar, ay] = pair[0];
            let [br, by] = pair[1];
            let dr = br - ar;
            let dy = by - ay;
            let length = dr * dr + dy * dy;
            let t = if length > 0.0 {
                ((r - ar) * dr + (y - ay) * dy) / length
            } else {
                0.0
            };
            let t = t.clamp(0.0, 1.0);
            let x = r - ar - dr * t;
            let y = y - ay - dy * t;
            let squared = x * x + y * y;
            if squared.is_normal() || (x == 0.0 && y == 0.0) {
                squared
            } else {
                exceptional = exceptional.min(x.hypot_fixed(y));
                f64::INFINITY
            }
        })
        .fold(f64::INFINITY, f64::min);
    squared.sqrt().min(exceptional)
}
