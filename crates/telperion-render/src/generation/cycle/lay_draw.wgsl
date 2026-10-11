// fn-256 task 2: an axis's units laid (`grow::draw::draw_bent`): each
// unit's phytomers sized as the final lay will size them, then walked,
// its pencil carried on. Part of the layout's shader.

// How far axis `a`, `years` old, is pulled towards the vertical at its
// base (`geometry::bend`).
fn bend_of(a: u32, years: f32) -> f32 {
    let pa = ax(a, A_PA);
    let x = fm(pa, F_ERECTION) * years;
    let grown = fails(apart(x * QLOG2E));
    let kind = kind_of(a);
    var straightening = 0.0;
    var erected = grown;
    if kind == CONTINUED || kind == RELAYED {
        let b = select(0.0, axf(a, A_BLEND), kind == RELAYED);
        straightening = m(fm(pa, F_STRAIGHTENING), b);
        erected = m(b, grown);
    } else if kind == LATERAL {
        straightening = fm(pa, F_STRAIGHTENING);
    }
    return 1.0 - m(1.0 - straightening, 1.0 - erected);
}

// The axis's age in ticks at the cycle, less the share it slept.
fn age_ticks(a: u32) -> i32 {
    let cycles = max(i32(cfg.cycle) - i32(ax(a, A_BIRTH)), 0);
    return cycles * i32(TICKS) - i32(ax(a, A_SLEEP));
}

// Lays axis `i`'s units not yet laid, its pencil carried on.
fn draw_axis(i: u32) {
    let pa = ax(i, A_PA);
    let birth = ax(i, A_BIRTH);
    let bent = bend_of(i, years_of(age_ticks(i)));
    let base_scale = pnf(i, P_BASE_SCALE);
    let trunk = pn(i, P_TRUNK) != 0u;
    let internode = fm(pa, F_INTERNODE);
    let leaf = fm(pa, F_LEAF_AREA);
    var wk = walk_of(i);
    var running = pnf(i, P_GOING);
    var rank = pnf(i, P_RANKED);
    var wood = pnf(i, P_WOOD);
    var unit = pn(i, P_UNITS);
    var laid = pn(i, P_LAID);
    let units_n = ax(i, A_UNITS_N);
    let n = nphyt(i);
    var u = NONE;
    if unit < units_n {
        u = unit_of(i, unit);
    }
    // cap: the axis's pending units.
    while unit < units_n {
        if u == NONE {
            status |= FAULT_UNITS;
            return;
        }
        let grown = unit_size(u);
        let present = running * unit_survive(u);
        let size = unit_shade(u);
        var end = laid;
        // cap: the axis's phytomers.
        while end < n && ph(prow(i, end), PH_CYCLE) - birth - 1u == unit {
            end += 1u;
        }
        var sum = neg0();
        for (var j = laid; j < end; j++) {
            let r = prow(i, j);
            let node = phf(r, PH_NODE);
            let grew = apart(present * node) * grown;
            let scale = grew * base_scale;
            set_ph(r, PH_SCALE, scale);
            set_ph(r, PH_GROWN, grew);
            set_ph(r, PH_SIZE, size);
            set_ph(r, PH_RANK, rank);
            rank = rank + m(node, grown);
            sum = sum + m(scale, size);
        }
        let total = wk.run + sum;
        for (var j = laid; j < end; j++) {
            let r = prow(i, j);
            let sized = m(phf(r, PH_SCALE), size);
            step(&wk, i, r, sized, bent, total, trunk);
            wood = wood + m(internode, sized);
            set_ph(r, PH_AREA, sized * leaf);
        }
        laid = end;
        running = present * unit_persist(u);
        unit += 1u;
        u = unit_word(u, UN_NEXT);
    }
    keep_walk(i, wk);
    set_pnf(i, P_GOING, running);
    set_pnf(i, P_RANKED, rank);
    set_pnf(i, P_WOOD, wood);
    set_pn(i, P_UNITS, unit);
    set_pn(i, P_LAID, laid);
    set_ax(i, A_RANK, bitcast<u32>(rank));
}
