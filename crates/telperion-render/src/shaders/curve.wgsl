// The wood surfaced from its curve on the GPU (fn-208, host decisions 2, 4,
// 14 and 16 to 21): the same function as telperion_core's `Curve::tessellate`,
// cluster by cluster, in its order and layout. A pass measures each budget
// scale, one chooses the finest that fits, one counts each cluster at it,
// three scan the counts into offsets, one walks each cluster into ring
// records, one writes every ring's vertices and indices, one thread a ring,
// and one writes the indirect draws.
//
// Every buffer larger than a storage binding may hold is bound in slices:
// the points, the ring records, the vertices and the indices in two each,
// which keeps the emitting pass within WebGPU's eight storage buffers.

struct Config {
    eye: vec4<f32>,        // xyz; w the near plane
    forward: vec4<f32>,    // xyz; w pixels a metre at one metre's depth
    planes: array<vec4<f32>, 6>,
    error: f32,
    lobe_depth: f32,
    twist_rate: f32,
    height: f32,
    lobes: u32,
    orthographic: u32,
    culling: u32,
    clusters: u32,
    vertex_budget: u32,
    tube_budget: u32,
    ribbon_budget: u32,
    ring_budget: u32,
    row: u32,              // per-cluster workgroups a dispatch row
    blocks: u32,           // scan blocks of 256 clusters
    ribbon: f32,           // below this radius in pixels a ring is a ribbon's
    point_slice: u32,      // words of points in the first binding
    vertex_slice: u32,     // floats of vertices in the first binding
    index_slice: u32,      // indices in the first binding
    record_slice: u32,     // ring records in the first binding
    pad1: u32,
};

@group(0) @binding(0) var<uniform> cfg: Config;
@group(0) @binding(1) var<storage, read> points0: array<u32>;
@group(0) @binding(2) var<storage, read> points1: array<u32>;
@group(0) @binding(3) var<storage, read> clusters: array<u32>;
@group(0) @binding(4) var<storage, read> sections: array<f32>;
@group(0) @binding(5) var<storage, read_write> counts: array<vec4<u32>>;
@group(0) @binding(6) var<storage, read_write> offsets: array<vec4<u32>>;
@group(0) @binding(7) var<storage, read_write> blocks: array<vec4<u32>>;
@group(0) @binding(8) var<storage, read_write> totals: array<atomic<u32>, 48>;
@group(0) @binding(9) var<storage, read_write> records0: array<u32>;
@group(0) @binding(12) var<storage, read_write> records1: array<u32>;
@group(0) @binding(10) var<storage, read_write> vertices0: array<f32>;
@group(0) @binding(11) var<storage, read_write> vertices1: array<f32>;
@group(0) @binding(13) var<storage, read_write> indices0: array<u32>;
@group(0) @binding(14) var<storage, read_write> indices1: array<u32>;
@group(0) @binding(15) var<storage, read_write> args: array<u32, 16>;

const TAU: f32 = 6.28318530718;
const PI: f32 = 3.14159265359;
const MOST_SIDES: u32 = 384u;
const MOST_PIECES: u32 = 256u;
const LEVELS: u32 = 6u;
const POINT_WORDS: u32 = 8u;
const CLUSTER_WORDS: u32 = 16u;
const SECTION_FLOATS: u32 = 28u;
const VERTEX_FLOATS: u32 = 9u;
const RECORD_WORDS: u32 = 22u;
// totals: [scale * 4 + k] the four counts (rings, vertices, tube indices,
// ribbon indices) at each of the four scales; 16 the scale chosen; 17
// overrun bits (1 no scale fits, 2 a cluster was left out); 20..23 the
// counts at the chosen scale; 24..26 the ends written (vertices, tube
// indices, ribbon indices).
const CHOSEN: u32 = 16u;
const OVERRUN: u32 = 17u;
const CHOSEN_COUNTS: u32 = 20u;
const ENDS: u32 = 24u;

// Word `k` of ring `r`'s record, in whichever slice holds the ring.
fn record(r: u32, k: u32) -> u32 {
    if (r < cfg.record_slice) { return records0[r * RECORD_WORDS + k]; }
    return records1[(r - cfg.record_slice) * RECORD_WORDS + k];
}

fn set_record(r: u32, k: u32, value: u32) {
    if (r < cfg.record_slice) {
        records0[r * RECORD_WORDS + k] = value;
    } else {
        records1[(r - cfg.record_slice) * RECORD_WORDS + k] = value;
    }
}

fn index_of(group: vec3<u32>, local: u32) -> u32 {
    return (group.x + group.y * cfg.row) * 256u + local;
}

// The error scales a budget may draw at: 1, 2, 4 and 8.
fn scale(s: u32) -> f32 {
    return f32(1u << s);
}

fn fits(n: vec4<u32>) -> bool {
    return n.x <= cfg.ring_budget && n.y <= cfg.vertex_budget && n.z <= cfg.tube_budget
        && n.w <= cfg.ribbon_budget;
}

@compute @workgroup_size(256)
fn measure(@builtin(workgroup_id) group: vec3<u32>, @builtin(local_invocation_index) local: u32) {
    let i = index_of(group, local);
    if (i >= cfg.clusters || !visible(i)) { return; }
    for (var s = 0u; s < 4u; s++) {
        let n = walk(i, cfg.error * scale(s), false, vec4(0u));
        for (var k = 0u; k < 4u; k++) { atomicAdd(&totals[s * 4u + k], n[k]); }
    }
}

