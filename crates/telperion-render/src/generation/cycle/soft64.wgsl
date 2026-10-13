// IEEE-754 binary64 in integers (DESIGN.md section 8): an f64 is its bits
// as (low word, high word), and every operation rounds to nearest, ties
// to even, by integer arithmetic alone, so no driver can fuse, reorder
// or approximate it. Add, subtract, multiply and divide, floor, and the
// conversions the light takes: f32 and integers to f64 exactly, f64 to
// f32 to nearest, f64 to i32 toward zero. Finite inputs only: no
// operation the sweep takes reaches an infinity or a NaN, and a finite
// result past the largest f64 rounds to infinity.
//
// A working real is a sign, an exponent `e` and a 64-bit significand `m`
// whose leading one sits at bit 62: its value is m 2^(e - 1085), and `e`
// is the f64's biased exponent where that is normal.

const SIGN: u32 = 0x80000000u;

struct Wide {
    s: u32,
    e: i32,
    m: vec2<u32>,
}

fn u64_add(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    let lo = a.x + b.x;
    return vec2<u32>(lo, a.y + b.y + select(0u, 1u, lo < a.x));
}

fn u64_sub(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    return vec2<u32>(a.x - b.x, a.y - b.y - select(0u, 1u, a.x < b.x));
}

fn u64_lt(a: vec2<u32>, b: vec2<u32>) -> bool {
    return a.y < b.y || (a.y == b.y && a.x < b.x);
}

fn u64_shl(a: vec2<u32>, n: u32) -> vec2<u32> {
    if n == 0u {
        return a;
    }
    if n >= 64u {
        return vec2<u32>(0u);
    }
    if n >= 32u {
        return vec2<u32>(0u, a.x << (n - 32u));
    }
    return vec2<u32>(a.x << n, (a.y << n) | (a.x >> (32u - n)));
}

fn u64_shr(a: vec2<u32>, n: u32) -> vec2<u32> {
    if n == 0u {
        return a;
    }
    if n >= 64u {
        return vec2<u32>(0u);
    }
    if n >= 32u {
        return vec2<u32>(a.y >> (n - 32u), 0u);
    }
    return vec2<u32>((a.x >> n) | (a.y << (32u - n)), a.y >> n);
}

// `a` shifted right by `n`, any bit shifted out kept as bit 0.
fn u64_shr_sticky(a: vec2<u32>, n: u32) -> vec2<u32> {
    let r = u64_shr(a, n);
    let lost = any(u64_shl(r, min(n, 64u)) != a) || (n >= 64u && any(a != vec2<u32>(0u)));
    return r | vec2<u32>(select(0u, 1u, lost), 0u);
}

fn u64_clz(a: vec2<u32>) -> u32 {
    if a.y != 0u {
        return countLeadingZeros(a.y);
    }
    return 32u + countLeadingZeros(a.x);
}

// The full 64-bit product of two words, in 16-bit halves.
fn mul32(a: u32, b: u32) -> vec2<u32> {
    let al = a & 0xffffu;
    let ah = a >> 16u;
    let bl = b & 0xffffu;
    let bh = b >> 16u;
    let ll = al * bl;
    let mid = u64_add(vec2<u32>(al * bh, 0u), vec2<u32>(ah * bl, 0u));
    let lo = u64_add(vec2<u32>(ll, 0u), vec2<u32>(mid.x << 16u, 0u));
    let hi = ah * bh + (mid.x >> 16u) + (mid.y << 16u) + lo.y;
    return vec2<u32>(lo.x, hi);
}

fn magnitude(x: vec2<u32>) -> vec2<u32> {
    return vec2<u32>(x.x, x.y & 0x7fffffffu);
}

fn is_zero64(x: vec2<u32>) -> bool {
    return all(magnitude(x) == vec2<u32>(0u));
}

// A nonzero finite f64 as a working real.
fn unpack(x: vec2<u32>) -> Wide {
    let s = x.y & SIGN;
    let e = i32((x.y >> 20u) & 0x7ffu);
    let frac = vec2<u32>(x.x, x.y & 0xfffffu);
    if e == 0 {
        let sh = u64_clz(frac) - 1u;
        return Wide(s, 11 - i32(sh), u64_shl(frac, sh));
    }
    return Wide(s, e, u64_shl(frac | vec2<u32>(0u, 0x100000u), 10u));
}

