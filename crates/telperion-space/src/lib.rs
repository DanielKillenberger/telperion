//! The tree space's growth engine core (docs/tree-space.md, phase A).
//!
//! Buds carry a physiological age (PA), a state on the species' reference
//! axis that sets how they grow, branch, live and age. A tree grows one
//! growth unit of phytomers per living apex per cycle; each lateral bud's PA
//! is drawn by its zone of the growth unit; dead laterals are shed. The
//! structures are tested against GreenLab's closed-form counts and against
//! Letort's GreenLab simulator, run unchanged in a browser
//! (`scripts/greenlab-oracle.mjs`). Nothing here is wired into the pipeline.
//!
//! Phase B makes the space continuous (`lineage.rs`): every draw is keyed to
//! the bud's path from the root, and every element a draw makes grows in
//! from nothing as a setting passes the draw, so a walk of any setting at a
//! fixed seed changes the tree by degree.
pub mod agree;
mod allocation;
mod beech;
mod birch;
mod blend;
mod bounded;
mod chain;
mod clock;
mod closed_form;
mod cores;
pub mod contract;
mod couple;
mod dormant;
mod error;
mod fine;
mod geometry;
pub mod invariants;
mod girth;
mod grow;
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
mod upward;

pub use beech::beech;
pub use birch::birch;
pub use blend::blend;
pub use bounded::{grow_bounded, Bounded, Expected};
pub use chain::{passed, AGES};
pub use closed_form::{expected_axes, expected_counts, expected_grown, expected_kept, expected_length};
pub use error::{Error, Result};
pub use fine::{
    canonical, digest, grow_coarse, identities, lineage_order, ChainTables, Coarse, PaRow, Stand,
    UnitZone, Units,
};
pub use grow::couple::{grow_coupled, Split};
pub use grow::oracle::{grow_oracle, Oracled, Read};
pub use grow::partition::{
    grow_partitioned, Grouping, Order as PartitionOrder, Partitioned, Run as PartitionRun, PHASES,
};
pub use grow::subtree::Measured;
pub use grow::cycle::{
    grow_capture, grow_capture_grown, grow_capture_grown_at, grow_capture_hierarchy_at, grow_capture_tables, grow_capture_layout, grow_capture_layout_at, grow_capture_light_at, grow_capture_units,
    grow_capture_units_at, grow_census, grow_cycle, tree_digest, Capture, Census, Cycle, Cycled, Paused,
    Retiring, admitted, presented,
};
pub use grow::cycle::{grow_capture_settle_tables, grow_capture_settled, grow_capture_retired, grow_capture_settled_at, grow_cycle_is_the_reference, settle_sag_turn, SettleTo};
pub use grow::{grow, sketch, Pass, Request, Stage};
pub use light::{bud_light, field_reads, Light};
pub use lineage::{internode_factor, join_key};
pub use lineage::keys as lineage_keys;
pub use oak::oak;
pub use palm::palm;
pub use presence::{FLOOR, RATE, SPAN};
pub use species::scale::{settings, zone_settings};
pub use species::{
    Form, NodeLaw, PaState, Species, Zone, BARE, MAX_BUDS, MAX_LEAF_AREA, MEDIAL, SPARE, TOP, ZONES,
};
pub use spruce::spruce;
pub use structure::{Axis, CountTable, Join, Origin, Phytomer, Structure, Vec3};
pub use transcendental::Transcendental;
