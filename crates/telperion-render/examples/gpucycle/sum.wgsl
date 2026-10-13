// Bottom-up subtree sums of one or two values and, optionally, whether
// anything in the subtree lives: the balance totals (light, upkeep,
// living) and the surviving allocation totals (lit).
//
// A: every axis walks its ancestors, adding its values in 2^-32 units
//    to each one's 64-bit integer sum (two atomic words, the carry
//    added to the high one): one dispatch, any order, one result.
// B: the tree's bounded blocks, a workgroup each, by local height with a
//    barrier between heights; each axis its own value plus its children's
//    totals from the last child to the first, the CPU's order to the bit;
//    block levels are dispatch boundaries.
//
// P.a[0]: n, two (a second value), level start in `blist`, living.

@group(0) @binding(1) var<storage, read_write> parent: array<u32>;
@group(0) @binding(2) var<storage, read_write> ch_start: array<u32>;
@group(0) @binding(3) var<storage, read_write> ch_list: array<u32>;
@group(0) @binding(4) var<storage, read_write> val_a: array<f32>;
@group(0) @binding(5) var<storage, read_write> val_b: array<f32>;
@group(0) @binding(6) var<storage, read_write> live_in: array<u32>;
@group(0) @binding(7) var<storage, read_write> out_a: array<f32>;
@group(0) @binding(8) var<storage, read_write> out_b: array<f32>;
@group(0) @binding(9) var<storage, read_write> living: array<u32>;
@group(0) @binding(10) var<storage, read_write> acc: array<atomic<u32>>;
@group(0) @binding(11) var<storage, read_write> liv_acc: array<atomic<u32>>;
@group(0) @binding(12) var<storage, read_write> blist: array<u32>;
@group(0) @binding(13) var<storage, read_write> bmeta: array<u32>;
@group(0) @binding(14) var<storage, read_write> offs: array<u32>;
@group(0) @binding(15) var<storage, read_write> nodes: array<u32>;

fn add_at(at: u32, v: vec2<u32>) {
    if v.x != 0u {
        let old = atomicAdd(&acc[at], v.x);
        if old + v.x < old {
            atomicAdd(&acc[at + 1u], v.y + 1u);
            return;
        }
    }
    if v.y != 0u {
        atomicAdd(&acc[at + 1u], v.y);
    }
}

@compute @workgroup_size(256)
fn walk(@builtin(global_invocation_id) g: vec3<u32>) {
    let i = g.x + g.y * 65535u * WG;
    let n = P.a[0].x;
    if i >= n {
        return;
    }
    let two = P.a[0].y != 0u;
    let va = fixed(val_a[i]);
    var vb = vec2<u32>(0u);
    if two {
        vb = fixed(val_b[i]);
    }
    var lv = P.a[0].w != 0u && live_in[i] != 0u;
    if is_zero(va) && is_zero(vb) && !lv {
        return;
    }
    var j = i;
    loop {
        add_at(4u * j, va);
        add_at(4u * j + 2u, vb);
        if lv && atomicOr(&liv_acc[j], 1u) != 0u {
            lv = false;
        }
        let p = parent[j];
        if p == NONE {
            break;
        }
        j = p;
    }
}

@compute @workgroup_size(256)
fn unpack(@builtin(global_invocation_id) g: vec3<u32>) {
    let i = g.x + g.y * 65535u * WG;
    if i >= P.a[0].x {
        return;
    }
    out_a[i] = unfixed(atomicLoad(&acc[4u * i]), atomicLoad(&acc[4u * i + 1u]));
    if P.a[0].y != 0u {
        out_b[i] = unfixed(atomicLoad(&acc[4u * i + 2u]), atomicLoad(&acc[4u * i + 3u]));
    }
    if P.a[0].w != 0u {
        living[i] = atomicLoad(&liv_acc[i]);
    }
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
    let two = P.a[0].y != 0u;
    let with_living = P.a[0].w != 0u;
    for (var h = 0u; h <= top; h++) {
        let s = offs[s0 + h];
        let e = offs[s0 + h + 1u];
        for (var k = s + lid; k < e; k += WG) {
            let i = nodes[k];
            var a = val_a[i];
            var b = 0.0;
            if two {
                b = val_b[i];
            }
            var lv = with_living && live_in[i] != 0u;
            let cs = ch_start[i];
            for (var c = ch_start[i + 1u]; c > cs; c--) {
                let ch = ch_list[c - 1u];
                a = a + out_a[ch];
                if two {
                    b = b + out_b[ch];
                }
                if with_living && living[ch] != 0u {
                    lv = true;
                }
            }
            out_a[i] = a;
            if two {
                out_b[i] = b;
            }
            if with_living {
                living[i] = u32(lv);
            }
        }
        storageBarrier();
        workgroupBarrier();
    }
}
