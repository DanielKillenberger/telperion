// fn-256 task 2 (DESIGN.md section 6, as decided for the build): retirement
// on the device, at each cycle's start before the balance. It reads the
// state as the last cycle left it, the hierarchy's lists and shape (the
// laid axes, as fn-255's working set links them), and keeps its own
// columns in `ret`: per laid axis, each a word `cfg.stride` rows apart.
// Its verdicts are logged in `verdicts` for the tests.

struct Cfg {
    // The laid axes at the cycle's start, the hierarchy's stride, the
    // cycle, a sweep's block level, a dispatch row and workgroups' width,
    // the levels the hierarchy holds, and the zero the driver cannot see.
    n: u32,
    stride: u32,
    cycle: u32,
    level: u32,
    row: u32,
    wx: u32,
    levels: u32,
    zero: u32,
    // The living apexes, sleeping buds and woken buds at the cycle's
    // start, and the tree's age.
    live: u32,
    sleepers: u32,
    woken: u32,
    age: u32,
    // The state's strides and its lists' sections.
    axes_cap: u32,
    units_cap: u32,
    phyts_cap: u32,
    sleep_at: u32,
    sleep_cap: u32,
    next_at: u32,
    woken_at: u32,
    woken_cap: u32,
    // Where `ret`'s header and the PA constants stand, and the log's
    // most entries.
    head: u32,
    log_cap: u32,
    // The block levels a folded sweep loops (`fold.wgsl`).
    fold_lo: u32,
    fold_hi: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> axes: array<u32>;
@group(0) @binding(2) var<storage, read> units: array<u32>;
@group(0) @binding(3) var<storage, read> grown: array<u32>;
@group(0) @binding(4) var<storage, read> lists: array<u32>;
@group(0) @binding(5) var<storage, read> hlists: array<u32>;
@group(0) @binding(6) var<storage, read> shape: array<u32>;
@group(0) @binding(7) var<storage, read_write> ret: array<u32>;
@group(0) @binding(8) var<storage, read_write> verdicts: array<atomic<u32>>;

const NONE: u32 = 0xffffffffu;
const ONE64 = vec2<u32>(0u, 0x3ff00000u);
const TICKS: u32 = 0x100000u;
const LATERAL: u32 = 1u;
const CONTINUED: u32 = 2u;
const RELAYED: u32 = 3u;
const WG: u32 = 64u;

// The state's columns (`arena.wgsl`).
const A_PA: u32 = 1u;
const A_BIRTH: u32 = 2u;
const A_ORIGIN: u32 = 3u;
const A_PARENT: u32 = 4u;
const A_NODE: u32 = 5u;
const A_LINEAGE: u32 = 6u;
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
const F_FAILED: u32 = 1u;
const F_OUTLIVED: u32 = 32u;
const U_SURVIVE: u32 = 1u;
const U_PERSIST: u32 = 2u;
const U_SHARE: u32 = 3u;
const U_SIZE: u32 = 4u;
const U_NEXT: u32 = 10u;
const U_FIRST: u32 = 11u;
const U_COUNT: u32 = 12u;
const P_CYCLE: u32 = 3u;
const P_NODE: u32 = 4u;
const S_WAKE: u32 = 0u;
const S_PARENT: u32 = 1u;
const W_AXIS: u32 = 0u;
const W_BEARER: u32 = 1u;
const W_NEXT: u32 = 2u;
const W_CYCLE: u32 = 3u;
const W_SHARE: u32 = 4u;

// The hierarchy's lists and shape (`hier_cols.wgsl`).
const R_KID_START: u32 = 0u;
const R_KIDS: u32 = 1u;
const R_MEM_START: u32 = 2u;
const R_MEMBERS: u32 = 3u;
const R_LVL_START: u32 = 4u;
const R_LVL_BLOCKS: u32 = 5u;
const H_DEPTH: u32 = 0u;

// `ret`'s columns: the last cycle an axis was open, its fate (open 0,
// kept 1, retired 2, inside a retired subtree 3), whether it waits on a woken bud, whether it closed
// this cycle, whether its presences are assigned, its own living time
// and its end's presence, its subtree's lived and alive, the cycle its
// woken bud's wait passes (none: 0) and its subtree's latest, a woken
// bud's birth presence by its bearer's kept share, and a retired root's
// lived and alive.
const R_OPEN: u32 = 0u;
const R_FATE: u32 = 1u;
const R_PEND: u32 = 2u;
const R_CLOSED: u32 = 3u;
const R_SEEN: u32 = 4u;
const R_LIVED0: u32 = 5u;
const R_END: u32 = 6u;
const R_LIVED: u32 = 7u;
const R_ALIVE: u32 = 8u;
const R_WAKE: u32 = 9u;
const R_WSUB: u32 = 10u;
const R_QB1: u32 = 11u;
const R_ROOT_LIVED: u32 = 12u;
const R_ROOT_ALIVE: u32 = 13u;
const R_COLS: u32 = 14u;
// The header from `cfg.head`: the laid axes the last cycle began with;
// each PA's constants from `cfg.head + 64` (`settle_shared.wgsl`'s `K_`).
const HD_PREV_LAID: u32 = 0u;
const KEPT: u32 = 1u;
const RETIRED: u32 = 2u;
const INSIDE: u32 = 3u;
const K_DELAY: u32 = 3u;
const K_FLAGS: u32 = 7u;
const KF_SHEDS: u32 = 1u;
// The log: its count, a fault where it would pass its most, then each
// entry's root, cycle, verdict (waited 0, kept 1, retired 2), lived and
// alive.
const L_COUNT: u32 = 0u;
const L_FAULT: u32 = 1u;
const L_AT: u32 = 4u;

fn m(a: f32, b: f32) -> f32 {
    return exact_kept(a * b);
}

fn index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
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
    return grown[c * cfg.phyts_cap + r];
}

fn kind_of(i: u32) -> u32 {
    return ax(i, A_ORIGIN) & 0xffu;
}

fn rt(c: u32, i: u32) -> u32 {
    return ret[c * cfg.stride + i];
}

fn rtf(c: u32, i: u32) -> f32 {
    return bitcast<f32>(rt(c, i));
}

fn set_rt(c: u32, i: u32, v: u32) {
    ret[c * cfg.stride + i] = v;
}

fn set_rtf(c: u32, i: u32, v: f32) {
    set_rt(c, i, bitcast<u32>(v));
}

fn kpa(pa: u32, k: u32) -> u32 {
    return ret[cfg.head + 64u + 16u * pa + k];
}

fn ls(col: u32, i: u32) -> u32 {
    return col * (cfg.stride + 64u) + i;
}

fn qf(x: f32) -> f32 {
    return select(0.0, x, x > 0.0);
}

fn tick_q(t: u32) -> f32 {
    return f32(t) * (1.0 / f32(TICKS));
}

// A block's run (its members' first and end, its least and greatest
// depth) within a hierarchy of `stride` rows, none where a halted cycle
// left it past them: no sweep loops on what it left (fn-256 task 5).
fn within(run: vec4<u32>, stride: u32) -> vec4<u32> {
    let ok = run.x < run.y && run.y <= stride && run.z <= run.w && run.w - run.z < stride;
    return select(vec4<u32>(0u), run, ok);
}

// An axis's children's range within the lists' rows, none past them.
fn kids_within(kids: vec2<u32>, stride: u32) -> vec2<u32> {
    let ok = kids.x <= kids.y && kids.y <= stride + 64u;
    return select(vec2<u32>(0u), kids, ok);
}
