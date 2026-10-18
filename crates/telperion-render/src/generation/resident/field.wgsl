// The field (fn-257 R8): a read-out of the drawn stage's output to a grid,
// as the renderer reads it. Its records are the drawn nodes as `gather`
// lays them (`sweep.wgsl`'s `NW` words) and the station segments
// (`stations.wgsl`'s `GW` words). Here: each node's limb system
// (`clumping::systems`, by pointer jumps), a frond leaflet's box as
// placement draws it, and the box every record stands in. `field/splat.wgsl`
// writes every record into the cells of the grid it reaches by the CPU's
// cell rule (`pipeline/field/splat.rs`), its reference.

struct Cfg {
    // What this dispatch covers and the invocations a dispatch row holds.
    n: u32,
    row: u32,
    // The drawn nodes, the root's id and the deepest lateral order that
    // opens a limb system; the station segments; which half of the
    // doubled jump words a jump reads; the slots of the table of each
    // cell's leaves by limb system (a power of two).
    nodes: u32,
    root: u32,
    most: u32,
    segs: u32,
    flip: u32,
    slots: u32,
    // The grid's cells a side, thrice, and their count.
    dims: vec4<u32>,
    // The grid's least corner and its cell's side, in metres.
    lo: vec4<f32>,
    // A leaf's reach off its wood (`Plan::reach`): the seat, the rachis
    // and the blade; and a frond's length.
    reach: vec4<f32>,
    // The radius structural wood is at least (`space.rs`'s kinds, the
    // expansion's `bearing_bounds`); then a leaf's draws as placement
    // takes them (`data::config`): its size's variation, its scatter in
    // radians and the leaves' seed (its bits).
    bound: vec4<f32>,
    // A frond leaflet's box in its own frame at its size (`fronds.rs`'s
    // `Shape`): its centre and the leaf's size, its half extents.
    leaf_centre: vec4<f32>,
    leaf_half: vec4<f32>,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> nodes: array<f32>;
@group(0) @binding(2) var<storage, read> segs: array<u32>;
// The work words, `cfg.nodes` each: the widest start radius of a node's
// carrying children and the least id of them so wide, whether the node
// opens a system, and the jumps' pointers and values, two halves each.
// After them the box (three least and three most, ordered), then the
// table's bound, low and high.
@group(0) @binding(3) var<storage, read_write> work: array<atomic<u32>>;
// Two counters (the pairs the table holds, and the shares lost to a sum
// past 32 bits), four words a cell (flags, the thickest radius's bits,
// its leaves in fixed point, low and high), then each cell's most leaves
// of one system and the least system holding them.
@group(0) @binding(4) var<storage, read_write> grid: array<atomic<u32>>;
// The table of each cell's leaves by limb system, its slots' cells, then
// their systems, then their sums.
@group(0) @binding(5) var<storage, read_write> table: array<atomic<u32>>;
// Four words a cell, then the counters.
@group(0) @binding(6) var<storage, read_write> answers: array<vec4<u32>>;

const NONE: u32 = 0xffffffffu;

const NW: u32 = 39u;

const GW: u32 = 40u;

// A retained leaf base (`boots.wgsl`): wood, but no structural node.
const BOOT: u32 = 0x40000000u;

// A record the dress drew a node in (`dress.wgsl`): the rest are none.
const MADE: u32 = 2u;

const WIDE: u32 = 0u;

const CARRIER: u32 = 1u;

const OPENS: u32 = 2u;

const PTR: u32 = 3u;

const VAL: u32 = 5u;

const WORDS: u32 = 7u;

// An answer's flags: wood, foliage.
const WOOD: u32 = 1u;

const FOLIAGE: u32 = 2u;

fn index_of(id: vec3<u32>) -> u32 {
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

fn made(i: u32) -> bool {
    return i == cfg.root || (word(i, 9u) & MADE) != 0u;
}

// A node of structural wood (`space.rs`'s kinds): the root, or wood as
// thick as the bound; a retained leaf base never is.
fn structural(i: u32) -> bool {
    let thick = i == cfg.root || nodes[i * NW + 3u] >= cfg.bound.x;
    return thick && (word(i, 9u) & BOOT) == 0u;
}

fn sibling(i: u32) -> bool {
    return nodes[i * NW + 5u] != 0.0;
}

fn at(region: u32, i: u32) -> u32 {
    return region * cfg.nodes + i;
}

// A node that carries a parent on: a structural child that is no
// codominant sibling (`clumping::systems`).
fn carries(i: u32) -> bool {
    let p = parent(i);
    return i != cfg.root && p < cfg.nodes && made(i) && structural(i) && !sibling(i);
}

// A float's bits, ordered as unsigned words.
fn ord(f: f32) -> u32 {
    let b = bitcast<u32>(f);
    if (b & 0x80000000u) != 0u {
        return ~b;
    }
    return b | 0x80000000u;
}

// Node `i`'s wood: its sweep from its parent (a ball at the root), its
// start and end radii.
struct Sweep {
    a: vec3<f32>,
    b: vec3<f32>,
    start: f32,
    end: f32,
}

fn sweep_of(i: u32) -> Sweep {
    let p = parent(i);
    let b = pos(i);
    let end = nodes[i * NW + 3u];
    if i != cfg.root && p < cfg.nodes {
        return Sweep(pos(p), b, nodes[i * NW + 4u], end);
    }
    return Sweep(b, b, end, end);
}

// Station segment `g`'s six words of floats.
fn segment(g: u32) -> array<vec4<f32>, 6> {
    let s = g * GW;
    var out: array<vec4<f32>, 6>;
    for (var v = 0u; v < 6u; v++) {
        out[v] = vec4<f32>(
            bitcast<f32>(segs[s + 4u * v]),
            bitcast<f32>(segs[s + 4u * v + 1u]),
            bitcast<f32>(segs[s + 4u * v + 2u]),
            bitcast<f32>(segs[s + 4u * v + 3u]),
        );
    }
    return out;
}

// Whether segment `g` is a frond's (`stations.wgsl`'s `rosette`), which
// stands as its leaflets.
fn frond(f: array<vec4<f32>, 6>) -> bool {
    return f[5].y < -1.5;
}

// A leaf segment's reach about its wood (`plan::build`'s descriptors).
fn leaf_reach(f: array<vec4<f32>, 6>) -> f32 {
    return max(f[0].w, f[1].w) * cfg.reach.x + cfg.reach.y + cfg.reach.z;
}

// The limb system of the node segment `g` stands on.
fn system_of(g: u32) -> u32 {
    return atomicLoad(&work[at(VAL + cfg.flip, segs[cfg.segs * GW + g])]);
}

// Whether the ball of `inflation` about `p` meets the tapered sweep
// (`splat.rs`'s `capsule`).
fn contains(a: vec3<f32>, b: vec3<f32>, start: f32, end: f32, p: vec3<f32>, inflation: f32) -> bool {
    let d = b - a;
    let q = p - a;
    let r = start + inflation;
    let dr = end - start;
    let aa = dot(d, d) - dr * dr;
    let bb = dot(q, d) + r * dr;
    var t = 0.0;
    if aa > 0.0 {
        t = clamp(bb / aa, 0.0, 1.0);
    } else if bb > aa * 0.5 {
        t = 1.0;
    }
    let distance = q - d * t;
    let rr = r + dr * t;
    return dot(distance, distance) <= rr * rr;
}

// The share of the segment's length inside the closed cube
// (`Leaf::share_inside`); a point segment counts whole inside.
fn inside(a: vec3<f32>, b: vec3<f32>, c: vec3<f32>, half: f32) -> f32 {
    var t0 = 0.0;
    var t1 = 1.0;
    for (var k = 0u; k < 3u; k++) {
        let d = b[k] - a[k];
        if d == 0.0 {
            if abs(a[k] - c[k]) > half {
                return 0.0;
            }
            continue;
        }
        let ta = (c[k] - half - a[k]) / d;
        let tb = (c[k] + half - a[k]) / d;
        t0 = max(t0, min(ta, tb));
        t1 = min(t1, max(ta, tb));
    }
    return max(t1 - t0, 0.0);
}

fn square_to(v: vec3<f32>) -> vec3<f32> {
    let a = abs(v);
    var reference = vec3<f32>(0.0, 0.0, 1.0);
    if a.x <= a.y && a.x <= a.z {
        reference = vec3<f32>(1.0, 0.0, 0.0);
    } else if a.y <= a.z {
        reference = vec3<f32>(0.0, 1.0, 0.0);
    }
    return normalize(cross(reference, v));
}

fn mix32(v: u32) -> u32 {
    var z = v;
    z = (z ^ (z >> 16u)) * 0x21f0aaadu;
    z = (z ^ (z >> 15u)) * 0x735a2d97u;
    return z ^ (z >> 15u);
}

// `pack.wgsl`'s `random`: the `ordinal`th draw of a run's stream.
fn random(key: u32, ordinal: u32) -> f32 {
    let stream = mix32(bitcast<u32>(cfg.bound.w) ^ 0x2c9e1a7fu ^ key);
    return f32(mix32(stream + (ordinal + 1u) * 0x9e3779b9u)) * (1.0 / 4294967296.0);
}

fn rotate(v: vec3<f32>, axis: vec3<f32>, angle: f32) -> vec3<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return v * c + cross(axis, v) * s + axis * (dot(axis, v) * (1.0 - c));
}

struct Leaflet {
    point: vec3<f32>,
    axis: vec3<f32>,
    tangent: vec3<f32>,
    normal: vec3<f32>,
    share: f32,
}

// `pack.wgsl`'s `on_rachis`.
fn on_rachis(point: vec3<f32>, heading: vec3<f32>, count: u32, j: u32, t: f32, rows: vec4<f32>, borne: vec3<f32>) -> Leaflet {
    var lift = vec3<f32>(0.0, 1.0, 0.0) - heading * heading.y;
    if dot(lift, lift) > 1e-12 {
        lift = normalize(lift);
    } else {
        lift = square_to(heading);
    }
    let length = rows.x;
    let arch = rows.y;
    let at = point + heading * (length * t) + lift * (arch * length * t * t);
    let run = normalize(heading + lift * (2.0 * arch * t));
    var closing = 1.0;
    if j + 1u == count {
        closing = 1.0 - rows.w;
    }
    let hand = select(-1.0, 1.0, j % 2u == 0u);
    var pitch = rows.z;
    var share = 1.0;
    if j < u32(borne.x) && borne.y > 0.0 {
        pitch = borne.z;
        share = borne.y;
    }
    return Leaflet(at, rotate(run, lift, hand * pitch * closing), run, cross(heading, lift), share);
}

// A frond leaflet's box (`fronds.rs`'s `Shape` with its packing margin):
// its midline from `a` to `b`, as wide as `wide` either way along `flank`
// and as thick as `thick` either way along their normal.
struct Blade {
    a: vec3<f32>,
    b: vec3<f32>,
    flank: vec3<f32>,
    wide: f32,
    thick: f32,
}

// Leaflet `j` of frond segment `g` as placement draws it (`pack.wgsl`'s
// `leaflet`, `on_rachis`, `packed`: its scatter and its size on the
// leaves' stream).
fn blade(g: u32, f: array<vec4<f32>, 6>, j: u32) -> Blade {
    let s = g * GW;
    let count = segs[s + 25u];
    let heading = f[2].xyz;
    let grown = f[5].w;
    let mature = bitcast<f32>(segs[s + 28u]);
    var t = f32(j + 1u) / f32(count);
    var stretch = 1.0;
    if grown < 1.0 {
        var station = t;
        if count == 1u {
            station = 0.0;
        }
        t = station * grown;
        stretch = pow((1.0 - t) * mature, 1.0 - grown);
    }
    var point = f[0].xyz;
    var axis = heading;
    var tangent = f[1].xyz;
    var normal = f[3].xyz;
    var share = f[1].w * stretch;
    if count > 1u {
        let rows = vec4<f32>(f[0].w, f[4].x, f[4].y, f[4].z);
        let l = on_rachis(f[0].xyz, heading, count, j, t, rows, vec3<f32>(f[5].z, f[2].w, f[4].w));
        point = l.point;
        axis = l.axis;
        tangent = l.tangent;
        normal = l.normal;
        share *= l.share;
    }
    var face = vec3<f32>(0.0, 1.0, 0.0) - axis * axis.y;
    if dot(face, face) <= 1e-12 {
        face = tangent - axis * dot(tangent, axis);
    }
    if dot(face, face) <= 1e-12 {
        face = normal - axis * dot(normal, axis);
    }
    face = normalize(face);
    var flank = normalize(cross(axis, face));
    let key = segs[s + 27u];
    let scatter = cfg.bound.z;
    var draw = j * select(1u, 4u, scatter > 0.0);
    if scatter > 0.0 {
        let z = random(key, draw) * 2.0 - 1.0;
        let phi = random(key, draw + 1u) * 6.283185307179586;
        let ring = sqrt(max(1.0 - z * z, 0.0));
        let jitter = vec3<f32>(ring * cos(phi), z, ring * sin(phi));
        let angle = scatter * random(key, draw + 2u);
        axis = rotate(axis, jitter, angle);
        face = rotate(face, jitter, angle);
        flank = rotate(flank, jitter, angle);
        draw += 3u;
    }
    let size = cfg.leaf_centre.w * (1.0 + cfg.bound.y * (random(key, draw) * 2.0 - 1.0));
    let k = size * share;
    let c = cfg.leaf_centre.xyz * k;
    let h = cfg.leaf_half.xyz * k;
    let margin = (length(c) + length(h)) * 0.005 + 0.001;
    let centre = point + flank * c.x + axis * c.y + face * c.z;
    let run = axis * (h.y + margin);
    return Blade(centre - run, centre + run, flank, h.x + margin, h.z + margin);
}

@compute @workgroup_size(64)
fn clear(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index_of(id);
    if i >= cfg.nodes {
        return;
    }
    atomicStore(&work[at(WIDE, i)], 0u);
    atomicStore(&work[at(CARRIER, i)], NONE);
}

// Each parent's widest carrying child, by its start radius (the pipe's,
// `Tree::pipe`), then the least id of those so wide.
@compute @workgroup_size(64)
fn widest(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index_of(id);
    if i >= cfg.nodes || !carries(i) {
        return;
    }
    atomicMax(&work[at(WIDE, parent(i))], ord(nodes[i * NW + 4u]));
}

@compute @workgroup_size(64)
fn carrier(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index_of(id);
    if i >= cfg.nodes || !carries(i) {
        return;
    }
    let p = parent(i);
    if ord(nodes[i * NW + 4u]) == atomicLoad(&work[at(WIDE, p)]) {
        atomicMin(&work[at(CARRIER, p)], i);
    }
}

// Whether each node opens a system, and the step of order it adds: a
// root stem opens one of order 0, a lateral one an order deeper, a
// codominant sibling one of its parent's order; the jumps start at the
// parent.
@compute @workgroup_size(64)
fn opens(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index_of(id);
    if i >= cfg.nodes {
        return;
    }
    let p = parent(i);
    if i == cfg.root || p >= cfg.nodes || !made(i) {
        atomicStore(&work[at(OPENS, i)], 0u);
        atomicStore(&work[at(PTR, i)], select(cfg.root, i, i == cfg.root));
        atomicStore(&work[at(VAL, i)], 0u);
        return;
    }
    let stem = p == cfg.root;
    let open = structural(i) && (stem || atomicLoad(&work[at(CARRIER, p)]) != i);
    atomicStore(&work[at(OPENS, i)], u32(open));
    atomicStore(&work[at(PTR, i)], p);
    atomicStore(&work[at(VAL, i)], u32(open && !stem && !sibling(i)));
}

// One jump: each node's order summed over twice the ancestors it had
// summed, its pointer twice as far up (the root points at itself and
// adds none).
@compute @workgroup_size(64)
fn order(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index_of(id);
    if i >= cfg.nodes {
        return;
    }
    let was = cfg.flip;
    let next = 1u - cfg.flip;
    let p = atomicLoad(&work[at(PTR + was, i)]);
    let v = atomicLoad(&work[at(VAL + was, i)]) + atomicLoad(&work[at(VAL + was, p)]);
    atomicStore(&work[at(VAL + next, i)], v);
    atomicStore(&work[at(PTR + next, i)], atomicLoad(&work[at(PTR + was, p)]));
}

// Each node's own system where it opens one at an order no deeper than
// the most, the root's its own; the jumps start again at the parent. A
// system is named by the node that opens it.
@compute @workgroup_size(64)
fn own(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index_of(id);
    if i >= cfg.nodes {
        return;
    }
    let order = atomicLoad(&work[at(VAL + cfg.flip, i)]);
    let opens = atomicLoad(&work[at(OPENS, i)]) != 0u;
    var system = NONE;
    let alone = i == cfg.root || parent(i) >= cfg.nodes || !made(i);
    if alone {
        system = cfg.root;
    } else if opens && order <= cfg.most {
        system = i;
    }
    atomicStore(&work[at(VAL, i)], system);
    atomicStore(&work[at(PTR, i)], select(parent(i), cfg.root, alone));
}

// One jump: a node without a system takes the nearest one its pointer
// found, and its pointer moves twice as far up.
@compute @workgroup_size(64)
fn inherit(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index_of(id);
    if i >= cfg.nodes {
        return;
    }
    let was = cfg.flip;
    let next = 1u - cfg.flip;
    let p = atomicLoad(&work[at(PTR + was, i)]);
    var s = atomicLoad(&work[at(VAL + was, i)]);
    if s == NONE {
        s = atomicLoad(&work[at(VAL + was, p)]);
    }
    atomicStore(&work[at(VAL + next, i)], s);
    atomicStore(&work[at(PTR + next, i)], atomicLoad(&work[at(PTR + was, p)]));
}

fn grow_box(lo: vec3<f32>, hi: vec3<f32>) {
    let b = WORDS * cfg.nodes;
    for (var c = 0u; c < 3u; c++) {
        atomicMin(&work[b + c], ord(lo[c]));
        atomicMax(&work[b + 3u + c], ord(hi[c]));
    }
}

// The box grown about each drawn node's wood.
@compute @workgroup_size(64)
fn box_wood(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index_of(id);
    if i >= cfg.nodes || !made(i) {
        return;
    }
    let w = sweep_of(i);
    grow_box(min(w.a - w.start, w.b - w.end), max(w.a + w.start, w.b + w.end));
}

// The box grown about each leaf segment's reach, or each of a frond's
// leaflets.
@compute @workgroup_size(64)
fn box_leaves(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = index_of(id);
    if g >= cfg.segs {
        return;
    }
    let f = segment(g);
    if !frond(f) {
        let r = leaf_reach(f);
        grow_box(min(f[0].xyz, f[1].xyz) - r, max(f[0].xyz, f[1].xyz) + r);
        return;
    }
    for (var j = 0u; j < segs[g * GW + 25u]; j++) {
        let l = blade(g, f, j);
        let r = l.wide + l.thick;
        grow_box(min(l.a, l.b) - r, max(l.a, l.b) + r);
    }
}
