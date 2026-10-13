// One step of a bitonic sort of `keys`, ascending by their first word,
// then their second (fn-261): `cfg.n` keys, a power of two, the step's
// span `cfg.j` within its run of `cfg.k`. The order is the keys' alone,
// however the device schedules the threads.

struct Cfg {
    n: u32,
    row: u32,
    j: u32,
    k: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read_write> keys: array<vec2<u32>>;

@compute @workgroup_size(64)
fn step(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x + id.y * cfg.row;
    let l = i ^ cfg.j;
    if i >= cfg.n || l <= i {
        return;
    }
    let a = keys[i];
    let b = keys[l];
    let greater = a.x > b.x || (a.x == b.x && a.y > b.y);
    if greater == ((i & cfg.k) == 0u) {
        keys[i] = b;
        keys[l] = a;
    }
}
