// fn-256 task 2 (DESIGN.md section 3): the sums by block level, a
// workgroup a block of `cfg.level`, its members by depth over the block's
// range of depths, deepest first upward and shallowest first downward, a
// barrier between depths.
// Each axis takes its children in fn-255's order, so any schedule that
// finishes them first gives the CPU's sums: the bottom-up `totals` and
// `sub`, the top-down `walk` and `split`.

var<workgroup> w_run: vec4<u32>;
var<workgroup> w_at: array<u32, WG_AT>;
const WG_AT: u32 = WG + 1u;

// The workgroup's block: its members' first and end, and its least and
// greatest depth (its first member's and its last's); an empty run where
// there is none.
fn open_block(wid: vec3<u32>, lid: u32) -> vec4<u32> {
    if lid == 0u {
        let first = lists[ls(R_LVL_START, cfg.level)];
        let k = first + wid.x + wid.y * cfg.wx;
        var run = vec4<u32>(0u);
        if cfg.cycle != 0u && cfg.level < cfg.levels && k < lists[ls(R_LVL_START, cfg.level + 1u)] {
            let b = lists[ls(R_LVL_BLOCKS, k)];
            let ms = lists[ls(R_MEM_START, b)];
            let me = lists[ls(R_MEM_START, b + 1u)];
            let least = shape[at(H_DEPTH, lists[ls(R_MEMBERS, ms)])];
            run = vec4<u32>(ms, me, least, shape[at(H_DEPTH, lists[ls(R_MEMBERS, me - 1u)])]);
        }
        w_run = within(run, cfg.stride);
    }
    return workgroupUniformLoad(&w_run);
}

// The first of the block's members at depth `d` or below.
fn lower(run: vec4<u32>, d: u32) -> u32 {
    var lo = run.x;
    var hi = run.y;
    for (var step = 0u; step < 32u && lo < hi; step++) {
        let mid = lo + (hi - lo) / 2u;
        if shape[at(H_DEPTH, lists[ls(R_MEMBERS, mid)])] < d {
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
            first = lower(run, d);
        }
        w_at[j] = first;
    }
    workgroupBarrier();
}

// The block's members at depth `d` of the depths `depth_starts` took
// from `d0`.
fn depth_span(d: u32, d0: u32) -> vec2<u32> {
    return vec2<u32>(w_at[d - d0], w_at[d - d0 + 1u]);
}

// Child slots `k` to `k + 3`'s axes.
fn kids4(k: u32) -> vec4<u32> {
    return vec4<u32>(kid(k), kid(k + 1u), kid(k + 2u), kid(k + 3u));
}

fn children(i: u32) -> vec2<u32> {
    return kids_within(vec2<u32>(lists[ls(R_KID_START, i)], lists[ls(R_KID_START, i + 1u)]), cfg.stride);
}

// An axis's light, upkeep and living (`Working::totals`): its own, then
// each child's in order.
fn totals(i: u32) {
    // A retired root whose subtree the arena let go stands for it by its
    // frozen upkeep, unlit and not living, as `Working::frozen` charges it.
    let frozen = tree[at(T_FROZEN, i)];
    if frozen != NONE {
        put(S_LIGHT, i, 0.0);
        put(S_COST, i, bitcast<f32>(frozen));
        sums[at(S_LIVING, i)] = 0u;
        return;
    }
    let lit = sums[at(S_LIT, i)] != 0u;
    var light = 0.0;
    if lit {
        light = tf(T_BASE, i) * tf(T_RUNNING, i) * single(wide(T_LIGHT, i));
    }
    var cost = row(tree[at(T_PA, i)], P_UPKEEP) * single(wide(T_WOOD, i));
    var living = lit;
    let kids = children(i);
    var k = kids.x;
    // Four children's words loaded before any is added, in order.
    for (; k + 4u <= kids.y; k += 4u) {
        let c = kids4(k);
        let l = vec4<f32>(sf(S_LIGHT, c.x), sf(S_LIGHT, c.y), sf(S_LIGHT, c.z), sf(S_LIGHT, c.w));
        let q = vec4<f32>(sf(S_COST, c.x), sf(S_COST, c.y), sf(S_COST, c.z), sf(S_COST, c.w));
        let v = sums[at(S_LIVING, c.x)] | sums[at(S_LIVING, c.y)] | sums[at(S_LIVING, c.z)] | sums[at(S_LIVING, c.w)];
        light = light + l.x;
        light = light + l.y;
        light = light + l.z;
        light = light + l.w;
        cost = cost + q.x;
        cost = cost + q.y;
        cost = cost + q.z;
        cost = cost + q.w;
        living = living || v != 0u;
    }
    for (; k < kids.y; k++) {
        let c = kid(k);
        light = light + sf(S_LIGHT, c);
        cost = cost + sf(S_COST, c);
        living = living || sums[at(S_LIVING, c)] != 0u;
    }
    put(S_LIGHT, i, light);
    put(S_COST, i, cost);
    sums[at(S_LIVING, i)] = u32(living);
}

