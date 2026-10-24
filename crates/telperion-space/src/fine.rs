//! What fn-209 F1's fine orders left behind them (fn-256 deleted the
//! fine roots grown below a cut): the canonical lineage order every tree
//! is returned in (`canonical.rs`); and the device's unit tables
//! (`tables.rs`, `surface.rs`).
mod canonical;
mod surface;
mod tables;
#[cfg(test)]
mod tests;

pub(crate) use canonical::lineage_permutation;
pub use canonical::{canonical, digest, identities, lineage_order};
pub use surface::{ChainTables, PaRow};
pub use tables::{UnitZone, Units};
