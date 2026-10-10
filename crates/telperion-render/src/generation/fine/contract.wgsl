// The decision contract on the GPU (fn-254, after fn-209 F1):
// `telperion_space::contract` and `contract::decide` in f32, by the same
// operations. Identity stays integer: a key is a 64-bit hash and a draw
// its 53 bits (`vec2<u32>`, low word first), time a count of ticks of
// 2^-20 cycle. A log beyond any finite one is `INF`, a finite sentinel,
// tested by name at every site: WGSL may assume no infinities.

const INF: f32 = 1e30;
const ONE53 = vec2<u32>(0u, 0x200000u);

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

// The internode draw of the node keyed `key` (`contract::internode_draw`,
// fn-222).
fn internode_draw(key: vec2<u32>) -> vec2<u32> {
    return draw(child(key, KEY_INTERNODE));
}

// The insertion draw of the axis whose lineage is `key`
// (`contract::insertion_draw`, fn-224).
fn insertion_draw(key: vec2<u32>) -> vec2<u32> {
    return draw(child(key, KEY_INSERTION));
}

// The tropism draw of the axis whose lineage is `key`
// (`contract::tropism_draw`, fn-224).
fn tropism_draw(key: vec2<u32>) -> vec2<u32> {
    return draw(child(key, KEY_TROPISM));
}

// The contract's own `log2` and `exp2` (`contract::log2`, `exp2`): plain
// f32 arithmetic, as the CPU's, never the device's own, whose accuracy
// near one is an absolute 2^-21 and differs from vendor to vendor.
fn log2c(x_in: f32) -> f32 {
    var x = x_in;
    var below = 0;
    if x < 1.17549435e-38 {
        x *= 16777216.0;
        below = 24;
    }
    let bits = bitcast<u32>(x);
    var e = i32((bits >> 23u) & 0xffu) - 127 - below;
    var m = bitcast<f32>((bits & 0x7fffffu) | 0x3f800000u);
    if m > 1.4142135 {
        m *= 0.5;
        e += 1;
    }
    let s = (m - 1.0) / (m + 1.0);
    let z = s * s;
    let ln = s * (2.0 + z * (2.0 / 3.0 + z * (0.4 + z * (2.0 / 7.0 + z * (2.0 / 9.0)))));
    return f32(e) + ln * 1.442695;
}

fn exp2c(x: f32) -> f32 {
    if x < -126.0 {
        return 0.0;
    }
    let n = floor(min(x, 127.0) + 0.5);
    let f = min(x, 127.0) - n;
    let p = 1.0 + f * (0.6931472 + f * (0.2402265 + f * (0.05550411 + f * (0.009618129
        + f * (0.0013333558 + f * (0.0001540353 + f * 0.000015252734))))));
    return p * bitcast<f32>(u32(i32(n) + 127) << 23u);
}

fn lnc(x: f32) -> f32 {
    return log2c(x) * 0.6931472;
}

fn expc(x: f32) -> f32 {
    return exp2c(x * 1.442695);
}

// 2^k for a whole k in the normal range, exactly.
fn pow2(k: i32) -> f32 {
    return bitcast<f32>(u32(k + 127) << 23u);
}

// An integer below 2^64 as the nearest f32, ties to even: Rust's `as f32`.
fn to_f32(v: vec2<u32>) -> f32 {
    let top = 63u - countLeadingZeros(v.y);
    var e = top;
    if v.y == 0u {
        if v.x < 0x1000000u {
            return f32(v.x);
        }
        e = 31u - countLeadingZeros(v.x);
    }
    let s = e - 23u;
    let m = shr64(v, s).x;
    let rest = sub64(v, shl64(w(m, 0u), s));
    let half = shl64(w(1u, 0u), s - 1u);
    var r = m;
    if lt64(half, rest) || (all(rest == half) && (m & 1u) == 1u) {
        r += 1u;
    }
    return f32(r) * pow2(i32(s));
}

// A non-negative f32's floor as an integer below 2^64: Rust's `as u64`.
fn floor64(x: f32) -> vec2<u32> {
    if !(x >= 1.0) {
        return vec2<u32>(0u);
    }
    let hi = floor(x * pow2(-32));
    return w(u32(x - hi * pow2(32)), u32(hi));
}

