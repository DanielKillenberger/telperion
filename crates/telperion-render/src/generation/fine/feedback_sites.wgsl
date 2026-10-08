// The feedback contract's sites on the GPU (fn-211 task 8, CONTRACT-A.md
// 2.5 to 2.7): `fixed::feedback::lattice`, `shed` and `edge`, word for
// word, over `feedback.wgsl`.

// `lattice::weights`: Q62, corner c upper on axis a where bit a is set.
fn weights(t: vec3<u32>) -> array<vec2<u32>, 8> {
    var out: array<vec2<u32>, 8>;
    var rest = p_one();
    let lo = vec3<u32>(0x80000000u) - t;
    for (var c = 0u; c < 7u; c++) {
        let up = ((vec3<u32>(c) >> vec3<u32>(0u, 1u, 2u)) & vec3<u32>(1u)) == vec3<u32>(1u);
        let f = select(lo, t, up);
        out[c] = shr128(mul128(mul32(f.x, f.y), w(f.z, 0u)), 31u);
        rest = sub64(rest, out[c]);
    }
    out[7] = rest;
    return out;
}

// `lattice::deposits`: W, summing to 8 area exactly.
fn deposits(area: vec2<u32>, wt: array<vec2<u32>, 8>) -> array<vec4<u32>, 8> {
    var weight = wt;
    var out: array<vec4<u32>, 8>;
    var rest = shift4(u4(area), 35u, true, false);
    for (var c = 0u; c < 7u; c++) {
        out[c] = shift4(mul128(area, weight[c]), 27u, false, false);
        rest = sub4(rest, out[c]);
    }
    out[7] = rest;
    return out;
}

// `lattice::node`: the node and the Q31 fraction.
fn node_of(x: vec2<u32>) -> vec2<u32> {
    return vec2<u32>(sar64(x, 31u).x, x.x & 0x7fffffffu);
}

// D16 (`shed::balance`).
struct Balance {
    memory: vec2<u32>,
    threshold: vec2<u32>,
    shed: bool,
    kept: vec2<u32>,
    status: u32,
}

fn balance(q: vec2<u32>, c: vec2<u32>, memory0: vec2<u32>, hazard0: vec2<u32>, tolerance: vec2<u32>, d: vec2<u32>, window: vec2<u32>, keep: vec2<u32>) -> Balance {
    var b: Balance;
    if negative(q) || negative(c) {
        b.status = 1u;
        return b;
    }
    let total = add64(q, c);
    var now = one32();
    if !is_zero(total) {
        let num = shift4(ext4(sub64(q, c)), 32u, true, false);
        now = xnarrow64(xquotient(xof(num), u4(total)).v).v.xy;
    }
    b.memory = shift4(add4(ext4(memory0), ext4(now)), 1u, false, true).xy;
    let hazard = mulq(hazard0, smax64(sub64(tolerance, b.memory), vec2<u32>(0u)));
    b.kept = keep;
    if !slt64(vec2<u32>(0u), hazard) {
        return b;
    }
    let x = smul_shr(hazard, LOG2E, 62u, false);
    b.threshold = threshold(sub64(p_one(), exp2_neg(x)));
    b.shed = lt64(d, b.threshold);
    if !b.shed {
        b.kept = times_q(keep, presence(over_q(b.threshold, d), window));
    }
    return b;
}

// `edge::ripe`.
fn ripe(years: vec2<u32>, ripening: vec2<u32>) -> vec2<u32> {
    if !slt64(vec2<u32>(0u), ripening) {
        return one32();
    }
    let num = shift4(ext4(years), 32u, true, false);
    let q = xnarrow(xquotient(xof(num), ext4(ripening)).v).v;
    if !slt4(q, u4(one32())) {
        return one32();
    }
    return q.xy;
}

// `edge::shade`.
fn shade(l: vec2<u32>, light: vec2<u32>, phi: vec2<u32>) -> vec2<u32> {
    let zero = vec2<u32>(0u);
    if !slt64(zero, phi) || !slt64(light, one32()) || !slt64(zero, l) || is_inf(l) {
        return l;
    }
    if !slt64(zero, light) {
        return inf();
    }
    let factor = exp2q(mulq(phi, sub64(w(0u, 32u), log2q(light))));
    if is_inf(factor) {
        return inf();
    }
    return smin64(mulq(l, factor), inf());
}

// `edge::settle`.
fn settle(fall: u32, x: vec2<u32>) -> u32 {
    let cs = sincos(fall >> 1u);
    let y = smul_shr(cs.zw, exp2_neg(x), 62u, false);
    return atan2_t(y, cs.xy) << 1u;
}

// `edge::tangent_ratio`.
fn tangent_ratio(fall: u32, high: u32) -> vec2<u32> {
    let f = sincos(fall >> 1u);
    let h = sincos(high >> 1u);
    let zero = vec2<u32>(0u);
    let a = mul128(smax64(f.zw, zero), smax64(h.xy, zero));
    let b = mul128(smax64(f.xy, zero), smax64(h.zw, zero));
    if zero4(a) {
        return neg64(inf());
    }
    if zero4(b) {
        return inf();
    }
    return smul_shr(sub64(log2_w(a), log2_w(b)), LN2, 62u, false);
}
