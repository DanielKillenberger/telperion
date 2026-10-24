// The tree's compact rings on the device (fn-212 task 6): each point
// of the curve `sweep.wgsl` swept, in the renderer's packed form, as the
// ring words `compact::read` writes, its frame read back as the renderer
// reads it (`pack.wgsl`; fn-293), its vertex and cap offsets in the fine
// block, so the positions pass's `emit` lays the vertices a seated leaf's
// contact reads (`stations.wgsl`'s `contact`).

struct Cfg {
    n: u32,
    row: u32,
    segments: u32,
    // The twist's phase a metre along (`TAU * twist_rate / height`).
    twist: f32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> table: array<u32>;
@group(0) @binding(2) var<storage, read> points: array<u32>;
@group(0) @binding(3) var<storage, read_write> rings: array<u32>;
@group(0) @binding(4) var<storage, read> clusters: array<u32>;

const NONE: u32 = 0xffffffffu;
const TW: u32 = 4u;
const RINGW: u32 = 16u;
const CLUSTER_WORDS: u32 = 16u;

// One run's rings, base first: the run's vertices start after its
// forebears' rings and two caps each.
@compute @workgroup_size(64)
fn rings_of(@builtin(global_invocation_id) id: vec3<u32>) {
    let s = id.x + id.y * cfg.row;
    if s >= cfg.n {
        return;
    }
    let first = table[s * TW];
    let count = table[s * TW + 2u];
    let base = first * cfg.segments + 2u * s;
    let cap = base + count * cfg.segments;
    // A trunk's run (its first cluster's flag 8, `sweep.wgsl`'s `standing`).
    let trunk = clusters[table[s * TW + 1u] * CLUSTER_WORDS + 4u] & 8u;
    for (var i = 0u; i < count; i++) {
        let p = (first + i) * POINT_WORDS;
        let r = (first + i) * RINGW;
        let frame = vec3<u32>(points[p + 5u], points[p + 6u], points[p + 7u]);
        let normal = frame_normal(frame);
        let binormal = frame_binormal(frame);
        // Centre and radius; normal and along; binormal and phase.
        for (var c = 0u; c < 3u; c++) {
            rings[r + c] = points[p + c];
            rings[r + 4u + c] = bitcast<u32>(normal[c]);
            rings[r + 8u + c] = bitcast<u32>(binormal[c]);
        }
        rings[r + 3u] = points[p + 3u];
        rings[r + 7u] = points[p + 4u];
        rings[r + 11u] = bitcast<u32>(cfg.twist * bitcast<f32>(points[p + 4u]));
        rings[r + 12u] = base + i * cfg.segments;
        rings[r + 13u] = select(NONE, cap, i == 0u);
        rings[r + 14u] = select(NONE, cap + 1u, i + 1u == count);
        rings[r + 15u] = trunk >> 3u;
    }
}
