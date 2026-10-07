// A wave's chains handed on, on the device (fn-209 F1, tasks 5 and 6;
// host decision 6): `describe` writes each chain's geometry item from its
// run record and its places in the wave (`geometry.wgsl` reads them), and
// `realized` writes every bud its axes bear as the next wave's buds, each
// with its root and its bearer's index in this wave's laid buffer.

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
@group(0) @binding(6) var<storage, read_write> scratch: array<u32>;
@group(0) @binding(7) var<storage, read_write> next_buds: array<u32>;
@group(0) @binding(8) var<storage, read_write> next_meta: array<vec2<u32>>;

const RW: u32 = 10u;
const CW: u32 = 3u;
const IW: u32 = 12u;
const AXW: u32 = 80u;
const SA: u32 = 66u;
const BW2: u32 = 28u;
const KIND: u32 = 4u;
const PARENT: u32 = 5u;
const LATERAL: u32 = 1u;
const IS_WOKEN: u32 = 0x80000u;

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
    items[at + 10u] = wi;
    items[at + 11u] = places[wi * CW + 2u];
}

// Writes bud `k` of the next wave: the borne lateral at packed word `a`,
// on the root `root`, its bearer at `bearer` in this wave's laid buffer.
fn put_bud(a: u32, k: u32, root: u32, bearer: u32) {
    let kind = axes[a + KIND];
    let b = k * BW2;
    let woke = (kind & IS_WOKEN) != 0u;
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
    next_buds[b + 10u] = select(0u, 1u, woke);
    next_buds[b + 11u] = (kind >> 8u) & 0xffu;
    next_buds[b + 12u] = select(0u, axes[a + 8u], woke);
    next_buds[b + 13u] = select(0u, axes[a + 9u], woke);
    next_buds[b + 14u] = select(0u, axes[a + 31u], woke);
    next_buds[b + 15u] = select(0u, axes[a + 20u], woke);
    next_buds[b + 16u] = select(0u, axes[a + 12u], woke);
    next_buds[b + 17u] = select(0u, axes[a + 13u], woke);
    next_buds[b + 18u] = select(0u, axes[a + 19u], woke);
    next_buds[b + 19u] = select(0u, axes[a + 3u] + 1u, woke);
    next_buds[b + 20u] = 0u;
    next_buds[b + 21u] = 0u;
    next_buds[b + 22u] = axes[a + 36u];
    next_buds[b + 23u] = axes[a + 37u];
    // Its making draws' presences, Q32 (`assign.wgsl`'s B0 and B1).
    for (var f = 0u; f < 4u; f++) {
        next_buds[b + 24u + f] = axes[a + 40u + f];
    }
    next_meta[k] = vec2<u32>(root, bearer);
}

// Realized mechanics' growth (fn-209 F1 task 6): every axis kept, so each
// own axis's place among the chain's own is written to its scratch here,
// no geometry having run, and every borne lateral is the next wave's bud.
@compute @workgroup_size(64)
fn realized(@builtin(global_invocation_id) id: vec3<u32>) {
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
    var own = 0u;
    for (var i = 0u; i < na; i++) {
        let lateral = (axes[(a0 + i) * AXW + KIND] & 0xffu) == LATERAL;
        if i == 1u || (i > 1u && !lateral) {
            scratch[sb + i * SA] = own;
            own += 1u;
        } else {
            scratch[sb + i * SA] = 0xffffffffu;
        }
    }
    let out_axes = places[wi * CW];
    var k = places[wi * CW + 2u];
    let root = owners[wi].x;
    for (var i = 2u; i < na; i++) {
        let a = (a0 + i) * AXW;
        if (axes[a + KIND] & 0xffu) != LATERAL {
            continue;
        }
        put_bud(a, k, root, out_axes + scratch[sb + axes[a + PARENT] * SA]);
        k += 1u;
    }
}
