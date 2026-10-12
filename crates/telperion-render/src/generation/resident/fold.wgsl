// The whole-tree folds over the drawn nodes on the device (fn-212 task 4,
// fn-256 task 3), which the pipeline reads its rows by
// (`pipeline::executor::Folds`): the highest node, the lowest of the crown
// and the widest (`extent`, then `widest`); the shoulder's bins and the
// leaf box (`crown`); and the crown's nodes no other may pass, sifted
// against the bins of the whole tree (`sift`), a bounded summary the
// pipeline reads the shoulder off. One invocation per drawn record; a
// fold is an atomic on the float's ordered bits.

struct Cfg {
    n: u32,
    records: u32,
    cap: u32,
    row: u32,
    // The crown's base, height, widest reach and fullness.
    shape: vec4<f32>,
    // A station's reach from its wood: scale, beyond, rachis, authored.
    reach: vec4<f32>,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> drawn: array<f32>;
@group(0) @binding(2) var<storage, read_write> sums: array<atomic<u32>>;
@group(0) @binding(3) var<storage, read> beyond: array<f32>;
@group(0) @binding(4) var<storage, read_write> kept: array<f32>;

const CV: u32 = 32u;
const MADE: u32 = 2u;
const STEM: u32 = 128u;
const BINS: u32 = 1024u;
// `sums`: the height, the crown's lowest, the widest reach; the widest
// node's position; the box's corners; the nodes kept; the widest node's
// id; then the bins.
const HEIGHT: u32 = 0u;
const LOW: u32 = 1u;
const REACH: u32 = 2u;
const WIDEST: u32 = 3u;
const LO: u32 = 6u;
const HI: u32 = 9u;
const KEPT: u32 = 12u;
const WINNER: u32 = 13u;
const BIN0: u32 = 16u;
// A sifted node may stand this share under the bin beyond it: the
// device's float32 way and reach against the pipeline's.
const SLACK: f32 = 1e-5;

// A float's bits in an order an unsigned compare keeps.
fn ord(f: f32) -> u32 {
    let b = bitcast<u32>(f);
    if (b & 0x80000000u) != 0u {
        return ~b;
    }
    return b | 0x80000000u;
}

// Record `i`'s node, where it is the device's to fold.
fn node(i: u32) -> vec4<f32> {
    let b = cfg.records + i * CV;
    let flags = bitcast<u32>(drawn[b + 6u]);
    if (flags & MADE) == 0u {
        return vec4<f32>(0.0, 0.0, 0.0, -1.0);
    }
    return vec4<f32>(drawn[b], drawn[b + 1u], drawn[b + 2u], 1.0);
}

fn index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

@compute @workgroup_size(64)
fn extent(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n {
        return;
    }
    let n = node(i);
    if n.w < 0.0 {
        return;
    }
    atomicMax(&sums[HEIGHT], ord(max(n.y, 0.0)));
    // The crown's lowest is off the stem (`Folds::of`).
    if (bitcast<u32>(drawn[cfg.records + i * CV + 6u]) & STEM) == 0u {
        atomicMin(&sums[LOW], ord(n.y));
    }
    atomicMax(&sums[REACH], ord(length(n.xz)));
}

@compute @workgroup_size(64)
fn widest(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n {
        return;
    }
    let n = node(i);
    if n.w < 0.0 || ord(length(n.xz)) != atomicLoad(&sums[REACH]) {
        return;
    }
    atomicMin(&sums[WINNER], id_of(i));
}

// The widest node, one of those tied at the widest reach (the least id),
// supplies all its coordinates (fn-212 task 11, Astra's finding 3).
@compute @workgroup_size(64)
fn publish(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n {
        return;
    }
    let n = node(i);
    if n.w < 0.0 || id_of(i) != atomicLoad(&sums[WINNER]) {
        return;
    }
    atomicStore(&sums[WIDEST], bitcast<u32>(n.x));
    atomicStore(&sums[WIDEST + 1u], bitcast<u32>(n.y));
    atomicStore(&sums[WIDEST + 2u], bitcast<u32>(n.z));
}

// Record `i`'s id, unique over the tree's drawn nodes.
fn id_of(i: u32) -> u32 {
    return bitcast<u32>(drawn[cfg.records + i * CV + 14u]);
}

// A crown node's way and reach share, as `crown_point`; none outside.
fn crown(n: vec4<f32>) -> vec2<f32> {
    let base = cfg.shape.x;
    let height = cfg.shape.y;
    let fullness = cfg.shape.w;
    let reach = length(n.xz);
    if n.w < 0.0 || n.y <= base || n.y >= height || reach <= 0.0 {
        return vec2<f32>(-1.0);
    }
    let t = (n.y - base) / (height - base);
    var p = (t - fullness) / (1.0 - fullness);
    if t < fullness {
        p = 1.0 - t / fullness;
    }
    if !(p > 0.0) {
        return vec2<f32>(-1.0);
    }
    return vec2<f32>(min(p, 1.0), reach / cfg.shape.z);
}

fn bin(p: f32) -> u32 {
    return min(u32(p * f32(BINS)), BINS - 1u);
}

@compute @workgroup_size(64)
fn crowns(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n {
        return;
    }
    let n = node(i);
    if n.w < 0.0 {
        return;
    }
    let b = cfg.records + i * CV;
    let wood = max(max(drawn[b + 3u], drawn[b + 4u]), drawn[b + 5u]);
    let r = cfg.reach;
    let out = max(max(max(wood * r.x, r.y) + r.z, 0.0), r.w);
    for (var c = 0u; c < 3u; c++) {
        atomicMin(&sums[LO + c], ord(n[c] - out));
        atomicMax(&sums[HI + c], ord(n[c] + out));
    }
    let pq = crown(n);
    if pq.x >= 0.0 && cfg.shape.z > 0.0 && cfg.shape.y > cfg.shape.x {
        atomicMax(&sums[BIN0 + bin(pq.x)], ord(pq.y));
    }
}

@compute @workgroup_size(64)
fn sift(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n {
        return;
    }
    let n = node(i);
    let pq = crown(n);
    if pq.x < 0.0 || !(cfg.shape.z > 0.0 && cfg.shape.y > cfg.shape.x) {
        return;
    }
    // The bin beyond its own: a float32 way may bin one off.
    let past = beyond[min(bin(pq.x) + 1u, BINS - 1u)];
    if pq.y * (1.0 + SLACK) <= past {
        return;
    }
    let k = atomicAdd(&sums[KEPT], 1u);
    if k < cfg.cap {
        kept[3u * k] = n.x;
        kept[3u * k + 1u] = n.y;
        kept[3u * k + 2u] = n.z;
    }
}