// The f64 nearest m 2^(e - 1085), ties to even; `m` nonzero, its leading
// one at bit 62 or 63.
fn round_pack(s: u32, e0: i32, m0: vec2<u32>) -> vec2<u32> {
    var e = e0;
    var m = m0;
    if (m.y & SIGN) != 0u {
        m = u64_shr_sticky(m, 1u);
        e += 1;
    }
    if e < 1 {
        m = u64_shr_sticky(m, u32(min(1 - e, 64)));
        e = 1;
    }
    if e >= 2047 {
        return vec2<u32>(0u, s | 0x7ff00000u);
    }
    let low = m.x & 0x3ffu;
    var q = u64_shr(m, 10u);
    if low > 0x200u || (low == 0x200u && (q.x & 1u) == 1u) {
        q = u64_add(q, vec2<u32>(1u, 0u));
    }
    let bits = u64_add(vec2<u32>(0u, u32(e - 1) << 20u), q);
    return vec2<u32>(bits.x, bits.y | s);
}

fn f64_add(a0: vec2<u32>, b0: vec2<u32>) -> vec2<u32> {
    if is_zero64(a0) && is_zero64(b0) {
        return vec2<u32>(0u, a0.y & b0.y & SIGN);
    }
    if is_zero64(b0) {
        return a0;
    }
    if is_zero64(a0) {
        return b0;
    }
    var a = a0;
    var b = b0;
    if u64_lt(magnitude(a), magnitude(b)) {
        a = b0;
        b = a0;
    }
    let wa = unpack(a);
    let wb = unpack(b);
    let mb = u64_shr_sticky(wb.m, u32(min(wa.e - wb.e, 64)));
    if wa.s == wb.s {
        return round_pack(wa.s, wa.e, u64_add(wa.m, mb));
    }
    let m = u64_sub(wa.m, mb);
    if all(m == vec2<u32>(0u)) {
        return vec2<u32>(0u);
    }
    let sh = u64_clz(m) - 1u;
    return round_pack(wa.s, wa.e - i32(sh), u64_shl(m, sh));
}

fn f64_mul(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    let s = (a.y ^ b.y) & SIGN;
    if is_zero64(a) || is_zero64(b) {
        return vec2<u32>(0u, s);
    }
    let wa = unpack(a);
    let wb = unpack(b);
    // The 53-bit significands' 106-bit product, in four words.
    let x = u64_shr(wa.m, 10u);
    let y = u64_shr(wb.m, 10u);
    let p00 = mul32(x.x, y.x);
    let mid = u64_add(u64_add(vec2<u32>(p00.y, 0u), mul32(x.x, y.y)), mul32(x.y, y.x));
    let top = u64_add(mul32(x.y, y.y), vec2<u32>(mid.y, 0u));
    // Its bits 42 and up, the rest kept as a sticky bit.
    var m = vec2<u32>((mid.x >> 10u) | (top.x << 22u), (top.x >> 10u) | (top.y << 22u));
    if p00.x != 0u || (mid.x & 0x3ffu) != 0u {
        m.x |= 1u;
    }
    return round_pack(s, wa.e + wb.e - 1023, m);
}

// The divisor nonzero.
fn f64_div(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    let s = (a.y ^ b.y) & SIGN;
    if is_zero64(a) {
        return vec2<u32>(0u, s);
    }
    let wa = unpack(a);
    let wb = unpack(b);
    var r = u64_shr(wa.m, 10u);
    let d = u64_shr(wb.m, 10u);
    var e = wa.e - wb.e + 1023;
    if u64_lt(r, d) {
        r = u64_shl(r, 1u);
        e -= 1;
    }
    // 63 quotient bits by restoring division, the leading one at bit 62.
    var q = vec2<u32>(0u);
    for (var i = 0u; i < 63u; i++) {
        q = u64_shl(q, 1u);
        if !u64_lt(r, d) {
            r = u64_sub(r, d);
            q.x |= 1u;
        }
        r = u64_shl(r, 1u);
    }
    if any(r != vec2<u32>(0u)) {
        q.x |= 1u;
    }
    return round_pack(s, e, q);
}

fn f64_neg(x: vec2<u32>) -> vec2<u32> {
    return vec2<u32>(x.x, x.y ^ SIGN);
}

