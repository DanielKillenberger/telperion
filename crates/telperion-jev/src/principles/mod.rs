//! The design principles, checked before a push. Three deterministic guards
//! block: the production boundary, entry coverage and the artifact budgets.
//! Jev review of designs is planned in fn-156. See `docs/principles.md`.

pub mod boundary;
pub mod budget;
pub mod cli;
pub mod modtree;
pub mod policy;
pub mod push;
pub mod resolve;
pub mod source;
