// fn-256 task 2: f32 products, sums and roots that keep IEEE's gradual
// underflow, in integers, built where a test holds the device to the CPU
// word for word (`Arith::Exact`). The device flushes every subnormal f32
// to zero, as an operand and as a result; the CPU keeps them, and sag's
// levers at an axis's fine tips square into them (SETTLE.md). Each rounds
// to nearest, ties to even; finite operands only.

// A finite nonzero f32's significand (its leading one at bit 23) and the
// exponent of its last bit: `x = m 2^e`.
fn sub_unpack(u: u32) -> vec2<i32> {
    let ex = (u >> 23u) & 0xffu;
    var m = u & 0x7fffffu;
    var e = -149;
    if ex != 0u {
        m |= 0x800000u;
        e = i32(ex) - 150;
    } else {
        let lead = 31u - countLeadingZeros(m);
        m <<= 23u - lead;
        e -= i32(23u - lead);
    }
    return vec2<i32>(i32(m), e);
}

// `p 2^k` of a positive `p` below 2^64 rounded to an f32, `sticky`
// whether anything below `p` was dropped, `sign` its sign bit.
fn sub_round(p: vec2<u32>, k: i32, sticky: bool, sign: u32) -> f32 {
    var lead = 31u - countLeadingZeros(p.x);
    if p.y != 0u {
        lead = 63u - countLeadingZeros(p.y);
    }
    let top = i32(lead) + k;
    var drop = i32(lead) - 23;
    if top < -126 {
        drop = -149 - k;
    }
    if drop >= 64 {
        return bitcast<f32>(sign);
    }
    var m = vec2<u32>(p);
    var rest = false;
    var half = false;
    if drop > 0 {
        let d = u32(drop);
        let kept = shr64(p, d);
        let low = sub64(p, shl64(kept, d));
        let h = shl64(w(1u, 0u), d - 1u);
        half = !lt64(low, h);
        rest = !is_zero(sub64(low, select(w(0u, 0u), h, half))) || sticky;
        m = kept;
    } else {
        m = shl64(p, u32(-drop));
    }
    var mant = m.x;
    if half && (rest || (mant & 1u) == 1u) {
        mant += 1u;
    }
    if top < -126 {
        // A subnormal, or rounded up to the least normal: its bits are its
        // significand's.
        return bitcast<f32>(sign | mant);
    }
    var e = top;
    if mant == 0x1000000u {
        mant >>= 1u;
        e += 1;
    }
    return bitcast<f32>(sign | (u32(e + 127) << 23u) | (mant & 0x7fffffu));
}

fn sub_mul(a: f32, b: f32) -> f32 {
    let ua = bitcast<u32>(a);
    let ub = bitcast<u32>(b);
    let sign = (ua ^ ub) & 0x80000000u;
    if (ua & 0x7fffffffu) == 0u || (ub & 0x7fffffffu) == 0u {
        return bitcast<f32>(sign);
    }
    let x = sub_unpack(ua & 0x7fffffffu);
    let y = sub_unpack(ub & 0x7fffffffu);
    return sub_round(mul32(u32(x.x), u32(y.x)), x.y + y.y, false, sign);
}

fn sub_add(a: f32, b: f32) -> f32 {
    var ua = bitcast<u32>(a);
    var ub = bitcast<u32>(b);
    if (ua & 0x7fffffffu) == 0u && (ub & 0x7fffffffu) == 0u {
        return bitcast<f32>(ua & ub & 0x80000000u);
    }
    if (ub & 0x7fffffffu) == 0u {
        return a;
    }
    if (ua & 0x7fffffffu) == 0u {
        return b;
    }
    if (ua & 0x7fffffffu) < (ub & 0x7fffffffu) {
        let t = ua;
        ua = ub;
        ub = t;
    }
    let x = sub_unpack(ua & 0x7fffffffu);
    let y = sub_unpack(ub & 0x7fffffffu);
    // Both significands 30 places up, the smaller one aligned below the
    // larger's, what falls off kept as a sticky bit.
    let big = shl64(w(u32(x.x), 0u), 30u);
    let d = u32(x.y - y.y);
    var small = vec2<u32>(0u);
    var sticky = true;
    if d < 54u {
        let s = shl64(w(u32(y.x), 0u), 30u);
        small = shr64(s, d);
        sticky = !is_zero(sub64(s, shl64(small, d)));
    }
    let sign = ua & 0x80000000u;
    if ((ua ^ ub) & 0x80000000u) == 0u {
        return sub_round(add64(big, small), x.y - 30, sticky, sign);
    }
    var diff = sub64(big, small);
    if sticky {
        // The bits below `small` were taken too: borrow one, and what is
        // left below is still nonzero.
        diff = sub64(diff, w(1u, 0u));
    }
    if is_zero(diff) && !sticky {
        return 0.0;
    }
    return sub_round(diff, x.y - 30, sticky, sign);
}

// The root of a nonnegative f32: a subnormal's taken as the normal it is
// times 2^64, its root then times 2^-32.
fn sub_sqrt(x: f32) -> f32 {
    let u = bitcast<u32>(x);
    if (u >> 23u) != 0u || u == 0u {
        return exact_sqrt(x);
    }
    let lead = 31u - countLeadingZeros(u);
    let normal = ((lead + 42u) << 23u) | ((u << (23u - lead)) & 0x7fffffu);
    return exact_sqrt(bitcast<f32>(normal)) * 0x1p-32f;
}
