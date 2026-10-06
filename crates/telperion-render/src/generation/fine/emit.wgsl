// A wave's chains handed on, on the device (fn-209 F1, task 5; host
// decision 6): `describe` writes each chain's geometry item from its run
// record and its places in the wave (`geometry.wgsl` reads them), and
// `emit` writes the buds its kept axes bear as the next wave's buds, each
// with its root and its bearer's index in this wave's laid buffer, in the
// order the CPU reference grows them.

struct Cfg {
    n: u32,
    row: u32,
    stub: u32,
    pad0: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> runoff: array<u32>;
@group(0) @binding(2) var<storage, read> places: array<u32>;
@group(0) @binding(3) var<storage, read> owners: array<vec2<u32>>;
@group(0) @binding(4) var<storage, read_write> items: array<u32>;
@group(0) @binding(5) var<storage, read> axes: array<u32>;
@group(0) @binding(6) var<storage, read> scratch: array<u32>;
@group(0) @binding(7) var<storage, read_write> next_buds: array<u32>;
@group(0) @binding(8) var<storage, read_write> next_meta: array<vec2<u32>>;

const RW: u32 = 10u;
const CW: u32 = 3u;
const IW: u32 = 10u;
const AXW: u32 = 40u;
const SA: u32 = 66u;
const BW2: u32 = 24u;
const KIND: u32 = 4u;
const PARENT: u32 = 5u;
const LATERAL: u32 = 1u;
const IS_WOKEN: u32 = 0x80000u;
const SHED_FLAGS: u32 = 28u;

fn chain_of(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

@compute @workgroup_size(64)
fn describe(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = chain_of(id);
    if c >= cfg.n {
        return;
    }
    let r = c * RW;
    let wi = runoff[r + 7u];
    let ok = runoff[r + 6u] != 0u;
    let m = owners[wi];
    let at = c * IW;
    items[at] = runoff[r];
    items[at + 1u] = select(0u, runoff[r + 8u], ok);
    items[at + 2u] = runoff[r + 1u];
    items[at + 3u] = runoff[r + 9u];
    items[at + 4u] = m.y;
    items[at + 5u] = places[wi * CW];
    items[at + 6u] = places[wi * CW + 1u];
    items[at + 7u] = runoff[r + 2u];
    items[at + 8u] = cfg.stub;
    items[at + 9u] = m.x;
}

@compute @workgroup_size(64)
fn emit(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = chain_of(id);
    if c >= cfg.n {
        return;
    }
    let r = c * RW;
    if runoff[r + 6u] == 0u {
        return;
    }
    let wi = runoff[r + 7u];
    let a0 = runoff[r];
    let na = runoff[r + 8u];
    let sb = runoff[r + 2u];
    let out_axes = places[wi * CW];
    var k = places[wi * CW + 2u];
    let root = owners[wi].x;
    for (var i = 2u; i < na; i++) {
        let a = (a0 + i) * AXW;
        let kind = axes[a + KIND];
        if (kind & 0xffu) != LATERAL {
            continue;
        }
        let own = scratch[sb + axes[a + PARENT] * SA];
        if own == 0xffffffffu {
            continue;
        }
        let b = k * BW2;
        let woke = (kind & IS_WOKEN) != 0u;
        let context = (axes[a + SHED_FLAGS] & 2u) != 0u;
        next_buds[b] = axes[a];
        next_buds[b + 1u] = axes[a + 1u];
        next_buds[b + 2u] = axes[a + 2u];
        next_buds[b + 3u] = axes[a + 3u];
        next_buds[b + 4u] = 0u;
        next_buds[b + 5u] = axes[a + 6u];
        next_buds[b + 6u] = axes[a + 8u];
        next_buds[b + 7u] = axes[a + 9u];
        next_buds[b + 8u] = axes[a + 10u];
        next_buds[b + 9u] = axes[a + 11u];
        next_buds[b + 10u] = select(0u, 1u, woke) | select(0u, 2u, context);
        next_buds[b + 11u] = (kind >> 8u) & 0xffu;
        next_buds[b + 12u] = select(0u, axes[a + 8u], woke);
        next_buds[b + 13u] = select(0u, axes[a + 9u], woke);
        next_buds[b + 14u] = select(0u, axes[a + 31u], woke);
        next_buds[b + 15u] = select(0u, axes[a + 20u], woke);
        next_buds[b + 16u] = select(0u, axes[a + 12u], woke);
        next_buds[b + 17u] = select(0u, axes[a + 13u], woke);
        next_buds[b + 18u] = select(0u, axes[a + 19u], woke);
        next_buds[b + 19u] = select(0u, axes[a + 3u] + 1u, woke);
        next_buds[b + 20u] = select(0u, axes[a + 29u], context);
        next_buds[b + 21u] = select(0u, axes[a + 30u], context);
        next_buds[b + 22u] = axes[a + 36u];
        next_buds[b + 23u] = axes[a + 37u];
        next_meta[k] = vec2<u32>(root, out_axes + own);
        k += 1u;
    }
}
