// fn-256 task 2 (DESIGN.md section 3, revised): each new axis's depth and
// block. A cycle's new axes are placed by generation (how many of their
// ancestors are new), each generation in passes. In a pass a waiting
// axis targets its bearer's block while it has room, else that block's
// open spill while it has room. A target's waiters join it while it holds
// fewer than `cap`, the lowest indices first; the rest wait for the next
// pass. Where both are full, the block's waiters fill fresh spills of it
// (siblings, a level below it) in index order, `cap` each; the last
// becomes its open spill. Openers are numbered by a scan in index order.

// The cycle begins: the blocks and levels before it kept.
@compute @workgroup_size(1)
fn begin_main() {
    atomicStore(&tally[X_BLOCKS_OLD], atomicLoad(&tally[X_BLOCKS]));
    atomicStore(&tally[X_LEVELS_OLD], atomicLoad(&tally[X_LEVELS]));
}

// Each new axis's depth and generation, from its nearest old ancestor
// (none for the seed), unplaced, and counted on its bearer's children.
@compute @workgroup_size(64)
fn depth_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = cfg.n_old + index(id);
    if i >= cfg.n_new {
        return;
    }
    var j = i;
    var depth = NONE;
    var gen = 0u;
    for (var s = 0u; s < cfg.n_new; s++) {
        let p = tree[at(T_PARENT, j)];
        gen = s;
        if p == NONE {
            depth = s;
            break;
        }
        if p < cfg.n_old {
            depth = shape[at(H_DEPTH, p)] + s + 1u;
            break;
        }
        j = p;
    }
    if depth == NONE {
        atomicOr(&tally[X_FAULT], 1u);
        return;
    }
    shape[at(H_DEPTH, i)] = depth;
    shape[at(H_GEN, i)] = gen;
    shape[at(H_BLOCK, i)] = NONE;
    atomicMax(&tally[X_GENS], gen + 1u);
    let p = tree[at(T_PARENT, i)];
    if p != NONE {
        atomicAdd(&counts[at(C_KIDS, p)], 1u);
    }
}

// A waiting axis's target word: the block it joins; or, its bearer's
// block and open spill both full, that block with the top bit set; `NONE`
// for the seed.
const AT_FULL: u32 = 0x80000000u;

fn waiting(i: u32) -> bool {
    return i < cfg.n_new && shape[at(H_GEN, i)] == cfg.round && shape[at(H_BLOCK, i)] == NONE;
}

fn has_room(b: u32) -> bool {
    return b != NONE && count(C_SIZE, b) < cfg.cap;
}

// Each waiting axis's target, counted on its block (a full block counts
// its waiters for fresh spills).
@compute @workgroup_size(64)
fn target_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    let i = cfg.n_old + k;
    if !waiting(i) {
        return;
    }
    let p = tree[at(T_PARENT, i)];
    var aim = NONE;
    if p != NONE {
        let t = shape[at(H_BLOCK, p)];
        let spill = shape[at(H_SPILL, t)];
        aim = t | AT_FULL;
        if has_room(t) {
            aim = t;
        } else if has_room(spill) {
            aim = spill;
        }
        atomicAdd(&counts[at(C_JOIN, aim & ~AT_FULL)], 1u);
    }
    atomicStore(&counts[at(C_TARGET, k)], aim);
}

// The pass's waiters listed under their targets, by fill.
@compute @workgroup_size(64)
fn join_place_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    let i = cfg.n_old + k;
    if !waiting(i) || count(C_TARGET, k) == NONE {
        return;
    }
    let t = count(C_TARGET, k) & ~AT_FULL;
    let slot = atomicAdd(&counts[at(C_JOIN_FILL, t)], 1u);
    bucket[at(U_JOINERS, count(C_JOIN, t) + slot)] = i;
}

// Axis `i`'s rank in index order among its target's waiters.
fn rank_of(i: u32, t: u32) -> u32 {
    let waiters = scanned(C_JOIN, t, cfg.block_rows, X_JOIN);
    var rank = 0u;
    for (var k = 0u; k < waiters.y; k++) {
        rank += u32(bucket[at(U_JOINERS, waiters.x + k)] < i);
    }
    return rank;
}

