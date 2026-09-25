//! The design principles, checked before a push and on a spec. Deterministic
//! guards block; the Jev reviewer advises. See `docs/principles.md`.

pub mod ask;
pub mod boundary;
pub mod budget;
pub mod candidates;
pub mod cli;
pub mod diff;
pub mod duplicate;
pub mod index;
pub mod modtree;
pub mod policy;
pub mod push;
pub mod replay;
pub mod resolve;
pub mod review;
pub mod run;
pub mod source;
pub mod spec;
pub mod switch;
pub mod unread;
