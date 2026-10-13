// fn-256 task 2 (SETTLE-PLAN.md G0): the grown tree's shape for the
// settle. Each axis's phytomers' and units' counts for their scans, its
// depth by jumping up its ancestors (two columns by turns, so no
// invocation reads what another writes), its children counted onto its
// parent and its depth's count; then each axis's phytomers and units at
// their scanned starts, itself among its parent's children and in its
// depth's list, and each parent's children sorted by index. An atomic
// slot only lists; the sort and the depths' passes make every result
// the same whatever order the slots were taken in.

@compute @workgroup_size(64)
fn seed_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = item(id);
    if i > cfg.n {
        return;
    }
    if i == cfg.n {
        // The scans' last rows, so each start column ends in its total.
        set_wa(W_PSTART, i, 0u);
        set_wa(W_USTART, i, 0u);
        return;
    }
    // A retired root's one place: its frozen terms in the leaf mean's order.
    set_wa(W_PSTART, i, phyts_of(i) + u32(retired_root(i)));
    set_wa(W_USTART, i, ax(i, A_UNITS));
    let p = ax(i, A_PARENT);
    set_wa(W_DEPTH, i, select(1u, 0u, p == NONE));
    set_wa(W_ANC, i, p);
    set_waf(W_QB1, i, q64(ax64(i, A_B1)));
    set_wa(W_FINAL, i, NONE);
}

// One jump: each axis's depth so far plus its ancestor's, and the
// ancestor's ancestor.
@compute @workgroup_size(64)
fn depth_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = item(id);
    if i >= cfg.n {
        return;
    }
    let was = cfg.flip;
    let now = 1u - was;
    let a = wa(W_ANC + was, i);
    var d = wa(W_DEPTH + was, i);
    var up = NONE;
    if a != NONE {
        d += wa(W_DEPTH + was, a);
        up = wa(W_ANC + was, a);
    }
    set_wa(W_DEPTH + now, i, d);
    set_wa(W_ANC + now, i, up);
}

// Each axis counted onto its parent's children and its depth.
@compute @workgroup_size(64)
fn count_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = item(id);
    if i >= cfg.n {
        return;
    }
    if wa(W_ANC + cfg.flip, i) != NONE {
        fault(FAULT_DEEP);
    }
    let d = wa(W_DEPTH + cfg.flip, i);
    if d >= DMAX {
        fault(FAULT_DEEP);
        return;
    }
    atomicMax(&atom[T_DEEPEST], d);
    atomicAdd(&atom[T_DCOUNT + d], 1u);
    let p = ax(i, A_PARENT);
    if p != NONE {
        atomicAdd(&atom[T_KCOUNT + p], 1u);
    }
}

// Each axis's phytomers and units at their starts, in its units' order;
// itself among its parent's children and in its depth's list.
@compute @workgroup_size(64)
fn fill_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = item(id);
    if i >= cfg.n {
        return;
    }
    let p = ax(i, A_PARENT);
    if p != NONE {
        let at = atomicLoad(&atom[T_KCOUNT + p]) + atomicAdd(&atom[cfg.kfill + p], 1u);
        set_wa(W_KIDS, at, i);
    }
    let d = wa(W_DEPTH + cfg.flip, i);
    if d < DMAX {
        let at = atomicLoad(&atom[T_DCOUNT + d]) + atomicAdd(&atom[T_DFILL + d], 1u);
        set_wa(W_DLIST, at, i);
    }
    let ps = wa(W_PSTART, i);
    let us = wa(W_USTART, i);
    let n_p = phyts_of(i);
    if retired_root(i) {
        set_wp(WP_ROW, ps, NONE);
        set_wp(WP_UNIT, ps, i);
    }
    var r = ax(i, A_HEAD);
    var k = 0u;
    var j = 0u;
    // cap: the axis's units, as counted.
    while r != NONE && k < ax(i, A_UNITS) {
        set_wu(WU_ROW, us + k, r);
        let first = unit(r, U_FIRST);
        let n = unit(r, U_COUNT);
        for (var t = 0u; t < n && j < n_p; t++) {
            set_wp(WP_ROW, ps + j, first + t);
            j += 1u;
        }
        k += 1u;
        r = unit(r, U_NEXT);
    }
    if r != NONE || k != ax(i, A_UNITS) || j != n_p {
        fault(FAULT_LIST);
    }
}

// Each parent's children in index order.
@compute @workgroup_size(64)
fn sort_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = item(id);
    if i >= cfg.n {
        return;
    }
    let run = kids_of(i);
    // cap: the parent's children, each moved past at most the others.
    for (var k = run.x + 1u; k < run.y; k++) {
        let x = kid(k);
        var j = k;
        for (; j > run.x; j--) {
            let y = kid(j - 1u);
            if y < x {
                break;
            }
            set_wa(W_KIDS, j, y);
        }
        set_wa(W_KIDS, j, x);
    }
}
