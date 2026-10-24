// fn-256 tasks 2 and 6: the cycle's exclusive prefix sum of a column, in
// at most two dispatches. A column of `SPAN` records or fewer is scanned
// by one workgroup in `scan_up`, which writes its total. A longer one is
// cut into blocks of `SPAN`: `scan_up` writes each block's total, then
// `scan_down` adds up the totals of the blocks before its own and scans
// its block in place from there; the last block writes the column's
// total. The records are the first `fields` words of records `stride`
// words apart, a field at a time. Integer adds: any order gives the same
// words, so every run writes the same.

struct Cfg {
    // Records in the column, where the blocks' totals start in `sums`
    // (a field's after another's), the dispatch's row of workgroups, the
    // records' stride, the fields scanned, the tally's word of the first
    // field's total, and whether the scan runs: none in a halted cycle,
    // whose recorded words the device left none (`vm.wgsl`).
    n: u32,
    next: u32,
    wx: u32,
    stride: u32,
    fields: u32,
    out: u32,
    on: u32,
    pad1: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read_write> data: array<u32>;
@group(0) @binding(2) var<storage, read_write> sums: array<u32>;
@group(0) @binding(3) var<storage, read_write> totals: array<u32>;

const SPAN: u32 = 1024u;
const LANES: u32 = 256u;

var<workgroup> w_lanes: array<u32, LANES>;

fn word(i: u32, f: u32) -> u32 {
    if i >= cfg.n {
        return 0u;
    }
    return data[i * cfg.stride + f];
}

fn set_word(i: u32, f: u32, v: u32) {
    if i < cfg.n {
        data[i * cfg.stride + f] = v;
    }
}

fn blocks() -> u32 {
    return (cfg.n + SPAN - 1u) / SPAN;
}

// The lanes' `own` scanned across the workgroup: each lane's exclusive
// prefix; the workgroup's total stands in the last lane after it.
fn lanes_scan(lane: u32, own: u32) -> u32 {
    workgroupBarrier();
    w_lanes[lane] = own;
    workgroupBarrier();
    for (var d = 1u; d < LANES; d *= 2u) {
        var add = 0u;
        if lane >= d {
            add = w_lanes[lane - d];
        }
        workgroupBarrier();
        w_lanes[lane] += add;
        workgroupBarrier();
    }
    return w_lanes[lane] - own;
}

// Block `w`'s records of field `f`, four a lane, scanned from `start` and
// written in place where `write`; the block's total.
fn block(w: u32, f: u32, lane: u32, start: u32, write: bool) -> u32 {
    let base = w * SPAN + 4u * lane;
    let v = array<u32, 4>(word(base, f), word(base + 1u, f), word(base + 2u, f), word(base + 3u, f));
    var run = start + lanes_scan(lane, v[0] + v[1] + v[2] + v[3]);
    if write {
        for (var k = 0u; k < 4u; k++) {
            set_word(base + k, f, run);
            run += v[k];
        }
    }
    return w_lanes[LANES - 1u];
}

// A column of one block scanned whole, its total kept; a longer one's
// blocks' totals kept for `scan_down`.
@compute @workgroup_size(256)
fn scan_up(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lane: u32) {
    let w = wid.x + wid.y * cfg.wx;
    let n_blocks = blocks();
    // A workgroup past the column's blocks (a dispatch the host sized from
    // its bounds); the first always runs, for an empty column's total.
    if cfg.on == 0u || (w > 0u && w * SPAN >= cfg.n) {
        return;
    }
    let whole = cfg.n <= SPAN;
    for (var f = 0u; f < cfg.fields; f++) {
        let total = block(w, f, lane, 0u, whole);
        if lane == 0u {
            if whole {
                totals[cfg.out + f] = total;
            } else {
                sums[cfg.next + f * n_blocks + w] = total;
            }
        }
    }
}

// Each block of a longer column scanned in place from the blocks'
// totals before it; the last block's end the column's total.
@compute @workgroup_size(256)
fn scan_down(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lane: u32) {
    let w = wid.x + wid.y * cfg.wx;
    let n_blocks = blocks();
    if cfg.on == 0u || cfg.n <= SPAN || w * SPAN >= cfg.n {
        return;
    }
    for (var f = 0u; f < cfg.fields; f++) {
        let at = cfg.next + f * n_blocks;
        var part = 0u;
        for (var j = lane; j < w; j += LANES) {
            part += sums[at + j];
        }
        _ = lanes_scan(lane, part);
        let before = w_lanes[LANES - 1u];
        let total = block(w, f, lane, before, true);
        if lane == 0u && w + 1u == n_blocks {
            totals[cfg.out + f] = before + total;
        }
    }
}
