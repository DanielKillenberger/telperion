// fn-256 task 2 (SETTLE-PLAN.md): the settle's bindings and columns. It
// reads the cycle's one resident state (`state.rs`), the axes, units and
// phytomers the build grew, by the CPU's index, and works in `work`:
// columns of one word a row in sections (the grown axes, the grown
// phytomers and units, the final axes, the lists), each `stride` rows
// apart. `atom` holds the counts that atomic adds take, which no order
// changes; `nt` is the node table, the settled tree in lineage order.

struct Cfg {
    // The grown axes, a dispatch row's invocations, and this dispatch's
    // first item and items.
    n: u32,
    row: u32,
    base: u32,
    count: u32,
    // The state's strides, and the tree's age.
    axes_cap: u32,
    units_cap: u32,
    phyts_cap: u32,
    age: u32,
    // Work's strides (grown and final axes, phytomers, units; the node
    // table's axes) and its sections' first words.
    sa: u32,
    sp: u32,
    su: u32,
    sk: u32,
    at_a: u32,
    at_p: u32,
    at_u: u32,
    at_f: u32,
    // The node table's strides (phytomers, units) and sections.
    skp: u32,
    sku: u32,
    at_np: u32,
    at_nu: u32,
    // A pass's own words: which of two columns it reads, the depth, and
    // the next depth's first final index.
    flip: u32,
    depth: u32,
    next: u32,
    at_l: u32,
    // The living apexes and woken buds, the species' flags, the zero the
    // driver cannot see through, the lists' stride, the settled
    // tree's axes, and where `atom` fills each parent's children.
    live: u32,
    woken: u32,
    flags: u32,
    zero: u32,
    sl: u32,
    kept: u32,
    kfill: u32,
    // The retired rows' records' stride (`retire_device.rs`).
    rec_cap: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> axes: array<u32>;
@group(0) @binding(2) var<storage, read> units: array<u32>;
@group(0) @binding(3) var<storage, read> grown: array<u32>;
@group(0) @binding(4) var<storage, read_write> work: array<u32>;
@group(0) @binding(5) var<storage, read_write> atom: array<atomic<u32>>;
@group(0) @binding(6) var<storage, read_write> nt: array<u32>;
// Retirement's columns (`retire_cols.wgsl`'s `R_`, `axes_cap` rows apart)
// and the retired rows' grown columns, a row the arena let go reached by
// its unit's first phytomer's high bit (`retire_units.wgsl`).
@group(0) @binding(7) var<storage, read> ret: array<u32>;
@group(0) @binding(8) var<storage, read> recs: array<u32>;

const NONE: u32 = 0xffffffffu;
const RECORDED: u32 = 0x80000000u;
const RT_FATE: u32 = 1u;
const RT_ROOT_LIVED: u32 = 8u;
const ONE64 = vec2<u32>(0u, 0x3ff00000u);
const TICKS: u32 = 0x100000u;
const SEED: u32 = 0u;
const LATERAL: u32 = 1u;
const CONTINUED: u32 = 2u;
const RELAYED: u32 = 3u;

// The species' flags (`cfg.flags`): a PA keeps the tree as grown, a PA
// thickens by its leaves, a PA retains shed pipes, a PA sheds.
const KEEPS: u32 = 1u;
const LEAVES: u32 = 2u;
const RETAINS: u32 = 4u;
const SHEDS: u32 = 8u;

// The state's columns (`arena.wgsl`'s `A_`, `U_`, `P_`).
const A_PA: u32 = 1u;
const A_BIRTH: u32 = 2u;
const A_ORIGIN: u32 = 3u;
const A_PARENT: u32 = 4u;
const A_NODE: u32 = 5u;
const A_LINEAGE: u32 = 6u;
const A_WHORL: u32 = 12u;
const A_BLEND: u32 = 14u;
const A_END: u32 = 16u;
const A_PHYTS: u32 = 17u;
const A_B0: u32 = 19u;
const A_B1: u32 = 21u;
const A_FLAGS: u32 = 23u;
const A_OUTLIVED: u32 = 25u;
const A_SLEEP: u32 = 28u;
const A_UNITS: u32 = 29u;
const A_SUCC0: u32 = 31u;
const A_HEAD: u32 = 34u;
const A_PHASE: u32 = 39u;
const F_FAILED: u32 = 1u;
const F_OUTLIVED: u32 = 32u;
const U_SURVIVE: u32 = 1u;
const U_PERSIST: u32 = 2u;
const U_SHARE: u32 = 3u;
const U_SIZE: u32 = 4u;
const U_SHADE: u32 = 5u;
const U_NEXT: u32 = 10u;
const U_FIRST: u32 = 11u;
const U_COUNT: u32 = 12u;
const P_KEY: u32 = 0u;
const P_AXIS: u32 = 2u;
const P_CYCLE: u32 = 3u;
const P_NODE: u32 = 4u;

fn item(id: vec3<u32>) -> u32 {
    return cfg.base + id.x + id.y * cfg.row;
}

fn m(a: f32, b: f32) -> f32 {
    return exact_kept(a * b);
}

fn apart(x: f32) -> f32 {
    return exact_kept(x);
}

fn fault(bits: u32) {
    atomicOr(&atom[T_FAULT], bits);
}

fn ax(i: u32, c: u32) -> u32 {
    return axes[c * cfg.axes_cap + i];
}

fn ax64(i: u32, c: u32) -> vec2<u32> {
    return vec2<u32>(ax(i, c), ax(i, c + 1u));
}

fn unit(r: u32, c: u32) -> u32 {
    return units[c * cfg.units_cap + r];
}

fn phyt(r: u32, c: u32) -> u32 {
    if (r & RECORDED) != 0u {
        return recs[c * cfg.rec_cap + (r ^ RECORDED)];
    }
    return grown[c * cfg.phyts_cap + r];
}

// A retired subtree's root's fate and the living it stands for its
// subtree by (`retire::Fate::Root`).
fn retired_root(i: u32) -> bool {
    return ret[RT_FATE * cfg.axes_cap + i] == 2u;
}

fn root_living(i: u32) -> vec2<f32> {
    return vec2<f32>(bitcast<f32>(ret[RT_ROOT_LIVED * cfg.axes_cap + i]), bitcast<f32>(ret[(RT_ROOT_LIVED + 1u) * cfg.axes_cap + i]));
}

fn kind_of(i: u32) -> u32 {
    return ax(i, A_ORIGIN) & 0xffu;
}

fn wa(c: u32, i: u32) -> u32 {
    return work[cfg.at_a + c * cfg.sa + i];
}

fn waf(c: u32, i: u32) -> f32 {
    return bitcast<f32>(wa(c, i));
}

fn set_wa(c: u32, i: u32, v: u32) {
    work[cfg.at_a + c * cfg.sa + i] = v;
}

fn set_waf(c: u32, i: u32, v: f32) {
    set_wa(c, i, bitcast<u32>(v));
}

fn wp(c: u32, g: u32) -> u32 {
    return work[cfg.at_p + c * cfg.sp + g];
}

fn set_wp(c: u32, g: u32, v: u32) {
    work[cfg.at_p + c * cfg.sp + g] = v;
}

fn wu(c: u32, g: u32) -> u32 {
    return work[cfg.at_u + c * cfg.su + g];
}

fn set_wu(c: u32, g: u32, v: u32) {
    work[cfg.at_u + c * cfg.su + g] = v;
}

fn wf(c: u32, f: u32) -> u32 {
    return work[cfg.at_f + c * cfg.sa + f];
}

fn set_wf(c: u32, f: u32, v: u32) {
    work[cfg.at_f + c * cfg.sa + f] = v;
}

fn wl(c: u32, k: u32) -> u32 {
    return work[cfg.at_l + c * cfg.sl + k];
}

fn kpa(pa: u32, k: u32) -> u32 {
    return work[K_AT + 16u * pa + k];
}

fn kpaf(pa: u32, k: u32) -> f32 {
    return bitcast<f32>(kpa(pa, k));
}

// An axis's children by grown index, from `atom`'s scanned starts.
fn kids_of(i: u32) -> vec2<u32> {
    return vec2<u32>(atomicLoad(&atom[T_KCOUNT + i]), atomicLoad(&atom[T_KCOUNT + i + 1u]));
}

fn kid(k: u32) -> u32 {
    return wa(W_KIDS, k);
}

// `presence::q` of a presence held in an f32: none for a negative one.
fn qf(x: f32) -> f32 {
    return select(0.0, x, x > 0.0);
}

// A count of ticks as the f32 of its cycles, exact below 2^24 ticks.
fn tick_q(t: u32) -> f32 {
    return f32(t) * (1.0 / f32(TICKS));
}
