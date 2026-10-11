// fn-256 task 2: libm's `sincosf` (FreeBSD's) on the device. The CPU
// reduces and sums its polynomials in f64 and rounds once to f32; here
// the reals are integers of 2^-61 (`Q3.61`, two's complement in two
// words), so no driver can fuse or reorder them: the reduction is
// rounded to f64's 53 bits as the CPU's add rounds it, the polynomials
// are summed 2^-61 apart from the exact, and the result rounded once to
// f32. Within |x| <= 5 pi / 4, which the layout's angles never pass
// (`q::sincos` wraps to half a turn).

fn neg64(a: vec2<u32>) -> vec2<u32> {
    return sub64(vec2<u32>(0u), a);
}

fn abs64(a: vec2<u32>) -> vec2<u32> {
    return select(a, neg64(a), negative(a));
}

// `a b` of two Q3.61 reals within one, the product's magnitude cut to
// 2^-61 before its sign.
fn qmul64(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    let x = abs64(a);
    let y = abs64(b);
    let p0 = mul32(x.x, y.x);
    let p1 = mul32(x.x, y.y);
    let p2 = mul32(x.y, y.x);
    let p3 = mul32(x.y, y.y);
    // The 128-bit product's words 1 to 3.
    let mid = add64(vec2<u32>(p0.y, 0u), vec2<u32>(p1.x, 0u));
    let mid2 = add64(mid, vec2<u32>(p2.x, 0u));
    let hi = add64(add64(p3, vec2<u32>(p1.y, 0u)), add64(vec2<u32>(p2.y, 0u), vec2<u32>(mid2.y, 0u)));
    let r = vec2<u32>((mid2.x >> 29u) | (hi.x << 3u), (hi.x >> 29u) | (hi.y << 3u));
    return select(r, neg64(r), negative(a) != negative(b));
}

// A finite f32 of magnitude in [2^-12, 4) as a Q3.61 real, exactly.
fn q61(x: f32) -> vec2<u32> {
    let bits = bitcast<u32>(x);
    let mant = (bits & 0x7fffffu) | 0x800000u;
    let m = shl64(vec2<u32>(mant, 0u), ((bits >> 23u) & 0xffu) - 89u);
    return select(m, neg64(m), (bits >> 31u) != 0u);
}

// A Q3.61 real rounded to f64's 53 significant bits, to nearest, ties to
// even: the CPU's f64 add.
fn round53(a: vec2<u32>) -> vec2<u32> {
    let m = abs64(a);
    var top = 31u - countLeadingZeros(m.x);
    if m.y != 0u {
        top = 63u - countLeadingZeros(m.y);
    }
    if is_zero(m) || top <= 52u {
        return a;
    }
    let drop = top - 52u;
    let kept = shr64(m, drop);
    let rest = sub64(m, shl64(kept, drop));
    let half = shl64(vec2<u32>(1u, 0u), drop - 1u);
    var r = kept;
    if lt64(half, rest) || (all(rest == half) && (kept.x & 1u) == 1u) {
        r = add64(r, vec2<u32>(1u, 0u));
    }
    r = shl64(r, drop);
    return select(r, neg64(r), negative(a));
}

// A Q3.61 real rounded once to f32.
fn f61(a: vec2<u32>) -> f32 {
    let v = to_f32(abs64(a)) * pow2(-61);
    return select(v, -v, negative(a));
}

// `k_sinf`: sin on [-pi/4, pi/4].
fn k_sinf(y: vec2<u32>) -> f32 {
    let z = qmul64(y, y);
    let w2 = qmul64(z, z);
    let r = add64(w(0x61d35189u, 0xfffe5ff0u), qmul64(z, w(0x61e30ed2u, 0x000005b3u)));
    let s = qmul64(z, y);
    let inner = add64(w(0xcd14e240u, 0xfaaaaaaau), qmul64(z, w(0x25bbeec8u, 0x00444442u)));
    return f61(add64(add64(y, qmul64(s, inner)), qmul64(qmul64(s, w2), r)));
}

// `k_cosf`: cos on [-pi/4, pi/4].
fn k_cosf(y: vec2<u32>) -> f32 {
    let z = qmul64(y, y);
    let w2 = qmul64(z, z);
    let r = add64(w(0x0bf870ecu, 0xfff49fbcu), qmul64(z, w(0x85c1dca1u, 0x00003326u)));
    let one = add64(w(0u, 0x20000000u), qmul64(z, w(0x79d0bf80u, 0xf0000001u)));
    let v = add64(add64(one, qmul64(w2, w(0x1053a420u, 0x0155553eu))), qmul64(qmul64(w2, z), r));
    return f61(v);
}

// The sine and cosine of `x` radians, as `sincosf` takes them.
fn sincosf(x: f32) -> vec2<f32> {
    let bits = bitcast<u32>(x);
    let ix = bits & 0x7fffffffu;
    if ix < 0x39800000u {
        return vec2<f32>(x, 1.0);
    }
    let y = q61(x);
    if ix <= 0x3f490fdau {
        return vec2<f32>(k_sinf(y), k_cosf(y));
    }
    let negative_x = (bits >> 31u) != 0u;
    let pio2 = w(0x885a3000u, 0x3243f6a8u);
    let pi = w(0x10b46000u, 0x6487ed51u);
    if ix <= 0x4016cbe3u {
        if negative_x {
            let r = round53(add64(y, pio2));
            return vec2<f32>(-k_cosf(r), k_sinf(r));
        }
        let r = round53(sub64(pio2, y));
        return vec2<f32>(k_cosf(r), k_sinf(r));
    }
    let r = round53(select(sub64(y, pi), add64(y, pi), negative_x));
    return vec2<f32>(-k_sinf(r), -k_cosf(r));
}
