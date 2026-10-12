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
var<workgroup> w_span: vec2<u32>;

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
    if closing && !(b < cfg.stride && rt(R_BLOCKS, b) == cfg.cycle) {
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

fn span_at(run: vec4<u32>, d: u32, lid: u32) -> vec2<u32> {
    if lid == 0u {
        w_span = vec2<u32>(first_at(run, d), first_at(run, d + 1u));
    }
    return workgroupUniformLoad(&w_span);
}

// Axis `c`'s vigour as its parent `i`'s subtree weighs it: its making
// draws' presences, a woken bud's by its bearer's kept share, and an axis
// carried on by its parent's end (`presence::assign`).
fn vigour(c: u32, i: u32) -> f32 {
    var b1 = q64(ax64(c, A_B1));
    if rt(R_WAKE, c) != 0u {
        b1 = rtf(R_QB1, c);
    }
    let made = m(q64(ax64(c, A_B0)), b1);
    let kind = kind_of(c);
    if kind == CONTINUED || kind == RELAYED {
        return m(rtf(R_END, i), made);
    }
    return made;
}

// Closed axis `i`'s subtree's living and its latest wait: never judged
// before it closes, so neither retired nor inside a retired subtree.
fn carried_up(i: u32) {
    var lived = rtf(R_LIVED0, i);
    var alive = 0.0;
    var wait = rt(R_WAKE, i);
    let birth = ax(i, A_BIRTH);
    let units_n = ax(i, A_UNITS);
    let kids = kids_within(vec2<u32>(hlists[ls(R_KID_START, i)], hlists[ls(R_KID_START, i + 1u)]), cfg.stride);
    for (var k = kids.x; k < kids.y; k++) {
        let c = hlists[ls(R_KIDS, k)];
        var weight = qf(vigour(c, i));
        if kind_of(c) == LATERAL {
            weight = m(weight, qf(node_scale(i, ax(c, A_NODE))));
        }
        let span = ax(c, A_BIRTH) - min(birth, ax(c, A_BIRTH));
        let upto = min(span, units_n);
        var before = 0.0;
        if upto != 0u {
            before = tick_q(ax(i, A_SLEEP)) + presences(i, upto).x;
        }
        lived = max(lived, before + m(weight, f32(span - upto) + rtf(R_LIVED, c)));
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
    for (var h = 0u; h <= run.w - run.z; h++) {
        let span = span_at(run, run.w - h, lid);
        for (var k = span.x + lid; k < span.y; k += WG) {
            living(hlists[ls(R_MEMBERS, k)]);
        }
        storageBarrier();
        workgroupBarrier();
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
    for (var d = run.z; d <= run.w; d++) {
        let span = span_at(run, d, lid);
        for (var k = span.x + lid; k < span.y; k += WG) {
            inside(hlists[ls(R_MEMBERS, k)]);
        }
        storageBarrier();
        workgroupBarrier();
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
