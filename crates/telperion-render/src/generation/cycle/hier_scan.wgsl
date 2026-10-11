// fn-256 task 2: the cycle's exclusive prefix sum, in levels. Each
// workgroup scans 256 records of a level in place and writes their total
// to the next level, until one workgroup holds the top, whose total is
// the scan's; each level then adds the next level's scanned totals back.
// The first level reads the first `fields` words of records `stride`
// words apart; each level above holds a field's words after another's.
// Integer adds in a fixed shape: the same words on every run, as
// `fine/scan.wgsl` gives, with no one-invocation spine.

struct Cfg {
    // Records in this level, where it starts (in `sums` past the first),
    // where the next starts in `sums`, whether this is the first level
    // (`data`) and whether it is the top; the first level's stride, the
    // fields scanned, and the tally's word of the first field's total.
    n: u32,
    at: u32,
    next: u32,
    first: u32,
    top: u32,
    wx: u32,
    stride: u32,
    fields: u32,
    out: u32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read_write> data: array<u32>;
@group(0) @binding(2) var<storage, read_write> sums: array<u32>;
@group(0) @binding(3) var<storage, read_write> totals: array<u32>;

const SPAN: u32 = 256u;
const LANES: u32 = 64u;

var<workgroup> w_lanes: array<u32, LANES>;

fn word(i: u32, f: u32) -> u32 {
    if i >= cfg.n {
        return 0u;
    }
    if cfg.first != 0u {
        return data[i * cfg.stride + f];
    }
    return sums[cfg.at + f * cfg.n + i];
}

fn set_word(i: u32, f: u32, v: u32) {
    if i >= cfg.n {
        return;
    }
    if cfg.first != 0u {
        data[i * cfg.stride + f] = v;
    } else {
        sums[cfg.at + f * cfg.n + i] = v;
    }
}

// One workgroup's 256 records scanned in place, four a lane, a field at
// a time; each field's total to the next level, or the scan's at the top.
@compute @workgroup_size(64)
fn scan_up(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lane: u32) {
    let w = wid.x + wid.y * cfg.wx;
    let base = w * SPAN + 4u * lane;
    let above = (cfg.n + SPAN - 1u) / SPAN;
    for (var f = 0u; f < cfg.fields; f++) {
        var v = array<u32, 4>(word(base, f), word(base + 1u, f), word(base + 2u, f), word(base + 3u, f));
        let own = v[0] + v[1] + v[2] + v[3];
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
        var run = w_lanes[lane] - own;
        for (var k = 0u; k < 4u; k++) {
            let x = v[k];
            set_word(base + k, f, run);
            run += x;
        }
        if lane == LANES - 1u {
            if cfg.top != 0u {
                totals[cfg.out + f] = w_lanes[lane];
            } else {
                sums[cfg.next + f * above + w] = w_lanes[lane];
            }
        }
    }
}

// Each record moved on by its workgroup's place in the next level.
@compute @workgroup_size(64)
fn scan_down(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x + id.y * cfg.wx * LANES;
    if i >= cfg.n {
        return;
    }
    let above = (cfg.n + SPAN - 1u) / SPAN;
    for (var f = 0u; f < cfg.fields; f++) {
        set_word(i, f, word(i, f) + sums[cfg.next + f * above + i / SPAN]);
    }
}
