// The decision contract on the GPU (fn-209 F1, FINE-CONTRACT.md, host
// decision 1): `telperion_space::fixed` and `fixed::decide`'s integer
// forms, word for word, over `wide.wgsl`. A log is a Q32 i64 saturated
// at ±2^62 (`INF`), a probability a Q62 u64, time ticks of 2^-20 cycle.

const INF_HI: u32 = 0x40000000u;

fn inf() -> vec2<u32> {
    return w(0u, INF_HI);
}

fn p_one() -> vec2<u32> {
    return w(0u, INF_HI);
}

fn one32() -> vec2<u32> {
    return w(0u, 1u);
}

fn is_inf(l: vec2<u32>) -> bool {
    return !slt64(l, inf());
}

// `2^-(2^-(i + 1))` in Q62, floored (`fixed::ROOTS`).
var<private> ROOTS: array<vec2<u32>, 32> = array<vec2<u32>, 32>(
    vec2<u32>(0xfe779921u, 0x2d413cccu), vec2<u32>(0xb5a75abdu, 0x35d13f32u),
    vec2<u32>(0xf7490e4bu, 0x3ab031b9u), vec2<u32>(0x4921b30bu, 0x3d495f45u),
    vec2<u32>(0xdc8a80ceu, 0x3ea0ecb6u), vec2<u32>(0x3d21b05du, 0x3f4f8303u),
    vec2<u32>(0x1ee3e212u, 0x3fa78457u), vec2<u32>(0x5447b229u, 0x3fd3b2d6u),
    vec2<u32>(0x87236bb4u, 0x3fe9d594u), vec2<u32>(0x703df842u, 0x3ff4e9d4u),
    vec2<u32>(0xbdf6c9d5u, 0x3ffa74acu), vec2<u32>(0xffc6e30fu, 0x3ffd3a46u),
    vec2<u32>(0xa8010199u, 0x3ffe9d1fu), vec2<u32>(0xde053ad4u, 0x3fff4e8eu),
    vec2<u32>(0x318376acu, 0x3fffa747u), vec2<u32>(0x8961e6feu, 0x3fffd3a3u),
    vec2<u32>(0xc0d8fd14u, 0x3fffe9d1u), vec2<u32>(0xdf7680c4u, 0x3ffff4e8u),
    vec2<u32>(0x6f7dc0ebu, 0x3ffffa74u), vec2<u32>(0x37af8097u, 0x3ffffd3au),
    vec2<u32>(0x1bd3e854u, 0x3ffffe9du), vec2<u32>(0x8de8fe2cu, 0x3fffff4eu),
    vec2<u32>(0x46f44196u, 0x3fffffa7u), vec2<u32>(0xa37a116bu, 0x3fffffd3u),
    vec2<u32>(0xd1bd04ddu, 0x3fffffe9u), vec2<u32>(0xe8de8178u, 0x3ffffff4u),
    vec2<u32>(0x746f407eu, 0x3ffffffau), vec2<u32>(0x3a37a030u, 0x3ffffffdu),
    vec2<u32>(0x9d1bd014u, 0x3ffffffeu), vec2<u32>(0x4e8de809u, 0x3fffffffu),
    vec2<u32>(0xa746f404u, 0x3fffffffu), vec2<u32>(0xd3a37a02u, 0x3fffffffu),
);

const LN2 = vec2<u32>(0xf473de6au, 0x2c5c85fdu);
const LOG2E = vec2<u32>(0xae0bf85du, 0x5c551d94u);

// The lineage's hash (`lineage.rs`, D1): SplitMix64's finalizer.
fn mix(key: vec2<u32>) -> vec2<u32> {
    var z = key;
    z = mul64(xor64(z, shr64(z, 30u)), vec2<u32>(0x1ce4e5b9u, 0xbf58476du));
    z = mul64(xor64(z, shr64(z, 27u)), vec2<u32>(0x133111ebu, 0x94d049bbu));
    return xor64(z, shr64(z, 31u));
}

