// Presences on the GPU chain grower (fn-209 F1, task 4): how grown each
// element a draw made is (`presence.rs`), in f32, from the species
// point's continuous values (`telperion_space::GeometryTables`, packed
// into `words` at `cfg.geo` one f32 a word). Presences set geometry only;
// every decision stays on the contract. Part of `chain.wgsl`'s shader.

const RATE: f32 = 6.0;
const FLOOR: f32 = 0.05;
const SPAN_F: f32 = 2.0;
// An unbounded lead or log-odds, as the tables hold one.
const HUGE: f32 = 1e30;
// A PA's block in the tables, and a zone's.
const GP: u32 = 16u;
const GZ: u32 = 4u;

fn gf(i: u32) -> f32 {
    return bitcast<f32>(words[cfg.geo + i].x);
}

fn gu(i: u32) -> u32 {
    return words[cfg.geo + i].x;
}

fn pa_f(pa: u32, k: u32) -> f32 {
    return gf(2u + GP * pa + k);
}

// The zone at table offset `zat`'s block.
fn zone_g(zat: u32) -> u32 {
    let z = (zat - 2u * gu(0u)) / ZW;
    return gu(1u) + GZ * z;
}

// `Windows::wood`, and `wood_at` between a cycle and the next by its log.
fn wood(pa: u32, cycle: u32) -> f32 {
    return gf(bitcast<u32>(pa_f(pa, 6u)) + cycle);
}

fn wood_at(pa: u32, cycle: u32, used: f32) -> f32 {
    let a = wood(pa, cycle);
    let b = wood(pa, cycle + 1u);
    if a <= 0.0 || b <= 0.0 {
        return a + (b - a) * used;
    }
    return exp(log(a) + (log(b) - log(a)) * used);
}

// A Q32 presence, or a tick count, as a real.
fn real_q(q: vec2<u32>) -> f32 {
    return f32(q.y) + f32(q.x) * (1.0 / 4294967296.0);
}

fn cycles_of(t: vec2<u32>) -> f32 {
    return (f32(t.y) * 4294967296.0 + f32(t.x)) * (1.0 / 1048576.0);
}

// A signed difference of two 54-bit integers, as a real.
fn diff_f(a: vec2<u32>, b: vec2<u32>) -> f32 {
    if lt64(a, b) {
        let d = sub64(b, a);
        return -(f32(d.y) * 4294967296.0 + f32(d.x));
    }
    let d = sub64(a, b);
    return f32(d.y) * 4294967296.0 + f32(d.x);
}

fn wide_f(a: vec2<u32>) -> f32 {
    return f32(a.y) * 4294967296.0 + f32(a.x);
}

// ln(1 + x), without the cancellation near none.
fn ln1p_f(x: f32) -> f32 {
    if abs(x) < 0.05 {
        return x * (1.0 - x * (0.5 - x * (1.0 / 3.0 - x * (0.25 - x * 0.2))));
    }
    return log(1.0 + x);
}

const WHOLE53 = vec2<u32>(0u, 0x200000u);

// How far, in log-odds, a draw `d` lies below the bound whose threshold
// is `t` (`lineage::below`): logit(p) - logit(u) as ln(p / u) plus
// ln((1 - u) / (1 - p)), each from the integers' difference, so a lead
// near its bound, where a presence is partial, has no cancellation.
fn below_t(d: vec2<u32>, t: vec2<u32>) -> f32 {
    if !lt64(t, WHOLE53) || is_zero(d) {
        return HUGE;
    }
    let delta = diff_f(t, d);
    return ln1p_f(delta / wide_f(d)) + ln1p_f(delta / wide_f(sub64(WHOLE53, t)));
}

// How far a draw `d` lies above the bound of threshold `t`
// (`lineage::above`).
fn above_t(d: vec2<u32>, t: vec2<u32>) -> f32 {
    if is_zero(t) {
        return HUGE;
    }
    let delta = diff_f(d, t);
    return ln1p_f(delta / wide_f(t)) + ln1p_f(delta / wide_f(sub64(WHOLE53, d)));
}

