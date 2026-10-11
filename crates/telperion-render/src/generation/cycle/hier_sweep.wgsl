// fn-256 task 2 (DESIGN.md section 3): the sums by block level, a
// workgroup a block of `cfg.level`, its members by local depth, deepest
// first upward and shallowest first downward, a barrier between depths.
// Each axis takes its children in fn-255's order, so any schedule that
// finishes them first gives the CPU's sums: the bottom-up `totals` and
// `sub`, the top-down `walk` and `split`.

var<workgroup> w_run: vec4<u32>;
var<workgroup> w_span: vec2<u32>;

// The workgroup's block: its members' first and end, its deepest local
// depth, and whether there is one.
fn open_block(wid: vec3<u32>, lid: u32) -> vec4<u32> {
    if lid == 0u {
        let first = lists[ls(R_LVL_START, cfg.level)];
        let k = first + wid.x + wid.y * cfg.wx;
        var run = vec4<u32>(0u);
        if k < lists[ls(R_LVL_START, cfg.level + 1u)] {
            let b = lists[ls(R_LVL_BLOCKS, k)];
            let ms = lists[ls(R_MEM_START, b)];
            let me = lists[ls(R_MEM_START, b + 1u)];
            run = vec4<u32>(ms, me, shape[at(H_LOCAL, lists[ls(R_MEMBERS, me - 1u)])], 1u);
        }
        w_run = run;
    }
    return workgroupUniformLoad(&w_run);
}

// The first of the block's members at local depth `d` or below.
fn lower(run: vec4<u32>, d: u32) -> u32 {
    var lo = run.x;
    var hi = run.y;
    for (var step = 0u; step < 32u && lo < hi; step++) {
        let mid = lo + (hi - lo) / 2u;
        if shape[at(H_LOCAL, lists[ls(R_MEMBERS, mid)])] < d {
            lo = mid + 1u;
        } else {
            hi = mid;
        }
    }
    return lo;
}

// The block's members at local depth `d`.
fn depth_span(run: vec4<u32>, d: u32, lid: u32) -> vec2<u32> {
    if lid == 0u {
        w_span = vec2<u32>(lower(run, d), lower(run, d + 1u));
    }
    return workgroupUniformLoad(&w_span);
}

fn children(i: u32) -> vec2<u32> {
    return vec2<u32>(lists[ls(R_KID_START, i)], lists[ls(R_KID_START, i + 1u)]);
}

// An axis's light, upkeep and living (`Working::totals`): its own, then
// each child's in order.
fn totals(i: u32) {
    let lit = sums[at(S_LIT, i)] != 0u;
    var light = 0.0;
    if lit {
        light = tf(T_BASE, i) * tf(T_RUNNING, i) * single(wide(T_LIGHT, i));
    }
    var cost = row(tree[at(T_PA, i)], P_UPKEEP) * single(wide(T_WOOD, i));
    var living = lit;
    let kids = children(i);
    for (var k = kids.x; k < kids.y; k++) {
        let c = kid(k);
        light = light + sf(S_LIGHT, c);
        cost = cost + sf(S_COST, c);
        living = living || sums[at(S_LIVING, c)] != 0u;
    }
    put(S_LIGHT, i, light);
    put(S_COST, i, cost);
    sums[at(S_LIVING, i)] = u32(living);
}

@compute @workgroup_size(64)
fn totals_main(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    let run = open_block(wid, lid);
    if run.w == 0u {
        return;
    }
    for (var h = 0u; h <= run.z; h++) {
        let span = depth_span(run, run.z - h, lid);
        for (var k = span.x + lid; k < span.y; k += WG) {
            totals(lists[ls(R_MEMBERS, k)]);
        }
        storageBarrier();
        workgroupBarrier();
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

@compute @workgroup_size(64)
fn walk_main(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    let run = open_block(wid, lid);
    if run.w == 0u {
        return;
    }
    for (var d = 0u; d <= run.z; d++) {
        let span = depth_span(run, d, lid);
        for (var k = span.x + lid; k < span.y; k += WG) {
            walk(lists[ls(R_MEMBERS, k)]);
        }
        storageBarrier();
        workgroupBarrier();
    }
}

// An axis's subtree's lit presence (`Working::split`'s first pass) and
// growing buds: its own, then each child's in order.
fn sub(i: u32) {
    var lit = sf(S_OWN, i);
    var buds = sums[at(S_GROWING, i)];
    let kids = children(i);
    for (var k = kids.x; k < kids.y; k++) {
        let c = kid(k);
        lit = lit + sf(S_SUB, c);
        buds += sums[at(S_BUDS, c)];
    }
    put(S_SUB, i, lit);
    sums[at(S_BUDS, i)] = buds;
}

@compute @workgroup_size(64)
fn sub_main(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    let run = open_block(wid, lid);
    if run.w == 0u {
        return;
    }
    for (var h = 0u; h <= run.z; h++) {
        let span = depth_span(run, run.z - h, lid);
        for (var k = span.x + lid; k < span.y; k += WG) {
            sub(lists[ls(R_MEMBERS, k)]);
        }
        storageBarrier();
        workgroupBarrier();
    }
}
