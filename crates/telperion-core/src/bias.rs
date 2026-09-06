use crate::{
    envelope::Envelope,
    math::{smoothstep, Vec3},
    noise::Noise,
    rng::Rng,
    Error, Result,
};
use std::f64::consts::TAU;

pub const MIN_STEPS_PER_BEND: f64 = 8.0;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SupernaturalParams {
    pub enabled: bool,
    pub writhe_amplitude: f64,
    pub writhe_wavelength: f64,
    pub spiral_rate: f64,
}
impl Default for SupernaturalParams {
    fn default() -> Self {
        Self::NONE
    }
}
impl SupernaturalParams {
    pub const NONE: Self = Self {
        enabled: false,
        writhe_amplitude: 0.0,
        writhe_wavelength: 0.45,
        spiral_rate: 0.0,
    };
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BiasParams {
    pub gravitropism: f64,
    pub lean: f64,
    pub supernatural: SupernaturalParams,
}
impl Default for BiasParams {
    fn default() -> Self {
        Self {
            gravitropism: 0.7,
            lean: 0.05,
            supernatural: SupernaturalParams::NONE,
        }
    }
}
impl BiasParams {
    pub const NONE: Self = Self {
        gravitropism: 0.0,
        lean: 0.0,
        supernatural: SupernaturalParams::NONE,
    };
    pub fn validate(&self) -> Result<()> {
        if ![
            self.gravitropism,
            self.lean,
            self.supernatural.writhe_amplitude,
            self.supernatural.writhe_wavelength,
            self.supernatural.spiral_rate,
        ]
        .iter()
        .all(|v| v.is_finite() && *v >= 0.0)
        {
            return Err(Error::InvalidInput("growth bias"));
        }
        Ok(())
    }
}
#[derive(Clone)]
pub struct GrowthBias {
    envelope: Envelope,
    params: BiasParams,
    lean: Vec3,
    phase: f64,
    noise: Noise,
}
impl GrowthBias {
    pub fn new(envelope: Envelope, seed: u32, params: BiasParams) -> Result<Self> {
        envelope.validate()?;
        params.validate()?;
        let mut rng = Rng::new(seed ^ 0x5bf03d11);
        let bearing = rng.next_f64() * TAU;
        Ok(Self {
            envelope,
            params,
            lean: Vec3::new(bearing.cos(), 0.0, bearing.sin()),
            phase: rng.next_f64() * TAU,
            noise: Noise::new(seed ^ 0x1f83d9ab),
        })
    }
    /// Inputs are finite; direction is unit length and step is positive (validated by growth).
    pub fn apply(&self, position: Vec3, direction: Vec3, step: f64) -> Vec3 {
        let p = self.params;
        let effects = if p.supernatural.enabled {
            p.supernatural
        } else {
            SupernaturalParams::NONE
        };
        let height = self.envelope.height.max(1e-6);
        let wavelength = effects
            .writhe_wavelength
            .max(0.001)
            .max(MIN_STEPS_PER_BEND * step / height);
        let turns = effects
            .spiral_rate
            .min(height / (MIN_STEPS_PER_BEND * step));
        let stray_limit = effects.writhe_amplitude * height;
        let t = (position.y / height).clamp(0.0, 1.0);
        let mean = self.lean * (p.lean * position.y);
        let stray = Vec3::new(position.x - mean.x, 0.0, position.z - mean.z);
        let strayed = stray.length();
        let mut writhe = self.lean * p.lean;
        let spiral_gain = TAU * turns * effects.writhe_amplitude;
        if spiral_gain > 0.0 {
            let theta = TAU * turns * t + self.phase;
            let helix = Vec3::new(theta.cos(), 0.0, theta.sin());
            let swirl = if strayed > 1e-9 {
                let tangent = Vec3::new(-stray.z, 0.0, stray.x) / strayed;
                let swirl = helix.lerp(tangent, smoothstep(0.0, 0.5 * stray_limit, strayed));
                if swirl.length() > 1e-6 {
                    swirl.normalized()
                } else {
                    tangent
                }
            } else {
                helix
            };
            writhe += swirl * spiral_gain;
        }
        let curl_gain = TAU * effects.writhe_amplitude / wavelength;
        if curl_gain > 0.0 {
            let curl = self.noise.curl(position, wavelength * height);
            if curl.length() > 1e-9 {
                writhe += curl.normalized() * curl_gain;
            }
        }
        if stray_limit > 0.0 && strayed > 1e-9 {
            let base = height * self.envelope.crown_base;
            let trunkness = 1.0 - smoothstep(base, base * 1.3, position.y);
            let over = (strayed / stray_limit).min(1.0);
            writhe += stray * (-2.0 * over * over * trunkness / strayed);
        }
        if writhe.length() > 0.9 {
            writhe = writhe.normalized() * 0.9;
        }
        let biased = direction + writhe + Vec3::Y * (p.gravitropism * (0.55 + 0.45 * (1.0 - t)));
        if biased.length() < 1e-9 {
            direction.normalized()
        } else {
            biased.normalized()
        }
    }
}
