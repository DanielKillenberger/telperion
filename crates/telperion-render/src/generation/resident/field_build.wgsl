// The field's build (`field.wgsl`): each node's limb system, the
// primitives and the grid of buckets that lists them.

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> nodes: array<f32>;
@group(0) @binding(2) var<storage, read> segs: array<u32>;
@group(0) @binding(3) var<storage, read_write> work: array<atomic<u32>>;
@group(0) @binding(4) var<storage, read_write> prims: array<vec4<u32>>;
// The leaves' limb systems, the first leaf primitive's first.
@group(0) @binding(5) var<storage, read_write> owner: array<u32>;
@group(0) @binding(6) var<storage, read_write> index: array<atomic<u32>>;
// Each node's dense name of the system it opens: one where it opens one,
// none elsewhere, then scanned (`marks`), so a field's limbs are named
// 0, 1, 2 and a crowded cell is asked a window of them at a time.
@group(0) @binding(7) var<storage, read_write> dense: array<u32>;

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
// the most, the root's its own; the jumps start again at the parent.
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

// Whether each node opens the system it is in, before the scan that
// names the systems densely.
@compute @workgroup_size(64)
fn marks(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index_of(id);
    if i >= cfg.nodes {
        return;
    }
    dense[i] = u32(atomicLoad(&work[at(VAL + cfg.flip, i)]) == i);
}

fn grow_box(lo: vec3<f32>, hi: vec3<f32>) {
    let b = WORDS * cfg.nodes;
    for (var c = 0u; c < 3u; c++) {
        atomicMin(&work[b + c], ord(lo[c]));
        atomicMax(&work[b + 3u + c], ord(hi[c]));
    }
}

fn unord(w: u32) -> f32 {
    return bitcast<f32>(select(~w, w & 0x7fffffffu, (w & 0x80000000u) != 0u));
}

// The primitives' kernels run twice: once to grow the box, once to write
// each primitive within it (`Cfg::extent`).
fn writing() -> bool {
    return cfg.extent.w != 0.0;
}

// A position in steps of the box grown (`field.wgsl`'s `endpoints`).
fn steps(p: vec3<f32>) -> vec3<u32> {
    let b = WORDS * cfg.nodes;
    var lo: vec3<f32>;
    var hi: vec3<f32>;
    for (var c = 0u; c < 3u; c++) {
        lo[c] = unord(atomicLoad(&work[b + c]));
        hi[c] = unord(atomicLoad(&work[b + 3u + c]));
    }
    let q = quantum(hi - lo);
    let f = clamp(round((p - lo) / max(q, vec3<f32>(1e-30))), vec3<f32>(0.0), vec3<f32>(STEPS));
    return vec3<u32>(f);
}

fn first_words(u: vec3<u32>) -> vec2<u32> {
    return vec2<u32>(u.x | (u.y << 20u), (u.y >> 12u) | (u.z << 8u));
}

// A positive half in 15 bits.
fn to_half15(v: f32) -> u32 {
    return pack2x16float(vec2<f32>(max(v, 0.0), 0.0)) & 0x7fffu;
}

// Node `i`'s wood from its parent `p` (`NONE` for a point), its radii
// and its box: grown, or written. A node drawn in no record stands for
// nothing, but its position stays for its children's sweeps.
fn keep_wood(i: u32, p: u32, radii: vec2<f32>, lo: vec3<f32>, hi: vec3<f32>, drawn: bool) {
    if !writing() {
        if drawn {
            grow_box(lo, hi);
        }
        return;
    }
    let f = first_words(steps(pos(i)));
    let r = to_half15(radii.x) | (to_half15(radii.y) << 15u);
    prims[i] = vec4<u32>(f, p, (r << 1u) | select(DEAD, 0u, drawn));
}

// Leaf primitive `k` from `a` to `b` with its attributes and its box:
// grown, or written, its second end its difference from the first in the
// fewest-shifted 11-bit counts that hold it.
fn keep_leaf(k: u32, a: vec3<f32>, b: vec3<f32>, attributes: u32, lo: vec3<f32>, hi: vec3<f32>) {
    if !writing() {
        grow_box(lo, hi);
        return;
    }
    let u = steps(a);
    let d = vec3<f32>(steps(b)) - vec3<f32>(u);
    let most = max(max(abs(d.x), abs(d.y)), abs(d.z));
    var shift = 0u;
    // cap: a difference of 2^20 steps fits at a shift of 10.
    while shift < 15u && most > 1023.0 * exp2(f32(shift)) {
        shift += 1u;
    }
    let c = vec3<i32>(round(d / exp2(f32(shift))));
    let m = vec3<u32>(c) & vec3<u32>(0x7ffu);
    let f = first_words(u);
    prims[k] = vec4<u32>(
        f.x,
        f.y | (shift << 28u),
        m.x | (m.y << 11u) | (m.z << 22u),
        (m.z >> 10u) | (attributes << 1u),
    );
}

// Leaf primitive `k` standing for nothing.
fn nothing(k: u32) {
    if writing() {
        prims[k] = vec4<u32>(0u, 0u, 0u, DEAD);
    }
}

