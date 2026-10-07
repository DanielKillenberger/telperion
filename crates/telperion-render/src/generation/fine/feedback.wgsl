// The feedback contract on the GPU (fn-211 task 8, CONTRACT-A.md):
// `telperion_space::fixed::feedback`'s angles, vectors, W powers, lattice,
// D16 and endpoint cases, word for word, over `contract.wgsl` and
// `feedback_wide.wgsl`. An i64 is a vec2<u32>, an i128 a vec4<u32>.

const QUARTER: u32 = 0x40000000u;
const HALF: u32 = 0x80000000u;
const TAU60 = vec2<u32>(0x10b4611au, 0x6487ed51u);
const INV_TAU64 = vec2<u32>(0x9391054au, 0x28be60dbu);
const CORDIC_K = vec2<u32>(0x10d7969au, 0x26dd3b6au);

// atan(2^-i) in turns at 2^62 a turn (`angle::ATAN`).
var<private> ATAN: array<vec2<u32>, 40> = array<vec2<u32>, 40>(
    vec2<u32>(0x00000000u, 0x08000000u), vec2<u32>(0x677cc21au, 0x04b90147u),
    vec2<u32>(0xd7b8e7a3u, 0x027ece16u), vec2<u32>(0x07776687u, 0x01444475u),
    vec2<u32>(0xc39626bbu, 0x00a2c350u), vec2<u32>(0x5641189eu, 0x005175f8u),
    vec2<u32>(0x970a098au, 0x0028bd87u), vec2<u32>(0x447510acu, 0x00145f15u),
    vec2<u32>(0xd1b430ceu, 0x000a2f94u), vec2<u32>(0xaecc2aceu, 0x000517cbu),
    vec2<u32>(0x00246e9fu, 0x00028be6u), vec2<u32>(0x052a032eu, 0x000145f3u),
    vec2<u32>(0x8337fb18u, 0x0000a2f9u), vec2<u32>(0xc1b05cbdu, 0x0000517cu),
    vec2<u32>(0x60daba44u, 0x000028beu), vec2<u32>(0x306dae9fu, 0x0000145fu),
    vec2<u32>(0x9836e17fu, 0x00000a2fu), vec2<u32>(0xcc1b7205u, 0x00000517u),
    vec2<u32>(0xe60db92bu, 0x0000028bu), vec2<u32>(0xf306dc9bu, 0x00000145u),
    vec2<u32>(0xf9836e4eu, 0x000000a2u), vec2<u32>(0x7cc1b727u, 0x00000051u),
    vec2<u32>(0xbe60db94u, 0x00000028u), vec2<u32>(0x5f306dcau, 0x00000014u),
    vec2<u32>(0x2f9836e5u, 0x0000000au), vec2<u32>(0x17cc1b72u, 0x00000005u),
    vec2<u32>(0x8be60db9u, 0x00000002u), vec2<u32>(0x45f306ddu, 0x00000001u),
    vec2<u32>(0xa2f9836eu, 0x00000000u), vec2<u32>(0x517cc1b7u, 0x00000000u),
    vec2<u32>(0x28be60dcu, 0x00000000u), vec2<u32>(0x145f306eu, 0x00000000u),
    vec2<u32>(0x0a2f9837u, 0x00000000u), vec2<u32>(0x0517cc1bu, 0x00000000u),
    vec2<u32>(0x028be60eu, 0x00000000u), vec2<u32>(0x0145f307u, 0x00000000u),
    vec2<u32>(0x00a2f983u, 0x00000000u), vec2<u32>(0x00517cc2u, 0x00000000u),
    vec2<u32>(0x0028be61u, 0x00000000u), vec2<u32>(0x00145f30u, 0x00000000u),
);

fn sar64(a: vec2<u32>, n: u32) -> vec2<u32> {
    return shift4(ext4(a), n, false, true).xy;
}

fn i64of(v: i32) -> vec2<u32> {
    return w(bitcast<u32>(v), select(0u, 0xffffffffu, v < 0));
}

// a < b, signed 128.
fn slt4(a: vec4<u32>, b: vec4<u32>) -> bool {
    if neg4q(a) != neg4q(b) {
        return neg4q(a);
    }
    return lt4(a, b);
}

fn xof(a: vec4<u32>) -> X {
    let f = select(0u, 0xffffffffu, neg4q(a));
    return X(a.x, a.y, a.z, a.w, f, f, f, f);
}

fn clz128(a: vec4<u32>) -> u32 {
    if a.w != 0u {
        return countLeadingZeros(a.w);
    }
    if a.z != 0u {
        return 32u + countLeadingZeros(a.z);
    }
    return 64u + clz64(a.xy);
}

