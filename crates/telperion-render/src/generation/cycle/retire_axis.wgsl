// fn-256 task 2: retirement's per-axis passes. The open axes marked from
// each living apex and sleeping bud to the root (`Working::open_from`);
// each laid axis closed this cycle flagged, and its presences assigned
// once (`presence::assign_one`, as `retire::copied` takes them); each
// woken bud's birth presence by its bearer's kept share (`Woken::kept`);
// then, once the levels have carried living up, each candidate judged
// (`retire::evaluate`). Every presence an f32, each product in the CPU's
// order and kept apart from the sum it feeds.

// The axis this invocation's apex or sleeping bud stands on, none past
// them or unlaid.
fn source(k: u32) -> u32 {
    var a = NONE;
    if k < cfg.live {
        a = lists[cfg.next_at + k];
    } else if k < cfg.live + cfg.sleepers {
        let s = k - cfg.live;
        if lists[cfg.sleep_at + S_WAKE * cfg.sleep_cap + s] >= cfg.cycle {
            a = lists[cfg.sleep_at + S_PARENT * cfg.sleep_cap + s];
        }
    }
    return select(NONE, a, a < cfg.n);
}

// Each living apex's and sleeping bud's way to the root marked open; a
// walk stops where another has marked it, which walks on to the root.
@compute @workgroup_size(64)
fn open_main(@builtin(global_invocation_id) id: vec3<u32>) {
    var a = source(index(id));
    // The tree's depth, each axis marked once; no walk passes the laid
    // axes, whatever a halted cycle left (fn-256 task 5).
    for (var steps = 0u; steps < cfg.n && a < cfg.n && rt(R_OPEN, a) != cfg.cycle; steps++) {
        set_rt(R_OPEN, a, cfg.cycle);
        a = ax(a, A_PARENT);
    }
}

// The presences of axis `i`'s first `upto` units summed in order, from
// none (`shed::fades`' `grown`), and its running presence past them.
fn presences(i: u32, upto: u32) -> vec2<f32> {
    var running = 1.0;
    var sum = 0.0;
    var r = ax(i, A_HEAD);
    // The axis's units, no more than the units' rows.
    for (var k = 0u; k < min(upto, cfg.units_cap) && r < cfg.units_cap; k++) {
        running = m(running, qf(bitcast<f32>(unit(r, U_SURVIVE))));
        sum = sum + m(running, tick_q(unit(r, U_SHARE)));
        running = m(running, qf(bitcast<f32>(unit(r, U_PERSIST))));
        r = unit(r, U_NEXT);
    }
    return vec2<f32>(sum, running);
}

// The assigned scale of axis `i`'s phytomer `node`, matched to its unit
// by its cycle (`assign_one`).
fn node_scale(i: u32, node: u32) -> f32 {
    let birth = ax(i, A_BIRTH);
    var running = 1.0;
    var r = ax(i, A_HEAD);
    var j = 0u;
    // The axis's units, no more than the units' rows.
    for (var k = 0u; k < cfg.units_cap && r < cfg.units_cap; k++) {
        running = m(running, qf(bitcast<f32>(unit(r, U_SURVIVE))));
        let laid = tick_q(unit(r, U_SIZE));
        // The unit's phytomers, in order; phytomer `node` is one of them
        // where its cycle matches.
        let first = unit(r, U_FIRST);
        for (var t = 0u; t < min(unit(r, U_COUNT), cfg.phyts_cap); t++) {
            let row = first + t;
            if i32(phyt(row, P_CYCLE)) - i32(birth) - 1 == i32(k) {
                if j == node {
                    return m(m(running, qf(bitcast<f32>(phyt(row, P_NODE)))), laid);
                }
                j += 1u;
            }
        }
        running = m(running, qf(bitcast<f32>(unit(r, U_PERSIST))));
        r = unit(r, U_NEXT);
    }
    return 0.0;
}

// Each laid axis closed this cycle flagged (open last cycle, or laid
// since, and open no more), and every closed axis's own living time and
// end's presence assigned once.
@compute @workgroup_size(64)
fn close_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n {
        return;
    }
    let open = rt(R_OPEN, i);
    let fresh = i >= ret[cfg.head + HD_PREV_LAID];
    let closed = open != cfg.cycle && (open + 1u == cfg.cycle || fresh);
    set_rc(C_CLOSED, i, u32(closed));
    // Its block is entered by the living's sweep (fn-256 task 10).
    let b = shape[H_BLOCK * cfg.stride + i];
    if closed && b < cfg.stride {
        atomicStore(&verdicts[block_stamp(b)], cfg.cycle);
    }
    if !closed {
        return;
    }
    let p = presences(i, ax(i, A_UNITS));
    set_rcf(C_LIVED0, i, tick_q(ax(i, A_SLEEP)) + p.x);
    set_rcf(C_END, i, p.y);
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
        return m(rcf(C_END, i), made);
    }
    return made;
}

