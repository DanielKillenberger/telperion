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
mod beech;
mod closed_form;
mod dormant;
mod error;
mod geometry;
mod girth;
mod grow;
mod light;
mod lineage;
mod oak;
mod palm;
mod presence;
mod sag;
mod shed;
mod species;
mod spruce;
mod structure;

pub use beech::beech;
pub use closed_form::expected_counts;
pub use error::{Error, Result};
pub use grow::{grow, grow_staged, sketch, Request, Stage};
pub use light::{bud_light, Light};
pub use oak::oak;
pub use palm::palm;
pub use presence::{FLOOR, RATE, SPAN};
pub use species::{Form, NodeLaw, PaState, Species, Zone, MAX_BUDS};
pub use spruce::spruce;
pub use structure::{Axis, CountTable, Origin, Phytomer, Structure, Vec3};
