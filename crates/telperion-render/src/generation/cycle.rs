//! fn-256 task 2 (DESIGN.md sections 2.1, 2.2 and 7), test-only: one
//! captured cycle's units and wake replayed on the device, every append
//! placed by an exclusive scan in the CPU's order, held exactly to the
//! CPU's cycle and timed per kernel.
mod columns;
mod compare;
mod device;
mod lay_columns;
mod lay_compare;
mod lay_device;
#[cfg(test)]
mod lay_math_tests;
#[cfg(test)]
mod lay_tests;
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
const LAYOUT: &str = include_str!("cycle/layout.wgsl");
const Q: &str = include_str!("cycle/q.wgsl");
const SINCOS: &str = include_str!("cycle/sincos.wgsl");
const LAY_WALK: &str = include_str!("cycle/lay_walk.wgsl");
const LAY_DRAW: &str = include_str!("cycle/lay_draw.wgsl");
const LAY_STAND: &str = include_str!("cycle/lay_stand.wgsl");
const LAY_FRAME: &str = include_str!("cycle/lay_frame.wgsl");

/// The units' and the wake's WGSL: the contract and the zones
/// (`fine::zones`), then the cycle's state, its reals, an invocation's
/// frames, the advance, the stops, the unit, and the entries.
pub(super) fn source() -> String {
    format!(
        "{}\n{ARENA}\n{REAL}\n{FRAME}\n{ADVANCE}\n{STOP}\n{GROW}\n{UNITS}\n{WAKE}",
        super::fine::zones()
    )
}

/// The rough layout's WGSL (DESIGN.md section 2.3): the contract, then
/// the layout's columns and entries, its f32 vectors and angles, the
/// walk, the draw, the stand and the frame.
pub(super) fn layout_source() -> String {
    format!(
        "{}\n{LAYOUT}\n{Q}\n{SINCOS}\n{LAY_WALK}\n{LAY_DRAW}\n{LAY_STAND}\n{LAY_FRAME}",
        super::fine::contract()
    )
}