@compute @workgroup_size(1)
fn choose() {
    for (var s = 0u; s < 4u; s++) {
        let n = vec4(atomicLoad(&totals[s * 4u]), atomicLoad(&totals[s * 4u + 1u]),
            atomicLoad(&totals[s * 4u + 2u]), atomicLoad(&totals[s * 4u + 3u]));
        if (fits(n)) {
            atomicStore(&totals[CHOSEN], s);
            return;
        }
    }
    atomicStore(&totals[CHOSEN], 3u);
    atomicOr(&totals[OVERRUN], 1u);
}

@compute @workgroup_size(256)
fn count(@builtin(workgroup_id) group: vec3<u32>, @builtin(local_invocation_index) local: u32) {
    let i = index_of(group, local);
    if (i >= cfg.clusters) { return; }
    var n = vec4(0u);
    if (visible(i)) {
        n = walk(i, cfg.error * scale(atomicLoad(&totals[CHOSEN])), false, vec4(0u));
    }
    counts[i] = n;
}

var<workgroup> scan: array<vec4<u32>, 256>;

fn scan_shared(local: u32) {
    for (var offset = 1u; offset < 256u; offset *= 2u) {
        workgroupBarrier();
        var add = vec4(0u);
        if (local >= offset) { add = scan[local - offset]; }
        workgroupBarrier();
        scan[local] += add;
    }
    workgroupBarrier();
}

@compute @workgroup_size(256)
fn scan_local(@builtin(workgroup_id) group: vec3<u32>, @builtin(local_invocation_index) local: u32) {
    let i = index_of(group, local);
    var v = vec4(0u);
    if (i < cfg.clusters) { v = counts[i]; }
    scan[local] = v;
    scan_shared(local);
    if (i < cfg.clusters) { offsets[i] = scan[local] - v; }
    let block = group.x + group.y * cfg.row;
    if (local == 255u && block < cfg.blocks) { blocks[block] = scan[255]; }
}

@compute @workgroup_size(256)
fn scan_blocks(@builtin(local_invocation_index) local: u32) {
    var carry = vec4(0u);
    for (var base = 0u; base < cfg.blocks; base += 256u) {
        let i = base + local;
        var v = vec4(0u);
        if (i < cfg.blocks) { v = blocks[i]; }
        scan[local] = v;
        scan_shared(local);
        if (i < cfg.blocks) { blocks[i] = scan[local] - v + carry; }
        carry += scan[255];
        workgroupBarrier();
    }
    if (local == 0u) {
        for (var k = 0u; k < 4u; k++) { atomicStore(&totals[CHOSEN_COUNTS + k], carry[k]); }
        // One thread a ring, rows of at most 65,535 workgroups.
        let rings = min(carry.x, cfg.ring_budget);
        let groups = max((rings + 255u) / 256u, 1u);
        args[10] = min(groups, 65535u);
        args[11] = (groups + 65534u) / 65535u;
        args[12] = 1u;
    }
}

@compute @workgroup_size(256)
fn scan_add(@builtin(workgroup_id) group: vec3<u32>, @builtin(local_invocation_index) local: u32) {
    let i = index_of(group, local);
    if (i >= cfg.clusters) { return; }
    offsets[i] += blocks[group.x + group.y * cfg.row];
}

// Each cluster's rings into their records, at the offsets the scan gave it;
// a cluster past the budget writes empty records, its rings drawn as none.
@compute @workgroup_size(256)
fn rings(@builtin(workgroup_id) group: vec3<u32>, @builtin(local_invocation_index) local: u32) {
    let i = index_of(group, local);
    if (i >= cfg.clusters || !visible(i)) { return; }
    let o = offsets[i];
    let n = counts[i];
    if (!fits(o + n)) {
        atomicOr(&totals[OVERRUN], 2u);
        for (var k = 0u; k < n.x; k++) {
            if (o.x + k < cfg.ring_budget) { set_record(o.x + k, 13u, 0u); }
        }
        return;
    }
    _ = walk(i, cfg.error * scale(atomicLoad(&totals[CHOSEN])), true,
        vec4(o.x, o.y, o.z, cfg.tube_budget + o.w));
    let end = o + n;
    atomicMax(&totals[ENDS], end.y);
    atomicMax(&totals[ENDS + 1u], end.z);
    atomicMax(&totals[ENDS + 2u], end.w);
}

// One ring's vertices and indices, from its record.
@compute @workgroup_size(256)
fn emit(@builtin(workgroup_id) group: vec3<u32>, @builtin(num_workgroups) groups: vec3<u32>,
    @builtin(local_invocation_index) local: u32) {
    let r = (group.x + group.y * groups.x) * 256u + local;
    let rings = min(atomicLoad(&totals[CHOSEN_COUNTS]), cfg.ring_budget);
    if (r >= rings) { return; }
    emit_ring(r);
}

@compute @workgroup_size(1)
fn draws() {
    args[0] = atomicLoad(&totals[ENDS + 1u]);
    args[1] = 1u;
    args[2] = 0u;
    args[3] = 0u;
    args[4] = 0u;
    args[5] = atomicLoad(&totals[ENDS + 2u]);
    args[6] = 1u;
    args[7] = cfg.tube_budget;
    args[8] = 0u;
    args[9] = 0u;
}
