// The feedback contract on the GPU (fn-211 task 8, CONTRACT-A.md):
// `telperion_space::fixed::feedback`'s angles, vectors, W powers, lattice,
// D16 and endpoint cases, word for word, over `contract.wgsl` and
// `feedback_wide.wgsl`. An i64 is a vec2<u32>, an i128 a vec4<u32>.

const QUARTER: u32 = 0x40000000u;
const HALF: u32 = 0x80000000u;
const TAU60 = vec2<u32>(0x10b4611au, 0x6487ed51u);
const INV_TAU64 = vec2<u32>(0x9391054au, 0x28be60dbu);
const HALF_ROOT = vec2<u32>(0xfe779921u, 0x2d413cccu);

// `angle::ATAN_TABLE` and `angle::ATAN_POLY`, turns at 2^62 (host
// decision, 2026-10-07: atan2 as a table-reduced polynomial).
var<private> ATAN_TABLE: array<vec2<u32>, 33> = array<vec2<u32>, 33>(
    vec2<u32>(0x00000000u, 0x00000000u), vec2<u32>(0x5641189eu, 0x005175f8u), vec2<u32>(0xc39626bbu, 0x00a2c350u),
    vec2<u32>(0x6b0d0241u, 0x00f3bfe2u), vec2<u32>(0x07776687u, 0x01444475u), vec2<u32>(0xa6d3f8ebu, 0x01942b2du),
    vec2<u32>(0xa10b9cbbu, 0x01e34ff3u), vec2<u32>(0x55d95868u, 0x023190c9u), vec2<u32>(0xd7b8e7a3u, 0x027ece16u),
    vec2<u32>(0x51d62da2u, 0x02caeae5u), vec2<u32>(0xa131dd86u, 0x0315cd0au), vec2<u32>(0x3a87bf6fu, 0x035f5d45u),
    vec2<u32>(0x05e7569eu, 0x03a78749u), vec2<u32>(0x4fd329a4u, 0x03ee39beu), vec2<u32>(0x4a4fb911u, 0x04336634u),
    vec2<u32>(0xd568f037u, 0x04770108u), vec2<u32>(0x677cc21au, 0x04b90147u), vec2<u32>(0xf4f0d62au, 0x04f96080u),
    vec2<u32>(0xa8f1f77cu, 0x05381a9eu), vec2<u32>(0x22231dc9u, 0x05752db1u), vec2<u32>(0xbb7f0790u, 0x05b099bdu),
    vec2<u32>(0x37a1c304u, 0x05ea608bu), vec2<u32>(0xef2a18f9u, 0x0622856eu), vec2<u32>(0x6d25b882u, 0x06590d1bu),
    vec2<u32>(0x31067bcdu, 0x068dfd71u), vec2<u32>(0x1e396c23u, 0x06c15d52u), vec2<u32>(0xf7409c90u, 0x06f33477u),
    vec2<u32>(0x1dba3856u, 0x07238b4eu), vec2<u32>(0xb22c6ed4u, 0x07526aceu), vec2<u32>(0x158f85d0u, 0x077fdc63u),
    vec2<u32>(0xbafc8e9fu, 0x07abe9c7u), vec2<u32>(0x29142823u, 0x07d69cf3u), vec2<u32>(0x00000000u, 0x08000000u),
);
var<private> ATAN_POLY: array<vec2<u32>, 4> = array<vec2<u32>, 4>(
    vec2<u32>(0xe4e44153u, 0x0a2f9836u), vec2<u32>(0x09093f8fu, 0xfc9acd43u), vec2<u32>(0xc760d9ddu, 0x020984d7u), vec2<u32>(0x71963fd0u, 0xfe8b7c8au),
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

// `angle::SIN` and `angle::COS`, Q62.
var<private> SIN: array<vec2<u32>, 6> = array<vec2<u32>, 6>(
    vec2<u32>(0x00000000u, 0x40000000u), vec2<u32>(0x55555555u, 0xf5555555u), vec2<u32>(0x88888889u, 0x00888888u), vec2<u32>(0xfcbfcbfdu, 0xfffcbfcbu), vec2<u32>(0xf1d2ab64u, 0x00000b8eu), vec2<u32>(0x19ba980bu, 0xffffffe5u),
);
var<private> COS: array<vec2<u32>, 6> = array<vec2<u32>, 6>(
    vec2<u32>(0x00000000u, 0x40000000u), vec2<u32>(0x00000000u, 0xe0000000u), vec2<u32>(0xaaaaaaabu, 0x02aaaaaau), vec2<u32>(0xe93e93e9u, 0xffe93e93u), vec2<u32>(0x80680680u, 0x00006806u), vec2<u32>(0x1b048876u, 0xfffffed8u),
);

// Horner from the highest term down, one floor a product.
fn horner_sin(z: vec2<u32>) -> vec2<u32> {
    var p = vec2<u32>(0u);
    for (var k = 6; k > 0; k--) {
        p = add64(SIN[k - 1], mul62s(z, p));
    }
    return p;
}

fn horner_cos(z: vec2<u32>) -> vec2<u32> {
    var p = vec2<u32>(0u);
    for (var k = 6; k > 0; k--) {
        p = add64(COS[k - 1], mul62s(z, p));
    }
    return p;
}

// `angle::sincos`: (cos.xy, sin.zw).
fn sincos(t: u32) -> vec4<u32> {
    let n = (t + (1u << 28u)) >> 29u;
    let r = bitcast<i32>(t - (n << 29u));
    let theta = smul_shr(i64of(r), TAU60, 30u, false);
    let z = mul62s(theta, theta);
    var c = horner_cos(z);
    var s = mul62s(theta, horner_sin(z));
    if (n & 1u) == 1u {
        let c2 = mul62s(sub64(c, s), HALF_ROOT);
        s = mul62s(add64(c, s), HALF_ROOT);
        c = c2;
    }
    let q = n >> 1u;
    var cs = vec4<u32>(s, neg64(c));
    if q == 0u {
        cs = vec4<u32>(c, s);
    } else if q == 1u {
        cs = vec4<u32>(neg64(s), c);
    } else if q == 2u {
        cs = vec4<u32>(neg64(c), neg64(s));
    }
    let lo = neg64(one32());
    return vec4<u32>(smax64(smin64(sar64(cs.xy, 30u), one32()), lo), smax64(smin64(sar64(cs.zw, 30u), one32()), lo));
}

// `angle::atan2`: the lesser magnitude over the greater, the nearest
// thirty-second's table entry and a cubic in u^2 by Horner, the octant
// restored, to the nearest T.
fn atan2_t(y0: vec2<u32>, x0: vec2<u32>) -> u32 {
    if is_zero(y0) {
        return select(0u, HALF, negative(x0));
    }
    if is_zero(x0) {
        return select(QUARTER, 3u * QUARTER, negative(y0));
    }
    let mx = abs64(x0);
    let my = abs64(y0);
    let steep = lt64(mx, my);
    let lo = select(my, mx, steep);
    let hi = select(mx, my, steep);
    let t = xquotient(xof(shift4(u4(lo), 62u, true, false)), u4(hi)).v;
    let tq = vec2<u32>(t[0], t[1]);
    let j = add64(tq, w(0u, 0x1000000u)).y >> 25u;
    let c = w(0u, j << 25u);
    let den = add4(vec4<u32>(0u, 0u, 0u, 0u), u4(add64(w(0u, 0x40000000u), smul_shr(tq, c, 62u, false))));
    let num = shift4(ext4(sub64(tq, c)), 62u, true, false);
    let uq = xnarrow64(xquotient(xof(num), den).v).v.xy;
    let z = smul_shr(uq, uq, 62u, false);
    var p = vec2<u32>(0u);
    for (var k = 4; k > 0; k--) {
        p = add64(ATAN_POLY[k - 1], smul_shr(z, p, 62u, false));
    }
    var a = add64(ATAN_TABLE[j], smul_shr(uq, p, 62u, false));
    if steep {
        a = sub64(w(0u, 0x10000000u), a);
    }
    if negative(x0) {
        a = sub64(w(0u, 0x20000000u), a);
    }
    if negative(y0) {
        a = neg64(a);
    }
    return sar64(add64(a, w(1u << 29u, 0u)), 30u).x;
}

// `vector::isqrt`: floor(sqrt n), digit by digit.
fn isqrt(n: vec4<u32>) -> vec2<u32> {
    var num = n;
    var res = vec4<u32>(0u);
    var bit = vec4<u32>(0u, 0u, 0u, 0x40000000u);
    // cap: bit shifts right by two from bit 126, 64 steps at most.
    while !zero4(bit) && lt4(num, bit) {
        bit = shift4(bit, 2u, false, false);
    }
    // cap: bit shifts right by two from bit 126, 64 steps at most.
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
    return atan2_t(x, rest(x));
}

fn acos_t(c0: vec2<u32>) -> u32 {
    let c = clamp_one(c0);
    return atan2_t(rest(c), c);
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

// `lay::over` (and `geometry::over`): trunc(a 2^32 / b), as `as i64`.
fn over(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    let q = xquotient(xof(shift4(ext4(a), 32u, true, false)), ext4(b)).v;
    return vec2<u32>(q[0], q[1]);
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
