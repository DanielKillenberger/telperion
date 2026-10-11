// fn-256 task 2 (SETTLE-PLAN.md S1 to S6): the settled tree's place
// passes over the node table, in its final order. The layout's names
// (`layout.wgsl`, `lay_state.wgsl`) read the node table's columns here,
// so the layout's stand, walk and frame (`lay_stand.wgsl`,
// `lay_walk.wgsl`, `lay_frame.wgsl`) lay the settled tree unchanged: an
// axis's phytomers are contiguous from its start, and each axis's walk
// (its `Layer`) stands in `pw`'s columns where the layout keeps a pencil.

struct Cfg {
    // A dispatch row's invocations, its first item and its items.
    row: u32,
    base: u32,
    count: u32,
    age: u32,
    // The node table's strides and sections.
    sk: u32,
    skp: u32,
    at_np: u32,
    at_nu: u32,
    // Work's grown axes and final axes (`settle_cols.wgsl`), and `pw`'s
    // phytomers' section.
    sa: u32,
    at_a: u32,
    at_f: u32,
    at_q: u32,
    // The species' flags, the zero the driver cannot see through, the
    // kept axes, and whether the lay takes the levers (the sag place).
    flags: u32,
    zero: u32,
    kept: u32,
    sagging: u32,
    // The walk's dispatch: its first phytomer on each axis and how many
    // it takes (`walk_main`).
    walk_at: u32,
    walk_n: u32,
    pad0: u32,
    pad1: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> form: array<f32>;
@group(0) @binding(2) var<storage, read> work: array<u32>;
@group(0) @binding(3) var<storage, read_write> atom: array<atomic<u32>>;
@group(0) @binding(4) var<storage, read_write> nt: array<u32>;
@group(0) @binding(5) var<storage, read_write> pw: array<u32>;

const NONE: u32 = 0xffffffffu;
const ONE64 = vec2<u32>(0u, 0x3ff00000u);
const TICKS: f32 = 1048576.0;
const SEED: u32 = 0u;
const LATERAL: u32 = 1u;
const CONTINUED: u32 = 2u;
const RELAYED: u32 = 3u;
const KEEPS: u32 = 1u;
const LEAVES: u32 = 2u;

// The node table's axis columns (`settle_compact.wgsl`'s `NA_`) by the
// layout's names; the f32 copies the lay reads of the f64s it holds.
const A_ORIGIN: u32 = 0u;
const A_PARENT: u32 = 1u;
const A_NODE: u32 = 2u;
const A_LINEAGE: u32 = 5u;
const A_PA: u32 = 7u;
const A_BIRTH: u32 = 8u;
const A_VIGOUR: u32 = 10u;
const A_SLEEP: u32 = 17u;
const A_PHASE: u32 = 18u;
const A_PHYTS: u32 = 21u;
const A_PSTART: u32 = 22u;
const A_GROWN: u32 = 23u;
const A_BASE: u32 = 24u;
const A_HEADING: u32 = 27u;
const A_SIDE: u32 = 30u;
const A_JOINED: u32 = 33u;
const A_RANK: u32 = 38u;
const A_BLEND: u32 = 39u;
const A_WHORL: u32 = 40u;
// The phytomers' (`NP_`), the rank an f32 copy.
const PH_CYCLE: u32 = 0u;
const PH_KEY: u32 = 1u;
const PH_SIZE: u32 = 5u;
const PH_LIGHT: u32 = 7u;
const PH_GROWN: u32 = 9u;
const PH_NODE: u32 = 9u;
const PH_SCALE: u32 = 10u;
const PH_RADIUS: u32 = 11u;
const PH_TIP: u32 = 12u;
const PH_HEADING: u32 = 15u;
const PH_SIDE: u32 = 18u;
const PH_RANK: u32 = 21u;
// An axis's walk (`Layer`), as the layout's pencil holds it, from `pw`'s
// column `Y_LAYER`; beside it the axis's base scale, its relay's
// attachment, its base radius, reach, bend and whether it is the trunk.
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
const Y_BASE: u32 = 0u;
const Y_ATTK: u32 = 1u;
const Y_ATTT: u32 = 2u;
const Y_RADIUS: u32 = 3u;
const Y_REACH: u32 = 4u;
const Y_BEND: u32 = 5u;
const Y_TRUNK: u32 = 6u;
// Whether the axis's lay stopped (its frame below the ground, or a step).
const Y_FAIL: u32 = 7u;
const Y_LAYER: u32 = 8u;
const Y_FAR: u32 = 56u;
const Y_LOAD: u32 = 59u;
// `pw`'s phytomers' columns: each one's section from what joins it, the
// load its node bears, and its lever (moment, then chord).
const Q_AT: u32 = 0u;
const Q_LOAD: u32 = 1u;
const Q_LEVER: u32 = 5u;
// A PA's form row, after the two globals (`layout.wgsl`).
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
// Faults, each a bit of `atom`'s fault word.
const FAULT_UNREAD: u32 = 1u;
const FAULT_BEYOND: u32 = 2u;
const FAULT_BELOW: u32 = 4u;
const FAULT_LIFTS: u32 = 8u;
const FAULT_LINE: u32 = 16u;
const FAULT_UNITS: u32 = 32u;
const FAULT_NARROW: u32 = 64u;

var<private> status: u32;

fn exact_zero() -> u32 {
    return cfg.zero;
}

fn forget() {
}

// The item this invocation takes.
fn item(id: vec3<u32>) -> u32 {
    return cfg.base + id.x + id.y * cfg.row;
}

fn ax(i: u32, c: u32) -> u32 {
    return nt[c * cfg.sk + i];
}

fn axf(i: u32, c: u32) -> f32 {
    return bitcast<f32>(ax(i, c));
}

fn ax3(i: u32, c: u32) -> vec3<f32> {
    return vec3<f32>(axf(i, c), axf(i, c + 1u), axf(i, c + 2u));
}

fn set_ax(i: u32, c: u32, v: u32) {
    nt[c * cfg.sk + i] = v;
}

fn set_ax3(i: u32, c: u32, v: vec3<f32>) {
    for (var k = 0u; k < 3u; k++) {
        set_ax(i, c + k, bitcast<u32>(v[k]));
    }
}

fn kind_of(i: u32) -> u32 {
    return ax(i, A_ORIGIN) & 0xffu;
}

fn lineage_of(i: u32) -> vec2<u32> {
    return vec2<u32>(ax(i, A_LINEAGE), ax(i, A_LINEAGE + 1u));
}

fn ph(r: u32, c: u32) -> u32 {
    return nt[cfg.at_np + c * cfg.skp + r];
}

fn phf(r: u32, c: u32) -> f32 {
    return bitcast<f32>(ph(r, c));
}

fn ph64(r: u32, c: u32) -> vec2<u32> {
    return vec2<u32>(ph(r, c), ph(r, c + 1u));
}

fn ph3(r: u32, c: u32) -> vec3<f32> {
    return vec3<f32>(phf(r, c), phf(r, c + 1u), phf(r, c + 2u));
}

fn set_ph(r: u32, c: u32, v: f32) {
    nt[cfg.at_np + c * cfg.skp + r] = bitcast<u32>(v);
}

fn set_ph3(r: u32, c: u32, v: vec3<f32>) {
    for (var k = 0u; k < 3u; k++) {
        set_ph(r, c + k, v[k]);
    }
}

fn py(i: u32, c: u32) -> u32 {
    return pw[c * cfg.sk + i];
}

fn pyf(i: u32, c: u32) -> f32 {
    return bitcast<f32>(py(i, c));
}

fn set_py(i: u32, c: u32, v: u32) {
    pw[c * cfg.sk + i] = v;
}

fn set_pyf(i: u32, c: u32, v: f32) {
    set_py(i, c, bitcast<u32>(v));
}

fn pq(r: u32) -> f32 {
    return bitcast<f32>(pw[cfg.at_q + r]);
}

fn set_pq(r: u32, v: f32) {
    pw[cfg.at_q + r] = bitcast<u32>(v);
}

fn pq_at(r: u32, c: u32) -> f32 {
    return bitcast<f32>(pw[cfg.at_q + c * cfg.skp + r]);
}

fn set_pq_at(r: u32, c: u32, v: f32) {
    pw[cfg.at_q + c * cfg.skp + r] = bitcast<u32>(v);
}

fn pn(i: u32, c: u32) -> u32 {
    return py(i, Y_LAYER + c);
}

fn pnf(i: u32, c: u32) -> f32 {
    return bitcast<f32>(pn(i, c));
}

fn pn3(i: u32, c: u32) -> vec3<f32> {
    return vec3<f32>(pnf(i, c), pnf(i, c + 1u), pnf(i, c + 2u));
}

fn set_pn(i: u32, c: u32, v: u32) {
    set_py(i, Y_LAYER + c, v);
}

fn set_pnf(i: u32, c: u32, v: f32) {
    set_pn(i, c, bitcast<u32>(v));
}

fn set_pn3(i: u32, c: u32, v: vec3<f32>) {
    for (var k = 0u; k < 3u; k++) {
        set_pnf(i, c + k, v[k]);
    }
}

fn fm(pa: u32, k: u32) -> f32 {
    return form[2u + pa * FORM + k];
}

// The phytomers axis `a` holds, and its phytomer `j`'s row.
fn nphyt(a: u32) -> u32 {
    return ax(a, A_PHYTS);
}

fn prow(a: u32, j: u32) -> u32 {
    return ax(a, A_PSTART) + j;
}

// A whole count of ticks, signed, as the f32 of its cycles (`years as
// f32`), rounded once.
fn years_of(ticks: i32) -> f32 {
    let mag = to_f32(vec2<u32>(u32(abs(ticks)), 0u)) * (1.0 / TICKS);
    return select(mag, -mag, ticks < 0);
}

// Work's columns the place reads (`settle_shared.wgsl`): the grown
// children, the disused pipes, the final axes' children by index.
fn wa(c: u32, i: u32) -> u32 {
    return work[cfg.at_a + c * cfg.sa + i];
}

fn wf(c: u32, f: u32) -> u32 {
    return work[cfg.at_f + c * cfg.sa + f];
}

fn kpa(pa: u32, k: u32) -> u32 {
    return work[K_AT + 16u * pa + k];
}

fn kf(pa: u32, k: u32) -> f32 {
    return bitcast<f32>(kpa(pa, k));
}
