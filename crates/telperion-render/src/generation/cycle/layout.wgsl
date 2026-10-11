// fn-256 task 2 (DESIGN.md section 2.3): one cycle's rough layout on the
// device, test-only. Round 0 lays the units the grown axes grew
// (`draw`); then each relay's node is set (`relay`), each new axis's
// generation found (`generation`), and per generation its axes are listed
// by a flag, a scan and a gather, framed (`frame`) and laid (`draw`).
// The state is the cycle's one resident state (`state.rs`): the axes'
// columns the units write and the layout's beside them, the units'
// records and the phytomers in one arena, the pencils by axis. An index
// is the CPU's. An axis's phytomers are found through its units' records
// (`prow`), each unit's phytomers contiguous.

struct Cfg {
    n: u32,
    row: u32,
    cycle: u32,
    first_new: u32,
    // The axes', phytomers' and units' columns' strides.
    axes: u32,
    phyts: u32,
    units: u32,
    new_axes: u32,
    // The list a `draw` takes: the grown axes (0) or a generation's (1).
    listed: u32,
    generation: u32,
    zero: u32,
    grown: u32,
    // Where the phytomers start in the arena, and the first item of this
    // dispatch's range of its list.
    phyt_at: u32,
    base: u32,
    pad0: u32,
    pad1: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> form: array<f32>;
@group(0) @binding(2) var<storage, read_write> arena: array<u32>;
@group(0) @binding(3) var<storage, read_write> axes: array<u32>;
@group(0) @binding(4) var<storage, read_write> pencils: array<u32>;
@group(0) @binding(5) var<storage, read_write> lists: array<u32>;
@group(0) @binding(6) var<storage, read_write> counts: array<u32>;

const NONE: u32 = 0xffffffffu;
const TICKS: f32 = 1048576.0;
const SEED: u32 = 0u;
const LATERAL: u32 = 1u;
const CONTINUED: u32 = 2u;
const RELAYED: u32 = 3u;

// An axis's columns (`state.rs`): the units' (`arena.wgsl`'s `A_`), then
// the layout's.
const A_PA: u32 = 1u;
const A_BIRTH: u32 = 2u;
const A_ORIGIN: u32 = 3u;
const A_PARENT: u32 = 4u;
const A_NODE: u32 = 5u;
const A_LINEAGE: u32 = 6u;
const A_PHYTS: u32 = 17u;
const A_SLEEP: u32 = 28u;
const A_UNITS_N: u32 = 29u;
const A_TAIL: u32 = 33u;
const A_WHORL: u32 = 35u;
const A_BLEND: u32 = 36u;
const A_B0: u32 = 37u;
const A_B1: u32 = 38u;
const A_PHASE: u32 = 39u;
const A_RANK: u32 = 40u;
const A_BASE: u32 = 41u;
const A_HEADING: u32 = 44u;
const A_SIDE: u32 = 47u;
const A_FAULT: u32 = 50u;
// A phytomer's.
const PH_KEY: u32 = 0u;
const PH_CYCLE: u32 = 3u;
const PH_NODE: u32 = 4u;
const PH_TIP: u32 = 5u;
const PH_HEADING: u32 = 8u;
const PH_SIDE: u32 = 11u;
const PH_SCALE: u32 = 14u;
const PH_GROWN: u32 = 15u;
const PH_SIZE: u32 = 16u;
const PH_RANK: u32 = 17u;
const PH_AREA: u32 = 18u;
// A unit's (`arena.wgsl`'s `U_`).
const UN_SURVIVE: u32 = 1u;
const UN_PERSIST: u32 = 2u;
const UN_SIZE: u32 = 4u;
const UN_SHADE: u32 = 5u;
const UN_PREV: u32 = 9u;
const UN_NEXT: u32 = 10u;
const UN_FIRST: u32 = 11u;
const UN_COUNT: u32 = 12u;
// A pencil's: its walk, its pulls, then its scalars.
const P_RUNNING: u32 = 0u;
const P_ACROSS: u32 = 3u;
const P_TIP: u32 = 6u;
const P_RUN: u32 = 9u;
const P_PULL: u32 = 10u;
const P_CURVE: u32 = 11u;
const P_TURNING: u32 = 14u;
const P_LIFTS_N: u32 = 15u;
const P_LIFTS: u32 = 16u;
const LIFTS: u32 = 8u;
const P_BASE_SCALE: u32 = 48u;
const P_GOING: u32 = 49u;
const P_RANKED: u32 = 50u;
const P_UNITS: u32 = 51u;
const P_LAID: u32 = 52u;
const P_TRUNK: u32 = 53u;
const P_WOOD: u32 = 54u;
const P_FRESH: u32 = 55u;
// A PA's form row, after the two globals (the cone and the ground).
const FORM: u32 = 19u;
const F_INTERNODE: u32 = 0u;
const F_TROPISM: u32 = 1u;
const F_ELEVATION: u32 = 2u;
const F_WANDER: u32 = 3u;
const F_BEND_LENGTH: u32 = 4u;
const F_INTERNODE_SPREAD: u32 = 5u;
const F_TROPISM_SPREAD: u32 = 6u;
const F_INSERTION: u32 = 7u;
const F_INSERTION_SPREAD: u32 = 8u;
const F_ROLL: u32 = 9u;
const F_PLANE: u32 = 10u;
const F_DIVERGENCE: u32 = 11u;
const F_EPITONY: u32 = 12u;
const F_STRAIGHTENING: u32 = 13u;
const F_ERECTION: u32 = 14u;
const F_RELAY_AT: u32 = 15u;
const F_RELAY_NODE: u32 = 16u;
const F_DOMINANCE: u32 = 17u;
const F_LEAF_AREA: u32 = 18u;
// Faults, each a bit of the axis's fault word.
const FAULT_UNREAD: u32 = 1u;
const FAULT_BEYOND: u32 = 2u;
const FAULT_BELOW: u32 = 4u;
const FAULT_LIFTS: u32 = 8u;
const FAULT_LINE: u32 = 16u;
const FAULT_UNITS: u32 = 32u;
const FAULT_NARROW: u32 = 64u;
// The tally's words at the head of `lists`: the generation listed, and
// every fault any axis met this cycle.
const T_LISTED: u32 = 0u;
const T_FAULT: u32 = 8u;
const TALLY: u32 = 16u;

var<private> status: u32;

fn exact_zero() -> u32 {
    return cfg.zero;
}

// A whole count of ticks, signed, as the f32 of its cycles (`years as
// f32`): rounded once, as Rust rounds the f64 it is exact in.
fn years_of(ticks: i32) -> f32 {
    let mag = to_f32(vec2<u32>(u32(abs(ticks)), 0u)) * (1.0 / TICKS);
    return select(mag, -mag, ticks < 0);
}

fn invocation(id: vec3<u32>) -> u32 {
    return cfg.base + id.x + id.y * cfg.row;
}

// An axis's faults kept on it and in the cycle's tally.
fn fault_out(i: u32) {
    set_ax(i, A_FAULT, ax(i, A_FAULT) | status);
    if status != 0u {
        lists[T_FAULT] = lists[T_FAULT] | status;
    }
}

// The axis a `draw` or `frame` invocation takes, or NONE past its list.
fn listed_axis(k: u32) -> u32 {
    if cfg.listed == 0u {
        return select(NONE, lists[TALLY + k], k < cfg.grown);
    }
    if k >= lists[T_LISTED] {
        return NONE;
    }
    return lists[listed_at() + k];
}

// Where the new axes' generations stand in `lists`, and the list of one.
fn generations_at() -> u32 {
    return TALLY + cfg.grown;
}

fn listed_at() -> u32 {
    return generations_at() + cfg.new_axes;
}

@compute @workgroup_size(64)
fn draw_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = listed_axis(invocation(id));
    if i == NONE {
        return;
    }
    status = 0u;
    forget();
    draw_axis(i);
    fault_out(i);
}

