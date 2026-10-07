// The chains' regions packed end to end (fn-209 F1, task 2): each chain's
// axes and phytomers from its region to its offsets, so a reader takes
// what grew and none of the room it did not use. A workgroup a chain, its
// threads copying neighbouring words (task 8: a thread a chain took 78 ms
// of the spruce 80's build).

struct Cfg {
    n: u32,
    row: u32,
    axes_cap: u32,
    phyt_cap: u32,
    stride: u32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> status: array<vec4<u32>>;
@group(0) @binding(2) var<storage, read> offsets: array<u32>;
@group(0) @binding(3) var<storage, read> axes: array<u32>;
@group(0) @binding(4) var<storage, read> phyts: array<u32>;
@group(0) @binding(5) var<storage, read_write> packed_axes: array<u32>;
@group(0) @binding(6) var<storage, read_write> packed_phyts: array<u32>;

const AXW: u32 = 80u;
const PW: u32 = 11u;

const LANES: u32 = 64u;

@compute @workgroup_size(64)
fn pack(@builtin(workgroup_id) wg: vec3<u32>, @builtin(local_invocation_id) lane: vec3<u32>) {
    let c = wg.x + wg.y * cfg.row;
    if c >= cfg.n || status[c].z != 0u {
        return;
    }
    let to = vec2<u32>(offsets[c * cfg.stride], offsets[c * cfg.stride + 1u]);
    let axes_from = c * cfg.axes_cap * AXW;
    for (var k = lane.x; k < status[c].x * AXW; k += LANES) {
        packed_axes[to.x * AXW + k] = axes[axes_from + k];
    }
    let phyts_from = c * cfg.phyt_cap * PW;
    for (var k = lane.x; k < status[c].y * PW; k += LANES) {
        packed_phyts[to.y * PW + k] = phyts[phyts_from + k];
    }
}
