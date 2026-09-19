//! Owned leaf elements and placements. No wood mesh is needed by this module.
use crate::math::Transcendental;
mod clumping;
mod element;
mod levels;
mod outline;
mod placement;
mod short_shoots;
mod station;
pub(crate) mod timeline;
use crate::{
    envelope::{distance_to_profile, Envelope},
    math::Vec3,
    Error, Result,
};
pub use element::{build_element, AnatomyGeometry, Element, ElementParams, FoliageUnit};
pub use levels::Level;
pub use placement::{place, place_on_surface, CanopyParams, TwigPlacement};
pub use short_shoots::{
    place_short_shoots, place_short_shoots_clumped, short_shoots, ShortShoot,
    MAX_SHORT_SHOOT_LEAVES, SHORT_SHOOT_SPACING,
};
pub use timeline::{Placement, PlacementIdentity};

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
///
/// Takes the crown it filters and hands the same allocation back with the
/// leaves that stay. A crown of seven million leaves is 470 MB, and a second
/// buffer at the input's own length doubled that whatever the cull dropped;
/// retaining in place leaves one copy resident. Capacity is not shrunk: the
/// vector keeps the block it was handed.
pub fn cull(
    mut instances: Instances,
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
    let extent = element.positions.iter().fold(Vec3::ZERO, |a, p| {
        Vec3::new(a.x.max(p.x.abs()), a.y.max(p.y.abs()), a.z.max(p.z.abs()))
    });
    // `retain` has no way to refuse, so an overflowing leaf is remembered and
    // the whole crown is dropped with the error below. Leaves after it keep
    // their places: the vector is discarded unread on that path.
    let mut overflow = false;
    instances.matrices.retain(|m| {
        if overflow {
            return true;
        }
        for row in 0..3 {
            let bound = (m[row] as f64).abs() * extent.x
                + (m[row + 4] as f64).abs() * extent.y
                + (m[row + 8] as f64).abs() * extent.z
                + (m[row + 12] as f64).abs();
            if bound > f32::MAX as f64 {
                overflow = true;
                return true;
            }
        }
        for v in &element.positions {
            let p = transform_point(m, *v);
            if !p.is_finite() || [p.x, p.y, p.z].iter().any(|v| !(*v as f32).is_finite()) {
                overflow = true;
                return true;
            }
            let r = p.x.hypot_fixed(p.z);
            if envelope.radius_at(p.y) - r <= shell
                || distance_to_profile(&profile, r, p.y) <= shell
            {
                return true;
            }
        }
        false
    });
    if overflow {
        return Err(Error::ResourceLimit("foliage transform overflow"));
    }
    Ok(instances)
}
fn range(v: f64, lo: f64, hi: f64, name: &'static str) -> Result<()> {
    if !v.is_finite() || v < lo || v > hi {
        Err(Error::InvalidInput(name))
    } else {
        Ok(())
    }
}
