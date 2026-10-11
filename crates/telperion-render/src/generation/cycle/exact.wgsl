// fn-256 task 2: the marked arithmetic held to the CPU's order, built
// only where a test asks for it (`Arith::Exact`). A product kept apart
// from the sum it feeds by its bits XORed with a uniform zero the driver
// cannot see through (`exact_zero`, each module's own), and a quotient
// rounded to nearest, ties to even, in integers: the driver fuses
// multiplies into adds and does not round WGSL's division correctly
// (UNITS.md, LAYOUT.md, HIERARCHY.md, LIGHT.md).
fn exact_kept(x: f32) -> f32 {
    return bitcast<f32>(bitcast<u32>(x) ^ exact_zero());
}

// IEEE division of normal values, to nearest, ties to even.
fn exact_div(a: f32, b: f32) -> f32 {
    if a == 0.0 {
        return a / b;
    }
    let sign = (bitcast<u32>(a) ^ bitcast<u32>(b)) & 0x80000000u;
    let ua = bitcast<u32>(a) & 0x7fffffffu;
    let ub = bitcast<u32>(b) & 0x7fffffffu;
    var e = i32(ua >> 23u) - i32(ub >> 23u) + 127;
    var ma = (ua & 0x7fffffu) | 0x800000u;
    let mb = (ub & 0x7fffffu) | 0x800000u;
    if ma < mb {
        ma <<= 1u;
        e -= 1;
    }
    var r = ma;
    var q = 0u;
    for (var i = 0u; i < 25u; i++) {
        q <<= 1u;
        if r >= mb {
            r -= mb;
            q |= 1u;
        }
        r <<= 1u;
    }
    let guard = q & 1u;
    q >>= 1u;
    if guard == 1u && (r != 0u || (q & 1u) == 1u) {
        q += 1u;
    }
    if q == 0x1000000u {
        q >>= 1u;
        e += 1;
    }
    return bitcast<f32>(sign | (u32(e) << 23u) | (q & 0x7fffffu));
}
