// fn-256 task 3 (DESIGN.md 12.2): the settled tree's chains, planned on
// the device from the node table. A chain is a lateral's first axis (or
// the seed) and every axis its continuations and relays carry on; its
// wave is the laterals between it and the seed. The dress draws a chain
// in one invocation, a wave a dispatch, as `space.rs`'s `convert` draws
// its axes: each chain's axes in the table's order, which is lineage
// order, depth by depth.

struct Cfg {
    // The invocations a dispatch row holds, its first item and its items.
    row: u32,
    base: u32,
    count: u32,
    // The kept axes, the node table's axis stride and the plan's column
    // stride.
    kept: u32,
    sk: u32,
    stride: u32,
    // The wave a flag or a list takes, and its first chain.
    wave: u32,
    first: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> nt: array<u32>;
@group(0) @binding(2) var<storage, read_write> plan: array<u32>;
@group(0) @binding(3) var<storage, read_write> atom: array<atomic<u32>>;

const NONE: u32 = 0xffffffffu;
const LATERAL: u32 = 1u;
const NA_KIND: u32 = 0u;
const NA_PARENT: u32 = 1u;
const NA_PHYTS: u32 = 21u;
// The plan's columns, `cfg.stride` words apart (`chains.rs`).
const HEAD: u32 = 0u;
const WAVE: u32 = 1u;
const KID0: u32 = 2u;
const RANK: u32 = 3u;
const LA: u32 = 4u;
const LP: u32 = 5u;
const MEMBERS: u32 = 6u;
const CH_HEAD: u32 = 7u;
const CH_AX: u32 = 8u;
const CH_PH: u32 = 9u;
const CH_N: u32 = 10u;
// `atom`'s words: a fault, the deepest wave, then each wave's chains,
// axes and phytomers; past them each axis's children, each head's
// chain's axes and phytomers, and the walk's queue, a column each.
const FAULT: u32 = 0u;
const DEEPEST: u32 = 1u;
const WAVES: u32 = 64u;
const W_CHAINS: u32 = 64u;
const W_AXES: u32 = 128u;
const W_PHYTS: u32 = 192u;
const COLUMNS: u32 = 256u;

fn kids_at(i: u32) -> u32 {
    return COLUMNS + i;
}

fn cax_at(i: u32) -> u32 {
    return COLUMNS + cfg.stride + i;
}

fn cph_at(i: u32) -> u32 {
    return COLUMNS + 2u * cfg.stride + i;
}

// The walk's queue: the chain's axes as it lists them, read back as it
// goes, so held atomic.
fn queue_at(i: u32) -> u32 {
    return COLUMNS + 3u * cfg.stride + i;
}

fn item(id: vec3<u32>) -> u32 {
    return cfg.base + id.x + id.y * cfg.row;
}

fn na(c: u32, i: u32) -> u32 {
    return nt[c * cfg.sk + i];
}

fn kind(i: u32) -> u32 {
    return na(NA_KIND, i) & 0xffu;
}

fn col(c: u32, i: u32) -> u32 {
    return plan[c * cfg.stride + i];
}

fn put(c: u32, i: u32, v: u32) {
    plan[c * cfg.stride + i] = v;
}

// Each axis of a depth: its chain's head and wave, from its parent's.
@compute @workgroup_size(64)
fn heads_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = item(id);
    if i >= cfg.base + cfg.count {
        return;
    }
    let p = na(NA_PARENT, i);
    put(KID0, i, NONE);
    if p == NONE {
        put(HEAD, i, i);
        put(WAVE, i, 0u);
        return;
    }
    if kind(i) == LATERAL {
        put(HEAD, i, i);
        put(WAVE, i, col(WAVE, p) + 1u);
        return;
    }
    put(HEAD, i, col(HEAD, p));
    put(WAVE, i, col(WAVE, p));
}

// Each axis counted onto its parent, its chain and its wave: integer
// sums, the same in any order. A parent's children are contiguous in
// the next depth, so its first child is the one after another parent's.
@compute @workgroup_size(64)
fn counts_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = item(id);
    if i >= cfg.kept {
        return;
    }
    let p = na(NA_PARENT, i);
    if p != NONE {
        if i == 0u || na(NA_PARENT, i - 1u) != p {
            put(KID0, p, i);
        }
        atomicAdd(&atom[kids_at(p)], 1u);
    }
    let h = col(HEAD, i);
    let w = col(WAVE, i);
    if w >= WAVES {
        atomicOr(&atom[FAULT], 1u);
        return;
    }
    let phyts = na(NA_PHYTS, i);
    atomicAdd(&atom[cax_at(h)], 1u);
    atomicAdd(&atom[cph_at(h)], phyts);
    atomicMax(&atom[DEEPEST], w);
    atomicAdd(&atom[W_AXES + w], 1u);
    atomicAdd(&atom[W_PHYTS + w], phyts);
    if h == i {
        atomicAdd(&atom[W_CHAINS + w], 1u);
    }
}

// Wave `cfg.wave`'s heads flagged, for the scan that ranks them.
@compute @workgroup_size(64)
fn flag_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = item(id);
    if i >= cfg.kept {
        return;
    }
    put(RANK, i, u32(col(HEAD, i) == i && col(WAVE, i) == cfg.wave));
}

// Wave `cfg.wave`'s chains listed from its first, its heads in order,
// each with its axes and phytomers for the scans that place them.
@compute @workgroup_size(64)
fn list_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = item(id);
    if i >= cfg.kept || col(HEAD, i) != i || col(WAVE, i) != cfg.wave {
        return;
    }
    let c = cfg.first + col(RANK, i);
    put(CH_HEAD, c, i);
    put(CH_AX, c, atomicLoad(&atom[cax_at(i)]));
    put(CH_N, c, atomicLoad(&atom[cax_at(i)]));
    put(CH_PH, c, atomicLoad(&atom[cph_at(i)]));
}

// One chain's axes listed breadth first from its head, children in
// order, laterals left to their own chains: on a table in lineage order,
// depth by depth, that is the axes' order in the table. Each axis's
// first phytomer and place among every wave's (the wave's own once its
// first is taken off).
@compute @workgroup_size(64)
fn walk_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = item(id);
    if c >= cfg.count {
        return;
    }
    let h = col(CH_HEAD, c);
    let base = col(CH_AX, c);
    var phyts = col(CH_PH, c);
    let n = atomicLoad(&atom[cax_at(h)]);
    atomicStore(&atom[queue_at(base)], h);
    var listed = 1u;
    // cap: a chain lists its `n` axes once each.
    for (var q = 0u; q < n; q++) {
        let a = atomicLoad(&atom[queue_at(base + q)]);
        put(MEMBERS, base + q, a);
        put(LA, a, base + q);
        put(LP, a, phyts);
        phyts += na(NA_PHYTS, a);
        let k0 = col(KID0, a);
        let kids = atomicLoad(&atom[kids_at(a)]);
        for (var k = 0u; k < kids; k++) {
            if kind(k0 + k) != LATERAL && listed < n {
                atomicStore(&atom[queue_at(base + listed)], k0 + k);
                listed += 1u;
            }
        }
    }
}
