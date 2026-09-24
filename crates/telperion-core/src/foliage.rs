//! Owned leaf elements and placements. No wood mesh is needed by this module.
use crate::math::Transcendental;
mod canopy;
mod clumping;
mod element;
mod levels;
mod outline;
pub(crate) mod packed;
#[cfg(feature = "geometry")]
mod placement;
pub mod plan;
#[cfg(feature = "geometry")]
pub mod prepared;
mod reference;
mod rosette;
#[cfg(feature = "geometry")]
mod short_shoots;
#[cfg(feature = "geometry")]
mod station;
pub(crate) mod timeline;
use crate::{
    envelope::{distance_to_profile, Envelope},
    math::Vec3,
    Error, Result,
};
pub use canopy::{CanopyParams, TwigPlacement, MAX_SHORT_SHOOT_LEAVES, SHORT_SHOOT_SPACING};
pub use element::{build_element, AnatomyGeometry, Element, ElementParams, FoliageUnit};
pub use levels::Level;
pub use packed::{Leaf, Reference, WORDS};
#[cfg(feature = "geometry")]
pub(crate) use placement::{leaf_count, place_on};
#[cfg(feature = "geometry")]
pub use placement::{place, place_on_surface};
#[cfg(feature = "geometry")]
pub use rosette::place_rosette;
/// The canopy's own rails, for a pass that reads the canopy rows without
/// placing a leaf: a bad row is refused by the name `place` refuses it by.
pub(crate) use rosette::validate as validate_canopy;
pub use rosette::{frame, rosettes, Rosette, MAX_FRONDS, MAX_LEAFLETS};
#[cfg(feature = "geometry")]
pub use short_shoots::{place_short_shoots, place_short_shoots_clumped, short_shoots, ShortShoot};
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
    fn transformed(self, m: &[f32; 16]) -> Self {
        let project = |row: usize, lower: bool| {
            let endpoint = |coefficient: f32, min: f64, max: f64| {
                if (coefficient >= 0.) == lower {
                    min
                } else {
                    max
                }
            };
            m[row] as f64 * endpoint(m[row], self.min.x, self.max.x)
                + m[row + 4] as f64 * endpoint(m[row + 4], self.min.y, self.max.y)
                + m[row + 8] as f64 * endpoint(m[row + 8], self.min.z, self.max.z)
                + m[row + 12] as f64
        };
        Self {
            min: Vec3::new(project(0, true), project(1, true), project(2, true)),
            max: Vec3::new(project(0, false), project(1, false), project(2, false)),
        }
    }
    fn strictly_contains(self, other: Self) -> bool {
        other.min.x > self.min.x
            && other.min.y > self.min.y
            && other.min.z > self.min.z
            && other.max.x < self.max.x
            && other.max.y < self.max.y
            && other.max.z < self.max.z
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
/// The crown's leaves, three words each, and the box their positions are
/// quantised against. Twelve bytes a leaf on the CPU and twelve in the GPU
/// storage buffer: the buffer is the bytes written here.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Instances {
    pub leaves: Vec<Leaf>,
    pub reference: Reference,
    /// Leaves the limb clumping dropped after placement sized the crown. The
    /// placed count plus this is what the station walk produced, which is the
    /// number a prediction is held to; the length alone is what survived.
    pub thinned: usize,
    /// Every transform handed to `push`, kept only in test builds so a round
    /// trip can be measured against what the constructor actually produced
    /// rather than against a constructed case (R2).
    #[cfg(test)]
    pub(crate) unquantised: Vec<[f32; 16]>,
}
/// The point a column-major affine transform carries `p` to. Arithmetic in
/// f64, as everything before storage is.
pub fn transform_point(m: &[f32; 16], p: Vec3) -> Vec3 {
    Vec3::new(
        m[0] as f64 * p.x + m[4] as f64 * p.y + m[8] as f64 * p.z + m[12] as f64,
        m[1] as f64 * p.x + m[5] as f64 * p.y + m[9] as f64 * p.z + m[13] as f64,
        m[2] as f64 * p.x + m[6] as f64 * p.y + m[10] as f64 * p.z + m[14] as f64,
    )
}
impl Instances {
    /// An empty crown quantised against this box.
    pub fn new(reference: Reference) -> Self {
        Self {
            leaves: Vec::new(),
            reference,
            thinned: 0,
            #[cfg(test)]
            unquantised: Vec::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.leaves.len()
    }
    pub fn is_empty(&self) -> bool {
        self.leaves.is_empty()
    }
    /// Stores one transform. The three words are what the crown keeps; the
    /// sixteen floats are a stack temporary the constructor hands over.
    pub fn push(&mut self, m: &[f32; 16]) {
        let leaf = self.reference.pack(m);
        self.leaves.push(leaf);
        #[cfg(test)]
        self.unquantised.push(*m);
    }
    /// What the station walk produced, before the limb clumping thinned it.
    pub fn placed(&self) -> usize {
        self.leaves.len() + self.thinned
    }
    /// The transform one stored leaf stands for.
    pub fn matrix(&self, index: usize) -> [f32; 16] {
        self.reference.unpack(self.leaves[index])
    }
    /// Every stored leaf's transform, rebuilt one at a time. A reader that
    /// wants only where a leaf stands takes `position` and pays for no
    /// rotation.
    pub fn matrices(&self) -> impl Iterator<Item = [f32; 16]> + '_ {
        self.leaves.iter().map(|&leaf| self.reference.unpack(leaf))
    }
    /// Where one stored leaf stands.
    pub fn position(&self, index: usize) -> Vec3 {
        self.reference.position(self.leaves[index])
    }
    /// The box has to be a box: a crown quantised against a reversed or
    /// unmeasurable one decodes to nothing anyone can draw.
    pub fn validate(&self) -> Result<()> {
        if !self.reference.is_finite() {
            return Err(Error::InvalidInput("foliage reference box"));
        }
        Ok(())
    }
    /// Exact bounds of transformed leaf vertices; None for empty foliage.
    pub fn bounds(&self, element: &Element) -> Result<Option<Bounds>> {
        self.validate()?;
        element.validate()?;
        let Some(&first) = element.positions.first() else {
            return Ok(None);
        };
        let mut local = Bounds {
            min: first,
            max: first,
        };
        for &p in &element.positions[1..] {
            local.include(p);
        }
        let mut bounds: Option<Bounds> = None;
        for m in self.matrices() {
            if let Some(exact) = bounds {
                // Signed endpoints and transform_point's sum order enclose every
                // vertex. Strict interior containment preserves extrema, including
                // signed-zero ties; unsafe enclosures use the checked vertex loop.
                let enclosure = local.transformed(&m);
                if [
                    enclosure.min.x,
                    enclosure.min.y,
                    enclosure.min.z,
                    enclosure.max.x,
                    enclosure.max.y,
                    enclosure.max.z,
                ]
                .iter()
                .all(|v| v.is_finite() && (*v as f32).is_finite())
                    && exact.strictly_contains(enclosure)
                {
                    continue;
                }
            }
            for v in &element.positions {
                let p = transform_point(&m, *v);
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
    let reference = instances.reference;
    instances.leaves.retain(|&leaf| {
        if overflow {
            return true;
        }
        let m = reference.unpack(leaf);
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
            let p = transform_point(&m, *v);
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

#[cfg(test)]
mod bounds_tests;
