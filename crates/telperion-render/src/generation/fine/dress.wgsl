// Fine roots drawn as the pipeline's nodes in the tree's frame on the
// device (fn-212 task 4): stage 2's conversion (`pipeline/space.rs`,
// `convert`) on each chain of a wave, from its laid buffer and its
// bearer's drawn nodes in the wave before. One invocation per chain, its
// own axes in their order, as the conversion takes a root's. Each laid
// phytomer gets a record: the node it is drawn as (its own, or the one it
// merges into), or none past the thinnest wood.

struct Cfg {
    n: u32,
    row: u32,
    out_phyts: u32,
    prev_phyts: u32,
    out_records: u32,
    prev_records: u32,
    // Where this wave's records and the wave before's start among every
    // drawn record of the group (`curve.wgsl`'s node ids).
    out_base: u32,
    prev_base: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> items: array<u32>;
@group(0) @binding(2) var<storage, read> axes: array<u32>;
@group(0) @binding(3) var<storage, read> scratch: array<u32>;
@group(0) @binding(4) var<storage, read> prev: array<f32>;
@group(0) @binding(5) var<storage, read> out: array<f32>;
@group(0) @binding(6) var<storage, read> before: array<f32>;
@group(0) @binding(7) var<storage, read_write> drawn: array<f32>;
// The geometry table (`laid.rs`): each PA's internode, which a node's
// grown wood is measured by.
@group(0) @binding(8) var<storage, read> gtab: array<f32>;

const NONE: u32 = 0xffffffffu;
const AXW: u32 = 80u;
const IW: u32 = 12u;
const KIND: u32 = 4u;
const PARENT: u32 = 5u;
const NODE: u32 = 6u;
const LATERAL: u32 = 1u;
const CONTINUED: u32 = 2u;
const RELAYED: u32 = 3u;
const SA: u32 = 70u;
const S_OWN: u32 = 0u;
const S_START: u32 = 1u;
const S_COUNT: u32 = 2u;
const AL: u32 = 8u;
const PL: u32 = 23u;
const PA: u32 = 2u;
// The geometry table's PA blocks (`geometry.wgsl`).
const GG: u32 = 25u;
// `space.rs`'s constants.
const FINEST: f32 = 2e-4;
const SHORTEST: f32 = 1000.0;
const FORK_FROM: f32 = 0.4;
// A drawn axis: how many of its phytomers the conversion took, the node
// it ends on, whether it took them all, its root's flags, its root's
// origin (its bearer's node as laid, z up), its bearer's distance
// along the wood from the tree's root, and its depth among its root's
// axes and lineage, which order its nodes as the conversion numbers them
// (`lineage_order`).
const CA: u32 = 12u;
// A drawn phytomer: its node's position (y up), radius, start radius and
// base radius, its flags, the node it is drawn as, its node's run, its
// laid axis, its node's parent's position, codominance, id, distance
// along the wood from its root's bearer, its parent's id, its wood's
// frame as laid (heading, side; y up), which a run it bears seeds from,
// its branch (the id of its branch's first node; a branch of the structure
// above the cut tagged `PREV`), its key (`tree::node_key`) and its wood
// as grown (`Tree::grown`): its phytomers' internodes at their scales.
const CV: u32 = 26u;
const TAKEN: u32 = 1u;
const MADE: u32 = 2u;
// Its root is laid on the CPU (host decision 3): drawn there.
const LANDED: u32 = 4u;
// The node is a fine root's; one drawn from wood above the cut is a seam,
// where a run of its own starts (D4).
const FINE: u32 = 8u;
const SEAM: u32 = 16u;
// A node of the wave before; a run begun there, its base radius carried.
const PREV: u32 = 0x80000000u;
const INHERITED: u32 = 0xfffffffeu;

var<private> ax0: u32;
var<private> sb: u32;
var<private> out_ax: u32;
var<private> out_ph: u32;

fn ax(i: u32, f: u32) -> u32 {
    return axes[(ax0 + i) * AXW + f];
}

fn s(i: u32, f: u32) -> u32 {
    return scratch[sb + i * SA + f];
}

// A node's values as the conversion reads them.
struct Nd {
    pos: vec3<f32>,
    radius: f32,
    start: f32,
    base: f32,
    run: u32,
    up: vec3<f32>,
    codom: f32,
    id: u32,
    dist: f32,
    fine: bool,
    branch: u32,
}

fn rec(r: u32) -> u32 {
    if (r & PREV) != 0u {
        return cfg.prev_records + (r & ~PREV) * CV;
    }
    return cfg.out_records + r * CV;
}

fn at(r: u32, f: u32) -> f32 {
    if (r & PREV) != 0u {
        return before[rec(r) + f];
    }
    return drawn[rec(r) + f];
}

fn node(r: u32) -> Nd {
    let p = vec3<f32>(at(r, 0u), at(r, 1u), at(r, 2u));
    let up = vec3<f32>(at(r, 10u), at(r, 11u), at(r, 12u));
    var run = INHERITED;
    if (r & PREV) == 0u {
        run = bitcast<u32>(drawn[rec(r) + 8u]);
    }
    let flags = bitcast<u32>(at(r, 6u));
    let id = bitcast<u32>(at(r, 14u));
    return Nd(p, at(r, 3u), at(r, 4u), at(r, 5u), run, up, at(r, 13u), id, at(r, 15u), (flags & FINE) != 0u, bitcast<u32>(at(r, 23u)));
}

fn put(r: u32, f: u32, v: f32) {
    drawn[cfg.out_records + r * CV + f] = v;
}

fn putu(r: u32, f: u32, v: u32) {
    drawn[cfg.out_records + r * CV + f] = bitcast<f32>(v);
}

fn getu(r: u32, f: u32) -> u32 {
    return bitcast<u32>(drawn[cfg.out_records + r * CV + f]);
}

// Record `r` drawn as node `n`'s values.
fn copy(r: u32, n: Nd) {
    put(r, 0u, n.pos.x);
    put(r, 1u, n.pos.y);
    put(r, 2u, n.pos.z);
    put(r, 3u, n.radius);
    put(r, 4u, n.start);
    put(r, 5u, n.base);
    put(r, 10u, n.up.x);
    put(r, 11u, n.up.y);
    put(r, 12u, n.up.z);
    put(r, 13u, n.codom);
    putu(r, 23u, n.branch);
    putu(r, 14u, n.id);
    put(r, 15u, n.dist);
    var fine = 0u;
    if n.fine {
        fine = FINE;
    }
    putu(r, 6u, (getu(r, 6u) & ~FINE) | fine);
}

fn mix32(v: u32) -> u32 {
    var z = v;
    z = (z ^ (z >> 16u)) * 0x21f0aaadu;
    z = (z ^ (z >> 15u)) * 0x735a2d97u;
    return z ^ (z >> 15u);
}

// A node's key from its axis's lineage and its place along the axis
// (`tree::node_key`).
fn node_key(lineage: vec2<u32>, place: u32) -> u32 {
    let low = mix32(lineage.x);
    let high = mix32(low ^ lineage.y);
    return mix32(high ^ place);
}

// A lateral as thick as `parent` parts from it as a fork does
// (`space.rs`, `codominance`).
fn codominance(radius: f32, parent: f32) -> f32 {
    return clamp((radius / parent - FORK_FROM) / (0.7 - FORK_FROM), 0.0, 1.0);
}

// Whether `r` is a node of this wave, which the conversion may still
// change.
fn mine(r: u32) -> bool {
    return (r & PREV) == 0u && (getu(r, 6u) & MADE) != 0u;
}

// A drawn axis's field.
fn da(la: u32, f: u32) -> u32 {
    return bitcast<u32>(drawn[la * CA + f]);
}

fn pda(la: u32, f: u32, v: u32) {
    drawn[la * CA + f] = bitcast<f32>(v);
}

// The laid index of own axis `i`'s `k`th phytomer.
fn li(i: u32, k: u32) -> u32 {
    return out_ph + s(i, S_START) + k;
}

// Its root-local tip as laid.
fn tip(i: u32, k: u32) -> vec3<f32> {
    let b = cfg.out_phyts + li(i, k) * PL;
    return vec3<f32>(out[b], out[b + 1u], out[b + 2u]);
}

// The engine's z up as the pipeline's y up.
fn up(v: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(v.x, v.z, -v.y);
}

// The node of parent `p`'s phytomers nearest a relay's base, from `node`
// on (`space.rs`, `joint`): the first of the nearest.
fn joint(i: u32, p: u32, node: u32) -> u32 {
    let a = (out_ax + s(i, S_OWN)) * AL + 3u;
    let base = vec3<f32>(out[a], out[a + 1u], out[a + 2u]);
    var best = node;
    var d = 3.4e38;
    for (var k = node; k < s(p, S_COUNT); k++) {
        let e = length(tip(p, k) - base);
        if e < d {
            d = e;
            best = k;
        }
    }
    return best;
}

// The head of a stopped module, parent `p`'s drawn phytomers from
// `j0` on, turned aside (`space.rs`, `aside`). Only this wave's nodes
// change.
fn aside(p: u32, j0: u32) {
    let taken = da(out_ax + s(p, S_OWN), 0u);
    let f0 = getu(li(p, j0), 7u);
    var first_at = NONE;
    for (var k = j0; k < taken; k++) {
        if getu(li(p, k), 7u) != f0 {
            first_at = k;
            break;
        }
    }
    if first_at == NONE {
        return;
    }
    let first = getu(li(p, first_at), 7u);
    for (var k = first_at; k < taken; k++) {
        let n = getu(li(p, k), 7u);
        if mine(n) {
            putu(n, 8u, first);
            putu(n, 23u, cfg.out_base + first);
        }
    }
    let carrier = node(f0).radius;
    let jp = node(f0).pos;
    var inside = true;
    for (var k = j0 + 1u; k < taken; k++) {
        if length(node(getu(li(p, k), 7u)).pos - jp) >= carrier {
            inside = false;
        }
    }
    let thickest = node(first).radius;
    let scaled = inside && thickest > FORK_FROM * carrier;
    if scaled {
        let scale = FORK_FROM * carrier / thickest;
        for (var k = first_at; k < taken; k++) {
            let n = getu(li(p, k), 7u);
            if mine(n) {
                put(n, 3u, node(n).radius * scale);
            }
        }
    }
    if mine(first) {
        put(first, 4u, node(first).radius);
        // Drawn down to `FORK_FROM`: no fork, by the branch taken.
        put(first, 13u, select(codominance(node(first).radius, carrier), 0.0, scaled));
    }
}

// Own axis `i` (laid `la`) drawn from node `start`, a lateral's first
// axis or carried on; its phytomers from `origin`.
fn draw(i: u32, la: u32, start: u32, lateral: bool, relay: bool, origin: vec3<f32>, flags: u32) {
    var along = vec3<f32>(0.0);
    if relay {
        let p = node(start);
        let span = p.pos - p.up;
        along = span * (1.0 / max(length(span), 1e-12));
    }
    var parent = start;
    var made = false;
    var run = NONE;
    var taken = 0u;
    // The wood grown since the axis's last node: the internodes merged
    // into a node are carried on to the next one drawn.
    let internode = gtab[4u + GG * ax(i, PA)];
    var pending = 0.0;
    for (var k = 0u; k < s(i, S_COUNT); k++) {
        let r = li(i, k);
        let radius = out[cfg.out_phyts + r * PL + 18u];
        if !(radius >= FINEST) {
            break;
        }
        taken += 1u;
        pending += internode * out[cfg.out_phyts + r * PL + 19u];
        let pos = up(origin + tip(i, k));
        let held = node(parent);
        let d = pos - held.pos;
        let inside = any(along != vec3<f32>(0.0)) && length(d) < held.radius;
        let behind = !made && (dot(d, along) < 0.0 || inside);
        // `SHORTEST` in whole micrometres of grown wood (`space.rs`).
        if behind || floor(pending * 1e6 + 0.5) < SHORTEST {
            putu(r, 6u, TAKEN | flags);
            putu(r, 7u, parent);
            copy(r, held);
            putu(r, 16u, NONE);
            continue;
        }
        let first = !made;
        var start_radius = max(held.radius, radius);
        if lateral && first {
            start_radius = radius;
        }
        if lateral {
            if first {
                run = r;
            }
            putu(r, 8u, run);
            putu(r, 23u, cfg.out_base + run);
        } else {
            putu(r, 8u, held.run);
            put(r, 5u, held.base);
            putu(r, 23u, held.branch);
        }
        putu(r, 24u, node_key(vec2<u32>(ax(i, 0u), ax(i, 1u)), k));
        put(r, 25u, pending);
        pending = 0.0;
        put(r, 0u, pos.x);
        put(r, 1u, pos.y);
        put(r, 2u, pos.z);
        put(r, 3u, radius);
        put(r, 4u, start_radius);
        put(r, 10u, held.pos.x);
        put(r, 11u, held.pos.y);
        put(r, 12u, held.pos.z);
        var codom = 0.0;
        if lateral && first {
            codom = codominance(radius, held.radius);
        }
        put(r, 13u, codom);
        putu(r, 14u, cfg.out_base + r);
        put(r, 15u, held.dist + length(d));
        putu(r, 16u, held.id);
        let lb = cfg.out_phyts + r * PL;
        let heading = up(vec3<f32>(out[lb + 3u], out[lb + 4u], out[lb + 5u]));
        let side = up(vec3<f32>(out[lb + 6u], out[lb + 7u], out[lb + 8u]));
        for (var c = 0u; c < 3u; c++) {
            put(r, 17u + c, heading[c]);
            put(r, 20u + c, side[c]);
        }
        var seam = 0u;
        if !held.fine {
            seam = SEAM;
        }
        putu(r, 6u, TAKEN | MADE | FINE | seam | flags);
        putu(r, 7u, r);
        parent = r;
        made = true;
    }
    pda(la, 0u, taken);
    pda(la, 1u, parent);
    pda(la, 2u, u32(taken == s(i, S_COUNT)));
}

@compute @workgroup_size(64)
fn dress(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x + id.y * cfg.row;
    if c >= cfg.n {
        return;
    }
    let it = c * IW;
    ax0 = items[it];
    let na = items[it + 1u];
    out_ax = items[it + 5u];
    out_ph = items[it + 6u];
    sb = items[it + 7u];
    let stub = (items[it + 8u] & 1u) != 0u;
    if na < 2u {
        return;
    }
    let bearer_axis = items[it + 4u];
    let b0 = bearer_axis * CA;
    let origin = vec3<f32>(before[b0 + 4u], before[b0 + 5u], before[b0 + 6u]);
    let flags = bitcast<u32>(before[b0 + 3u]);
    var node_at = 0u;
    if !stub {
        node_at = ax(1u, NODE);
    }
    let bearer = bitcast<u32>(prev[bearer_axis * AL + 7u]) + node_at;
    let held = cfg.prev_records + bearer * CV;
    let taken = (bitcast<u32>(before[held + 6u]) & TAKEN) != 0u;
    let same = bitcast<u32>(before[held + 9u]) == bearer_axis;
    for (var i = 1u; i < na; i++) {
        if s(i, S_OWN) == NONE {
            continue;
        }
        let la = out_ax + s(i, S_OWN);
        drawn[la * CA + 3u] = bitcast<f32>(flags);
        drawn[la * CA + 4u] = origin.x;
        drawn[la * CA + 5u] = origin.y;
        drawn[la * CA + 6u] = origin.z;
        drawn[la * CA + 7u] = before[b0 + 7u];
        var depth = bitcast<u32>(before[b0 + 8u]) + 1u;
        if i > 1u && s(ax(i, PARENT), S_OWN) != NONE {
            depth = bitcast<u32>(drawn[(out_ax + s(ax(i, PARENT), S_OWN)) * CA + 8u]) + 1u;
        }
        pda(la, 8u, depth);
        pda(la, 9u, ax(i, 0u));
        pda(la, 10u, ax(i, 1u));
        pda(la, 0u, 0u);
        pda(la, 1u, NONE);
        pda(la, 2u, 0u);
        for (var k = 0u; k < s(i, S_COUNT); k++) {
            putu(li(i, k), 6u, 0u);
            putu(li(i, k), 9u, la);
        }
        let kind = ax(i, KIND) & 0xffu;
        if i == 1u {
            if taken && same {
                draw(i, la, bearer | PREV, true, false, origin, flags);
            }
            continue;
        }
        let p = ax(i, PARENT);
        if s(p, S_OWN) == NONE {
            continue;
        }
        let lp = out_ax + s(p, S_OWN);
        let relay = kind == RELAYED;
        if relay {
            let bud = ax(i, NODE);
            let j = joint(i, p, bud);
            if j + 1u < s(p, S_COUNT) {
                if j >= da(lp, 0u) {
                    continue;
                }
                // The stem between the bud's node and the joint carries
                // the relay too.
                let girth = node(getu(li(p, bud), 7u)).radius;
                for (var k = bud + 1u; k <= j; k++) {
                    let n = getu(li(p, k), 7u);
                    if mine(n) {
                        put(n, 3u, max(node(n).radius, girth));
                        put(n, 4u, max(node(n).start, girth));
                    }
                }
                aside(p, j);
                draw(i, la, getu(li(p, j), 7u), false, true, origin, flags);
                continue;
            }
        }
        if da(lp, 2u) != 0u {
            draw(i, la, da(lp, 1u), false, relay, origin, flags);
        }
    }
    // Each node's base radius, its run's start radius; then each phytomer
    // merged into a node of this wave drawn as it now stands.
    for (var i = 1u; i < na; i++) {
        if s(i, S_OWN) == NONE {
            continue;
        }
        for (var k = 0u; k < s(i, S_COUNT); k++) {
            let r = li(i, k);
            let run = getu(r, 8u);
            if (getu(r, 6u) & MADE) != 0u && run != INHERITED && run != NONE {
                put(r, 5u, drawn[cfg.out_records + run * CV + 4u]);
            }
        }
    }
    for (var i = 1u; i < na; i++) {
        if s(i, S_OWN) == NONE {
            continue;
        }
        for (var k = 0u; k < s(i, S_COUNT); k++) {
            let r = li(i, k);
            let f = getu(r, 6u);
            let n = getu(r, 7u);
            if (f & TAKEN) != 0u && (f & MADE) == 0u && mine(n) {
                copy(r, node(n));
            }
        }
    }
}
