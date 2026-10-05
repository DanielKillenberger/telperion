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

// 0 outside the frustum, 1 a tube, 2 a ribbon.
fn mode(i: u32) -> u32 {
    let c = cluster(i);
    if (cfg.culling != 0u) {
        for (var k = 0u; k < 6u; k++) {
            let p = cfg.planes[k];
            if (dot(p.xyz, c.centre) + p.w < -c.reach) { return 0u; }
        }
    }
    let shaped = (c.flags & 4u) != 0u;
    if (shaped || c.largest * lobe() * nearest(c) >= RIBBON) { return 1u; }
    return select(0u, 2u, cfg.ribbons != 0u);
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

// One ring of a cluster: its point, the radius's slope, and for a shaped run
// its cell ring (section, index, count; section ~0u for none).
struct Ring {
    p: Point,
    slope: f32,
    section: u32,
    index: u32,
    samples: u32,
};

// What a walk has written so far, and what it remembers of its rings.
struct Walk {
    counts: vec3<u32>,   // vertices, tube indices, ribbon indices
    rings: u32,
    first_start: u32,
    first_sides: u32,
    first_point: Point,
    last_start: u32,
    last_sides: u32,
    last_point: Point,
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

fn add_ring(w: ptr<function, Walk>, ring: Ring, tube: bool, error: f32, write: bool,
    base: vec3<u32>) {
    let p = ring.p;
    let start = (*w).counts.x;
    if (!tube) {
        var view = cfg.forward.xyz;
        if (cfg.orthographic == 0u && dot(p.centre - cfg.eye.xyz, p.centre - cfg.eye.xyz) > 0.0) {
            view = normalize(p.centre - cfg.eye.xyz);
        }
        let across = cross(tangent(p), view);
        var side = p.binormal;
        if (dot(across, across) > 1e-12) { side = normalize(across); }
        let half = max(p.radius, 0.5 / pixels_at(p.centre));
        let coverage = p.radius / half;
        if (write) {
            put(base.x + start, p.centre - side * half, -view, p.along, 0.0, p.radius, coverage);
            put(base.x + start + 1u, p.centre + side * half, -view, p.along, 0.0, p.radius, coverage);
        }
        if ((*w).rings > 0u) {
            if (write) {
                let a = base.x + start - 2u;
                let b = base.x + start;
                let at = base.z + (*w).counts.z;
                indices[at] = a;
                indices[at + 1u] = a + 1u;
                indices[at + 2u] = b;
                indices[at + 3u] = b;
                indices[at + 4u] = a + 1u;
                indices[at + 5u] = b + 1u;
            }
            (*w).counts.z += 6u;
        }
        (*w).counts.x += 2u;
        (*w).rings += 1u;
        return;
    }
    let n = sides(p.radius * lobe() * pixels_at(p.centre), 0.5 * error);
    if (write) {
        for (var k = 0u; k < n; k++) {
            let angle = TAU * f32(k) / f32(n);
            var v: array<vec3<f32>, 2>;
            if (ring.section != 0xFFFFFFFFu) {
                v = cell_vertex(ring.section, ring.index, ring.samples, angle);
            } else {
                v = tube_vertex(p, angle, ring.slope);
            }
            put(base.x + start + k, v[0], v[1], p.along, angle, p.radius, 1.0);
        }
    }
    (*w).counts.x += n;
    if ((*w).rings > 0u) {
        zipper(w, write, base, (*w).last_start, (*w).last_sides, start, n);
    } else {
        (*w).first_start = start;
        (*w).first_sides = n;
        (*w).first_point = p;
    }
    (*w).last_start = start;
    (*w).last_sides = n;
    (*w).last_point = p;
    (*w).rings += 1u;
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

// Cluster `i` at `error`, its tube or ribbon, counted, or written at `base`
// (first vertex, first tube index, first ribbon index) where `write`.
fn walk(i: u32, m: u32, error: f32, write: bool, base: vec3<u32>) -> vec3<u32> {
    var w: Walk;
    let c = cluster(i);
    let tube = m == 1u;
    let half = 0.5 * error;
    var ring: Ring;
    ring.section = 0xFFFFFFFFu;
    if ((c.flags & 4u) != 0u) {
        for (var k = 0u; k < c.count; k++) {
            ring.p = point(c.first + k);
            ring.slope = slope(c.first + k, c.run_first, c.run_count);
            ring.section = c.flags >> 8u;
            ring.index = k;
            ring.samples = c.count;
            add_ring(&w, ring, tube, error, write, base);
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
                ring.p = hermite(a, b, t);
                ring.slope = sa + (sb - sa) * t;
                add_ring(&w, ring, tube, error, write, base);
            }
            ka = kb;
        }
        ring.p = point(c.first + last);
        ring.slope = slope(c.first + last, c.run_first, c.run_count);
        add_ring(&w, ring, tube, error, write, base);
    }
    if (tube) {
        let opens = vec2((c.flags & 1u) != 0u || mode(i - 1u) != 1u,
            (c.flags & 2u) != 0u || mode(i + 1u) != 1u);
        if (opens.x) { cap(&w, write, base, w.first_point, w.first_start, w.first_sides, -1.0); }
        if (opens.y) { cap(&w, write, base, w.last_point, w.last_start, w.last_sides, 1.0); }
    }
    return w.counts;
}

