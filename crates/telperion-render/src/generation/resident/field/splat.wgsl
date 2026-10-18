// The field's one pass (fn-257 R8), appended to `field.wgsl`: every drawn
// record written into the cells of the grid it reaches by the CPU's cell
// rule (`pipeline/field/splat.rs`): wood where the cell's circumsphere
// meets its tapered sweep, the thickest radius so met; leaves where the
// circumsphere meets a leaf segment's reach (a leaflet's box: the cube
// itself), the share of its midline inside the cell, and the limb system
// with the most of them, the lower one on a tie.
//
// A cell's leaves by system are summed in a table keyed by the cell and
// the system (two words), sized from a bound on the pairs that can occur:
// every cell a record's range holds (`bound_leaves`).

const LOST: u32 = 0u;

const HEAD: u32 = 1u;

// A 4096th of a leaf, as the cells sum them.
const UNIT: f32 = 4096.0;

fn cells() -> u32 {
    return cfg.dims.w;
}

fn cell_word(c: u32, f: u32) -> u32 {
    return HEAD + c * 4u + f;
}

fn best_word(c: u32) -> u32 {
    return HEAD + 4u * cells() + c;
}

fn limb_word(c: u32) -> u32 {
    return HEAD + 5u * cells() + c;
}

fn centre(c: vec3<i32>) -> vec3<f32> {
    return cfg.lo.xyz + (vec3<f32>(c) + 0.5) * cfg.lo.w;
}

fn flat(c: vec3<i32>) -> u32 {
    let n = cfg.dims.x;
    return u32(c.x) + n * (u32(c.y) + n * u32(c.z));
}

fn half() -> f32 {
    return cfg.lo.w * 0.5;
}

// The cells whose centres may lie within `lo`..`hi`, a hair wider: the
// cell rule decides, never the range.
fn range(lo: vec3<f32>, hi: vec3<f32>) -> array<vec3<i32>, 2> {
    let pad = cfg.lo.w * 1e-3;
    let a = ceil((lo - pad - cfg.lo.xyz) / cfg.lo.w - 0.5);
    let b = floor((hi + pad - cfg.lo.xyz) / cfg.lo.w - 0.5);
    let most = f32(cfg.dims.x) - 1.0;
    return array<vec3<i32>, 2>(
        vec3<i32>(clamp(a, vec3<f32>(0.0), vec3<f32>(most + 1.0))),
        vec3<i32>(clamp(b, vec3<f32>(-1.0), vec3<f32>(most))),
    );
}

// A leaf segment's range, by its reach and the cell's circumsphere.
fn leaf_range(f: array<vec4<f32>, 6>) -> array<vec3<i32>, 2> {
    let r = leaf_reach(f) + half() * sqrt(3.0);
    return range(min(f[0].xyz, f[1].xyz) - r, max(f[0].xyz, f[1].xyz) + r);
}

// A leaflet's range, by its box and the cell's half side.
fn blade_range(l: Blade) -> array<vec3<i32>, 2> {
    let r = l.wide + l.thick + half();
    return range(min(l.a, l.b) - r, max(l.a, l.b) + r);
}

// The cells a range holds, added to the table's bound.
fn add_bound(r: array<vec3<i32>, 2>) {
    let d = max(r[1] - r[0] + 1, vec3<i32>(0));
    let v = u32(d.x) * u32(d.y) * u32(d.z);
    let b = WORDS * cfg.nodes + 6u;
    let old = atomicAdd(&work[b], v);
    if old + v < old {
        atomicAdd(&work[b + 1u], 1u);
    }
}

// The table's word `at` holding `want`, claimed for it where it held
// nothing: its value after the claim.
fn claimed(at: u32, want: u32) -> u32 {
    var k = atomicLoad(&table[at]);
    // cap: a weak exchange that failed spuriously is tried again.
    loop {
        if k != NONE {
            break;
        }
        let r = atomicCompareExchangeWeak(&table[at], NONE, want);
        if r.exchanged {
            return want;
        }
        k = r.old_value;
    }
    return k;
}

