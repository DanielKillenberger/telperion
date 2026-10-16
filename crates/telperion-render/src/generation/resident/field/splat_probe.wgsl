// fn-257 task 12's probe (R8): the field splatted to a grid from what the
// drawn stage hands its consumers, the drawn nodes and the station
// segments, appended to `field.wgsl` and `field_build.wgsl`, whose
// bindings and derivations (each node's limb system and its dense name,
// a segment's leaf reach, a frond's leaflets) it reads. Each record
// writes the cells it touches by the CPU reference's cell rule
// (`pipeline/field.rs` `query`): wood where the cell's circumsphere meets
// its tapered sweep, the thickest radius so met, leaves the share of a
// leaf-bearing midline inside the cell, and the limb system with the most
// of them, the lower name on a tie.
//
// The grid: `cfg.dims` its cells a side and their count, `cfg.lo` its
// corner and its cell's side; `cfg.buckets` the slots of the table of
// each cell's leaves by system (a power of two). `index` holds two
// counters (the pairs a table holds, and the shares lost to a full probe
// or a 32-bit overflow), then four words a cell (flags, the thickest
// radius's bits, its leaves in fixed point, low and high), each cell's
// most leaves of one system and the least system holding them, then the
// table's keys (`cell + cells * system`) and its sums. `prims` takes each
// cell's answer, then the counters.

const S_PAIRS: u32 = 0u;

const S_LOST: u32 = 1u;

const S_HEAD: u32 = 2u;

const S_EMPTY: u32 = 0xffffffffu;

const S_PROBES: u32 = 64u;

// A 4096th of a leaf, as the query sums them.
const S_UNIT: f32 = 4096.0;

fn s_cells() -> u32 {
    return cfg.dims.w;
}

fn s_grid(c: u32, f: u32) -> u32 {
    return S_HEAD + c * 4u + f;
}

fn s_best(c: u32) -> u32 {
    return S_HEAD + 4u * s_cells() + c;
}

fn s_limb(c: u32) -> u32 {
    return S_HEAD + 5u * s_cells() + c;
}

fn s_key(t: u32) -> u32 {
    return S_HEAD + 6u * s_cells() + t;
}

fn s_sum(t: u32) -> u32 {
    return S_HEAD + 6u * s_cells() + cfg.buckets + t;
}

fn s_centre(c: vec3<i32>) -> vec3<f32> {
    return cfg.lo.xyz + (vec3<f32>(c) + 0.5) * cfg.lo.w;
}

fn s_flat(c: vec3<i32>) -> u32 {
    let n = cfg.dims.x;
    return u32(c.x) + n * (u32(c.y) + n * u32(c.z));
}

// The cells whose centres may lie within `lo`..`hi`, a hair wider.
fn s_range(lo: vec3<f32>, hi: vec3<f32>) -> array<vec3<i32>, 2> {
    let pad = cfg.lo.w * 1e-3;
    let a = ceil((lo - pad - cfg.lo.xyz) / cfg.lo.w - 0.5);
    let b = floor((hi + pad - cfg.lo.xyz) / cfg.lo.w - 0.5);
    let most = f32(cfg.dims.x) - 1.0;
    return array<vec3<i32>, 2>(
        vec3<i32>(clamp(a, vec3<f32>(0.0), vec3<f32>(most + 1.0))),
        vec3<i32>(clamp(b, vec3<f32>(-1.0), vec3<f32>(most))),
    );
}

fn s_half() -> f32 {
    return cfg.lo.w * 0.5;
}

// A share added to a word pair, a carry into the high word.
fn s_add(at: u32, v: u32) {
    let old = atomicAdd(&index[at], v);
    if old + v < old {
        atomicAdd(&index[at + 1u], 1u);
    }
}

// `v` added to the table's sum for `key`, its slot claimed where none
// holds it.
fn s_tally(key: u32, v: u32) {
    let mask = cfg.buckets - 1u;
    var t = mix32(key) & mask;
    for (var p = 0u; p < S_PROBES; p++) {
        let at = s_key(t);
        var k = atomicLoad(&index[at]);
        // cap: a weak exchange that failed spuriously is tried again.
        loop {
            if k != S_EMPTY {
                break;
            }
            let r = atomicCompareExchangeWeak(&index[at], S_EMPTY, key);
            if r.exchanged {
                atomicAdd(&index[S_PAIRS], 1u);
                k = key;
                break;
            }
            k = r.old_value;
        }
        if k == key {
            let old = atomicAdd(&index[s_sum(t)], v);
            if old + v < old {
                atomicAdd(&index[S_LOST], 1u);
            }
            return;
        }
        t = (t + 1u) & mask;
    }
    atomicAdd(&index[S_LOST], 1u);
}

