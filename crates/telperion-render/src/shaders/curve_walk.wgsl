// The walk one cluster's rings are planned by (fn-208): its points decoded,
// its frustum test, its ring levels, Hermite pieces and sides, and each
// ring's record - where its vertices and indices go and what it joins -
// written or only counted. Concatenated after curve.wgsl.

struct Point {
    centre: vec3<f32>,
    radius: f32,
    along: f32,
    normal: vec3<f32>,
    binormal: vec3<f32>,
};

struct Cluster {
    first: u32,
    count: u32,
    run_first: u32,
    run_count: u32,
    flags: u32,
    centre: vec3<f32>,
    reach: f32,
    largest: f32,
};

fn snorm24(w: u32) -> f32 {
    return f32(bitcast<i32>(w << 8u) >> 8u) / 8388607.0;
}

fn signum(x: f32) -> f32 {
    return select(-1.0, 1.0, x >= 0.0);
}

fn unoctahedral(a: u32, b: u32) -> vec3<f32> {
    var x = snorm24(a);
    var y = snorm24(b);
    let z = 1.0 - abs(x) - abs(y);
    if (z < 0.0) {
        let ox = x;
        x = (1.0 - abs(y)) * signum(ox);
        y = (1.0 - abs(ox)) * signum(y);
    }
    return normalize(vec3(x, y, z));
}

// A word of the points, from whichever binding holds it.
fn word(at: u32) -> u32 {
    if (at < cfg.point_slice) { return points0[at]; }
    return points1[at - cfg.point_slice];
}

fn point(i: u32) -> Point {
    let b = i * POINT_WORDS;
    let w5 = word(b + 5u);
    let w6 = word(b + 6u);
    let w7 = word(b + 7u);
    var p: Point;
    p.centre = vec3(bitcast<f32>(word(b)), bitcast<f32>(word(b + 1u)), bitcast<f32>(word(b + 2u)));
    p.radius = bitcast<f32>(word(b + 3u));
    p.along = bitcast<f32>(word(b + 4u));
    p.normal = unoctahedral(w5 & 0xFFFFFFu, (w5 >> 24u) | ((w6 & 0xFFFFu) << 8u));
    p.binormal = unoctahedral((w6 >> 16u) | ((w7 & 0xFFu) << 16u), w7 >> 8u);
    return p;
}

fn cluster(i: u32) -> Cluster {
    let b = i * CLUSTER_WORDS;
    var c: Cluster;
    c.first = clusters[b];
    c.count = clusters[b + 1u];
    c.run_first = clusters[b + 2u];
    c.run_count = clusters[b + 3u];
    c.flags = clusters[b + 4u];
    c.centre = vec3(bitcast<f32>(clusters[b + 5u]), bitcast<f32>(clusters[b + 6u]), bitcast<f32>(clusters[b + 7u]));
    c.reach = bitcast<f32>(clusters[b + 8u]);
    c.largest = bitcast<f32>(clusters[b + 9u]);
    return c;
}

fn ladder(i: u32, k: u32) -> f32 {
    return bitcast<f32>(clusters[i * CLUSTER_WORDS + 10u + k]);
}

fn tangent(p: Point) -> vec3<f32> {
    return cross(p.normal, p.binormal);
}

fn pixels_at(c: vec3<f32>) -> f32 {
    if (cfg.orthographic != 0u) { return cfg.forward.w; }
    return cfg.forward.w / max(dot(c - cfg.eye.xyz, cfg.forward.xyz), cfg.eye.w);
}

fn nearest(c: Cluster) -> f32 {
    if (cfg.orthographic != 0u) { return cfg.forward.w; }
    let depth = dot(c.centre - cfg.eye.xyz, cfg.forward.xyz) - c.reach;
    return cfg.forward.w / max(depth, cfg.eye.w);
}

fn lobe() -> f32 {
    return select(1.0, 1.0 + abs(cfg.lobe_depth), cfg.lobes > 0u);
}

// Whether any of cluster `i` stands inside the frustum.
fn visible(i: u32) -> bool {
    if (cfg.culling == 0u) { return true; }
    let c = cluster(i);
    for (var k = 0u; k < 6u; k++) {
        let p = cfg.planes[k];
        if (dot(p.xyz, c.centre) + p.w < -c.reach) { return false; }
    }
    return true;
}

// How much more a lobed outline bends than its circle (the CPU's `bend`).
fn bend() -> f32 {
    if (cfg.lobes == 0u || cfg.lobe_depth == 0.0) { return 1.0; }
    let n = f32(cfg.lobes) + 1.0;
    let d = abs(cfg.lobe_depth);
    return (1.0 + d * n * n) / (1.0 + d);
}

// Twisting lobes' turn in lobe radians a metre along, and their depth.
fn twisting() -> vec2<f32> {
    if (cfg.lobes == 0u || cfg.lobe_depth == 0.0 || cfg.twist_rate == 0.0) { return vec2(0.0); }
    let rate = TAU * cfg.twist_rate / cfg.height;
    return vec2(f32(cfg.lobes) * abs(rate), abs(cfg.lobe_depth));
}

