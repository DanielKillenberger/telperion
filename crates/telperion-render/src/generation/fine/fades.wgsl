// Phase 1's shedding on the device (fn-209 F1 task 6; REALIZED.md, host
// decision 8, step 3): `up` carries each chain's subtree's living from
// the deepest wave to the first (`shed.rs`, `fades`), each chain reading
// its borne buds' chains' living in the next wave at the range the emit
// scan gave it, so the maxima need no atomics; `down` then keeps, from
// the first wave to the deepest, each axis whose bearer is kept and
// whose fade is more than none. Every sum and product is the CPU's, in
// integers, so the shedding is the CPU's to the bit.

struct Cfg {
    n: u32,
    row: u32,
    age: u32,
    pad0: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> runoff: array<u32>;
@group(0) @binding(2) var<storage, read> places: array<u32>;
@group(0) @binding(3) var<storage, read_write> axes: array<u32>;
@group(0) @binding(4) var<storage, read> phyts: array<u32>;
// `up`: each PA's fade delay (Q32) and whether it sheds; `down`: the
// wave's buds' roots and bearers.
@group(0) @binding(5) var<storage, read> info: array<u32>;
// `up`: the next wave's chains' living; `down`: the wave before's kept
// own axes (the first wave's: each root's).
@group(0) @binding(6) var<storage, read> prior: array<u32>;
// `up`: this wave's chains' living; `down`: its kept own axes.
@group(0) @binding(7) var<storage, read_write> mine: array<u32>;

const RW: u32 = 10u;
const CW: u32 = 3u;
const AXW: u32 = 80u;
const KIND: u32 = 4u;
const PARENT: u32 = 5u;
const NODE: u32 = 6u;
const BIRTH: u32 = 3u;
const PA: u32 = 2u;
const SLEEP: u32 = 19u;
const SHED_FLAGS: u32 = 28u;
const QVIG: u32 = 48u;
const QALIVE: u32 = 52u;
const QGROWN: u32 = 54u;
const BEFORE: u32 = 58u;
const GAP: u32 = 60u;
const LIVED: u32 = 62u;
const SALIVE: u32 = 64u;
const FADE: u32 = 66u;
const NSCALE: u32 = 68u;
const LATERAL: u32 = 1u;
const RELAYED: u32 = 3u;
const KEPT: u32 = 1u;

var<private> base: u32;

fn rd(i: u32, f: u32) -> u32 {
    return axes[base + i * AXW + f];
}

fn rq(i: u32, f: u32) -> vec2<u32> {
    return vec2<u32>(rd(i, f), rd(i, f + 1u));
}

fn wq(i: u32, f: u32, v: vec2<u32>) {
    axes[base + i * AXW + f] = v.x;
    axes[base + i * AXW + f + 1u] = v.y;
}

fn max64(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    return select(a, b, lt64(a, b));
}

fn kind_of(i: u32) -> u32 {
    return rd(i, KIND) & 0xffu;
}

fn own(i: u32) -> bool {
    return i == 1u || (i > 1u && kind_of(i) != LATERAL);
}

// Whether axis `i` sheds at all: a lateral or relay of a PA that sheds.
fn sheds(i: u32) -> bool {
    let k = kind_of(i);
    return (k == LATERAL || k == RELAYED) && info[3u * rd(i, PA) + 2u] != 0u;
}

fn chain_of(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

@compute @workgroup_size(64)
fn up(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = chain_of(id);
    if c >= cfg.n || runoff[c * RW + 6u] == 0u {
        return;
    }
    let r = c * RW;
    let wi = runoff[r + 7u];
    let na = runoff[r + 8u];
    base = runoff[r] * AXW;
    // Each axis's own living, a borne bud's from its own chain.
    var k = places[wi * CW + 2u];
    for (var i = 1u; i < na; i++) {
        if own(i) {
            wq(i, LIVED, add64(shl64(w(rd(i, SLEEP), 0u), 12u), rq(i, QGROWN)));
            wq(i, SALIVE, rq(i, QALIVE));
        } else {
            wq(i, LIVED, vec2<u32>(prior[4u * k], prior[4u * k + 1u]));
            wq(i, SALIVE, vec2<u32>(prior[4u * k + 2u], prior[4u * k + 3u]));
            k += 1u;
        }
    }
    // Children follow their parents: one backward pass carries each up.
    for (var i = na - 1u; i >= 2u; i--) {
        let p = rd(i, PARENT);
        var weight = rq(i, QVIG);
        if kind_of(i) == LATERAL {
            weight = times_q(weight, rq(i, NSCALE));
        }
        let carried = carry_q(rq(i, BEFORE), weight, rd(i, GAP), rq(i, LIVED));
        wq(p, LIVED, max64(rq(p, LIVED), carried));
        wq(p, SALIVE, max64(rq(p, SALIVE), times_q(rq(i, SALIVE), weight)));
    }
    for (var i = 1u; i < na; i++) {
        var fade = one32();
        if own(i) && sheds(i) {
            let pa = rd(i, PA);
            let delay = vec2<u32>(info[3u * pa], info[3u * pa + 1u]);
            fade = fade_q(delay, rd(i, BIRTH), rq(i, LIVED), rq(i, SALIVE), cfg.age);
        }
        wq(i, FADE, fade);
    }
    let lived = rq(1u, LIVED);
    let alive = rq(1u, SALIVE);
    mine[4u * wi] = lived.x;
    mine[4u * wi + 1u] = lived.y;
    mine[4u * wi + 2u] = alive.x;
    mine[4u * wi + 3u] = alive.y;
}

@compute @workgroup_size(64)
fn down(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = chain_of(id);
    if c >= cfg.n || runoff[c * RW + 6u] == 0u {
        return;
    }
    let r = c * RW;
    let wi = runoff[r + 7u];
    let na = runoff[r + 8u];
    base = runoff[r] * AXW;
    let above = prior[info[2u * wi + 1u]] != 0u;
    var o = places[wi * CW];
    for (var i = 1u; i < na; i++) {
        if !own(i) {
            continue;
        }
        var kept = above;
        if i > 1u {
            kept = (rd(rd(i, PARENT), SHED_FLAGS) & KEPT) != 0u;
        }
        // `up` left a fade of one on every axis that does not shed.
        kept = kept && !is_zero(rq(i, FADE));
        axes[base + i * AXW + SHED_FLAGS] = select(0u, KEPT, kept);
        mine[o] = select(0u, 1u, kept);
        o += 1u;
    }
}