// A leaf-bearing record's `leaves` in cell `c` on `system`.
fn s_leaf(c: u32, system: u32, leaves: f32) {
    atomicOr(&index[s_grid(c, 0u)], FOLIAGE);
    let v = u32(round(leaves * S_UNIT));
    s_add(s_grid(c, 2u), v);
    s_tally(c + s_cells() * system, v);
}

@compute @workgroup_size(64)
fn splat_clear(@builtin(global_invocation_id) id: vec3<u32>) {
    let w = index_of(id);
    if w >= cfg.n {
        return;
    }
    let empty = (w >= s_limb(0u) && w < s_sum(0u));
    atomicStore(&index[w], select(0u, S_EMPTY, empty));
}

// Each drawn node's wood (`field_build.wgsl`'s `wood`) into the cells
// whose circumsphere meets it.
@compute @workgroup_size(64)
fn splat_wood(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index_of(id);
    if i >= cfg.nodes || !made(i) {
        return;
    }
    let p = parent(i);
    let b = pos(i);
    let end = nodes[i * NW + 3u];
    var a = b;
    var start = end;
    if i != cfg.root && p < cfg.nodes {
        a = pos(p);
        start = nodes[i * NW + 4u];
    }
    let inflation = s_half() * sqrt(3.0);
    let r = s_range(min(a - start, b - end) - inflation, max(a + start, b + end) + inflation);
    let thickest = bitcast<u32>(max(start, end));
    for (var z = r[0].z; z <= r[1].z; z++) {
        for (var y = r[0].y; y <= r[1].y; y++) {
            for (var x = r[0].x; x <= r[1].x; x++) {
                let c = vec3<i32>(x, y, z);
                if contains(a, b, start, end, s_centre(c), inflation) {
                    let k = s_flat(c);
                    atomicOr(&index[s_grid(k, 0u)], WOOD);
                    atomicMax(&index[s_grid(k, 1u)], thickest);
                }
            }
        }
    }
}

// Each station segment's leaves (`field_build.wgsl`'s `leaves`): the
// wood's segment with a leaf's reach about it, on the limb system of the
// node it stands on. A frond's stand as its leaflets (`splat_leaflets`).
@compute @workgroup_size(64)
fn splat_leaves(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = index_of(id);
    if g >= cfg.segs {
        return;
    }
    let s = g * GW;
    let f = segment(s);
    if f[5].y < -1.5 {
        return;
    }
    let a = f[0].xyz;
    let b = f[1].xyz;
    let reach = max(f[0].w, f[1].w) * cfg.reach.x + cfg.reach.y + cfg.reach.z;
    let count = f32(segs[s + 25u]);
    let system = dense[atomicLoad(&work[at(VAL + cfg.flip, segs[cfg.segs * GW + g])])];
    let half = s_half();
    let inflation = half * sqrt(3.0);
    let r = s_range(min(a, b) - reach - inflation, max(a, b) + reach + inflation);
    for (var z = r[0].z; z <= r[1].z; z++) {
        for (var y = r[0].y; y <= r[1].y; y++) {
            for (var x = r[0].x; x <= r[1].x; x++) {
                let c = vec3<i32>(x, y, z);
                let centre = s_centre(c);
                if contains(a, b, reach, reach, centre, inflation) {
                    s_leaf(s_flat(c), system, count * inside(a, b, centre, half));
                }
            }
        }
    }
}

