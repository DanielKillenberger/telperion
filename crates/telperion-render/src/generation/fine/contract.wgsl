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

// The table-reduced polynomials of log2 and exp2_neg (host decision,
// 2026-10-07; `fixed::RECIP`, `LOGS`, `LOG_POLY`, `POWERS`, `EXP_POLY`).
var<private> RECIP: array<vec2<u32>, 64> = array<vec2<u32>, 64>(
    vec2<u32>(0xf80fe040u, 0x3f80fe03u), vec2<u32>(0x9484e2b0u, 0x3e88cb3cu),
    vec2<u32>(0x03d980f6u, 0x3d980f66u), vec2<u32>(0x03cae759u, 0x3cae7592u),
    vec2<u32>(0xf10d148eu, 0x3bcbadc7u), vec2<u32>(0x70586723u, 0x3aef6ca9u),
    vec2<u32>(0xdd80e866u, 0x3a196b1eu), vec2<u32>(0xbdc32204u, 0x3949660au),
    vec2<u32>(0x87f1e038u, 0x387f1e03u), vec2<u32>(0x280dee96u, 0x37ba5713u),
    vec2<u32>(0xb4671656u, 0x36fad87bu), vec2<u32>(0xd901b203u, 0x36406c80u),
    vec2<u32>(0x8ae0358bu, 0x358ae035u), vec2<u32>(0xa034da03u, 0x34da034du),
    vec2<u32>(0xf4960343u, 0x342da7f2u), vec2<u32>(0xc9420338u, 0x3385a29du),
    vec2<u32>(0x1970e4f8u, 0x32e1c9f0u), vec2<u32>(0xa1c451abu, 0x3241f693u),
    vec2<u32>(0x6031a603u, 0x31a6031au), vec2<u32>(0x57609329u, 0x310dcbe1u),
    vec2<u32>(0x657dba52u, 0x30792ef5u), vec2<u32>(0x02fe80c0u, 0x2fe80bfau),
    vec2<u32>(0xc1d986a9u, 0x2f5a4411u), vec2<u32>(0x69536203u, 0x2ecfb9c8u),
    vec2<u32>(0x8dbd780cu, 0x2e4850feu), vec2<u32>(0x866f8d96u, 0x2dc3eed6u),
    vec2<u32>(0xa6e52088u, 0x2d4279a2u), vec2<u32>(0xa245f203u, 0x2cc3d8d4u),
    vec2<u32>(0x02c47f4fu, 0x2c47f4eeu), vec2<u32>(0xa02bceb7u, 0x2bceb771u),
    vec2<u32>(0x02b580adu, 0x2b580ad6u), vec2<u32>(0xa0d67344u, 0x2ae3da78u),
    vec2<u32>(0xe81fd58eu, 0x2a721291u), vec2<u32>(0x02a02a03u, 0x2a02a02au),
    vec2<u32>(0x4b5edceau, 0x2995710eu), vec2<u32>(0x65a0cdd4u, 0x292a73c7u),
    vec2<u32>(0xeb9f3438u, 0x28c1978fu), vec2<u32>(0xaa3f0ddfu, 0x285acc4bu),
    vec2<u32>(0x6027f602u, 0x27f6027fu), vec2<u32>(0xf757ce88u, 0x27932b48u),
    vec2<u32>(0x30fec66eu, 0x27323858u), vec2<u32>(0xbc0e8f2au, 0x26d31be7u),
    vec2<u32>(0xaf7963c2u, 0x2675c8b6u), vec2<u32>(0x61a32026u, 0x261a3202u),
    vec2<u32>(0x97012e02u, 0x25c04b80u), vec2<u32>(0x02568096u, 0x2568095au),
    vec2<u32>(0x11602511u, 0x25116025u), vec2<u32>(0x0125e227u, 0x24bc44e1u),
    vec2<u32>(0x3579be02u, 0x2468acf1u), vec2<u32>(0xcf81b10fu, 0x24168e18u),
    vec2<u32>(0x7f70e886u, 0x23c5de76u), vec2<u32>(0x8dda5202u, 0x23769480u),
    vec2<u32>(0x19453809u, 0x2328a701u), vec2<u32>(0x84e6f1d7u, 0x22dc0d12u),
    vec2<u32>(0x159a76d2u, 0x2290be1cu), vec2<u32>(0xba6066d4u, 0x2246b1ceu),
    vec2<u32>(0xfde021feu, 0x21fde021u), vec2<u32>(0x1e8d2b32u, 0x21b64151u),
    vec2<u32>(0x4b3b8f27u, 0x216fcdd8u), vec2<u32>(0x0212a7e7u, 0x212a7e72u),
    vec2<u32>(0x8fef8cdau, 0x20e64c14u), vec2<u32>(0xae680829u, 0x20a32fefu),
    vec2<u32>(0x3ebc349eu, 0x2061236au), vec2<u32>(0x20202020u, 0x20202020u),
);
var<private> LOGS: array<vec2<u32>, 64> = array<vec2<u32>, 64>(
    vec2<u32>(0xb778428bu, 0x00b7f285u), vec2<u32>(0xa266825eu, 0x02239a3au),
    vec2<u32>(0x21556bdfu, 0x0389bf57u), vec2<u32>(0x49fc70b6u, 0x04ea8bf7u),
    vec2<u32>(0x918cd6acu, 0x0646285bu), vec2<u32>(0x5fe96c87u, 0x079cbb04u),
    vec2<u32>(0xaa95bdafu, 0x08ee68cbu), vec2<u32>(0xc1fba698u, 0x0a3b54fcu),
    vec2<u32>(0x77813f96u, 0x0b83a16au), vec2<u32>(0xbf386f81u, 0x0cc76e83u),
    vec2<u32>(0xeca6f720u, 0x0e06db66u), vec2<u32>(0xa7352665u, 0x0f4205f3u),
    vec2<u32>(0xb030096fu, 0x10790adbu), vec2<u32>(0x91f07053u, 0x11ac05b2u),
    vec2<u32>(0x4d9aaf6fu, 0x12db10fcu), vec2<u32>(0x1b044976u, 0x1406463bu),
    vec2<u32>(0x4c96b37eu, 0x152dbdfcu), vec2<u32>(0x677ba6e5u, 0x16518fe4u),
    vec2<u32>(0x7efb3be5u, 0x1771d2bau), vec2<u32>(0xe0b225a5u, 0x188e9c72u),
    vec2<u32>(0x1e232f34u, 0x19a80239u), vec2<u32>(0x7f1f48e1u, 0x1abe1879u),
    vec2<u32>(0xe79030abu, 0x1bd0f2e9u), vec2<u32>(0x3a587cc9u, 0x1ce0a492u),
    vec2<u32>(0x42364c4eu, 0x1ded3fd4u), vec2<u32>(0x28e2207du, 0x1ef6d673u),
    vec2<u32>(0x83ff9ab9u, 0x1ffd799au), vec2<u32>(0xfee07aa4u, 0x210139e4u),
    vec2<u32>(0xa791d4f6u, 0x22022762u), vec2<u32>(0xe52ed49eu, 0x2300519eu),
    vec2<u32>(0x1d024c85u, 0x23fbc7a6u), vec2<u32>(0x0b97f00cu, 0x24f4980bu),
    vec2<u32>(0xd67e4c28u, 0x25ead0ebu), vec2<u32>(0xdb22c480u, 0x26de7ff6u),
    vec2<u32>(0x3ee02cd6u, 0x27cfb26fu), vec2<u32>(0x440e714au, 0x28be7531u),
    vec2<u32>(0x679d8895u, 0x29aad4b6u), vec2<u32>(0x4a871b64u, 0x2a94dd19u),
    vec2<u32>(0x6a286957u, 0x2b7c9a19u), vec2<u32>(0xaa609081u, 0x2c62171eu),
    vec2<u32>(0xb41e1411u, 0x2d455f3cu), vec2<u32>(0x2ad8f7cbu, 0x2e267d36u),
    vec2<u32>(0xbb4caef6u, 0x2f057b7fu), vec2<u32>(0x059e2ba3u, 0x2fe26443u),
    vec2<u32>(0x64f64c22u, 0x30bd4161u), vec2<u32>(0x96777b66u, 0x31961c76u),
    vec2<u32>(0x41565d3bu, 0x326cfedbu), vec2<u32>(0x61c09fbdu, 0x3341f1a7u),
    vec2<u32>(0x982259cdu, 0x3414fdb4u), vec2<u32>(0x5e418f7au, 0x34e62ba0u),
    vec2<u32>(0x239065a3u, 0x35b583ceu), vec2<u32>(0x520117b4u, 0x36830e69u),
    vec2<u32>(0x3c92c81du, 0x374ed367u), vec2<u32>(0xf8bba6c5u, 0x3818da88u),
    vec2<u32>(0x23c38ec0u, 0x38e12b5du), vec2<u32>(0x95120d85u, 0x39a7cd41u),
    vec2<u32>(0xfe63aa53u, 0x3a6cc764u), vec2<u32>(0x7acc2986u, 0x3b3020c8u),
    vec2<u32>(0x0d5e5897u, 0x3bf1e041u), vec2<u32>(0x10459cadu, 0x3cb20c79u),
    vec2<u32>(0x9511f61au, 0x3d70abf1u), vec2<u32>(0xb6ec6076u, 0x3e2dc503u),
    vec2<u32>(0xdf5f5bafu, 0x3ee95de1u), vec2<u32>(0xfe55f286u, 0x3fa37c98u),
);
var<private> LOG_POLY: array<vec2<u32>, 6> = array<vec2<u32>, 6>(
    vec2<u32>(0xae0bf85eu, 0x5c551d94u), vec2<u32>(0xa8fa03d1u, 0xd1d57135u),
    vec2<u32>(0x3a03fd75u, 0x1ec709dcu), vec2<u32>(0xd47d01e9u, 0xe8eab89au),
    vec2<u32>(0xef9bfe79u, 0x12776c50u), vec2<u32>(0xe2fe0146u, 0xf09c7b11u),
);
var<private> POWERS: array<vec2<u32>, 64> = array<vec2<u32>, 64>(
    vec2<u32>(0x00000000u, 0x40000000u), vec2<u32>(0x3d21b05du, 0x3f4f8303u),
    vec2<u32>(0xdc8a80cfu, 0x3ea0ecb6u), vec2<u32>(0xc2b44eeeu, 0x3df437dcu),
    vec2<u32>(0x4921b30bu, 0x3d495f45u), vec2<u32>(0x167fec4eu, 0x3ca05dcfu),
    vec2<u32>(0xf736bd73u, 0x3bf92e66u), vec2<u32>(0xb6650ae1u, 0x3b53cc07u),
    vec2<u32>(0xf7490e4cu, 0x3ab031b9u), vec2<u32>(0x0f12f71au, 0x3a0e5a94u),
    vec2<u32>(0xdf20d22au, 0x396e41b9u), vec2<u32>(0xafa29694u, 0x38cfe25cu),
    vec2<u32>(0x0aa53844u, 0x383337bbu), vec2<u32>(0x9783a449u, 0x37983d20u),
    vec2<u32>(0xf6bc8dd5u, 0x36feede5u), vec2<u32>(0x9e2bf5edu, 0x36674570u),
    vec2<u32>(0xb5a75abdu, 0x35d13f32u), vec2<u32>(0xf3fb7e88u, 0x353cd6aau),
    vec2<u32>(0x7c4ab917u, 0x34aa0764u), vec2<u32>(0xbbcac965u, 0x3418ccf6u),
    vec2<u32>(0x47e12039u, 0x33892305u), vec2<u32>(0xbc9c9f17u, 0x32fb053fu),
    vec2<u32>(0x9b8bc9e9u, 0x326e6f61u), vec2<u32>(0x2aee6c57u, 0x31e35d32u),
    vec2<u32>(0x5541b6b7u, 0x3159ca84u), vec2<u32>(0x8925d902u, 0x30d1b336u),
    vec2<u32>(0x999c2516u, 0x304b1332u), vec2<u32>(0x9e9cc421u, 0x2fc5e66du),
    vec2<u32>(0xd6030db0u, 0x2f4228e7u), vec2<u32>(0x84cf917fu, 0x2ebfd6acu),
    vec2<u32>(0xd8bee7bau, 0x2e3eebd1u), vec2<u32>(0xca345de4u, 0x2dbf6478u),
    vec2<u32>(0xfe779921u, 0x2d413cccu), vec2<u32>(0xaa444922u, 0x2cc47104u),
    vec2<u32>(0x74ab0964u, 0x2c48fd60u), vec2<u32>(0x5a4290ddu, 0x2bcede2bu),
    vec2<u32>(0x90a852b2u, 0x2b560fbau), vec2<u32>(0x6a4fb4ceu, 0x2ade8e6du),
    vec2<u32>(0x3a9f03beu, 0x2a6856adu), vec2<u32>(0x3a594d5au, 0x29f364edu),
    vec2<u32>(0x6c544e3bu, 0x297fb5aau), vec2<u32>(0x8279a028u, 0x290d456bu),
    vec2<u32>(0xc3125a06u, 0x289c10c0u), vec2<u32>(0xee5c53f1u, 0x282c1443u),
    vec2<u32>(0x2468446bu, 0x27bd4c98u), vec2<u32>(0xcb3fedd4u, 0x274fb669u),
    vec2<u32>(0x7553954eu, 0x26e34e6eu), vec2<u32>(0xc82dfe99u, 0x26781164u),
    vec2<u32>(0x636e2a5cu, 0x260dfc14u), vec2<u32>(0xc8061680u, 0x25a50b4du),
    vec2<u32>(0x3fbdc258u, 0x253d3beau), vec2<u32>(0xc4f9ba4bu, 0x24d68acbu),
    vec2<u32>(0xeac470ceu, 0x2470f4dcu), vec2<u32>(0xc519ac77u, 0x240c7710u),
    vec2<u32>(0xd17354f0u, 0x23a90e62u), vec2<u32>(0xdf96ea79u, 0x2346b7d6u),
    vec2<u32>(0xfaa2f5bau, 0x22e57078u), vec2<u32>(0x525bbf67u, 0x2285355du),
    vec2<u32>(0x24b6a14au, 0x222603a0u), vec2<u32>(0xa7a34404u, 0x21c7d865u),
    vec2<u32>(0xf3121ec5u, 0x216ab0d9u), vec2<u32>(0xeb379012u, 0x210e8a30u),
    vec2<u32>(0x2b0ae876u, 0x20b361a6u), vec2<u32>(0xef00c1ddu, 0x2059347cu),
);
var<private> EXP_POLY: array<vec2<u32>, 7> = array<vec2<u32>, 7>(
    vec2<u32>(0x00000000u, 0x40000000u), vec2<u32>(0x00000000u, 0xc0000000u),
    vec2<u32>(0x00000000u, 0x20000000u), vec2<u32>(0x55555555u, 0xf5555555u),
    vec2<u32>(0xaaaaaaabu, 0x02aaaaaau), vec2<u32>(0x77777777u, 0xff777777u),
    vec2<u32>(0x16c16c17u, 0x0016c16cu),
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
// fn-222).
fn internode_draw(key: vec2<u32>) -> vec2<u32> {
    return draw(child(key, KEY_INTERNODE));
}

// The insertion draw of the axis whose lineage is `key`
// (`fixed::insertion_draw`, fn-224).
fn insertion_draw(key: vec2<u32>) -> vec2<u32> {
    return draw(child(key, KEY_INSERTION));
}

// The tropism draw of the axis whose lineage is `key`
// (`fixed::tropism_draw`, fn-224).
fn tropism_draw(key: vec2<u32>) -> vec2<u32> {
    return draw(child(key, KEY_TROPISM));
}

// A Q62 probability's threshold against a 53-bit draw.
fn threshold(p: vec2<u32>) -> vec2<u32> {
    return shr64(add64(min64(p, p_one()), w(511u, 0u)), 9u);
}

fn mul62(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    return shr128(mul128(a, b), 62u);
}

// `fixed::q32` of a float the device holds (a scale; the fine roots'
// girth, host 2026-10-08): round(x 2^32),
// half away from zero, saturated at INF; none at or below zero.
fn q32_f(x: f32) -> vec2<u32> {
    if !(x > 0.0) {
        return vec2<u32>(0u);
    }
    let bits = bitcast<u32>(x);
    let m = (bits & 0x7fffffu) | 0x800000u;
    // x = m 2^(e - 150), so x 2^32 = m 2^(e - 118).
    let e = i32((bits >> 23u) & 0xffu) - 118;
    // Saturated at INF, as `q32` clamps.
    if e >= 39 {
        return inf();
    }
    if e >= 0 {
        return min64(shl64(w(m, 0u), u32(e)), inf());
    }
    let s = u32(-e);
    if s >= 25u {
        return vec2<u32>(0u);
    }
    return w((m + (1u << (s - 1u))) >> s, 0u);
}

// log2 n in Q32; -INF for none.
fn log2q(n: vec2<u32>) -> vec2<u32> {
    if is_zero(n) {
        return neg64(inf());
    }
    let e = 63u - clz64(n);
    return w(log2_frac(select(shr64(n, 1u), shl64(n, 62u - e), e <= 62u)), e);
}

// `a b / 2^62`, floored, signed.
fn mul62s(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    return smul_shr(a, b, 62u, false);
}

// The fraction of log2 m, Q32, of a Q62 mantissa in [1, 2)
// (`fixed::log2_frac`): the reciprocal of its sixty-fourth, a sixth-degree
// polynomial by Horner, one floor to Q32.
fn log2_frac(m: vec2<u32>) -> u32 {
    let j = (m.y >> 24u) & 63u;
    let t = sub64(shr128(mul128(m, RECIP[j]), 62u), w(0u, 0x40000000u));
    var p = vec2<u32>(0u);
    for (var k = 6; k > 0; k--) {
        p = add64(LOG_POLY[k - 1], mul62s(t, p));
    }
    let log = add64(LOGS[j], mul62s(t, p));
    if negative(log) {
        return 0u;
    }
    let q = shr64(log, 30u);
    return select(0xffffffffu, q.x, q.y == 0u);
}

// 2^-x for x in Q32, a Q62 probability (`fixed::exp2_neg`).
fn exp2_neg(x: vec2<u32>) -> vec2<u32> {
    if negative(x) || is_zero(x) {
        return p_one();
    }
    let whole = x.y;
    if whole >= 63u {
        return vec2<u32>(0u);
    }
    let j = x.x >> 26u;
    let g = x.x & 0x3ffffffu;
    let y = shr128(mul128(w(g, 0u), LN2), 32u);
    var q = vec2<u32>(0u);
    for (var k = 7; k > 0; k--) {
        q = add64(EXP_POLY[k - 1], mul62s(y, q));
    }
    return shr64(shr128(mul128(POWERS[j], q), 62u), whole);
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
    return survival_fine_q(l, shl64(share, 12u));
}

// A share in 2^-32 of a cycle, finer than a tick (fn-199 B2).
fn scaled_fine(l: vec2<u32>, share: vec2<u32>) -> vec2<u32> {
    if is_inf(l) {
        return inf();
    }
    return smin64(smul_shr(l, share, 32u, true), inf());
}

fn survival_fine_q(l: vec2<u32>, share: vec2<u32>) -> vec2<u32> {
    return threshold(prob(scaled_fine(l, share)));
}

// D6; `rises` false where the hazard does not rise.
fn abortion_q(l0: vec2<u32>, rises: bool, rise: vec2<u32>, grown: vec2<u32>, exposure: vec2<u32>) -> vec2<u32> {
    return abortion_fine_q(l0, rises, rise, grown, shl64(exposure, 12u));
}

// D6 over an exposure in 2^-32 of a cycle (fn-199 B2).
fn abortion_fine_q(l0: vec2<u32>, rises: bool, rise: vec2<u32>, grown: vec2<u32>, exposure: vec2<u32>) -> vec2<u32> {
    var l = l0;
    if rises && !is_inf(l) {
        if is_zero(grown) {
            l = select(inf(), vec2<u32>(0u), slt64(vec2<u32>(0u), rise));
        } else {
            let lt = sub64(log2q(grown), w(0u, 20u));
            l = mulq(l, exp2q(mulq(rise, lt)));
        }
    }
    return sub64(w(0u, 0x200000u), shr64(prob(scaled_fine(l, exposure)), 9u));
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
// D5's lived time (fn-199 R1): `-log2(d / 2^53) 2^20 / l` ticks, rounded
// up, at most `share`.
fn lived_q(l: vec2<u32>, share: vec2<u32>, d: vec2<u32>) -> vec2<u32> {
    if !slt64(vec2<u32>(0u), l) {
        return share;
    }
    if is_inf(l) {
        return vec2<u32>(0u);
    }
    let u = neg_log2(shl64(d, 9u));
    if !slt64(u, l) {
        return min64(share, w(0x100000u, 0u));
    }
    return min64(fraction_up(u, l, 20u), share);
}

// D5's lived time before the tick grid, in 2^-32 of a cycle (fn-199 B2,
// `lived_fine_q`).
fn lived_fine_q(l: vec2<u32>, share: vec2<u32>, d: vec2<u32>) -> vec2<u32> {
    if !slt64(vec2<u32>(0u), l) {
        return shl64(share, 12u);
    }
    if is_inf(l) {
        return vec2<u32>(0u);
    }
    let u = neg_log2(shl64(d, 9u));
    if !slt64(u, l) {
        return shl64(min64(share, w(0x100000u, 0u)), 12u);
    }
    return min64(fraction(u, l, 32u), shl64(share, 12u));
}

// `fraction`, rounded up: one more where a remainder is left.
fn fraction_up(num: vec2<u32>, den: vec2<u32>, bits: u32) -> vec2<u32> {
    var r = num;
    var q = vec2<u32>(0u);
    for (var i = 0u; i < bits; i++) {
        r = shl64(r, 1u);
        q = shl64(q, 1u);
        if !lt64(r, den) {
            r = sub64(r, den);
            q.x |= 1u;
        }
    }
    if !is_zero(r) {
        q = add64(q, w(1u, 0u));
    }
    return q;
}

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
