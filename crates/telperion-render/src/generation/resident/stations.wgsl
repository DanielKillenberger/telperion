// The tree's leaf stations on the device (fn-212 task 6): the bearing
// runs of `plan::bearing_runs` over the nodes `dress.wgsl` drew (`kids`,
// `bear`, `starts`), each run's stations as `plan::runs` counts them
// (`measure`, then `bases` numbers them), and its station segments as
// `prepare_compact_stations` writes them for placement (`emit`): frames
// carried along the run from its attachment's side (host decision,
// 2026-10-07), tiles of 256 stations, contact rings read from the run the
// curve swept each node in. A segment's leaves draw by its run's first
// node's key, but the renderer varies each leaf by its place in the leaf
// buffer, so the runs are numbered by a key, never by the order the
// device runs its threads (fn-261): the runs in their first nodes'
// order. A short shoot's
// tip is a run of its own (fn-223): one segment of its cluster's leaves,
// drawing on the tip's cluster stream, and its wood bears no station.
// Each fine node's florets are counted beside its runs and scanned with
// them, and each bloom that keeps one is written as a record placement
// reads (fn-229, `blooms`). Each run of wood and each tip bears by its
// draw at the cauline share, and each station and cluster leaf carries
// its own leaflets (fn-277). Where the canopy stands a rosette, each stem
// apex that draws a living frond is a run of its own, after every other,
// a segment a frond, its stations the frond's leaflets (`rosette.rs`;
// fn-256, DESIGN.md 12.7.1); an apex that is also a tip or starts a run
// of wood bears that run too, among the wood's (fn-297 R2).

