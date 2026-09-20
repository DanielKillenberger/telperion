use super::{reserved, Result, SurfaceParams};
use crate::math::Transcendental;

pub(super) struct Angular {
    pub angle: f64,
    pub cos: f64,
    pub sin: f64,
    untwisted: Option<f64>,
}
impl Angular {
    pub fn profile(&self, params: &SurfaceParams, phase: f64) -> f64 {
        self.untwisted
            .unwrap_or_else(|| profile(self.angle, phase, params))
    }
}
fn profile(angle: f64, phase: f64, params: &SurfaceParams) -> f64 {
    if params.lobes == 0 {
        1.0
    } else {
        1.0 + params.lobe_depth * (params.lobes as f64 * (angle + phase)).cos_fixed()
    }
}
pub(super) fn samples(segments: usize, params: &SurfaceParams) -> Result<Vec<Angular>> {
    let mut out = reserved(segments)?;
    for k in 0..segments {
        let angle = k as f64 / segments as f64 * std::f64::consts::TAU;
        out.push(Angular {
            angle,
            cos: angle.cos_fixed(),
            sin: angle.sin_fixed(),
            untwisted: (params.twist_rate == 0.0).then(|| profile(angle, 0.0, params)),
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec3;
    #[test]
    fn cached_rings_match_original_formula_bits() {
        for twist in [0.0, -0.0, 0.17, -0.23] {
            for lobes in [0, 3, 7] {
                let params = SurfaceParams {
                    twist_rate: twist,
                    lobes,
                    lobe_depth: 0.13,
                    ..Default::default()
                };
                let samples = samples(28, &params).unwrap();
                for d in [-0.0, 0.0, 0.25, 11.7] {
                    let phase = std::f64::consts::TAU * twist * (d / 12.0);
                    for (k, sample) in samples.iter().enumerate() {
                        let angle = k as f64 / 28.0 * std::f64::consts::TAU;
                        let old_profile = if lobes == 0 {
                            1.0
                        } else {
                            1.0 + params.lobe_depth * (lobes as f64 * (angle + phase)).cos_fixed()
                        };
                        let normal = Vec3::new(-0.0, 0.6, -0.8);
                        let binormal = Vec3::new(1.0, -0.0, 0.0);
                        let old = (normal * angle.cos_fixed() + binormal * angle.sin_fixed())
                            * (0.031 * old_profile);
                        let new = (normal * sample.cos + binormal * sample.sin)
                            * (0.031 * sample.profile(&params, phase));
                        assert_eq!(
                            [old.x.to_bits(), old.y.to_bits(), old.z.to_bits()],
                            [new.x.to_bits(), new.y.to_bits(), new.z.to_bits()]
                        );
                        assert_eq!(sample.angle.to_bits(), angle.to_bits());
                    }
                }
            }
        }
    }
}
