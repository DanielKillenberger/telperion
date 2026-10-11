// fn-256 task 2 (SETTLE-PLAN.md S5): libm's `asinf` and `hypotf` as
// sag's turn takes them (`sag::turn`), in their f32 operations, their
// f64 steps in the software f64 and `asinf`'s f64 root correctly
// rounded in integers.

const PIO2_64 = vec2<u32>(0x54442d18u, 0x3ff921fbu);

// The f64 of `sqrt(z)` for a positive normal f32 `z`, to nearest, ties
// to even: the integer root of its significand scaled by 2^84.
fn sqrt64_of(z: f32) -> vec2<u32> {
    let b = bitcast<u32>(z);
    var mant = (b & 0x7fffffu) | 0x800000u;
    var e = i32(b >> 23u) - 150;
    if (e & 1) != 0 {
        mant <<= 1u;
        e -= 1;
    }
    // The radicand `mant 2^84` in four words, high first, and its root
    // bit by bit, the remainder kept.
    var x = array<u32, 4>(mant >> 12u, mant << 20u, 0u, 0u);
    var root = vec2<u32>(0u);
    var rem = array<u32, 4>(0u, 0u, 0u, 0u);
    // cap: the radicand's 128 bits, two a step.
    for (var k = 0u; k < 64u; k++) {
        // rem = rem * 4 + the next two bits of x.
        let top = x[0] >> 30u;
        for (var w = 0u; w < 3u; w++) {
            x[w] = (x[w] << 2u) | (x[w + 1u] >> 30u);
            rem[w] = (rem[w] << 2u) | (rem[w + 1u] >> 30u);
        }
        x[3] = x[3] << 2u;
        rem[3] = (rem[3] << 2u) | top;
        // trial = root * 4 + 1, against rem.
        let r4 = shl64(root, 2u);
        let trial = array<u32, 4>(0u, root.y >> 30u, r4.y, r4.x | 1u);
        var ge = true;
        for (var w = 0u; w < 4u; w++) {
            if rem[w] != trial[w] {
                ge = rem[w] > trial[w];
                break;
            }
        }
        root = shl64(root, 1u);
        if ge {
            var borrow = 0u;
            for (var w = 4u; w > 0u; w--) {
                let a = rem[w - 1u];
                let d = a - trial[w - 1u] - borrow;
                borrow = u32(a < trial[w - 1u] || (a == trial[w - 1u] && borrow == 1u));
                rem[w - 1u] = d;
            }
            root.x |= 1u;
        }
    }
    // root = floor(sqrt(mant 2^84)) in [2^54, 2^55): keep 53 bits.
    let extra = select(1u, 2u, (root.y >> 22u) != 0u);
    let kept = shr64(root, extra);
    let dropped = root.x & ((1u << extra) - 1u);
    let half = 1u << (extra - 1u);
    let sticky = (dropped & (half - 1u)) != 0u || rem[0] != 0u || rem[1] != 0u || rem[2] != 0u || rem[3] != 0u;
    var q = kept;
    if (dropped & half) != 0u && (sticky || (kept.x & 1u) == 1u) {
        q = add64(q, w(1u, 0u));
    }
    // The value is q 2^(e/2 - 42 + extra); q's top bit at 52 (or 53).
    var ex = e / 2 - 42 + i32(extra) + 52 + 1023;
    if (q.y >> 21u) != 0u {
        q = shr64(q, 1u);
        ex += 1;
    }
    return vec2<u32>(q.x, (u32(ex) << 20u) | (q.y & 0xfffffu));
}

// libm's `asinf` (FreeBSD's): its R(z) in f32, its last steps in f64.
fn asin_r(z: f32) -> f32 {
    let p = m(z, 0x1.5554eap-3f + m(z, -0x1.5e2774p-5f + m(z, -0x1.1ba6d6p-7f)));
    let q = 1.0 + m(z, -0x1.69cb5cp-1f);
    return exact_div(p, q);
}

fn asinf(x: f32) -> f32 {
    let hx = bitcast<u32>(x);
    let ix = hx & 0x7fffffffu;
    if ix >= 0x3f800000u {
        // |x| = 1: pi/2 as the f32 it rounds to; |x| > 1 is never asked.
        return select(0x1.921fb6p+0f, -0x1.921fb6p+0f, (hx >> 31u) != 0u);
    }
    if ix < 0x3f000000u {
        if ix < 0x39800000u && ix >= 0x00800000u {
            return x;
        }
        return x + m(x, asin_r(m(x, x)));
    }
    let z = (1.0 - abs(x)) * 0.5;
    let s = sqrt64_of(z);
    let inner = f64_add(s, f64_mul(s, f64_from_f32(asin_r(z))));
    let r = f64_to_f32(f64_add(PIO2_64, f64_neg(f64_add(inner, inner))));
    return select(r, -r, (hx >> 31u) != 0u);
}

// libm's `hypotf`: the squares summed in f64, rounded to f32, its root.
fn hypotf(x0: f32, y0: f32) -> f32 {
    var ux = bitcast<u32>(x0) & 0x7fffffffu;
    var uy = bitcast<u32>(y0) & 0x7fffffffu;
    if ux < uy {
        let t = ux;
        ux = uy;
        uy = t;
    }
    var x = bitcast<f32>(ux);
    var y = bitcast<f32>(uy);
    if uy == (0xffu << 23u) {
        return y;
    }
    if ux >= (0xffu << 23u) || uy == 0u || ux - uy >= (25u << 23u) {
        return x + y;
    }
    var z = 1.0;
    if ux >= ((0x7fu + 60u) << 23u) {
        z = 0x1p90f;
        x *= 0x1p-90f;
        y *= 0x1p-90f;
    } else if uy < ((0x7fu - 60u) << 23u) {
        z = 0x1p-90f;
        x *= 0x1p90f;
        y *= 0x1p90f;
    }
    let xx = f64_mul(f64_from_f32(x), f64_from_f32(x));
    let yy = f64_mul(f64_from_f32(y), f64_from_f32(y));
    return z * qsqrt(f64_to_f32(f64_add(xx, yy)));
}

