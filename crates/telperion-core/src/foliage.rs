//! Owned leaf elements and placements. No wood mesh is needed by this module.
mod element;
mod placement;
use crate::{
    envelope::{distance_to_profile, Envelope},
    math::Vec3,
    Error, Result,
};
pub use element::{build_element, Element, ElementParams};
pub use placement::{place, CanopyParams, TwigPlacement};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub min: Vec3,
    pub max: Vec3,
}
impl Bounds {
    pub fn contains(&self, p: Vec3) -> bool {
        p.x >= self.min.x
            && p.y >= self.min.y
            && p.z >= self.min.z
            && p.x <= self.max.x
            && p.y <= self.max.y
            && p.z <= self.max.z
    }
    fn include(&mut self, p: Vec3) {
        self.min = Vec3::new(
            self.min.x.min(p.x),
            self.min.y.min(p.y),
            self.min.z.min(p.z),
        );
        self.max = Vec3::new(
            self.max.x.max(p.x),
            self.max.y.max(p.y),
            self.max.z.max(p.z),
        );
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Instances {
    /// Column-major affine matrices: side, leaf axis (+Y), face (+Z), petiole.
    /// Packed directly for Three.js; arithmetic before storage uses f64.
    pub matrices: Vec<[f32; 16]>,
}
pub fn transform_point(m: &[f32; 16], p: Vec3) -> Vec3 {
    Vec3::new(
        m[0] as f64 * p.x + m[4] as f64 * p.y + m[8] as f64 * p.z + m[12] as f64,
        m[1] as f64 * p.x + m[5] as f64 * p.y + m[9] as f64 * p.z + m[13] as f64,
        m[2] as f64 * p.x + m[6] as f64 * p.y + m[10] as f64 * p.z + m[14] as f64,
    )
}
impl Instances {
    pub fn validate(&self) -> Result<()> {
        if self.matrices.iter().any(|m| {
            !m.iter().all(|v| v.is_finite())
                || m[3] != 0.
                || m[7] != 0.
                || m[11] != 0.
                || m[15] != 1.
        }) {
            return Err(Error::InvalidInput("foliage transform"));
        }
        Ok(())
    }
    /// Exact bounds of transformed leaf vertices; None for empty foliage.
    pub fn bounds(&self, element: &Element) -> Result<Option<Bounds>> {
        self.validate()?;
        element.validate()?;
        let mut bounds: Option<Bounds> = None;
        for m in &self.matrices {
            for v in &element.positions {
                let p = transform_point(m, *v);
                if !p.is_finite() || [p.x, p.y, p.z].iter().any(|v| !(*v as f32).is_finite()) {
                    return Err(Error::ResourceLimit("foliage bounds overflow"));
                }
                if let Some(b) = &mut bounds {
                    b.include(p)
                } else {
                    bounds = Some(Bounds { min: p, max: p })
                }
            }
        }
        Ok(bounds)
    }
}
/// Keeps a leaf unless every vertex lies deeper than the shell fraction.
pub fn cull(
    instances: &Instances,
    element: &Element,
    envelope: Envelope,
    shell_depth: f64,
) -> Result<Instances> {
    instances.validate()?;
    element.validate()?;
    envelope.validate()?;
    range(shell_depth, 0., 1., "shell depth")?;
    let shell = shell_depth * envelope.max_radius();
    let profile = envelope.profile();
    let mut out = Instances::default();
    out.matrices
        .try_reserve(instances.matrices.len())
        .map_err(|_| Error::ResourceLimit("foliage allocation"))?;
    let extent = element.positions.iter().fold(Vec3::ZERO, |a, p| {
        Vec3::new(a.x.max(p.x.abs()), a.y.max(p.y.abs()), a.z.max(p.z.abs()))
    });
    for m in &instances.matrices {
        for row in 0..3 {
            let bound = (m[row] as f64).abs() * extent.x
                + (m[row + 4] as f64).abs() * extent.y
                + (m[row + 8] as f64).abs() * extent.z
                + (m[row + 12] as f64).abs();
            if bound > f32::MAX as f64 {
                return Err(Error::ResourceLimit("foliage transform overflow"));
            }
        }
        let mut keep = element.positions.is_empty();
        for v in &element.positions {
            let p = transform_point(m, *v);
            if !p.is_finite() || [p.x, p.y, p.z].iter().any(|v| !(*v as f32).is_finite()) {
                return Err(Error::ResourceLimit("foliage transform overflow"));
            }
            let r = p.x.hypot(p.z);
            if envelope.radius_at(p.y) - r <= shell
                || distance_to_profile(&profile, r, p.y) <= shell
            {
                keep = true;
                break;
            }
        }
        if keep {
            out.matrices.push(*m)
        }
    }
    Ok(out)
}
fn range(v: f64, lo: f64, hi: f64, name: &'static str) -> Result<()> {
    if !v.is_finite() || v < lo || v > hi {
        Err(Error::InvalidInput(name))
    } else {
        Ok(())
    }
}
