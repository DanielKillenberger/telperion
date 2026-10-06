// The chains a run's regions could not hold, gathered on the device for
// the next, larger region (fn-209 F1, task 5; the overflow rule): each
// queued chain's bud copied from its wave's buds, in the queue's order,
// which the run's records map back to the wave.

struct Cfg {
    n: u32,
    row: u32,
    pad0: u32,
    pad1: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> queue: array<u32>;
@group(0) @binding(2) var<storage, read> wave: array<u32>;
@group(0) @binding(3) var<storage, read_write> gathered: array<u32>;

const BW2: u32 = 24u;

@compute @workgroup_size(64)
fn gather(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x + id.y * cfg.row;
    if c >= cfg.n {
        return;
    }
    let src = queue[c] * BW2;
    for (var f = 0u; f < BW2; f++) {
        gathered[c * BW2 + f] = wave[src + f];
    }
}
