use crate::{math::Vec3, rng::Rng, Error, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Envelope {
    pub height: f64,
    pub crown_base: f64,
    pub spread: f64,
    pub fullness: f64,
    pub shoulder: f64,
}
impl Default for Envelope {
    fn default() -> Self {
        Self {
            height: 24.0,
            crown_base: 0.3,
            spread: 0.3,
            fullness: 0.45,
            shoulder: 2.2,
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
        let fullness = self.fullness.clamp(0.001, 0.999);
        let shoulder = self.shoulder.max(0.1);
        let p = if t < fullness {
            1.0 - t / fullness
        } else {
            (t - fullness) / (1.0 - fullness)
        };
        self.max_radius() * (1.0 - p.powf(shoulder)).max(0.0).powf(1.0 / shoulder)
    }
    /// Includes the bare trunk axis within the tree's vertical extent.
    pub fn contains(&self, p: Vec3, tolerance: f64) -> bool {
        p.is_finite()
            && tolerance.is_finite()
            && tolerance >= 0.0
            && p.y >= -tolerance
            && p.y <= self.height + tolerance
            && p.x.hypot(p.z) <= self.radius_at(p.y) + tolerance
    }
    pub fn sample(&self, count: usize, rng: &mut Rng) -> Result<Vec<Vec3>> {
        self.validate()?;
        if count > 1_000_000 {
            return Err(Error::ResourceLimit("attractors"));
        }
        let r = self.max_radius();
        let base = self.height * self.crown_base;
        if r <= 0.0 || self.height <= base {
            return Ok(Vec::new());
        }
        let mut out = Vec::with_capacity(count);
        for _ in 0..count * 64 {
            if out.len() == count {
                return Ok(out);
            }
            let p = Vec3::new(
                rng.range(-r, r),
                rng.range(base, self.height),
                rng.range(-r, r),
            );
            if self.contains(p, 0.0) {
                out.push(p);
            }
        }
        if out.len() == count {
            Ok(out)
        } else {
            Err(Error::ResourceLimit("envelope sampling attempts"))
        }
    }
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
                exceptional = exceptional.min(x.hypot(y));
                f64::INFINITY
            }
        })
        .fold(f64::INFINITY, f64::min);
    squared.sqrt().min(exceptional)
}
