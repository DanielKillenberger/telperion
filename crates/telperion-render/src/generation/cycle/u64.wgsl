// Wide integers in 32-bit words (fn-209 F1): a u64 or two's complement
// i64 is a vec2<u32> (low, high). What stays exact under f32 growth
// (fn-254 R3): lineage keys and draws, ticks and fine time units, cycle
// and event indices.

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
