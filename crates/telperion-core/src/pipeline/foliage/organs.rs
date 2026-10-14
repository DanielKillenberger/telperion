//! Counts of organs between two whole numbers (fn-277). A count walks by
//! degree: each bearer draws the whole count either side of its row
//! (`telperion_space::organs`), an apex its fronds and dead fronds, a
//! placement its leaflets and spines, so the expected count is the row's
//! and a walk between leaf forms changes the leaves by degree.
use super::CanopyParams;
use telperion_space::{organ, organs, placement_key};

/// The table the apex of the stem keyed `stem` draws its crown by: the
/// whole fronds and dead fronds its draws give it.
pub(crate) fn apex(p: &CanopyParams, stem: u32) -> CanopyParams {
    CanopyParams {
        rosette_fronds: f64::from(organs(p.rosette_fronds, stem, organ::FRONDS)),
        skirt_fronds: f64::from(organs(p.skirt_fronds, stem, organ::SKIRT)),
        ..*p
    }
}

/// The table one placement keyed `key` is drawn by: the whole leaflets and
/// spines its draws give it.
pub(super) fn placement(p: &CanopyParams, key: u32) -> CanopyParams {
    CanopyParams {
        leaflet_count: f64::from(organs(p.leaflet_count, key, organ::LEAFLETS)),
        acanthophylls: f64::from(organs(p.acanthophylls, key, organ::SPINES)),
        ..*p
    }
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


/// The leaflets the frond keyed `key` carries: its own count of them
/// (`Crown::leaflets`) drawn on the apex's whole rows, so one drawn at no
/// size still stands them (fn-270 R5b, fn-275 R2b).
pub(super) fn frond_leaflets(p: &CanopyParams, count: f64, key: u32) -> usize {
    leaflets(&placement(&CanopyParams { leaflet_count: count, ..*p }, key))
}

/// The key a frond's placement draws its leaflets and spines under.
pub(super) fn frond_key(stem: u32, serial: i64) -> u32 {
    placement_key(stem, serial as u32)
}

/// The table placement `ordinal` of the bearer keyed `bearer` is drawn
/// by: a station of a run, a leaf of a cluster.
pub(super) fn placed(p: &CanopyParams, bearer: u32, ordinal: u32) -> CanopyParams {
    placement(p, placement_key(bearer, ordinal))
}