fn child(key: vec2<u32>, part: vec2<u32>) -> vec2<u32> {
    let step = mix(add64(part, vec2<u32>(0xf3bcc909u, 0x6a09e667u)));
    return mix(xor64(rotl64(key, 23u), step));
}

// A key's 53-bit draw.
fn draw(key: vec2<u32>) -> vec2<u32> {
    return shr64(mix(xor64(key, vec2<u32>(0xd192ed03u, 0xd1b54a32u))), 11u);
}

// The internode draw of the node keyed `key` (`fixed::internode_draw`,
// fn-222): under `lineage::INTERNODE`, u64::MAX - 9.
fn internode_draw(key: vec2<u32>) -> vec2<u32> {
    return draw(child(key, vec2<u32>(0xfffffff6u, 0xffffffffu)));
}

// A Q62 probability's threshold against a 53-bit draw.
fn threshold(p: vec2<u32>) -> vec2<u32> {
    return shr64(add64(min64(p, p_one()), w(511u, 0u)), 9u);
}

fn mul62(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    return shr128(mul128(a, b), 62u);
}

// log2 n in Q32; -INF for none.
fn log2q(n: vec2<u32>) -> vec2<u32> {
    if is_zero(n) {
        return neg64(inf());
    }
    let e = 63u - clz64(n);
    var m = select(shr64(n, 1u), shl64(n, 62u - e), e <= 62u);
    var frac = 0u;
    for (var i = 0u; i < 32u; i++) {
        let square = mul62(m, m);
        frac <<= 1u;
        if (square.y & 0x80000000u) != 0u {
            frac |= 1u;
            m = shr64(square, 1u);
        } else {
            m = square;
        }
    }
    return w(frac, e);
}

// 2^-x for x in Q32, a Q62 probability.
fn exp2_neg(x: vec2<u32>) -> vec2<u32> {
    if negative(x) || is_zero(x) {
        return p_one();
    }
    let whole = x.y;
    if whole >= 63u {
        return vec2<u32>(0u);
    }
    var r = p_one();
    for (var i = 0u; i < 32u; i++) {
        if (x.x & (1u << (31u - i))) != 0u {
            r = mul62(r, ROOTS[i]);
        }
    }
    return shr64(r, whole);
}

// 2^y for y in Q32, in Q32, saturated.
fn exp2q(y: vec2<u32>) -> vec2<u32> {
    if negative(y) || is_zero(y) {
        return shr64(exp2_neg(neg64(y)), 30u);
    }
    let n = y.y + 1u;
    if n >= 62u {
        return inf();
    }
    let below = exp2_neg(w(0u - y.x, select(0u, 1u, y.x == 0u)));
    if n >= 30u {
        let s = n - 30u;
        if s > 0u && !lt64(below, shl64(w(1u, 0u), 62u - s)) {
            return inf();
        }
        return shl64(below, s);
    }
    return shr64(below, 30u - n);
}

fn neg_log2(p: vec2<u32>) -> vec2<u32> {
    if is_zero(p) {
        return inf();
    }
    return sub64(w(0u, 62u), log2q(min64(p, p_one())));
}

fn prob(l: vec2<u32>) -> vec2<u32> {
    if is_inf(l) {
        return vec2<u32>(0u);
    }
    return exp2_neg(l);
}

fn ln_ratio(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    if is_zero(a) {
        return neg64(inf());
    }
    if is_zero(b) {
        return inf();
    }
    return smul_shr(sub64(log2q(a), log2q(b)), LN2, 62u, false);
}

fn expq(x: vec2<u32>) -> vec2<u32> {
    if !slt64(neg64(inf()), x) {
        return vec2<u32>(0u);
    }
    return exp2q(smul_shr(x, LOG2E, 62u, false));
}

