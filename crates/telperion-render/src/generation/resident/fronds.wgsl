// fn-283 R4: each crown's fronds drawn once, in the leaf-base stage
// (`rosette::fronds`, `Crown`), into the table the stations read their
// frond segments from (`stations.wgsl`'s `rosette`). `crowns` lists the
// stem apices that draw a living frond, in record order; `crown_fronds`
// writes each crown's fronds, newest first, one invocation a crown;
// `rachises` hangs each frond's axis from the table (fn-283 R1).
//
// The table: the crowns and their fronds; a crown each (`CW`): its apex's
// id, its fronds, its first frond and its leaflets; then a frond each
// (`FW`): a frond segment's words as `pack.wgsl` reads them, its first
// leaflet counted from its crown's.
@group(0) @binding(6) var<storage, read_write> frond_table: array<u32>;
const CW: u32 = 4u;
const FW: u32 = 32u;

// Word `f` of node `id`'s drawn record.
fn node_word(id: u32, f: u32) -> u32 {
    return bitcast<u32>(drawn[id * CV + f]);
}

fn node_pos(id: u32) -> vec3<f32> {
    let b = id * CV;
    return vec3<f32>(drawn[b], drawn[b + 1u], drawn[b + 2u]);
}

fn node_parent(id: u32) -> u32 {
    return node_word(id, 16u);
}

// Its flags where the dress made it a node; none where it did not.
fn node_flags(id: u32) -> u32 {
    let f = node_word(id, 6u);
    return select(0u, f, (f & MADE) != 0u);
}

// The first node of the stem an apex ends: what keys its fronds, which
// stays as the stem grows (`rosette::stem_key`).
fn stem_first(apex: u32) -> u32 {
    var first = apex;
    // cap: the nodes; no stem is longer.
    let cap = arrayLength(&drawn) / CV;
    for (var n = 0u; n < cap; n++) {
        let p = node_parent(first);
        // The root bears every stem: never stepped onto.
        let root = node_parent(p) == NONE || node_parent(p) == p;
        if p == NONE || p == first || root || (node_flags(p) & STEM) == 0u {
            break;
        }
        first = p;
    }
    return first;
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
    let key = node_word(stem_first(apex), 24u);
    let living = organs(organ_word(1u), key, ORGAN_FRONDS);
    let dead = organs(organ_word(2u), key, ORGAN_SKIRT);
    let more = select(0u, 1u, living > (organ_word(1u) >> 16u));
    let skirt = select(0u, 1u, dead > (organ_word(2u) >> 16u));
    let newest = bitcast<i32>(organ_word(6u + more));
    return Drawn(key, living, dead, newest, organ_word(8u + 2u * more + skirt));
}

