// The feedback slice on the GPU (fn-211 task 8): leaves deposited into
// the W lattice by 128-bit atomic adds (exact in any order), and buds
// sampling the CPU's swept light with D16 after it
// (`fixed::feedback::slice`, `fine/feedback_slice.rs`).

struct Cfg {
    n: u32,
    start: u32,
    capacity: u32,
    row: u32,
    low: vec4<i32>,
    size: vec4<u32>,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
// Per leaf: x, y, z, scale, size, area (Q32 each).
@group(0) @binding(1) var<storage, read> leaves: array<vec2<u32>>;
@group(0) @binding(2) var<storage, read> order: array<u32>;
// Per read: x, y, z, presence, c, memory, hazard, tolerance, draw,
// window, keep.
@group(0) @binding(3) var<storage, read> reads: array<vec2<u32>>;
@group(0) @binding(4) var<storage, read> light: array<vec2<u32>>;
// Four words a node, its low word first.
@group(0) @binding(5) var<storage, read_write> lattice: array<atomic<u32>>;
// Bit 0: a node past the buffer's capacity; bit 1: an area out of range;
// bit 2: a corner outside the box.
@group(0) @binding(6) var<storage, read_write> flag: atomic<u32>;
// Per read: light, memory, threshold, shed, kept, status.
@group(0) @binding(7) var<storage, read_write> out: array<vec2<u32>>;

const LEAF: u32 = 6u;
const READ: u32 = 11u;
const OUT: u32 = 6u;

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

// The index of node `n` in the box, or none (0xffffffff).
fn index_of(n: vec3<i32>) -> u32 {
    let k = n - cfg.low.xyz;
    if any(k < vec3<i32>(0)) || any(vec3<u32>(k) >= cfg.size.xyz) {
        return 0xffffffffu;
    }
    let u = vec3<u32>(k);
    return (u.z * cfg.size.y + u.y) * cfg.size.x + u.x;
}

// The node and Q31 fraction of a Q32 point.
fn corner_base(p: array<vec2<u32>, 3>) -> array<vec2<u32>, 3> {
    var q = p;
    return array<vec2<u32>, 3>(node_of(q[0]), node_of(q[1]), node_of(q[2]));
}

fn corner_node(base: array<vec2<u32>, 3>, c: u32) -> u32 {
    var b = base;
    let up = vec3<i32>((vec3<u32>(c) >> vec3<u32>(0u, 1u, 2u)) & vec3<u32>(1u));
    let n = vec3<i32>(bitcast<i32>(b[0].x), bitcast<i32>(b[1].x), bitcast<i32>(b[2].x));
    return index_of(n + up);
}

@compute @workgroup_size(64)
fn deposit(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = id.x + id.y * cfg.row;
    if k >= cfg.n {
        return;
    }
    let at = order[cfg.start + k] * LEAF;
    let area = prod_shr(leaves[at + 3u], leaves[at + 4u], leaves[at + 5u], w(1u, 0u), 64u);
    if area.status != 0u || negative(area.v.xy) || !lt64(area.v.xy, w(0u, 0x10000u)) {
        atomicOr(&flag, 2u);
        return;
    }
    let base = corner_base(array<vec2<u32>, 3>(leaves[at], leaves[at + 1u], leaves[at + 2u]));
    let d = deposits(area.v.xy, weights(vec3<u32>(base[0].y, base[1].y, base[2].y)));
    for (var c = 0u; c < 8u; c++) {
        let i = corner_node(base, c);
        if i == 0xffffffffu {
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
    let k = id.x + id.y * cfg.row;
    if k >= cfg.n {
        return;
    }
    let r = k * READ;
    let base = corner_base(array<vec2<u32>, 3>(reads[r], reads[r + 1u], reads[r + 2u]));
    let wt = weights(vec3<u32>(base[0].y, base[1].y, base[2].y));
    var sum = vec4<u32>(0u);
    var inside = vec2<u32>(0u);
    for (var c = 0u; c < 8u; c++) {
        let i = corner_node(base, c);
        if i != 0xffffffffu {
            sum = add4(sum, mul128(wt[c], light[i]));
            inside = add64(inside, wt[c]);
        }
    }
    sum = add4(sum, mul128(sub64(p_one(), inside), one32()));
    let lit = shift4(sum, 62u, false, false).xy;
    let q = mulq(reads[r + 3u], lit);
    let b = balance(q, reads[r + 4u], reads[r + 5u], reads[r + 6u], reads[r + 7u], reads[r + 8u], reads[r + 9u], reads[r + 10u]);
    let o = k * OUT;
    out[o] = lit;
    out[o + 1u] = b.memory;
    out[o + 2u] = b.threshold;
    out[o + 3u] = w(select(0u, 1u, b.shed), 0u);
    out[o + 4u] = b.kept;
    out[o + 5u] = w(b.status, 0u);
}
