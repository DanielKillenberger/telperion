//! One whole shipped tree in the test binary at a time (fn-274 R2). A test
//! that builds every shipped preset reaches 2 to 6 GB on Telperion alone;
//! side by side they took the gate past the 13 GB the desk has beside the
//! owner's desktop (fn-251).

use std::sync::{Mutex, MutexGuard};

/// The binary's one whole-tree turn, held until dropped. A test that
/// panicked in its turn passes it on.
pub(crate) fn turn() -> MutexGuard<'static, ()> {
    static ONE: Mutex<()> = Mutex::new(());
    ONE.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}
