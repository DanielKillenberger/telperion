//! fn-256 (DESIGN.md): the whole grower on the device, every append
//! placed by an exclusive scan in the CPU's order, built plain and held
//! to the CPU's to the ruled standard (`ruled.rs`); then the settled tree
//! drawn as the pipeline's nodes (`tree.rs`), which the resident build
//! draws from. The decision contract every cycle shader begins with is
//! here: `telperion_space::contract` by the same f32 operations.
mod build;
mod build_arena;
mod build_grow;
mod build_light;
mod build_move;
mod build_plan;
mod build_size;
mod census;
mod chains;
mod columns;
#[cfg(test)]
mod compare;
mod device;
mod drawn;
mod gate;
mod held;
mod given;
mod glue;
#[cfg(test)]
mod grown;
#[cfg(test)]
mod grown_compare;
mod hier_build;
mod hier_columns;
#[cfg(test)]
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
mod compact;
mod retire_compact;
mod retire_device;
#[cfg(test)]
mod ruled;
mod settle_device;
mod settle_place;
#[cfg(test)]
mod settle_read;
mod settle_run;
mod settle_scratch;
mod state;
pub(super) mod tree;
mod units;
mod vm;
#[cfg(test)]
mod compile_tests;
#[cfg(test)]
mod contract_tests;
#[cfg(test)]
mod mirror_tests;
#[cfg(test)]
mod selectors;
#[cfg(test)]
mod grown_det_tests;
#[cfg(test)]
mod arena_tests;
#[cfg(test)]
mod grown_life_tests;
#[cfg(test)]
mod halt_tests;
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
mod light_tests;
#[cfg(test)]
mod retire_tests;
#[cfg(test)]
mod settle_det_tests;
#[cfg(test)]
mod settle_tests;
#[cfg(test)]
mod settle_time_tests;
#[cfg(test)]
mod executor_time_tests;
#[cfg(test)]
mod roofline_life;
#[cfg(test)]
mod roofline_tests;
#[cfg(test)]
mod settle_float_tests;
#[cfg(test)]
mod soft64_tests;
#[cfg(test)]
mod tint_rules;
#[cfg(test)]
mod vm_tests;

const ARENA: &str = include_str!("cycle/arena.wgsl");
const REAL: &str = include_str!("cycle/real.wgsl");
const FRAME: &str = include_str!("cycle/frame.wgsl");
const JOB: &str = include_str!("cycle/job.wgsl");
const ADVANCE: &str = include_str!("cycle/advance.wgsl");
const STOP: &str = include_str!("cycle/stop.wgsl");
const GROW: &str = include_str!("cycle/grow.wgsl");
const UNITS: &str = include_str!("cycle/units.wgsl");
const WAKE: &str = include_str!("cycle/wake.wgsl");
const LAYOUT: &str = include_str!("cycle/layout.wgsl");
const LAY_STATE: &str = include_str!("cycle/lay_state.wgsl");
const Q: &str = include_str!("cycle/q.wgsl");
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
/// A sweep's narrow block levels in one workgroup (fn-256 task 6).
pub(super) const FOLD: &str = include_str!("cycle/fold.wgsl");
const SOFT64: &str = include_str!("cycle/soft64.wgsl");
const LIGHT_VM: &str = include_str!("cycle/light_vm.wgsl");
const LIGHT: &str = include_str!("cycle/light.wgsl");
const LIGHT_SWEEP: &str = include_str!("cycle/light_sweep.wgsl");

/// The decision contract (`decide.wgsl`), after its 64-bit words and
/// its lineage keys; and a growth unit's zones.
const U64: &str = include_str!("cycle/u64.wgsl");
const KEYS: &str = include_str!("cycle/keys.wgsl");
const DECIDE: &str = include_str!("cycle/decide.wgsl");
const ZONES: &str = include_str!("cycle/zones.wgsl");
/// The contract's marked arithmetic, plain (DESIGN.md section 9): its
/// floor sites held to the CPU's order (`held.rs`), every other float
/// the operation WGSL writes.
const ARITH: &str = include_str!("cycle/arith.wgsl");