// Whether the closed cube of `half` about `p` meets the box about the
// midline `a`..`b`, `breadth` either way along `across` and `thickness`
// either way along their normal (`Cuboid::meets`, `field.wgsl`'s
// `meets_box` with its flank unquantized).
fn s_meets(a: vec3<f32>, b: vec3<f32>, across: vec3<f32>, breadth: f32, thickness: f32, p: vec3<f32>, half: f32) -> bool {
    let d = b - a;
    var along = d - across * dot(d, across);
    if dot(along, along) > 0.0 {
        along = normalize(along);
    } else {
        along = square_to(across);
    }
    let normal = cross(along, across);
    let centre = (a + b) * 0.5;
    let axes = array<vec3<f32>, 3>(along, across, normal);
    let extent = vec3<f32>(dot(d, along) * 0.5, breadth, thickness);
    for (var k = 0u; k < 3u; k++) {
        var u = vec3<f32>(0.0);
        u[k] = 1.0;
        let r = abs(dot(along, u)) * extent.x + abs(dot(across, u)) * extent.y + abs(dot(normal, u)) * extent.z;
        if p[k] + half < centre[k] - r || p[k] - half > centre[k] + r {
            return false;
        }
    }
    for (var k = 0u; k < 3u; k++) {
        let u = axes[k];
        let c = dot(p, u);
        let r = half * (abs(u.x) + abs(u.y) + abs(u.z));
        let at = dot(centre, u);
        if c + r < at - extent[k] || c - r > at + extent[k] {
            return false;
        }
    }
    return true;
}

// Each leaflet of each frond (`field_build.wgsl`'s `leaflets`, its rule
// kept line for line up to the box), one leaf a leaflet.
@compute @workgroup_size(64)
fn splat_leaflets(@builtin(global_invocation_id) id: vec3<u32>) {
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
    let system = dense[atomicLoad(&work[at(VAL + cfg.flip, segs[cfg.segs * GW + g])])];
    let heading = f[2].xyz;
    let grown = f[5].w;
    let mature = bitcast<f32>(segs[s + 28u]);
    let half = s_half();
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
        let margin = (length(c) + length(h)) * 0.005 + 0.001;
        let centre = point + flank * c.x + axis * c.y + face * c.z;
        let run = axis * (h.y + margin);
        let wide = h.x + margin;
        let thick = h.z + margin;
        let a = centre - run;
        let b = centre + run;
        let r = wide + thick;
        let cells = s_range(min(a, b) - r - half, max(a, b) + r + half);
        for (var z = cells[0].z; z <= cells[1].z; z++) {
            for (var y = cells[0].y; y <= cells[1].y; y++) {
                for (var x = cells[0].x; x <= cells[1].x; x++) {
                    let cell = vec3<i32>(x, y, z);
                    let p = s_centre(cell);
                    if s_meets(a, b, flank, wide, thick, p, half) {
                        s_leaf(s_flat(cell), system, inside(a, b, p, half));
                    }
                }
            }
        }
    }
}

// Each cell's most leaves of one system.
@compute @workgroup_size(64)
fn splat_best(@builtin(global_invocation_id) id: vec3<u32>) {
    let t = index_of(id);
    if t >= cfg.buckets {
        return;
    }
    let key = atomicLoad(&index[s_key(t)]);
    if key == S_EMPTY {
        return;
    }
    atomicMax(&index[s_best(key % s_cells())], atomicLoad(&index[s_sum(t)]));
}

// The least system holding them.
@compute @workgroup_size(64)
fn splat_limb(@builtin(global_invocation_id) id: vec3<u32>) {
    let t = index_of(id);
    if t >= cfg.buckets {
        return;
    }
    let key = atomicLoad(&index[s_key(t)]);
    if key == S_EMPTY {
        return;
    }
    let c = key % s_cells();
    if atomicLoad(&index[s_sum(t)]) == atomicLoad(&index[s_best(c)]) {
        atomicMin(&index[s_limb(c)], key / s_cells());
    }
}

// Each cell's answer (`field_query.wgsl`'s words), then the counters.
@compute @workgroup_size(64)
fn splat_finish(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = index_of(id);
    if c >= s_cells() {
        return;
    }
    let flags = atomicLoad(&index[s_grid(c, 0u)]);
    let lo = atomicLoad(&index[s_grid(c, 2u)]);
    let hi = atomicLoad(&index[s_grid(c, 3u)]);
    let leaves = (f32(hi) * 4294967296.0 + f32(lo)) / S_UNIT;
    var limb = NONE;
    if (flags & FOLIAGE) != 0u {
        limb = atomicLoad(&index[s_limb(c)]);
    }
    prims[c] = vec4<u32>(flags, atomicLoad(&index[s_grid(c, 1u)]), bitcast<u32>(leaves), limb);
    if c == 0u {
        prims[s_cells()] = vec4<u32>(atomicLoad(&index[S_PAIRS]), atomicLoad(&index[S_LOST]), 0u, 0u);
    }
}