// Each new axis's state this pass: 1 joined (its block set), 2 opening a
// fresh spill, 3 one of its later members, 0 waiting or placed before;
// above the state, a fresh spill's group; its opening flag for the scan.
@compute @workgroup_size(64)
fn decide_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    let i = cfg.n_old + k;
    if i >= cfg.n_new {
        return;
    }
    var state = 0u;
    if waiting(i) {
        let t = count(C_TARGET, k);
        if t == NONE {
            state = 2u;
        } else if (t & AT_FULL) != 0u {
            let rank = rank_of(i, t & ~AT_FULL);
            state = select(3u, 2u, rank % cfg.cap == 0u) | ((rank / cfg.cap) << 2u);
        } else {
            let room = cfg.cap - min(count(C_SIZE, t), cfg.cap);
            if scanned(C_JOIN, t, cfg.block_rows, X_JOIN).y <= room || rank_of(i, t) < room {
                shape[at(H_BLOCK, i)] = t;
                state = 1u;
            }
        }
    }
    atomicStore(&counts[at(C_STATE, k)], state);
    atomicStore(&counts[at(C_OPEN, k)], u32((state & 3u) == 2u));
}

// The pass's joiners counted on their blocks; each opener numbered past
// the blocks by the scan of the openings, its spill a level below the
// full block (the root block for the seed), sized for its group, listed
// in the full block's slots, and the last group's made its open spill.
@compute @workgroup_size(64)
fn assign_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    let i = cfg.n_old + k;
    if i >= cfg.n_new || shape[at(H_GEN, i)] != cfg.round {
        return;
    }
    let state = count(C_STATE, k);
    if state == 1u {
        atomicAdd(&counts[at(C_SIZE, shape[at(H_BLOCK, i)])], 1u);
        return;
    }
    if (state & 3u) != 2u {
        return;
    }
    let opened = atomicLoad(&tally[X_BLOCKS]) + count(C_OPEN, k);
    // A block past the shape's rows is never written: the host refuses
    // the fault by name.
    if opened >= cfg.stride {
        atomicOr(&tally[X_FAULT], 1u);
        return;
    }
    let t = count(C_TARGET, k);
    var level = 0u;
    var size = 1u;
    var origin = NONE;
    if t != NONE {
        let full = t & ~AT_FULL;
        origin = full;
        let group = state >> 2u;
        let waiters = scanned(C_JOIN, full, cfg.block_rows, X_JOIN);
        level = shape[at(H_LEVEL, full)] + 1u;
        size = min(cfg.cap, waiters.y - group * cfg.cap);
        bucket[at(U_GROUPS, waiters.x + group)] = opened;
        if (waiters.y - 1u) / cfg.cap == group {
            shape[at(H_SPILL, full)] = opened;
        }
    }
    shape[at(H_BLOCK, i)] = opened;
    shape[at(H_SPILL, opened)] = NONE;
    shape[at(H_LEVEL, opened)] = level;
    shape[at(H_FROM, opened)] = origin;
    atomicStore(&counts[at(C_SIZE, opened)], size);
    atomicMax(&tally[X_LEVELS], level + 1u);
}

// Each later member of a fresh spill given its block; the axes still
// waiting counted; and the pass's openers added to the blocks, which
// `assign_main` read before them (the pass's end, fn-256 task 6).
@compute @workgroup_size(64)
fn follow_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k == 0u && cfg.n_new > cfg.n_old {
        atomicAdd(&tally[X_BLOCKS], atomicLoad(&tally[X_OPEN]));
    }
    let i = cfg.n_old + k;
    if !waiting(i) {
        return;
    }
    let state = count(C_STATE, k);
    if (state & 3u) != 3u {
        atomicAdd(&tally[X_UNPLACED], 1u);
        return;
    }
    let waiters = scanned(C_JOIN, count(C_TARGET, k) & ~AT_FULL, cfg.block_rows, X_JOIN);
    shape[at(H_BLOCK, i)] = bucket[at(U_GROUPS, waiters.x + (state >> 2u))];
}
