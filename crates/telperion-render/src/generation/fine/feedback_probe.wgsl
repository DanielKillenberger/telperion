// The feedback contract's conformance probe: each case's eight u64
// arguments in, `SLOTS` u64 results out (values and error statuses), by
// the functions `cfg.y` selects (`fine/feedback_tests.rs`).

@group(0) @binding(0) var<uniform> cfg: vec4<u32>;
@group(0) @binding(1) var<storage, read> cases: array<vec2<u32>>;
@group(0) @binding(2) var<storage, read> terms: array<vec2<u32>>;
@group(0) @binding(3) var<storage, read_write> results: array<vec2<u32>>;

const SLOTS: u32 = 24u;

var<private> at: u32;

fn put(k: u32, v: vec2<u32>) {
    results[at * SLOTS + k] = v;
}

fn put4(k: u32, v: vec4<u32>) {
    put(k, v.xy);
    put(k + 1u, v.zw);
}

fn put_r4(k: u32, r: R4) {
    if r.status == 0u {
        put4(k, r.v);
    }
    put(k + 2u, w(r.status, 0u));
}

fn put32(k: u32, v: u32) {
    put(k, w(v, 0u));
}

fn term(k: u32) -> vec4<u32> {
    return vec4<u32>(terms[2u * k], terms[2u * k + 1u]);
}

// The sum of `len` terms from `start` in chunks of `chunk`, the chunks
// merged last first: any grouping gives the CPU's forward sum.
fn summed(start: u32, len: u32, chunk: u32) -> R4 {
    var all = Total(vec4<u32>(0u), vec4<u32>(0u));
    let chunks = (len + chunk - 1u) / chunk;
    for (var c = chunks; c > 0u; c--) {
        var part = Total(vec4<u32>(0u), vec4<u32>(0u));
        for (var k = (c - 1u) * chunk; k < min(c * chunk, len); k++) {
            part = total_add(part, term(start + k));
        }
        all = total_merge(all, part);
    }
    return total_get(all);
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x + id.y * cfg.z;
    if i >= cfg.x {
        return;
    }
    at = i;
    var a: array<vec2<u32>, 8>;
    for (var k = 0u; k < 8u; k++) {
        a[k] = cases[i * 8u + k];
    }
    let n = a[4].x;
    switch cfg.y {
        case 0u: {
            put_r4(0u, mul_shr(vec4<u32>(a[0], a[1]), vec4<u32>(a[2], a[3]), n));
        }
        case 1u: {
            let r = prod_shr(a[0], a[1], a[2], a[3], n);
            if r.status == 0u {
                put(0u, r.v.xy);
            }
            put32(1u, r.status);
        }
        case 2u: {
            put_r4(0u, ratio(vec4<u32>(a[0], a[1]), vec4<u32>(a[2], a[3]), n));
        }
        case 3u: {
            let x = vec4<u32>(a[0], a[1]);
            put4(0u, shift4(x, a[2].x, false, true));
            put4(2u, zero_shift(x, a[2].x));
        }
        case 4u: {
            let r = radians(a[0]);
            if r.status == 0u {
                put(0u, r.v.xy);
            }
            put32(1u, r.status);
            put(2u, turns_of(a[1]));
            put(3u, between(a[2].x, a[3].x));
            put32(4u, toward(a[2].x, a[3].x, a[4]));
            put(5u, sar64(a[0], 1u));
        }
        case 5u: {
            put4(0u, sincos(a[0].x));
            put32(2u, atan2(a[1], a[2]));
        }
        case 6u: {
            put32(0u, asin_t(a[0]));
            put32(1u, acos_t(a[1]));
            put(2u, isqrt(vec4<u32>(a[2], a[3])));
            put(3u, length3(a[4], a[5], a[6]));
        }
        case 7u: {
            let u = unit3(array<vec2<u32>, 3>(a[0], a[1], a[2]));
            for (var k = 0u; k < 3u; k++) {
                put(k, u.c[k]);
            }
            put32(3u, select(0u, 1u, u.some));
        }
        case 8u: {
            let u = unit_w(array<vec4<u32>, 3>(vec4<u32>(a[0], a[1]), vec4<u32>(a[2], a[3]), vec4<u32>(a[4], a[5])));
            for (var k = 0u; k < 3u; k++) {
                put(k, u.c[k]);
            }
            put(3u, u.root);
        }
        case 9u: {
            put(0u, log2_w(vec4<u32>(a[0], a[1])));
            put_r4(1u, exp2_w(a[2]));
        }
        case 10u: {
            put_r4(0u, summed(a[0].x, a[1].x, a[2].x));
        }
        case 11u: {
            let wt = weights(vec3<u32>(a[0].x, a[1].x, a[2].x));
            let d = deposits(a[3], wt);
            for (var c = 0u; c < 8u; c++) {
                put(c, wt[c]);
                put4(8u + 2u * c, d[c]);
            }
        }
        case 12u: {
            let b = balance(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]);
            if b.status == 0u {
                put(0u, b.memory);
                put(1u, b.threshold);
                put32(2u, select(0u, 1u, b.shed));
                put(3u, b.kept);
            }
            put32(4u, b.status);
        }
        case 14u: {
            let t = Turn(V3(a[3], a[4], a[5]), a[6], a[7], true);
            let v = turned_by(V3(a[0], a[1], a[2]), t);
            for (var k = 0u; k < 3u; k++) {
                put(k, v[k]);
            }
        }
        case 15u: {
            let from_v = V3(a[0], a[1], a[2]);
            let to_v = V3(a[3], a[4], a[5]);
            let t = carry(from_v, to_v);
            var p = t.pivot;
            var c = cross3(from_v, to_v);
            for (var k = 0u; k < 3u; k++) {
                put(k, p[k]);
                put(7u + k, c[k]);
            }
            put(3u, t.cos);
            put(4u, t.sin);
            put32(5u, select(0u, 1u, t.some));
            put(6u, dot3(from_v, to_v));
        }
        case 13u: {
            put(0u, ripe(a[0], a[1]));
            put(1u, shade(a[2], a[3], a[4]));
            put32(2u, settle(a[5].x, a[6]));
            put(3u, tangent_ratio(a[5].x, a[5].y));
        }
        default: {}
    }
}
