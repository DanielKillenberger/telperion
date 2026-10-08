// The stepped executor's fine roots on the GPU (fn-211 task 10): their
// leaves deposited into the W lattice by 128-bit atomic adds (exact in
// any order), their buds sampling the CPU's one sweep, and each item's
// D16 walked parents first (`grow::stepped::Feedback`,
// `fixed::feedback::shed::walk`), word for word the CPU's.

struct Cfg {
    n: u32,
    start: u32,
    capacity: u32,
    row: u32,
    low: vec4<i32>,
    size: vec4<u32>,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
// Per leaf: x, y, z, area (Q32 each); per point read: x, y, z; per
// axis: parent and decides, q, c, memory, hazard, tolerance, draw,
// window.
@group(0) @binding(1) var<storage, read> rows: array<vec2<u32>>;
// The leaves' order; or per item: its first axis, its count, its root's
// shed, a pad, its root's kept.
@group(0) @binding(2) var<storage, read> order: array<u32>;
@group(0) @binding(3) var<storage, read> light: array<vec2<u32>>;
// Four words a node, its low word first.
@group(0) @binding(4) var<storage, read_write> lattice: array<atomic<u32>>;
// Bit 0: a node past the buffer's capacity; bit 1: an area out of range;
// bit 2: a corner outside the box; bit 3: a balance's light or upkeep
// out of range.
@group(0) @binding(5) var<storage, read_write> flag: atomic<u32>;
// A read's light; or an axis's shed, decided, kept and memory.
@group(0) @binding(6) var<storage, read_write> out: array<vec2<u32>>;

const LEAF: u32 = 4u;
const AXIS: u32 = 8u;
const ITEM: u32 = 6u;
const WALKED: u32 = 3u;
const NONE: u32 = 0xffffffffu;

// v added to the node at word `at`, each word's carry into the next: the
// sum is exact modulo 2^128, so the same in any order.
fn add_atomic(at: u32, v: vec4<u32>) {
    var carry = 0u;
    for (var k = 0u; k < 4u; k++) {
        let add = v[k] + carry;
        carry = select(0u, 1u, add < v[k]);
        if add != 0u {
            let old = atomicAdd(&lattice[at + k], add);
            carry += select(0u, 1u, old + add < old);
        }
    }
}

// The index of node `n` in the box, or none.
fn index_of(n: vec3<i32>) -> u32 {
    let k = n - cfg.low.xyz;
    if any(k < vec3<i32>(0)) || any(vec3<u32>(k) >= cfg.size.xyz) {
        return NONE;
    }
    let u = vec3<u32>(k);
    return (u.z * cfg.size.y + u.y) * cfg.size.x + u.x;
}

fn corner_node(base: array<vec2<u32>, 3>, c: u32) -> u32 {
    var b = base;
    let up = vec3<i32>((vec3<u32>(c) >> vec3<u32>(0u, 1u, 2u)) & vec3<u32>(1u));
    let n = vec3<i32>(bitcast<i32>(b[0].x), bitcast<i32>(b[1].x), bitcast<i32>(b[2].x));
    return index_of(n + up);
}

fn index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

@compute @workgroup_size(64)
fn deposit(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.n {
        return;
    }
    let at = order[cfg.start + k] * LEAF;
    let area = rows[at + 3u];
    if negative(area) || !lt64(area, w(0u, 0x10000u)) {
        atomicOr(&flag, 2u);
        return;
    }
    let base = array<vec2<u32>, 3>(node_of(rows[at]), node_of(rows[at + 1u]), node_of(rows[at + 2u]));
    let d = deposits(area, weights(vec3<u32>(base[0].y, base[1].y, base[2].y)));
    for (var c = 0u; c < 8u; c++) {
        let i = corner_node(base, c);
        if i == NONE {
            atomicOr(&flag, 4u);
        } else if i >= cfg.capacity {
            atomicOr(&flag, 1u);
        } else {
            add_atomic(4u * i, d[c]);
        }
    }
}

@compute @workgroup_size(64)
fn sample(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.n {
        return;
    }
    let r = k * 3u;
    let base = array<vec2<u32>, 3>(node_of(rows[r]), node_of(rows[r + 1u]), node_of(rows[r + 2u]));
    let wt = weights(vec3<u32>(base[0].y, base[1].y, base[2].y));
    var sum = vec4<u32>(0u);
    var inside = vec2<u32>(0u);
    for (var c = 0u; c < 8u; c++) {
        let i = corner_node(base, c);
        if i != NONE {
            sum = add4(sum, mul128(wt[c], light[i]));
            inside = add64(inside, wt[c]);
        }
    }
    sum = add4(sum, mul128(sub64(p_one(), inside), one32()));
    out[k] = shift4(sum, 62u, false, false).xy;
}

// One item's axes, parents first: each inherits its bearer's shedding
// and kept share, its root takes its bearer's decision, and each living
// lateral under a living bearer decides by D16 with its bearer's share.
@compute @workgroup_size(64)
fn shed(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.n {
        return;
    }
    let first = order[k * ITEM];
    for (var i = 0u; i < order[k * ITEM + 1u]; i++) {
        let a = (first + i) * AXIS;
        let parent = rows[a].x;
        var dead = false;
        var keep = one32();
        if parent != NONE {
            let p = (first + parent) * WALKED;
            dead = out[p].x != 0u;
            keep = out[p + 1u];
        }
        var shed = dead;
        var kept = keep;
        var decided = 0u;
        var memory = vec2<u32>(0u);
        if i == 1u {
            shed = order[k * ITEM + 2u] != 0u;
            kept = vec2<u32>(order[k * ITEM + 4u], order[k * ITEM + 5u]);
        } else if !dead && rows[a].y != 0u {
            let b = balance(rows[a + 1u], rows[a + 2u], rows[a + 3u], rows[a + 4u], rows[a + 5u], rows[a + 6u], rows[a + 7u], keep);
            if b.status != 0u {
                atomicOr(&flag, 8u);
            }
            decided = 1u;
            memory = b.memory;
            if b.shed {
                shed = true;
            } else if !is_zero(b.threshold) {
                kept = b.kept;
            }
        }
        let o = (first + i) * WALKED;
        out[o] = vec2<u32>(select(0u, 1u, shed), decided);
        out[o + 1u] = kept;
        out[o + 2u] = memory;
    }
}
