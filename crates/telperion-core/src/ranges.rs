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
}
pub const ANATOMY: Range = Range(1e-6, 1e6);
pub const STATIONS: Range = Range(1., 32.);
pub const LENGTH_RATIO: Range = Range(0.05, 1.);
pub const RATIO_POWER: Range = Range(0., 8.);
pub const INTERNODE_FACTOR: Range = Range(0.05, 32.);
pub const LATERALS: Range = Range(0., 7.);
pub const UNIT: Range = Range(0., 1.);
pub const REACH: Range = Range(0., 0.9);
pub const ANGLE: Range = Range(0., 90.);
pub const VIGOUR: Range = Range(0., 0.95);
pub const POSITIVE_COUNT: Range = Range(1., u32::MAX as f64);
pub const MAX_DROOP: Range = Range(0., 10.);
pub const MAX_WRITHE: Range = Range(0., 8.);
// 64 * forkExponent's upper bound 8 + ln(u32::MAX) < ln(f64::MAX).
pub const MAX_TAPER: Range = Range(0., 64.);
pub const TRUNK_RADIUS: Range = Range(4e-6, f64::MAX);
pub const FORK_EXPONENT: Range = Range(1., 8.);
pub const LENGTH_TAPER: Range = Range(0., f64::MAX);
pub const SURFACE_RADIAL_SEGMENTS: Range = Range(3., 64.);
pub const SURFACE_LOBES: Range = Range(0., 16.);
pub const SURFACE_LOBE_DEPTH: Range = Range(0., 0.9);
pub const SURFACE_TWIST_RATE: Range = Range(-64., 64.);
pub const SURFACE_FLARE_RADIUS: Range = Range(1., 8.);
pub const SURFACE_FLARE_FALLOFF: Range = Range(1e-4, 1.);
pub const SURFACE_FORK_SOCKET: Range = Range(0., 0.9);
pub const SURFACE_FORK_SWELL: Range = Range(1., 4.);

pub fn max_nodes(value: usize) -> Result<()> {
    if value > MAX_NODES {
        return Err(Error::InvalidValue {
            field: "maxNodes",
            value: value.to_string(),
        });
    }
    Ok(())
}
