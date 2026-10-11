// fn-256 task 2: the layout's square root held to IEEE's, to nearest, of
// a normal value, in integers (`Arith::Exact`): WGSL's `sqrt` is not
// correctly rounded on the driver (LAYOUT.md). Needs the wide words.
fn exact_sqrt(x: f32) -> f32 {
    if !(x > 0.0) {
        return sqrt(x);
    }
    let b = bitcast<u32>(x);
    var s = i32(b >> 23u) - 150;
    let mant = (b & 0x7fffffu) | 0x800000u;
    var big = vec2<u32>(mant, 0u);
    if (s & 1) != 0 {
        big = shl64(big, 1u);
        s -= 1;
    }
    var rest = shl64(big, 26u);
    s -= 26;
    var res = vec2<u32>(0u);
    var bit = shl64(vec2<u32>(1u, 0u), 50u);
    // cap: 26 bit pairs.
    while lt64(rest, bit) {
        bit = shr64(bit, 2u);
    }
    // cap: 26 bit pairs.
    while !is_zero(bit) {
        let t = add64(res, bit);
        if !lt64(rest, t) {
            rest = sub64(rest, t);
            res = add64(shr64(res, 1u), bit);
        } else {
            res = shr64(res, 1u);
        }
        bit = shr64(bit, 2u);
    }
    var top = 31u - countLeadingZeros(res.x);
    if res.y != 0u {
        top = 63u - countLeadingZeros(res.y);
    }
    let d = top - 23u;
    var kept_bits = shr64(res, d);
    let low = sub64(res, shl64(kept_bits, d)).x;
    let half = 1u << (d - 1u);
    if low > half || (low == half && (!is_zero(rest) || (kept_bits.x & 1u) == 1u)) {
        kept_bits = add64(kept_bits, vec2<u32>(1u, 0u));
    }
    return f32(kept_bits.x) * pow2(i32(d) + s / 2);
}
