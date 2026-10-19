// Shedding (D16, `contract::shed::walk`): each living lateral's balance
// against its PA's tolerance, its hazard's threshold against its draw,
// and what that leaves its subtree: shed with it, or kept by the product
// of the presences above it, root first.
//
// A: `local` decides each axis alone; `walk` climbs its ancestors into a
//    path and replays the walk root first.
// B: `block`, top-down by local depth inside a block, block levels from
//    the root's down.
//
// P.a[0]: n, cycle, level start in `blist`.

@group(0) @binding(1) var<storage, read_write> parent: array<u32>;
@group(0) @binding(2) var<storage, read_write> node: array<u32>;
@group(0) @binding(3) var<storage, read_write> pa: array<u32>;
@group(0) @binding(4) var<storage, read_write> lineage: array<vec2<u32>>;
@group(0) @binding(5) var<storage, read_write> tab: array<f32>;
@group(0) @binding(6) var<storage, read_write> q_in: array<f32>;
@group(0) @binding(7) var<storage, read_write> c_in: array<f32>;
@group(0) @binding(8) var<storage, read_write> living: array<u32>;
@group(0) @binding(9) var<storage, read_write> memory_in: array<f32>;
@group(0) @binding(10) var<storage, read_write> loc: array<u32>;
@group(0) @binding(11) var<storage, read_write> shed: array<u32>;
@group(0) @binding(12) var<storage, read_write> kept: array<f32>;
@group(0) @binding(13) var<storage, read_write> mem_out: array<f32>;
@group(0) @binding(14) var<storage, read_write> blist: array<u32>;
@group(0) @binding(15) var<storage, read_write> bmeta: array<u32>;
@group(0) @binding(16) var<storage, read_write> offs: array<u32>;
@group(0) @binding(17) var<storage, read_write> nodes: array<u32>;
@group(0) @binding(18) var<storage, read_write> draws: array<vec2<u32>>;

const NAN_BITS: u32 = 0x7fc00000u;

struct Decision {
    decides: bool,
    shed: bool,
    // The presence it keeps, where its threshold is set and it stands.
    factored: bool,
    factor: f32,
    memory: f32,
}

fn over(t: vec2<u32>, d: vec2<u32>) -> f32 {
    if is_zero(t) {
        return INF;
    }
    return logit53(d) - logit53(t);
}

fn now(q: f32, c: f32) -> f32 {
    let total = q + c;
    if total <= 0.0 {
        return 1.0;
    }
    return (q - c) / total;
}

fn decide(i: u32) -> Decision {
    var out = Decision(false, false, false, 1.0, 0.0);
    out.decides = parent[i] != NONE && node[i] != NONE && living[i] != 0u;
    if !out.decides {
        return out;
    }
    let k = 8u * pa[i];
    out.memory = (memory_in[i] + now(q_in[i], c_in[i])) * 0.5;
    let hazard = tab[k + 1u] * max(tab[k + 2u] - out.memory, 0.0);
    let d = draw(child(child(lineage[i], KEY_SHED), vec2<u32>(P.a[0].y, 0u)));
    draws[i] = d;
    if hazard <= 0.0 {
        return out;
    }
    let t = threshold(fails(hazard * 1.4426950408889634));
    if lt64(d, t) {
        out.shed = true;
        return out;
    }
    out.factored = !is_zero(t);
    out.factor = presence(over(t, d), tab[k + 5u]);
    return out;
}

@compute @workgroup_size(256)
fn local(@builtin(global_invocation_id) g: vec3<u32>) {
    let i = g.x;
    if i >= P.a[0].x {
        return;
    }
    let d = decide(i);
    loc[4u * i] = u32(d.decides) | (u32(d.shed) << 1u) | (u32(d.factored) << 2u);
    loc[4u * i + 1u] = bitcast<u32>(d.factor);
    loc[4u * i + 2u] = bitcast<u32>(d.memory);
}

@compute @workgroup_size(256)
fn walk(@builtin(global_invocation_id) g: vec3<u32>) {
    let i = g.x;
    if i >= P.a[0].x {
        return;
    }
    var path: array<u32, 96>;
    var len = 0u;
    var j = i;
    loop {
        path[len] = j;
        len++;
        let p = parent[j];
        if p == NONE {
            break;
        }
        j = p;
    }
    var dead = false;
    var keep = 1.0;
    var memory = bitcast<f32>(NAN_BITS);
    for (var k = len; k > 0u; k--) {
        let a = path[k - 1u];
        let flags = loc[4u * a];
        if dead || (flags & 1u) == 0u {
            continue;
        }
        if a == i {
            memory = bitcast<f32>(loc[4u * a + 2u]);
        }
        if (flags & 2u) != 0u {
            dead = true;
        } else if (flags & 4u) != 0u {
            keep = keep * bitcast<f32>(loc[4u * a + 1u]);
        }
    }
    shed[i] = u32(dead);
    kept[i] = keep;
    mem_out[i] = memory;
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
            let p = parent[i];
            var dead = false;
            var keep = 1.0;
            if p != NONE {
                dead = shed[p] != 0u;
                keep = kept[p];
            }
            var memory = bitcast<f32>(NAN_BITS);
            if !dead {
                let d = decide(i);
                if d.decides {
                    memory = d.memory;
                    if d.shed {
                        dead = true;
                    } else if d.factored {
                        keep = keep * d.factor;
                    }
                }
            }
            shed[i] = u32(dead);
            kept[i] = keep;
            mem_out[i] = memory;
        }
        storageBarrier();
        workgroupBarrier();
    }
}
