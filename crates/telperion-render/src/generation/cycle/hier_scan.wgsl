// fn-256 task 2: the hierarchy's exclusive prefix sum, in levels. Each
// workgroup scans 256 words of a level in place and writes their total
// to the next level, until one workgroup holds the top, whose total is
// the scan's; each level then adds the next level's scanned totals back.
// Integer adds in a fixed shape: the same words on every run, as
// `fine/scan.wgsl` gives, with no one-invocation spine.

struct Cfg {
    // Words in this level, where it starts (in `sums` past the first),
    // where the next starts in `sums`, whether this is the first level
    // (`data`) and whether it is the top.
    n: u32,
    at: u32,
    next: u32,
    first: u32,
    top: u32,
    wx: u32,
    pad0: u32,
    pad1: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read_write> data: array<u32>;
@group(0) @binding(2) var<storage, read_write> sums: array<u32>;
@group(0) @binding(3) var<storage, read_write> totals: array<u32>;

const SPAN: u32 = 256u;
const LANES: u32 = 64u;

var<workgroup> w_lanes: array<u32, LANES>;

fn word(i: u32) -> u32 {
    if i >= cfg.n {
        return 0u;
    }
    if cfg.first != 0u {
        return data[i];
    }
    return sums[cfg.at + i];
}

fn set_word(i: u32, v: u32) {
    if i >= cfg.n {
        return;
    }
    if cfg.first != 0u {
        data[i] = v;
    } else {
        sums[cfg.at + i] = v;
    }
}

// One workgroup's 256 words scanned in place, four a lane; their total
// to the next level, or the scan's total at the top.
@compute @workgroup_size(64)
fn scan_up(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lane: u32) {
    let w = wid.x + wid.y * cfg.wx;
    let base = w * SPAN + 4u * lane;
    var v = array<u32, 4>(word(base), word(base + 1u), word(base + 2u), word(base + 3u));
    let own = v[0] + v[1] + v[2] + v[3];
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
    var run = w_lanes[lane] - own;
    for (var k = 0u; k < 4u; k++) {
        let x = v[k];
        set_word(base + k, run);
        run += x;
    }
    if lane == LANES - 1u {
        if cfg.top != 0u {
            totals[0] = w_lanes[lane];
        } else {
            sums[cfg.next + w] = w_lanes[lane];
        }
    }
}

// Each word moved on by its workgroup's place in the next level.
@compute @workgroup_size(64)
fn scan_down(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x + id.y * cfg.wx * LANES;
    if i >= cfg.n {
        return;
    }
    set_word(i, word(i) + sums[cfg.next + i / SPAN]);
}