@compute @workgroup_size(WG)
fn totals_main(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    let run = open_block(wid, lid);
    if run.y == run.x {
        return;
    }
    let depths = run.w - run.z + 1u;
    for (var c = 0u; c < depths; c += WG) {
        let d0 = run.w + 1u - min(c + WG, depths);
        depth_starts(run, d0, lid);
        for (var d = run.w - c; d + 1u > d0; d--) {
            let span = depth_span(d, d0);
            for (var k = span.x + lid; k < span.y; k += WG) {
                totals(lists[ls(R_MEMBERS, k)]);
            }
            storageBarrier();
            workgroupBarrier();
        }
    }
}

// `(q - c) / (q + c)`; one where `q + c` is none or less.
fn now(q: f32, c: f32) -> f32 {
    let total = q + c;
    if total <= 0.0 {
        return 1.0;
    }
    return hdiv(q - c, total);
}

// A draw's lead in log-odds over the threshold `t` (`presence::over`).
fn over53(t: vec2<u32>, d: vec2<u32>) -> f32 {
    if is_zero(t) {
        return INF;
    }
    return logit53(d) - logit53(t);
}

// D16's walk at one axis (`contract::shed::walk`): its bearer's shedding
// and kept share, and where it is a living lateral under a living bearer,
// its own balance, its memory kept.
fn walk(i: u32) {
    let p = tree[at(T_PARENT, i)];
    var dead = false;
    var keep = 1.0;
    if p != NONE {
        dead = sums[at(S_SHED, p)] != 0u;
        keep = sf(S_KEEP, p);
    }
    let decides = p != NONE && tree[at(T_NODE, i)] != NONE && sums[at(S_LIVING, i)] != 0u;
    if !dead && decides {
        let pa = tree[at(T_PA, i)];
        let memory = (tf(T_MEMORY, i) + now(sf(S_LIGHT, i), sf(S_COST, i))) * 0.5;
        put(S_MEMORY, i, memory);
        let hazard = row(pa, P_HAZARD) * max(row(pa, P_TOLERANCE) - memory, 0.0);
        if hazard > 0.0 {
            let key = child(child(wide(T_LINEAGE, i), KEY_SHED), vec2<u32>(cfg.cycle, 0u));
            let d = draw(key);
            let t = threshold(fails(hazard * HLOG2E));
            if lt64(d, t) {
                dead = true;
            } else if !is_zero(t) {
                keep = keep * presence(over53(t, d), row(pa, P_WINDOW));
            }
        }
    }
    sums[at(S_SHED, i)] = u32(dead);
    put(S_KEEP, i, keep);
}

@compute @workgroup_size(WG)
fn walk_main(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    let run = open_block(wid, lid);
    if run.y == run.x {
        return;
    }
    let depths = run.w - run.z + 1u;
    for (var c = 0u; c < depths; c += WG) {
        let d0 = run.z + c;
        depth_starts(run, d0, lid);
        for (var d = d0; d < d0 + min(WG, depths - c); d++) {
            let span = depth_span(d, d0);
            for (var k = span.x + lid; k < span.y; k += WG) {
                walk(lists[ls(R_MEMBERS, k)]);
            }
            storageBarrier();
            workgroupBarrier();
        }
    }
}

