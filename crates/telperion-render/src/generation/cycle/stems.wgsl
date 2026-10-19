// fn-256 task 3 (DESIGN.md 12.3): the whole-tree measures the CPU's
// tree read off its trunk, folded over the drawn records: the root's
// radius (`convert`'s node 0, its first node's start radius) and the
// wood a canopy measures its slender wood against (`Tree::stem_radius`):
// the widest stem leaving the first node two stems leave, or the root.
// A node is created after its parent, and every fork of the stem lies
// beyond the first, so the first fork is the one nearest the root along
// the wood. Then the root's record is drawn at its radius.

struct Cfg {
    n: u32,
    row: u32,
    records: u32,
    pad0: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read_write> drawn: array<f32>;
@group(0) @binding(2) var<storage, read_write> parts: array<atomic<u32>>;
@group(0) @binding(3) var<storage, read_write> sums: array<atomic<u32>>;

const NONE: u32 = 0xffffffffu;
const CV: u32 = 38u;
const MADE: u32 = 2u;
const STEM: u32 = 128u;
// `sums`: the root's radius, the first fork's distance and id, and the
// widest stem leaving it.
const ROOT: u32 = 0u;
const FORK_AT: u32 = 1u;
const FORK: u32 = 2u;
const WIDEST: u32 = 3u;

fn word(i: u32, f: u32) -> u32 {
    return bitcast<u32>(drawn[cfg.records + i * CV + f]);
}

fn index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

// Record `i`'s parent where it is a node that parts its parent: a stem's,
// or any of the root's (id 0); none otherwise.
fn parted(i: u32) -> u32 {
    if (word(i, 6u) & MADE) == 0u || word(i, 7u) != i {
        return NONE;
    }
    let p = word(i, 16u);
    if p == 0u || (word(i, 6u) & STEM) != 0u {
        return p;
    }
    return NONE;
}

// Each node's parts counted onto its parent, and the root's radius.
@compute @workgroup_size(64)
fn count(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n {
        return;
    }
    let p = parted(i);
    if p == NONE {
        return;
    }
    atomicAdd(&parts[p], 1u);
    if p == 0u {
        atomicStore(&sums[ROOT], word(i, 4u));
    }
}

// A fork's distance along the wood, its float's bits (never negative).
fn forked(i: u32) -> bool {
    return (word(i, 6u) & MADE) != 0u && word(i, 7u) == i && atomicLoad(&parts[word(i, 14u)]) > 1u;
}

@compute @workgroup_size(64)
fn nearest(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i < cfg.n && forked(i) {
        atomicMin(&sums[FORK_AT], word(i, 15u));
    }
}

@compute @workgroup_size(64)
fn fork(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i < cfg.n && forked(i) && word(i, 15u) == atomicLoad(&sums[FORK_AT]) {
        atomicMin(&sums[FORK], word(i, 14u));
    }
}

// The widest stem leaving the first fork: a radius's bits order as it.
@compute @workgroup_size(64)
fn widest(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i < cfg.n && parted(i) == atomicLoad(&sums[FORK]) && parted(i) != NONE {
        atomicMax(&sums[WIDEST], word(i, 3u));
    }
}

// The root's record drawn at the root's radius (`convert`'s node 0).
@compute @workgroup_size(64)
fn root(@builtin(global_invocation_id) id: vec3<u32>) {
    if index(id) != 0u {
        return;
    }
    let r = atomicLoad(&sums[ROOT]);
    for (var f = 3u; f < 6u; f++) {
        drawn[cfg.records + f] = bitcast<f32>(r);
    }
}
