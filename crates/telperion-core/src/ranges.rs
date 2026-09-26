//! Authoring bounds and defaults for generation budgets.
use crate::{Error, Result};

pub const DEFAULT_MAX_NODES: usize = 250_000;
pub const MAX_NODES: usize = u32::MAX as usize;
pub const DEFAULT_MAX_INTERNODES: u32 = 32;
pub const DEFAULT_WORK_BUDGET: u32 = 250_000;
pub const MAX_ATTRACTORS: usize = 1_000_000;
pub const fn default_clump_system_order() -> u32 {
    2
}
pub const fn default_clump_neighbours() -> u32 {
    12
}
pub const fn default_sampling_attempts_per_attractor() -> u32 {
    64
}
pub const fn default_reach_probe_steps() -> u32 {
    96
}
pub const fn default_work_budget() -> u32 {
    DEFAULT_WORK_BUDGET
}
pub const fn default_max_taper_exponent() -> f64 {
    12.0
}
pub const fn default_max_writhe_magnitude() -> f64 {
    0.9
}
pub const fn default_max_internodes() -> u32 {
    DEFAULT_MAX_INTERNODES
}
/// The rosette's own neutrals: a phyllotactic spiral, a frond that stands off
/// the axis, a crown that opens from spike to skirt, and one leaflet, which is
/// the single blade every family drew.
pub const fn default_rosette_divergence() -> f64 {
    137.508
}
pub const fn default_rosette_pitch() -> f64 {
    45.0
}
pub const fn default_rosette_pitch_spread() -> f64 {
    60.0
}
pub const fn default_leaflet_count() -> u32 {
    1
}
pub const fn default_leaflet_pitch() -> f64 {
    45.0
}
/// The trunk organs' own neutrals: a base a third of the stem's girth
/// standing well out of the bark, and a spine drawn at a third of the leaflet
/// it replaces, leaving the rachis far steeper than a blade does.
pub const fn default_leaf_base_radius() -> f64 {
    0.35
}
pub const fn default_leaf_base_pitch() -> f64 {
    60.0
}
pub const fn default_acanthophyll_length() -> f64 {
    0.35
}
pub const fn default_acanthophyll_pitch() -> f64 {
    80.0
}
/// The skirt's own neutrals: a dead frond hung well below level, dried to
/// four fifths of a living one's length. Both stand off their rail's end.
pub const fn default_skirt_pitch() -> f64 {
    140.0
}
pub const fn default_skirt_length() -> f64 {
    0.8
}
pub const fn default_max_droop() -> f64 {
    0.35
}
pub const fn default_curtain_step_clearance() -> f64 {
    0.8
}
pub const fn default_socket_containment() -> f64 {
    0.9
}

#[derive(Clone, Copy)]
pub struct Range(pub f64, pub f64);
impl Range {
    pub fn check(self, value: f64, field: &'static str) -> Result<()> {
        if !value.is_finite() || !(self.0..=self.1).contains(&value) {
            return Err(Error::InvalidValue {
                field,
                value: value.to_string(),
            });
        }
        Ok(())
    }
    /// The catalogue bounds this range states.
    pub const fn bounds(self) -> crate::catalogue::Bounds {
        crate::catalogue::Bounds::closed(self.0, self.1)
    }
}
/// Two rows' bounds, also the clamp a varied draw of each takes.
pub const LENGTH_RATIO: Range = Range(0.05, 1.);
pub const ANGLE: Range = Range(0., 90.);
pub const POSITIVE_COUNT: Range = Range(1., u32::MAX as f64);

pub fn max_nodes(value: usize) -> Result<()> {
    if value > MAX_NODES {
        return Err(Error::InvalidValue {
            field: "maxNodes",
            value: value.to_string(),
        });
    }
    Ok(())
}