// `angle::radians`: v.xy, or out of range at 2^60.
fn radians(a: vec2<u32>) -> R4 {
    if !lt64(abs64(a), w(0u, 0x10000000u)) {
        return R4(vec4<u32>(0u), 1u);
    }
    let p = xfloor(xmul(ext4(a), u4(TAU60)), 60u);
    return R4(vec4<u32>(p[0], p[1], 0u, 0u), 0u);
}

fn turns_of(r: vec2<u32>) -> vec2<u32> {
    let p = xfloor(xmul(ext4(r), u4(INV_TAU64)), 64u);
    return vec2<u32>(p[0], p[1]);
}

fn between(origin: u32, to: u32) -> vec2<u32> {
    return i64of(bitcast<i32>(to - origin));
}

fn toward(origin: u32, to: u32, share: vec2<u32>) -> u32 {
    return origin + smul_shr(between(origin, to), share, 32u, false).x;
}

// CORDIC in rotation mode through r, T units in [-1/8, 1/8): (cos, sin).
fn rotated(r: i32) -> vec4<u32> {
    var x = CORDIC_K;
    var y = vec2<u32>(0u);
    var z = shl64(i64of(r), 30u);
    for (var i = 0u; i < 40u; i++) {
        let dx = sar64(y, i);
        let dy = sar64(x, i);
        if !negative(z) {
            x = sub64(x, dx);
            y = add64(y, dy);
            z = sub64(z, ATAN[i]);
        } else {
            x = add64(x, dx);
            y = sub64(y, dy);
            z = add64(z, ATAN[i]);
        }
    }
    let lo = neg64(one32());
    return vec4<u32>(smax64(smin64(sar64(x, 30u), one32()), lo), smax64(smin64(sar64(y, 30u), one32()), lo));
}

// `angle::sincos`: (cos.xy, sin.zw).
fn sincos(t: u32) -> vec4<u32> {
    let q = (t + (1u << 29u)) >> 30u;
    let r = bitcast<i32>(t - (q << 30u));
    var cs = vec4<u32>(one32(), 0u, 0u);
    if r != 0 {
        cs = rotated(r);
    }
    let c = cs.xy;
    let s = cs.zw;
    var out = vec4<u32>(s, neg64(c));
    if q == 0u {
        out = vec4<u32>(c, s);
    } else if q == 1u {
        out = vec4<u32>(neg64(s), c);
    } else if q == 2u {
        out = vec4<u32>(neg64(c), neg64(s));
    }
    return out;
}

// A magnitude scaled by the common shift, its sign restored.
fn scaled_by(v: vec2<u32>, top: u32) -> vec2<u32> {
    let m = abs64(v);
    let s = select(shr64(m, top - 60u), shl64(m, 60u - top), top <= 60u);
    return select(s, neg64(s), negative(v));
}

// `angle::atan2`.
fn atan2(y0: vec2<u32>, x0: vec2<u32>) -> u32 {
    if is_zero(y0) {
        return select(0u, HALF, negative(x0));
    }
    if is_zero(x0) {
        return select(QUARTER, 3u * QUARTER, negative(y0));
    }
    let mx = abs64(x0);
    let my = abs64(y0);
    let top = 63u - clz64(select(my, mx, lt64(my, mx)));
    var x = scaled_by(x0, top);
    var y = scaled_by(y0, top);
    var base = 0u;
    if negative(x) {
        x = neg64(x);
        y = neg64(y);
        base = HALF;
    }
    var z = vec2<u32>(0u);
    for (var i = 0u; i < 40u; i++) {
        let dx = sar64(y, i);
        let dy = sar64(x, i);
        if !negative(y) {
            x = add64(x, dx);
            y = sub64(y, dy);
            z = add64(z, ATAN[i]);
        } else {
            x = sub64(x, dx);
            y = add64(y, dy);
            z = sub64(z, ATAN[i]);
        }
    }
    return base + sar64(add64(z, w(1u << 29u, 0u)), 30u).x;
}

// `vector::isqrt`: floor(sqrt n), digit by digit.
fn isqrt(n: vec4<u32>) -> vec2<u32> {
    var num = n;
    var res = vec4<u32>(0u);
    var bit = vec4<u32>(0u, 0u, 0u, 0x40000000u);
    while !zero4(bit) && lt4(num, bit) {
        bit = shift4(bit, 2u, false, false);
    }
    while !zero4(bit) {
        let t = add4(res, bit);
        if !lt4(num, t) {
            num = sub4(num, t);
            res = add4(shift4(res, 1u, false, false), bit);
        } else {
            res = shift4(res, 1u, false, false);
        }
        bit = shift4(bit, 2u, false, false);
    }
    return res.xy;
}