// Each laid axis whose bearer closed this cycle: its part in the living
// the bearer carries up (`carried_up`), each as `shed::fades` forms it:
// its weight, the bearer's living before it was made, and the cycles
// between. Taken once, in a pass of its own: inside the sweep, the loops
// over the bearer's units and phytomers cost every block's workgroup
// as much as the axes it took (DESIGN.md section 16).
@compute @workgroup_size(64)
fn terms_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = index(id);
    if c >= cfg.n {
        return;
    }
    let i = ax(c, A_PARENT);
    if !(i < cfg.n && rc(C_CLOSED, i) != 0u) {
        return;
    }
    var weight = qf(vigour(c, i));
    if kind_of(c) == LATERAL {
        weight = m(weight, qf(node_scale(i, ax(c, A_NODE))));
    }
    let span = ax(c, A_BIRTH) - min(ax(i, A_BIRTH), ax(c, A_BIRTH));
    let upto = min(span, ax(i, A_UNITS));
    var before = 0.0;
    if upto != 0u {
        before = tick_q(ax(i, A_SLEEP)) + presences(i, upto).x;
    }
    set_rcf(C_WEIGHT, c, weight);
    set_rcf(C_BEFORE, c, before);
    set_rcf(C_GAP, c, f32(span - upto));
}

// The presences of the draws that made `a`'s successors, summed in order
// from Rust's -0.0.
fn successors_on(a: u32) -> f32 {
    var on = bitcast<f32>(0x80000000u);
    for (var k = 0u; k < 2u; k++) {
        let n = ax(a, A_SUCC0 + k);
        if n != NONE {
            on = on + m(q64(ax64(n, A_B0)), q64(ax64(n, A_B1)));
        }
    }
    return on;
}

// How much of its size a woken bud keeps as the tree stands
// (`Woken::kept`; a bearer is never `alive` before the settle marks it).
fn woken_kept(k: u32) -> f32 {
    let at = cfg.woken_at + k;
    let bearer = lists[at + W_BEARER * cfg.woken_cap];
    let next = lists[at + W_NEXT * cfg.woken_cap];
    let cycle = lists[at + W_CYCLE * cfg.woken_cap];
    let share = tick_q(lists[at + W_SHARE * cfg.woken_cap]);
    let flags = ax(bearer, A_FLAGS);
    let outlived = (flags & F_OUTLIVED) != 0u;
    let used = tick_q(ax(bearer, A_OUTLIVED));
    let end = ax(bearer, A_END);
    let alone = ax(bearer, A_SUCC0) == NONE;
    if outlived && end == cycle + 1u && alone {
        return max(used + share - 1.0, 0.0);
    }
    var p = 1.0;
    if cycle < cfg.age {
        var u = NONE;
        if next < ax(bearer, A_UNITS) {
            u = ax(bearer, A_HEAD);
            // The bearer's units, no more than the units' rows.
            for (var s = 0u; s < min(next, cfg.units_cap); s++) {
                u = unit(u, U_NEXT);
            }
        }
        if u == NONE {
            p = successors_on(bearer);
        } else {
            let stopped = end == cycle + 2u && (flags & F_FAILED) == 0u;
            let carried = m(qf(bitcast<f32>(unit(u, U_SURVIVE))), qf(bitcast<f32>(unit(u, U_PERSIST))));
            p = carried;
            if stopped && outlived && alone {
                p = m(carried, used);
            } else if stopped {
                p = m(carried, successors_on(bearer));
            }
        }
    }
    return share + m(1.0 - share, p);
}

// Each woken bud's wait (its cycle three on) and its birth presence by
// its bearer's kept share now.
@compute @workgroup_size(64)
fn woken_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.woken {
        return;
    }
    let a = lists[cfg.woken_at + W_AXIS * cfg.woken_cap + k];
    if a >= cfg.n {
        return;
    }
    set_rt(R_WAKE, a, lists[cfg.woken_at + W_CYCLE * cfg.woken_cap + k] + 3u);
    set_rtf(R_QB1, a, m(q64(ax64(a, A_B1)), qf(woken_kept(k))));
}

// The log's next entry, a fault past its most.
// The log is a test's alone: a build that asks for none keeps none.
fn logged(i: u32, verdict: u32, lived: f32, alive: f32) {
    if cfg.log_cap == 0u {
        return;
    }
    let k = atomicAdd(&verdicts[L_COUNT], 1u);
    if k >= cfg.log_cap {
        atomicOr(&verdicts[L_FAULT], 1u);
        return;
    }
    // The root by its lineage: its slot moves as the arena compacts.
    let at = L_AT + 6u * k;
    let key = ax64(i, A_LINEAGE);
    let words = array<u32, 6>(key.x, key.y, cfg.cycle, verdict, bitcast<u32>(lived), bitcast<u32>(alive));
    for (var w = 0u; w < 6u; w++) {
        atomicStore(&verdicts[at + w], words[w]);
    }
}

