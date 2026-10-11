// fn-256 task 2 (DESIGN.md section 2, steps 1 and 2): one cycle's
// balance and allotment over the whole tree (`passes.rs`'s
// `balance_one` and `allot_one`, each in their order). The sums run by
// block level, a workgroup a block (`hier_sweep.wgsl`); the living apexes
// are marked, shed by a flag, a scan and a compaction, given their
// presence and their sizes; the normaliser's sums are taken over the
// growing buds in subtree order, 1,024 a block, the blocks in order.
// A product an add takes is written `hm`, a quotient `hdiv`: the sites
// where the driver may fuse or round otherwise than the CPU (UNITS.md),
// each plain or held to the CPU's as the kernels are built (`Arith`).

struct Cfg {
    n: u32,
    stride: u32,
    live: u32,
    live_stride: u32,
    cycle: u32,
    pas: u32,
    level: u32,
    row: u32,
    wx: u32,
    // A word the driver cannot see is zero (`exact.wgsl`).
    zero: u32,
    // The block levels the hierarchy holds: a sweep's level past them
    // (a dispatch the host sized from its bounds) has no blocks.
    levels: u32,
    pad2: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> tree: array<u32>;
@group(0) @binding(2) var<storage, read> rows: array<f32>;
@group(0) @binding(3) var<storage, read> lists: array<u32>;
@group(0) @binding(4) var<storage, read> shape: array<u32>;
@group(0) @binding(5) var<storage, read_write> sums: array<u32>;
@group(0) @binding(6) var<storage, read_write> live: array<u32>;
@group(0) @binding(7) var<storage, read_write> norm: array<u32>;
@group(0) @binding(8) var<storage, read_write> tally: array<atomic<u32>>;

// The sums' columns, per axis: lit (a living apex), the balance's light,
// upkeep and living, shed and kept share by the walk, memory, kept and
// size as the cycle leaves them, the allotment's own lit presence, its
// presence and whether it grows, its subtree's lit presence and growing
// buds, its incoming vigour and its bud's, its first growing bud's place
// in subtree order, and whether its apex was shed; and per child slot,
// the light standing above it.
const S_LIT: u32 = 0u;
const S_LIGHT: u32 = 1u;
const S_COST: u32 = 2u;
const S_LIVING: u32 = 3u;
const S_SHED: u32 = 4u;
const S_KEEP: u32 = 5u;
const S_MEMORY: u32 = 6u;
const S_KEPT: u32 = 7u;
const S_SIZE: u32 = 8u;
const S_OWN: u32 = 9u;
const S_PRESENCE: u32 = 10u;
const S_GROWING: u32 = 11u;
const S_SUB: u32 = 12u;
const S_BUDS: u32 = 13u;
const S_INCOMING: u32 = 14u;
const S_BUD: u32 = 15u;
const S_RANK: u32 = 16u;
const S_REST: u32 = 17u;
const S_GONE: u32 = 18u;

// The living apexes' columns: the cycle's, their flags, and the living
// after the balance; each an axis and its time's two words.
const V_AXIS: u32 = 0u;
const V_SPENT: u32 = 1u;
const V_FLAG: u32 = 3u;
const V_OUT_AXIS: u32 = 4u;
const V_OUT_SPENT: u32 = 5u;

// A PA's row: upkeep, balance hazard, tolerance, window, apical control
// and ψ.
const P_UPKEEP: u32 = 0u;
const P_HAZARD: u32 = 1u;
const P_TOLERANCE: u32 = 2u;
const P_WINDOW: u32 = 3u;
const P_APICAL: u32 = 4u;
const P_PSI: u32 = 5u;

// The normaliser: each PA's sums, the growing buds in subtree order,
// then each block's sums per PA.
const N_SUMS: u32 = 0u;
const N_ORDER: u32 = 128u;
const BUD_BLOCK: u32 = 1024u;

// The tally: the living after the balance (the scan's total) and the
// growing buds.
const Y_LIVE: u32 = 0u;
const Y_GROWING: u32 = 64u;

const HLOG2E: f32 = 0x1.715476p+0f;

fn hm(a: f32, b: f32) -> f32 {
    return exact_kept(a * b);
}

fn hdiv(a: f32, b: f32) -> f32 {
    return exact_div(a, b);
}

fn exact_zero() -> u32 {
    return cfg.zero;
}

fn at(col: u32, i: u32) -> u32 {
    return col * cfg.stride + i;
}

fn ls(col: u32, i: u32) -> u32 {
    return col * (cfg.stride + 64u) + i;
}

fn lv(col: u32, k: u32) -> u32 {
    return col * cfg.live_stride + k;
}

fn index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

fn tf(col: u32, i: u32) -> f32 {
    return bitcast<f32>(tree[at(col, i)]);
}

fn wide(col: u32, i: u32) -> vec2<u32> {
    return vec2<u32>(tree[at(col, i)], tree[at(col + 1u, i)]);
}

fn sf(col: u32, i: u32) -> f32 {
    return bitcast<f32>(sums[at(col, i)]);
}

fn put(col: u32, i: u32, x: f32) {
    sums[at(col, i)] = bitcast<u32>(x);
}

fn row(pa: u32, k: u32) -> f32 {
    return rows[8u * pa + k];
}

fn kid(k: u32) -> u32 {
    return lists[ls(R_KIDS, k)];
}

fn part(k: u32) -> u32 {
    return N_ORDER + cfg.stride + k;
}

// Each living apex laid marks its axis lit.
@compute @workgroup_size(64)
fn mark_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.live {
        return;
    }
    let a = live[lv(V_AXIS, k)];
    if a < cfg.n {
        sums[at(S_LIT, a)] = 1u;
    }
}

