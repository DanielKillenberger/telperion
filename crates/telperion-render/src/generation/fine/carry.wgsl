// The presence products a chain's end takes beside its asks (fn-209 F1
// task 6; host decision 8): each axis's parent's time to its birth, a woken
// bud's carrying and keeping (`dormant.rs`), and how much a living apex
// lives on (`Grower::living`). Part of `chain.wgsl`'s shader, after
// `assign.wgsl`.

// The first `units` growth units of `a`, their presences summed.
fn prefix_q(a: u32, units: u32) -> f32 {
    if units == rd(a, NUNITS) {
        return rq(a, QGROWN);
    }
    var sum = 0.0;
    for (var i = 0u; i < n_units; i++) {
        let u = unit_base + i * UW;
        if sleepers[u] == a && sleepers[u + 1u] < units {
            sum += unit_q(u, QUNIT);
        }
    }
    return sum;
}

// An axis's parent's own time to its birth, and the whole cycles from
// there (`shed.rs`, `fades`).
fn before_q(i: u32) {
    let p = rd(i, PARENT);
    if p == NONE || (rd(p, KIND) & 0xffu) == STUB {
        return;
    }
    let span = rd(i, BIRTH) - min(rd(p, BIRTH), rd(i, BIRTH));
    let units = min(span, rd(p, NUNITS));
    var before = 0.0;
    if units != 0u {
        before = ticks_q(w(rd(p, SLEEP), 0u)) + prefix_q(p, units);
    }
    wq(i, BEFORE, before);
    wr(i, GAP, span - units);
}

// The scale of phytomer `node` of axis `a`.
fn node_scale_q(a: u32, node: u32) -> f32 {
    var seen = 0u;
    for (var k = 0u; k < n_phyts; k++) {
        let q = phyt_base + k * PW;
        if phyts[q + 3u] != a {
            continue;
        }
        if seen == node {
            return bitcast<f32>(phyts[q + QSCALE]);
        }
        seen += 1u;
    }
    return 0.0;
}

// How much the apex of `axis`, living at the age having spent `spent`
// ticks of its PA, lives on (`Grower::living`).
fn living_q(axis: u32, spent: vec2<u32>) {
    let pa = rd(axis, PA);
    let life = row(pa, 0u);
    var left = 0.0;
    if lt64(spent, life) {
        left = ticks_q(min64(sub64(life, spent), ticks_whole()));
    }
    let go = fr(pa, 24u);
    let on = go + (1.0 - go) * fr(pa, 25u);
    wq(axis, QLIVING, left + (1.0 - left) * on);
    wr(axis, KIND, rd(axis, KIND) | IS_ALIVE);
}

// The presences of the draws that made `a`'s successors, summed.
fn successors_q(a: u32) -> f32 {
    var on = 0.0;
    for (var k = 0u; k < 2u; k++) {
        let n = rd(a, SUCC + k);
        if n != NONE {
            on += rq(n, B0) * rq(n, B1);
        }
    }
    return on;
}

// `p` times `a`'s units' survival and persistence from `lo` up to `hi`,
// each product taken in turn.
fn carried_units(p_in: f32, a: u32, lo: u32, hi: u32) -> f32 {
    var p = p_in;
    for (var i = 0u; i < n_units; i++) {
        let u = unit_base + i * UW;
        let k = sleepers[u + 1u];
        if sleepers[u] == a && k >= lo && k < hi {
            p = p * unit_q(u, QSURV) * unit_q(u, QPERS);
        }
    }
    return p;
}

// What carried a woken bud's bearer from its node's unit to the axis it
// woke on (`dormant::carried`): its pending record at `p`, the path from
// that axis up to the node's, walked down again.
fn carried_q(p: u32) -> f32 {
    let top_axis = sleepers[p + 3u];
    var path: array<u32, 32>;
    var n = 0u;
    var a = sleepers[p + 7u];
    // cap: the path's 32 entries.
    loop {
        path[n] = a;
        n += 1u;
        if a == top_axis || n == 32u {
            break;
        }
        a = rd(a, PARENT);
    }
    let unit = sleepers[p + 17u];
    var c = 1.0;
    let u0 = unit_at(top_axis, unit);
    if u0 != NONE {
        c = unit_q(u0, QPERS);
    }
    for (var k = n; k > 0u; k--) {
        let x = path[k - 1u];
        var first = 0u;
        if k == n {
            first = unit + 1u;
        } else {
            c = c * rq(x, B0) * rq(x, B1);
        }
        var last = rd(x, NUNITS);
        if k == 1u {
            last = sleepers[p + 14u];
        }
        c = carried_units(c, x, first, last);
    }
    return c;
}

// How much of its size a bud that woke keeps (`Woken::kept`), from its
// pending record at `p`.
fn woken_kept_q(p: u32) -> f32 {
    let a = sleepers[p + 7u];
    let cycle = sleepers[p + 6u];
    let share = 1.0 - ticks_q(w(sleepers[p + 8u], 0u));
    let end = rd(a, APEX_END);
    let succ = rd(a, SUCC) != NONE;
    if (rd(a, KIND) & HAS_OUTLIVED) != 0u && end == cycle + 1u && !succ {
        return max(ticks_q(w(rd(a, OUTLIVED), 0u)) + share - 1.0, 0.0);
    }
    var on = 1.0;
    if cycle >= cfg.age {
        if (rd(a, KIND) & IS_ALIVE) != 0u {
            on = rq(a, QLIVING);
        }
    } else {
        let u = unit_at(a, sleepers[p + 14u]);
        if u == NONE {
            on = successors_q(a);
        } else {
            on = unit_q(u, QSURV) * unit_q(u, QPERS);
            if end == cycle + 2u && (rd(a, KIND) & IS_FAILED) == 0u {
                if (rd(a, KIND) & HAS_OUTLIVED) != 0u && !succ {
                    on = on * ticks_q(w(rd(a, OUTLIVED), 0u));
                } else {
                    on = on * successors_q(a);
                }
            }
        }
    }
    return share + (1.0 - share) * on;
}
