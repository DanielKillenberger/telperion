//! fn-256 task 2 (DESIGN.md sections 2.1, 2.2 and 7), test-only: one
//! captured cycle's units and wake replayed on the device, every append
//! placed by an exclusive scan in the CPU's order, held exactly to the
//! CPU's cycle and timed per kernel.
mod columns;
mod compare;
mod device;
#[cfg(test)]
mod tests;

const ARENA: &str = include_str!("cycle/arena.wgsl");
const REAL: &str = include_str!("cycle/real.wgsl");
const FRAME: &str = include_str!("cycle/frame.wgsl");
const ADVANCE: &str = include_str!("cycle/advance.wgsl");
const STOP: &str = include_str!("cycle/stop.wgsl");
const GROW: &str = include_str!("cycle/grow.wgsl");
const UNITS: &str = include_str!("cycle/units.wgsl");
const WAKE: &str = include_str!("cycle/wake.wgsl");

/// The units' and the wake's WGSL: the contract and the zones
/// (`fine::zones`), then the cycle's state, its reals, an invocation's
/// frames, the advance, the stops, the unit, and the entries.
pub(super) fn source() -> String {
    format!(
        "{}\n{ARENA}\n{REAL}\n{FRAME}\n{ADVANCE}\n{STOP}\n{GROW}\n{UNITS}\n{WAKE}",
        super::fine::zones()
    )
}
