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
    // How many times its natural size the wood is drawn (fn-217 S1), as
    // the pipeline drew the tree above the cut: positions, radii and grown
    // wood. Every decision of the conversion reads natural units.
    scale: f32,
    // This wave's laid phytomers: its joins' records follow them, one
    // per laid axis (`join_rec`).
    out_count: u32,
    pad1: u32,
    pad2: u32,
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
// grown wood is measured by, with each phytomer's factor as laid.
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
const SA: u32 = 79u;
const S_OWN: u32 = 0u;
const S_START: u32 = 1u;
const S_COUNT: u32 = 2u;
// A relay's join as girth set it (`geometry.wgsl`): its parent's node,
// share, radii below and above, and its key (two words).
const S_JOIN: u32 = 70u;
// Its attachment: the node and its share of that node's span
// (`geometry.wgsl`).
const S_ATTACH: u32 = 76u;
const AL: u32 = 8u;
const PL: u32 = 27u;
const PA: u32 = 2u;
// The geometry table's PA blocks (`geometry.wgsl`).
const GG: u32 = 42u;
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
// above the cut tagged `PREV`), its key (`tree::node_key`), its wood
// as grown (`Tree::grown`): its phytomers' internodes at their scales,
// what its blooms are drawn by (fn-229): its phytomer's key (two words)
// and its age's flowering share, at a short shoot's tip its cluster's
// spread (fn-223), and its leaf retention (fn-248, `tree::Leafing`): the
// metres of its grown wood before the boundary, at the scale, and the
// retentions before and from it.
const CV: u32 = 32u;
const TAKEN: u32 = 1u;
const MADE: u32 = 2u;
// Its root is laid on the CPU (host decision 3): drawn there.
const LANDED: u32 = 4u;
// The node is a fine root's; one drawn from wood above the cut is a seam,
// where a run of its own starts (D4).
const FINE: u32 = 8u;
const SEAM: u32 = 16u;
// A short shoot's node (fn-223), which bears no station; its tip, the
// axis's last drawn node, carries its cluster's leaves from `LEAVES` up.
const CLUSTERED: u32 = 32u;
const TIP: u32 = 64u;
const LEAVES: u32 = 8u;
// A node of the wave before; a run begun there, its base radius carried.
const PREV: u32 = 0x80000000u;
const INHERITED: u32 = 0xfffffffeu;

var<private> ax0: u32;
var<private> sb: u32;
var<private> out_ax: u32;
var<private> out_ph: u32;
var<private> na_: u32;

fn ax(i: u32, f: u32) -> u32 {
    return axes[(ax0 + i) * AXW + f];
}

fn s(i: u32, f: u32) -> u32 {
    return scratch[sb + i * SA + f];
}

fn sfl(i: u32, f: u32) -> f32 {
    return bitcast<f32>(s(i, f));
}

// Own relay `c`'s share of its join's span (`S_ATTACH`): every choice of
// where it joins is taken on it, as the CPU takes it.
fn join_t(c: u32) -> f32 {
    return bitcast<f32>(s(c, S_ATTACH + 1u));
}

fn at_start(t: f32) -> bool {
    return t <= 0.0;
}

fn at_end(t: f32) -> bool {
    return t >= 1.0;
}

// The record of own relay `c`'s join: after the wave's phytomers, by its
// laid axis.
fn join_rec(c: u32) -> u32 {
    return cfg.out_count + out_ax + s(c, S_OWN);
}

