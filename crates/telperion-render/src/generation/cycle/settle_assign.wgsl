// fn-256 task 2 (SETTLE-PLAN.md G1 and G2): the living apexes marked,
// the woken buds sized by what their bearers kept of them, then every
// axis's presences assigned and its vigour set (`Grower::mark`,
// `presence::assign`). Every presence an f32, each product in the CPU's
// order and kept apart from the sum it feeds (`contract::presence`).

// How much a living apex lives on (`Grower::living`): whole while a
// cycle of its age is left, else the share left and the rest as far as
// its continuation or relay carries it on.
@compute @workgroup_size(64)
fn living_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = item(id);
    if k >= cfg.live {
        return;
    }
    let a = wl(WL_LIVE, k);
    let spent = wl(WL_SPENT, k);
    let pa = ax(a, A_PA);
    let life = kpa(pa, K_LIFE);
    var left = 0.0;
    if life > spent {
        left = tick_q(min(life - spent, TICKS));
    }
    let go = kpaf(pa, K_GO);
    let on = go + m(1.0 - go, kpaf(pa, K_RELAY_ENDED));
    set_waf(W_LIVING, a, left + m(1.0 - left, on));
    set_wa(W_ALIVE, a, 1u);
}

// The presences of the draws that made axis `a`'s successors, summed in
// order from Rust's -0.0.
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

// Axis `a`'s growth unit `k`'s record, none past its units.
fn unit_at(a: u32, k: u32) -> u32 {
    if k >= ax(a, A_UNITS) {
        return NONE;
    }
    return wu(WU_ROW, wa(W_USTART, a) + k);
}

// How much of its size a woken bud keeps (`Woken::kept`).
fn woken_kept(bearer: u32, next: u32, cycle: u32, share: f32) -> f32 {
    let flags = ax(bearer, A_FLAGS);
    let outlived = (flags & F_OUTLIVED) != 0u;
    let used = tick_q(ax(bearer, A_OUTLIVED));
    let end = ax(bearer, A_END);
    let alone = ax(bearer, A_SUCC0) == NONE;
    if outlived && end == cycle + 1u && alone {
        return max(used + share - 1.0, 0.0);
    }
    var p = 1.0;
    if cycle >= cfg.age {
        if wa(W_ALIVE, bearer) != 0u {
            p = qf(waf(W_LIVING, bearer));
        }
    } else {
        let u = unit_at(bearer, next);
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

@compute @workgroup_size(64)
fn woken_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = item(id);
    if k >= cfg.woken {
        return;
    }
    let a = wl(WL_WOKEN, k);
    let share = tick_q(wl(WL_WOKEN + 4u, k));
    let kept = woken_kept(wl(WL_WOKEN + 1u, k), wl(WL_WOKEN + 2u, k), wl(WL_WOKEN + 3u, k), share);
    set_waf(W_QB1, a, m(q64(ax64(a, A_B1)), qf(kept)));
}

// Axis `a`'s phytomer `j`'s row in the arena.
fn prow(a: u32, j: u32) -> u32 {
    return wp(WP_ROW, wa(W_PSTART, a) + j);
}

// The node of `p`'s last unit a relay of it stands at (D18,
// `decide::relay_node_f`): the node whose span holds `share` of the
// unit's node presences, by count where it has none.
fn relay_node(p: u32) -> u32 {
    let n = ax(p, A_PHYTS);
    if n == 0u {
        return 0u;
    }
    let last = phyt(prow(p, n - 1u), P_CYCLE);
    var first = n - 1u;
    // cap: the last unit's phytomers.
    while first > 0u && phyt(prow(p, first - 1u), P_CYCLE) == last {
        first -= 1u;
    }
    let share = kpaf(ax(p, A_PA), K_RELAY_AT);
    var total = bitcast<f32>(0x80000000u);
    for (var j = first; j < n; j++) {
        total = total + bitcast<f32>(phyt(prow(p, j), P_NODE));
    }
    if total > 0.0 {
        let aim = apart(share * total);
        var before = 0.0;
        for (var j = first; j < n; j++) {
            let q = bitcast<f32>(phyt(prow(p, j), P_NODE));
            if q > 0.0 && before + q >= aim {
                return j;
            }
            before = before + q;
        }
        return n - 1u;
    }
    let k = first + u32(ceil(share * f32(n - first)));
    return clamp(k, first + 1u, n) - 1u;
}

// One axis's presences (`assign_one`): each unit's, summed in order;
// each phytomer's scale, matched to its unit by its cycle; its end's
// presence, its living apex's, and its own living time.
@compute @workgroup_size(64)
fn assign_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = item(id);
    if i >= cfg.n {
        return;
    }
    let kind = kind_of(i);
    var node = 0u;
    if kind == RELAYED {
        node = relay_node(ax(i, A_PARENT));
    } else if kind == LATERAL {
        node = ax(i, A_NODE);
    }
    set_wa(W_NODE, i, node);
    let birth = ax(i, A_BIRTH);
    let us = wa(W_USTART, i);
    let ps = wa(W_PSTART, i);
    let n = ax(i, A_PHYTS);
    var running = 1.0;
    var sum = 0.0;
    var j = 0u;
    for (var k = 0u; k < ax(i, A_UNITS); k++) {
        let r = wu(WU_ROW, us + k);
        running = m(running, qf(bitcast<f32>(unit(r, U_SURVIVE))));
        let presence = m(running, tick_q(unit(r, U_SHARE)));
        sum = sum + presence;
        set_wu(WU_PRES, us + k, bitcast<u32>(presence));
        set_wu(WU_PSUM, us + k, bitcast<u32>(sum));
        let laid = tick_q(unit(r, U_SIZE));
        // cap: the axis's phytomers.
        while j < n && i32(phyt(wp(WP_ROW, ps + j), P_CYCLE)) - i32(birth) - 1 == i32(k) {
            let node_p = bitcast<f32>(phyt(wp(WP_ROW, ps + j), P_NODE));
            set_wp(WP_SCALE, ps + j, bitcast<u32>(m(m(running, qf(node_p)), laid)));
            set_wp(WP_UNIT, ps + j, r);
            j += 1u;
        }
        running = m(running, qf(bitcast<f32>(unit(r, U_PERSIST))));
    }
    if j < n {
        fault(FAULT_UNASSIGNED);
    }
    set_waf(W_END, i, running);
    var alive = 0.0;
    if wa(W_ALIVE, i) != 0u {
        alive = m(running, qf(waf(W_LIVING, i)));
    }
    set_waf(W_ALIVEQ, i, alive);
    set_waf(W_LIVED, i, tick_q(ax(i, A_SLEEP)) + sum);
    set_waf(W_SALIVE, i, alive);
}

// Each axis's vigour from its making draws, and for an axis carried on,
// from its parent's end.
@compute @workgroup_size(64)
fn vigour_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = item(id);
    if i >= cfg.n {
        return;
    }
    let made = m(q64(ax64(i, A_B0)), waf(W_QB1, i));
    let kind = kind_of(i);
    var vigour = made;
    if kind == CONTINUED || kind == RELAYED {
        vigour = m(waf(W_END, ax(i, A_PARENT)), made);
    }
    set_waf(W_VIG, i, vigour);
}