// sqrt(1 - x^2), Q32, of x clamped to [-1, 1].
fn rest(x: vec2<u32>) -> vec2<u32> {
    let m = abs64(x);
    return isqrt(sub4(vec4<u32>(0u, 0u, 1u, 0u), mul128(m, m)));
}

fn clamp_one(x: vec2<u32>) -> vec2<u32> {
    return smax64(smin64(x, one32()), neg64(one32()));
}

fn asin_t(x0: vec2<u32>) -> u32 {
    let x = clamp_one(x0);
    return atan2(x, rest(x));
}

fn acos_t(c0: vec2<u32>) -> u32 {
    let c = clamp_one(c0);
    return atan2(rest(c), c);
}

fn length3(a: vec2<u32>, b: vec2<u32>, c: vec2<u32>) -> vec2<u32> {
    let ma = abs64(a);
    let mb = abs64(b);
    let mc = abs64(c);
    return isqrt(add4(add4(mul128(ma, ma), mul128(mb, mb)), mul128(mc, mc)));
}

// A vector at the common scale (`vector::normal`).
struct Normal {
    c: array<vec2<u32>, 3>,
    root: vec2<u32>,
    shift: i32,
    some: bool,
}

fn normal(v: array<vec4<u32>, 3>) -> Normal {
    var n: Normal;
    var u = v;
    var most = vec4<u32>(0u);
    for (var k = 0u; k < 3u; k++) {
        let m = abs4(u[k]);
        most = select(most, m, lt4(most, m));
    }
    if zero4(most) {
        return n;
    }
    let shift = 61 - (127 - i32(clz128(most)));
    var sum = vec4<u32>(0u);
    for (var k = 0u; k < 3u; k++) {
        let m = abs4(u[k]);
        var s: vec2<u32>;
        if shift >= 0 {
            s = shift4(m, u32(shift), true, false).xy;
        } else {
            s = shift4(m, min(u32(-shift), 127u), false, false).xy;
        }
        sum = add4(sum, mul128(s, s));
        n.c[k] = select(s, neg64(s), neg4q(u[k]));
    }
    n.root = isqrt(sum);
    n.shift = shift;
    n.some = true;
    return n;
}

// c 2^32 / root toward zero, each component.
fn unit_of(n: Normal) -> array<vec2<u32>, 3> {
    var c = n.c;
    var out: array<vec2<u32>, 3>;
    for (var k = 0u; k < 3u; k++) {
        let num = shift4(ext4(c[k]), 32u, true, false);
        out[k] = xnarrow64(xquotient(xof(num), u4(n.root)).v).v.xy;
    }
    return out;
}

// `vector::unit`: none (all zero, `some` false) for the zero vector.
fn unit3(v: array<vec2<u32>, 3>) -> Normal {
    var u = v;
    var n = normal(array<vec4<u32>, 3>(ext4(u[0]), ext4(u[1]), ext4(u[2])));
    if n.some {
        n.c = unit_of(n);
    }
    return n;
}

// `vector::unit_w`: the unit in `c` and the log2 length in `root`.
fn unit_w(v: array<vec4<u32>, 3>) -> Normal {
    var n = normal(v);
    if !n.some {
        n.root = neg64(inf());
        return n;
    }
    n.c = unit_of(n);
    n.root = sub64(log2q(n.root), w(0u, bitcast<u32>(n.shift + 64)));
    return n;
}

// `power::log2_w`.
fn log2_w(n: vec4<u32>) -> vec2<u32> {
    if zero4(n) {
        return neg64(inf());
    }
    let e = 127u - clz128(n);
    let m = select(shift4(n, 62u - e, true, false), shift4(n, e - 62u, false, false), e >= 62u);
    return w(log2_frac(m.xy), bitcast<u32>(i32(e) - 64));
}

// `power::exp2_w`.
fn exp2_w(y: vec2<u32>) -> R4 {
    let whole = bitcast<i32>(y.y);
    if whole >= 62 {
        return R4(vec4<u32>(0u), 1u);
    }
    var m = p_one();
    var s = whole + 2;
    if y.x != 0u {
        m = exp2_neg(w(0u - y.x, 0u));
        s = whole + 3;
    }
    if s >= 64 {
        return R4(vec4<u32>(0u), 1u);
    }
    if s >= 0 {
        return R4(shift4(u4(m), u32(s), true, false), 0u);
    }
    if s >= -63 {
        return R4(shift4(u4(m), u32(-s), false, false), 0u);
    }
    return R4(vec4<u32>(0u), 0u);
}
