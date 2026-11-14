//! What fn-209 F1's fine orders left behind them (fn-256 deleted the
//! fine roots grown below a cut): the canonical lineage order every tree
//! is returned in (`canonical.rs`).
mod canonical;
#[cfg(test)]
mod tests;

pub use canonical::{canonical, digest, identities, lineage_order};
