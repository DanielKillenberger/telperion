//! fn-256 task 2 (DESIGN.md sections 2.1, 2.2 and 7), test-only: one
//! captured cycle's units and wake replayed on the device, every append
//! placed by an exclusive scan in the CPU's order, held exactly to the
//! CPU's cycle and timed per kernel.
mod build;
mod build_grow;
mod build_light;
mod build_move;
mod build_plan;
mod census;
mod columns;
mod compare;
mod device;
mod gate;
mod given;
mod glue;
mod grown;
mod grown_compare;
mod hier_build;
mod hier_columns;
mod hier_compare;
mod hier_device;
mod hier_plan;
mod hier_scan;
mod lay_device;
mod light_device;
mod light_program;
mod plan;
mod plan_hier;
mod plan_retire;
mod plan_lay;
mod plan_scan;
mod plan_units;
mod retire_compact;
mod retire_device;
mod settle_device;
mod settle_place;
mod settle_read;
mod settle_run;
mod settle_scratch;
mod state;
mod vm;
#[cfg(test)]
mod grown_det_tests;
#[cfg(test)]
mod grown_life_tests;
#[cfg(test)]
mod grown_tests;
#[cfg(test)]
mod grown_time_tests;
#[cfg(test)]
mod hier_grid_tests;
#[cfg(test)]
mod hier_math_tests;
#[cfg(test)]
mod hier_tests;
#[cfg(test)]
mod lay_math_tests;
#[cfg(test)]
mod light_tests;
#[cfg(test)]
mod retire_tests;
#[cfg(test)]
mod settle_det_tests;
#[cfg(test)]
mod settle_math_tests;
#[cfg(test)]
mod settle_tests;
#[cfg(test)]
mod settle_time_tests;
#[cfg(test)]
mod executor_time_tests;
#[cfg(test)]
mod settle_float_tests;
#[cfg(test)]
mod soft64_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tint_rules;
#[cfg(test)]
mod vm_tests;

const ARENA: &str = include_str!("cycle/arena.wgsl");
const REAL: &str = include_str!("cycle/real.wgsl");
const FRAME: &str = include_str!("cycle/frame.wgsl");
const ADVANCE: &str = include_str!("cycle/advance.wgsl");
const STOP: &str = include_str!("cycle/stop.wgsl");
const GROW: &str = include_str!("cycle/grow.wgsl");
const UNITS: &str = include_str!("cycle/units.wgsl");
const WAKE: &str = include_str!("cycle/wake.wgsl");
const LAYOUT: &str = include_str!("cycle/layout.wgsl");
const LAY_STATE: &str = include_str!("cycle/lay_state.wgsl");
const Q: &str = include_str!("cycle/q.wgsl");
const SINCOS: &str = include_str!("cycle/sincos.wgsl");
const LAY_WALK: &str = include_str!("cycle/lay_walk.wgsl");
const LAY_DRAW: &str = include_str!("cycle/lay_draw.wgsl");
const LAY_STAND: &str = include_str!("cycle/lay_stand.wgsl");
const LAY_FRAME: &str = include_str!("cycle/lay_frame.wgsl");
const LAY_PENCIL: &str = include_str!("cycle/lay_pencil.wgsl");
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

/// How the marked arithmetic is built, chosen once with the kernels: the
/// plain operations (the default, COUNCIL.md's tolerance standard), or
/// every product the driver would fuse kept apart and every marked
/// quotient and root rounded as IEEE rounds them, so a test can hold the
/// device to the CPU word for word. The host has not ruled whether
/// production guards them (STEP2.md).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(super) enum Arith {
    #[default]
    Plain,
    Exact,
}

const ARITH: &str = include_str!("fine/arith.wgsl");
const EXACT: &str = include_str!("cycle/exact.wgsl");
const EXACT_SQRT: &str = include_str!("cycle/exact_sqrt.wgsl");
const PLAIN_SQRT: &str = "fn exact_sqrt(x: f32) -> f32 {\n    return sqrt(x);\n}\n";