// `v` added to the table's sum for (`cell`, `system`), its slot claimed
// where none holds the pair: the cell's word first, then the system's,
// so a slot whose cell matches but whose system another pair took is
// passed by.
fn tally(cell: u32, system: u32, v: u32) {
    let mask = cfg.slots - 1u;
    var t = mix32(cell ^ mix32(system)) & mask;
    // cap: the table holds more slots than its bound on the pairs, so a
    // pair always finds its slot or a free one.
    for (var p = 0u; p < cfg.slots; p++) {
        if claimed(t, cell) == cell && claimed(cfg.slots + t, system) == system {
            let at = 2u * cfg.slots + t;
            let old = atomicAdd(&table[at], v);
            if old + v < old {
                atomicAdd(&grid[LOST], 1u);
            }
            return;
        }
        t = (t + 1u) & mask;
    }
    atomicAdd(&grid[LOST], 1u);
}

// A leaf record's `leaves` in cell `c` on `system`.
fn leaf(c: u32, system: u32, leaves: f32) {
    atomicOr(&grid[cell_word(c, 0u)], FOLIAGE);
    let v = u32(round(leaves * UNIT));
    let at = cell_word(c, 2u);
    let old = atomicAdd(&grid[at], v);
    if old + v < old {
        atomicAdd(&grid[at + 1u], 1u);
    }
    tally(c, system, v);
}

@compute @workgroup_size(64)
fn grid_clear(@builtin(global_invocation_id) id: vec3<u32>) {
    let w = index_of(id);
    if w >= cfg.n {
        return;
    }
    atomicStore(&grid[w], select(0u, NONE, w >= limb_word(0u)));
}

@compute @workgroup_size(64)
fn table_clear(@builtin(global_invocation_id) id: vec3<u32>) {
    let w = index_of(id);
    if w >= cfg.n {
        return;
    }
    atomicStore(&table[w], select(0u, NONE, w < 2u * cfg.slots));
}

// Every leaf record's range, summed: the bound on the table's pairs.
@compute @workgroup_size(64)
fn bound_leaves(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = index_of(id);
    if g >= cfg.segs {
        return;
    }
    let f = segment(g);
    if !frond(f) {
        add_bound(leaf_range(f));
        return;
    }
    for (var j = 0u; j < segs[g * GW + 25u]; j++) {
        add_bound(blade_range(blade(g, f, j)));
    }
}

// Each drawn node's wood into the cells whose circumsphere meets it.
@compute @workgroup_size(64)
fn splat_wood(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index_of(id);
    if i >= cfg.nodes || !made(i) {
        return;
    }
    let w = sweep_of(i);
    let inflation = half() * sqrt(3.0);
    let r = range(min(w.a - w.start, w.b - w.end) - inflation, max(w.a + w.start, w.b + w.end) + inflation);
    let thickest = bitcast<u32>(max(w.start, w.end));
    for (var z = r[0].z; z <= r[1].z; z++) {
        for (var y = r[0].y; y <= r[1].y; y++) {
            for (var x = r[0].x; x <= r[1].x; x++) {
                let c = vec3<i32>(x, y, z);
                if contains(w.a, w.b, w.start, w.end, centre(c), inflation) {
                    let k = flat(c);
                    atomicOr(&grid[cell_word(k, 0u)], WOOD);
                    atomicMax(&grid[cell_word(k, 1u)], thickest);
                }
            }
        }
    }
}

// Each leaf segment: the wood's segment with a leaf's reach about it, on
// the limb system of the node it stands on.
@compute @workgroup_size(64)
fn splat_leaves(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = index_of(id);
    if g >= cfg.segs {
        return;
    }
    let f = segment(g);
    if frond(f) {
        return;
    }
    let a = f[0].xyz;
    let b = f[1].xyz;
    let reach = leaf_reach(f);
    let count = f32(segs[g * GW + 25u]);
    let system = system_of(g);
    let inflation = half() * sqrt(3.0);
    let r = leaf_range(f);
    for (var z = r[0].z; z <= r[1].z; z++) {
        for (var y = r[0].y; y <= r[1].y; y++) {
            for (var x = r[0].x; x <= r[1].x; x++) {
                let c = vec3<i32>(x, y, z);
                let p = centre(c);
                if contains(a, b, reach, reach, p, inflation) {
                    leaf(flat(c), system, count * inside(a, b, p, half()));
                }
            }
        }
    }
}