fn mulq(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    return smul_shr(a, b, 32u, true);
}

fn scaled(l: vec2<u32>, share: vec2<u32>) -> vec2<u32> {
    if is_inf(l) {
        return inf();
    }
    return smin64(smul_shr(l, share, 20u, true), inf());
}

fn presence(lead: vec2<u32>, window: vec2<u32>) -> vec2<u32> {
    if is_inf(lead) || !slt64(vec2<u32>(0u), window) {
        return one32();
    }
    if !slt64(vec2<u32>(0u), lead) {
        return vec2<u32>(0u);
    }
    if !lt64(lead, window) {
        return one32();
    }
    return fraction(lead, window, 32u);
}

fn logit_draw(d: vec2<u32>) -> vec2<u32> {
    return ln_ratio(d, sub64(w(0u, 0x200000u), d));
}

fn logit(p: vec2<u32>) -> vec2<u32> {
    return ln_ratio(p, sub64(p_one(), min64(p, p_one())));
}

// D5.
fn survival_q(l: vec2<u32>, share: vec2<u32>) -> vec2<u32> {
    return threshold(prob(scaled(l, share)));
}

// D6; `rises` false where the hazard does not rise.
fn abortion_q(l0: vec2<u32>, rises: bool, rise: vec2<u32>, grown: vec2<u32>, exposure: vec2<u32>) -> vec2<u32> {
    var l = l0;
    if rises && !is_inf(l) {
        if is_zero(grown) {
            l = select(inf(), vec2<u32>(0u), slt64(vec2<u32>(0u), rise));
        } else {
            let lt = sub64(log2q(grown), w(0u, 20u));
            l = mulq(l, exp2q(mulq(rise, lt)));
        }
    }
    return sub64(w(0u, 0x200000u), shr64(prob(scaled(l, exposure)), 9u));
}

// D10.
fn moved_q(go: vec2<u32>, d: vec2<u32>, a: vec2<u32>, b: vec2<u32>, used: vec2<u32>, rate: vec2<u32>, lo: vec2<u32>, hi: vec2<u32>) -> vec2<u32> {
    if !lt64(go, p_one()) || is_zero(d) {
        return one32();
    }
    let lead = sub64(logit(go), logit_draw(d));
    var wood: vec2<u32>;
    if !slt64(neg64(inf()), a) || !slt64(neg64(inf()), b) {
        let ea = expq(a);
        wood = add64(ea, smul_shr(sub64(expq(b), ea), used, 20u, false));
    } else {
        wood = expq(add64(a, smul_shr(sub64(b, a), used, 20u, false)));
    }
    let window = smin64(smax64(mulq(rate, wood), lo), hi);
    return presence(lead, window);
}

// Host decision 8 (`fixed/presence.rs`): a draw's lead in log-odds under
// the 53-bit threshold `t` it had to stay under, and over the one it had
// to reach; unbounded where the element is certain.
fn under_q(t: vec2<u32>, d: vec2<u32>) -> vec2<u32> {
    if !lt64(t, w(0u, 0x200000u)) || is_zero(d) {
        return inf();
    }
    return sub64(logit(shl64(t, 9u)), logit_draw(d));
}

fn over_q(t: vec2<u32>, d: vec2<u32>) -> vec2<u32> {
    if is_zero(t) {
        return inf();
    }
    return sub64(logit_draw(d), logit(shl64(t, 9u)));
}