@compute @workgroup_size(64)
fn wood(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index_of(id);
    if i >= cfg.nodes {
        return;
    }
    let p = parent(i);
    let b = pos(i);
    let end = nodes[i * NW + 3u];
    var a = b;
    var start = end;
    var up = NONE;
    if i != cfg.root && p < cfg.nodes {
        a = pos(p);
        start = nodes[i * NW + 4u];
        up = p;
    }
    let lo = min(a - start, b - end);
    let hi = max(a + start, b + end);
    keep_wood(i, up, vec2<f32>(start, end), lo, hi, made(i));
}

// Each station segment's leaves, on the limb system of the node it
// stands on (`plan::build`'s descriptors): the wood's segment with a
// leaf's reach about it. A frond's (`stations.wgsl`'s `rosette`) runs
// from its base along its heading.
@compute @workgroup_size(64)
fn leaves(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = index_of(id);
    if g >= cfg.segs {
        return;
    }
    let s = g * GW;
    let f = segment(s);
    var a = f[0].xyz;
    var b = f[1].xyz;
    var reach = max(f[0].w, f[1].w) * cfg.reach.x + cfg.reach.y + cfg.reach.z;
    if f[5].y < -1.5 {
        // A frond: its leaflets stand as primitives of their own
        // (`leaflets`); the segment stands for none.
        nothing(cfg.nodes + g);
        if writing() {
            owner[g] = NONE;
        }
        return;
    }
    let on = segs[cfg.segs * GW + g];
    let borne = segs[s + 25u];
    let attributes = to_half15(reach) | (min(borne, 0x7fffu) << 15u);
    keep_leaf(cfg.nodes + g, a, b, attributes, min(a, b) - reach, max(a, b) + reach);
    if writing() {
        owner[g] = dense[atomicLoad(&work[at(VAL + cfg.flip, on)])];
    }
}

// Each leaflet of each frond as placement draws it (`pack.wgsl`'s
// `leaflet`, `on_rachis`, `packed`: its scatter and its size on the
// leaves' stream), the box `fronds.rs` describes it by as a sweep along
// its midline: from its centre either way along its axis, as wide as its
// flank.
@compute @workgroup_size(64)
fn leaflets(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = index_of(id);
    if g >= cfg.segs {
        return;
    }
    let s = g * GW;
    let f = segment(s);
    if !(f[5].y < -1.5) {
        return;
    }
    let count = segs[s + 25u];
    let first = segs[s + 24u];
    let system = dense[atomicLoad(&work[at(VAL + cfg.flip, segs[cfg.segs * GW + g])])];
    let heading = f[2].xyz;
    let grown = f[5].w;
    let mature = bitcast<f32>(segs[s + 28u]);
    for (var j = 0u; j < count; j++) {
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
        // `fronds.rs`'s packing margin, and its box: the midline, the
        // half-width along the flank and the half-thickness.
        let margin = (length(c) + length(h)) * 0.005 + 0.001;
        let centre = point + flank * c.x + axis * c.y + face * c.z;
        let run = axis * (h.y + margin);
        let wide = h.x + margin;
        let thick = h.z + margin;
        let p = cfg.segs + first + j;
        let r = wide + thick;
        let shape = flanked(flank, thick / r);
        let attributes = to_half15(r) | (shape << 15u);
        let a = centre - run;
        let b = centre + run;
        keep_leaf(cfg.nodes + p, a, b, attributes, min(a, b) - r, max(a, b) + r);
        if writing() {
            owner[p] = system;
        }
    }
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

struct Leaflet {
    point: vec3<f32>,
    axis: vec3<f32>,
    tangent: vec3<f32>,
    normal: vec3<f32>,
    share: f32,
}

fn rotate(v: vec3<f32>, axis: vec3<f32>, angle: f32) -> vec3<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return v * c + cross(axis, v) * s + axis * (dot(axis, v) * (1.0 - c));
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

fn segment(s: u32) -> array<vec4<f32>, 6> {
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

// The buckets each primitive's box meets, counted.
@compute @workgroup_size(64)
fn count(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index_of(id);
    if k >= cfg.dims.w || dead(k) {
        return;
    }
    let s = span(k);
    for (var z = s[0].z; z <= s[1].z; z++) {
        for (var y = s[0].y; y <= s[1].y; y++) {
            for (var x = s[0].x; x <= s[1].x; x++) {
                atomicAdd(&index[flat(vec3<u32>(x, y, z))], 1u);
            }
        }
    }
}

// Each primitive listed in every bucket its box meets, from the bucket's
// first place (the counts scanned), its fill after the starts.
@compute @workgroup_size(64)
fn fill(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index_of(id);
    if k >= cfg.dims.w || dead(k) {
        return;
    }
    let s = span(k);
    let list = 2u * cfg.buckets + 1u;
    for (var z = s[0].z; z <= s[1].z; z++) {
        for (var y = s[0].y; y <= s[1].y; y++) {
            for (var x = s[0].x; x <= s[1].x; x++) {
                let b = flat(vec3<u32>(x, y, z));
                let slot = atomicAdd(&index[cfg.buckets + 1u + b], 1u);
                atomicStore(&index[list + atomicLoad(&index[b]) + slot], k);
            }
        }
    }
}

