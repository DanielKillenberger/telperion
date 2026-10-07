// A chain grown alone on the GPU (fn-209 F1, task 2): P2's chain grower
// (`telperion_space::grow::alone::grow`), one invocation per chain, from
// its bud and the species point's tables, every decision on the contract
// (`contract.wgsl`). It writes the chain's axes (its own, and the
// laterals it bears, made but not grown) and phytomers into the chain's
// region; a chain that outgrows its region stops and is reported, to be
// grown again in a larger one (the overflow rule: a chain is grown from
// its own bud alone, so a larger region grows the same chain). Its
// sleeping buds wake as buds it bears, and a woken bud's chain begins
// with the stages it slept through (`dormant.rs`, `wake.rs`).

struct Cfg {
    n: u32,
    row: u32,
    axes_cap: u32,
    phyt_cap: u32,
    live_cap: u32,
    rows: u32,
    age: u32,
    span_lo: u32,
    span_hi: u32,
    sleep_cap: u32,
    pend_cap: u32,
    geo: u32,
    unit_cap: u32,
    first: u32,
    pad0: u32,
    pad1: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> buds: array<vec2<u32>>;
@group(0) @binding(2) var<storage, read> table: array<u32>;
@group(0) @binding(3) var<storage, read> words: array<vec2<u32>>;
@group(0) @binding(4) var<storage, read_write> axes: array<u32>;
@group(0) @binding(5) var<storage, read_write> phyts: array<u32>;
@group(0) @binding(6) var<storage, read_write> live: array<vec4<u32>>;
@group(0) @binding(7) var<storage, read_write> status: array<vec4<u32>>;
@group(0) @binding(8) var<storage, read_write> sleepers: array<u32>;

const AXW: u32 = 80u;
const PW: u32 = 11u;
const ROW: u32 = 18u;
const TICKS: u32 = 0x100000u;
const STACK: u32 = 16u;
const SW: u32 = 15u;
const QW: u32 = 21u;
const BW: u32 = 14u;
const DORMANT = vec2<u32>(0xfffffff9u, 0xffffffffu);

// The draws' parts (`lineage.rs`).
const VIABILITY: u32 = 0u;
const ABORTION: u32 = 1u;
const MOVE: u32 = 2u;
const RELAY = vec2<u32>(0xfffffffdu, 0xffffffffu);
const ENDED = vec2<u32>(0xfffffff8u, 0xffffffffu);
const END = vec2<u32>(0xfffffff7u, 0xffffffffu);
const CONTINUATION = vec2<u32>(0xffffffffu, 0xffffffffu);

// How an axis stands on its parent.
const SEED: u32 = 0u;
const LATERAL: u32 = 1u;
const CONTINUED: u32 = 2u;
const RELAYED: u32 = 3u;
const STUB: u32 = 4u;

// Why an apex stopped: the relay share that takes it over.
const ABORTED: u32 = 0u;
const ENDS: u32 = 1u;
const FAILED: u32 = 2u;

// The fields of an axis.
const LINEAGE: u32 = 0u;
const PA: u32 = 2u;
const BIRTH: u32 = 3u;
const KIND: u32 = 4u;
const PARENT: u32 = 5u;
const NODE: u32 = 6u;
const APEX_END: u32 = 7u;
const ROOT: u32 = 8u;
const HORIZON: u32 = 10u;
const UNITS: u32 = 12u;
const AGED: u32 = 14u;
const SHARES: u32 = 16u;
const NUNITS: u32 = 18u;
const SLEEP: u32 = 19u;
const ENDED_RELAYS: u32 = 20u;
const OUTLIVED: u32 = 21u;
const SUCC: u32 = 22u;
const NPHYT: u32 = 24u;
// Its presences (f32, `presence.wgsl`): its draws' as made, its running
// survival, its nodes counted by presence, its relay's blend, a borne
// bud's vigour as made and its node's buds a whorl, and how much its apex
// lives on at the age (below none: it has stopped).
const MADE: u32 = 32u;
const RUN: u32 = 33u;
// Its rank, Q52 in two words (34, 39): a node's presence (Q32) times its
// unit's share (ticks), summed exactly, so its phyllotaxis turns as the
// CPU's does however many nodes it holds.
const RANKF: u32 = 34u;
const RANK_HI: u32 = 39u;
const BLEND: u32 = 35u;
const VIG: u32 = 36u;
const WHORL: u32 = 37u;
const LIVING: u32 = 38u;
// A phytomer's scale by presence (f32), and its rank (Q52, two words).
const PSCALE: u32 = 6u;
const PRANK: u32 = 7u;

// The KIND word's flags.
const IS_ENDED: u32 = 0x10000u;
const IS_FAILED: u32 = 0x20000u;
const HAS_OUTLIVED: u32 = 0x40000u;
const IS_WOKEN: u32 = 0x80000u;

var<private> axis_base: u32;
var<private> phyt_base: u32;
var<private> live_base: u32;
var<private> n_axes: u32;
var<private> n_phyts: u32;
var<private> n_next: u32;
var<private> next_half: u32;
var<private> overflow: u32;
var<private> sleep_base: u32;
var<private> pend_base: u32;
var<private> n_sleepers: u32;
var<private> n_pends: u32;
var<private> unit_base: u32;
var<private> n_units: u32;

struct Apex {
    axis: u32,
    spent: vec2<u32>,
}

// A growth unit's advance still to run: the tail of a stop or a move.
struct Next {
    go: bool,
    apex: Apex,
}

// What a move on leaves to do once its continuation's same-cycle growth
// is done: the relay of an apex that all but stopped (host decision 11).
struct After {
    apex: Apex,
    used: vec2<u32>,
    made: vec2<u32>,
    key: vec2<u32>,
}

var<private> stack: array<After, STACK>;
var<private> depth: u32;

fn none() -> Next {
    return Next(false, Apex(0u, vec2<u32>(0u)));
}

fn at(i: u32, f: u32) -> u32 {
    return axis_base + i * AXW + f;
}

fn rd(i: u32, f: u32) -> u32 {
    return axes[at(i, f)];
}

fn wr(i: u32, f: u32, v: u32) {
    axes[at(i, f)] = v;
}

fn rd64(i: u32, f: u32) -> vec2<u32> {
    return vec2<u32>(axes[at(i, f)], axes[at(i, f + 1u)]);
}

fn wr64(i: u32, f: u32, v: vec2<u32>) {
    axes[at(i, f)] = v.x;
    axes[at(i, f + 1u)] = v.y;
}

// PA `pa`'s row: its flags word, and its words from `k`.
fn flags(pa: u32) -> u32 {
    return table[cfg.rows + 4u * pa];
}

fn lived(pa: u32) -> u32 {
    return table[cfg.rows + 4u * pa + 1u];
}

fn successor(pa: u32) -> u32 {
    return table[cfg.rows + 4u * pa + 2u];
}

fn row(pa: u32, k: u32) -> vec2<u32> {
    return words[table[cfg.rows + 4u * pa + 3u] + k];
}

fn log_wood(pa: u32, cycle: u32) -> vec2<u32> {
    return words[row(pa, 15u).x + cycle];
}

fn q32_whole(n: u32) -> vec2<u32> {
    return w(0u, n);
}

fn ticks_whole() -> vec2<u32> {
    return w(TICKS, 0u);
}

// A new axis; its index, or none where the region is full.
fn push_axis(lineage: vec2<u32>, pa: u32, birth: u32, kind: u32, parent: u32, node: u32, root: vec2<u32>, horizon: vec2<u32>, units: vec2<u32>) -> u32 {
    if n_axes >= cfg.axes_cap {
        overflow |= 1u;
        return NONE;
    }
    let i = n_axes;
    n_axes += 1u;
    for (var f = 0u; f < AXW; f++) {
        wr(i, f, 0u);
    }
    wr64(i, LINEAGE, lineage);
    wr(i, PA, pa);
    wr(i, BIRTH, birth);
    wr(i, KIND, kind);
    wr(i, PARENT, parent);
    wr(i, NODE, node);
    wr64(i, ROOT, root);
    wr64(i, HORIZON, horizon);
    wr64(i, UNITS, units);
    wr(i, SUCC, NONE);
    wr(i, SUCC + 1u, NONE);
    made_axis(i, 1.0, 1.0);
    return i;
}

fn add_successor(i: u32, s: u32) {
    if rd(i, SUCC) == NONE {
        wr(i, SUCC, s);
    } else {
        wr(i, SUCC + 1u, s);
    }
}

fn push_next(axis: u32, spent: vec2<u32>) {
    if n_next >= cfg.live_cap {
        overflow |= 4u;
        return;
    }
    live[live_base + next_half * cfg.live_cap + n_next] = vec4<u32>(axis, spent.x, spent.y, 0u);
    n_next += 1u;
}

// The horizon of a new axis of `pa` standing on `parent` as `kind`.
fn horizon_of(pa: u32, kind: u32, parent: u32) -> vec2<u32> {
    let h = rd64(parent, HORIZON);
    if (kind == LATERAL || kind == RELAYED) && (flags(pa) & 16u) != 0u {
        return smax64(h, row(pa, 13u));
    }
    return h;
}

// D14: a lateral of `pa` made in `cycle` on `parent` certain to be shed
// by the tree's age.
fn shed_certain(pa: u32, cycle: u32, parent: u32) -> bool {
    if (flags(pa) & 48u) != 48u {
        return false;
    }
    let h = smax64(rd64(parent, HORIZON), row(pa, 13u));
    let bound = sub64(q32_whole(cfg.age), add64(q32_whole(cycle), row(pa, 14u)));
    return !slt64(bound, h);
}

// Unit `i` (1 or 2) of a bud carrying `spent` ticks, `gone` of its first
// cycle, as the relay of an ended age or failed unit (`relay`), in an age
// of `life` ticks: its share (signed ticks), whether the age ends with it,
// and the share of its cycle gone when it ends.
struct Step {
    share: vec2<u32>,
    ends: bool,
    used: vec2<u32>,
}

fn step(spent: vec2<u32>, gone: u32, relay: bool, life: vec2<u32>, i: u32) -> Step {
    var before = spent;
    var room = w(TICKS - gone, 0u);
    var g = gone;
    if i == 2u {
        before = add64(spent, w(TICKS - gone, 0u));
        room = ticks_whole();
        g = 0u;
    }
    let left = sub64(life, before);
    var share = smin64(room, left);
    if i == 1u && relay {
        share = room;
    }
    return Step(share, !slt64(room, left), add64(w(g, 0u), share));
}

// A signed tick count as the decisions read it: none below none.
fn unsigned_ticks(t: vec2<u32>) -> vec2<u32> {
    return select(t, vec2<u32>(0u), negative(t));
}

fn onto(key: vec2<u32>, pa: u32) -> vec2<u32> {
    return child(child(key, CONTINUATION), w(pa, 0u));
}

@compute @workgroup_size(64)
fn grow(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x + id.y * cfg.row;
    if c >= cfg.n {
        return;
    }
    axis_base = c * cfg.axes_cap * AXW;
    phyt_base = c * cfg.phyt_cap * PW;
    live_base = c * cfg.live_cap * 2u;
    sleep_base = c * (cfg.sleep_cap * SW + cfg.pend_cap * QW + cfg.unit_cap * UW);
    pend_base = sleep_base + cfg.sleep_cap * SW;
    unit_base = pend_base + cfg.pend_cap * QW;
    n_axes = 0u;
    n_phyts = 0u;
    n_sleepers = 0u;
    n_pends = 0u;
    n_units = 0u;
    overflow = 0u;
    depth = 0u;
    next_half = 0u;
    n_next = 0u;
    let b = (cfg.first + c) * BW;
    let lineage = buds[b];
    let pa = buds[b + 1u].x;
    let birth = buds[b + 1u].y;
    let seed = buds[b + 2u].x == 1u;
    let node = buds[b + 2u].y;
    let woken = (buds[b + 5u].x & 1u) != 0u;
    let slot = buds[b + 5u].y;
    let root = buds[b + 3u];
    let horizon = buds[b + 4u];
    let vigour = bitcast<f32>(buds[b + 11u].x);
    let whorl = bitcast<f32>(buds[b + 11u].y);
    var start = 1u;
    if seed {
        push_axis(lineage, pa, 0u, SEED, NONE, 0u, root, horizon, vec2<u32>(0u));
        push_next(0u, vec2<u32>(0u));
    } else if woken {
        // The bearer's stub stands at the bud's horizon.
        push_axis(vec2<u32>(0u), pa, 0u, STUB, NONE, 0u, vec2<u32>(0u), horizon, vec2<u32>(0u));
        start = woke(b, vigour, whorl);
    } else {
        push_axis(vec2<u32>(0u), pa, 0u, STUB, NONE, 0u, vec2<u32>(0u), neg64(inf()), vec2<u32>(0u));
        push_axis(lineage, pa, birth, LATERAL | (slot << 8u), 0u, node, root, horizon, vec2<u32>(0u));
        wrf(1u, MADE, vigour);
        wrf(1u, WHORL, whorl);
        wq(1u, B0, buds[b + 12u]);
        wq(1u, B1, buds[b + 13u]);
        push_next(1u, vec2<u32>(0u));
        start = birth + 1u;
    }
    var cycles = 0u;
    for (var cycle = start; cycle <= cfg.age; cycle++) {
        if overflow != 0u || (n_next == 0u && !asleep_from(cycle)) {
            break;
        }
        // This cycle's living apexes, in order; the next cycle's are made
        // in the other half.
        let half = next_half;
        let count = n_next;
        next_half = 1u - half;
        n_next = 0u;
        for (var k = 0u; k < count; k++) {
            let e = live[live_base + half * cfg.live_cap + k];
            run(Apex(e.x, e.yz), cycle);
        }
        wakes(cycle);
        cycles += 1u;
    }
    for (var i = 0u; i < n_axes; i++) {
        if (rd(i, KIND) & 0xffu) == RELAYED {
            relay_node(i);
        }
    }
    // How much each apex still living at the age lives on (`living`).
    for (var k = 0u; k < n_next; k++) {
        let e = live[live_base + next_half * cfg.live_cap + k];
        living(e.x, e.yz);
        living_q(e.x, e.yz);
    }
    // The buds that woke on it, after its axes, as it bears them.
    for (var k = 0u; k < n_pends; k++) {
        let p = pend_base + k * QW;
        let key = vec2<u32>(sleepers[p], sleepers[p + 1u]);
        let first = lived(sleepers[p + 2u]);
        let kind = LATERAL | IS_WOKEN | (sleepers[p + 5u] << 8u);
        let at = push_axis(onto(key, first), first, sleepers[p + 6u] - 1u, kind, sleepers[p + 3u], sleepers[p + 4u], key, vec2<u32>(0u), vec2<u32>(0u));
        if at == NONE {
            break;
        }
        // Its waking, for its own chain: the share of its waking cycle it
        // slept, the stage it wakes in and its time there, its sleeping
        // bud's PA, and its bearer's horizon.
        wr(at, SLEEP, sleepers[p + 8u]);
        wr(at, ENDED_RELAYS, sleepers[p + 9u]);
        wr64(at, UNITS, vec2<u32>(sleepers[p + 10u], sleepers[p + 11u]));
        wr(at, 31u, sleepers[p + 2u]);
        wr64(at, HORIZON, rd64(sleepers[p + 3u], HORIZON));
        // Its vigour as made: its draw's presence, what carried its
        // bearer to its waking, the share of it reaching its stage, and
        // the share its bearer kept (`Woken::kept`).
        let kept = woken_kept(p);
        let vig = bitcast<f32>(sleepers[p + 15u]) * bitcast<f32>(sleepers[p + 12u])
            * bitcast<f32>(sleepers[p + 13u]) * kept;
        wrf(at, VIG, vig);
        wr(at, WHORL, sleepers[p + 16u]);
        // The same, Q32 (host decision 8), at the chain's end.
        ask(at, vec4<u32>(WOKEN_ASKS, sleepers[p + 19u], sleepers[p + 20u], sleepers[p + 2u]), vec4<u32>(sleepers[p], sleepers[p + 1u], p, 0u));
    }
    assign_q();
    if overflow == 0u {
        keep_all();
    }
    status[c] = vec4<u32>(n_axes, n_phyts, overflow, n_axes - n_pends);
}
