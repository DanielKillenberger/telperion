// Wide integers in 32-bit words (fn-209 F1, the decision contract's
// mirror): a u64 or two's complement i64 is a vec2<u32> (low, high), a
// 128-bit product a vec4<u32> from its low word up. Every operation here
// is exact, so the contract's arithmetic (`contract.wgsl`) gives the CPU's
// bits (`telperion_space::fixed`).

fn w(lo: u32, hi: u32) -> vec2<u32> {
    return vec2<u32>(lo, hi);
}

fn add64(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    let lo = a.x + b.x;
    return vec2<u32>(lo, a.y + b.y + select(0u, 1u, lo < a.x));
}

fn sub64(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    return vec2<u32>(a.x - b.x, a.y - b.y - select(0u, 1u, a.x < b.x));
}

fn neg64(a: vec2<u32>) -> vec2<u32> {
    return sub64(vec2<u32>(0u), a);
}

fn is_zero(a: vec2<u32>) -> bool {
    return a.x == 0u && a.y == 0u;
}

// a < b, unsigned.
fn lt64(a: vec2<u32>, b: vec2<u32>) -> bool {
    return a.y < b.y || (a.y == b.y && a.x < b.x);
}

// a < b, signed.
fn slt64(a: vec2<u32>, b: vec2<u32>) -> bool {
    let ah = bitcast<i32>(a.y);
    let bh = bitcast<i32>(b.y);
    return ah < bh || (ah == bh && a.x < b.x);
}

fn negative(a: vec2<u32>) -> bool {
    return (a.y & 0x80000000u) != 0u;
}

fn min64(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    return select(b, a, lt64(a, b));
}

fn smin64(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    return select(b, a, slt64(a, b));
}

fn smax64(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    return select(a, b, slt64(a, b));
}

fn shl64(a: vec2<u32>, n: u32) -> vec2<u32> {
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

fn shr64(a: vec2<u32>, n: u32) -> vec2<u32> {
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

fn rotl64(a: vec2<u32>, n: u32) -> vec2<u32> {
    let r = shl64(a, n);
    let s = shr64(a, 64u - n);
    return vec2<u32>(r.x | s.x, r.y | s.y);
}

fn xor64(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    return vec2<u32>(a.x ^ b.x, a.y ^ b.y);
}

fn clz64(a: vec2<u32>) -> u32 {
    if a.y != 0u {
        return countLeadingZeros(a.y);
    }
    return 32u + countLeadingZeros(a.x);
}

// The full product of two words.
fn mul32(a: u32, b: u32) -> vec2<u32> {
    let a0 = a & 0xffffu;
    let a1 = a >> 16u;
    let b0 = b & 0xffffu;
    let b1 = b >> 16u;
    let p00 = a0 * b0;
    let p01 = a0 * b1;
    let p10 = a1 * b0;
    let p11 = a1 * b1;
    let mid = p01 + p10;
    let mid_carry = select(0u, 0x10000u, mid < p01);
    let lo = p00 + (mid << 16u);
    let lo_carry = select(0u, 1u, lo < p00);
    return vec2<u32>(lo, p11 + (mid >> 16u) + mid_carry + lo_carry);
}

// The low 64 bits of a product, wrapping.
fn mul64(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    let p = mul32(a.x, b.x);
    return vec2<u32>(p.x, p.y + a.x * b.y + a.y * b.x);
}

// The full 128-bit product.
fn mul128(a: vec2<u32>, b: vec2<u32>) -> vec4<u32> {
    let p0 = mul32(a.x, b.x);
    let p1 = mul32(a.x, b.y);
    let p2 = mul32(a.y, b.x);
    let p3 = mul32(a.y, b.y);
    let c1 = add64(add64(w(p0.y, 0u), w(p1.x, 0u)), w(p2.x, 0u));
    let c2 = add64(add64(add64(w(p1.y, 0u), w(p2.y, 0u)), w(p3.x, 0u)), w(c1.y, 0u));
    return vec4<u32>(p0.x, c1.x, c2.x, p3.y + c2.y);
}

// A 128-bit value shifted right by `n` < 128, its low 64 bits.
fn shr128(p: vec4<u32>, n: u32) -> vec2<u32> {
    let word = n / 32u;
    let bit = n % 32u;
    var v = array<u32, 6>(p.x, p.y, p.z, p.w, 0u, 0u);
    let a = v[word];
    let b = v[word + 1u];
    let c = v[word + 2u];
    if bit == 0u {
        return vec2<u32>(a, b);
    }
    return vec2<u32>((a >> bit) | (b << (32u - bit)), (b >> bit) | (c << (32u - bit)));
}

// Whether a 128-bit value shifted right by `n` reaches 2^62.
fn reaches62(p: vec4<u32>, n: u32) -> bool {
    // Bits at or above n + 62.
    let top = n + 62u;
    let word = top / 32u;
    let bit = top % 32u;
    var v = array<u32, 5>(p.x, p.y, p.z, p.w, 0u);
    if (v[word] >> bit) != 0u {
        return true;
    }
    for (var k = word + 1u; k < 4u; k++) {
        if v[k] != 0u {
            return true;
        }
    }
    return false;
}

fn add128(a: vec4<u32>, b: vec2<u32>) -> vec4<u32> {
    let x = add64(w(a.x, 0u), w(b.x, 0u));
    let y = add64(add64(w(a.y, 0u), w(b.y, 0u)), w(x.y, 0u));
    let z = add64(w(a.z, 0u), w(y.y, 0u));
    return vec4<u32>(x.x, y.x, z.x, a.w + z.y);
}

fn abs64(a: vec2<u32>) -> vec2<u32> {
    return select(a, neg64(a), negative(a));
}

// floor(a b / 2^n) of two signed values, as Rust's `(i128 * i128) >> n`
// followed by `as i64` (`saturate` false) or a clamp to ±2^62 (true).
fn smul_shr(a: vec2<u32>, b: vec2<u32>, n: u32, saturate: bool) -> vec2<u32> {
    let sign = negative(a) != negative(b);
    var p = mul128(abs64(a), abs64(b));
    if sign {
        // -ceil(|p| / 2^n) is floor(-|p| / 2^n).
        p = add128(p, sub64(shl64(w(1u, 0u), n), w(1u, 0u)));
    }
    var q = shr128(p, n);
    if saturate && reaches62(p, n) {
        q = w(0u, 0x40000000u);
    }
    return select(q, neg64(q), sign && !is_zero(q));
}

// Long division of two positive values with `num` < `den` < 2^63: the
// first `bits` binary digits of num / den, as an integer.
fn fraction(num: vec2<u32>, den: vec2<u32>, bits: u32) -> vec2<u32> {
    var r = num;
    var q = vec2<u32>(0u);
    for (var i = 0u; i < bits; i++) {
        r = shl64(r, 1u);
        q = shl64(q, 1u);
        if !lt64(r, den) {
            r = sub64(r, den);
            q.x |= 1u;
        }
    }
    return q;
}