// The stem apices a rosette crowns (`Tree::stem_apices`), in record
// order: a fine stem node no fine stem carries on, drawing a living
// frond; each its fronds and its first. One invocation.
@compute @workgroup_size(1)
fn crowns() {
    var count = 0u;
    var total = 0u;
    for (var r = 0u; r < cfg.count; r++) {
        let id = cfg.base + r;
        let f = node_flags(id);
        let fine = (f & FINE) != 0u && (f & BOOT) == 0u;
        if !fine || (f & STEM) == 0u || (atomicLoad(&carried[r]) & STEMMED) != 0u {
            continue;
        }
        let c = crown_of(id);
        if c.living == 0u {
            continue;
        }
        let at = 2u + count * CW;
        frond_table[at] = id;
        frond_table[at + 1u] = c.slots;
        frond_table[at + 2u] = total;
        frond_table[at + 3u] = 0u;
        count += 1u;
        total += c.slots;
    }
    frond_table[0] = count;
    frond_table[1] = total;
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

fn unfold_run(t: f32, days: f32) -> f32 {
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
    return unfold_run(y.w - emerges, s.z);
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

fn put_frond(at: u32, v: vec4<f32>) {
    for (var c = 0u; c < 4u; c++) {
        frond_table[at + c] = bitcast<u32>(v[c]);
    }
}

// Each crown's fronds on `rosette::fronds`' spiral, newest first, by its
// serial among the fronds made, a place further down for every frond
// grown above it. A frond's words: where it leaves the axis and its
// rachis length; the radial it leans out along and its size share; its
// heading and its spines' share; the apex's axis (every leaflet kept);
// the rachis's arch, the leaflet pitch, the terminal share and the
// spines' pitch; whether it withers, the frond's kind (-2), its spines
// and how far it splits; its first leaflet, leaflets and key; its strips'
// blade over a mature leaflet (`pack.wgsl`'s `leaflet`). One invocation a
// crown.
@compute @workgroup_size(64)
fn crown_fronds(@builtin(global_invocation_id) id: vec3<u32>) {
    let crown = index(id);
    if crown >= cfg.n {
        return;
    }
    let e = 2u + crown * CW;
    let apex = frond_table[e];
    let records = 2u + frond_table[0] * CW;
    let at = node_pos(apex);
    let along = at - node_pos(node_parent(apex));
    var axis = vec3<f32>(0.0, 1.0, 0.0);
    if dot(along, along) > 1e-18 {
        axis = normalize(along);
    }
    let normal = frame(axis);
    let binormal = cross(axis, normal);
    let r = cfg.rosette;
    // The apex's own whole counts (fn-277).
    let c = crown_of(apex);
    let fronds = f32(c.living);
    let oldest = f32(max(c.living, 1u) - 1u);
    let steps = max(oldest, 1.0);
    let total = fronds + f32(c.dead);
    let stream = organ_mix(c.key ^ KEY_ROSETTE.x);
    var first = 0u;
    var above = 0.0;
    for (var i = 0u; i < c.slots; i++) {
        let serial = c.newest - i32(i);
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
        if r[6].z <= 0.0 {
            split = 1.0;
        }
        let blade = segments(split);
        let heading = normalize(axis * cos(pitch) + radial * sin(pitch));
        let s = records + (frond_table[e + 2u] + i) * FW;
        put_frond(s, vec4<f32>(at - axis * depth, r[2].y * share));
        put_frond(s + 4u, vec4<f32>(radial, share));
        put_frond(s + 8u, vec4<f32>(heading, r[3].z));
        put_frond(s + 12u, vec4<f32>(axis, 0.0));
        put_frond(s + 16u, vec4<f32>(r[2].z, blade.pitch, r[3].x, r[3].w));
        // Its own leaflets and spines (`organs::placement`, fn-277).
        let leaflets = frond_leaves(c.key, serial, fronds);
        let spines = organs(blade.spines, placement_key(c.key, u32(serial)), ORGAN_SPINES);
        put_frond(s + 20u, vec4<f32>(f32(turned >= 0.5), -2.0, f32(spines), split));
        frond_table[s + 15u] = NONE;
        frond_table[s + 24u] = first;
        frond_table[s + 25u] = leaflets;
        frond_table[s + 26u] = 0u;
        frond_table[s + 27u] = organ_mix(stream + (u32(serial) + 1u) * 0x9e3779b9u);
        frond_table[s + 28u] = bitcast<u32>(r[6].z);
        for (var w = 29u; w < FW; w++) {
            frond_table[s + w] = 0u;
        }
        first += leaflets;
    }
    frond_table[e + 3u] = first;
}

// fn-283 R1: the thinnest wood the engine draws (`space.rs`' `FINEST`).
const FINEST: f32 = 2e-4;

fn table3(at: u32) -> vec3<f32> {
    return vec3<f32>(bitcast<f32>(frond_table[at]), bitcast<f32>(frond_table[at + 1u]), bitcast<f32>(frond_table[at + 2u]));
}

// Each frond's axis as wood (`rachis.rs`' `rachises` and `hang`): a run
// of its own from where it leaves the stem's axis through every station
// its leaflets stand at, in the room its frond takes past the bases'
// records, borne on the stem node that depth falls on as a base is
// (`along`), tapering from half the width times its size share and never
// below FINEST; none for a frond of one leaflet or of no rachis length.
// One invocation a frond; the first opens the wave.
@compute @workgroup_size(64)
fn rachises(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = index(id);
    if g >= cfg.n {
        return;
    }
    if g == 0u {
        open_wave();
    }
    let crowns = frond_table[0];
    var crown = 0u;
    // cap: the crowns.
    for (var t = 0u; t < crowns; t++) {
        if frond_table[2u + t * CW + 2u] <= g {
            crown = t;
        }
    }
    let s = 2u + crowns * CW + g * FW;
    let count = frond_table[s + 25u];
    let length = bitcast<f32>(frond_table[s + 3u]);
    if count < 2u || length <= 0.0 {
        return;
    }
    let insertion = table3(s);
    let heading = table3(s + 8u);
    let arch = bitcast<f32>(frond_table[s + 16u]);
    let split = bitcast<f32>(frond_table[s + 23u]);
    let r0 = bitcast<f32>(cfg.axes.z) / 2.0 * length / cfg.rosette[2].y;
    let lift = frame(heading);
    let across = cross(heading, lift);
    // The stem node it leaves from: the apex itself on a stem of one node.
    let apex = frond_table[2u + crown * CW] - cfg.base;
    var bearer = apex;
    if down(apex) != NONE {
        bearer = along(apex, distance(insertion, pos(apex))).at;
    }
    let held = pos(bearer);
    let first = cfg.axes.x + g * cfg.axes.y;
    // A first node standing on its bearer is drawn from the bearer, as a
    // base's is (`hang`).
    let stands = distance(held, insertion) <= 1e-9 + STANDS * polyline(apex);
    let opens = select(first, first + 1u, stands);
    let base = max(r0, FINEST);
    var parent = word(bearer, 14u);
    var at = held;
    var travelled = drawn[cfg.records + bearer * CV + 15u];
    var before = base;
    // cap: the frond's leaflets, its room past the first.
    for (var k = 0u; k <= count; k++) {
        var t = f32(k) / f32(count);
        if split < 1.0 {
            t *= split;
        }
        let point = rachis(insertion, heading, lift, length, arch, t).at;
        let radius = max(r0 * (1.0 - t), FINEST);
        let r = first + k;
        travelled += distance(at, point);
        put_node(r, point, vec3<f32>(radius, before, base), parent, at, travelled);
        putu(r, 6u, TAKEN | MADE | FINE | BOOT);
        putu(r, 8u, opens);
        putu(r, 23u, cfg.out_base + first);
        for (var c = 0u; c < 3u; c++) {
            put(r, 17u + c, heading[c]);
            put(r, 20u + c, across[c]);
        }
        parent = cfg.out_base + r;
        at = point;
        before = radius;
    }
    if stands {
        putu(first, 6u, TAKEN | BOOT);
        putu(first + 1u, 16u, word(bearer, 14u));
        for (var c = 0u; c < 3u; c++) {
            put(first + 1u, 10u + c, held[c]);
        }
    }
}