// An axis's subtree's lit presence (`Working::split`'s first pass) and
// growing buds: its own, then each child's in order.
fn sub(i: u32) {
    var lit = sf(S_OWN, i);
    var buds = sums[at(S_GROWING, i)];
    let kids = children(i);
    var k = kids.x;
    for (; k + 4u <= kids.y; k += 4u) {
        let c = kids4(k);
        let l = vec4<f32>(sf(S_SUB, c.x), sf(S_SUB, c.y), sf(S_SUB, c.z), sf(S_SUB, c.w));
        lit = lit + l.x;
        lit = lit + l.y;
        lit = lit + l.z;
        lit = lit + l.w;
        buds += sums[at(S_BUDS, c.x)] + sums[at(S_BUDS, c.y)] + sums[at(S_BUDS, c.z)] + sums[at(S_BUDS, c.w)];
    }
    for (; k < kids.y; k++) {
        let c = kid(k);
        lit = lit + sf(S_SUB, c);
        buds += sums[at(S_BUDS, c)];
    }
    put(S_SUB, i, lit);
    sums[at(S_BUDS, i)] = buds;
}

@compute @workgroup_size(WG)
fn sub_main(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    let run = open_block(wid, lid);
    if run.y == run.x {
        return;
    }
    let depths = run.w - run.z + 1u;
    for (var c = 0u; c < depths; c += WG) {
        let d0 = run.w + 1u - min(c + WG, depths);
        depth_starts(run, d0, lid);
        for (var d = run.w - c; d + 1u > d0; d--) {
            let span = depth_span(d, d0);
            for (var k = span.x + lid; k < span.y; k += WG) {
                sub(lists[ls(R_MEMBERS, k)]);
            }
            storageBarrier();
            workgroupBarrier();
        }
    }
}

// The folded sweeps' blocks (`fold.wgsl`): a level's first block and end,
// a block's run, its first member at a depth, and a member.
fn fold_level(l: u32) -> vec2<u32> {
    return vec2<u32>(lists[ls(R_LVL_START, l)], lists[ls(R_LVL_START, l + 1u)]);
}

fn fold_run(op: u32, k: u32) -> vec4<u32> {
    let b = lists[ls(R_LVL_BLOCKS, k)];
    let ms = lists[ls(R_MEM_START, b)];
    let me = lists[ls(R_MEM_START, b + 1u)];
    let least = shape[at(H_DEPTH, lists[ls(R_MEMBERS, ms)])];
    return within(vec4<u32>(ms, me, least, shape[at(H_DEPTH, lists[ls(R_MEMBERS, me - 1u)])]), cfg.stride);
}

fn fold_lower(run: vec4<u32>, d: u32) -> u32 {
    return lower(run, d);
}

fn fold_member(k: u32) -> u32 {
    return lists[ls(R_MEMBERS, k)];
}

const OP_TOTALS: u32 = 0u;
const OP_WALK: u32 = 1u;
const OP_SUB: u32 = 2u;
const OP_SPLIT: u32 = 3u;

fn fold_apply(op: u32, i: u32) {
    switch op {
        case OP_TOTALS: {
            totals(i);
        }
        case OP_WALK: {
            walk(i);
        }
        case OP_SUB: {
            sub(i);
        }
        default: {
            split(i);
        }
    }
}

// Each sweep's folded levels: the top's few and the deepest.
@compute @workgroup_size(FOLDED)
fn totals_fold_main(@builtin(local_invocation_index) lid: u32) {
    if cfg.cycle != 0u {
        folded(OP_TOTALS, true, lid);
    }
}

@compute @workgroup_size(FOLDED)
fn walk_fold_main(@builtin(local_invocation_index) lid: u32) {
    if cfg.cycle != 0u {
        folded(OP_WALK, false, lid);
    }
}

@compute @workgroup_size(FOLDED)
fn sub_fold_main(@builtin(local_invocation_index) lid: u32) {
    if cfg.cycle != 0u {
        folded(OP_SUB, true, lid);
    }
}

@compute @workgroup_size(FOLDED)
fn split_fold_main(@builtin(local_invocation_index) lid: u32) {
    if cfg.cycle != 0u {
        folded(OP_SPLIT, false, lid);
    }
}
