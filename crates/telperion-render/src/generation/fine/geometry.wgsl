// A fine root's chains settled on the GPU under realized mechanics
// (fn-209 F1, tasks 4 and 6): each chain of a wave, as the chain grower
// left it in its packed region, scaled, thickened, laid before it bends,
// loaded and laid under its levers (`settle.wgsl`), from its bearer's
// laid frame in the wave before and its borne buds' records in the wave
// after. One invocation per chain; its kept axes and phytomers are
// written to the wave's laid buffer, where the next wave's chains read
// their bearers. Nothing is read back but each root's record.

struct Cfg {
    n: u32,
    row: u32,
    out_phyts: u32,
    prev_phyts: u32,
    age: u32,
    sags: u32,
    // Realized mechanics (`settle.wgsl`): where the next wave's chains'
    // records and this wave's start in `records`.
    prior: u32,
    mine: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> items: array<u32>;
@group(0) @binding(2) var<storage, read> gtab: array<f32>;
@group(0) @binding(3) var<storage, read> axes: array<u32>;
@group(0) @binding(4) var<storage, read> phyts: array<u32>;
@group(0) @binding(5) var<storage, read> prev: array<f32>;
@group(0) @binding(6) var<storage, read_write> out: array<f32>;
@group(0) @binding(7) var<storage, read_write> scratch: array<u32>;
// Each root's origins' heights, two a root; under realized mechanics
// then each wave's chains' records (`settle.wgsl`).
@group(0) @binding(8) var<storage, read_write> records: array<f32>;

const NONE: u32 = 0xffffffffu;
const AXW: u32 = 80u;
const PW: u32 = 11u;
const IW: u32 = 12u;
// The grower's axis fields (`chain.wgsl`).
const LINEAGE: u32 = 0u;
const PA: u32 = 2u;
const BIRTH: u32 = 3u;
const KIND: u32 = 4u;
const PARENT: u32 = 5u;
const NODE: u32 = 6u;
const SLEEP: u32 = 19u;
const NPHYT: u32 = 24u;
const SHED_FLAGS: u32 = 28u;
const RANKF: u32 = 34u;
const RANK_HI: u32 = 39u;
const BLEND: u32 = 35u;
const WHORL: u32 = 37u;
const LATERAL: u32 = 1u;
const CONTINUED: u32 = 2u;
const RELAYED: u32 = 3u;
const IS_WOKEN: u32 = 0x80000u;
// A laid axis: its base before bending and as laid, its PA, and its
// first phytomer's index; a laid phytomer: as laid (tip, heading, side),
// before bending, its radius, scale and rank.
const AL: u32 = 8u;
const PL: u32 = 24u;
// The scratch: per region axis, then per region phytomer.
const SA: u32 = 70u;
const SP: u32 = 12u;
const S_OWN: u32 = 0u;
const S_START: u32 = 1u;
const S_COUNT: u32 = 2u;
const S_SCALE: u32 = 3u;
const S_REACH: u32 = 4u;
const S_TIP: u32 = 5u;
const S_LOAD: u32 = 6u;
const S_WALK: u32 = 10u;
const S_LIFTS: u32 = 21u;
const S_FAR: u32 = 54u;
const S_FRAME: u32 = 57u;
// What a relay standing in its parent's first span hands its parent's
// base: a section (`girths`) or a load (`levers`).
const S_BASE: u32 = 66u;
const P_ORDER: u32 = 0u;
const P_NODE: u32 = 1u;
const P_LOAD: u32 = 2u;
const P_LEVER: u32 = 6u;
// The geometry table: a PA's block and a row.
const GG: u32 = 25u;

var<private> ax0: u32;
var<private> ph0: u32;
var<private> sb: u32;
var<private> sp: u32;
var<private> out_ax: u32;
var<private> out_ph: u32;
var<private> status: u32;

fn ax(i: u32, f: u32) -> u32 {
    return axes[(ax0 + i) * AXW + f];
}

fn axf(i: u32, f: u32) -> f32 {
    return bitcast<f32>(ax(i, f));
}

fn kind_of(i: u32) -> u32 {
    return ax(i, KIND) & 0xffu;
}

fn ph(p: u32, f: u32) -> u32 {
    return phyts[(ph0 + p) * PW + f];
}

fn s(i: u32, f: u32) -> u32 {
    return scratch[sb + i * SA + f];
}

fn sf(i: u32, f: u32) -> f32 {
    return bitcast<f32>(s(i, f));
}

fn ss(i: u32, f: u32, v: u32) {
    scratch[sb + i * SA + f] = v;
}

fn ssf(i: u32, f: u32, v: f32) {
    scratch[sb + i * SA + f] = bitcast<u32>(v);
}

fn s3(i: u32, f: u32) -> vec3<f32> {
    return vec3<f32>(sf(i, f), sf(i, f + 1u), sf(i, f + 2u));
}

fn ss3(i: u32, f: u32, v: vec3<f32>) {
    ssf(i, f, v.x);
    ssf(i, f + 1u, v.y);
    ssf(i, f + 2u, v.z);
}

fn pf(p: u32, f: u32) -> f32 {
    return bitcast<f32>(scratch[sp + p * SP + f]);
}

fn spf(p: u32, f: u32, v: f32) {
    scratch[sp + p * SP + f] = bitcast<u32>(v);
}

// The region phytomer at `k` along own axis `i`, and its laid index.
fn at_k(i: u32, k: u32) -> u32 {
    return scratch[sp + (s(i, S_START) + k) * SP + P_ORDER];
}

fn lo(i: u32, k: u32) -> u32 {
    return cfg.out_phyts + (out_ph + s(i, S_START) + k) * PL;
}

fn o3(at: u32) -> vec3<f32> {
    return vec3<f32>(out[at], out[at + 1u], out[at + 2u]);
}

fn wo3(at: u32, v: vec3<f32>) {
    out[at] = v.x;
    out[at + 1u] = v.y;
    out[at + 2u] = v.z;
}

fn p3(at: u32) -> vec3<f32> {
    return vec3<f32>(prev[at], prev[at + 1u], prev[at + 2u]);
}

// A Q52 rank's turn of phyllotaxis under a divergence of `d` turns
// (Q60): its fraction of a whole turn, the whole turns dropped exactly.
fn turns(rank: vec2<u32>, d: vec2<u32>) -> f32 {
    let p = mul128(rank, d);
    let q = (p.z >> 16u) | (p.w << 16u);
    return f32(q) * (1.0 / 4294967296.0);
}

fn rank_of(at: u32) -> vec2<u32> {
    return vec2<u32>(bitcast<u32>(out[at + 20u]), bitcast<u32>(out[at + 21u]));
}

fn rank_f(r: vec2<u32>) -> f32 {
    return (f32(r.y) * 4294967296.0 + f32(r.x)) * (1.0 / 4503599627370496.0);
}

fn axis_rank(i: u32) -> vec2<u32> {
    return vec2<u32>(ax(i, RANKF), ax(i, RANK_HI));
}

// A PA's divergence in turns, Q60.
fn divergence_q(pa: u32) -> vec2<u32> {
    let b = 4u + GG * pa;
    return vec2<u32>(bitcast<u32>(gtab[b + 20u]), bitcast<u32>(gtab[b + 21u]));
}

// The pull carried at scratch `at`'s `k`th place.
fn lift_at(at: u32, k: u32) -> vec4<f32> {
    let b = at + 4u * k;
    return vec4<f32>(bitcast<f32>(scratch[b]), bitcast<f32>(scratch[b + 1u]), bitcast<f32>(scratch[b + 2u]), bitcast<f32>(scratch[b + 3u]));
}

fn pa_of(pa: u32) -> Pa {
    let b = 4u + GG * pa;
    return Pa(gtab[b], gtab[b + 1u], gtab[b + 2u], gtab[b + 3u], gtab[b + 4u], gtab[b + 5u], gtab[b + 6u], gtab[b + 7u], gtab[b + 8u], gtab[b + 9u], gtab[b + 10u], gtab[b + 11u], gtab[b + 12u], gtab[b + 13u], gtab[b + 14u], gtab[b + 15u], gtab[b + 16u], gtab[b + 17u], gtab[b + 18u], gtab[b + 19u], gtab[b + 22u], gtab[b + 23u], gtab[b + 24u]);
}

fn ripe(years: f32, ripening: f32) -> f32 {
    if ripening <= 0.0 {
        return 1.0;
    }
    return min(years / ripening, 1.0);
}

fn dominance(d: f32, lineage: vec2<u32>) -> f32 {
    if d <= 0.0 {
        return 1.0;
    }
    let held_share = pow(unit_of(child(lineage, KEY_DOMINANCE)), 4.0);
    return 1.0 - d * (1.0 - held_share);
}

fn lineage_of(i: u32) -> vec2<u32> {
    return vec2<u32>(ax(i, LINEAGE), ax(i, LINEAGE + 1u));
}