// Axis `i`'s fade at the age grown (D17, `shed::faded`), its subtree's
// living `lived` and `alive`: whole where it is no lateral that sheds.
fn faded_at(i: u32, lived: f32, alive: f32) -> f32 {
    let pa = ax(i, A_PA);
    if kind_of(i) != LATERAL || (kpa(pa, K_FLAGS) & KF_SHEDS) == 0u {
        return 1.0;
    }
    return fade(bitcast<f32>(kpa(pa, K_DELAY)), ax(i, A_BIRTH), lived, alive, cfg.age);
}

// Axis `i` retired: it stands for its subtree by the living it carried
// up, and waits on nothing more, its bearer reading no wait from it.
fn retired(i: u32, lived: f32, alive: f32) {
    set_rt(R_FATE, i, RETIRED);
    set_rtf(R_ROOT_LIVED, i, lived);
    set_rtf(R_ROOT_ALIVE, i, alive);
    set_rt(R_WSUB, i, 0u);
    logged(i, RETIRED, lived, alive);
}

// Each candidate judged: closed this cycle or waiting, beside an open
// axis, not judged: waiting while a bud in it woke under three cycles
// ago, else retired where it is a lateral whose fade at the age grown
// is none (`shed::faded`), else kept.
@compute @workgroup_size(64)
fn verdict_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n {
        return;
    }
    let waited = rt(R_PEND, i) != 0u;
    set_rt(R_PEND, i, 0u);
    let p = ax(i, A_PARENT);
    let beside = p == NONE || rt(R_OPEN, p) == cfg.cycle;
    if !(rc(C_CLOSED, i) != 0u || waited) || !beside || rt(R_FATE, i) != 0u {
        return;
    }
    if rt(R_WSUB, i) > cfg.cycle {
        set_rt(R_PEND, i, 1u);
        logged(i, 0u, 0.0, 0.0);
        return;
    }
    let lived = rtf(R_LIVED, i);
    let alive = rtf(R_ALIVE, i);
    if faded_at(i, lived, alive) <= 0.0 {
        retired(i, lived, alive);
    } else {
        set_rt(R_FATE, i, KEPT);
        logged(i, KEPT, lived, alive);
    }
}

// Whether a subtree retired before `i`, or retiring with it this cycle,
// holds it: an ancestor retired or inside one, or kept and faded out at
// the age grown. Whichever its own invocation has written yet, an
// ancestor reads the same.
fn held_inside(i: u32) -> bool {
    var a = ax(i, A_PARENT);
    // The tree's depth, no walk past the laid axes.
    for (var steps = 0u; steps < cfg.n && a < cfg.n; steps++) {
        let f = rt(R_FATE, a);
        if f >= RETIRED || (f == KEPT && faded_at(a, rtf(R_LIVED, a), rtf(R_ALIVE, a)) <= 0.0) {
            return true;
        }
        a = ax(a, A_PARENT);
    }
    return false;
}

// Each kept lateral that sheds, its living final, retired once its fade
// at the age grown reaches none (fn-270 R3, `retire::faded_out`); one a
// retired subtree holds goes inside it unlogged. After the verdicts, so
// a subtree retired this cycle holds what fades out below it.
@compute @workgroup_size(64)
fn fading_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n || rt(R_FATE, i) != KEPT {
        return;
    }
    let lived = rtf(R_LIVED, i);
    let alive = rtf(R_ALIVE, i);
    if faded_at(i, lived, alive) > 0.0 {
        return;
    }
    if held_inside(i) {
        set_rt(R_FATE, i, INSIDE);
        return;
    }
    retired(i, lived, alive);
}

// The laid axes this cycle began with, for the next's closing.
@compute @workgroup_size(1)
fn end_main() {
    // A halted cycle's words are none (`vm.wgsl`): it leaves the last
    // cycle's, as no cycle lays fewer axes than the one before.
    if cfg.n != 0u {
        ret[cfg.head + HD_PREV_LAID] = cfg.n;
    }
}

var<workgroup> w_reused: array<atomic<u32>, 3>;

// A measurement's count (`Tables::counted`): the laid axes closed, those
// closed this cycle, and the blocks the living's sweep enters (a block's
// number is under the laid axes).
@compute @workgroup_size(64)
fn reused_main(@builtin(global_invocation_id) id: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    if lid < 3u {
        atomicStore(&w_reused[lid], 0u);
    }
    workgroupBarrier();
    let i = index(id);
    if i < cfg.n {
        var held = array<bool, 3>(rt(R_OPEN, i) != cfg.cycle, rc(C_CLOSED, i) != 0u, i < cfg.stride && atomicLoad(&verdicts[block_stamp(i)]) == cfg.cycle);
        for (var k = 0u; k < 3u; k++) {
            if held[k] {
                atomicAdd(&w_reused[k], 1u);
            }
        }
    }
    workgroupBarrier();
    if lid < 3u {
        atomicAdd(&verdicts[L_REUSE + lid], atomicLoad(&w_reused[lid]));
    }
}
