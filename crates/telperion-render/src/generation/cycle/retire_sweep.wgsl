// fn-256 task 2: retirement's living carried up the hierarchy's block
// levels, a workgroup a block of `cfg.level`, its members by depth from
// the deepest, a barrier between depths (`hier_sweep.wgsl`'s shape). Each
// closed axis's subtree's living is its own, or the most its children
// carry up (`shed::fades`). Living folds by `max`, so it is fn-255's
// per-subtree evaluation (`retire::copied`, `subtree_living`) at every
// closed axis. Beside it, the latest cycle a woken bud in the subtree
// waits until.
//
// fn-256 task 10: an axis's living is carried up once, in the cycle it
// closes. Nothing in a closed subtree grows, wakes or is judged again (a
// candidate stands beside an open axis), so what it carried up then is
// what it would carry up in every later cycle; a retired root's wait is
// let go by its verdict (`verdict_main`). The sweep enters only the
// blocks holding an axis closed this cycle (`close_main`).

var<workgroup> w_run: vec4<u32>;
var<workgroup> w_at: array<u32, WG_AT>;
const WG_AT: u32 = WG + 1u;

// The workgroup's block: its members' first and end, and its least and
// greatest depth; an empty run past the level's blocks, or where `closing`
// asks for a block holding an axis closed this cycle and it holds none.
fn block_run(wid: vec3<u32>, lid: u32, closing: bool) -> vec4<u32> {
    if lid == 0u {
        var run = vec4<u32>(0u);
        if cfg.level < cfg.levels {
            let k = hlists[ls(R_LVL_START, cfg.level)] + wid.x + wid.y * cfg.wx;
            if k < hlists[ls(R_LVL_START, cfg.level + 1u)] {
                run = members_of(hlists[ls(R_LVL_BLOCKS, k)], closing);
            }
        }
        w_run = within(run, cfg.stride);
    }
    return workgroupUniformLoad(&w_run);
}

// Block `b`'s run, empty where `closing` and it holds no axis closed this
// cycle.
fn members_of(b: u32, closing: bool) -> vec4<u32> {
    if closing && !(b < cfg.stride && atomicLoad(&verdicts[block_stamp(b)]) == cfg.cycle) {
        return vec4<u32>(0u);
    }
    let ms = hlists[ls(R_MEM_START, b)];
    let me = hlists[ls(R_MEM_START, b + 1u)];
    let least = shape[H_DEPTH * cfg.stride + hlists[ls(R_MEMBERS, ms)]];
    return vec4<u32>(ms, me, least, shape[H_DEPTH * cfg.stride + hlists[ls(R_MEMBERS, me - 1u)]]);
}

// The first of the block's members at depth `d` or below.
fn first_at(run: vec4<u32>, d: u32) -> u32 {
    var lo = run.x;
    var hi = run.y;
    for (var step = 0u; step < 32u && lo < hi; step++) {
        let mid = lo + (hi - lo) / 2u;
        if shape[H_DEPTH * cfg.stride + hlists[ls(R_MEMBERS, mid)]] < d {
            lo = mid + 1u;
        } else {
            hi = mid;
        }
    }
    return lo;
}

// Where each of the depths `[d0, d0 + WG]` begins in the block, a lane a
// depth at once (`w_at`), the block's end past its deepest.
fn depth_starts(run: vec4<u32>, d0: u32, lid: u32) {
    for (var j = lid; j <= WG; j += WG) {
        let d = d0 + j;
        var first = run.y;
        if d <= run.z {
            first = run.x;
        } else if d <= run.w {
            first = first_at(run, d);
        }
        w_at[j] = first;
    }
    workgroupBarrier();
}

// The block's members at depth `d` of the depths `depth_starts` took
// from `d0`.
fn span_at(d: u32, d0: u32) -> vec2<u32> {
    return vec2<u32>(w_at[d - d0], w_at[d - d0 + 1u]);
}

