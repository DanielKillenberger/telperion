// The feedback contract's exact intermediates on the GPU (fn-211 task 8,
// CONTRACT-A.md 2.3): `telperion_space::fixed::feedback::wide` and
// `total`, word for word. A 128-bit value is a vec4<u32> from its low
// word up, a 256-bit one an array<u32, 8>, both two's complement.

alias X = array<u32, 8>;

// A 128-bit result and whether it has one (`status` 0), else 1 out of
// range or 2 by none.
struct R4 {
    v: vec4<u32>,
    status: u32,
}

struct R8 {
    v: X,
    status: u32,
}

fn add4(a: vec4<u32>, b: vec4<u32>) -> vec4<u32> {
    let x = a.x + b.x;
    var c = select(0u, 1u, x < a.x);
    let y0 = a.y + b.y;
    let y = y0 + c;
    c = select(0u, 1u, y0 < a.y) + select(0u, 1u, y < y0);
    let z0 = a.z + b.z;
    let z = z0 + c;
    c = select(0u, 1u, z0 < a.z) + select(0u, 1u, z < z0);
    return vec4<u32>(x, y, z, a.w + b.w + c);
}

fn neg4(a: vec4<u32>) -> vec4<u32> {
    return add4(~a, vec4<u32>(1u, 0u, 0u, 0u));
}

fn sub4(a: vec4<u32>, b: vec4<u32>) -> vec4<u32> {
    return add4(a, neg4(b));
}

fn neg4q(a: vec4<u32>) -> bool {
    return (a.w & 0x80000000u) != 0u;
}

fn abs4(a: vec4<u32>) -> vec4<u32> {
    return select(a, neg4(a), neg4q(a));
}

// a < b, unsigned.
fn lt4(a: vec4<u32>, b: vec4<u32>) -> bool {
    if a.w != b.w {
        return a.w < b.w;
    }
    if a.z != b.z {
        return a.z < b.z;
    }
    if a.y != b.y {
        return a.y < b.y;
    }
    return a.x < b.x;
}

fn zero4(a: vec4<u32>) -> bool {
    return all(a == vec4<u32>(0u));
}

// A signed 64-bit value as 128.
fn ext4(a: vec2<u32>) -> vec4<u32> {
    let fill = select(0u, 0xffffffffu, negative(a));
    return vec4<u32>(a.x, a.y, fill, fill);
}

// An unsigned 64-bit value as 128.
fn u4(a: vec2<u32>) -> vec4<u32> {
    return vec4<u32>(a.x, a.y, 0u, 0u);
}

// a shifted by n < 128, logical (`sar` false) or arithmetic.
fn shift4(a: vec4<u32>, n: u32, left: bool, sar: bool) -> vec4<u32> {
    var v = array<u32, 4>(a.x, a.y, a.z, a.w);
    let fill = select(0u, 0xffffffffu, sar && neg4q(a));
    var r = array<u32, 4>(0u, 0u, 0u, 0u);
    let word = n / 32u;
    let bit = n % 32u;
    for (var k = 0u; k < 4u; k++) {
        if left {
            let lo = select(0u, v[min(k - word, 3u)], k >= word);
            let below = select(0u, v[min(k - word - 1u, 3u)], k >= word + 1u);
            r[k] = select((lo << bit) | (below >> (32u - bit)), lo, bit == 0u);
        } else {
            let lo = select(fill, v[min(k + word, 3u)], k + word < 4u);
            let hi = select(fill, v[min(k + word + 1u, 3u)], k + word + 1u < 4u);
            r[k] = select((lo >> bit) | (hi << (32u - bit)), lo, bit == 0u);
        }
    }
    return vec4<u32>(r[0], r[1], r[2], r[3]);
}

// Two's complement of a 256-bit value.
fn xneg(a: X) -> X {
    var m = a;
    var r: X;
    var carry = 1u;
    for (var k = 0u; k < 8u; k++) {
        let v = ~m[k];
        r[k] = v + carry;
        carry = select(0u, 1u, r[k] < v);
    }
    return r;
}

fn xnegative(a: X) -> bool {
    return (a[7] & 0x80000000u) != 0u;
}

