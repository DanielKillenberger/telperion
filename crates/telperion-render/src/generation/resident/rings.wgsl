// The fine roots' compact rings on the device (fn-212 task 6): each point
// of the curve `sweep.wgsl` swept as the ring words `compact::read`
// writes, its vertex and cap offsets in the fine block, so the positions
// pass's `emit` lays the vertices a seated leaf's contact reads
// (`stations.wgsl`'s `contact`).

struct Cfg {
    n: u32,
    row: u32,
    segments: u32,
    // The twist's phase a metre along (`TAU * twist_rate / height`).
    twist: f32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> table: array<u32>;
@group(0) @binding(2) var<storage, read> points: array<f32>;
@group(0) @binding(3) var<storage, read_write> rings: array<u32>;

const NONE: u32 = 0xffffffffu;
const TW: u32 = 4u;
const PW: u32 = 11u;
const RINGW: u32 = 16u;

fn f(at: u32) -> u32 {
    return bitcast<u32>(points[at]);
}

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
    for (var i = 0u; i < count; i++) {
        let p = (first + i) * PW;
        let r = (first + i) * RINGW;
        // Centre and radius; normal and along; binormal and phase.
        for (var c = 0u; c < 3u; c++) {
            rings[r + c] = f(p + c);
            rings[r + 4u + c] = f(p + 5u + c);
            rings[r + 8u + c] = f(p + 8u + c);
        }
        rings[r + 3u] = f(p + 3u);
        rings[r + 7u] = f(p + 4u);
        rings[r + 11u] = bitcast<u32>(cfg.twist * points[p + 4u]);
        rings[r + 12u] = base + i * cfg.segments;
        rings[r + 13u] = select(NONE, cap, i == 0u);
        rings[r + 14u] = select(NONE, cap + 1u, i + 1u == count);
        rings[r + 15u] = 0u;
    }
}