fn sides(rho: f32, error: f32) -> u32 {
    var exact = 3.0;
    if (rho > error) { exact = PI / acos(1.0 - error / rho); }
    let b = bend();
    if (b > 1.0) { exact = max(exact, TAU * sqrt(rho * b / (8.0 * error))); }
    var n = 3u;
    loop {
        if (f32(n) >= exact || n >= MOST_SIDES) { break; }
        n *= 2u;
    }
    return n;
}

// The CPU's `pieces`: the turn's sag, the cubic's control bound and the
// twisting lobes', the most of them.
fn pieces(a: Point, b: Point, error: f32) -> u32 {
    let ta = tangent(a);
    let tb = tangent(b);
    let turn = acos(clamp(dot(ta, tb), -1.0, 1.0));
    let span = distance(a.centre, b.centre);
    let pixels = max(pixels_at(a.centre), pixels_at(b.centre));
    let chord = b.centre - a.centre;
    let bent = max(length(chord - (ta * 2.0 + tb) * (span / 3.0)),
        length(chord - (ta + tb * 2.0) * (span / 3.0)));
    let twist = twisting();
    let phase = twist.x * abs(b.along - a.along) * sqrt(twist.y);
    let sag = max(span * turn, 6.0 * bent) + max(a.radius, b.radius) * phase * phase;
    let m = ceil(sqrt(sag * pixels / (8.0 * error)));
    return clamp(u32(m), 1u, MOST_PIECES);
}

fn hermite(a: Point, b: Point, t: f32) -> Point {
    if (t == 0.0) { return a; }
    let length = distance(a.centre, b.centre);
    let ta = tangent(a) * length;
    let tb = tangent(b) * length;
    let t2 = t * t;
    let t3 = t2 * t;
    var p: Point;
    p.centre = a.centre * (2.0 * t3 - 3.0 * t2 + 1.0) + ta * (t3 - 2.0 * t2 + t)
        + b.centre * (3.0 * t2 - 2.0 * t3) + tb * (t3 - t2);
    let along = a.centre * (6.0 * t2 - 6.0 * t) + ta * (3.0 * t2 - 4.0 * t + 1.0)
        + b.centre * (6.0 * t - 6.0 * t2) + tb * (3.0 * t2 - 2.0 * t);
    let direction = normalize(along);
    let mixed = mix(a.normal, b.normal, t);
    p.normal = normalize(mixed - direction * dot(mixed, direction));
    p.binormal = cross(direction, p.normal);
    p.radius = a.radius + (b.radius - a.radius) * t;
    p.along = a.along + (b.along - a.along) * t;
    return p;
}

fn slope(g: u32, first: u32, count: u32) -> f32 {
    let a = point(max(select(g - 1u, g, g == 0u), first));
    let b = point(min(g + 1u, first + count - 1u));
    let run = b.along - a.along;
    return select((b.radius - a.radius) / run, 0.0, abs(run) < 1e-12);
}

// One ring of a cluster: its point, the radius's slope, its sides, whether
// it stands wide enough for a tube, and for a shaped run its cell ring
// (section, index, count; section ~0u for none).
struct Ring {
    p: Point,
    slope: f32,
    sides: u32,
    tube: bool,
    section: u32,
    index: u32,
    samples: u32,
};

// What a walk has counted (rings, vertices, tube indices, ribbon indices),
// and the ring it holds back until it knows whether the stretch after it is
// a tube's.
struct Walk {
    counts: vec4<u32>,
    pending: Ring,
    before: bool,        // the stretch before the pending ring is a tube's
    tube_start: u32,     // the previous ring's polygon and ribbon, as global
    ribbon_start: u32,   // vertex indices, and its sides
    sides: u32,
};

// Record flags: the ring's polygon, the strip to the previous polygon, the
// caps at its start and end, its ribbon vertices, the quad to the previous
// ribbon.
const POLYGON: u32 = 1u;
const STRIP: u32 = 2u;
const CAP_START: u32 = 4u;
const CAP_END: u32 = 8u;
const RIBBON: u32 = 16u;
const QUAD: u32 = 32u;

fn put_record(at: u32, ring: Ring, flags: u32, starts: vec3<u32>, previous: vec3<u32>) {
    let p = ring.p;
    let f = array<f32, 12>(p.centre.x, p.centre.y, p.centre.z, p.radius, p.along,
        p.normal.x, p.normal.y, p.normal.z, p.binormal.x, p.binormal.y, p.binormal.z, ring.slope);
    for (var k = 0u; k < 12u; k++) { set_record(at, k, bitcast<u32>(f[k])); }
    set_record(at, 12u, ring.sides);
    set_record(at, 13u, flags);
    set_record(at, 14u, ring.section);
    set_record(at, 15u, ring.index | (ring.samples << 16u));
    set_record(at, 16u, starts.x);
    set_record(at, 17u, starts.y);
    set_record(at, 18u, starts.z);
    set_record(at, 19u, previous.x);
    set_record(at, 20u, previous.y);
    set_record(at, 21u, previous.z);
}