// `ceil(p 2^53)` (or its floor), exact for every f32: a probability's
// threshold against a draw (`contract::threshold`); none for none, the
// whole 2^53 for one or more.
fn scaled53(p: f32, up: bool) -> vec2<u32> {
    if !(p > 0.0) {
        return vec2<u32>(0u);
    }
    if p >= 1.0 {
        return ONE53;
    }
    let bits = bitcast<u32>(p);
    let ex = (bits >> 23u) & 0xffu;
    var m = bits & 0x7fffffu;
    var e = -149;
    if ex != 0u {
        m |= 0x800000u;
        e = i32(ex) - 150;
    }
    let k = e + 53;
    if k >= 0 {
        return shl64(w(m, 0u), u32(k));
    }
    let s = u32(-k);
    if s >= 32u {
        return w(select(0u, 1u, up), 0u);
    }
    let q = m >> s;
    let rest = m & ((1u << s) - 1u);
    return w(q + select(0u, 1u, up && rest != 0u), 0u);
}

fn threshold(p: f32) -> vec2<u32> {
    return scaled53(p, true);
}

// `-log2 p`: `INF` for none, none for one or more.
fn neg_log2(p: f32) -> f32 {
    if !(p > 0.0) {
        return INF;
    }
    if p >= 1.0 {
        return 0.0;
    }
    return -log2c(p);
}

// `2^-l`: one for none or less, none for `INF`.
fn prob(l: f32) -> f32 {
    if l <= 0.0 {
        return 1.0;
    }
    if l >= INF {
        return 0.0;
    }
    return exp2c(-l);
}

// `1 - 2^-l` (`contract::fails`): a series under 1/8, so a chance near
// none keeps its bits.
fn fails(l: f32) -> f32 {
    if l <= 0.0 {
        return 0.0;
    }
    if l >= INF {
        return 1.0;
    }
    if l < 0.125 {
        let y = l * 0.6931472;
        return y * (1.0 - y * (0.5 - y * (1.0 / 6.0 - y * (1.0 / 24.0 - y * (1.0 / 120.0)))));
    }
    return 1.0 - exp2c(-l);
}

// A `-log2` chance a cycle over `share`, in 2^-32 of a cycle.
fn scaled(l: f32, share: vec2<u32>) -> f32 {
    if l >= INF {
        return INF;
    }
    return l * (to_f32(share) * pow2(-32));
}

// `lead / window` in [0, 1]: whole for an unbounded lead or no window.
fn presence(lead: f32, window: f32) -> f32 {
    if lead >= INF || window <= 0.0 {
        return 1.0;
    }
    if lead <= 0.0 {
        return 0.0;
    }
    return min(lead / window, 1.0);
}

fn logit(p: f32) -> f32 {
    if !(p > 0.0) {
        return -INF;
    }
    if p >= 1.0 {
        return INF;
    }
    return lnc(p / (1.0 - p));
}

// The log-odds of a 53-bit integer `d` against 2^53: a draw's, or a
// threshold's as its probability's.
fn logit53(d: vec2<u32>) -> f32 {
    if is_zero(d) {
        return -INF;
    }
    if !lt64(d, ONE53) {
        return INF;
    }
    return lnc(to_f32(d) / to_f32(sub64(ONE53, d)));
}

// `-log2` of a draw's uniform.
fn draw_log2(d: vec2<u32>) -> f32 {
    return neg_log2(to_f32(d) * pow2(-53));
}

// D5: the threshold of `2^-x`, as the complement of its failure's where
// survival is likelier than not, as its own where not.
fn survival_fine(l: f32, share: vec2<u32>) -> vec2<u32> {
    let x = scaled(l, share);
    if x >= 1.0 {
        return threshold(prob(x));
    }
    return sub64(ONE53, scaled53(fails(x), false));
}

// D6 over an exposure in 2^-32 of a cycle; `rises` false where the
// hazard does not rise.
fn abortion_fine(l0: f32, rises: bool, rise: f32, grown: vec2<u32>, exposure: vec2<u32>) -> vec2<u32> {
    var l = l0;
    if rises && l < INF {
        if is_zero(grown) {
            l = select(INF, 0.0, rise > 0.0);
        } else {
            l = l * exp2c(rise * log2c(to_f32(grown) * pow2(-20)));
        }
    }
    return threshold(fails(scaled(l, exposure)));
}