struct Cfg {
    n: u32,
    row: u32,
    spi: u32,
    segments: u32,
    // A fine node's kind and bearing: `STRUCTURAL` of the root's radius
    // and the slender bound in whole micrometres; the twig layer's internode, in whole micrometres as the CPU rounds it; the divergence in
    // radians; where the nodes' contacts start in the table, where leaves
    // are seated (`sweep.wgsl`), else none (0).
    structural: f32,
    slender: u32,
    unit: u32,
    divergence: f32,
    contacts: u32,
    // Whether the canopy bears leaves (bit 0): a tree may flower where it
    // does not; and whether its wood does (`WOOD`, its cauline share).
    leaves: u32,
    // Whether some wood sheds its leaves by age (fn-248): every station
    // keeps its leaf where none does.
    retains: u32,
    // The rosette's divergence, a fixed-point share of a turn
    // (`stations.rs`'s `rosette_turn`).
    turn: u32,
    // The blooms' rows (`bloom.wgsl`).
    flower: Flower,
    // The rosette's rows (`stations.rs`'s `rosette_rows`): fronds, skirt
    // fronds, divergence, depth; pitch, its spread, the skirt's pitch and
    // length; leaflets, rachis length, arch, leaflet pitch; terminal share,
    // spines, their share and pitch; the crown's year (`Crown::rows`):
    // fronds made, a frond's years, whole years before the last (negative where
    // no engine grew it), days into the last; budburst, elongation,
    // expansion, and the fronds it draws (`Crown::slots`); the years the
    // crown takes to make a mature frond, its first frond's share, and the
    // rachis's length over a mature leaflet's (none where it groups none).
    rosette: array<vec4<f32>, 7>,
    // The counts of organs (`stations.rs`'s `organ_rows`, fn-277): the
    // cauline share, the fronds, the dead fronds, the leaflets and the
    // spines as counts' words; the rachis's length; the newest frond's
    // serial at the lesser and the greater whole living count; the fronds
    // drawn at each pair of whole living and dead counts.
    organs: array<vec4<u32>, 3>,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
// The drawn records by id (`fold.rs`'s `Dressed`).
@group(0) @binding(1) var<storage, read> drawn: array<f32>;
@group(0) @binding(2) var<storage, read> table: array<u32>;
@group(0) @binding(3) var<storage, read_write> st: array<atomic<u32>>;
@group(0) @binding(4) var<storage, read_write> runs: array<u32>;
@group(0) @binding(5) var<storage, read_write> counts: array<atomic<u32>>;
@group(0) @binding(6) var<storage, read_write> segs: array<u32>;
// Each node's `OW` words, scanned together: its florets and whether it
// keeps a bloom (counted, then scanned into its first floret and its
// record; fn-229).
@group(0) @binding(7) var<storage, read_write> owners: array<atomic<u32>>;
const OW: u32 = 2u;
// `cfg.leaves`: the leaf-bearing wood and short shoots bear (fn-277).
const WOOD: u32 = 2u;

const NONE: u32 = 0xffffffffu;
const CV: u32 = 38u;
const TW: u32 = 4u;
const MADE: u32 = 2u;
const FINE: u32 = 8u;
// A short shoot's node and its tip, its leaves from `LEAVES` up
// (`dress.wgsl`).
const CLUSTERED: u32 = 32u;
const TIP: u32 = 64u;
// A stem node (`dress.wgsl`), and the mark `kids` leaves on a stem's
// parent above its children's count.
const STEM: u32 = 128u;
// A retained leaf base (`boots.wgsl`): wood hung after the plan's kinds,
// which bears nothing and is no node's child here (fn-266).
const BOOT: u32 = 0x40000000u;
const STEM_KID: u32 = 0x80000000u;
const LEAVES: u32 = 8u;
// A node's station words: its children, its bearing children that are not
// seams and the last of them, and whether it bears.
const SW: u32 = 4u;
// A run: its first node, its stations, its segments, its nodes, its first
// station and first segment, and whether it is a crown.
const RW: u32 = 7u;
// The leaves every run carries, a 64-bit count in two words of `counts`
// (fn-297 R4 B.3): the low word and the carries past it.
const LEAVES_LO: u32 = 6u;
const LEAVES_HI: u32 = 7u;
// A segment, as `data.rs` packs it for `pack.wgsl`.
const GW: u32 = 40u;
const TILE: u32 = 256u;
const PI: f32 = 3.141592653589793;

fn index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

// Word `f` of node `i`'s drawn record (`dress.wgsl`'s `CV`).
fn rec(i: u32, f: u32) -> f32 {
    return drawn[i * CV + f];
}

fn word(i: u32, f: u32) -> u32 {
    return bitcast<u32>(rec(i, f));
}

// Its flags where the dress made it a node; none where it did not.
fn flags(i: u32) -> u32 {
    let f = word(i, 6u);
    return select(0u, f, (f & MADE) != 0u);
}

fn pos(i: u32) -> vec3<f32> {
    return vec3<f32>(rec(i, 0u), rec(i, 1u), rec(i, 2u));
}

fn parent(i: u32) -> u32 {
    return word(i, 16u);
}

// A length in whole micrometres (`plan::micrometres`): grown wood is
// counted and summed in them, exactly.
fn micrometres(metres: f32) -> u32 {
    return u32(floor(metres * 1e6 + 0.5));
}

// The wood grown into node `i` (`plan::walk`).
fn grown(i: u32) -> u32 {
    return micrometres(rec(i, 25u));
}

// The twig layer's internode.
fn unit() -> u32 {
    return cfg.unit;
}

fn metres(micrometres: u32) -> f32 {
    return f32(micrometres) * 1e-6;
}

// A node the device draws stations on: one the dress drew, not the root
// and no retained leaf base.
fn fine(i: u32) -> bool {
    let f = flags(i);
    return (f & MADE) != 0u && (f & FINE) != 0u && (f & BOOT) == 0u;
}

// Whether the canopy stands a rosette, and the fronds each draws, living,
// dead and emerging (`Crown::slots`): none where it stands none, and none
// yet where its tree has made none.
fn bearing() -> bool {
    return cfg.rosette[0].x > 0.0;
}

// Word `i` of the counts of organs.
fn organ_word(i: u32) -> u32 {
    return cfg.organs[i / 4u][i % 4u];
}

// Whether the bearer keyed `key` keeps its leaves at the cauline share
// (`plan::cauline`).
fn cauline(key: u32) -> bool {
    return organs(organ_word(0u), key, ORGAN_CAULINE) == 1u;
}

// The leaves placement `k` of the bearer keyed `key` carries: its
// leaflets, one where it draws one or there is no rachis to string them
// on (`organs::leaflets`).
fn leaves_at(key: u32, k: u32) -> u32 {
    if !grouped() {
        return 1u;
    }
    let n = organs(organ_word(3u), placement_key(key, k), ORGAN_LEAFLETS);
    return select(1u, n, n > 1u);
}

// Whether any placement may carry more than one leaflet.
fn grouped() -> bool {
    return organ_word(3u) > 0x10000u && bitcast<f32>(organ_word(5u)) > 0.0;
}

// The crown an apex draws (`organs::apex`, `Crown`): its stem's key, its
// whole living and dead fronds, its newest frond's serial and the fronds
// it draws.
struct Drawn {
    key: u32,
    living: u32,
    dead: u32,
    newest: i32,
    slots: u32,
}

fn crown_of(apex: u32) -> Drawn {
    let key = word(stem_first(apex), 24u);
    let living = organs(organ_word(1u), key, ORGAN_FRONDS);
    let dead = organs(organ_word(2u), key, ORGAN_SKIRT);
    let more = select(0u, 1u, living > (organ_word(1u) >> 16u));
    let skirt = select(0u, 1u, dead > (organ_word(2u) >> 16u));
    let newest = bitcast<i32>(organ_word(6u + more));
    return Drawn(key, living, dead, newest, organ_word(8u + 2u * more + skirt));
}

// The logistic course over a span (`unfold::course`, `unfold::run`).
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

// The share of its whole size frond `serial` stands at: whole once made
// in a year before this one, one of this year's expanding from its
// emergence (`Crown::unfolded`).
fn unfolded_frond(serial: i32, fronds: f32) -> f32 {
    let y = cfg.rosette[4];
    let s = cfg.rosette[5];
    if y.z < 0.0 || y.y <= 0.0 || fronds <= 0.0 {
        return 1.0;
    }
    let made = (f32(serial) + 1.0) * y.y / fronds;
    if made <= y.z {
        return 1.0;
    }
    if made > y.z + 1.0 {
        return 0.0;
    }
    let emerges = s.x + s.y * min(made - y.z, 1.0);
    return run(y.w - emerges, s.z);
}

// The crown's age when frond `serial` emerged, evenly over its year's
// elongation from budburst; negative with no establishment and on a crown
// no engine grew (`Crown::born`).
fn born_frond(serial: i32, fronds: f32) -> f32 {
    let e = cfg.rosette[6];
    let y = cfg.rosette[4];
    let s = cfg.rosette[5];
    if e.x <= 0.0 || y.z < 0.0 || y.y <= 0.0 || fronds <= 0.0 {
        return -1.0;
    }
    let made = (f32(serial) + 1.0) * y.y / fronds;
    let year = ceil(made) - 1.0;
    return year + (s.x + s.y * (made - year)) / 365.0;
}

// The share of a mature frond's size frond `serial` was made at: the
// crown's age when it emerged along the course from the early share to
// whole over the establishment years (`Crown::established`, fn-275 R2).
fn established_frond(serial: i32, fronds: f32) -> f32 {
    let born = born_frond(serial, fronds);
    if born < 0.0 {
        return 1.0;
    }
    let e = cfg.rosette[6];
    return 1.0 - (1.0 - e.y) * (1.0 - course(born / e.x));
}

// How far frond `serial` was made along the course from one leaflet to
// the mature count, on a log scale (`Crown::pinnate`, fn-275 R2b): whole
// with no establishment.
fn pinnate_frond(serial: i32, fronds: f32) -> f32 {
    let born = born_frond(serial, fronds);
    return select(1.0, course(born / cfg.rosette[6].x), born >= 0.0);
}

// The leaves frond `serial` of the crown keyed `key` carries: the
// leaflets it was made with, by the same course on a log scale from one
// to the mature count, drawn as a count between two whole numbers
// (`Crown::leaflets`, `organs::frond_leaflets`, fn-275 R2b).
fn frond_leaves(key: u32, serial: i32, fronds: f32) -> u32 {
    let f = pinnate_frond(serial, fronds);
    if f >= 1.0 || !grouped() {
        return leaves_at(key, u32(serial));
    }
    let count = pow(max(cfg.rosette[2].x, 1.0), f);
    let n = organs(count_word(count), placement_key(key, u32(serial)), ORGAN_LEAFLETS);
    return select(1u, n, n > 1u);
}

// A young frond as strips of one blade that splits (`Crown::segments`,
// fn-275 R2b): at `f` its leaflet pitch and its spines' count word; where
// each strip stands and how long it is `pack.wgsl`'s `leaflet` reads from
// `f` and the rosette's strap row. The mature rows where it is made
// mature or groups no leaflets.
struct Blade {
    pitch: f32,
    spines: u32,
}

fn segments(f: f32) -> Blade {
    let r = cfg.rosette;
    if f >= 1.0 || r[6].z <= 0.0 {
        return Blade(r[2].w, organ_word(4u));
    }
    let n = pow(r[2].x, f);
    return Blade(r[2].w * f, count_word(r[3].y * n / r[2].x));
}

// `a` a share `t` of the way to `b` (`rosette::fronds`' mix).
fn toward(a: f32, b: f32, t: f32) -> f32 {
    return a + (b - a) * t;
}

// The first node of the stem an apex ends: what keys its fronds, which
// stays as the stem grows (`rosette::stem_key`).
fn stem_first(apex: u32) -> u32 {
    var first = apex;
    // cap: the nodes; no stem is longer.
    let cap = arrayLength(&drawn) / CV;
    for (var n = 0u; n < cap; n++) {
        let p = parent(first);
        // The root bears every stem: never stepped onto.
        let root = parent(p) == NONE || parent(p) == p;
        if p == NONE || p == first || root || (flags(p) & STEM) == 0u {
            break;
        }
        first = p;
    }
    return first;
}

// A stem's apex, which a rosette crowns (`Tree::stem_apices`): a drawn
// stem node no stem continues.
fn apex(i: u32) -> bool {
    let stem = (flags(i) & STEM) != 0u;
    return bearing() && fine(i) && stem && (atomicLoad(&st[i * SW]) & STEM_KID) == 0u && crown_of(i).living > 0u;
}

fn bears(i: u32) -> bool {
    return fine(i) && atomicLoad(&st[i * SW + 3u]) != 0u;
}

// A short shoot's tip the device seats a cluster at.
// None where the canopy bears no leaves, though the tree may flower.
fn tip(i: u32) -> bool {
    let cluster = mix32(word(i, 24u) ^ KEY_CLUSTER.x);
    return (cfg.leaves & WOOD) != 0u && fine(i) && (flags(i) & TIP) != 0u && cauline(cluster);
}

// The 32-bit finaliser (`tree::mix32`).
fn mix32(v: u32) -> u32 {
    var z = v;
    z = (z ^ (z >> 16u)) * 0x21f0aaadu;
    z = (z ^ (z >> 15u)) * 0x735a2d97u;
    return z ^ (z >> 15u);
}

@compute @workgroup_size(64)
fn kids(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n || !fine(i) {
        return;
    }
    atomicAdd(&st[parent(i) * SW], 1u);
    if (flags(i) & STEM) != 0u {
        atomicOr(&st[parent(i) * SW], STEM_KID);
    }
}

@compute @workgroup_size(64)
fn bear(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    // Wood none of which is cauline bears nothing along its length
    // (`plan::bearing_runs`, fn-277).
    if i >= cfg.n || !fine(i) || (cfg.leaves & WOOD) == 0u {
        return;
    }
    let radius = rec(i, 3u);
    let start = rec(i, 4u);
    // A short shoot bears its leaves at its tip alone.
    if (flags(i) & CLUSTERED) != 0u {
        return;
    }
    let twig = radius < cfg.structural && (atomicLoad(&st[i * SW]) & ~STEM_KID) == 0u;
    // In whole micrometres, as the CPU executor compares them (task 10).
    let slender = cfg.slender > 0u && micrometres(max(start, radius)) <= cfg.slender;
    if !(twig || slender) {
        return;
    }
    atomicStore(&st[i * SW + 3u], 1u);
    atomicAdd(&st[parent(i) * SW + 1u], 1u);
    atomicStore(&st[parent(i) * SW + 2u], i);
}

// Whether a bearing run carries on from `p` into its bearing child `c`.
fn continues(p: u32, c: u32) -> bool {
    return bears(p) && word(p, 23u) == word(c, 23u) && atomicLoad(&st[p * SW + 1u]) == 1u;
}

// The run's next node after `at`, or none.
fn next(at: u32) -> u32 {
    if atomicLoad(&st[at * SW + 1u]) != 1u {
        return NONE;
    }
    let c = atomicLoad(&st[at * SW + 2u]);
    if continues(at, c) {
        return c;
    }
    return NONE;
}

// Whether node `i` starts a bearing run, or is a tip's: a run among the
// wood's, whether or not it is also an apex.
fn starts_wood(i: u32) -> bool {
    return (bears(i) && !continues(parent(i), i) && cauline(word(i, 24u))) || tip(i);
}

// The node a drawn node's tree is drawn from: up its parents to the
// first the dress did not draw, the root.
fn stub(i: u32) -> u32 {
    var at = i;
    // cap: the nodes; no path up is longer.
    var cap = arrayLength(&drawn) / CV;
    while (flags(at) & FINE) != 0u && cap > 0u {
        cap -= 1u;
        at = parent(at);
    }
    return at;
}

// Each run's key, its root's stub then its first node, in `runs` (bound
// as the keys), a pair a run in whatever order the threads take; the runs
// counted. A crown's leaves follow every other's, as the CPU places them,
// so an apex's crown is keyed after every root, beside the run of wood or
// the cluster it may also start (fn-297 R2). The keys are sorted before
// `starts` numbers the runs by them.
@compute @workgroup_size(64)
fn keys(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n {
        return;
    }
    if starts_wood(i) {
        keyed(stub(i), i);
    }
    if apex(i) {
        keyed(NONE, i);
    }
}

// A run keyed by `root` then `first`, counted past the keys' room too, so
// the host sees the room it needs.
fn keyed(root: u32, first: u32) {
    let k = atomicAdd(&counts[0], 1u);
    if k < arrayLength(&runs) / 2u {
        runs[2u * k] = root;
        runs[2u * k + 1u] = first;
    }
}

// Each run at its place among the sorted keys, in `segs` (bound as the
// keys): its first node, and whether it is a crown.
@compute @workgroup_size(64)
fn starts(@builtin(global_invocation_id) id: vec3<u32>) {
    let r = index(id);
    if r >= cfg.n {
        return;
    }
    runs[r * RW] = segs[2u * r + 1u];
    runs[r * RW + 6u] = u32(segs[2u * r] == NONE);
}

// Stations standing before `distance` micrometres along a run of
// `internodes`
// (`plan::Run::stations_before`).
fn before(distance: u32, internodes: u32) -> u32 {
    var lo = 0u;
    var hi = internodes;
    // cap: each step halves hi - lo, a u32, so 32 steps at most.
    while lo < hi {
        let mid = lo + (hi - lo) / 2u;
        if mid * unit() < distance {
            lo = mid + 1u;
        } else {
            hi = mid;
        }
    }
    return lo * cfg.spi;
}

// A segment's station range (`plan::Run::segment_stations`).
fn range(j: u32, nodes_n: u32, along: vec2<u32>, internodes: u32, count: u32) -> vec2<u32> {
    var first = 0u;
    if j > 0u {
        first = before(along.x, internodes);
    }
    var last = count;
    if j + 2u != nodes_n {
        last = before(along.y, internodes);
    }
    return vec2<u32>(first, last);
}

// The stream a run's retention draws take (`tree::keeps`): its first
// node's key under the registry's `RETAIN`.
fn retain(first: u32) -> u32 {
    return mix32(word(first, 24u) ^ KEY_RETAIN.x);
}

// Where node `b`'s retention changes, in whole micrometres along its run
// from `along`, where its wood starts (`plan::keeps`).
fn boundary(b: u32, along: u32) -> u32 {
    return along + micrometres(rec(b, 30u));
}

// Whether station `k` on node `b`'s wood, which starts `along` its run,
// keeps its leaf (`plan::kept`): by its retention a year before, which
// holds every station it keeps now (fn-270 R5a).
fn keeps(b: u32, along: u32, k: u32, stream: u32) -> bool {
    let keep = word(b, 33u);
    var q = keep >> 16u;
    if (k / cfg.spi) * unit() >= boundary(b, along) {
        q = keep & 0xffffu;
    }
    if q == 0xffffu {
        return true;
    }
    return (mix32(stream + (k + 1u) * 0x9e3779b9u) >> 16u) < q;
}

// The stations from `from` up to `to` on node `b`'s wood that keep their
// leaves (`plan::Run::kept_between`).
// The leaves they carry, each station its leaflets on the run keyed `key`
// (`plan::Run::leaves_between`, fn-277).
fn kept(b: u32, along: u32, span: vec2<u32>, stream: u32, key: u32) -> u32 {
    if cfg.retains == 0u && !grouped() {
        return span.y - span.x;
    }
    var count = 0u;
    // cap: the span's stations, a tile's at most where it is called.
    for (var k = span.x; k < span.y; k++) {
        if cfg.retains == 0u || keeps(b, along, k, stream) {
            count += leaves_at(key, k);
        }
    }
    return count;
}

// The stations a run of `length` micrometres of wood stands, before any
// sheds its leaf: none on a run with no wood grown (`plan::runs`).
fn internodes_of(length: u32) -> u32 {
    if length == 0u {
        return 0u;
    }
    return max((length + unit() - 1u) / unit(), 1u);
}

// `a + b`, a carry past 32 bits counted in the leaves' high word, so the
// host's instance budget sees the whole count (fn-297 R4 B.3).
fn add_leaves(a: u32, b: u32) -> u32 {
    let sum = a + b;
    if sum < a {
        atomicAdd(&counts[LEAVES_HI], 1u);
    }
    return sum;
}

// Run `r`'s leaves, segments and nodes, its leaves added to every run's.
fn measured(r: u32, leaves: u32, pieces: u32, n: u32) {
    runs[r * RW + 1u] = leaves;
    runs[r * RW + 2u] = pieces;
    runs[r * RW + 3u] = n;
    add_leaves(atomicAdd(&counts[LEAVES_LO], leaves), leaves);
}

@compute @workgroup_size(64)
fn measure(@builtin(global_invocation_id) id: vec3<u32>) {
    let r = index(id);
    if r >= cfg.n {
        return;
    }
    let first = runs[r * RW];
    // A rosette: a segment a frond, a station a leaflet.
    if runs[r * RW + 6u] != 0u {
        let c = crown_of(first);
        var leaves = 0u;
        // cap: the fronds the crown draws.
        for (var i = 0u; i < c.slots; i++) {
            leaves = add_leaves(leaves, frond_leaves(c.key, c.newest - i32(i), f32(c.living)));
        }
        measured(r, leaves, c.slots, 2u);
        return;
    }
    if tip(first) {
        // Each of its leaves its leaflets, on the tip's cluster key.
        let cluster = mix32(word(first, 24u) ^ KEY_CLUSTER.x);
        var leaves = 0u;
        let held = flags(first) >> LEAVES;
        // cap: a cluster's leaves, at most `MAX_CLUSTER_LEAVES`.
        for (var j = 0u; j < held; j++) {
            leaves = add_leaves(leaves, leaves_at(cluster, j));
        }
        measured(r, leaves, 1u, 2u);
        return;
    }
    var length = grown(first);
    var n = 2u;
    var at = first;
    // cap: the nodes; no run is longer (`cfg.n` counts runs here).
    let cap = arrayLength(&drawn) / CV + 1u;
    loop {
        if n > cap {
            break;
        }
        let c = next(at);
        if c == NONE {
            break;
        }
        length += grown(c);
        at = c;
        n += 1u;
    }
    // A run with no wood grown carries nothing (`plan::runs`).
    if length == 0u {
        measured(r, 0u, 0u, n);
        return;
    }
    let internodes = internodes_of(length);
    let count = internodes * cfg.spi;
    // The segments' tiles, walked again: those that keep a leaf, and the
    // leaves kept.
    let stream = retain(first);
    var tiles = 0u;
    var leaves = 0u;
    var b = first;
    var along = 0u;
    for (var j = 0u; j + 1u < n; j++) {
        let step = grown(b);
        let span = range(j, n, vec2<u32>(along, along + step), internodes, count);
        var tile = span.x;
        // cap: tile moves on by at least one each step, to span.y.
        while tile < span.y {
            let end = min(tile + TILE, span.y);
            let k = kept(b, along, vec2<u32>(tile, end), stream, word(first, 24u));
            tiles += u32(k > 0u);
            leaves = add_leaves(leaves, k);
            tile = end;
        }
        along += step;
        b = next(b);
    }
    measured(r, leaves, tiles, n);
}

// Each run's first station and segment, in the runs' order: an exclusive
// scan of the runs' stations and segments, in three passes (fn-212 task
// 9). `bases` scans each block of `LANES` runs and leaves its totals in
// `segs`; `blocks` scans the blocks' totals in one workgroup and leaves
// the whole tree's in `counts`; `spread` adds each block's base to its
// runs. The same numbering as one walk over the runs, to the run.
const LANES: u32 = 256u;
var<workgroup> scan: array<vec2<u32>, 256>;

fn block_of(group: vec3<u32>, groups: vec3<u32>) -> u32 {
    return group.x + group.y * groups.x;
}

// The inclusive scan of `own` across the workgroup, `lane`'s.
fn inclusive(lane: u32, own: vec2<u32>) -> vec2<u32> {
    scan[lane] = own;
    workgroupBarrier();
    for (var step = 1u; step < LANES; step *= 2u) {
        var add = vec2<u32>(0u);
        if lane >= step {
            add = scan[lane - step];
        }
        workgroupBarrier();
        scan[lane] += add;
        workgroupBarrier();
    }
    return scan[lane];
}

@compute @workgroup_size(256)
fn bases(
    @builtin(local_invocation_index) lane: u32,
    @builtin(workgroup_id) group: vec3<u32>,
    @builtin(num_workgroups) groups: vec3<u32>,
) {
    let block = block_of(group, groups);
    let r = block * LANES + lane;
    var own = vec2<u32>(0u);
    if r < cfg.n {
        own = vec2<u32>(runs[r * RW + 1u], runs[r * RW + 2u]);
    }
    let sum = inclusive(lane, own);
    if r < cfg.n {
        runs[r * RW + 4u] = sum.x - own.x;
        runs[r * RW + 5u] = sum.y - own.y;
    }
    if lane == LANES - 1u {
        segs[block * 2u] = sum.x;
        segs[block * 2u + 1u] = sum.y;
    }
}

@compute @workgroup_size(256)
fn blocks(@builtin(local_invocation_index) lane: u32) {
    let n = (cfg.n + LANES - 1u) / LANES;
    let per = (n + LANES - 1u) / LANES;
    let end = min(n, (lane + 1u) * per);
    var own = vec2<u32>(0u);
    for (var k = lane * per; k < end; k++) {
        own += vec2<u32>(segs[k * 2u], segs[k * 2u + 1u]);
    }
    let sum = inclusive(lane, own);
    var base = sum - own;
    for (var k = lane * per; k < end; k++) {
        let total = vec2<u32>(segs[k * 2u], segs[k * 2u + 1u]);
        segs[k * 2u] = base.x;
        segs[k * 2u + 1u] = base.y;
        base += total;
    }
    if lane == LANES - 1u {
        atomicStore(&counts[1], sum.x);
        atomicStore(&counts[2], sum.y);
    }
}

@compute @workgroup_size(256)
fn spread(
    @builtin(local_invocation_index) lane: u32,
    @builtin(workgroup_id) group: vec3<u32>,
    @builtin(num_workgroups) groups: vec3<u32>,
) {
    let block = block_of(group, groups);
    let r = block * LANES + lane;
    if r < cfg.n {
        runs[r * RW + 4u] += segs[block * 2u];
        runs[r * RW + 5u] += segs[block * 2u + 1u];
    }
}

fn perpendicular(v: vec3<f32>) -> vec3<f32> {
    let a = abs(v);
    var reference = vec3<f32>(0.0, 0.0, 1.0);
    if a.x <= a.y && a.x <= a.z {
        reference = vec3<f32>(1.0, 0.0, 0.0);
    } else if a.y <= a.z {
        reference = vec3<f32>(0.0, 1.0, 0.0);
    }
    return normalize(cross(reference, v));
}

fn rotate(v: vec3<f32>, axis: vec3<f32>, s: f32, c: f32) -> vec3<f32> {
    return v * c + cross(axis, v) * s + axis * (dot(axis, v) * (1.0 - c));
}

// `station::transport_normal`.
fn transport(normal: vec3<f32>, previous: vec3<f32>, tangent: vec3<f32>) -> vec3<f32> {
    let x = cross(previous, tangent);
    let d = clamp(dot(previous, tangent), -1.0, 1.0);
    if d < -1.0 + 1.2e-7 {
        var axis = vec3<f32>(0.0, -previous.z, previous.y);
        if abs(previous.x) > abs(previous.z) {
            axis = vec3<f32>(-previous.y, previous.x, 0.0);
        }
        return rotate(normal, normalize(axis), 0.0, -1.0);
    }
    if dot(x, x) > 0.0 {
        let s = length(x);
        let norm = sqrt(s * s + d * d);
        return rotate(normal, x / s, s / norm, d / norm);
    }
    return normal;
}

fn put4(at: u32, v: vec4<f32>) {
    for (var c = 0u; c < 4u; c++) {
        segs[at + c] = bitcast<u32>(v[c]);
    }
}

// The contact rings of `node` as vertices of the fine block's compact
// rings: its lower and upper ring, then its run's first and last.
fn contact(node: u32) -> vec4<u32> {
    if cfg.contacts == 0u || table[cfg.contacts + 2u * node] == 0u {
        return vec4<u32>(0u);
    }
    let i = table[cfg.contacts + 2u * node];
    let s = table[cfg.contacts + 2u * node + 1u];
    let at = table[s * TW];
    let count = table[s * TW + 2u];
    let base = at * cfg.segments + 2u * s;
    return vec4<u32>(base + (i - 1u) * cfg.segments, base + i * cfg.segments, base, base + (count - 1u) * cfg.segments);
}

// The fronds of the rosette crowning `apex`, a segment each from `g`
// on, its leaflets from station `station0` on: `rosette::fronds`' spiral.
// A frond's segment, for `pack.wgsl`: where it leaves the axis and its
// rachis length; the radial it leans out along and its size share; its
// heading and its spines' share; the apex's axis (every leaflet kept);
// the rachis's arch, the leaflet pitch, the terminal share and the
// spines' pitch; whether it withers, the frond's kind (-2), its spines;
// its first leaflet, leaflets and key.
fn rosette(apex: u32, station0: u32, g: u32) {
    let at = pos(apex);
    let along = at - pos(parent(apex));
    var axis = vec3<f32>(0.0, 1.0, 0.0);
    if dot(along, along) > 1e-18 {
        axis = normalize(along);
    }
    // `rosette::frame`.
    var normal = vec3<f32>(0.0, 1.0, 0.0) - axis * axis.y;
    if dot(normal, normal) > 1e-12 {
        normal = normalize(normal);
    } else {
        normal = perpendicular(axis);
    }
    let binormal = cross(axis, normal);
    let r = cfg.rosette;
    // The apex's own whole counts (fn-277).
    let c = crown_of(apex);
    let fronds = f32(c.living);
    let living = c.living;
    let oldest = f32(max(living, 1u) - 1u);
    let steps = max(oldest, 1.0);
    let total = fronds + f32(c.dead);
    let newest = c.newest;
    let stream = mix32(c.key ^ KEY_ROSETTE.x);
    var first = station0;
    // Each frond at its place on the spiral, newest first, by its serial
    // among the fronds made, a place further down for every frond grown
    // above it (`rosette::fronds`).
    var above = 0.0;
    for (var i = 0u; i < c.slots; i++) {
        let serial = newest - i32(i);
        let grown = unfolded_frond(serial, fronds);
        let made_at = established_frond(serial, fronds);
        let k = above;
        above += grown;
        let turn = f32(u32(serial) * cfg.turn) * (6.283185307179586 / 4294967296.0);
        let radial = normal * cos(turn) + binormal * sin(turn);
        var age = 0.0;
        if oldest > 0.0 {
            age = min(k, oldest) / oldest;
        }
        // From living to dead over its last living place, and faded over
        // the skirt's last.
        var turned = 0.0;
        if c.dead > 0u {
            turned = clamp(k - oldest, 0.0, 1.0);
        }
        let pitch = toward(r[1].x + r[1].y * age, r[1].z, turned);
        let depth = toward(r[0].w * age, r[0].w * (max(k, fronds) / steps), turned);
        let length = toward(1.0, r[1].w, turned);
        let share = length * grown * made_at * clamp(total - k, 0.0, 1.0);
        var split = pinnate_frond(serial, fronds);
        if cfg.rosette[6].z <= 0.0 {
            split = 1.0;
        }
        let blade = segments(split);
        let heading = normalize(axis * cos(pitch) + radial * sin(pitch));
        let s = (g + i) * GW;
        put4(s, vec4<f32>(at - axis * depth, r[2].y * share));
        put4(s + 4u, vec4<f32>(radial, share));
        put4(s + 8u, vec4<f32>(heading, r[3].z));
        put4(s + 12u, vec4<f32>(axis, 0.0));
        put4(s + 16u, vec4<f32>(r[2].z, blade.pitch, r[3].x, r[3].w));
        // Its own leaflets and spines (`organs::placement`, fn-277).
        let leaflets = frond_leaves(c.key, serial, fronds);
        let spines = organs(blade.spines, placement_key(c.key, u32(serial)), ORGAN_SPINES);
        put4(s + 20u, vec4<f32>(f32(turned >= 0.5), -2.0, f32(spines), split));
        segs[s + 15u] = NONE;
        segs[s + 24u] = first;
        segs[s + 25u] = leaflets;
        owns(g + i, apex);
        first += leaflets;
        segs[s + 26u] = 0u;
        segs[s + 27u] = mix32(stream + (u32(serial) + 1u) * 0x9e3779b9u);
        for (var c = 0u; c < 4u; c++) {
            segs[s + 28u + c] = 0u;
        }
        // Its strips' blade over a mature leaflet (`pack.wgsl`'s `leaflet`).
        segs[s + 28u] = bitcast<u32>(cfg.rosette[6].z);
        // A frond is drawn whole (fn-270 R5).
        unfolded(s, NONE, NONE, 0u);
    }
}

// The node segment `g` stands on, after every segment's words: the
// field reads its limb system (fn-257).
fn owns(g: u32, node: u32) {
    segs[atomicLoad(&counts[2u]) * GW + g] = node;
}

// A segment's leaves within the year (`place.wgsl`'s `Segment`, fn-270
// R5, R5c), as node `n` holds them: this year's share, the retentions now
// on node `b`'s wood (kept where `b` is none), the share of falling leaves
// gone, a regrowing one's share; how far a falling one has coloured, the
// share passing its lifespan and the streams its lifespan's and its
// abscission's draws take, keyed `key` (`data.rs`'s `lived`, `abscised`).
// Whole where `n` is none.
fn unfolded(at: u32, n: u32, b: u32, key: u32) {
    var now = 0xffffffffu;
    if b != NONE {
        now = word(b, 31u);
    }
    if n == NONE {
        put4(at + 32u, vec4<f32>(1.0, bitcast<f32>(now), 0.0, 0.0));
        put4(at + 36u, vec4<f32>(0.0));
        return;
    }
    put4(at + 32u, vec4<f32>(rec(n, 32u), bitcast<f32>(now), rec(n, 34u), rec(n, 36u)));
    let stream = mix32(key ^ LIFE ^ KEY_RETAIN.x);
    let fall = mix32(key ^ FALL ^ KEY_RETAIN.x);
    put4(at + 36u, vec4<f32>(rec(n, 35u), rec(n, 37u), bitcast<f32>(stream), bitcast<f32>(fall)));
}

// What the lifespan's and the abscission's draws salt a run's stream
// with (`tree::LIFE`, `tree::FALL`).
const LIFE: u32 = 0x1eaf11feu;
const FALL: u32 = 0x0fa11e17u;

@compute @workgroup_size(64)
fn emit(@builtin(global_invocation_id) id: vec3<u32>) {
    let r = index(id);
    if r >= cfg.n {
        return;
    }
    let first = runs[r * RW];
    let count = runs[r * RW + 1u];
    let n = runs[r * RW + 3u];
    let station0 = runs[r * RW + 4u];
    var g = runs[r * RW + 5u];
    if runs[r * RW + 6u] != 0u {
        rosette(first, station0, g);
        return;
    }
    if tip(first) {
        // `prepared::prepare_inner`'s cluster segment.
        let at = g * GW;
        let p = pos(first);
        let radius = rec(first, 3u);
        let h = vec3<f32>(rec(first, 17u), rec(first, 18u), rec(first, 19u));
        var heading = vec3<f32>(0.0, 1.0, 0.0);
        if dot(h, h) > 1e-24 {
            heading = normalize(h);
        }
        put4(at, vec4<f32>(p, radius));
        put4(at + 4u, vec4<f32>(p, radius));
        put4(at + 8u, vec4<f32>(heading, 0.0));
        put4(at + 12u, vec4<f32>(0.0));
        put4(at + 16u, vec4<f32>(0.0));
        // Its spread and its leaves; `count` is their leaflets (fn-277).
        put4(at + 20u, vec4<f32>(0.0, -1.0, rec(first, 29u), f32(flags(first) >> LEAVES)));
        segs[at + 24u] = station0;
        segs[at + 25u] = count;
        owns(g, first);
        segs[at + 26u] = 0u;
        segs[at + 27u] = mix32(word(first, 24u) ^ KEY_CLUSTER.x);
        for (var c = 0u; c < 4u; c++) {
            segs[at + 28u + c] = 0u;
        }
        // Every leaf of a cluster stays.
        segs[at + 15u] = 0xffffffffu;
        unfolded(at, first, NONE, mix32(word(first, 24u) ^ KEY_CLUSTER.x));
        return;
    }
    // The run's stations before any sheds its leaf: its wood walked.
    var grown_all = 0u;
    var w = first;
    for (var j = 0u; j + 1u < n; j++) {
        grown_all += grown(w);
        w = next(w);
    }
    let internodes = internodes_of(grown_all);
    let raw = internodes * cfg.spi;
    let stream = retain(first);
    var seen = 0u;
    let key = word(first, 24u);
    let attach = parent(first);
    let seed = vec3<f32>(rec(attach, 20u), rec(attach, 21u), rec(attach, 22u));
    var a = attach;
    var b = first;
    var along = 0u;
    var previous_segment = vec3<f32>(0.0);
    var previous_tangent = vec3<f32>(0.0);
    var carried = vec3<f32>(0.0);
    for (var j = 0u; j + 1u < n; j++) {
        let delta = pos(b) - pos(a);
        var segment = previous_segment;
        if dot(delta, delta) > 0.0 {
            segment = normalize(delta);
        } else if j == 0u {
            segment = vec3<f32>(0.0, 1.0, 0.0);
        }
        var tangent = segment;
        if j > 0u {
            let sum = previous_segment + segment;
            if dot(sum, sum) > 1e-9 {
                tangent = normalize(sum);
            }
            carried = transport(carried, previous_tangent, tangent);
        } else {
            carried = seed;
        }
        carried -= tangent * dot(carried, tangent);
        if dot(carried, carried) <= 1e-9 {
            carried = perpendicular(tangent);
        }
        carried = normalize(carried);
        previous_segment = segment;
        previous_tangent = tangent;
        var t = tangent;
        var nm = carried;
        let step = length(delta);
        if step > 1e-12 {
            t = delta / step;
            nm -= t * dot(nm, t);
            if dot(nm, nm) <= 1e-12 {
                nm = perpendicular(t);
            }
            nm = normalize(nm);
        }
        let bn = normalize(cross(t, nm));
        let wood = grown(b);
        let span = range(j, n, vec2<u32>(along, along + wood), internodes, raw);
        let edge = contact(b);
        var keep = 0xffffffffu;
        if cfg.retains != 0u {
            keep = word(b, 33u);
        }
        var tile = span.x;
        // cap: tile moves on by at least one each step, to span.y.
        while tile < span.y {
            let tiles = min(span.y - tile, TILE);
            let leaves = kept(b, along, vec2<u32>(tile, tile + tiles), stream, key);
            if leaves == 0u {
                tile += tiles;
                continue;
            }
            let turn = f32(tile / cfg.spi) * cfg.divergence;
            let at = g * GW;
            put4(at, vec4<f32>(pos(a), rec(b, 4u)));
            put4(at + 4u, vec4<f32>(pos(b), rec(b, 3u)));
            put4(at + 8u, vec4<f32>(t, 0.0));
            put4(at + 12u, vec4<f32>(nm, 0.0));
            put4(at + 16u, vec4<f32>(bn, 0.0));
            put4(at + 20u, vec4<f32>(metres(along), metres(wood), sin(turn), cos(turn)));
            segs[at + 24u] = station0 + seen;
            segs[at + 25u] = leaves;
            owns(g, b);
            segs[at + 26u] = tile;
            segs[at + 27u] = key;
            for (var c = 0u; c < 4u; c++) {
                segs[at + 28u + c] = edge[c];
            }
            // Where its stations may shed (`data::segments`): the boundary,
            // the retentions and the stream.
            segs[at + 11u] = boundary(b, along);
            segs[at + 15u] = keep;
            segs[at + 19u] = stream;
            unfolded(at, b, select(NONE, b, cfg.retains != 0u), key);
            seen += leaves;
            g += 1u;
            tile += tiles;
        }
        along += wood;
        a = b;
        b = next(b);
    }
}
