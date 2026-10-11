//! fn-256 task 2 (DESIGN.md sections 2.1, 2.2 and 7), test-only: one
//! captured cycle's units and wake replayed on the device, every append
//! placed by an exclusive scan in the CPU's order, held exactly to the
//! CPU's cycle and timed per kernel.
mod columns;
mod compare;
mod device;
mod hier_build;
mod hier_columns;
mod hier_compare;
mod hier_device;
mod hier_plan;
mod hier_scan;
mod lay_columns;
mod lay_compare;
mod lay_device;
mod light_device;
mod light_program;
#[cfg(test)]
mod hier_grid_tests;
#[cfg(test)]
mod hier_math_tests;
#[cfg(test)]
mod hier_tests;
#[cfg(test)]
mod lay_math_tests;
#[cfg(test)]
mod lay_tests;
#[cfg(test)]
mod light_tests;
#[cfg(test)]
mod soft64_tests;
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
const HIER_COLS: &str = include_str!("cycle/hier_cols.wgsl");
const HIER_BUILD: &str = include_str!("cycle/hier_build.wgsl");
const HIER_BLOCKS: &str = include_str!("cycle/hier_blocks.wgsl");
const HIER_MEMBERS: &str = include_str!("cycle/hier_members.wgsl");
const HIER_SCAN: &str = include_str!("cycle/hier_scan.wgsl");
const HIER_REAL: &str = include_str!("cycle/hier_real.wgsl");
const HIER_CYCLE: &str = include_str!("cycle/hier_cycle.wgsl");
const HIER_SWEEP: &str = include_str!("cycle/hier_sweep.wgsl");
const HIER_SPLIT: &str = include_str!("cycle/hier_split.wgsl");
const SOFT64: &str = include_str!("cycle/soft64.wgsl");
const LIGHT_VM: &str = include_str!("cycle/light_vm.wgsl");
const LIGHT: &str = include_str!("cycle/light.wgsl");
const LIGHT_SWEEP: &str = include_str!("cycle/light_sweep.wgsl");

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

/// The light's WGSL (DESIGN.md section 2, step 8): the software f64,
/// the programs and their registers, the loop that runs them, then the
/// light's bindings and entries.
pub(super) fn light_source() -> String {
    format!("{SOFT64}\n{}\n{LIGHT_VM}\n{LIGHT}\n{LIGHT_SWEEP}", light_program::wgsl())
}

/// The hierarchy's build (DESIGN.md sections 3 and 4): its columns, the
/// children's lists, the blocks, then the members' and levels' lists.
pub(super) fn hier_build_source() -> String {
    format!("{HIER_COLS}\n{HIER_BUILD}\n{HIER_BLOCKS}\n{HIER_MEMBERS}")
}

/// The hierarchy's exclusive scan, in levels.
pub(super) fn hier_scan_source() -> String {
    HIER_SCAN.to_string()
}

/// The balance and allotment over the hierarchy (DESIGN.md section 2,
/// steps 1 and 2): the contract and its reals, the f64 products, the
/// columns, the per-apex entries, then the sweeps by block level.
pub(super) fn hier_cycle_source() -> String {
    hier_cycle_source_at(hier_build::WIDTH)
}

/// `hier_cycle_source` with the sweeps' workgroups `width` lanes wide.
pub(super) fn hier_cycle_source_at(width: u32) -> String {
    let cols = HIER_COLS.replace("const WG: u32 = 64u;", &format!("const WG: u32 = {width}u;"));
    format!(
        "{}\n{REAL}\n{HIER_REAL}\n{cols}\n{HIER_CYCLE}\n{HIER_SWEEP}\n{HIER_SPLIT}",
        super::fine::contract()
    )
}
