//! The bytes the build may still allocate, where its host can say.
//!
//! The Wasm binding installs a source that counts the live heap (fn-291
//! R2f), so freed memory counts as left. A native host installs none: no
//! limit there is known, and no budget is checked.
use std::sync::OnceLock;

static LEFT: OnceLock<fn() -> u64> = OnceLock::new();

/// Installs the source of the bytes left; the first source installed stays.
pub fn set_left(source: fn() -> u64) {
    let _ = LEFT.set(source);
}

/// The bytes left, where a source is installed.
pub(crate) fn left() -> Option<u64> {
    LEFT.get().map(|source| source())
}
