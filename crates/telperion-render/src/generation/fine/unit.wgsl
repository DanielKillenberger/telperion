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

fn unit_index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.z;
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
        let at = first + ZW * k;
        let n = zone_count(at, draw(zone_key(key, at)));
        total += n;
        most = max(most, n);
    }
    // A zone past the kernel's bound is reported, never cut short.
    counts[i] = select(total, NONE, most > MOST);
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
    for (var k = 0u; k < zones; k++) {
        let at = first + ZW * k;
        let zk = zone_key(key, at);
        let d = draw(zk);
        let n = zone_count(at, d);
        let side = zone_side(at, d, n);
        var drawn: array<u32, MOST>;
        sorted(zk, n, &drawn);
        let window = words[table[at + 7u] + u.w];
        for (var m = 0u; m < n; m++) {
            let j = drawn[m];
            let node = child(zk, w(j, 0u));
            let lead = node_lead(at, d, side, j);
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