// Closed axis `i`'s subtree's living and its latest wait: never judged
// before it closes, so neither retired nor inside a retired subtree.
fn carried_up(i: u32) {
    var lived = rtf(R_LIVED0, i);
    var alive = 0.0;
    var wait = rt(R_WAKE, i);
    let kids = kids_within(vec2<u32>(hlists[ls(R_KID_START, i)], hlists[ls(R_KID_START, i + 1u)]), cfg.stride);
    // Each child's part, as `terms_main` took it: four children's words
    // loaded before any is folded, in order.
    var k = kids.x;
    for (; k + 4u <= kids.y; k += 4u) {
        let c = vec4<u32>(hlists[ls(R_KIDS, k)], hlists[ls(R_KIDS, k + 1u)], hlists[ls(R_KIDS, k + 2u)], hlists[ls(R_KIDS, k + 3u)]);
        let weight = vec4<f32>(rtf(R_WEIGHT, c.x), rtf(R_WEIGHT, c.y), rtf(R_WEIGHT, c.z), rtf(R_WEIGHT, c.w));
        let before = vec4<f32>(rtf(R_BEFORE, c.x), rtf(R_BEFORE, c.y), rtf(R_BEFORE, c.z), rtf(R_BEFORE, c.w));
        let gap = vec4<f32>(rtf(R_GAP, c.x), rtf(R_GAP, c.y), rtf(R_GAP, c.z), rtf(R_GAP, c.w));
        let sub = vec4<f32>(rtf(R_LIVED, c.x), rtf(R_LIVED, c.y), rtf(R_LIVED, c.z), rtf(R_LIVED, c.w));
        let live = vec4<f32>(rtf(R_ALIVE, c.x), rtf(R_ALIVE, c.y), rtf(R_ALIVE, c.z), rtf(R_ALIVE, c.w));
        let waits = vec4<u32>(rt(R_WSUB, c.x), rt(R_WSUB, c.y), rt(R_WSUB, c.z), rt(R_WSUB, c.w));
        for (var j = 0u; j < 4u; j++) {
            lived = max(lived, before[j] + m(weight[j], gap[j] + sub[j]));
            alive = max(alive, m(live[j], weight[j]));
            wait = max(wait, waits[j]);
        }
    }
    for (; k < kids.y; k++) {
        let c = hlists[ls(R_KIDS, k)];
        let weight = rtf(R_WEIGHT, c);
        lived = max(lived, rtf(R_BEFORE, c) + m(weight, rtf(R_GAP, c) + rtf(R_LIVED, c)));
        alive = max(alive, m(rtf(R_ALIVE, c), weight));
        wait = max(wait, rt(R_WSUB, c));
    }
    set_rtf(R_LIVED, i, lived);
    set_rtf(R_ALIVE, i, alive);
    set_rt(R_WSUB, i, wait);
}

// An axis's living carried up, where it is laid and closed this cycle.
fn living(i: u32) {
    if i < cfg.n && rt(R_CLOSED, i) != 0u {
        carried_up(i);
    }
}

@compute @workgroup_size(WG)
fn living_main(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    let run = block_run(wid, lid, true);
    if run.y == run.x {
        return;
    }
    let depths = run.w - run.z + 1u;
    for (var c = 0u; c < depths; c += WG) {
        let d0 = run.w + 1u - min(c + WG, depths);
        depth_starts(run, d0, lid);
        for (var d = run.w - c; d + 1u > d0; d--) {
            let span = span_at(d, d0);
            for (var k = span.x + lid; k < span.y; k += WG) {
                living(hlists[ls(R_MEMBERS, k)]);
            }
            storageBarrier();
            workgroupBarrier();
        }
    }
}

// Every axis below a retired root marked inside it, a root under a later
// one among them (`retire::let_go`), from the root's level down.
fn inside(i: u32) {
    let p = select(NONE, ax(i, A_PARENT), i < cfg.n);
    if p != NONE && rt(R_FATE, p) >= RETIRED {
        set_rt(R_FATE, i, INSIDE);
    }
}

@compute @workgroup_size(WG)
fn inside_main(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    let run = block_run(wid, lid, false);
    if run.y == run.x {
        return;
    }
    let depths = run.w - run.z + 1u;
    for (var c = 0u; c < depths; c += WG) {
        let d0 = run.z + c;
        depth_starts(run, d0, lid);
        for (var d = d0; d < d0 + min(WG, depths - c); d++) {
            let span = span_at(d, d0);
            for (var k = span.x + lid; k < span.y; k += WG) {
                inside(hlists[ls(R_MEMBERS, k)]);
            }
            storageBarrier();
            workgroupBarrier();
        }
    }
}

// The folded sweeps' blocks (`fold.wgsl`), as `block_run` and `first_at`
// take them.
fn fold_level(l: u32) -> vec2<u32> {
    return vec2<u32>(hlists[ls(R_LVL_START, l)], hlists[ls(R_LVL_START, l + 1u)]);
}

fn fold_run(op: u32, k: u32) -> vec4<u32> {
    return within(members_of(hlists[ls(R_LVL_BLOCKS, k)], op == 0u), cfg.stride);
}

fn fold_lower(run: vec4<u32>, d: u32) -> u32 {
    return first_at(run, d);
}

fn fold_member(k: u32) -> u32 {
    return hlists[ls(R_MEMBERS, k)];
}

fn fold_apply(op: u32, i: u32) {
    if op == 0u {
        living(i);
    } else {
        inside(i);
    }
}

// Each sweep's folded levels: the top's few and the deepest.
@compute @workgroup_size(FOLDED)
fn living_fold_main(@builtin(local_invocation_index) lid: u32) {
    folded(0u, true, lid);
}

@compute @workgroup_size(FOLDED)
fn inside_fold_main(@builtin(local_invocation_index) lid: u32) {
    folded(1u, false, lid);
}