impl Arith {
    /// The marked products and quotients (`exact_kept`, `exact_div`).
    fn marks(self) -> &'static str {
        match self {
            Arith::Plain => ARITH,
            Arith::Exact => EXACT,
        }
    }

    /// The marked root (`exact_sqrt`), which needs the wide words.
    fn root(self) -> &'static str {
        match self {
            Arith::Plain => PLAIN_SQRT,
            Arith::Exact => EXACT_SQRT,
        }
    }
}

/// The units' and the wake's WGSL: the contract and the zones
/// (`fine::zones`), then the cycle's state, its reals, an invocation's
/// frames, the advance, the stops, the unit, and the entries.
pub(super) fn source(arith: Arith) -> String {
    format!(
        "{}\n{ARENA}\n{REAL}\n{FRAME}\n{ADVANCE}\n{STOP}\n{GROW}\n{UNITS}\n{WAKE}",
        super::fine::zones_with(arith.marks())
    )
}

/// The rough layout's WGSL (DESIGN.md section 2.3): the contract, then
/// the layout's columns and entries, its f32 vectors and angles, the
/// walk, the draw, the stand, the frame and the pencil.
pub(super) fn layout_source(arith: Arith) -> String {
    format!(
        "{}\n{}\n{LAYOUT}\n{LAY_STATE}\n{Q}\n{SINCOS}\n{LAY_WALK}\n{LAY_DRAW}\n{LAY_STAND}\n{LAY_FRAME}\n{LAY_PENCIL}",
        super::fine::contract_with(arith.marks()),
        arith.root()
    )
}

/// The light's WGSL (DESIGN.md section 2, step 8): the software f64,
/// the programs and their registers, the loop that runs them, then the
/// light's bindings and entries.
pub(super) fn light_source(arith: Arith) -> String {
    format!(
        "{SOFT64}\n{}\n{LIGHT_VM}\n{LIGHT}\n{LIGHT_SWEEP}\n{}",
        light_program::wgsl(),
        arith.marks()
    )
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
pub(super) fn hier_cycle_source(arith: Arith) -> String {
    hier_cycle_source_at(hier_build::WIDTH, arith)
}

/// `hier_cycle_source` with the sweeps' workgroups `width` lanes wide.
pub(super) fn hier_cycle_source_at(width: u32, arith: Arith) -> String {
    let cols = HIER_COLS.replace("const WG: u32 = 64u;", &format!("const WG: u32 = {width}u;"));
    format!(
        "{}\n{REAL}\n{HIER_REAL}\n{cols}\n{HIER_CYCLE}\n{HIER_SWEEP}\n{HIER_SPLIT}",
        super::fine::contract_with(arith.marks())
    )
}

/// Every cycle module's WGSL under both arithmetics, by name, for the
/// shader checks.
pub(super) fn sources() -> Vec<(&'static str, String)> {
    let mut out = vec![
        ("cycle hierarchy build", hier_build_source()),
        ("cycle hierarchy scan", hier_scan_source()),
        ("cycle glue", glue::source()),
        ("cycle vm", vm::source()),
    ];
    for (arith, names) in [
        (Arith::Plain, ["cycle units and wake", "cycle layout", "cycle hierarchy sums", "cycle light"]),
        (
            Arith::Exact,
            [
                "cycle units and wake, exact",
                "cycle layout, exact",
                "cycle hierarchy sums, exact",
                "cycle light, exact",
            ],
        ),
    ] {
        let [units, layout, sums, light] = names;
        out.push((units, source(arith)));
        out.push((layout, layout_source(arith)));
        out.push((sums, hier_cycle_source(arith)));
        out.push((light, light_source(arith)));
    }
    out.push(("cycle settle", settle_device::source(Arith::Plain)));
    out.push(("cycle settle, exact", settle_device::source(Arith::Exact)));
    out.push(("cycle settle place", settle_device::place_source(Arith::Plain)));
    out.push(("cycle settle place, exact", settle_device::place_source(Arith::Exact)));
    out.push(("cycle retirement", retire_device::source(Arith::Plain)));
    out.push(("cycle retirement, exact", retire_device::source(Arith::Exact)));
    let [pack, units] = retire_device::compaction_sources();
    out.push(("cycle compaction", pack.to_string()));
    out.push(("cycle compaction units", units.to_string()));
    out
}