// `a b`, exact (`I256::mul`).
fn xmul(a: vec4<u32>, b: vec4<u32>) -> X {
    let ma = abs4(a);
    let mb = abs4(b);
    var r: X;
    for (var i = 0u; i < 4u; i++) {
        var carry = 0u;
        for (var j = 0u; j < 4u; j++) {
            let p = mul32(ma[i], mb[j]);
            let s1 = r[i + j] + p.x;
            let s2 = s1 + carry;
            r[i + j] = s2;
            carry = p.y + select(0u, 1u, s1 < p.x) + select(0u, 1u, s2 < s1);
        }
        r[i + 4u] = carry;
    }
    if neg4q(a) != neg4q(b) {
        return xneg(r);
    }
    return r;
}

// floor(a / 2^n), n < 256.
fn xfloor(a: X, n: u32) -> X {
    var v = a;
    let fill = select(0u, 0xffffffffu, xnegative(a));
    let word = n / 32u;
    let bit = n % 32u;
    var r: X;
    for (var k = 0u; k < 8u; k++) {
        let lo = select(fill, v[min(k + word, 7u)], k + word < 8u);
        let hi = select(fill, v[min(k + word + 1u, 7u)], k + word + 1u < 8u);
        r[k] = select((lo >> bit) | (hi << (32u - bit)), lo, bit == 0u);
    }
    return r;
}

// The value as 128 bits, or out of range.
fn xnarrow(a: X) -> R4 {
    let fill = select(0u, 0xffffffffu, (a[3] & 0x80000000u) != 0u);
    let ok = a[4] == fill && a[5] == fill && a[6] == fill && a[7] == fill;
    return R4(vec4<u32>(a[0], a[1], a[2], a[3]), select(1u, 0u, ok));
}

// The value as 64 bits (in `v.xy`), or out of range.
fn xnarrow64(a: X) -> R4 {
    var m = a;
    let fill = select(0u, 0xffffffffu, (a[1] & 0x80000000u) != 0u);
    var ok = true;
    for (var k = 2u; k < 8u; k++) {
        ok = ok && m[k] == fill;
    }
    return R4(vec4<u32>(a[0], a[1], fill, fill), select(1u, 0u, ok));
}

// a / d toward zero: the magnitudes' quotient by Knuth's algorithm D in
// 16-bit digits, so every partial product and estimate fits a u32
// (Hacker's Delight, `divmnu`). The quotient is unique, so its bits are
// the CPU's bit loop's whatever the algorithm.
fn xquotient(a: X, d: vec4<u32>) -> R8 {
    var q: X;
    if zero4(d) {
        return R8(q, 2u);
    }
    var m = a;
    if xnegative(a) {
        m = xneg(a);
    }
    let den = abs4(d);
    // u has one digit more for the normalising shift.
    var u: array<u32, 17>;
    var v: array<u32, 8>;
    for (var k = 0u; k < 16u; k++) {
        u[k] = (m[k / 2u] >> (16u * (k % 2u))) & 0xffffu;
    }
    for (var k = 0u; k < 8u; k++) {
        v[k] = (den[k / 2u] >> (16u * (k % 2u))) & 0xffffu;
    }
    var n = 8u;
    while v[n - 1u] == 0u {
        n--;
    }
    var len = 16u;
    while len > 0u && u[len - 1u] == 0u {
        len--;
    }
    var digits: array<u32, 16>;
    if len >= n {
        if n == 1u {
            digits = short_quotient(u, len, v[0]);
        } else {
            digits = long_quotient(u, v, len, n);
        }
    }
    for (var k = 0u; k < 16u; k++) {
        q[k / 2u] |= digits[k] << (16u * (k % 2u));
    }
    if xnegative(a) != neg4q(d) {
        q = xneg(q);
    }
    return R8(q, 0u);
}

// The `len` digits of u over the one digit `v`: each partial remainder
// and digit together stay below 2^32.
fn short_quotient(u: array<u32, 17>, len: u32, v: u32) -> array<u32, 16> {
    var digits: array<u32, 16>;
    var r = 0u;
    for (var j = i32(len) - 1; j >= 0; j--) {
        let top = (r << 16u) | u[j];
        digits[j] = top / v;
        r = top - digits[j] * v;
    }
    return digits;
}