// Whether the living apex at `k` lives on: unlaid, or not shed.
fn lives(k: u32) -> bool {
    let a = live[lv(V_AXIS, k)];
    return a >= cfg.n || sums[at(S_SHED, a)] == 0u;
}

// Each living apex shed or given its kept share, and flagged to stay.
@compute @workgroup_size(64)
fn shed_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.live {
        return;
    }
    let a = live[lv(V_AXIS, k)];
    let stays = lives(k);
    if a < cfg.n {
        if stays {
            sums[at(S_KEPT, a)] = sums[at(S_KEEP, a)];
        } else {
            sums[at(S_GONE, a)] = 1u;
        }
    }
    live[lv(V_FLAG, k)] = u32(stays);
}

// The living that stay, compacted in order by the scan of the flags.
@compute @workgroup_size(64)
fn compact_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.live || !lives(k) {
        return;
    }
    let to = live[lv(V_FLAG, k)];
    live[lv(V_OUT_AXIS, to)] = live[lv(V_AXIS, k)];
    live[lv(V_OUT_SPENT, to)] = live[lv(V_SPENT, k)];
    live[lv(V_OUT_SPENT + 1u, to)] = live[lv(V_SPENT + 1u, k)];
}

// Each living apex's presence, `base_scale × running × kept`, its lit
// share, that times its light, each in f64 then as f32; and whether it
// grows (its presence above none), counted.
@compute @workgroup_size(64)
fn presence_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if cfg.cycle == 0u || k >= atomicLoad(&tally[Y_LIVE]) {
        return;
    }
    let a = live[lv(V_OUT_AXIS, k)];
    if a >= cfg.n {
        return;
    }
    let scale = mul_real(real64(tf(T_BASE, a)), real64(tf(T_RUNNING, a)));
    let p = mul_real(scale, real64(sf(S_KEPT, a)));
    put(S_PRESENCE, a, single(p));
    put(S_OWN, a, single(mul_real(p, wide(T_LIGHT, a))));
    if !is_zero(p) {
        sums[at(S_GROWING, a)] = 1u;
        atomicAdd(&tally[Y_GROWING], 1u);
    }
}

// `allocation::drawn`: (v/p)^ψ.
fn drawn(v: f32, p: f32, psi: f32) -> f32 {
    if p <= 0.0 || psi == 0.0 {
        return 1.0;
    }
    if v <= 0.0 {
        return select(INF, 0.0, psi > 0.0);
    }
    return exp2c(psi * (log2c(v) - log2c(p)));
}

// One block of 1,024 growing buds in subtree order: each PA's Σp and
// Σp(v/p)^ψ, a thread a PA, each summed in order.
@compute @workgroup_size(64)
fn partials_main(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) pa: u32) {
    let beta = wid.x + wid.y * cfg.wx;
    let g = atomicLoad(&tally[Y_GROWING]);
    let s = beta * BUD_BLOCK;
    if s >= g || pa >= cfg.pas || cfg.cycle == 0u {
        return;
    }
    var total = 0.0;
    var weighted = 0.0;
    for (var k = s; k < min(s + BUD_BLOCK, g); k++) {
        let i = norm[N_ORDER + k];
        if tree[at(T_PA, i)] != pa {
            continue;
        }
        let p = sf(S_PRESENCE, i);
        total = total + p;
        weighted = weighted + hm(drawn(sf(S_BUD, i), p, row(pa, P_PSI)), p);
    }
    norm[part(2u * (beta * cfg.pas + pa))] = bitcast<u32>(total);
    norm[part(2u * (beta * cfg.pas + pa) + 1u)] = bitcast<u32>(weighted);
}

// Each PA's sums: its blocks' added in order.
@compute @workgroup_size(64)
fn combine_main(@builtin(local_invocation_index) pa: u32) {
    if pa >= cfg.pas {
        return;
    }
    let blocks = (atomicLoad(&tally[Y_GROWING]) + BUD_BLOCK - 1u) / BUD_BLOCK;
    var total = 0.0;
    var weighted = 0.0;
    for (var b = 0u; b < blocks; b++) {
        total = total + bitcast<f32>(norm[part(2u * (b * cfg.pas + pa))]);
        weighted = weighted + bitcast<f32>(norm[part(2u * (b * cfg.pas + pa) + 1u)]);
    }
    norm[N_SUMS + 2u * pa] = bitcast<u32>(total);
    norm[N_SUMS + 2u * pa + 1u] = bitcast<u32>(weighted);
}

// Each living apex's size against its PA's sums (`allocation::size_of`).
@compute @workgroup_size(64)
fn sizes_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if cfg.cycle == 0u || k >= atomicLoad(&tally[Y_LIVE]) {
        return;
    }
    let a = live[lv(V_OUT_AXIS, k)];
    if a >= cfg.n {
        return;
    }
    let pa = tree[at(T_PA, a)];
    let p = sf(S_PRESENCE, a);
    let weighted = bitcast<f32>(norm[N_SUMS + 2u * pa + 1u]);
    if p <= 0.0 || weighted <= 0.0 {
        put(S_SIZE, a, 1.0);
        return;
    }
    let total = bitcast<f32>(norm[N_SUMS + 2u * pa]);
    put(S_SIZE, a, hdiv(hm(drawn(sf(S_BUD, a), p, row(pa, P_PSI)), total), weighted));
}
