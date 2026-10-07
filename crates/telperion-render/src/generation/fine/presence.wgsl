// The chain grower's reals for geometry (fn-209 F1, tasks 4 and 6): a
// relay's blend from its stop's lead, in f32, and the zone tables
// (`telperion_space::GeometryTables`, packed into `words` at `cfg.geo`
// one f32 a word); and its growth units' records. Every presence is the
// contract's Q32 (`assign.wgsl`); every decision stays on the contract.
// Part of `chain.wgsl`'s shader.

const SPAN_F: f32 = 2.0;
// An unbounded lead or log-odds, as the tables hold one.
const HUGE: f32 = 1e30;
// A zone's block in the tables.
const GZ: u32 = 4u;

fn gf(i: u32) -> f32 {
    return bitcast<f32>(words[cfg.geo + i].x);
}

fn gu(i: u32) -> u32 {
    return words[cfg.geo + i].x;
}

// The zone at table offset `zat`'s block.
fn zone_g(zat: u32) -> u32 {
    let z = (zat - 2u * gu(0u)) / ZW;
    return gu(1u) + GZ * z;
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

// A unit's record: its axis, its index there, its survival and its
// persistence (`presence::Unit`), after the chain's sleeping buds; then
// what its Q32 presences are taken from at the chain's end and those
// presences (`assign.wgsl`).
const UW: u32 = 28u;

fn push_unit(axis: u32, k: u32) -> u32 {
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


fn rdf(i: u32, f: u32) -> f32 {
    return bitcast<f32>(rd(i, f));
}

fn wrf(i: u32, f: u32, v: f32) {
    wr(i, f, bitcast<u32>(v));
}

// The presences the made axis `i` starts from: its draws' (`made`), its
// running survival, rank and blend, and where it lives on at the age.
fn made_axis(i: u32, blend: f32) {
    wq(i, B0, one32());
    wq(i, B1, one32());
    wq(i, QRUN, one32());
    wr(i, RANKF, 0u);
    wr(i, RANK_HI, 0u);
    wrf(i, BLEND, blend);
}
