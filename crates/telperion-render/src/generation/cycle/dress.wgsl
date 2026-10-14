// fn-256 task 3 (DESIGN.md 12.2): the settled tree drawn as the
// pipeline's nodes on the device, stage 2's conversion (`pipeline/
// space.rs`, `convert`) over the node table, a wave of chains a dispatch
// (`chains.wgsl`). One invocation a chain, its axes in the table's
// order, as the conversion takes them, each drawn from the node it
// starts on: a lateral's bearer in the wave before, the root for the
// seed. Each phytomer gets a record: the node it is drawn as (its own, or
// the one it merges into), or none past the thinnest wood; after them,
// one record an axis for a relay's join.

struct Cfg {
    // The chains, a dispatch row's invocations and the wave's first chain.
    n: u32,
    row: u32,
    first: u32,
    // Where this wave's axes and phytomers start among every wave's, and
    // the wave before's.
    axes_at: u32,
    phyts_at: u32,
    prev_axes_at: u32,
    prev_phyts_at: u32,
    // Where this wave's records and the wave before's start in their
    // buffers (past their axes), and their first ids.
    out_records: u32,
    prev_records: u32,
    out_base: u32,
    prev_base: u32,
    // This wave's phytomers: its joins' records follow them.
    out_count: u32,
    // How many times its natural size the wood is drawn (fn-217 S1).
    // Every decision of the conversion reads natural units.
    scale: f32,
    // The tree's age, the node table's strides and sections, the plan's
    // stride.
    age: u32,
    sk: u32,
    skp: u32,
    at_np: u32,
    stride: u32,
    // Whether the chains are the seed's: drawn from the root.
    seed: u32,
    // How far into its last year the tree is drawn (fn-270 R5a), which
    // the leaves the wood's age keeps move by: 1, the year whole; and
    // whether `leaf` holds each phytomer's leaf share (`present.wgsl`).
    fraction: f32,
    unfolding: u32,
    // The rows a rosette's stem establishes by (`space/establish.rs`,
    // fn-275 R2a): the crown's establishment years (none: dormant) and
    // early share, and its trunk PA's rise a year.
    est_years: f32,
    est_early: f32,
    est_rise: f32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> nt: array<u32>;
@group(0) @binding(2) var<storage, read> plan: array<u32>;
@group(0) @binding(3) var<storage, read> before: array<f32>;
@group(0) @binding(4) var<storage, read_write> drawn: array<f32>;
// Each PA's row (`dress.rs`): its internode and spread, its flowering
// share, the leaves at its tips and their fan, its leaf retention's first
// shedding age and share, and whether it carries the trunk.
@group(0) @binding(5) var<storage, read> rows: array<f32>;
// Each phytomer's leaf's share of its whole size, where `unfolding`.
@group(0) @binding(6) var<storage, read> leaf: array<f32>;

const NONE: u32 = 0xffffffffu;
const SEED: u32 = 0u;
const LATERAL: u32 = 1u;
const CONTINUED: u32 = 2u;
const RELAYED: u32 = 3u;
// The node table's columns (`settle_compact.wgsl`, `settle_state.wgsl`).
const NA_KIND: u32 = 0u;
const NA_PARENT: u32 = 1u;
const NA_NODE: u32 = 2u;
const NA_LINEAGE: u32 = 5u;
const NA_PA: u32 = 7u;
const NA_PHYTS: u32 = 21u;
const NA_PSTART: u32 = 22u;
const NA_BASE: u32 = 24u;
const NA_JOINED: u32 = 33u;
const NP_CYCLE: u32 = 0u;
const NP_KEY: u32 = 1u;
const NP_SCALE: u32 = 10u;
const NP_RADIUS: u32 = 11u;
const NP_TIP: u32 = 12u;
const NP_HEADING: u32 = 15u;
const NP_SIDE: u32 = 18u;
// The plan's columns (`chains.wgsl`).
const LA: u32 = 4u;
const LP: u32 = 5u;
const MEMBERS: u32 = 6u;
const CH_HEAD: u32 = 7u;
const CH_AX: u32 = 8u;
const CH_N: u32 = 10u;
// A PA's row.
const ROW: u32 = 14u;
const R_INTERNODE: u32 = 0u;
const R_SPREAD: u32 = 1u;
const R_FLOWERING: u32 = 2u;
const R_LEAVES: u32 = 3u;
const R_FAN: u32 = 4u;
const R_SHEDS: u32 = 5u;
const R_SHARE: u32 = 6u;
const R_TRUNK: u32 = 7u;
// Its year on the presentation's clock (`unfold::Season`, fn-270 R5c):
// days from the whole age to budburst and to colouring, the days of
// colouring, of falling and of a leaf's expansion, and a leaf's years.
const R_BUDBURST: u32 = 8u;
const R_SENESCENCE: u32 = 9u;
const R_COLOURING: u32 = 10u;
const R_FALL: u32 = 11u;
const R_EXPANSION: u32 = 12u;
const R_LIFE: u32 = 13u;
// `space.rs`'s constants.
const FINEST: f32 = 2e-4;
const SHORTEST: f32 = 1000.0;
const FORK_FROM: f32 = 0.4;
// A drawn axis (`resident/fold.rs`'s `CA`): how many of its phytomers the
// conversion took, the node it ends on, whether it took them all, its
// flags, its origin (none: the tree's own frame), its bearer's distance
// along the wood (none), its depth, and its lineage, which order its
// nodes as the conversion numbers them (`lineage_order`).
const CA: u32 = 12u;
// A drawn phytomer (`resident/fold.rs`'s `CV`): its node's position (y up),
// radius, start radius and base radius, its flags, the node it is drawn
// as, its node's run, its laid axis, its node's parent's position,
// codominance, id, distance along the wood from the root, its parent's
// id, its wood's frame as laid (heading, side; y up), its branch (the id
// of its branch's first node), its key (`tree::node_key`), its wood as
// grown (`Tree::grown`), its phytomer's key (two words) and its age's
// flowering share, at a short shoot's tip its cluster's spread, and its
// leaf retention: the metres of its grown wood before the boundary, at
// the scale, and the retentions before and from it.
const CV: u32 = 38u;
const TAKEN: u32 = 1u;
const MADE: u32 = 2u;
// Every node the dress draws is the device's; one drawn from the root is
// a seam, where the seed's run starts.
const FINE: u32 = 8u;
const SEAM: u32 = 16u;
// A short shoot's node (fn-223), which bears no station; its tip, the
// axis's last drawn node, carries its cluster's leaves from `LEAVES` up.
const CLUSTERED: u32 = 32u;
const TIP: u32 = 64u;
// A node of the stem (`Node::stem`): what the crown's folds leave out.
const STEM: u32 = 128u;
const LEAVES: u32 = 8u;
// A node of the wave before; a run begun there, its base radius carried.
const PREV: u32 = 0x80000000u;
const INHERITED: u32 = 0xfffffffeu;

var<private> chain_at: u32;
var<private> na_: u32;

fn plan_of(c: u32, i: u32) -> u32 {
    return plan[c * cfg.stride + i];
}

fn na(c: u32, a: u32) -> u32 {
    return nt[c * cfg.sk + a];
}

fn naf(c: u32, a: u32) -> f32 {
    return bitcast<f32>(na(c, a));
}

fn np(c: u32, p: u32) -> u32 {
    return nt[cfg.at_np + c * cfg.skp + p];
}

fn npf(c: u32, p: u32) -> f32 {
    return bitcast<f32>(np(c, p));
}

fn np3(c: u32, p: u32) -> vec3<f32> {
    return vec3<f32>(npf(c, p), npf(c + 1u, p), npf(c + 2u, p));
}

// Own axis `i` of the chain: its row in the table.
fn ax_of(i: u32) -> u32 {
    return plan_of(MEMBERS, chain_at + i);
}

fn kind(a: u32) -> u32 {
    return na(NA_KIND, a) & 0xffu;
}

fn pa(a: u32) -> u32 {
    return na(NA_PA, a);
}

fn row(a: u32, r: u32) -> f32 {
    return rows[pa(a) * ROW + r];
}

fn rowu(a: u32, r: u32) -> u32 {
    return bitcast<u32>(row(a, r));
}

// The whole leaves axis `a`'s cluster holds: its age's count drawn by its
// lineage (`telperion_space::cluster`, fn-277); none, it is no short shoot.
fn cluster_of(a: u32) -> u32 {
    let key = na(NA_LINEAGE, a) ^ na(NA_LINEAGE + 1u, a);
    return organs(rowu(a, R_LEAVES), key, ORGAN_CLUSTER);
}

fn count(a: u32) -> u32 {
    return na(NA_PHYTS, a);
}

// Axis `a`'s `k`th phytomer's row among the table's.
fn ph(a: u32, k: u32) -> u32 {
    return na(NA_PSTART, a) + k;
}

// Axis `a`'s laid axis and its `k`th phytomer's record in this wave.
fn la(a: u32) -> u32 {
    return plan_of(LA, a) - cfg.axes_at;
}

fn li(a: u32, k: u32) -> u32 {
    return plan_of(LP, a) - cfg.phyts_at + k;
}

// Own relay `c`'s join (`Axis::joined`): whether it has one, the node of
// its parent's it joins, its share of that node's span, and its radii
// below and above.
fn has_join(c: u32) -> bool {
    return na(NA_JOINED, c) != 0u;
}

fn join_node(c: u32) -> u32 {
    return na(NA_JOINED + 1u, c);
}

fn join_t(c: u32) -> f32 {
    return naf(NA_JOINED + 2u, c);
}

fn at_start(t: f32) -> bool {
    return t <= 0.0;
}

fn at_end(t: f32) -> bool {
    return t >= 1.0;
}

// The record of relay `c`'s join: after the wave's phytomers, by its
// laid axis.
fn join_rec(c: u32) -> u32 {
    return cfg.out_count + la(c);
}

// Whether own axis `c` is a relay of axis `p` that joins it
// (`space/join.rs`): one that grew a phytomer, with a join.
fn joins(c: u32, p: u32) -> bool {
    let a = ax_of(c);
    return na(NA_PARENT, a) == p && kind(a) == RELAYED && count(a) > 0u && has_join(a);
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
    stem: bool,
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
    return Nd(p, at(r, 3u), at(r, 4u), at(r, 5u), run, up, at(r, 13u), id, at(r, 15u), (flags & FINE) != 0u, bitcast<u32>(at(r, 23u)), (flags & STEM) != 0u);
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
fn da(l: u32, f: u32) -> u32 {
    return bitcast<u32>(drawn[l * CA + f]);
}

fn pda(l: u32, f: u32, v: u32) {
    drawn[l * CA + f] = bitcast<f32>(v);
}

// Its tip as laid, lowered as its stem establishes.
fn tip(a: u32, k: u32) -> vec3<f32> {
    return np3(NP_TIP, ph(a, k)) - vec3<f32>(0.0, 0.0, lowering(a, k));
}

// The engine's z up as the pipeline's y up.
fn up(v: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(v.x, v.z, -v.y);
}

// The head of a stopped module, parent `p`'s drawn phytomers from
// `head` on, turned aside from the join `f0` it carried on through
// (`space.rs`, `aside`): off the stem. Only this wave's nodes change.
fn aside(a: u32, f0: u32, head: u32) {
    let taken = da(la(a), 0u);
    var first_at = NONE;
    for (var k = head; k < taken; k++) {
        if getu(li(a, k), 7u) != f0 {
            first_at = k;
            break;
        }
    }
    if first_at == NONE {
        return;
    }
    let first = getu(li(a, first_at), 7u);
    for (var k = first_at; k < taken; k++) {
        let n = getu(li(a, k), 7u);
        if mine(n) {
            putu(n, 8u, first);
            putu(n, 23u, cfg.out_base + first);
            putu(n, 6u, getu(n, 6u) & ~STEM);
        }
    }
    let carrier = node(f0).radius;
    let jp = node(f0).pos;
    var inside = true;
    for (var k = head; k < taken; k++) {
        let n = getu(li(a, k), 7u);
        if n != f0 && length(node(n).pos - jp) >= carrier {
            inside = false;
        }
    }
    let thickest = node(first).radius;
    let scaled = inside && thickest > FORK_FROM * carrier;
    if scaled {
        let scale = FORK_FROM * carrier / thickest;
        for (var k = first_at; k < taken; k++) {
            let n = getu(li(a, k), 7u);
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

// The retention of wood of `cycle` now and a year before (`space.rs`'s
// `Unfolding::kept`): the same at the whole year and on this year's wood.
fn kept(sheds: u32, share: u32, cycle: u32) -> vec2<u32> {
    let now = retention(sheds, share, cfg.age - cycle);
    if cfg.fraction >= 1.0 || cycle >= cfg.age {
        return vec2<u32>(now);
    }
    return vec2<u32>(now, retention(sheds, share, cfg.age - 1u - cycle));
}

// The logistic course over a span (`unfold::course`), and how far one
// `days` long has run `t` days after it began (`unfold::run`).
fn course(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let lo = 1.0 / (1.0 + exp(5.0));
    let hi = 1.0 / (1.0 + exp(-5.0));
    return (1.0 / (1.0 + exp(-10.0 * (x - 0.5))) - lo) / (hi - lo);
}

fn run(t: f32, days: f32) -> f32 {
    if days > 0.0 {
        return course(t / days);
    }
    return select(0.0, 1.0, t > 0.0);
}

// Phytomer `p`'s leaves within the year on axis `a` (`space/unfolding.rs`,
// `tree::Unfolded`): this year's share, a falling leaf's share, how far
// it has coloured, a regrowing one's share, and the share of its wood's
// stations past their lifespan. A gone subtree's leaves (`present.wgsl`
// marks them 2 past their share) fall with its wood and grow none again.
fn season_of(a: u32, p: u32) -> Leaves {
    if cfg.unfolding == 0u {
        return Leaves(1.0, 0.0, 0.0, 0.0, 0u);
    }
    let tau = cfg.fraction * 365.0;
    let marked = leaf[p];
    let gone = marked >= 2.0;
    let grow = select(marked, marked - 2.0, gone);
    let budburst = row(a, R_BUDBURST);
    // The share of the falling leaves gone, each on its own day.
    var fall = run(tau - row(a, R_SENESCENCE) - row(a, R_COLOURING), row(a, R_FALL));
    var regrow = run(tau - budburst, row(a, R_EXPANSION));
    if gone {
        fall = max(fall, 1.0 - grow);
        regrow = 0.0;
    }
    var colour = 0.0;
    if tau < budburst {
        colour = run(tau - row(a, R_SENESCENCE), row(a, R_COLOURING));
    }
    // The wood's age once its leaves have fallen against their lifespan
    // (`unfold::Season::passing`), none on this year's wood.
    var life = 0u;
    let cycle = np(NP_CYCLE, p);
    if cycle < cfg.age {
        let span = 365.0 - budburst + row(a, R_SENESCENCE) + row(a, R_COLOURING) + row(a, R_FALL);
        let fallen = f32(cfg.age - 1u - cycle) + span / 365.0;
        life = u32(floor(clamp(fallen + 1.0 - row(a, R_LIFE), 0.0, 1.0) * 65535.0 + 0.5));
    }
    // A gone subtree's leaves all fall.
    if gone {
        life = 0xffffu;
    }
    return Leaves(grow, fall, colour, regrow, life);
}

// A phytomer's leaves within the year (`season_of`), worked once a
// phytomer in `draw_axis` and written to each record it places: the
// dress's one copy of the work, which keeps its pipeline quick to make.
struct Leaves {
    grow: f32,
    fall: f32,
    colour: f32,
    regrow: f32,
    life: u32,
}

fn seasoned(r: u32, l: Leaves) {
    put(r, 32u, l.grow);
    put(r, 34u, l.fall);
    put(r, 35u, l.colour);
    put(r, 36u, l.regrow);
    putu(r, 37u, l.life);
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
    // The same pieces' retentions a year before (fn-270 R5a): before the
    // change and the latest.
    was_before: u32,
    was_last: u32,
}

// The walk with wood of retention `k` (now, a year before) about to join
// its pending wood.
fn mark(w: Walk, k: vec2<u32>) -> Walk {
    var o = w;
    let q = k.x;
    if o.fresh {
        o.older = 0.0;
        o.before = q;
        o.last = q;
        o.was_before = k.y;
        o.fresh = false;
    } else if q != o.last {
        o.older = o.pending;
        o.before = o.last;
        o.last = q;
        o.was_before = o.was_last;
    }
    o.was_last = k.y;
    return o;
}

// How an axis is drawn: whether it starts its own run (a lateral, or the
// seed from the root), whether it parts as a lateral, whether its base
// stands inside its parent's span (a relay), and whether it is stem.
struct How {
    run: bool,
    lateral: bool,
    relay: bool,
    stem: bool,
}

// A node of axis `a` (`space.rs`'s `convert`, one drawn node): record `r`
// at `at` (position and radius, drawn units), with the frame of its
// phytomer `p` and, where `bloom`, its bloom, key and the wood grown into
// it; merged into the node before where it lies behind it or its wood is
// too short.
fn place(a: u32, r: u32, at: vec4<f32>, p: u32, key: u32, bloom: bool, w: Walk, along: vec3<f32>, how: How, l: Leaves) -> Walk {
    var o = w;
    let pos = at.xyz;
    let thick = at.w;
    let held = node(o.parent);
    let d = pos - held.pos;
    let inside = any(along != vec3<f32>(0.0)) && length(d) < held.radius;
    let behind = !o.made && (dot(d, along) < 0.0 || inside);
    // `SHORTEST` in whole micrometres of grown wood (`space.rs`).
    if behind || floor(o.pending * 1e6 + 0.5) < SHORTEST {
        putu(r, 6u, TAKEN);
        putu(r, 7u, o.parent);
        copy(r, held);
        putu(r, 16u, NONE);
        return o;
    }
    let first = !o.made;
    var start_radius = max(held.radius, thick);
    if how.run && first {
        start_radius = thick;
    }
    if how.run {
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
    putu(r, 33u, (o.was_before << 16u) | o.was_last);
    seasoned(r, l);
    o.fresh = true;
    o.pending = 0.0;
    // A bloom stands at its axis's tip alone (`draw_axis`, fn-253).
    if bloom {
        put(r, 26u, bitcast<f32>(np(NP_KEY, p)));
        put(r, 27u, bitcast<f32>(np(NP_KEY + 1u, p)));
        put(r, 28u, 0.0);
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
    if how.lateral && first {
        codom = codominance(thick, held.radius);
    }
    put(r, 13u, codom);
    putu(r, 14u, cfg.out_base + r);
    put(r, 15u, held.dist + length(d));
    putu(r, 16u, held.id);
    let heading = up(np3(NP_HEADING, p));
    let side = up(np3(NP_SIDE, p));
    for (var c = 0u; c < 3u; c++) {
        put(r, 17u + c, heading[c]);
        put(r, 20u + c, side[c]);
    }
    var seam = 0u;
    if !held.fine {
        seam = SEAM;
    }
    var clustered = 0u;
    if cluster_of(a) > 0u {
        clustered = CLUSTERED;
    }
    putu(r, 6u, TAKEN | MADE | FINE | seam | clustered | select(0u, STEM, how.stem));
    putu(r, 7u, r);
    o.parent = r;
    o.made = true;
    return o;
}

// The next relay of axis `i` joining inside the span of its node `k`
// after share `after` (relay `last` at it), by share then index; `NONE`
// past the last.
fn next_join(i: u32, k: u32, after: f32, last: u32) -> u32 {
    var best = NONE;
    var bt = 2.0;
    for (var c = 1u; c < na_; c++) {
        if !joins(c, i) || join_node(ax_of(c)) != k {
            continue;
        }
        let t = join_t(ax_of(c));
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

// Axis `a` drawn from node `start`, a lateral's first axis or carried
// on; and the joins of its relays where they stand (`space/join.rs`).
fn draw_axis(a: u32, start: u32, how: How) {
    var along = vec3<f32>(0.0);
    if how.relay {
        let p = node(start);
        let span = p.pos - p.up;
        along = span * (1.0 / max(length(span), 1e-12));
    }
    var w = Walk(start, false, NONE, 0.0, true, 0.0, 0u, 0u, 0u, 0u);
    let sheds = rowu(a, R_SHEDS);
    let share = rowu(a, R_SHARE);
    var taken = 0u;
    // Whether the axis's last node was drawn from its own wood, not a
    // relay's join: where its bloom stands (`space.rs`'s `convert`).
    var own = false;
    // The wood grown since the axis's last node: the internodes merged
    // into a node are carried on to the next one drawn.
    let internode = row(a, R_INTERNODE);
    let spread = row(a, R_SPREAD);
    // The leaves at the axis's tip and their fan (fn-223).
    let leaves = cluster_of(a);
    let lineage = vec2<u32>(na(NA_LINEAGE, a), na(NA_LINEAGE + 1u, a));
    for (var k = 0u; k < count(a); k++) {
        let r = li(a, k);
        let p = ph(a, k);
        let l = season_of(a, p);
        var radius = established(a, k, npf(NP_RADIUS, p));
        if !(radius >= FINEST) {
            break;
        }
        // Each internode as laid: its scale and its spread's factor.
        let key = vec2<u32>(np(NP_KEY, p), np(NP_KEY + 1u, p));
        let piece = internode * npf(NP_SCALE, p) * internode_factor(spread, key);
        // Every piece of the span is this phytomer's wood (fn-248).
        let q = kept(sheds, share, np(NP_CYCLE, p));
        var base = vec3<f32>(naf(NA_BASE, a), naf(NA_BASE + 1u, a), naf(NA_BASE + 2u, a) - bearing(a));
        if k > 0u {
            base = tip(a, k - 1u);
        }
        // Relays joining at the node before this span.
        for (var c = 1u; c < na_; c++) {
            if joins(c, a) && join_node(ax_of(c)) == k && at_start(join_t(ax_of(c))) {
                putu(join_rec(ax_of(c)), 7u, w.parent);
            }
        }
        // Then those joining inside it, each its share of the span's wood,
        // the span above them drawn at its own wood.
        var rest = piece;
        var after = 0.0;
        var last = NONE;
        // cap: one join per relay of the chain.
        for (var n = 0u; n < na_; n++) {
            let c = next_join(a, k, after, last);
            if c == NONE {
                break;
            }
            after = join_t(ax_of(c));
            last = c;
            let t = after;
            let ca = ax_of(c);
            radius = min(radius, naf(NA_JOINED + 4u, ca));
            let at = vec4<f32>(up(base + (tip(a, k) - base) * t) * cfg.scale, naf(NA_JOINED + 3u, ca) * cfg.scale);
            w = mark(w, q);
            w.pending += piece * t;
            rest -= piece * t;
            let first = na(NA_PSTART, ca);
            let joined = child(vec2<u32>(np(NP_KEY, first), np(NP_KEY + 1u, first)), KEY_JOIN);
            let held = w.parent;
            w = place(a, join_rec(ca), at, p, node_key(joined, 0u), false, w, along, how, l);
            own = own && w.parent == held;
        }
        // Wood above an inside join thinner than `FINEST` ends the axis at
        // the join, as it does anywhere (`space.rs`'s `convert`).
        let thin = !(radius >= FINEST);
        if !thin {
            taken += 1u;
            w = mark(w, q);
            w.pending += rest;
            let pos = up(tip(a, k)) * cfg.scale;
            let held = w.parent;
            w = place(a, r, vec4<f32>(pos, radius * cfg.scale), p, node_key(lineage, k), true, w, along, how, l);
            own = own || w.parent != held;
        }
        // Relays joining at this span's node.
        for (var c = 1u; c < na_; c++) {
            if joins(c, a) && join_node(ax_of(c)) == k && at_end(join_t(ax_of(c))) {
                putu(join_rec(ax_of(c)), 7u, w.parent);
            }
        }
        if thin {
            break;
        }
    }
    // The axis's own tip keeps its bloom, at its age's flowering share
    // (fn-253), and its cluster (`space.rs`'s `convert`).
    if w.made && own {
        put(w.parent, 28u, row(a, R_FLOWERING));
    }
    if w.made && leaves > 0u {
        putu(w.parent, 6u, getu(w.parent, 6u) | TIP | (leaves << LEAVES));
        put(w.parent, 29u, row(a, R_FAN));
    }
    let l = la(a);
    pda(l, 0u, taken);
    pda(l, 1u, w.parent);
    pda(l, 2u, u32(taken == count(a)));
}

// Whether the PA of axis `a` carries the trunk on (`space.rs`'s `trunk`).
fn trunk(a: u32) -> bool {
    return rowu(a, R_TRUNK) != 0u;
}

@compute @workgroup_size(64)
fn dress(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x + id.y * cfg.row;
    if c >= cfg.n {
        return;
    }
    let chain = cfg.first + c;
    chain_at = plan_of(CH_AX, chain);
    na_ = plan_of(CH_N, chain);
    let h = ax_of(0u);
    // The node the chain starts on: the root, or its head's bearer's
    // phytomer in the wave before.
    var bearer = 0u;
    var bearer_axis = 0u;
    if cfg.seed == 0u {
        let p = na(NA_PARENT, h);
        bearer_axis = plan_of(LA, p) - cfg.prev_axes_at;
        bearer = plan_of(LP, p) - cfg.prev_phyts_at + na(NA_NODE, h);
    }
    let b0 = bearer_axis * CA;
    let held = cfg.prev_records + bearer * CV;
    let taken = (bitcast<u32>(before[held + 6u]) & TAKEN) != 0u;
    let same = bitcast<u32>(before[held + 9u]) == bearer_axis;
    // Each relay's join record: none drawn yet.
    for (var i = 0u; i < na_; i++) {
        let a = ax_of(i);
        let rj = join_rec(a);
        putu(rj, 6u, 0u);
        putu(rj, 7u, NONE);
        putu(rj, 9u, la(a));
    }
    for (var i = 0u; i < na_; i++) {
        let a = ax_of(i);
        let l = la(a);
        drawn[l * CA + 7u] = before[b0 + 7u];
        var depth = bitcast<u32>(before[b0 + 8u]) + 1u;
        if i > 0u {
            depth = da(la(na(NA_PARENT, a)), 8u) + 1u;
        }
        pda(l, 8u, depth);
        pda(l, 9u, na(NA_LINEAGE, a));
        pda(l, 10u, na(NA_LINEAGE + 1u, a));
        pda(l, 0u, 0u);
        pda(l, 1u, NONE);
        pda(l, 2u, 0u);
        for (var k = 0u; k < count(a); k++) {
            putu(li(a, k), 6u, 0u);
            putu(li(a, k), 9u, l);
        }
        if i == 0u {
            // The seed is drawn from the root, its run its own and parting
            // as no lateral; a lateral from its bearer.
            let seed = cfg.seed != 0u;
            if taken && same {
                draw_axis(a, bearer | PREV, How(true, !seed, false, seed && trunk(a)));
            }
            continue;
        }
        let p = na(NA_PARENT, a);
        let lp = la(p);
        let relay = kind(a) == RELAYED;
        // A relay takes its parent's run on from its join, drawn while
        // the parent was (`space.rs`, fn-244); the stopped module's head
        // beyond turns aside from it, its stub.
        if relay && joins(i, p) && getu(join_rec(a), 7u) != NONE {
            let at = getu(join_rec(a), 7u);
            var stub = join_node(a);
            if at_end(join_t(a)) {
                stub += 1u;
            }
            aside(p, at, stub);
            draw_axis(a, at, How(false, false, true, node(at).stem && trunk(a)));
            continue;
        }
        if da(lp, 2u) != 0u {
            let start = da(lp, 1u);
            draw_axis(a, start, How(false, false, relay, node(start).stem && trunk(a)));
        }
    }
    // Each node's base radius, its run's start radius; then each phytomer
    // merged into a node of this wave drawn as it now stands.
    for (var i = 0u; i < na_; i++) {
        let a = ax_of(i);
        for (var k = 0u; k <= count(a); k++) {
            var r = join_rec(a);
            if k < count(a) {
                r = li(a, k);
            }
            let run = getu(r, 8u);
            if (getu(r, 6u) & MADE) != 0u && run != INHERITED && run != NONE {
                put(r, 5u, drawn[cfg.out_records + run * CV + 4u]);
            }
        }
    }
    for (var i = 0u; i < na_; i++) {
        let a = ax_of(i);
        for (var k = 0u; k <= count(a); k++) {
            var r = join_rec(a);
            if k < count(a) {
                r = li(a, k);
            }
            let f = getu(r, 6u);
            let n = getu(r, 7u);
            if (f & TAKEN) != 0u && (f & MADE) == 0u && mine(n) {
                copy(r, node(n));
            }
        }
    }
}
