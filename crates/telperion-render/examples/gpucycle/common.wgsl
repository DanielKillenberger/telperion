// The spike's prelude, after the contract (wide words, lineage keys,
// arithmetic): the dispatch's parameters and the fixed-point sums.
struct Params {
    a: array<vec4<u32>, 2>,
    f: array<vec4<f32>, 2>,
}

@group(0) @binding(0) var<uniform> P: Params;

const NONE: u32 = 0xffffffffu;
const WG: u32 = 256u;

// `x` (non-negative, finite) in units of 2^-32, half away from zero: its
// mantissa shifted by its exponent, as `tree::fixed` takes it.
fn fixed(x: f32) -> vec2<u32> {
    let b = bitcast<u32>(x);
    let e = i32((b >> 23u) & 0xffu);
    if (b & 0x7fffffffu) == 0u {
        return vec2<u32>(0u);
    }
    var m = b & 0x7fffffu;
    var ex = -126;
    if e != 0 {
        m |= 0x800000u;
        ex = e - 127;
    }
    let sh = ex + 9;
    if sh >= 0 {
        return shl64(vec2<u32>(m, 0u), u32(sh));
    }
    let k = u32(-sh);
    if k > 25u {
        return vec2<u32>(0u);
    }
    let q = m >> k;
    let half = (m >> (k - 1u)) & 1u;
    return vec2<u32>(q + half, 0u);
}

// A fixed-point sum back to a real.
fn unfixed(lo: u32, hi: u32) -> f32 {
    return (f32(hi) * 4294967296.0 + f32(lo)) * 2.3283064e-10;
}
