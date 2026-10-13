// The vigour split at ordered attachments (`allocation::split`): each
// axis's incoming vigour shared, attachment by attachment from the base,
// between its continuing axis and the laterals there by λ and their
// subtrees' light, the rest at its tip between its own bud and what
// carries it on.
//
// A: `factor` runs every axis's split with a vigour of one, giving each
//    child its share of its bearer's incoming and each bud its share;
//    `walk` multiplies the shares root first down each axis's ancestors.
// B: `block`, top-down by local depth inside a block, the CPU's split
//    to the operation.
//
// P.a[0]: n, mode (0 incoming, 1 factors), level start in `blist`.

@group(0) @binding(1) var<storage, read_write> parent: array<u32>;
@group(0) @binding(2) var<storage, read_write> ch_start: array<u32>;
@group(0) @binding(3) var<storage, read_write> cn_list: array<u32>;
@group(0) @binding(4) var<storage, read_write> node: array<u32>;
@group(0) @binding(5) var<storage, read_write> pa: array<u32>;
@group(0) @binding(6) var<storage, read_write> tab: array<f32>;
@group(0) @binding(7) var<storage, read_write> own: array<f32>;
@group(0) @binding(8) var<storage, read_write> sub: array<f32>;
@group(0) @binding(9) var<storage, read_write> incoming: array<f32>;
@group(0) @binding(10) var<storage, read_write> bud: array<f32>;
@group(0) @binding(11) var<storage, read_write> rest: array<f32>;
@group(0) @binding(12) var<storage, read_write> blist: array<u32>;
@group(0) @binding(13) var<storage, read_write> bmeta: array<u32>;
@group(0) @binding(14) var<storage, read_write> offs: array<u32>;
@group(0) @binding(15) var<storage, read_write> nodes: array<u32>;

// The light standing above slot `c` of `i`'s children: its own bud's
// plus every child with light from there to the tip.
fn rest_at(c: u32, ce: u32, own_i: f32) -> f32 {
    if c >= ce {
        return own_i;
    }
    return rest[c];
}

// Axis `i`'s split of `v_in`, writing each child's incoming (or, in
// factor mode, its share) and its own bud's.
fn split_axis(i: u32, v_in: f32) {
    var v = v_in;
    let own_i = own[i];
    if v <= 0.0 || sub[i] <= 0.0 {
        return;
    }
    let l = tab[8u * pa[i] + 3u];
    let cs = ch_start[i];
    let ce = ch_start[i + 1u];
    var r = own_i;
    for (var c = ce; c > cs; c--) {
        let s = sub[cn_list[c - 1u]];
        if s > 0.0 {
            r = r + s;
        }
        rest[c - 1u] = r;
    }
    var c = cs;
    loop {
        while c < ce && !(sub[cn_list[c]] > 0.0) {
            c++;
        }
        if c >= ce {
            break;
        }
        let at = node[cn_list[c]];
        if at == NONE {
            break;
        }
        var e = c;
        var lateral = 0.0;
        while e < ce && node[cn_list[e]] == at {
            let s = sub[cn_list[e]];
            if s > 0.0 {
                lateral = lateral + s;
            }
            e++;
        }
        let main = rest_at(e, ce, own_i);
        let split = l * main + (1.0 - l) * lateral;
        if split > 0.0 {
            for (var k = c; k < e; k++) {
                let s = sub[cn_list[k]];
                if s > 0.0 {
                    incoming[cn_list[k]] = v * (1.0 - l) * s / split;
                }
            }
            v = v * l * main / split;
        }
        c = e;
    }
    let above = rest_at(c, ce, own_i);
    if above > 0.0 {
        bud[i] = v * own_i / above;
        for (var k = c; k < ce; k++) {
            let s = sub[cn_list[k]];
            if s > 0.0 {
                incoming[cn_list[k]] = v * s / above;
            }
        }
    }
}

@compute @workgroup_size(256)
fn factor(@builtin(global_invocation_id) g: vec3<u32>) {
    let i = g.x;
    if i >= P.a[0].x {
        return;
    }
    split_axis(i, 1.0);
}

@compute @workgroup_size(256)
fn walk(@builtin(global_invocation_id) g: vec3<u32>) {
    let i = g.x;
    if i >= P.a[0].x {
        return;
    }
    // `incoming` holds each axis's share of its bearer's and `bud` each
    // bud's of its axis's; the root's incoming is its subtree's light.
    var path: array<u32, 96>;
    var len = 0u;
    var j = i;
    loop {
        let p = parent[j];
        if p == NONE {
            break;
        }
        path[len] = j;
        len++;
        j = p;
    }
    var v = sub[j];
    for (var k = len; k > 0u; k--) {
        v = v * incoming[path[k - 1u]];
    }
    // Written beside the shares, which other axes still read: `rest`
    // (sized for every axis) takes the incoming.
    rest[i] = v;
    bud[i] = v * bud[i];
}

var<workgroup> w_start: u32;
var<workgroup> w_top: u32;

@compute @workgroup_size(256)
fn block(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    if lid == 0u {
        let b = blist[P.a[0].z + wid.x];
        w_start = bmeta[2u * b];
        w_top = bmeta[2u * b + 1u];
    }
    let s0 = workgroupUniformLoad(&w_start);
    let top = workgroupUniformLoad(&w_top);
    for (var h = 0u; h <= top; h++) {
        let s = offs[s0 + h];
        let e = offs[s0 + h + 1u];
        for (var k = s + lid; k < e; k += WG) {
            let i = nodes[k];
            if parent[i] == NONE {
                incoming[i] = sub[i];
            }
            split_axis(i, incoming[i]);
        }
        storageBarrier();
        workgroupBarrier();
    }
}
