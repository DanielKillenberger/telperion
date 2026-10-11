// fn-256 task 2 (DESIGN.md sections 3 and 4): the hierarchy built on the
// device as the tree grows, one cycle's new axes `[n_old, n_new)` at a
// time. Each axis's children stand in fn-255's order (by node, then
// index, the tip's last); each block's members by depth, then index;
// each level's blocks by index. A cycle's counts are atomic adds, which
// no order changes; every place is a scan's, or a rank's or a sort's of
// the few records an owner gained, never an atomic slot's.

struct Cfg {
    n_old: u32,
    n_new: u32,
    // The most axes a block holds, and the round (generation) placed.
    cap: u32,
    round: u32,
    stride: u32,
    row: u32,
    // The rows the blocks' and the levels' scans cover.
    block_rows: u32,
    level_rows: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> tree: array<u32>;
@group(0) @binding(2) var<storage, read_write> shape: array<u32>;
@group(0) @binding(3) var<storage, read_write> counts: array<atomic<u32>>;
@group(0) @binding(4) var<storage, read> cur: array<u32>;
@group(0) @binding(5) var<storage, read_write> next: array<u32>;
@group(0) @binding(6) var<storage, read_write> bucket: array<u32>;
@group(0) @binding(7) var<storage, read_write> tally: array<atomic<u32>>;

// The counts' columns: new children per axis and their fill; joiners per
// block this pass, their fill and each new axis's opening; each block's
// size; new members per block and their fill; new blocks per level; each
// new axis's target this pass and its state (joined, opened, a fresh
// spill's member).
const C_KIDS: u32 = 0u;
const C_KID_FILL: u32 = 1u;
const C_JOIN: u32 = 2u;
const C_JOIN_FILL: u32 = 3u;
const C_OPEN: u32 = 4u;
const C_SIZE: u32 = 5u;
const C_MEMBERS: u32 = 6u;
const C_MEMBER_FILL: u32 = 7u;
const C_LEVEL: u32 = 8u;
const C_TARGET: u32 = 9u;
const C_STATE: u32 = 11u;

// The buckets: each owner's new records, before their sort, the members
// sorted, and the fresh spills each full block opened, by group.
const U_KIDS: u32 = 0u;
const U_JOINERS: u32 = 1u;
const U_MEMBERS: u32 = 2u;
const U_SORTED: u32 = 3u;
const U_GROUPS: u32 = 4u;

// The tally: blocks and levels, the cycle's generations, the blocks and
// levels before it, faults, the axes a pass left unplaced; then each
// scan's total, 64 words apart.
const X_BLOCKS: u32 = 0u;
const X_LEVELS: u32 = 1u;
const X_GENS: u32 = 2u;
const X_BLOCKS_OLD: u32 = 3u;
const X_LEVELS_OLD: u32 = 4u;
const X_FAULT: u32 = 5u;
const X_UNPLACED: u32 = 6u;
const X_KIDS: u32 = 64u;
const X_JOIN: u32 = 128u;
const X_OPEN: u32 = 192u;
const X_MEMBERS: u32 = 256u;
const X_LEVEL: u32 = 320u;

// The two orders: children by node, members by depth.
const BY_NODE: u32 = 0u;
const BY_DEPTH: u32 = 1u;

fn at(col: u32, i: u32) -> u32 {
    return col * cfg.stride + i;
}

fn ls(col: u32, i: u32) -> u32 {
    return col * (cfg.stride + 64u) + i;
}

fn index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

fn count(col: u32, i: u32) -> u32 {
    return atomicLoad(&counts[at(col, i)]);
}

// A scanned count's start and length: its row's start, to the next's,
// or to the scan's total past the last of `rows`.
fn scanned(col: u32, i: u32, rows: u32, total: u32) -> vec2<u32> {
    let s = count(col, i);
    var e = atomicLoad(&tally[total]);
    if i + 1u < rows {
        e = count(col, i + 1u);
    }
    return vec2<u32>(s, e - s);
}

fn key(by: u32, x: u32) -> u32 {
    if by == BY_NODE {
        return tree[at(T_NODE, x)];
    }
    return shape[at(H_DEPTH, x)];
}

fn before(by: u32, a: u32, b: u32) -> bool {
    let ka = key(by, a);
    let kb = key(by, b);
    return ka < kb || (ka == kb && a < b);
}

// An owner's `n` new records at `s` in `col`, sorted in place.
fn sort_run(col: u32, s: u32, n: u32, by: u32) {
    for (var k = 1u; k < n; k++) {
        let x = bucket[at(col, s + k)];
        var j = k;
        for (; j > 0u; j--) {
            let y = bucket[at(col, s + j - 1u)];
            if !before(by, x, y) {
                break;
            }
            bucket[at(col, s + j)] = y;
        }
        bucket[at(col, s + j)] = x;
    }
}

// An owner's old run `old` of list `list` and its sorted new records
// (`col`, `fresh`) merged into `next` from `out`.
fn merge_run(list: u32, old: vec2<u32>, col: u32, fresh: vec2<u32>, out: u32, by: u32) {
    var a = old.x;
    var b = 0u;
    for (var o = 0u; o < old.y - old.x + fresh.y; o++) {
        var take_new = a >= old.y;
        if !take_new && b < fresh.y {
            take_new = before(by, bucket[at(col, fresh.x + b)], cur[ls(list, a)]);
        }
        if take_new {
            next[ls(list, out + o)] = bucket[at(col, fresh.x + b)];
            b++;
        } else {
            next[ls(list, out + o)] = cur[ls(list, a)];
            a++;
        }
    }
}

// Each new axis's place in its bearer's new children, by fill.
@compute @workgroup_size(64)
fn kid_place_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = cfg.n_old + index(id);
    if i >= cfg.n_new {
        return;
    }
    let p = tree[at(T_PARENT, i)];
    if p == NONE {
        return;
    }
    let slot = atomicAdd(&counts[at(C_KID_FILL, p)], 1u);
    bucket[at(U_KIDS, count(C_KIDS, p) + slot)] = i;
}

// Each axis's children: its old run and its new children, sorted by
// node then index, merged at its new start.
@compute @workgroup_size(64)
fn kid_merge_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let p = index(id);
    if p >= cfg.n_new {
        return;
    }
    let os = cur[ls(R_KID_START, min(p, cfg.n_old))];
    var oe = os;
    if p < cfg.n_old {
        oe = cur[ls(R_KID_START, p + 1u)];
    }
    let fresh = scanned(C_KIDS, p, cfg.n_new, X_KIDS);
    let out = os + fresh.x;
    next[ls(R_KID_START, p)] = out;
    if p + 1u == cfg.n_new {
        let total = cur[ls(R_KID_START, cfg.n_old)] + atomicLoad(&tally[X_KIDS]);
        next[ls(R_KID_START, cfg.n_new)] = total;
    }
    sort_run(U_KIDS, fresh.x, fresh.y, BY_NODE);
    merge_run(R_KIDS, vec2<u32>(os, oe), U_KIDS, fresh, out, BY_NODE);
}
