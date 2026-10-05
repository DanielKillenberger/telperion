// The wood surfaced from its curve on the GPU (fn-208, host decisions 2, 4
// and 14): the same function as telperion_core's `Curve::tessellate`, cluster
// by cluster, in its order and layout. A pass measures each budget scale,
// one chooses the finest that fits, one counts each cluster at it, three
// scan the counts into offsets, one writes the vertices and indices, and one
// writes the indirect draws.

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
    row: u32,              // per-cluster workgroups a dispatch row
    ribbons: u32,          // 1 coverage ribbons (the camera), 2 at true width (the sun)
    blocks: u32,           // scan blocks of 256 clusters
    ribbon: f32,           // below this radius in pixels a ring is a ribbon's
    pad1: u32,
};

@group(0) @binding(0) var<uniform> cfg: Config;
@group(0) @binding(1) var<storage, read> points: array<u32>;
@group(0) @binding(2) var<storage, read> clusters: array<u32>;
@group(0) @binding(3) var<storage, read> sections: array<f32>;
@group(0) @binding(4) var<storage, read_write> counts: array<vec4<u32>>;
@group(0) @binding(5) var<storage, read_write> offsets: array<vec4<u32>>;
@group(0) @binding(6) var<storage, read_write> blocks: array<vec4<u32>>;
@group(0) @binding(7) var<storage, read_write> totals: array<atomic<u32>, 32>;
@group(0) @binding(8) var<storage, read_write> vertices: array<f32>;
@group(0) @binding(9) var<storage, read_write> indices: array<u32>;
@group(0) @binding(10) var<storage, read_write> args: array<u32, 10>;

const TAU: f32 = 6.28318530718;
const PI: f32 = 3.14159265359;
const MOST_SIDES: u32 = 384u;
const MOST_PIECES: u32 = 256u;
const LEVELS: u32 = 6u;
const POINT_WORDS: u32 = 8u;
const CLUSTER_WORDS: u32 = 16u;
const SECTION_FLOATS: u32 = 28u;
const VERTEX_FLOATS: u32 = 10u;
// totals: [scale * 4 + k] the three counts at each of the four scales; 16 the
// scale chosen; 17 overrun bits (1 no scale fits, 2 a cluster was left out);
// 20..22 the counts at the chosen scale; 24..26 the ends written.
const CHOSEN: u32 = 16u;
const OVERRUN: u32 = 17u;
const ENDS: u32 = 24u;

fn index_of(group: vec3<u32>, local: u32) -> u32 {
    return (group.x + group.y * cfg.row) * 256u + local;
}

// The error scales a budget may draw at: 1, 2, 4 and 8.
fn scale(s: u32) -> f32 {
    return f32(1u << s);
}

@compute @workgroup_size(256)
fn measure(@builtin(workgroup_id) group: vec3<u32>, @builtin(local_invocation_index) local: u32) {
    let i = index_of(group, local);
    if (i >= cfg.clusters) { return; }
    if (!visible(i)) { return; }
    for (var s = 0u; s < 4u; s++) {
        let n = walk(i, cfg.error * scale(s), false, vec3(0u));
        atomicAdd(&totals[s * 4u], n.x);
        atomicAdd(&totals[s * 4u + 1u], n.y);
        atomicAdd(&totals[s * 4u + 2u], n.z);
    }
}

@compute @workgroup_size(1)
fn choose() {
    for (var s = 0u; s < 4u; s++) {
        let fits = atomicLoad(&totals[s * 4u]) <= cfg.vertex_budget
            && atomicLoad(&totals[s * 4u + 1u]) <= cfg.tube_budget
            && atomicLoad(&totals[s * 4u + 2u]) <= cfg.ribbon_budget;
        if (fits) {
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
    var n = vec3(0u);
    if (visible(i)) {
        n = walk(i, cfg.error * scale(atomicLoad(&totals[CHOSEN])), false, vec3(0u));
    }
    counts[i] = vec4(n, 0u);
}

var<workgroup> scan: array<vec3<u32>, 256>;

fn scan_shared(local: u32) {
    for (var offset = 1u; offset < 256u; offset *= 2u) {
        workgroupBarrier();
        var add = vec3(0u);
        if (local >= offset) { add = scan[local - offset]; }
        workgroupBarrier();
        scan[local] += add;
    }
    workgroupBarrier();
}

@compute @workgroup_size(256)
fn scan_local(@builtin(workgroup_id) group: vec3<u32>, @builtin(local_invocation_index) local: u32) {
    let i = index_of(group, local);
    var v = vec3(0u);
    if (i < cfg.clusters) { v = counts[i].xyz; }
    scan[local] = v;
    scan_shared(local);
    if (i < cfg.clusters) { offsets[i] = vec4(scan[local] - v, 0u); }
    let block = group.x + group.y * cfg.row;
    if (local == 255u && block < cfg.blocks) { blocks[block] = vec4(scan[255], 0u); }
}

@compute @workgroup_size(256)
fn scan_blocks(@builtin(local_invocation_index) local: u32) {
    var carry = vec3(0u);
    for (var base = 0u; base < cfg.blocks; base += 256u) {
        let i = base + local;
        var v = vec3(0u);
        if (i < cfg.blocks) { v = blocks[i].xyz; }
        scan[local] = v;
        scan_shared(local);
        if (i < cfg.blocks) { blocks[i] = vec4(scan[local] - v + carry, 0u); }
        carry += scan[255];
        workgroupBarrier();
    }
    if (local == 0u) {
        atomicStore(&totals[20], carry.x);
        atomicStore(&totals[21], carry.y);
        atomicStore(&totals[22], carry.z);
    }
}

@compute @workgroup_size(256)
fn scan_add(@builtin(workgroup_id) group: vec3<u32>, @builtin(local_invocation_index) local: u32) {
    let i = index_of(group, local);
    if (i >= cfg.clusters) { return; }
    offsets[i] += blocks[group.x + group.y * cfg.row];
}

@compute @workgroup_size(256)
fn emit(@builtin(workgroup_id) group: vec3<u32>, @builtin(local_invocation_index) local: u32) {
    let i = index_of(group, local);
    if (i >= cfg.clusters) { return; }
    if (!visible(i)) { return; }
    let o = offsets[i].xyz;
    let n = counts[i].xyz;
    let end = o + n;
    if (end.x > cfg.vertex_budget || end.y > cfg.tube_budget || end.z > cfg.ribbon_budget) {
        atomicOr(&totals[OVERRUN], 2u);
        return;
    }
    _ = walk(i, cfg.error * scale(atomicLoad(&totals[CHOSEN])), true,
        vec3(o.x, o.y, cfg.tube_budget + o.z));
    atomicMax(&totals[ENDS], end.x);
    atomicMax(&totals[ENDS + 1u], end.y);
    atomicMax(&totals[ENDS + 2u], end.z);
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
