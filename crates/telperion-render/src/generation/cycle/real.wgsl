// The CPU's few f64 values in the cycle, by their bits (DESIGN.md section
// 8): a presence held as a real (`presence::real`), its complement
// `1 - real(p)` as `make_relay`'s birth draw takes it, a real read back
// as a presence (`presence::q`), and the horizon's max. Each is exact or
// rounds as the CPU's IEEE operation does, to nearest, ties to even.

// `f64::from(p)`: exact.
fn real64(p: f32) -> vec2<u32> {
    let b = bitcast<u32>(p);
    let sign = b & 0x80000000u;
    var e = i32((b >> 23u) & 0xffu);
    var m = b & 0x7fffffu;
    if e == 0 && m == 0u {
        return vec2<u32>(0u, sign);
    }
    if e == 0 {
        // A subnormal: its leading bit moved to the implicit one.
        let lead = 31u - countLeadingZeros(m);
        m = (m << (23u - lead)) & 0x7fffffu;
        e = i32(lead) - 22;
    }
    let ex = u32(e - 127 + 1023);
    return vec2<u32>(m << 29u, sign | (ex << 20u) | (m >> 3u));
}

// `x as f32` of an f64 in [0, 2^128): to nearest, ties to even.
fn single(x: vec2<u32>) -> f32 {
    let ex = i32((x.y >> 20u) & 0x7ffu);
    if ex == 0 {
        return 0.0;
    }
    let e = ex - 1023;
    // The 53-bit significand, its implicit one at bit 52.
    let sig = vec2<u32>(x.x, (x.y & 0xfffffu) | 0x100000u);
    // Bits kept: 24 for a normal f32, fewer below 2^-126.
    var keep = 24;
    if e < -126 {
        keep = 24 - (-126 - e);
    }
    if keep <= 0 {
        // Below half the least subnormal, or at most at it: none, or the
        // least where it is past half.
        let half = keep == 0 && !is_zero(vec2<u32>(sig.x, sig.y & 0xfffffu));
        return select(0.0, bitcast<f32>(1u), half);
    }
    let drop = u32(53 - keep);
    let q = shr64(sig, drop).x;
    let rest = sub64(sig, shl64(w(q, 0u), drop));
    let halfway = shl64(w(1u, 0u), drop - 1u);
    var r = q;
    if lt64(halfway, rest) || (all(rest == halfway) && (q & 1u) == 1u) {
        r += 1u;
    }
    // r carries `keep` bits (or one more where it rounded up past them).
    if e < -126 {
        return bitcast<f32>(r);
    }
    return f32(r) * pow2(-23) * pow2(e);
}

// `presence::q`: a non-negative real as a presence, none for a negative
// one.
fn q64(x: vec2<u32>) -> f32 {
    if (x.y & 0x80000000u) != 0u || is_zero(x) {
        return 0.0;
    }
    return single(x);
}

// `1.0 - f64::from(p)` for a presence `p` in [0, 1]: to nearest, ties
// to even. Its exact value is (2^k - m) 2^-k, `p` = m 2^-k.
fn one_less(p: f32) -> vec2<u32> {
    if p <= 0.0 {
        return ONE64;
    }
    if p >= 1.0 {
        return vec2<u32>(0u);
    }
    let b = bitcast<u32>(p);
    let ex = (b >> 23u) & 0xffu;
    var m = b & 0x7fffffu;
    var e = -149;
    if ex != 0u {
        m |= 0x800000u;
        e = i32(ex) - 150;
    }
    // p = m 2^e, so 1 - p = (2^-e - m) 2^e: exact while 2^-e is 53 bits.
    let k = u32(-e);
    if k <= 53u {
        let exact = sub64(shl64(w(1u, 0u), k), w(m, 0u));
        return from_scaled(exact, e);
    }
    // Past it p is under 2^-30: 1 - p lies in (1/2, 1), its ulp 2^-53,
    // so p rounds to whole ulps of 2^-53, ties to even; past 24 more bits
    // it is under half of one.
    let shift = k - 53u;
    var units = 0u;
    if shift < 25u {
        units = m >> shift;
        let rest = m & ((1u << shift) - 1u);
        let half = 1u << (shift - 1u);
        // Ties to even on 1 - p's last bit, which is units' own.
        if rest > half || (rest == half && (units & 1u) == 1u) {
            units += 1u;
        }
    }
    // 1 - units 2^-53: (2^53 - units) 2^-53.
    return from_scaled(sub64(w(0u, 0x200000u), w(units, 0u)), -53);
}

// The f64 of the integer `v` (below 2^53) times 2^e: exact.
fn from_scaled(v: vec2<u32>, e: i32) -> vec2<u32> {
    if is_zero(v) {
        return vec2<u32>(0u);
    }
    var top = 31u - countLeadingZeros(v.x);
    if v.y != 0u {
        top = 63u - countLeadingZeros(v.y);
    }
    let sig = shl64(v, 52u - top);
    let ex = u32(e + i32(top) + 1023);
    return vec2<u32>(sig.x, (ex << 20u) | (sig.y & 0xfffffu));
}

// `a.max(b)` of two f64 neither NaN: by their bits' order.
fn max64(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    return select(a, b, lt_real(a, b));
}

fn lt_real(a: vec2<u32>, b: vec2<u32>) -> bool {
    let na = (a.y & 0x80000000u) != 0u;
    let nb = (b.y & 0x80000000u) != 0u;
    if na != nb {
        // -0 and +0 are equal.
        let zero = is_zero(vec2<u32>(a.x, a.y & 0x7fffffffu)) && is_zero(vec2<u32>(b.x, b.y & 0x7fffffffu));
        return na && !zero;
    }
    if na {
        return lt64(b, a);
    }
    return lt64(a, b);
}
