// fn-266 (fn-256 DESIGN.md 12.8): the retained leaf bases hung on the
// device, between the dress and the folds: `leaf_bases.rs`' table and
// `hung`, `lattice.rs`' cell and `section.rs`' rings, in plain f32. Only
// the seed's wave holds stem nodes, so it is the one wave read. `kids`
// marks each stem node a stem carries on; `apices` lists the clothed
// stem apices; `place` lays each base on its stem; `lattice` gives each
// stem its bark and cell; `hang` writes each base's two drawn records
// and its cell in `packed_sections`' layout.

struct Cfg {
    n: u32,
    row: u32,
    // The seed wave's records: where they start, how many, their first id.
    records: u32,
    count: u32,
    base: u32,
    // The bases a stem hangs, as a count's word (`telperion_space::word`,
    // each stem drawing its whole count, fn-277), the boots' first id,
    // whether they pack.
    bases: u32,
    out_base: u32,
    packed: u32,
    // The crown's living fronds as a count's word, each apex drawing its
    // whole count; the spiral's divergence (radians) and the depth below
    // the apex the oldest living frond leaves at.
    fronds: u32,
    divergence: f32,
    depth: f32,
    // The bases' pitch (radians, held off the axis where they pack), length,
    // girth share, weathering, width and flatness.
    pitch: f32,
    length: f32,
    radius: f32,
    weathering: f32,
    width: f32,
    flatness: f32,
    // Where the boots' records and their axis start in the drawn records
    // (`fold.rs`'s `Dressed`, in the room past the dress's), and the
    // axis's slot, which their axis word holds.
    out_records: u32,
    out_axis: u32,
    out_slot: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
// Every drawn record by id, then every axis by slot (`fold.rs`'s
// `Dressed`): the seed's wave read, the boots' written.
@group(0) @binding(1) var<storage, read_write> drawn: array<f32>;
@group(0) @binding(2) var<storage, read_write> carried: array<atomic<u32>>;
// The clothed apices' count and their bases', then a slot each (`AW`).
@group(0) @binding(3) var<storage, read_write> stems: array<u32>;
// A base each (`BW`): from, axis, radial, heading, girth, its bearer's
// record, its age and its stem's slot.
@group(0) @binding(4) var<storage, read_write> table: array<f32>;
@group(0) @binding(5) var<storage, read_write> cells: array<f32>;

const NONE: u32 = 0xffffffffu;
const CA: u32 = 12u;
const CV: u32 = 38u;
const TAKEN: u32 = 1u;
const MADE: u32 = 2u;
const FINE: u32 = 8u;
const STEM: u32 = 128u;
// A boot's record, and the record a boot's run opens at where it is
// drawn as its cell: above a tip's leaf count (`dress.wgsl`'s `LEAVES`,
// at most 8 leaves, `MAX_CLUSTER_LEAVES`).
const BOOT: u32 = 0x40000000u;
const CELL: u32 = 0x20000000u;
// A stem's slot: its apex's record, its first base, its polyline's
// length, its bark, the lattice's pitch and its cell; its bases and its
// living fronds, each its whole count (fn-277).
const AW: u32 = 12u;
const BW: u32 = 16u;
const SECTION: u32 = 28u;
const SAMPLES: u32 = 9u;
const PI: f32 = 3.141592653589793;
const TAU: f32 = 6.283185307179586;
// `leaf_bases.rs`' `WORN`.
const WORN: f32 = 0.05;
// A few f32 ulps of a stem's length: where a node stands on its parent.
const STANDS: f32 = 1e-6;

fn index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

fn word(r: u32, f: u32) -> u32 {
    return bitcast<u32>(drawn[cfg.records + r * CV + f]);
}

fn pos(r: u32) -> vec3<f32> {
    let b = cfg.records + r * CV;
    return vec3<f32>(drawn[b], drawn[b + 1u], drawn[b + 2u]);
}

fn stem(r: u32) -> bool {
    let f = word(r, 6u);
    return (f & MADE) != 0u && (f & STEM) != 0u;
}

// Record `r`'s parent's record in the seed wave; none for the root.
fn parent(r: u32) -> u32 {
    let id = word(r, 16u);
    if id < cfg.base || id >= cfg.base + cfg.count {
        return NONE;
    }
    return id - cfg.base;
}

// The next node down a stem's polyline, or none at its foot.
fn down(r: u32) -> u32 {
    let p = parent(r);
    if p == NONE || !stem(p) {
        return NONE;
    }
    return p;
}

@compute @workgroup_size(64)
fn kids(@builtin(global_invocation_id) id: vec3<u32>) {
    let r = index(id);
    if r >= cfg.n || !stem(r) {
        return;
    }
    let p = parent(r);
    if p != NONE {
        atomicOr(&carried[p], 1u);
    }
}

// The length of the stem's polyline from apex `r` to its foot.
fn polyline(r: u32) -> f32 {
    var at = r;
    var run = 0.0;
    // cap: the wave's records; no polyline is longer.
    for (var n = 0u; n < cfg.count; n++) {
        let d = down(at);
        if d == NONE {
            break;
        }
        run += distance(pos(at), pos(d));
        at = d;
    }
    return run;
}

// The key of the first node of the stem apex `r` ends: its foot, which
// keys its counts of organs (`rosette::stem_node`).
fn stem_key(r: u32) -> u32 {
    var at = r;
    // cap: the wave's records; no polyline is longer.
    for (var n = 0u; n < cfg.count; n++) {
        let d = down(at);
        if d == NONE {
            break;
        }
        at = d;
    }
    return word(at, 24u);
}

// The clothed apices in record order (`Tree::stem_apices`, `leaf_bases`):
// a stem node no stem carries on, whose stem is longer than the crown's
// depth, and that draws a base; each its bases and living fronds. One
// invocation.
@compute @workgroup_size(1)
fn apices() {
    var clothed = 0u;
    var total = 0u;
    for (var r = 0u; r < cfg.count; r++) {
        if !stem(r) || atomicLoad(&carried[r]) != 0u {
            continue;
        }
        let run = polyline(r);
        if run - cfg.depth <= 0.0 {
            continue;
        }
        let key = stem_key(r);
        let bases = organs(cfg.bases, key, ORGAN_BASES);
        if bases == 0u {
            continue;
        }
        let s = 2u + clothed * AW;
        stems[s] = r;
        stems[s + 1u] = total;
        stems[s + 2u] = bitcast<u32>(run);
        stems[s + 9u] = bases;
        stems[s + 10u] = organs(cfg.fronds, key, ORGAN_FRONDS);
        clothed += 1u;
        total += bases;
    }
    stems[0] = clothed;
    stems[1] = total;
}

fn slot(s: u32, f: u32) -> u32 {
    return stems[2u + s * AW + f];
}

// The clothed stem base `b` hangs on.
fn stem_of(b: u32) -> u32 {
    var s = 0u;
    // cap: the clothed stems.
    for (var t = 0u; t < stems[0]; t++) {
        if slot(t, 1u) <= b {
            s = t;
        }
    }
    return s;
}

fn slotf(s: u32, f: u32) -> f32 {
    return bitcast<f32>(slot(s, f));
}

struct Along {
    at: u32,
    point: vec3<f32>,
    axis: vec3<f32>,
    girth: f32,
}

// Where a depth below apex `apex` falls on its stem (`leaf_bases.rs`'
// `along`): the node a base is borne on, the point, the stem's direction
// and its girth there.
fn along(apex: u32, want: f32) -> Along {
    var upper = apex;
    var above = 0.0;
    var lower = down(apex);
    var below = above + distance(pos(upper), pos(lower));
    // cap: the wave's records; no polyline is longer.
    for (var n = 0u; n < cfg.count; n++) {
        let next = down(lower);
        if below >= want || next == NONE {
            break;
        }
        upper = lower;
        above = below;
        lower = next;
        below = above + distance(pos(upper), pos(lower));
    }
    let span = below - above;
    var t = 0.0;
    if span > 1e-12 {
        t = clamp((want - above) / span, 0.0, 1.0);
    }
    let a = pos(upper);
    let b = pos(lower);
    var axis = vec3<f32>(0.0, 1.0, 0.0);
    if dot(a - b, a - b) > 1e-18 {
        axis = normalize(a - b);
    }
    let radius = drawn[cfg.records + upper * CV + 3u];
    let start = drawn[cfg.records + upper * CV + 4u];
    return Along(lower, a + (b - a) * t, axis, radius + (start - radius) * t);
}

// The mean of the polyline over a trunk's width either way of `at`,
// narrowed toward either end of the clothed run (`centreline`'s `mean`).
fn mean(apex: u32, run: f32, at0: f32, girth: f32) -> vec3<f32> {
    let top = cfg.depth;
    let at = clamp(at0, top, run);
    let half = min(min(2.0 * girth, at - top), run - at);
    var sum = vec3<f32>(0.0);
    for (var k = 0u; k < SAMPLES; k++) {
        let d = at - half + 2.0 * half * f32(k) / f32(SAMPLES - 1u);
        sum += along(apex, d).point;
    }
    return sum * (1.0 / f32(SAMPLES));
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

// `rosette::frame`: the crown's reference normal on `axis`.
fn frame(axis: vec3<f32>) -> vec3<f32> {
    let up = vec3<f32>(0.0, 1.0, 0.0) - axis * axis.y;
    if dot(up, up) > 1e-12 {
        return normalize(up);
    }
    return perpendicular(axis);
}

fn rotate(v: vec3<f32>, axis: vec3<f32>, angle: f32) -> vec3<f32> {
    let s = sin(angle);
    let c = cos(angle);
    return v * c + cross(axis, v) * s + axis * (dot(axis, v) * (1.0 - c));
}

// The crown's reference carried down to `axis` by the one rotation that
// takes the apex's axis to it (`carried`): its normal.
fn carried_normal(apex: vec3<f32>, axis: vec3<f32>, reference: vec3<f32>) -> vec3<f32> {
    let turn = cross(apex, axis);
    if dot(turn, turn) > 1e-18 {
        return rotate(reference, normalize(turn), acos(clamp(dot(apex, axis), -1.0, 1.0)));
    }
    if dot(apex, axis) > 0.0 {
        return reference;
    }
    return frame(axis);
}

fn put3(at: u32, v: vec3<f32>) {
    table[at] = v.x;
    table[at + 1u] = v.y;
    table[at + 2u] = v.z;
}

fn get3(at: u32) -> vec3<f32> {
    return vec3<f32>(table[at], table[at + 1u], table[at + 2u]);
}

// Each base on its stem (`leaf_bases`' loop): `k` counting from the
// newest down, one invocation a base.
@compute @workgroup_size(64)
fn place(@builtin(global_invocation_id) id: vec3<u32>) {
    let b = index(id);
    if b >= cfg.n {
        return;
    }
    let s = stem_of(b);
    let k = b - slot(s, 1u);
    let apex = slot(s, 0u);
    let run = slotf(s, 2u);
    let clothed = run - cfg.depth;
    let oldest = f32(slot(s, 9u) - 1u);
    var age = 0.0;
    if oldest > 0.0 {
        age = f32(k) / oldest;
    }
    let want = cfg.depth + clothed * age;
    var a = along(apex, want);
    if cfg.packed != 0u {
        let chord = mean(apex, run, want - a.girth, a.girth) - mean(apex, run, want + a.girth, a.girth);
        if dot(chord, chord) > 1e-18 {
            a.axis = normalize(chord);
        }
        a.point = mean(apex, run, want, a.girth);
    }
    // The apex's own axis (`rosettes`), and the crown's reference on it.
    var below = vec3<f32>(0.0);
    let p = parent(apex);
    if p != NONE {
        below = pos(p);
    }
    var crown = vec3<f32>(0.0, 1.0, 0.0);
    if dot(pos(apex) - below, pos(apex) - below) > 1e-18 {
        crown = normalize(pos(apex) - below);
    }
    let normal = carried_normal(crown, a.axis, frame(crown));
    let binormal = cross(a.axis, normal);
    let turn = (f32(slot(s, 10u)) + f32(k)) * cfg.divergence;
    let radial = normalize(normal * cos(turn) + binormal * sin(turn));
    let heading = normalize(a.axis * cos(cfg.pitch) + radial * sin(cfg.pitch));
    let t = b * BW;
    put3(t, a.point);
    put3(t + 3u, a.axis);
    put3(t + 6u, radial);
    put3(t + 9u, heading);
    table[t + 12u] = a.girth;
    table[t + 13u] = bitcast<f32>(a.at);
    table[t + 14u] = age;
    table[t + 15u] = bitcast<f32>(s);
}

fn dot2(a: vec2<f32>, b: vec2<f32>) -> f32 {
    return a.x * b.x + a.y * b.y;
}

// `f64::round`: half away from zero (WGSL's `round` takes half to even).
fn round_away(x: f32) -> f32 {
    return sign(x) * floor(abs(x) + 0.5);
}

// `lattice::cell` at full width: two half-diagonals in (radians round,
// metres along), the first leading the second counterclockwise.
fn cell(girth: f32, spacing: f32) -> vec4<f32> {
    let step = (cfg.divergence + PI) - TAU * floor((cfg.divergence + PI) / TAU) - PI;
    var a = vec2<f32>(TAU * girth, 0.0);
    var b = vec2<f32>(step * girth, -spacing);
    if dot2(a, a) > dot2(b, b) {
        let w = a;
        a = b;
        b = w;
    }
    // cap: `lattice.rs`' `STEPS`.
    for (var n = 0u; n < 64u; n++) {
        b = b - a * round_away(dot2(a, b) / dot2(a, a));
        if dot2(b, b) >= dot2(a, a) {
            break;
        }
        let w = a;
        a = b;
        b = w;
    }
    let first = vec2<f32>((a.x + b.x) / 2.0 / girth, (a.y + b.y) / 2.0);
    let second = vec2<f32>((a.x - b.x) / 2.0 / girth, (a.y - b.y) / 2.0);
    if first.x * second.y - first.y * second.x >= 0.0 {
        return vec4<f32>(first, second);
    }
    return vec4<f32>(second, first);
}

// Each clothed stem's lattice: its bases' mean girth, the distance they
// stand apart along it, and the cell that gives them. One invocation a
// stem.
@compute @workgroup_size(64)
fn lattice(@builtin(global_invocation_id) id: vec3<u32>) {
    let s = index(id);
    if s >= cfg.n {
        return;
    }
    let first = slot(s, 1u);
    let bases = slot(s, 9u);
    var bark = 0.0;
    for (var k = 0u; k < bases; k++) {
        bark += table[(first + k) * BW + 12u];
    }
    bark = bark / f32(bases);
    let oldest = max(f32(bases - 1u), 1.0);
    var pitch = (slotf(s, 2u) - cfg.depth) / oldest;
    if cfg.packed != 0u && bases > 1u {
        var length = 0.0;
        for (var k = 0u; k + 1u < bases; k++) {
            length += distance(get3((first + k) * BW), get3((first + k + 1u) * BW));
        }
        pitch = length / f32(bases - 1u);
    }
    let c = cell(bark, pitch);
    let o = 2u + s * AW;
    stems[o + 3u] = bitcast<u32>(bark);
    stems[o + 4u] = bitcast<u32>(pitch);
    for (var w = 0u; w < 4u; w++) {
        stems[o + 5u + w] = bitcast<u32>(c[w]);
    }
}

fn put(r: u32, f: u32, v: f32) {
    drawn[cfg.out_records + r * CV + f] = v;
}

fn putu(r: u32, f: u32, v: u32) {
    put(r, f, bitcast<f32>(v));
}

fn put_node(r: u32, at: vec3<f32>, radii: vec3<f32>, parent_id: u32, held: vec3<f32>, along: f32) {
    for (var f = 0u; f < CV; f++) {
        putu(r, f, 0u);
    }
    for (var c = 0u; c < 3u; c++) {
        put(r, c, at[c]);
        put(r, 3u + c, radii[c]);
        put(r, 10u + c, held[c]);
    }
    putu(r, 7u, r);
    putu(r, 9u, cfg.out_slot);
    putu(r, 14u, cfg.out_base + r);
    put(r, 15u, along);
    putu(r, 16u, parent_id);
}

fn put_cell(c: u32, at: u32, v: vec3<f32>) {
    for (var k = 0u; k < 3u; k++) {
        cells[c * SECTION + at + k] = v[k];
    }
}

// Each base's two records and, where the bases pack, its cell (`hung`,
// `section`). One invocation a base; the first also writes the wave's
// axis, which loses every tie for a bearer's run.
@compute @workgroup_size(64)
fn hang(@builtin(global_invocation_id) id: vec3<u32>) {
    let b = index(id);
    if b >= cfg.n {
        return;
    }
    if b == 0u {
        // Runtime words: Tint refuses a constant NaN, which naga folds.
        var ones = NONE;
        for (var f = 0u; f < CA; f++) {
            drawn[cfg.out_axis + f] = 0.0;
        }
        drawn[cfg.out_axis + 8u] = bitcast<f32>(63u);
        drawn[cfg.out_axis + 9u] = bitcast<f32>(ones);
        drawn[cfg.out_axis + 10u] = bitcast<f32>(ones);
    }
    let t = b * BW;
    let point = get3(t);
    let axis = get3(t + 3u);
    let radial = get3(t + 6u);
    let heading = get3(t + 9u);
    let girth = table[t + 12u];
    let bearer = bitcast<u32>(table[t + 13u]);
    let age = table[t + 14u];
    let s = bitcast<u32>(table[t + 15u]);
    let wear = max(1.0 - cfg.weathering * age, WORN);
    let thick = max(girth * cfg.radius, 1e-6);
    let packed = cfg.packed != 0u;
    var emerges = girth;
    if packed {
        emerges = girth / dot(radial, heading);
    }
    let reach = emerges + cfg.length * wear;
    var tip = point + heading * reach;
    let across = cross(axis, radial);
    if packed {
        let lean = dot(radial, heading);
        let upright = dot(axis, heading);
        let bark = slotf(s, 3u);
        let lift = (bark - girth) * upright / lean;
        let shares = vec3<f32>(emerges / 2.0, emerges, reach);
        put_cell(b, 0u, point);
        put_cell(b, 3u, axis);
        put_cell(b, 6u, radial);
        put_cell(b, 9u, across);
        for (var w = 0u; w < 4u; w++) {
            cells[b * SECTION + 12u + w] = slotf(s, 5u + w) * cfg.width;
        }
        cells[b * SECTION + 16u] = cfg.flatness;
        for (var k = 0u; k < 3u; k++) {
            let scale = select(1.0, wear, k == 2u);
            put_cell(b, 17u + 3u * k, vec3<f32>(shares[k] * lean, shares[k] * upright + lift, scale));
        }
        for (var w = 26u; w < SECTION; w++) {
            cells[b * SECTION + w] = 0.0;
        }
        tip = point + axis * (reach * upright + lift) + radial * (reach * lean);
    }
    let held = pos(bearer);
    let first = 2u * b;
    let second = first + 1u;
    // A base whose first node stands on its bearer (the oldest, at a
    // stem's foot) is drawn from the bearer: `paths.rs` merges a node
    // within 1e-9 of its parent into the parent's stand, and in f32 that
    // is a few ulps of the depth walked down the stem to reach it.
    let stands = distance(held, point) <= 1e-9 + STANDS * slotf(s, 2u);
    let opens = select(first, second, stands);
    let travelled = drawn[cfg.records + bearer * CV + 15u] + distance(held, point);
    put_node(first, point, vec3<f32>(thick), word(bearer, 14u), held, travelled);
    put_node(second, tip, vec3<f32>(thick * wear, thick, thick), cfg.out_base + first, point, travelled + distance(point, tip));
    let flags = TAKEN | MADE | FINE | BOOT;
    putu(first, 6u, select(flags, TAKEN | BOOT, stands));
    putu(second, 6u, flags);
    if stands {
        putu(second, 16u, word(bearer, 14u));
        for (var c = 0u; c < 3u; c++) {
            put(second, 10u + c, held[c]);
        }
    }
    putu(opens, 6u, bitcast<u32>(drawn[cfg.out_records + opens * CV + 6u]) | select(0u, CELL, packed));
    for (var r = first; r <= second; r++) {
        putu(r, 8u, opens);
        putu(r, 23u, cfg.out_base + first);
        for (var c = 0u; c < 3u; c++) {
            put(r, 17u + c, heading[c]);
            put(r, 20u + c, across[c]);
        }
    }
    putu(opens, 29u, b);
}
