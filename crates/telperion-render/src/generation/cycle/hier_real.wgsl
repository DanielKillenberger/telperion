// fn-256 task 2 (DESIGN.md section 8): the allotment's f64 products by
// their bits. A presence is `base_scale × running × kept` and its lit
// share that times the light, each an f64 multiply on the CPU; here the
// 106-bit product of the significands is formed in words and rounded to
// 53 bits, to nearest, ties to even, as the CPU's multiply rounds it.

// The one f64 the units' reals (`real.wgsl`) take as a constant.
const ONE64 = vec2<u32>(0u, 0x3ff00000u);

// `a * b` of two non-negative finite f64s whose product is a normal
// number (a presence's and a light's range) or none.
fn mul_real(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    if is_zero(a) || is_zero(b) {
        return vec2<u32>(0u);
    }
    let ea = i32((a.y >> 20u) & 0x7ffu);
    let eb = i32((b.y >> 20u) & 0x7ffu);
    let ah = (a.y & 0xfffffu) | 0x100000u;
    let bh = (b.y & 0xfffffu) | 0x100000u;
    // The product's four words, low first.
    let p0 = mul32(a.x, b.x);
    let p1 = mul32(a.x, bh);
    let p2 = mul32(ah, b.x);
    let p3 = mul32(ah, bh);
    var w1 = p0.y + p1.x;
    var c2 = u32(w1 < p1.x);
    w1 += p2.x;
    c2 += u32(w1 < p2.x);
    var w2 = p1.y + p2.y;
    var c3 = u32(w2 < p2.y);
    w2 += p3.x;
    c3 += u32(w2 < p3.x);
    w2 += c2;
    c3 += u32(w2 < c2);
    let w3 = p3.y + c3;
    // The product lies in [2^104, 2^106): keep its top 53 bits.
    let high = (w3 >> 9u) != 0u;
    let s = select(20u, 21u, high);
    var lo = (w1 >> s) | (w2 << (32u - s));
    var hi = (w2 >> s) | (w3 << (32u - s));
    let half = (w1 >> (s - 1u)) & 1u;
    let rest = (w1 & ((1u << (s - 1u)) - 1u)) != 0u || p0.x != 0u;
    var e = ea + eb - 1023 + i32(high);
    if half == 1u && (rest || (lo & 1u) == 1u) {
        lo += 1u;
        hi += u32(lo == 0u);
        if hi == 0x200000u {
            hi = 0x100000u;
            e += 1;
        }
    }
    return vec2<u32>(lo, (u32(e) << 20u) | (hi & 0xfffffu));
}