// The `len` digits of u over the `n` (two or more) of v, both normalised
// so v's top digit has its high bit set: each digit is estimated from the
// top two of the remainder, corrected by v's second digit, then by one
// add-back where it is still one too large.
fn long_quotient(u_in: array<u32, 17>, v_in: array<u32, 8>, len: u32, n: u32) -> array<u32, 16> {
    var u = u_in;
    var v = v_in;
    var digits: array<u32, 16>;
    let s = countLeadingZeros(v[n - 1u]) - 16u;
    for (var i = n - 1u; i > 0u; i--) {
        v[i] = ((v[i] << s) | (v[i - 1u] >> (16u - s))) & 0xffffu;
    }
    v[0] = (v[0] << s) & 0xffffu;
    u[len] = u[len - 1u] >> (16u - s);
    for (var i = len - 1u; i > 0u; i--) {
        u[i] = ((u[i] << s) | (u[i - 1u] >> (16u - s))) & 0xffffu;
    }
    u[0] = (u[0] << s) & 0xffffu;
    let top = v[n - 1u];
    let next = v[n - 2u];
    for (var j = i32(len - n); j >= 0; j--) {
        let lo = u32(j);
        let head = (u[lo + n] << 16u) | u[lo + n - 1u];
        var qhat = head / top;
        var rhat = head - qhat * top;
        loop {
            if qhat < 0x10000u && qhat * next <= ((rhat << 16u) | u[lo + n - 2u]) {
                break;
            }
            qhat--;
            rhat += top;
            if rhat >= 0x10000u {
                break;
            }
        }
        // Multiply and subtract, borrowing in i32.
        var k = 0;
        for (var i = 0u; i < n; i++) {
            let p = qhat * v[i];
            let t = i32(u[i + lo]) - k - i32(p & 0xffffu);
            u[i + lo] = u32(t) & 0xffffu;
            k = i32(p >> 16u) - (t >> 16u);
        }
        let t = i32(u[lo + n]) - k;
        u[lo + n] = u32(t) & 0xffffu;
        digits[lo] = qhat;
        if t < 0 {
            digits[lo] = qhat - 1u;
            var c = 0u;
            for (var i = 0u; i < n; i++) {
                let sum = u[i + lo] + v[i] + c;
                u[i + lo] = sum & 0xffffu;
                c = sum >> 16u;
            }
            u[lo + n] = (u[lo + n] + c) & 0xffffu;
        }
    }
    return digits;
}

// `mul_shr`: floor(a b / 2^n) as 128 bits.
fn mul_shr(a: vec4<u32>, b: vec4<u32>, n: u32) -> R4 {
    return xnarrow(xfloor(xmul(a, b), n));
}

// The exact 128-bit product of two signed 64-bit values.
fn pair(a: vec2<u32>, b: vec2<u32>) -> vec4<u32> {
    let p = xmul(ext4(a), ext4(b));
    return vec4<u32>(p[0], p[1], p[2], p[3]);
}

// `prod_shr`: floor(f0 f1 f2 f3 / 2^n) as 64 bits.
fn prod_shr(f0: vec2<u32>, f1: vec2<u32>, f2: vec2<u32>, f3: vec2<u32>, n: u32) -> R4 {
    return xnarrow64(xfloor(xmul(pair(f0, f1), pair(f2, f3)), n));
}

// `ratio`: a 2^n / b toward zero, n at most 126.
fn ratio(a: vec4<u32>, b: vec4<u32>, n: u32) -> R4 {
    let q = xquotient(xmul(a, shift4(vec4<u32>(1u, 0u, 0u, 0u), n, true, false)), b);
    if q.status != 0u {
        return R4(vec4<u32>(0u), q.status);
    }
    return xnarrow(q.v);
}

// `zero_shift`: the magnitude shifted, its sign kept.
fn zero_shift(x: vec4<u32>, n: u32) -> vec4<u32> {
    if n == 0u {
        return x;
    }
    let m = shift4(abs4(x), n, false, false);
    return select(m, neg4(m), neg4q(x));
}

// `Total`: a wrapping sum and its terms' magnitudes, saturating.
struct Total {
    value: vec4<u32>,
    magnitude: vec4<u32>,
}

fn total_merge(a: Total, b: Total) -> Total {
    let m = add4(a.magnitude, b.magnitude);
    let over = lt4(m, a.magnitude);
    return Total(add4(a.value, b.value), select(m, vec4<u32>(0xffffffffu), over));
}

fn total_add(a: Total, term: vec4<u32>) -> Total {
    return total_merge(a, Total(term, abs4(term)));
}

// The exact sum, or out of range where the magnitudes reach 2^127.
fn total_get(a: Total) -> R4 {
    return R4(a.value, select(0u, 1u, (a.magnitude.w & 0x80000000u) != 0u));
}