// The lesser of `over_q(low, d)` and `under_q(high, d)` (task 8;
// REALIZED.md's options a and b): the draw's log-odds taken once where
// both bounds hold, and a bound's own, `logit(bound << 9)`, read from
// `given` where `known` says a table gave it (bit 0 the low bound's, bit
// 1 the high's): the same words, a quarter of the `log2` calls.
fn lead_q(low: vec2<u32>, high: vec2<u32>, d: vec2<u32>, given: vec4<u32>, known: u32) -> vec2<u32> {
    let over = !is_zero(low);
    let under = lt64(high, w(0u, 0x200000u)) && !is_zero(d);
    if !over && !under {
        return inf();
    }
    let drawn = logit_draw(d);
    var lead = inf();
    if over {
        var own = given.xy;
        if (known & 1u) == 0u {
            own = logit(shl64(low, 9u));
        }
        lead = smin64(lead, sub64(drawn, own));
    }
    if under {
        var own = given.zw;
        if (known & 2u) == 0u {
            own = logit(shl64(high, 9u));
        }
        lead = smin64(lead, sub64(own, drawn));
    }
    return lead;
}

// `a b / 2^32`, unsigned, floored: a product of presences.
fn times_q(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    return shr128(mul128(a, b), 32u);
}

// The wood's share `used` ticks between the cycle's and the next's logs.
fn wood_at_q(a: vec2<u32>, b: vec2<u32>, used: vec2<u32>) -> vec2<u32> {
    if !slt64(neg64(inf()), a) || !slt64(neg64(inf()), b) {
        let ea = expq(a);
        return add64(ea, smul_shr(sub64(expq(b), ea), used, 20u, false));
    }
    return expq(add64(a, smul_shr(sub64(b, a), used, 20u, false)));
}

fn window_q(wood: vec2<u32>, rate: vec2<u32>, lo: vec2<u32>, hi: vec2<u32>) -> vec2<u32> {
    return smin64(smax64(mulq(rate, wood), lo), hi);
}

// D17: what a child carries up into its bearer's living time, Q32.32.
fn carry_q(before: vec2<u32>, weight: vec2<u32>, gap: u32, lived: vec2<u32>) -> vec2<u32> {
    return add64(before, times_q(weight, add64(w(0u, gap), lived)));
}

// D17: an axis's fade, Q32; shed where it is none.
fn fade_q(delay: vec2<u32>, birth: u32, lived: vec2<u32>, alive: vec2<u32>, age: u32) -> vec2<u32> {
    let until = add64(w(0u, birth), lived);
    let t = sub64(delay, sub64(w(0u, age), until));
    return smin64(smax64(t, min64(alive, one32())), one32());
}

fn past_q(d: vec2<u32>, bound: vec2<u32>) -> vec2<u32> {
    if is_zero(bound) {
        return inf();
    }
    return ln_ratio(shl64(d, 9u), bound);
}

fn above_q(d: vec2<u32>, bound: vec2<u32>) -> vec2<u32> {
    if is_zero(bound) {
        return inf();
    }
    return sub64(logit_draw(d), logit(bound));
}

// D11: the waking ticks, or all ones where it never wakes.
fn wake_q(delay: vec2<u32>, per: vec2<u32>, d: vec2<u32>) -> vec2<u32> {
    let one = w(0u, 0x200000u);
    let wait = mulq(ln_ratio(one, sub64(one, d)), per);
    if is_inf(wait) {
        return vec2<u32>(0xffffffffu);
    }
    return add64(delay, shr64(wait, 12u));
}

// The sum of two `-log2` chances, saturated at `INF` (`decide::and`): two
// below `INF` sum below 2^63, so only an unbounded one needs its own test.
fn and_q(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    if is_inf(a) || is_inf(b) {
        return inf();
    }
    return smin64(add64(a, b), inf());
}

// D15.
fn kept_q(dead: vec2<u32>, conditioned: bool, context: vec2<u32>, d: vec2<u32>) -> bool {
    let gone = prob(dead);
    var lives = sub64(p_one(), gone);
    if conditioned {
        let rest = sub64(p_one(), mul62(gone, prob(context)));
        if is_zero(rest) || !lt64(lives, rest) {
            lives = p_one();
        } else {
            lives = fraction(lives, rest, 62u);
        }
    }
    return !lt64(d, threshold(sub64(p_one(), lives)));
}