// `f64::from(x)`: exact.
fn f64_from_f32(x: f32) -> vec2<u32> {
    let b = bitcast<u32>(x);
    let s = b & SIGN;
    var e = i32((b >> 23u) & 0xffu);
    var m = b & 0x7fffffu;
    if e == 0 && m == 0u {
        return vec2<u32>(0u, s);
    }
    if e == 0 {
        let lead = 31u - countLeadingZeros(m);
        m = (m << (23u - lead)) & 0x7fffffu;
        e = i32(lead) - 22;
    }
    let ex = u32(e - 127 + 1023);
    return vec2<u32>(m << 29u, s | (ex << 20u) | (m >> 3u));
}

// `x as f64` of a word: exact.
fn f64_from_u32(n: u32) -> vec2<u32> {
    if n == 0u {
        return vec2<u32>(0u);
    }
    let lead = 31u - countLeadingZeros(n);
    let frac = u64_shl(vec2<u32>(n, 0u), 52u - lead);
    return vec2<u32>(frac.x, (frac.y & 0xfffffu) | ((1023u + lead) << 20u));
}

fn f64_from_i32(n: i32) -> vec2<u32> {
    let x = f64_from_u32(select(u32(n), 0u - u32(n), n < 0));
    return vec2<u32>(x.x, x.y | select(0u, SIGN, n < 0));
}

// `x as f32`: to nearest, ties to even, subnormals included.
fn f64_to_f32(x: vec2<u32>) -> f32 {
    let s = x.y & SIGN;
    if is_zero64(x) {
        return bitcast<f32>(s);
    }
    let w = unpack(x);
    var e = w.e - 896;
    var m = w.m;
    if e < 1 {
        m = u64_shr_sticky(m, u32(min(1 - e, 64)));
        e = 1;
    }
    if e >= 255 {
        return bitcast<f32>(s | 0x7f800000u);
    }
    // 24 bits kept, from bit 39; the 39 below decide the rounding.
    var q = m.y >> 7u;
    let low = vec2<u32>(m.x, m.y & 0x7fu);
    let half = vec2<u32>(0u, 0x40u);
    if u64_lt(half, low) || (all(low == half) && (q & 1u) == 1u) {
        q += 1u;
    }
    return bitcast<f32>(s | ((u32(e - 1) << 23u) + q));
}

// `x.floor()`.
fn f64_floor(x: vec2<u32>) -> vec2<u32> {
    let s = x.y & SIGN;
    let ue = i32((x.y >> 20u) & 0x7ffu) - 1023;
    if ue >= 52 {
        return x;
    }
    if ue < 0 {
        if is_zero64(x) || s == 0u {
            return vec2<u32>(0u, s);
        }
        return vec2<u32>(0u, 0xbff00000u);
    }
    let unit = u64_shl(vec2<u32>(1u, 0u), u32(52 - ue));
    let mask = u64_sub(unit, vec2<u32>(1u, 0u));
    if all((x & mask) == vec2<u32>(0u)) {
        return x;
    }
    let whole = x & ~mask;
    if s == 0u {
        return whole;
    }
    let up = u64_add(magnitude(whole), unit);
    return vec2<u32>(up.x, up.y | s);
}

// `x as i32`: toward zero, saturating.
fn f64_to_i32(x: vec2<u32>) -> i32 {
    let neg = (x.y & SIGN) != 0u;
    let ue = i32((x.y >> 20u) & 0x7ffu) - 1023;
    if ue < 0 {
        return 0;
    }
    if ue >= 31 {
        return select(2147483647, -2147483647 - 1, neg);
    }
    let sig = vec2<u32>(x.x, (x.y & 0xfffffu) | 0x100000u);
    let v = i32(u64_shr(sig, u32(52 - ue)).x);
    return select(v, -v, neg);
}

// `a < b` of two finite f64s, a zero of either sign equal to the other.
fn f64_lt(a: vec2<u32>, b: vec2<u32>) -> bool {
    let na = (a.y & SIGN) != 0u && !is_zero64(a);
    let nb = (b.y & SIGN) != 0u && !is_zero64(b);
    if na != nb {
        return na;
    }
    if na {
        return u64_lt(magnitude(b), magnitude(a));
    }
    return u64_lt(magnitude(a), magnitude(b));
}