// Whether own axis `c` is a relay of own axis `i` that joins it
// (`space/join.rs`): one that grew a phytomer, on a parent with nodes.
fn joins(c: u32, i: u32) -> bool {
    return ax(c, PARENT) == i && (ax(c, KIND) & 0xffu) == RELAYED && s(c, S_OWN) != NONE
        && s(c, S_COUNT) > 0u && s(c, S_JOIN) != NONE;
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

// The head of a stopped module, parent `p`'s drawn phytomers from
// `head` on, turned aside from the join `f0` it carried on through
// (`space.rs`, `aside`). Only this wave's nodes change.
fn aside(p: u32, f0: u32, head: u32) {
    let taken = da(out_ax + s(p, S_OWN), 0u);
    var first_at = NONE;
    for (var k = head; k < taken; k++) {
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
    for (var k = head; k < taken; k++) {
        let n = getu(li(p, k), 7u);
        if n != f0 && length(node(n).pos - jp) >= carrier {
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

// The 16-bit retention of wood `age` years old under its PA's first age
// that sheds and that age's share (`tree::kept_share`).
fn retention(sheds: u32, share: u32, age: u32) -> u32 {
    if age < sheds {
        return 0xffffu;
    }
    if age == sheds {
        return share;
    }
    return 0u;
}

// What drawing an axis carries from one node to the next.
struct Walk {
    parent: u32,
    made: bool,
    run: u32,
    pending: f32,
    // The retentions of the wood merged into the next node (`space.rs`'s
    // `convert`, fn-248): whether none is yet, where the last change falls,
    // the retention before it and the latest.
    fresh: bool,
    older: f32,
    before: u32,
    last: u32,
}

// The walk with wood of retention `q` about to join its pending wood.
fn mark(w: Walk, q: u32) -> Walk {
    var o = w;
    if o.fresh {
        o.older = 0.0;
        o.before = q;
        o.last = q;
        o.fresh = false;
    } else if q != o.last {
        o.older = o.pending;
        o.before = o.last;
        o.last = q;
    }
    return o;
}

// A node of own axis `i` (`space.rs`'s `convert`, one drawn node): record
// `r` at `at` (position and radius, drawn units), with the frame and,
// where `bloom`, the bloom of laid phytomer `lb`, its key and the wood
// grown into it; merged into the node before where it lies behind it or
// its wood is too short.
fn place(i: u32, r: u32, at: vec4<f32>, lb: u32, key: u32, bloom: bool, w: Walk, along: vec3<f32>, lateral: bool, flags: u32) -> Walk {
    var o = w;
    let pos = at.xyz;
    let thick = at.w;
    let held = node(o.parent);
    let d = pos - held.pos;
    let inside = any(along != vec3<f32>(0.0)) && length(d) < held.radius;
    let behind = !o.made && (dot(d, along) < 0.0 || inside);
    // `SHORTEST` in whole micrometres of grown wood (`space.rs`).
    if behind || floor(o.pending * 1e6 + 0.5) < SHORTEST {
        putu(r, 6u, TAKEN | flags);
        putu(r, 7u, o.parent);
        copy(r, held);
        putu(r, 16u, NONE);
        return o;
    }
    let first = !o.made;
    var start_radius = max(held.radius, thick);
    if lateral && first {
        start_radius = thick;
    }
    if lateral {
        if first {
            o.run = r;
        }
        putu(r, 8u, o.run);
        putu(r, 23u, cfg.out_base + o.run);
    } else {
        putu(r, 8u, held.run);
        put(r, 5u, held.base);
        putu(r, 23u, held.branch);
    }
    putu(r, 24u, key);
    put(r, 25u, o.pending * cfg.scale);
    put(r, 30u, o.older * cfg.scale);
    putu(r, 31u, (o.before << 16u) | o.last);
    o.fresh = true;
    o.pending = 0.0;
    if bloom {
        put(r, 26u, out[lb + 24u]);
        put(r, 27u, out[lb + 25u]);
        put(r, 28u, gtab[4u + GG * ax(i, PA) + 37u]);
    }
    put(r, 0u, pos.x);
    put(r, 1u, pos.y);
    put(r, 2u, pos.z);
    put(r, 3u, thick);
    put(r, 4u, start_radius);
    put(r, 10u, held.pos.x);
    put(r, 11u, held.pos.y);
    put(r, 12u, held.pos.z);
    var codom = 0.0;
    if lateral && first {
        codom = codominance(thick, held.radius);
    }
    put(r, 13u, codom);
    putu(r, 14u, cfg.out_base + r);
    put(r, 15u, held.dist + length(d));
    putu(r, 16u, held.id);
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
    var clustered = 0u;
    if u32(gtab[4u + GG * ax(i, PA) + 38u]) > 0u {
        clustered = CLUSTERED;
    }
    putu(r, 6u, TAKEN | MADE | FINE | seam | clustered | flags);
    putu(r, 7u, r);
    o.parent = r;
    o.made = true;
    return o;
}

// The next relay of own axis `i` joining inside the span of its node `k`
// after share `after` (relay `last` at it), by share then index; `NONE`
// past the last.
fn next_join(i: u32, k: u32, after: f32, last: u32) -> u32 {
    var best = NONE;
    var bt = 2.0;
    for (var c = 2u; c < na_; c++) {
        if !joins(c, i) || s(c, S_JOIN) != k {
            continue;
        }
        let t = join_t(c);
        let same = t == after;
        let later = after < t || (same && last != NONE && c > last);
        let first = t < bt || (t == bt && c < best);
        if !at_start(t) && !at_end(t) && later && first {
            best = c;
            bt = t;
        }
    }
    return best;
}

// Own axis `i` (laid `la`) drawn from node `start`, a lateral's first
// axis or carried on; its phytomers from `origin`, and the joins of its
// relays where they stand (`space/join.rs`).
fn draw(i: u32, la: u32, start: u32, lateral: bool, relay: bool, origin: vec3<f32>, flags: u32) {
    var along = vec3<f32>(0.0);
    if relay {
        let p = node(start);
        let span = p.pos - p.up;
        along = span * (1.0 / max(length(span), 1e-12));
    }
    var w = Walk(start, false, NONE, 0.0, true, 0.0, 0u, 0u);
    let block = 4u + GG * ax(i, PA);
    let sheds = bitcast<u32>(gtab[block + 40u]);
    let share = bitcast<u32>(gtab[block + 41u]);
    let age = bitcast<u32>(gtab[1u]);
    var taken = 0u;
    // The wood grown since the axis's last node: the internodes merged
    // into a node are carried on to the next one drawn.
    let internode = gtab[4u + GG * ax(i, PA)];
    // The leaves at the axis's tip and their fan (`laid.rs`, fn-223).
    let leaves = u32(gtab[4u + GG * ax(i, PA) + 38u]);
    let lineage = vec2<u32>(ax(i, 0u), ax(i, 1u));
    let ab = (out_ax + s(i, S_OWN)) * AL;
    for (var k = 0u; k < s(i, S_COUNT); k++) {
        let r = li(i, k);
        var radius = out[cfg.out_phyts + r * PL + 18u];
        if !(radius >= FINEST) {
            break;
        }
        taken += 1u;
        // Each internode as laid: its scale and its spread's factor.
        let lb = cfg.out_phyts + r * PL;
        let piece = internode * out[lb + 19u] * out[lb + 23u];
        // Every piece of the span is this phytomer's wood (fn-248).
        let q = retention(sheds, share, age - bitcast<u32>(out[lb + 26u]));
        var base = vec3<f32>(out[ab + 3u], out[ab + 4u], out[ab + 5u]);
        if k > 0u {
            base = tip(i, k - 1u);
        }
        // Relays joining at the node before this span.
        for (var c = 2u; c < na_; c++) {
            if joins(c, i) && s(c, S_JOIN) == k && at_start(join_t(c)) {
                putu(join_rec(c), 7u, w.parent);
            }
        }
        // Then those joining inside it, each its share of the span's wood,
        // the span above them drawn at its own wood.
        var rest = piece;
        var after = 0.0;
        var last = NONE;
        // cap: one join per relay of the chain.
        for (var n = 0u; n < na_; n++) {
            let c = next_join(i, k, after, last);
            if c == NONE {
                break;
            }
            after = join_t(c);
            last = c;
            let t = after;
            radius = min(radius, sfl(c, S_JOIN + 3u));
            let at = vec4<f32>(up(origin + base + (tip(i, k) - base) * t) * cfg.scale, sfl(c, S_JOIN + 2u) * cfg.scale);
            w = mark(w, q);
            w.pending += piece * t;
            rest -= piece * t;
            let key = node_key(vec2<u32>(s(c, S_JOIN + 4u), s(c, S_JOIN + 5u)), 0u);
            w = place(i, join_rec(c), at, lb, key, false, w, along, lateral, flags);
        }
        w = mark(w, q);
        w.pending += rest;
        let pos = up(origin + tip(i, k)) * cfg.scale;
        w = place(i, r, vec4<f32>(pos, radius * cfg.scale), lb, node_key(lineage, k), true, w, along, lateral, flags);
        // Relays joining at this span's node.
        for (var c = 2u; c < na_; c++) {
            if joins(c, i) && s(c, S_JOIN) == k && at_end(join_t(c)) {
                putu(join_rec(c), 7u, w.parent);
            }
        }
    }
    // The axis's own tip keeps its cluster (`space.rs`'s `convert`).
    if w.made && leaves > 0u {
        putu(w.parent, 6u, getu(w.parent, 6u) | TIP | (leaves << LEAVES));
        put(w.parent, 29u, gtab[4u + GG * ax(i, PA) + 39u]);
    }
    pda(la, 0u, taken);
    pda(la, 1u, w.parent);
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
    na_ = na;
    // Each relay's join record: none drawn yet.
    for (var i = 2u; i < na; i++) {
        if s(i, S_OWN) != NONE {
            let rj = join_rec(i);
            putu(rj, 6u, 0u);
            putu(rj, 7u, NONE);
            putu(rj, 9u, out_ax + s(i, S_OWN));
        }
    }
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
        // A relay takes its parent's run on from its join, drawn while
        // the parent was (`space.rs`, fn-244); the stopped module's head
        // beyond turns aside from it, its stub.
        if relay && joins(i, p) && getu(join_rec(i), 7u) != NONE {
            let at = getu(join_rec(i), 7u);
            var stub = s(i, S_JOIN);
            if at_end(join_t(i)) {
                stub += 1u;
            }
            aside(p, at, stub);
            draw(i, la, at, false, true, origin, flags);
            continue;
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
        for (var k = 0u; k <= s(i, S_COUNT); k++) {
            var r = join_rec(i);
            if k < s(i, S_COUNT) {
                r = li(i, k);
            }
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
        for (var k = 0u; k <= s(i, S_COUNT); k++) {
            var r = join_rec(i);
            if k < s(i, S_COUNT) {
                r = li(i, k);
            }
            let f = getu(r, 6u);
            let n = getu(r, 7u);
            if (f & TAKEN) != 0u && (f & MADE) == 0u && mine(n) {
                copy(r, node(n));
            }
        }
    }
}
