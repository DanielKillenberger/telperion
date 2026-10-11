// The chain grower's reals for geometry (fn-209 F1, tasks 4 and 6): a
// relay's blend from its stop's lead, in f32, and the zone tables
// (`telperion_space::GeometryTables`, packed into `words` at `cfg.geo`
// one f32 a word); and its growth units' records. Every presence is the
// contract's (`assign.wgsl`), and every decision.
// Part of `chain.wgsl`'s shader.

const SPAN_F: f32 = 2.0;
// A zone's block in the tables.
const GZ: u32 = 3u;

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

// A unit's record: its axis, its index there, its survival and its
// persistence (`presence::Unit`), after the chain's sleeping buds; then
// what its presences are taken from at the chain's end and those
// presences (`assign.wgsl`).
const UW: u32 = 28u;

fn push_unit(axis: u32, k: u32) -> u32 {
    if n_units >= cfg.unit_cap {
        overflow |= 256u;
        return NONE;
    }
    let u = unit_base + n_units * UW;
    for (var f = 2u; f < UW; f++) {
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


// The presences the made axis `i` starts from: its draws' (`made`), its
// running survival, rank and blend, and where it lives on at the age.
fn made_axis(i: u32, blend: f32) {
    wq(i, B0, 1.0);
    wq(i, B1, 1.0);
    wq(i, QRUN, 1.0);
    wf(i, RANKF, 0.0);
    wf(i, BLEND, blend);
}
