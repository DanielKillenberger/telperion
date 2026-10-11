// fn-256 task 2 (DESIGN.md section 3): each new axis's depth and block.
// A cycle's new axes are placed by generation (how many of their
// ancestors are new): in a round, an axis joins its bearer's block while
// that block holds fewer than `cap` axes, the lowest indices first, and
// otherwise opens a block of its own one level below its bearer's. The
// openers are numbered by a scan in index order. A block's place in its
// level's list is a scan too, by index.

// The cycle begins: the blocks and levels before it kept.
@compute @workgroup_size(1)
fn begin_main() {
    atomicStore(&tally[X_BLOCKS_OLD], atomicLoad(&tally[X_BLOCKS]));
    atomicStore(&tally[X_LEVELS_OLD], atomicLoad(&tally[X_LEVELS]));
}

// Each new axis's depth and generation, from its nearest old ancestor
// (none for the seed), and its count on its bearer's new children.
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
    atomicMax(&tally[X_GENS], gen + 1u);
    let p = tree[at(T_PARENT, i)];
    if p != NONE {
        atomicAdd(&counts[at(C_KIDS, p)], 1u);
    }
}

// The round's axis at `i`, and its bearer: none where `i` is not in the
// round or has no bearer.
fn joiner(i: u32) -> u32 {
    if i >= cfg.n_new || shape[at(H_GEN, i)] != cfg.round {
        return NONE;
    }
    return tree[at(T_PARENT, i)];
}

// The round's axes counted on their bearers' blocks.
@compute @workgroup_size(64)
fn join_count_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = cfg.n_old + index(id);
    let p = joiner(i);
    if p != NONE {
        atomicAdd(&counts[at(C_JOIN, shape[at(H_BLOCK, p)])], 1u);
    }
}

// The round's axes listed under their bearers' blocks, by fill.
@compute @workgroup_size(64)
fn join_place_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = cfg.n_old + index(id);
    let p = joiner(i);
    if p == NONE {
        return;
    }
    let t = shape[at(H_BLOCK, p)];
    let slot = atomicAdd(&counts[at(C_JOIN_FILL, t)], 1u);
    bucket[at(U_JOINERS, count(C_JOIN, t) + slot)] = i;
}

// Whether the round's axis `i`, bearer `p`, opens a block: where its
// bearer's has no room for it, the round's lower indices first.
fn opens(i: u32, p: u32) -> bool {
    if p == NONE {
        return true;
    }
    let t = shape[at(H_BLOCK, p)];
    let joining = scanned(C_JOIN, t, cfg.block_rows, X_JOIN);
    let room = cfg.cap - min(count(C_SIZE, t), cfg.cap);
    if joining.y <= room {
        return false;
    }
    var rank = 0u;
    for (var k = 0u; k < joining.y; k++) {
        rank += u32(bucket[at(U_JOINERS, joining.x + k)] < i);
    }
    return rank >= room;
}

// Each new axis's opening flag (none outside the round); a joiner's
// block set, an opener's `NONE` until it is numbered.
@compute @workgroup_size(64)
fn decide_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    let i = cfg.n_old + k;
    if i >= cfg.n_new {
        return;
    }
    var open = false;
    if shape[at(H_GEN, i)] == cfg.round {
        let p = tree[at(T_PARENT, i)];
        open = opens(i, p);
        var b = NONE;
        if !open {
            b = shape[at(H_BLOCK, p)];
        }
        shape[at(H_BLOCK, i)] = b;
    }
    atomicStore(&counts[at(C_OPEN, k)], u32(open));
}

