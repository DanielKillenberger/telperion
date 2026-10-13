// An exclusive prefix sum on the device (fn-209 F1, task 5): of the
// first `fields` words of each of `n` records `stride` words apart, in
// place, in blocks of 256 records; the totals after them in `totals`.
// The record kernels' counts become places, and only the totals cross to
// the host, to size the buffers they fill.

struct Cfg {
    n: u32,
    stride: u32,
    fields: u32,
    blocks: u32,
    row: u32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read_write> data: array<u32>;
@group(0) @binding(2) var<storage, read_write> sums: array<u32>;
@group(0) @binding(3) var<storage, read_write> totals: array<u32>;

const BLOCK: u32 = 256u;
const MOST: u32 = 4u;

// Each block's records summed in turn, the block's sums kept.
@compute @workgroup_size(64)
fn blocks(@builtin(global_invocation_id) id: vec3<u32>) {
    let b = id.x;
    if b >= cfg.blocks {
        return;
    }
    var run = array<u32, MOST>(0u, 0u, 0u, 0u);
    let end = min((b + 1u) * BLOCK, cfg.n);
    for (var i = b * BLOCK; i < end; i++) {
        for (var f = 0u; f < cfg.fields; f++) {
            let at = i * cfg.stride + f;
            let v = data[at];
            data[at] = run[f];
            run[f] += v;
        }
    }
    for (var f = 0u; f < cfg.fields; f++) {
        sums[b * MOST + f] = run[f];
    }
}

// The blocks' sums scanned by one invocation; the totals.
@compute @workgroup_size(1)
fn spine() {
    var run = array<u32, MOST>(0u, 0u, 0u, 0u);
    for (var b = 0u; b < cfg.blocks; b++) {
        for (var f = 0u; f < cfg.fields; f++) {
            let v = sums[b * MOST + f];
            sums[b * MOST + f] = run[f];
            run[f] += v;
        }
    }
    for (var f = 0u; f < cfg.fields; f++) {
        totals[f] = run[f];
    }
}

// Each record moved on by its block's place.
@compute @workgroup_size(64)
fn spread(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x + id.y * cfg.row;
    if i >= cfg.n {
        return;
    }
    let b = i / BLOCK;
    for (var f = 0u; f < cfg.fields; f++) {
        data[i * cfg.stride + f] += sums[b * MOST + f];
    }
}
