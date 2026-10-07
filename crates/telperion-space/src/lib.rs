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
mod allocation;
mod alone;
mod beech;
mod blend;
mod bounded;
mod chain;
mod clock;
mod closed_form;
mod cores;
mod couple;
mod dormant;
mod error;
mod fine;
pub mod fixed;
mod geometry;
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
mod transcendental;
mod upward;

pub use alone::{field_probe, grow_alone, grow_alone_with, Alone, ChainCost, FieldProbe};
pub use beech::beech;
pub use blend::blend;
pub use bounded::{grow_bounded, Bounded, Expected};
pub use chain::{passed, AGES};
pub use closed_form::{expected_counts, expected_kept};
pub use error::{Error, Result};
pub use fine::{
    bearer_frame, canonical, chain_cases, digest, grow_fine, identities, landed, lineage_order,
    realized_cases, root_axes, settle, survey_laid, survey_lit, BearerFrame, ChainAxis, ChainBud,
    ChainTables, Expanded, Fine, GeometryTables, Lived, Loaded, Order, PaGeometry, PaRow, Plan,
    Presences, RealizedCase, RootAxes, RootGeometry, RootSeed, Roots, Settled, Surveyed, UnitZone,
    Units, Woke, Work, ZoneGeometry,
};
pub use grow::couple::{grow_coupled, Split};
pub use grow::oracle::{grow_oracle, Oracled, Read};
pub use grow::partition::{
    grow_partitioned, Grouping, Order as PartitionOrder, Partitioned, Run as PartitionRun, PHASES,
};
pub use grow::stepped::{grow_stepped, Stepped};
pub use grow::subtree::Measured;
pub use grow::{grow, grow_marked, grow_staged, grow_whole, sketch, Pass, Request, Stage};
pub use light::{bud_light, field_reads, Light};
pub use oak::oak;
pub use palm::palm;
pub use presence::{FLOOR, RATE, SPAN};
pub use species::scale::{settings, zone_settings};
pub use species::{
    Form, NodeLaw, PaState, Species, Zone, BARE, MAX_BUDS, MEDIAL, SPARE, TOP, ZONES,
};
pub use spruce::spruce;
pub use structure::{Axis, CountTable, Origin, Phytomer, Structure, Vec3};
pub use transcendental::Transcendental;