// Whether the closed cube of `half` about `p` meets the leaflet's box
// (`Shape::meets`): the world's axes and the box's tried as separating
// axes.
fn meets(l: Blade, p: vec3<f32>, h: f32) -> bool {
    let d = l.b - l.a;
    var along = d - l.flank * dot(d, l.flank);
    if dot(along, along) > 0.0 {
        along = normalize(along);
    } else {
        along = square_to(l.flank);
    }
    let normal = cross(along, l.flank);
    let mid = (l.a + l.b) * 0.5;
    let axes = array<vec3<f32>, 3>(along, l.flank, normal);
    let extent = vec3<f32>(dot(d, along) * 0.5, l.wide, l.thick);
    for (var k = 0u; k < 3u; k++) {
        var u = vec3<f32>(0.0);
        u[k] = 1.0;
        let r = abs(dot(along, u)) * extent.x + abs(dot(l.flank, u)) * extent.y + abs(dot(normal, u)) * extent.z;
        if p[k] + h < mid[k] - r || p[k] - h > mid[k] + r {
            return false;
        }
    }
    for (var k = 0u; k < 3u; k++) {
        let u = axes[k];
        let c = dot(p, u);
        let r = h * (abs(u.x) + abs(u.y) + abs(u.z));
        let at = dot(mid, u);
        if c + r < at - extent[k] || c - r > at + extent[k] {
            return false;
        }
    }
    return true;
}

// Each leaflet of each frond, one leaf a leaflet.
@compute @workgroup_size(64)
fn splat_leaflets(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = index_of(id);
    if g >= cfg.segs {
        return;
    }
    let f = segment(g);
    if !frond(f) {
        return;
    }
    let system = system_of(g);
    for (var j = 0u; j < segs[g * GW + 25u]; j++) {
        let l = blade(g, f, j);
        let r = blade_range(l);
        for (var z = r[0].z; z <= r[1].z; z++) {
            for (var y = r[0].y; y <= r[1].y; y++) {
                for (var x = r[0].x; x <= r[1].x; x++) {
                    let c = vec3<i32>(x, y, z);
                    let p = centre(c);
                    if meets(l, p, half()) {
                        leaf(flat(c), system, inside(l.a, l.b, p, half()));
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
    if t >= cfg.slots {
        return;
    }
    let cell = atomicLoad(&table[t]);
    if cell == NONE || atomicLoad(&table[cfg.slots + t]) == NONE {
        return;
    }
    atomicMax(&grid[best_word(cell)], atomicLoad(&table[2u * cfg.slots + t]));
}

// The least system holding them.
@compute @workgroup_size(64)
fn splat_limb(@builtin(global_invocation_id) id: vec3<u32>) {
    let t = index_of(id);
    if t >= cfg.slots {
        return;
    }
    let cell = atomicLoad(&table[t]);
    let system = atomicLoad(&table[cfg.slots + t]);
    if cell == NONE || system == NONE {
        return;
    }
    if atomicLoad(&table[2u * cfg.slots + t]) == atomicLoad(&grid[best_word(cell)]) {
        atomicMin(&grid[limb_word(cell)], system);
    }
}

// Each cell's answer: its flags, its thickest radius's bits, its leaves'
// bits and its limb system (none without foliage); then the lost count.
@compute @workgroup_size(64)
fn splat_finish(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = index_of(id);
    if c >= cells() {
        return;
    }
    let flags = atomicLoad(&grid[cell_word(c, 0u)]);
    let lo = atomicLoad(&grid[cell_word(c, 2u)]);
    let hi = atomicLoad(&grid[cell_word(c, 3u)]);
    let leaves = (f32(hi) * 4294967296.0 + f32(lo)) / UNIT;
    var limb = NONE;
    if (flags & FOLIAGE) != 0u {
        limb = atomicLoad(&grid[limb_word(c)]);
    }
    answers[c] = vec4<u32>(flags, atomicLoad(&grid[cell_word(c, 1u)]), bitcast<u32>(leaves), limb);
    if c == 0u {
        answers[cells()] = vec4<u32>(atomicLoad(&grid[LOST]), 0u, 0u, 0u);
    }
}