@compute @workgroup_size(64)
fn frame_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = listed_axis(invocation(id));
    if i == NONE {
        return;
    }
    status = 0u;
    forget();
    frame(i);
    fault_out(i);
}

// Each new relay set at its node (`presence::relay_node`).
@compute @workgroup_size(64)
fn relay_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = invocation(id);
    if k >= cfg.new_axes {
        return;
    }
    let i = cfg.first_new + k;
    if kind_of(i) == RELAYED {
        status = 0u;
        forget();
        set_ax(i, A_NODE, relay_node(ax(i, A_PARENT)));
        fault_out(i);
    }
}

// Each new axis's generation: 0 on an axis the cycle began with, its
// parent's and one on a new one.
@compute @workgroup_size(64)
fn generation_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = invocation(id);
    if k >= cfg.new_axes {
        return;
    }
    var g = 0u;
    var a = ax(cfg.first_new + k, A_PARENT);
    // cap: the cycle's new axes, each ancestor met once.
    while a != NONE && a >= cfg.first_new {
        g += 1u;
        a = ax(a, A_PARENT);
    }
    lists[generations_at() + k] = g;
}

// The new axes of the generation listed flagged, for the scan.
@compute @workgroup_size(64)
fn flag_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = invocation(id);
    if k >= cfg.new_axes {
        return;
    }
    counts[k] = u32(lists[generations_at() + k] == cfg.generation);
}

// Each flagged axis at its scanned place.
@compute @workgroup_size(64)
fn gather_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = invocation(id);
    if k >= cfg.new_axes || lists[generations_at() + k] != cfg.generation {
        return;
    }
    lists[listed_at() + counts[k]] = cfg.first_new + k;
}