/// The contract's WGSL: its 64-bit words, its lineage keys, the contract,
/// then its plain arithmetic.
pub(super) fn contract() -> String {
    contract_with(ARITH)
}

/// The contract with its marked arithmetic `arith`.
pub(super) fn contract_with(arith: &str) -> String {
    format!("{U64}\n{KEYS}\n{DECIDE}\n{arith}")
}

/// The contract and a growth unit's zones, for a shader that binds the
/// unit tables as `table` and `words`.
fn zones_with(arith: &str) -> String {
    format!("{}\n{ZONES}", contract_with(arith))
}
const HELD: &str = include_str!("cycle/held.wgsl");
const ROOT: &str = "fn exact_sqrt(x: f32) -> f32 {\n    return sqrt(x);\n}\n";

/// The units' and the wake's WGSL: the contract and the zones
/// (`zones_with`), the held arithmetic and the contract's held copies
/// (`held.rs`), then the cycle's state, its reals, a job's state and
/// frames, the advance, the stops, the zones, and the entries.
pub(super) fn source() -> String {
    format!(
        "{}\n{HELD}\n{}\n{ARENA}\n{REAL}\n{JOB}\n{FRAME}\n{ADVANCE}\n{STOP}\n{GROW}\n{UNITS}\n{WAKE}",
        zones_with(ARITH),
        held::source()
    )
}

/// The rough layout's WGSL (DESIGN.md section 2.3): the contract, then
/// the layout's columns and entries, its f32 vectors and angles, the
/// walk, the draw, the stand, the frame and the pencil.
pub(super) fn layout_source() -> String {
    format!(
        "{}\n{}\n{LAYOUT}\n{LAY_STATE}\n{Q}\n{LAY_WALK}\n{LAY_DRAW}\n{LAY_STAND}\n{LAY_FRAME}\n{LAY_PENCIL}",
        contract_with(ARITH),
        ROOT
    )
}

/// The light's WGSL (DESIGN.md section 2, step 8): the software f64,
/// the programs and their registers, the loop that runs them, then the
/// light's bindings and entries.
pub(super) fn light_source() -> String {
    format!(
        "{SOFT64}\n{}\n{LIGHT_VM}\n{LIGHT}\n{LIGHT_SWEEP}\n{}",
        light_program::wgsl(),
        ARITH
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

#[cfg(test)]
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
        "{}\n{REAL}\n{HIER_REAL}\n{cols}\n{HIER_CYCLE}\n{HIER_SWEEP}\n{HIER_SPLIT}\n{FOLD}",
        contract_with(ARITH)
    )
}

/// Every cycle module's WGSL, by name, for the shader checks.
#[cfg(test)]
pub(super) fn sources() -> Vec<(&'static str, String)> {
    let mut out = vec![
        ("cycle contract probe", contract_tests::source()),
        ("cycle chains", include_str!("cycle/chains.wgsl").to_string()),
        ("cycle dress", format!("{}\n{}", contract(), include_str!("cycle/dress.wgsl"))),
        ("cycle stems", include_str!("cycle/stems.wgsl").to_string()),
        ("cycle hierarchy build", hier_build_source()),
        ("cycle hierarchy scan", hier_scan_source()),
        ("cycle glue", glue::source()),
        ("cycle vm", vm::source()),
        ("cycle units and wake", source()),
        ("cycle layout", layout_source()),
        ("cycle hierarchy sums", hier_cycle_source()),
        ("cycle light", light_source()),
        ("cycle settle", settle_device::source()),
        ("cycle settle place", settle_device::place_source()),
        ("cycle retirement", retire_device::source()),
    ];
    let [compact, units] = retire_device::compaction_sources();
    out.push(("cycle compaction", compact.to_string()));
    out.push(("cycle compaction units", units.to_string()));
    out
}
