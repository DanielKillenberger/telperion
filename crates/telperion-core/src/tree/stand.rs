//! A fine root of a coarse tree as its expectation (fn-214): kept beside
//! the tree, as `clusters` are, and answered in the field by its expected
//! occupancy.
use super::mix32;
use crate::math::Vec3;
use telperion_space::lineage_keys::STAND;

/// The node it is borne on, its wood from `base` toward `tip` (twice its
/// expected wood's centre along its heading) at its base radius, the reach
/// about that line its expected wood spreads to (the centre's distance),
/// its expected kept phytomers, and the key its occupancy draws under.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Stand {
    pub node: u32,
    pub base: Vec3,
    pub tip: Vec3,
    pub radius: f64,
    pub reach: f64,
    pub phytomers: f64,
    pub key: u32,
}

/// A stand-in's draw key: its fine root's lineage under the registry's
/// `STAND`, so the cells it fills draw apart from every other stand's.
pub fn stand_key(lineage: u64) -> u32 {
    let low = mix32(lineage as u32 ^ STAND as u32);
    mix32(low ^ (lineage >> 32) as u32 ^ (STAND >> 32) as u32)
}
