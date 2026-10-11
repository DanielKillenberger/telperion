// The light's f64 programs (`light_program.rs`) on a file of registers:
// each instruction one software operation, so each operation has one call
// site an entry, here, and the driver inlines each once (fn-235). The
// module that includes this defines `hook`, the integer steps between;
// `exp`'s own are here.

var<private> R: array<vec2<u32>, REGISTERS>;
// `exp`'s scale and its case: 0 the reduction, 1 below 2^-28, 2 past
// the least subnormal, 3 past the largest f64.
var<private> exp_k: i32;
var<private> exp_case: u32;

fn load_constants() {
    for (var i = 0u; i < 11u; i++) {
        R[i] = CONSTANTS[i];
    }
}

fn arith(code: u32, a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    switch code {
        case 2u: {
            return f64_mul(a, b);
        }
        case 3u: {
            return f64_div(a, b);
        }
        case 4u: {
            return f64_from_f32(f64_to_f32(a));
        }
        default: {
            return f64_add(a, select(b, f64_neg(b), code == 1u));
        }
    }
}

fn exec(word: u32) {
    let code = word & 0xffu;
    if code >= 8u {
        hook(code);
        return;
    }
    R[(word >> 8u) & 0xffu] = arith(code, R[(word >> 16u) & 0xffu], R[word >> 24u]);
}

// libm's `exp`, its integer steps: the half its rounding adds, its case
// and k, and the factors `scalbn` takes.
fn exp_hook(code: u32) {
    let x = R[R_XN];
    let negative = (x.y & SIGN) != 0u;
    switch code {
        case H_SIGN: {
            R[R_HALFS] = vec2<u32>(0u, select(0x3fe00000u, 0xbfe00000u, negative));
        }
        case H_K: {
            exp_k_of(x, negative);
        }
        default: {
            exp_scale();
        }
    }
}

fn exp_k_of(x: vec2<u32>, negative: bool) {
    let hx = x.y & 0x7fffffffu;
    exp_case = 0u;
    if hx >= 0x4086232bu {
        if !negative && u64_lt(EXP_OVER, magnitude(x)) {
            exp_case = 3u;
        }
        if negative && u64_lt(EXP_ZERO, magnitude(x)) {
            exp_case = 2u;
        }
    }
    var k = 0;
    if hx > 0x3fd62e42u {
        if hx >= 0x3ff0a2b2u {
            k = f64_to_i32(R[R_KT]);
        } else {
            k = select(1, -1, negative);
        }
    } else if hx <= 0x3e300000u {
        exp_case = 1u;
    }
    exp_k = k;
    R[R_KF] = f64_from_i32(k);
}

fn exp_scale() {
    R[R_S1] = R[R_ONE];
    R[R_S2] = R[R_ONE];
    R[R_S3] = R[R_ONE];
    R[R_E] = R[R_YY];
    switch exp_case {
        case 1u: {
            R[R_E] = R[R_TINY];
        }
        case 2u: {
            R[R_S1] = vec2<u32>(0u);
        }
        case 3u: {
            R[R_E] = R[R_XN];
            R[R_S1] = TWO_1023;
        }
        default: {
            scalbn_factors(exp_k);
        }
    }
}

// `scalbn`'s prescales into S1 and S2, its last scale into S3.
fn scalbn_factors(k: i32) {
    var n = k;
    if n > 1023 {
        R[R_S1] = TWO_1023;
        n -= 1023;
        if n > 1023 {
            R[R_S2] = TWO_1023;
            n = min(n - 1023, 1023);
        }
    } else if n < -1022 {
        R[R_S1] = TWO_M969;
        n += 969;
        if n < -1022 {
            R[R_S2] = TWO_M969;
            n = max(n + 969, -1022);
        }
    }
    R[R_S3] = vec2<u32>(0u, u32(1023 + n) << 20u);
}
