// fn-256 task 2: the allotment's top-down pass at one axis: its growing
// buds' places in subtree order (preorder, each axis's children in the
// order they stand on it), and its incoming vigour split between what
// carries it on and its lit children (`Working::share`).

// The light standing above child slot `c` of `[c, end)`: its own past
// the last.
fn rest_at(c: u32, end: u32, own: f32) -> f32 {
    if c >= end {
        return own;
    }
    return sf(S_REST, c);
}

// `Working::share`: `v` split at each node from the base, the continuing
// axis and the laterals there by λ and their light, then at the tip
// between its own bud and what carries it on.
fn share(i: u32, v_in: f32, kids: vec2<u32>) {
    let own = sf(S_OWN, i);
    let l = row(tree[at(T_PA, i)], P_APICAL);
    var r = own;
    for (var k = kids.y; k > kids.x; k--) {
        let s = sf(S_SUB, kid(k - 1u));
        if s > 0.0 {
            r = r + s;
        }
        put(S_REST, k - 1u, r);
    }
    var v = v_in;
    var c = kids.x;
    for (var group = kids.x; group < kids.y; group++) {
        for (; c < kids.y && !(sf(S_SUB, kid(c)) > 0.0); c++) {}
        if c >= kids.y || tree[at(T_NODE, kid(c))] == NONE {
            break;
        }
        let node = tree[at(T_NODE, kid(c))];
        var e = c;
        var lateral = 0.0;
        for (; e < kids.y && tree[at(T_NODE, kid(e))] == node; e++) {
            let s = sf(S_SUB, kid(e));
            if s > 0.0 {
                lateral = lateral + s;
            }
        }
        let carried = rest_at(e, kids.y, own);
        let weights = hm(l, carried) + hm(1.0 - l, lateral);
        if weights > 0.0 {
            for (var k = c; k < e; k++) {
                let s = sf(S_SUB, kid(k));
                if s > 0.0 {
                    put(S_INCOMING, kid(k), hdiv(hm(hm(v, 1.0 - l), s), weights));
                }
            }
            v = hdiv(hm(hm(v, l), carried), weights);
        }
        c = e;
    }
    let top = rest_at(c, kids.y, own);
    if top > 0.0 {
        put(S_BUD, i, hdiv(hm(v, own), top));
        for (var k = c; k < kids.y; k++) {
            let s = sf(S_SUB, kid(k));
            if s > 0.0 {
                put(S_INCOMING, kid(k), hdiv(hm(v, s), top));
            }
        }
    }
}

// One axis: the base's incoming its subtree's light; its growing bud
// listed at its place, its children's places after it; its vigour split.
fn split(i: u32) {
    if tree[at(T_PARENT, i)] == NONE {
        sums[at(S_INCOMING, i)] = sums[at(S_SUB, i)];
        sums[at(S_RANK, i)] = 0u;
    }
    let place = sums[at(S_RANK, i)];
    let grows = sums[at(S_GROWING, i)];
    if grows != 0u {
        norm[N_ORDER + place] = i;
    }
    let kids = children(i);
    var next = place + grows;
    for (var k = kids.x; k < kids.y; k++) {
        sums[at(S_RANK, kid(k))] = next;
        next += sums[at(S_BUDS, kid(k))];
    }
    let v = sf(S_INCOMING, i);
    if v > 0.0 && sf(S_SUB, i) > 0.0 {
        share(i, v, kids);
    }
}

@compute @workgroup_size(WG)
fn split_main(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
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
                split(lists[ls(R_MEMBERS, k)]);
            }
            storageBarrier();
            workgroupBarrier();
        }
    }
}
