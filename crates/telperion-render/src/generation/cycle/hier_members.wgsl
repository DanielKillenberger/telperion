// fn-256 task 2 (DESIGN.md section 3): each block's members by depth,
// then index, and each level's blocks by index, merged a cycle at a time.
// A block's new members are ranked among themselves; every member, old
// or new, then finds its place by a binary search of the other run, so
// no one invocation walks a whole block.

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

// Each new axis's place in its block's new members, by fill.
@compute @workgroup_size(64)
fn member_place_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = cfg.n_old + index(id);
    if i >= cfg.n_new {
        return;
    }
    let b = shape[at(H_BLOCK, i)];
    let slot = atomicAdd(&counts[at(C_MEMBER_FILL, b)], 1u);
    bucket[at(U_MEMBERS, count(C_MEMBERS, b) + slot)] = i;
}

fn new_members(b: u32) -> vec2<u32> {
    return scanned(C_MEMBERS, b, cfg.block_rows, X_MEMBERS);
}

// Block `b`'s old run in `cur`: none for a block the cycle opened.
fn old_members(b: u32) -> vec2<u32> {
    let old = atomicLoad(&tally[X_BLOCKS_OLD]);
    let os = cur[ls(R_MEM_START, min(b, old))];
    if b >= old {
        return vec2<u32>(os, os);
    }
    return vec2<u32>(os, cur[ls(R_MEM_START, b + 1u)]);
}

// Each new axis sorted among its block's new members by its rank.
@compute @workgroup_size(64)
fn member_rank_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = cfg.n_old + index(id);
    if i >= cfg.n_new {
        return;
    }
    let run = new_members(shape[at(H_BLOCK, i)]);
    var rank = 0u;
    for (var k = run.x; k < run.x + run.y; k++) {
        rank += u32(before(BY_DEPTH, bucket[at(U_MEMBERS, k)], i));
    }
    bucket[at(U_SORTED, run.x + rank)] = i;
}

// How many of `n` sorted words of `list` from `lo` stand before `x`.
fn below_sorted(lo: u32, n: u32, x: u32, in_cur: bool) -> u32 {
    var a = 0u;
    var b = n;
    for (var step = 0u; step < 32u && a < b; step++) {
        let mid = a + (b - a) / 2u;
        var y = bucket[at(U_SORTED, lo + mid)];
        if in_cur {
            y = cur[ls(R_MEMBERS, lo + mid)];
        }
        if before(BY_DEPTH, y, x) {
            a = mid + 1u;
        } else {
            b = mid;
        }
    }
    return a;
}

// Each old member moved to its block's new run, past the new members
// before it.
@compute @workgroup_size(64)
fn member_old_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.n_old {
        return;
    }
    let m = cur[ls(R_MEMBERS, k)];
    let b = shape[at(H_BLOCK, m)];
    let old = old_members(b);
    let fresh = new_members(b);
    let past = below_sorted(fresh.x, fresh.y, m, false);
    next[ls(R_MEMBERS, old.x + fresh.x + k - old.x + past)] = m;
}

// Each new member placed in its block's run, past the old members before
// it.
@compute @workgroup_size(64)
fn member_new_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let j = index(id);
    if j >= cfg.n_new - cfg.n_old {
        return;
    }
    let x = bucket[at(U_SORTED, j)];
    let old = old_members(shape[at(H_BLOCK, x)]);
    let past = below_sorted(old.x, old.y - old.x, x, true);
    next[ls(R_MEMBERS, old.x + j + past)] = x;
}

// Each block's start in the merged members.
@compute @workgroup_size(64)
fn member_start_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let b = index(id);
    let blocks = atomicLoad(&tally[X_BLOCKS]);
    if b >= blocks {
        return;
    }
    next[ls(R_MEM_START, b)] = old_members(b).x + new_members(b).x;
    if b + 1u == blocks {
        next[ls(R_MEM_START, blocks)] = cfg.n_new;
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
