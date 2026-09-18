//! Shared Jev caller, ledger and evidence tools.
//!
//! This crate is never a dependency of `telperion-core`, `telperion-render`
//! or `telperion-wasm`. Workspace tests run with the key unset.

pub mod caller;
pub mod cases;
pub mod cite;
pub mod extract;
pub mod isolation;
pub mod ledger;
pub mod pipeline;
pub mod questions;
pub mod screen;
pub mod select;
pub mod triage;

pub use caller::{
    evaluate, load_key_from_env, CallerError, KeyError, Transport, ENDPOINT, INTERACTIVE_SHELL,
    MODEL,
};
pub use ledger::{LedgerEntry, SourceRef, Usage};
pub use questions::{citation_questions, screen_questions, thresholds, Thresholds};

/// SHA-256 of `bytes`, lowercase hex.
pub fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(64);
    for byte in digest {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}
