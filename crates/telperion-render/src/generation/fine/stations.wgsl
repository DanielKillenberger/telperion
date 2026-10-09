// The fine roots' leaf stations on the device (fn-212 task 6): the bearing
// runs of `plan::bearing_runs` over the nodes `dress.wgsl` drew (`kids`,
// `bear`, `starts`), each run's stations as `plan::runs` counts them
// (`measure`, then `bases` numbers them), and its station segments as
// `prepare_compact_stations` writes them for placement (`emit`): frames
// carried along the run from its attachment's side (host decision,
// 2026-10-07), tiles of 256 stations, contact rings read from the run the
// curve swept each node in. A segment's leaves draw by its run's first
// node's key, so the runs may be numbered in any order. A short shoot's
// tip is a run of its own (fn-223): one segment of its cluster's leaves,
// drawing on the tip's cluster stream, and its wood bears no station.
// Each fine node's florets are counted beside its runs and scanned with
// them, and each bloom that keeps one is written as a record placement
// reads (fn-229, `blooms`).

struct Cfg {
    n: u32,
    row: u32,
    spi: u32,
    segments: u32,
    // A fine node's kind and bearing: `STRUCTURAL` of the root's radius
    // and the slender bound in whole micrometres; the twig layer's internode; the divergence in
    // radians; whether leaves are seated.
    structural: f32,
    slender: u32,
    internode: f32,
    divergence: f32,
    contact: u32,
    // Whether the canopy bears leaves: a tree may flower where it does not.
    leaves: u32,
    _b: u32,
    _c: u32,
    // The blooms' rows (`bloom.wgsl`).
    flower: Flower,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> nodes: array<f32>;
@group(0) @binding(2) var<storage, read> table: array<u32>;
@group(0) @binding(3) var<storage, read_write> st: array<atomic<u32>>;
@group(0) @binding(4) var<storage, read_write> runs: array<u32>;
@group(0) @binding(5) var<storage, read_write> counts: array<atomic<u32>>;
@group(0) @binding(6) var<storage, read_write> segs: array<u32>;
// Each node's `OW` words, scanned together: the bearing runs of the fine
// root whose stub it is (counted, then scanned into each root's first
// run), its florets and whether it keeps a bloom (counted, then scanned
// into its first floret and its record; fn-229); then each run's first
// node's stub and its rank among its root's runs (task 10, host decision
// 3: a root's runs, and so its leaves, are numbered together).
@group(0) @binding(7) var<storage, read_write> owners: array<atomic<u32>>;
const OW: u32 = 3u;

const NONE: u32 = 0xffffffffu;
const NW: u32 = 31u;
const TW: u32 = 4u;
const MADE: u32 = 2u;
const LANDED: u32 = 4u;
const FINE: u32 = 8u;
const SEAM: u32 = 16u;
// A short shoot's node and its tip, its leaves from `LEAVES` up
// (`dress.wgsl`).
const CLUSTERED: u32 = 32u;
const TIP: u32 = 64u;
const LEAVES: u32 = 8u;
// A node's station words: its children, its bearing children that are not
// seams and the last of them, and whether it bears.
const SW: u32 = 4u;
// A run: its first node, its stations, its segments, its nodes, its first
// station and first segment.
const RW: u32 = 6u;
// A segment, as `data.rs` packs it for `pack.wgsl`.
const GW: u32 = 32u;
const TILE: u32 = 256u;
const PI: f32 = 3.141592653589793;

fn index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

fn word(i: u32, f: u32) -> u32 {
    return bitcast<u32>(nodes[i * NW + f]);
}

fn pos(i: u32) -> vec3<f32> {
    return vec3<f32>(nodes[i * NW], nodes[i * NW + 1u], nodes[i * NW + 2u]);
}

fn parent(i: u32) -> u32 {
    return word(i, 8u);
}

// A length in whole micrometres (`plan::micrometres`): grown wood is
// counted and summed in them, exactly.
fn micrometres(metres: f32) -> u32 {
    return u32(floor(metres * 1e6 + 0.5));
}

// The wood grown into node `i` (`plan::walk`).
fn grown(i: u32) -> u32 {
    return micrometres(nodes[i * NW + 26u]);
}

// The twig layer's internode.
fn unit() -> u32 {
    return micrometres(cfg.internode);
}

fn metres(micrometres: u32) -> f32 {
    return f32(micrometres) * 1e-6;
}

// A node the device draws stations on: a fine root's, not laid on the CPU.
fn fine(i: u32) -> bool {
    let f = word(i, 9u);
    return (f & MADE) != 0u && (f & FINE) != 0u && (f & LANDED) == 0u;
}

fn bears(i: u32) -> bool {
    return fine(i) && atomicLoad(&st[i * SW + 3u]) != 0u;
}

// A short shoot's tip the device seats a cluster at.
// None where the canopy bears no leaves, though the tree may flower.
fn tip(i: u32) -> bool {
    return cfg.leaves != 0u && fine(i) && (word(i, 9u) & TIP) != 0u;
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
}

@compute @workgroup_size(64)
fn bear(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n || !fine(i) || cfg.leaves == 0u {
        return;
    }
    let radius = nodes[i * NW + 3u];
    let start = nodes[i * NW + 4u];
    // A short shoot bears its leaves at its tip alone.
    if (word(i, 9u) & CLUSTERED) != 0u {
        return;
    }
    let twig = radius < cfg.structural && atomicLoad(&st[i * SW]) == 0u;
    // In whole micrometres, as the CPU executor compares them (task 10).
    let slender = cfg.slender > 0u && micrometres(max(start, radius)) <= cfg.slender;
    if !(twig || slender) {
        return;
    }
    atomicStore(&st[i * SW + 3u], 1u);
    // A fine root's first node never counts among its bearer's bearing
    // children (host decision, 2026-10-07).
    if (word(i, 9u) & SEAM) == 0u {
        atomicAdd(&st[parent(i) * SW + 1u], 1u);
        atomicStore(&st[parent(i) * SW + 2u], i);
    }
}

// Whether a bearing run carries on from `p` into its bearing child `c`.
fn continues(p: u32, c: u32) -> bool {
    return bears(p) && word(p, 24u) == word(c, 24u) && atomicLoad(&st[p * SW + 1u]) == 1u;
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

// Whether node `i` starts a bearing run, or is a tip's.
fn starts_run(i: u32) -> bool {
    return (bears(i) && !continues(parent(i), i)) || tip(i);
}

// The stub a fine node's root is drawn from: up its parents to the first
// node that is not a fine root's.
fn stub(i: u32) -> u32 {
    var at = i;
    // cap: the nodes; no path up is longer.
    var cap = cfg.n;
    while (word(at, 9u) & FINE) != 0u && cap > 0u {
        cap -= 1u;
        at = parent(at);
    }
    return at;
}

// Each run counted on its root's stub, its rank among the root's runs kept.
@compute @workgroup_size(64)
fn owned(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n || !starts_run(i) {
        return;
    }
    let s = stub(i);
    atomicStore(&owners[OW * cfg.n + i], s);
    atomicStore(&owners[(OW + 1u) * cfg.n + i], atomicAdd(&owners[s * OW], 1u));
}

// Each run at its root's first run plus its rank: a root's runs together,
// the roots in their stubs' order.
@compute @workgroup_size(64)
fn starts(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n || !starts_run(i) {
        return;
    }
    let s = atomicLoad(&owners[OW * cfg.n + i]);
    let r = atomicLoad(&owners[s * OW]) + atomicLoad(&owners[(OW + 1u) * cfg.n + i]);
    atomicAdd(&counts[0], 1u);
    runs[r * RW] = i;
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

@compute @workgroup_size(64)
fn measure(@builtin(global_invocation_id) id: vec3<u32>) {
    let r = index(id);
    if r >= cfg.n {
        return;
    }
    let first = runs[r * RW];
    if tip(first) {
        runs[r * RW + 1u] = word(first, 9u) >> LEAVES;
        runs[r * RW + 2u] = 1u;
        runs[r * RW + 3u] = 2u;
        return;
    }
    var length = grown(first);
    var n = 2u;
    var at = first;
    // cap: the nodes; no run is longer.
    loop {
        if n > cfg.n + 1u {
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
        runs[r * RW + 1u] = 0u;
        runs[r * RW + 2u] = 0u;
        runs[r * RW + 3u] = n;
        return;
    }
    let internodes = max((length + unit() - 1u) / unit(), 1u);
    let count = internodes * cfg.spi;
    // The segments' tiles, walked again.
    var tiles = 0u;
    var b = first;
    var along = 0u;
    for (var j = 0u; j + 1u < n; j++) {
        let step = grown(b);
        let span = range(j, n, vec2<u32>(along, along + step), internodes, count);
        if span.y > span.x {
            tiles += (span.y - span.x + TILE - 1u) / TILE;
        }
        along += step;
        b = next(b);
    }
    runs[r * RW + 1u] = count;
    runs[r * RW + 2u] = tiles;
    runs[r * RW + 3u] = n;
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
    let at = word(node, 21u);
    if cfg.contact == 0u || at == NONE {
        return vec4<u32>(0u);
    }
    let i = word(node, 22u);
    let s = word(node, 23u);
    let count = table[s * TW + 2u];
    let base = at * cfg.segments + 2u * s;
    return vec4<u32>(base + (i - 1u) * cfg.segments, base + i * cfg.segments, base, base + (count - 1u) * cfg.segments);
}

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
    if tip(first) {
        // `prepared::prepare_inner`'s cluster segment.
        let at = g * GW;
        let p = pos(first);
        let radius = nodes[first * NW + 3u];
        let h = vec3<f32>(nodes[first * NW + 15u], nodes[first * NW + 16u], nodes[first * NW + 17u]);
        var heading = vec3<f32>(0.0, 1.0, 0.0);
        if dot(h, h) > 1e-24 {
            heading = normalize(h);
        }
        put4(at, vec4<f32>(p, radius));
        put4(at + 4u, vec4<f32>(p, radius));
        put4(at + 8u, vec4<f32>(heading, 0.0));
        put4(at + 12u, vec4<f32>(0.0));
        put4(at + 16u, vec4<f32>(0.0));
        put4(at + 20u, vec4<f32>(0.0, -1.0, nodes[first * NW + 30u], 0.0));
        segs[at + 24u] = station0;
        segs[at + 25u] = count;
        segs[at + 26u] = 0u;
        segs[at + 27u] = mix32(word(first, 25u) ^ KEY_CLUSTER.x);
        for (var c = 0u; c < 4u; c++) {
            segs[at + 28u + c] = 0u;
        }
        return;
    }
    let internodes = count / cfg.spi;
    let key = word(first, 25u);
    let attach = parent(first);
    let seed = vec3<f32>(nodes[attach * NW + 18u], nodes[attach * NW + 19u], nodes[attach * NW + 20u]);
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
        let span = range(j, n, vec2<u32>(along, along + wood), internodes, count);
        let edge = contact(b);
        var tile = span.x;
        // cap: tile moves on by at least one each step, to span.y.
        while tile < span.y {
            let tiles = min(span.y - tile, TILE);
            let turn = f32(tile / cfg.spi) * cfg.divergence;
            let at = g * GW;
            put4(at, vec4<f32>(pos(a), nodes[b * NW + 4u]));
            put4(at + 4u, vec4<f32>(pos(b), nodes[b * NW + 3u]));
            put4(at + 8u, vec4<f32>(t, 0.0));
            put4(at + 12u, vec4<f32>(nm, 0.0));
            put4(at + 16u, vec4<f32>(bn, 0.0));
            put4(at + 20u, vec4<f32>(metres(along), metres(wood), sin(turn), cos(turn)));
            segs[at + 24u] = station0 + tile;
            segs[at + 25u] = tiles;
            segs[at + 26u] = tile;
            segs[at + 27u] = key;
            for (var c = 0u; c < 4u; c++) {
                segs[at + 28u + c] = edge[c];
            }
            g += 1u;
            tile += tiles;
        }
        along += wood;
        a = b;
        b = next(b);
    }
}
