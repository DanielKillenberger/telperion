// A growth unit's nodes on the GPU (fn-209 F1, task 2): `grow_unit`'s
// draws for each unit a fine root grows, from `telperion_space::Units`'
// tables. `count` sizes each unit's nodes; `emit` writes them in their
// order along the unit, each with its key, its presence (Q32, D18) and
// the PA each bud place drew (D3), 0xff for a bare place.
//
// `table` holds each PA's (first zone, zones) and then each zone's eight
// words: zone, poisson, nodes, law at, places, laterals at, PAs, windows
// at; `words` holds the laws (threshold, bound per node), the laterals'
// thresholds (all ones for none) and the windows (Q32 per cycle).

@group(0) @binding(0) var<uniform> cfg: vec4<u32>;
@group(0) @binding(1) var<storage, read> units: array<vec4<u32>>;
@group(0) @binding(2) var<storage, read> table: array<u32>;
@group(0) @binding(3) var<storage, read> words: array<vec2<u32>>;
@group(0) @binding(4) var<storage, read_write> counts: array<u32>;
@group(0) @binding(5) var<storage, read_write> nodes: array<vec2<u32>>;

const MOST: u32 = 64u;
const ZONE: u32 = 16u;
const NONE: u32 = 0xffffffffu;

fn unit_index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.z;
}

// Zone `z`'s node count for its draw `d`, at most `MOST + 1`.
fn zone_count(at: u32, d: vec2<u32>) -> u32 {
    let n = table[at + 2u];
    let law = table[at + 3u];
    for (var j = 0u; j < n; j++) {
        if lt64(d, words[law + 2u * j]) {
            return j;
        }
    }
    return n;
}

fn zone_key(unit: vec2<u32>, at: u32) -> vec2<u32> {
    return child(unit, w(ZONE + table[at], 0u));
}

fn unit_key(u: vec4<u32>) -> vec2<u32> {
    return child(u.xy, w(u.w, 0u));
}

@compute @workgroup_size(64)
fn count(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = unit_index(id);
    if i >= cfg.x {
        return;
    }
    let u = units[i];
    let key = unit_key(u);
    let first = table[2u * u.z];
    let zones = table[2u * u.z + 1u];
    var total = 0u;
    var most = 0u;
    for (var k = 0u; k < zones; k++) {
        let at = first + 8u * k;
        let n = zone_count(at, draw(zone_key(key, at)));
        total += n;
        most = max(most, n);
    }
    // A zone past the kernel's bound is reported, never cut short.
    counts[i] = select(total, NONE, most > MOST);
}

// The PA bud place `place` drew, 0xff for a bare one.
fn bud(at: u32, place: vec2<u32>) -> u32 {
    let d = draw(place);
    let laterals = table[at + 5u];
    for (var j = 0u; j < table[at + 6u]; j++) {
        let t = words[laterals + j];
        if !(t.x == NONE && t.y == NONE) && lt64(d, t) {
            return j;
        }
    }
    return 0xffu;
}

@compute @workgroup_size(64)
fn emit(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = unit_index(id);
    if i >= cfg.x {
        return;
    }
    let u = units[i];
    let key = unit_key(u);
    let first = table[2u * u.z];
    let zones = table[2u * u.z + 1u];
    var out = counts[i];
    var order: array<vec2<u32>, MOST>;
    var drawn: array<u32, MOST>;
    for (var k = 0u; k < zones; k++) {
        let at = first + 8u * k;
        let zk = zone_key(key, at);
        let d = draw(zk);
        let n = zone_count(at, d);
        // Acrotony (D4): by the first place's draw, descending; equal
        // draws by the node's index.
        for (var j = 0u; j < n; j++) {
            let o = draw(child(child(zk, w(j, 0u)), w(0u, 0u)));
            var m = j;
            while m > 0u && lt64(order[m - 1u], o) {
                order[m] = order[m - 1u];
                drawn[m] = drawn[m - 1u];
                m -= 1u;
            }
            order[m] = o;
            drawn[m] = j;
        }
        let law = table[at + 3u];
        let window = words[table[at + 7u] + u.w];
        for (var m = 0u; m < n; m++) {
            let j = drawn[m];
            let node = child(zk, w(j, 0u));
            let bound = words[law + 2u * j + 1u];
            var lead: vec2<u32>;
            if table[at + 1u] != 0u {
                lead = above_q(d, bound);
            } else {
                lead = past_q(d, bound);
            }
            var packed = vec2<u32>(0xffffffffu);
            for (var s = 0u; s < table[at + 4u]; s++) {
                let pa = bud(at, child(node, w(s, 0u)));
                let word = s / 4u;
                let shift = 8u * (s % 4u);
                packed[word] = (packed[word] & ~(0xffu << shift)) | (pa << shift);
            }
            nodes[3u * out] = node;
            nodes[3u * out + 1u] = presence(lead, window);
            nodes[3u * out + 2u] = packed;
            out += 1u;
        }
    }
}
