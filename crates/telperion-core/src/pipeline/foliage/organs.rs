//! Counts of organs between two whole numbers (fn-277). A count walks by
//! degree: each bearer draws the whole count either side of its row
//! (`telperion_space::organs`), an apex its fronds and dead fronds, a
//! placement its leaflets and spines, so the expected count is the row's
//! and a walk between leaf forms changes the leaves by degree.
use super::{rosette::bearing, CanopyParams};
use telperion_space::{organ, organs};

/// The table the apex of the stem keyed `stem` draws its crown by: the
/// whole fronds and dead fronds its draws give it.
pub(crate) fn apex(p: &CanopyParams, stem: u64) -> CanopyParams {
    CanopyParams {
        rosette_fronds: f64::from(organs(p.rosette_fronds, stem, organ::FRONDS)),
        skirt_fronds: f64::from(organs(p.skirt_fronds, stem, organ::SKIRT)),
        ..*p
    }
}

/// The table one placement keyed `key` is drawn by: the whole leaflets and
/// spines its draws give it.
pub(super) fn placement(p: &CanopyParams, key: u64) -> CanopyParams {
    CanopyParams {
        leaflet_count: f64::from(organs(p.leaflet_count, key, organ::LEAFLETS)),
        acanthophylls: f64::from(organs(p.acanthophylls, key, organ::SPINES)),
        ..*p
    }
}

/// The key of a placement: what bears it and its ordinal there.
pub(super) fn placement_key(bearer: u64, ordinal: u64) -> u64 {
    bearer.rotate_left(32) ^ ordinal
}

/// Leaflets one placement of a whole table carries (`placement`): the
/// grouping is off until a table states both a count above one and a
/// rachis to string them along.
pub(super) fn leaflets(p: &CanopyParams) -> usize {
    if p.leaflet_count > 1. && p.rachis_length > 0. {
        p.leaflet_count as usize
    } else {
        1
    }
}

/// Whether every count of organs `p` states is whole, and its cauline
/// share all or none, never both with a crown: what the leaf plan and the
/// device draw (fn-277). Anything else the CPU reference places.
pub fn whole(p: &CanopyParams) -> bool {
    let counts = [p.rosette_fronds, p.skirt_fronds, p.leaflet_count, p.acanthophylls, p.leaf_bases];
    let shared = p.cauline == 0. || (p.cauline == 1. && !bearing(p));
    counts.iter().all(|c| c.fract() == 0.) && shared
}


/// The key a frond's placement draws its leaflets and spines under.
pub(super) fn frond_key(stem: u64, serial: i64) -> u64 {
    placement_key(stem, serial as u64)
}
