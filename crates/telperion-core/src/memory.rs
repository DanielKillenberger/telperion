//! Memory a build asks the machine for. A store whose size is known before
//! it fills is reserved at once, and a reserve the machine refuses is the
//! refusal by name (fn-369); the stage tags name a refusal the code cannot
//! return.
use crate::{Error, Result};

/// The stage tags the Wasm binding's allocator records with a refusal.
pub use telperion_space::stage;

/// The phytomers the last grown tree kept, read by the ceiling's
/// measurement build (`scripts/wasm-ceiling.mjs`) alone, even after a trap.
#[cfg(feature = "measure-ceiling")]
pub static KEPT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// `v` with room for `n` elements in all, reserved at once; the machine's
/// refusal is `store` named with the bytes it asked for.
pub(crate) fn reserve<T>(v: &mut Vec<T>, n: usize, store: &'static str) -> Result<()> {
    let more = n.saturating_sub(v.len());
    v.try_reserve_exact(more).map_err(|_| Error::Memory {
        store,
        needed: (n as u64).saturating_mul(size_of::<T>() as u64),
    })
}