// The pending ring, now that whether a tube stretch follows it is known: its
// polygon where a tube stretch meets it (stitched to the previous one, capped
// where its tube ends), its three ribbon vertices where a ribbon stretch
// meets it (joined to the previous ring's). As the CPU reference's `emit`,
// counted here and written by `emit_ring`. `base`: the cluster's first
// ring, vertex, tube index and ribbon index.
fn settle(w: ptr<function, Walk>, after: bool, has_next: bool, write: bool, base: vec4<u32>) {
    let ring = (*w).pending;
    let first = (*w).counts.x == 0u;
    let before = (*w).before;
    let polygon = before || after;
    let ribbon = (!first && !before) || (has_next && !after);
    var flags = 0u;
    let v = base.y + (*w).counts.y;
    let t = base.z + (*w).counts.z;
    let r = base.w + (*w).counts.w;
    var tube_start = 0u;
    if (polygon) {
        flags |= POLYGON;
        tube_start = base.y + (*w).counts.y;
        (*w).counts.y += ring.sides;
        if (before) {
            flags |= STRIP;
            (*w).counts.z += 3u * ((*w).sides + ring.sides);
        }
    }
    var ribbon_start = 0u;
    if (ribbon) {
        flags |= RIBBON;
        ribbon_start = base.y + (*w).counts.y;
        (*w).counts.y += 3u;
        if (!first && !before) {
            flags |= QUAD;
            (*w).counts.w += 12u;
        }
    }
    if (polygon) {
        if (!before) { flags |= CAP_START; (*w).counts.y += 1u; (*w).counts.z += 3u * ring.sides; }
        if (!after) { flags |= CAP_END; (*w).counts.y += 1u; (*w).counts.z += 3u * ring.sides; }
    }
    if (write && base.x + (*w).counts.x < cfg.ring_budget) {
        let previous = vec3((*w).tube_start, (*w).sides, (*w).ribbon_start);
        put_record(base.x + (*w).counts.x, ring, flags, vec3(v, t, r), previous);
    }
    (*w).counts.x += 1u;
    (*w).tube_start = tube_start;
    (*w).ribbon_start = ribbon_start;
    (*w).sides = ring.sides;
    (*w).before = after;
}

// A new ring: the pending one settles against it, and it waits in its place.
fn push(w: ptr<function, Walk>, ring: Ring, held: bool, write: bool, base: vec4<u32>) {
    if (held) {
        settle(w, (*w).pending.tube && ring.tube, true, write, base);
    }
    (*w).pending = ring;
}

fn ring_at(p: Point, slope: f32, error: f32, shaped: bool) -> Ring {
    var r: Ring;
    r.p = p;
    r.slope = slope;
    let rho = p.radius * lobe() * pixels_at(p.centre);
    r.sides = sides(rho, 0.5 * error);
    r.tube = shaped || rho >= cfg.ribbon;
    r.section = 0xFFFFFFFFu;
    return r;
}

// Cluster `i` at `error`: its counts (rings, vertices, tube indices, ribbon
// indices), and where `write` its ring records at `base`.
fn walk(i: u32, error: f32, write: bool, base: vec4<u32>) -> vec4<u32> {
    var w: Walk;
    let c = cluster(i);
    let half = 0.5 * error;
    var held = false;
    if ((c.flags & 4u) != 0u) {
        for (var k = 0u; k < c.count; k++) {
            var r = ring_at(point(c.first + k), slope(c.first + k, c.run_first, c.run_count), error, true);
            r.section = c.flags >> 8u;
            r.index = k;
            r.samples = c.count;
            push(&w, r, held, write, base);
            held = true;
        }
    } else {
        let pixels = nearest(c);
        var level = 0u;
        for (var k = LEVELS; k > 0u; k--) {
            if (ladder(i, k - 1u) * pixels <= half) { level = k - 1u; break; }
        }
        let step = 1u << level;
        let last = c.count - 1u;
        var ka = 0u;
        loop {
            if (ka >= last) { break; }
            let kb = min(ka + step, last);
            let a = point(c.first + ka);
            let b = point(c.first + kb);
            let sa = slope(c.first + ka, c.run_first, c.run_count);
            let sb = slope(c.first + kb, c.run_first, c.run_count);
            let n = pieces(a, b, half);
            for (var j = 0u; j < n; j++) {
                let t = f32(j) / f32(n);
                push(&w, ring_at(hermite(a, b, t), sa + (sb - sa) * t, error, false), held, write, base);
                held = true;
            }
            ka = kb;
        }
        let end = c.first + last;
        push(&w, ring_at(point(end), slope(end, c.run_first, c.run_count), error, false), held, write, base);
        held = true;
    }
    if (held) { settle(&w, false, false, write, base); }
    return w.counts;
}
