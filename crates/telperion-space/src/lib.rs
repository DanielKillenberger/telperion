//! The tree space's growth engine core (docs/tree-space.md, phase A).
//!
//! Buds carry a physiological age (PA), a state on the species' reference
//! axis that sets how they grow, branch, live and age. A tree grows one
//! growth unit of phytomers per living apex per cycle; each lateral bud's PA
//! is drawn by its zone of the growth unit; dead laterals are shed. The
//! structures are tested against GreenLab's closed-form counts and against
//! Letort's GreenLab simulator, run unchanged in a browser
//! (`scripts/greenlab-oracle.mjs`). The pipeline grows every tree through
//! `grow` (`telperion-core`'s `pipeline/space.rs`).
//!
//! Phase B makes the space continuous (`lineage.rs`): every draw is keyed to
//! the bud's path from the root, and every element a draw makes grows in
//! from nothing as a setting passes the draw, so a walk of any setting at a
//! fixed seed changes the tree by degree.
#[cfg(test)]
mod agree;
mod allocation;
mod beech;
mod birch;
mod blend;
mod chain;
mod clock;
mod closed_form;
pub mod contract;
mod cores;
mod dormant;
mod error;
mod fine;
mod geometry;
mod girth;
mod grow;
pub mod invariants;
mod light;
mod lineage;
mod memo;
mod oak;
mod palm;
mod presence;
mod sag;
mod schedule;
mod shed;
mod species;
mod spruce;
mod structure;
mod tolerance;
mod transcendental;
pub mod unfold;
mod upward;

pub use beech::beech;
pub use birch::birch;
pub use blend::blend;
pub use chain::{passed, AGES};
pub use closed_form::{
    expected_axes, expected_counts, expected_grown, expected_kept, expected_length,
};
pub use error::{Error, Result};
pub use fine::{canonical, digest, identities, lineage_order, ChainTables, PaRow, UnitZone, Units};
pub use grow::cycle::{
    admitted, grow_capture, grow_capture_grown, grow_capture_hierarchy_at, grow_capture_light_at,
    grow_capture_retired, grow_capture_settle_tables, grow_capture_settled_at, grow_capture_tables,
    grow_census, grow_cycle, grow_cycle_is_the_reference, tree_digest, within_year, Capture,
    Census, Cycle, Cycled, Paused, Retiring, SettleTo,
};
pub use grow::{grow, Pass, Request, Stage};
pub use light::Light;
pub use lineage::keys as lineage_keys;
pub use lineage::{
    cluster, drawn, internode_factor, join_key, lineage_key, mix32, organ, organs, placement_key,
    word,
};
pub use oak::oak;
pub use palm::palm;
pub use presence::{FLOOR, RATE, SPAN};
pub use schedule::most_units;
pub use species::scale::{scale, setting, settings, zone_settings, Scale, Setting, SETTINGS};
pub use species::{
    Form, NodeLaw, PaState, Species, Zone, BARE, MAX_BUDS, MAX_LEAF_AREA, MEDIAL, SPARE, TOP, ZONES,
};
pub use spruce::spruce;
pub use structure::{Axis, CountTable, Join, Origin, Phytomer, Structure, Vec3};
pub use transcendental::Transcendental;
