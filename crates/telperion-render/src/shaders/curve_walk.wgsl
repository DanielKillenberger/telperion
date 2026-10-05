// The walk one cluster's rings are surfaced by (fn-208): its points, its
// mode, its ring levels, pieces and sides, and the vertices and indices it
// writes or counts. Concatenated after curve.wgsl's bindings.

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

fn point(i: u32) -> Point {
    let b = i * POINT_WORDS;
    let w5 = points[b + 5u];
    let w6 = points[b + 6u];
    let w7 = points[b + 7u];
    var p: Point;
    p.centre = vec3(bitcast<f32>(points[b]), bitcast<f32>(points[b + 1u]), bitcast<f32>(points[b + 2u]));
    p.radius = bitcast<f32>(points[b + 3u]);
    p.along = bitcast<f32>(points[b + 4u]);
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

fn sides(rho: f32, error: f32) -> u32 {
    var exact = 3.0;
    if (rho > error) { exact = PI / acos(1.0 - error / rho); }
    var n = 3u;
    loop {
        if (f32(n) >= exact || n >= MOST_SIDES) { break; }
        n *= 2u;
    }
    return n;
}

fn pieces(a: Point, b: Point, error: f32) -> u32 {
    let turn = acos(clamp(dot(tangent(a), tangent(b)), -1.0, 1.0));
    let length = distance(a.centre, b.centre);
    let pixels = max(pixels_at(a.centre), pixels_at(b.centre));
    let m = ceil(sqrt(length * turn * pixels / (8.0 * error)));
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

fn section_ring(s: u32, k: u32, samples: u32) -> vec3<f32> {
    let r = s * SECTION_FLOATS + 17u + 3u * (k + 3u - clamp(samples, 2u, 3u));
    return vec3(sections[r], sections[r + 1u], sections[r + 2u]); // reach, rise, scale
}

fn section_vec(s: u32, at: u32) -> vec3<f32> {
    let b = s * SECTION_FLOATS + at;
    return vec3(sections[b], sections[b + 1u], sections[b + 2u]);
}

// A shaped run's ring point, and its normal out from the ring's centre.
fn cell_vertex(s: u32, k: u32, samples: u32, angle: f32) -> array<vec3<f32>, 2> {
    let ring = section_ring(s, k, samples);
    let origin = section_vec(s, 0u);
    let axis = section_vec(s, 3u);
    let radial = section_vec(s, 6u);
    let across = section_vec(s, 9u);
    let b = s * SECTION_FLOATS;
    let corners = vec4(sections[b + 12u], sections[b + 13u], sections[b + 14u], sections[b + 15u]);
    let flatness = sections[b + 16u];
    let c = cos(angle);
    let sn = sin(angle);
    let diamond = 1.0 / (abs(c) + abs(sn));
    let blend = 1.0 + flatness * (diamond - 1.0);
    let u = c * blend * ring.z;
    let v = sn * blend * ring.z;
    let turn = corners.x * u + corners.z * v;
    let up = corners.y * u + corners.w * v;
    let p = origin + axis * (ring.y + up) + (radial * cos(turn) + across * sin(turn)) * ring.x;
    let centre = origin + axis * ring.y + radial * ring.x;
    return array(p, normalize(p - centre));
}

fn tube_vertex(p: Point, angle: f32, slope: f32) -> array<vec3<f32>, 2> {
    let c = cos(angle);
    let s = sin(angle);
    let radial = p.normal * c + p.binormal * s;
    let around = p.binormal * c - p.normal * s;
    let phase = TAU * cfg.twist_rate * (p.along / cfg.height);
    var profile = 1.0;
    var turn = 0.0;
    if (cfg.lobes > 0u) {
        let a = f32(cfg.lobes) * (angle + phase);
        profile = 1.0 + cfg.lobe_depth * cos(a);
        turn = -cfg.lobe_depth * f32(cfg.lobes) * sin(a);
    }
    let width = p.radius * profile;
    let normal = radial * width - around * (p.radius * turn) - tangent(p) * (width * slope * profile);
    return array(p.centre + radial * width, normalize(normal));
}

fn put(at: u32, position: vec3<f32>, normal: vec3<f32>, along: f32, angle: f32,
    radius: f32, coverage: f32) {
    let b = at * VERTEX_FLOATS;
    vertices[b] = position.x;
    vertices[b + 1u] = position.y;
    vertices[b + 2u] = position.z;
    vertices[b + 3u] = normal.x;
    vertices[b + 4u] = normal.y;
    vertices[b + 5u] = normal.z;
    vertices[b + 6u] = along;
    vertices[b + 7u] = angle;
    vertices[b + 8u] = radius;
    vertices[b + 9u] = coverage;
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

// What a walk has written so far, and the ring it holds back until it knows
// whether the stretch after it is a tube's.
struct Walk {
    counts: vec3<u32>,   // vertices, tube indices, ribbon indices
    rings: u32,
    pending: Ring,
    before: bool,        // the stretch before the pending ring is a tube's
    tube_start: u32,     // the previous ring's polygon and ribbon vertices
    ribbon_start: u32,
    sides: u32,
};

fn zipper(w: ptr<function, Walk>, write: bool, base: vec3<u32>, a: u32, na: u32, b: u32, nb: u32) {
    var i = 0u;
    var j = 0u;
    loop {
        if (i >= na && j >= nb) { break; }
        let at = base.y + (*w).counts.y;
        if (j >= nb || (i < na && (i + 1u) * nb <= (j + 1u) * na)) {
            if (write) {
                indices[at] = base.x + a + i % na;
                indices[at + 1u] = base.x + a + (i + 1u) % na;
                indices[at + 2u] = base.x + b + j % nb;
            }
            i += 1u;
        } else {
            if (write) {
                indices[at] = base.x + a + i % na;
                indices[at + 1u] = base.x + b + (j + 1u) % nb;
                indices[at + 2u] = base.x + b + j % nb;
            }
            j += 1u;
        }
        (*w).counts.y += 3u;
    }
}

fn cap(w: ptr<function, Walk>, write: bool, base: vec3<u32>, p: Point, start: u32, n: u32,
    away: f32) {
    let centre = (*w).counts.x;
    if (write) {
        put(base.x + centre, p.centre, tangent(p) * away, p.along, 0.0, p.radius, 1.0);
        for (var k = 0u; k < n; k++) {
            let at = base.y + (*w).counts.y + 3u * k;
            indices[at] = base.x + centre;
            indices[at + 1u] = base.x + start + k;
            indices[at + 2u] = base.x + start + (k + 1u) % n;
        }
    }
    (*w).counts.x += 1u;
    (*w).counts.y += 3u * n;
}

// The pending ring, now that whether a tube stretch follows it is known: its
// polygon where a tube stretch meets it (stitched to the previous one, capped
// where its tube ends), its two ribbon vertices where a ribbon stretch meets
// it (joined to the previous ring's). As the CPU reference's `emit`.
fn settle(w: ptr<function, Walk>, after: bool, has_next: bool, write: bool, base: vec3<u32>) {
    let ring = (*w).pending;
    let p = ring.p;
    let first = (*w).rings == 0u;
    let before = (*w).before;
    let ribbon = cfg.ribbons != 0u && ((!first && !before) || (has_next && !after));
    var tube_start = 0u;
    if (before || after) {
        tube_start = (*w).counts.x;
        if (write) {
            for (var k = 0u; k < ring.sides; k++) {
                let angle = TAU * f32(k) / f32(ring.sides);
                var v: array<vec3<f32>, 2>;
                if (ring.section != 0xFFFFFFFFu) {
                    v = cell_vertex(ring.section, ring.index, ring.samples, angle);
                } else {
                    v = tube_vertex(p, angle, ring.slope);
                }
                put(base.x + tube_start + k, v[0], v[1], p.along, angle, p.radius, 1.0);
            }
        }
        (*w).counts.x += ring.sides;
        if (before) { zipper(w, write, base, (*w).tube_start, (*w).sides, tube_start, ring.sides); }
    }
    var ribbon_start = 0u;
    if (ribbon) {
        ribbon_start = (*w).counts.x;
        var view = cfg.forward.xyz;
        if (cfg.orthographic == 0u && dot(p.centre - cfg.eye.xyz, p.centre - cfg.eye.xyz) > 0.0) {
            view = normalize(p.centre - cfg.eye.xyz);
        }
        let t = tangent(p);
        let across = cross(t, view);
        var side = p.binormal;
        if (dot(across, across) > 1e-12) { side = normalize(across); }
        var facing = -view - t * dot(-view, t);
        if (dot(facing, facing) > 1e-12) { facing = normalize(facing); } else { facing = p.normal; }
        let half = max(p.radius, 0.5 / pixels_at(p.centre));
        let coverage = p.radius / half;
        if (write) {
            let normals = array<vec3<f32>, 3>(-side, facing, side);
            for (var k = 0u; k < 3u; k++) {
                let n = normals[k];
                let angle = atan2(dot(n, p.binormal), dot(n, p.normal));
                let at = p.centre + side * (half * (f32(k) - 1.0));
                put(base.x + ribbon_start + k, at, n, p.along, angle, p.radius, coverage);
            }
        }
        (*w).counts.x += 3u;
        if (!first && !before) {
            if (write) {
                let a = base.x + (*w).ribbon_start;
                let b = base.x + ribbon_start;
                let at = base.z + (*w).counts.z;
                let order = array<u32, 12>(a, a + 1u, b, b, a + 1u, b + 1u,
                    a + 1u, a + 2u, b + 1u, b + 1u, a + 2u, b + 2u);
                for (var k = 0u; k < 12u; k++) { indices[at + k] = order[k]; }
            }
            (*w).counts.z += 12u;
        }
    }
    if (before || after) {
        if (!before) { cap(w, write, base, p, tube_start, ring.sides, -1.0); }
        if (!after) { cap(w, write, base, p, tube_start, ring.sides, 1.0); }
    }
    (*w).tube_start = tube_start;
    (*w).ribbon_start = ribbon_start;
    (*w).sides = ring.sides;
    (*w).before = after;
    (*w).rings += 1u;
}

// A new ring: the pending one settles against it, and it waits in its place.
fn push(w: ptr<function, Walk>, ring: Ring, held: bool, write: bool, base: vec3<u32>) {
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

// Cluster `i` at `error`, counted, or written at `base` (first vertex, first
// tube index, first ribbon index) where `write`.
fn walk(i: u32, error: f32, write: bool, base: vec3<u32>) -> vec3<u32> {
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