// `Windows::decided`.
fn decided(lead: f32, wood_share: f32, share: f32) -> f32 {
    let window = clamp(RATE * wood_share * share, FLOOR * share, SPAN_F);
    if lead >= HUGE || window <= 0.0 {
        return 1.0;
    }
    return clamp(lead / window, 0.0, 1.0);
}

// The presence of a relay a stop keyed `key` would have, of PA `pa`'s
// relay `k`: none where it relays none or the draw fails.
fn relayed_f(pa: u32, k: u32, key: vec2<u32>, wood_share: f32) -> f32 {
    let t = row(pa, 6u + k);
    if is_zero(t) {
        return 0.0;
    }
    let d = draw(child(key, RELAY));
    if !lt64(d, t) {
        return 0.0;
    }
    return decided(below_t(d, t), wood_share, 1.0);
}

// The lead of a draw `d` drawn as run `j` of the thresholds at `at`
// (`lineage::bud`): past the nearer of its run's bounds.
fn run_lead(at: u32, j: u32, d: vec2<u32>) -> f32 {
    let low = words[cfg.geo + at + 2u * j];
    let high = words[cfg.geo + at + 2u * j + 1u];
    return min(above_t(d, low), below_t(d, high));
}

// A unit's record: its axis, its index there, its survival and its
// persistence (`presence::Unit`), after the chain's sleeping buds; then
// what its Q32 presences are taken from at the chain's end and those
// presences (`assign.wgsl`).
const UW: u32 = 24u;

fn push_unit(axis: u32, k: u32, survive: f32) -> u32 {
    if n_units >= cfg.unit_cap {
        overflow |= 256u;
        return NONE;
    }
    let u = unit_base + n_units * UW;
    for (var f = 4u; f < UW; f++) {
        sleepers[u + f] = 0u;
    }
    sleepers[u] = axis;
    sleepers[u + 1u] = k;
    sleepers[u + 2u] = bitcast<u32>(survive);
    sleepers[u + 3u] = bitcast<u32>(1.0);
    n_units += 1u;
    return u;
}

// Unit `k` of `axis`'s record, none where it grew no such unit.
fn unit_at(axis: u32, k: u32) -> u32 {
    for (var i = 0u; i < n_units; i++) {
        let u = unit_base + i * UW;
        if sleepers[u] == axis && sleepers[u + 1u] == k {
            return u;
        }
    }
    return NONE;
}

fn unit_f(u: u32, f: u32) -> f32 {
    return bitcast<f32>(sleepers[u + f]);
}

// The product of `axis`'s units' survival and persistence from `from`.
fn units_from(axis: u32, start: u32) -> f32 {
    var p = 1.0;
    for (var i = 0u; i < n_units; i++) {
        let u = unit_base + i * UW;
        if sleepers[u] == axis && sleepers[u + 1u] >= start {
            p *= unit_f(u, 2u) * unit_f(u, 3u);
        }
    }
    return p;
}


fn rdf(i: u32, f: u32) -> f32 {
    return bitcast<f32>(rd(i, f));
}

fn wrf(i: u32, f: u32, v: f32) {
    wr(i, f, bitcast<u32>(v));
}

// The presences the made axis `i` starts from: its draws' (`made`), its
// running survival, rank and blend, and where it lives on at the age.
fn made_axis(i: u32, made: f32, blend: f32) {
    wq(i, B0, one32());
    wq(i, B1, one32());
    wq(i, QRUN, one32());
    wrf(i, MADE, made);
    wrf(i, RUN, 1.0);
    wr(i, RANKF, 0u);
    wr(i, RANK_HI, 0u);
    wrf(i, BLEND, blend);
    wrf(i, LIVING, -1.0);
}
