//! The tests' overrides of a cycle's limits on this thread: the layout's
//! generations forced (fn-298 R2 (d)), stepped limits forced short, and
//! stepped limits pinned at their bound from the first cycle (fn-302).
use super::build_plan::Bounds;
use super::gate::{Limits, Pinned};
use std::cell::Cell;

thread_local! {
    static LAYGENS: Cell<Option<u32>> = const { Cell::new(None) };
    /// The limits forced short, and whether past the build's pins.
    static SHORT: Cell<Option<(Pinned, bool)>> = const { Cell::new(None) };
    static PINNED: Cell<Pinned> = const { Cell::new(Pinned::NONE) };
}

/// `f` run with every cycle's layout generations forced to `n`: the
/// closing pass lays the deeper axes (fn-298 R2 (d)).
pub(in crate::generation) fn with_laygens<T>(n: u32, f: impl FnOnce() -> T) -> T {
    LAYGENS.set(Some(n));
    let out = f();
    LAYGENS.set(None);
    out
}

/// `f` run with the stepped limits `p` names forced to 1 in every cycle:
/// under the build's pins, or `through` them, so a retry passes them too.
pub(super) fn with_short<T>((p, through): (Pinned, bool), f: impl FnOnce() -> T) -> T {
    SHORT.set(Some((p, through)));
    let out = f();
    SHORT.set(None);
    out
}

/// `f` run with the stepped limits `p` names at their bound in every
/// cycle, from the first.
pub(super) fn with_pinned<T>(p: Pinned, f: impl FnOnce() -> T) -> T {
    PINNED.set(p);
    let out = f();
    PINNED.set(Pinned::NONE);
    out
}

/// The limits `p` names at 1.
fn short(l: Limits, p: Pinned) -> Limits {
    let one = Limits {
        units: 1,
        draws: 1,
        deposits: 1,
        sweeps: 1,
        rounds: 1,
        passes: 1,
        unit_rounds: [1; 2],
        ..l
    };
    l.taking(p, one)
}

/// The overrides the build's pins win over.
pub(super) fn under(l: Limits) -> Limits {
    let l = Limits {
        laygens: LAYGENS.get().unwrap_or(l.laygens),
        ..l
    };
    match SHORT.get() {
        Some((p, false)) => short(l, p),
        _ => l,
    }
}

/// The overrides that win over the build's pins.
pub(super) fn over(l: Limits, b: &Bounds) -> Limits {
    let l = l.pinned(PINNED.get(), b);
    match SHORT.get() {
        Some((p, true)) => short(l, p),
        _ => l,
    }
}
