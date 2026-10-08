// The fine roots' curve handed to the renderer (fn-212 task 7, host
// decision 2): `sweep.wgsl`'s points and clusters packed into the
// renderer's own curve form, after the CPU block's (D5's order).
// `points` packs each point as `CurvePoint::pack` does (8 words: the
// centre, radius and distance along in float32, the frame octahedral at
// 24 bits a component); `clusters` packs each cluster as
// `packed_clusters` does (16 words), every index offset by the CPU
// block's, and folds its part of the curve's demand (`Curve::demand`)
// and bounds (`Curve::bounds`); `total` sums the clusters' sags into
// one partial a lane, which the host adds in float64.

struct Cfg {
    n: u32,
    row: u32,
    // The CPU block's points and clusters, which the fine block follows.
    points: u32,
    clusters: u32,
    // The lobes' twist (radians a metre) and depth (`Curve::twisting`).
    twist: f32,
    depth: f32,
    _a: u32,
    _b: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> fine_points: array<f32>;
@group(0) @binding(2) var<storage, read> fine_clusters: array<f32>;
@group(0) @binding(3) var<storage, read> table: array<u32>;
@group(0) @binding(4) var<storage, read_write> points: array<u32>;
@group(0) @binding(5) var<storage, read_write> clusters: array<u32>;
// The stretches, then the bounds' corners as ordered bits, then one
// partial sum of the sags a lane of `total`.
@group(0) @binding(6) var<storage, read_write> sums: array<atomic<u32>>;
@group(0) @binding(7) var<storage, read_write> sags: array<f32>;

// Words of a fine point, cluster and run (`sweep.rs`).
const PW: u32 = 11u;
const KW: u32 = 15u;
const TW: u32 = 4u;
const BITS: u32 = 24u;
const MASK: u32 = 0xffffffu;
const LANES: u32 = 256u;

fn index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

fn at(p: u32, k: u32) -> f32 {
    return fine_points[p * PW + k];
}

fn v3(p: u32, k: u32) -> vec3<f32> {
    return vec3<f32>(at(p, k), at(p, k + 1u), at(p, k + 2u));
}

// `f64::signum` as the CPU's packing takes it: a zero is one, signed as
// its sign bit says (WGSL's `sign` makes it zero; fn-212 task 11).
fn signum(x: f32) -> f32 {
    return select(1.0, -1.0, (bitcast<u32>(x) & 0x80000000u) != 0u);
}

// A unit vector folded onto the octahedron and quantised to two snorm24
// (`pack.rs`, `octahedral`).
fn octahedral(v: vec3<f32>) -> vec2<u32> {
    let l1 = abs(v.x) + abs(v.y) + abs(v.z);
    var x = v.x / l1;
    var y = v.y / l1;
    if v.z < 0.0 {
        let ox = x;
        let oy = y;
        x = (1.0 - abs(oy)) * signum(ox);
        y = (1.0 - abs(ox)) * signum(oy);
    }
    let top = f32((1u << (BITS - 1u)) - 1u);
    let qx = u32(i32(round(clamp(x, -1.0, 1.0) * top))) & MASK;
    let qy = u32(i32(round(clamp(y, -1.0, 1.0) * top))) & MASK;
    return vec2<u32>(qx, qy);
}

@compute @workgroup_size(64)
fn pack(@builtin(global_invocation_id) id: vec3<u32>) {
    let p = index(id);
    if p >= cfg.n {
        return;
    }
    let n = octahedral(v3(p, 5u));
    let b = octahedral(v3(p, 8u));
    let o = (cfg.points + p) * 8u;
    for (var k = 0u; k < 5u; k++) {
        points[o + k] = bitcast<u32>(at(p, k));
    }
    // Four 24-bit components into three words, low first.
    points[o + 5u] = n.x | (n.y << 24u);
    points[o + 6u] = (n.y >> 8u) | (b.x << 16u);
    points[o + 7u] = (b.x >> 16u) | (b.y << 8u);
}

fn tangent(p: u32) -> vec3<f32> {
    return cross(v3(p, 5u), v3(p, 8u));
}

// A stretch's sag (`plan.rs`, `sag`).
fn sag(a: u32, b: u32) -> f32 {
    let ta = tangent(a);
    let tb = tangent(b);
    // The angle between the tangents, well conditioned where they nearly
    // agree (an f32 `acos` of a dot within an ulp of 1 is not).
    let turn = atan2(length(cross(ta, tb)), dot(ta, tb));
    let chord = v3(b, 0u) - v3(a, 0u);
    let span = length(chord);
    let bent = max(
        length(chord - (ta * 2.0 + tb) * (span / 3.0)),
        length(chord - (ta + tb * 2.0) * (span / 3.0)),
    );
    let radius = max(at(a, 3u), at(b, 3u));
    let phase = cfg.twist * abs(at(b, 4u) - at(a, 4u)) * sqrt(cfg.depth);
    return max(span * turn, 6.0 * bent) + radius * phase * phase;
}

// A float's bits, ordered as unsigned integers (`place.wgsl`).
fn ordered(f: f32) -> u32 {
    let u = bitcast<u32>(f);
    return select(~u, u | 0x80000000u, (u & 0x80000000u) == 0u);
}

@compute @workgroup_size(64)
fn pack_clusters(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = index(id);
    if c >= cfg.n {
        return;
    }
    let f = c * KW;
    let run = bitcast<u32>(fine_clusters[f]);
    let first = bitcast<u32>(fine_clusters[f + 1u]);
    let count = bitcast<u32>(fine_clusters[f + 2u]);
    let run_first = table[run * TW];
    let run_count = table[run * TW + 2u];
    let flags = u32(first == run_first) | (u32(first + count == run_first + run_count) << 1u);
    let o = (cfg.clusters + c) * 16u;
    clusters[o] = cfg.points + first;
    clusters[o + 1u] = count;
    clusters[o + 2u] = cfg.points + run_first;
    clusters[o + 3u] = run_count;
    clusters[o + 4u] = flags;
    // Centre, reach and largest radius; then the ladder.
    for (var k = 0u; k < 5u; k++) {
        clusters[o + 5u + k] = bitcast<u32>(fine_clusters[f + 3u + k]);
    }
    for (var k = 0u; k < 6u; k++) {
        clusters[o + 10u + k] = bitcast<u32>(fine_clusters[f + 9u + k]);
    }
    var root = 0.0;
    for (var p = first; p + 1u < first + count; p++) {
        root += sqrt(sag(p, p + 1u));
    }
    sags[c] = root;
    atomicAdd(&sums[0], count - 1u);
    let centre = vec3<f32>(fine_clusters[f + 3u], fine_clusters[f + 4u], fine_clusters[f + 5u]);
    let reach = fine_clusters[f + 6u];
    for (var k = 0u; k < 3u; k++) {
        atomicMin(&sums[1u + k], ordered(centre[k] - reach));
        atomicMax(&sums[4u + k], ordered(centre[k] + reach));
    }
}

// Each lane's partial sum of the clusters' sags, compensated.
@compute @workgroup_size(256)
fn total(@builtin(local_invocation_index) lane: u32) {
    var sum = 0.0;
    var lost = 0.0;
    for (var c = lane; c < cfg.n; c += LANES) {
        let y = sags[c] - lost;
        let t = sum + y;
        lost = (t - sum) - y;
        sum = t;
    }
    atomicStore(&sums[8u + lane], bitcast<u32>(sum));
}
