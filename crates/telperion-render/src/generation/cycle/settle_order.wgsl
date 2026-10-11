// fn-256 task 2 (SETTLE-PLAN.md G7): the kept tree in its lineage order
// (`canonical::lineage_order`), a depth at a time: breadth first from the
// seed, each parent's kept children in the order their parents stand,
// each parent's sorted by (lineage, identity). The previous depth's
// parents count their kept children (`count_kids_main`), a scan places
// each run, and each parent ranks its own (`place_main`); each run is
// also kept by grown index, last first, for the sweeps that take
// children as the CPU's tree does (`upward::Tree`).

fn kept(i: u32) -> bool {
    return wa(W_KEPT, i) != 0u;
}

// The kept children of the grown axis at final index `f`.
fn kept_kids(g: u32) -> u32 {
    let run = kids_of(g);
    var n = 0u;
    for (var k = run.x; k < run.y; k++) {
        n += u32(kept(kid(k)));
    }
    return n;
}

// The order's key of axis `i`: its lineage, then its identity, as four
// words from the most significant.
fn key_of(i: u32) -> vec4<u32> {
    return vec4<u32>(ax(i, A_LINEAGE + 1u), ax(i, A_LINEAGE), wa(W_ID + 1u, i), wa(W_ID, i));
}

fn key_lt(a: vec4<u32>, b: vec4<u32>) -> bool {
    for (var k = 0u; k < 4u; k++) {
        if a[k] != b[k] {
            return a[k] < b[k];
        }
    }
    return false;
}

// The seed, first in the order.
@compute @workgroup_size(64)
fn root_main(@builtin(global_invocation_id) id: vec3<u32>) {
    if item(id) == 0u {
        set_wa(W_FINAL, 0u, 0u);
        set_wf(WF_GROWN, 0u, 0u);
    }
}

// Each parent of the depth before's kept children, for the scan; and
// the scan's last row.
@compute @workgroup_size(64)
fn count_kids_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = id.x + id.y * cfg.row;
    if k >= cfg.count {
        return;
    }
    set_wf(WF_SCAN, k, kept_kids(wf(WF_GROWN, cfg.base + k)));
    if k + 1u == cfg.count {
        set_wf(WF_SCAN, cfg.count, 0u);
    }
}

// Each parent's kept children at their places: the run from the scan,
// each child by its rank among its kept siblings by key.
@compute @workgroup_size(64)
fn place_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = id.x + id.y * cfg.row;
    if k >= cfg.count {
        return;
    }
    let f = cfg.base + k;
    let g = wf(WF_GROWN, f);
    let first = cfg.next + wf(WF_SCAN, k);
    set_wf(WF_KSTART, f, first);
    let n = kept_kids(g);
    let run = kids_of(g);
    var a = 0u;
    // cap: the parent's children, each against each.
    for (var x = run.x; x < run.y; x++) {
        let c = kid(x);
        if !kept(c) {
            continue;
        }
        let key = key_of(c);
        var rank = 0u;
        for (var y = run.x; y < run.y; y++) {
            let o = kid(y);
            if o == c || !kept(o) {
                continue;
            }
            let other = key_of(o);
            if all(other == key) {
                fault(FAULT_TWINS);
                rank += u32(o < c);
            } else {
                rank += u32(key_lt(other, key));
            }
        }
        let at = first + rank;
        set_wa(W_FINAL, c, at);
        set_wf(WF_GROWN, at, c);
        set_wf(WF_KBI, first + n - 1u - a, at);
        a += 1u;
    }
    set_wf(WF_KC, f, n);
}
