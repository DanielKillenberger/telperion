use crate::math::Transcendental;
use crate::{
    envelope::Envelope,
    math::{smoothstep, Vec3},
    noise::Noise,
    rng::Rng,
    Error, Result,
};
use std::f64::consts::TAU;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct SupernaturalParams {
    pub enabled: bool,
    /// How far a branch may wander from a straight course, as a share of
    /// the tree's height. Raising it makes the wood wind and stray more. At
    /// zero the wood holds a straight course and the spiral rate does
    /// nothing, and any rise starts the wander.
    pub writhe_amplitude: f64,
    /// How long each of those wanders runs, as a share of the height.
    /// Raising it gives fewer, lazier bends; lowering it gives tighter kinks.
    pub writhe_wavelength: f64,
    /// How many full turns the wander winds around the trunk over the
    /// tree's height. Raising it tightens the spiral. At zero the wander
    /// winds around nothing, and any rise starts the spiral.
    pub spiral_rate: f64,
    /// The ceiling on how hard the wander may pull in any one step, so
    /// the other writhe rows cannot bend the wood arbitrarily.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_max_writhe_magnitude")
    )]
    pub max_writhe_magnitude: f64,
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
        max_writhe_magnitude: crate::ranges::default_max_writhe_magnitude(),
    };
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct BiasParams {
    /// How strongly growth is pulled upward, most near the ground.
    /// Raising it makes the tree grow more erect. At zero nothing pulls
    /// growth upright, and any rise starts that pull.
    pub gravitropism: f64,
    /// How far the whole tree leans off vertical, increasing with
    /// height. Raising it tips the trunk further in one direction. At zero
    /// the tree stands plumb, and any rise starts the lean.
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
        crate::ranges::MAX_WRITHE
            .check(self.supernatural.max_writhe_magnitude, "maxWritheMagnitude")?;
        if !self.supernatural.writhe_wavelength.is_finite()
            || self.supernatural.writhe_wavelength <= 0.0
        {
            return Err(Error::InvalidInput("writheWavelength"));
        }
        if !(TAU * self.supernatural.spiral_rate * self.supernatural.writhe_amplitude).is_finite()
            || !(TAU * self.supernatural.writhe_amplitude / self.supernatural.writhe_wavelength)
                .is_finite()
        {
            return Err(Error::InvalidInput("supernatural numeric range"));
        }
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
            lean: Vec3::new(bearing.cos_fixed(), 0.0, bearing.sin_fixed()),
            phase: rng.next_f64() * TAU,
            noise: Noise::new(seed ^ 0x1f83d9ab),
        })
    }
    /// Whether changing crown height can change a planned direction.
    pub(crate) fn height_independent(&self) -> bool {
        self.params.gravitropism == 0.0
            && (!self.params.supernatural.enabled
                || self.params.supernatural.writhe_amplitude == 0.0)
    }
    /// Inputs are finite; direction is unit length (validated by growth).
    pub fn apply(&self, position: Vec3, direction: Vec3) -> Vec3 {
        let p = self.params;
        let effects = if p.supernatural.enabled {
            p.supernatural
        } else {
            SupernaturalParams::NONE
        };
        let height = self.envelope.height.max(1e-6);
        let wavelength = effects.writhe_wavelength;
        let turns = effects.spiral_rate;
        let stray_limit = effects.writhe_amplitude * height;
        let t = (position.y / height).clamp(0.0, 1.0);
        let mean = self.lean * (p.lean * position.y);
        let stray = Vec3::new(position.x - mean.x, 0.0, position.z - mean.z);
        let strayed = stray.length();
        let mut writhe = self.lean * p.lean;
        let spiral_gain = TAU * turns * effects.writhe_amplitude;
        if spiral_gain > 0.0 {
            let theta = TAU * turns * t + self.phase;
            let helix = Vec3::new(theta.cos_fixed(), 0.0, theta.sin_fixed());
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
        if writhe.length() > p.supernatural.max_writhe_magnitude {
            writhe = writhe.normalized() * p.supernatural.max_writhe_magnitude;
        }
        let biased = direction + writhe + Vec3::Y * (p.gravitropism * (0.55 + 0.45 * (1.0 - t)));
        if biased.length() < 1e-9 {
            direction.normalized()
        } else {
            biased.normalized()
        }
    }
}
