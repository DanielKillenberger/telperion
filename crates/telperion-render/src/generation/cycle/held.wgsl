// fn-256 task 2: the contract's operations whose results become integers
// (`held.rs`, DESIGN.md section 8), held to the CPU's order in the plain
// build. A product kept apart from the sum it feeds by its bits XORed
// with a uniform zero the driver cannot see through (`exact_zero`, the
// units' own), and a quotient
// rounded to nearest, ties to even, in integers: the driver fuses
// multiplies into adds and does not round WGSL's division correctly
// (UNITS.md, LAYOUT.md, HIERARCHY.md, LIGHT.md).
fn held_kept(x: f32) -> f32 {
    return bitcast<f32>(bitcast<u32>(x) ^ exact_zero());
}

// IEEE division of normal values, to nearest, ties to even, its
// quotient's underflow to the subnormals as IEEE takes it.
fn held_div(a: f32, b: f32) -> f32 {
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
    if e <= 0 {
        return bitcast<f32>(sign | below_normal(q, 2 - e, r != 0u));
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

// A quotient's 25 bits `q` below the least normal: its subnormal bits,
// `q` shifted right `shift` places to nearest, ties to even, `rest`
// whether anything below `q` remained (a rounding to the least normal
// carries into its exponent bit).
fn below_normal(q: u32, shift: i32, rest: bool) -> u32 {
    if shift >= 26 {
        return 0u;
    }
    let s = u32(shift);
    let half = 1u << (s - 1u);
    let dropped = q & ((1u << s) - 1u);
    var m = q >> s;
    if (dropped & half) != 0u && ((dropped & (half - 1u)) != 0u || rest || (m & 1u) == 1u) {
        m += 1u;
    }
    return m;
}