// The wood's share `used` ticks between the cycle's and the next's logs.
fn wood_at(a: f32, b: f32, used: vec2<u32>) -> f32 {
    let u = to_f32(used) * pow2(-20);
    if a <= -INF || b <= -INF {
        let ea = select(expc(a), 0.0, a <= -INF);
        let eb = select(expc(b), 0.0, b <= -INF);
        return ea * (1.0 - u) + eb * u;
    }
    return expc(a * (1.0 - u) + b * u);
}

fn window(wood: f32, rate: f32, lo: f32, hi: f32) -> f32 {
    return clamp(rate * wood, lo, hi);
}

// D10, `go` the log-odds of the move's chance.
fn moved(go: f32, d: vec2<u32>, a: f32, b: f32, used: vec2<u32>, rate: f32, lo: f32, hi: f32) -> f32 {
    if go >= INF || is_zero(d) {
        return 1.0;
    }
    let lead = go - logit53(d);
    return presence(lead, window(wood_at(a, b, used), rate, lo, hi));
}

// Host decision 8 (`contract/presence.rs`): the lesser of a draw's lead
// over the threshold `low` it had to reach and under the threshold `high`
// it had to stay under, in log-odds; unbounded where both are certain.
fn lead(low: vec2<u32>, high: vec2<u32>, d: vec2<u32>) -> f32 {
    let drawn = logit53(d);
    var out = INF;
    if !is_zero(low) {
        out = min(out, drawn - logit53(low));
    }
    if lt64(high, ONE53) && !is_zero(d) {
        out = min(out, logit53(high) - drawn);
    }
    return out;
}

// The cycles lived, `-log2(d / 2^53) / l`, where under one.
fn lived_part(l: f32, d: vec2<u32>) -> f32 {
    let u = draw_log2(d);
    return select(-1.0, u / l, u < l);
}

// D5's lived time (fn-199 R1), in ticks rounded up, at most `share`.
fn lived_ticks(l: f32, share: vec2<u32>, d: vec2<u32>) -> vec2<u32> {
    if l <= 0.0 {
        return share;
    }
    if l >= INF {
        return vec2<u32>(0u);
    }
    let t = lived_part(l, d);
    if t < 0.0 {
        return min64(share, w(0x100000u, 0u));
    }
    return min64(w(u32(ceil(t * pow2(20))), 0u), share);
}

// D5's lived time before the tick grid, in 2^-32 of a cycle (fn-199 B2).
fn lived_fine(l: f32, share: vec2<u32>, d: vec2<u32>) -> vec2<u32> {
    if l <= 0.0 {
        return shl64(share, 12u);
    }
    if l >= INF {
        return vec2<u32>(0u);
    }
    let t = lived_part(l, d);
    if t < 0.0 {
        return shl64(min64(share, w(0x100000u, 0u)), 12u);
    }
    return min64(floor64(t * pow2(32)), shl64(share, 12u));
}

// D17: what a child carries up into its bearer's living time, in cycles.
fn carry(before: f32, weight: f32, gap: u32, lived: f32) -> f32 {
    return before + weight * (f32(gap) + lived);
}

// D17: an axis's fade; shed where it is none.
fn fade(delay: f32, birth: u32, lived: f32, alive: f32, age: u32) -> f32 {
    let idle = f32(i32(age) - i32(birth)) - lived;
    return clamp(delay - idle, min(alive, 1.0), 1.0);
}

// A node's lead past a uniform law's bound of log `own`: `ln u - own`.
fn past(d: vec2<u32>, own: f32) -> f32 {
    if own <= -INF {
        return INF;
    }
    if is_zero(d) {
        return -INF;
    }
    return lnc(to_f32(d) * pow2(-53)) - own;
}

// A node's lead past a Poisson law's cumulative bound of log-odds `own`.
fn above(d: vec2<u32>, own: f32) -> f32 {
    if own <= -INF {
        return INF;
    }
    return logit53(d) - own;
}

// D11: the waking ticks, or all ones where it never wakes.
fn wake_ticks(delay: vec2<u32>, per: f32, d: vec2<u32>) -> vec2<u32> {
    let nats = -lnc(to_f32(sub64(ONE53, d)) * pow2(-53));
    let wait = nats * per * pow2(20);
    if !(wait < pow2(62)) {
        return vec2<u32>(0xffffffffu);
    }
    return add64(delay, floor64(wait));
}