// The round's axes placed: an opener numbered past the blocks by the
// scan of the openings, a level below its bearer's block; a joiner one
// local depth below its bearer.
@compute @workgroup_size(64)
fn assign_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    let i = cfg.n_old + k;
    if i >= cfg.n_new || shape[at(H_GEN, i)] != cfg.round {
        return;
    }
    let p = tree[at(T_PARENT, i)];
    let b = shape[at(H_BLOCK, i)];
    if b != NONE {
        shape[at(H_LOCAL, i)] = shape[at(H_LOCAL, p)] + 1u;
        atomicAdd(&counts[at(C_SIZE, b)], 1u);
        return;
    }
    let opened = atomicLoad(&tally[X_BLOCKS]) + count(C_OPEN, k);
    var level = 0u;
    if p != NONE {
        level = shape[at(H_LEVEL, shape[at(H_BLOCK, p)])] + 1u;
    }
    shape[at(H_BLOCK, i)] = opened;
    shape[at(H_LOCAL, i)] = 0u;
    shape[at(H_LEVEL, opened)] = level;
    atomicStore(&counts[at(C_SIZE, opened)], 1u);
    atomicMax(&tally[X_LEVELS], level + 1u);
}

// The round's openers added to the blocks.
@compute @workgroup_size(1)
fn round_end_main() {
    atomicAdd(&tally[X_BLOCKS], atomicLoad(&tally[X_OPEN]));
}

// Each new axis counted on its block's new members, each new block on
// its level's.
@compute @workgroup_size(64)
fn member_count_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    let i = cfg.n_old + k;
    if i >= cfg.n_new {
        return;
    }
    atomicAdd(&counts[at(C_MEMBERS, shape[at(H_BLOCK, i)])], 1u);
    let b = atomicLoad(&tally[X_BLOCKS_OLD]) + k;
    if b < atomicLoad(&tally[X_BLOCKS]) {
        atomicAdd(&counts[at(C_LEVEL, shape[at(H_LEVEL, b)])], 1u);
    }
}

var<workgroup> w_level: vec4<u32>;
var<workgroup> w_blocks: vec2<u32>;
var<workgroup> w_flags: array<u32, WG>;

// One level's blocks: its old run, then its new blocks in index order,
// each placed by a workgroup's scan of the flags of a run of 64.
@compute @workgroup_size(64)
fn level_merge_main(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    let l = wid.x;
    if lid == 0u {
        let old = atomicLoad(&tally[X_LEVELS_OLD]);
        let os = cur[ls(R_LVL_START, min(l, old))];
        var oe = os;
        if l < old {
            oe = cur[ls(R_LVL_START, l + 1u)];
        }
        let out = os + scanned(C_LEVEL, l, cfg.level_rows, X_LEVEL).x;
        w_level = vec4<u32>(os, oe, out, atomicLoad(&tally[X_LEVELS]));
        w_blocks = vec2<u32>(atomicLoad(&tally[X_BLOCKS_OLD]), atomicLoad(&tally[X_BLOCKS]));
    }
    let run = workgroupUniformLoad(&w_level);
    let span = workgroupUniformLoad(&w_blocks);
    if l >= run.w {
        return;
    }
    let blocks = span.y;
    if lid == 0u {
        next[ls(R_LVL_START, l)] = run.z;
        if l + 1u == run.w {
            next[ls(R_LVL_START, run.w)] = blocks;
        }
    }
    for (var k = run.x + lid; k < run.y; k += WG) {
        next[ls(R_LVL_BLOCKS, run.z + k - run.x)] = cur[ls(R_LVL_BLOCKS, k)];
    }
    var to = run.z + run.y - run.x;
    for (var s = span.x; s < blocks; s += WG) {
        let b = s + lid;
        let mine = b < blocks && shape[at(H_LEVEL, b)] == l;
        w_flags[lid] = u32(mine);
        workgroupBarrier();
        for (var d = 1u; d < WG; d *= 2u) {
            var add = 0u;
            if lid >= d {
                add = w_flags[lid - d];
            }
            workgroupBarrier();
            w_flags[lid] += add;
            workgroupBarrier();
        }
        if mine {
            next[ls(R_LVL_BLOCKS, to + w_flags[lid] - 1u)] = b;
        }
        to += workgroupUniformLoad(&w_flags[WG - 1u]);
    }
}
