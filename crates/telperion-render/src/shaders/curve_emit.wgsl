// One ring's vertices and indices from its record (fn-208, host decision
// 18): a thread a ring, so a close view pays a ring's work, not the longest
// cluster's walk. Concatenated after curve.wgsl and curve_walk.wgsl.

fn put_float(at: u32, value: f32) {
    if (at < cfg.vertex_slice) {
        vertices0[at] = value;
    } else {
        vertices1[at - cfg.vertex_slice] = value;
    }
}

fn put_index(at: u32, value: u32) {
    if (at < cfg.index_slice) {
        indices0[at] = value;
    } else {
        indices1[at - cfg.index_slice] = value;
    }
}

fn put(at: u32, position: vec3<f32>, normal: vec3<f32>, along: f32, angle: f32, radius: f32) {
    let b = at * VERTEX_FLOATS;
    let v = array<f32, 9>(position.x, position.y, position.z, normal.x, normal.y, normal.z,
        along, angle, radius);
    for (var k = 0u; k < 9u; k++) { put_float(b + k, v[k]); }
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

fn triangle(at: u32, a: u32, b: u32, c: u32) {
    put_index(at, a);
    put_index(at + 1u, b);
    put_index(at + 2u, c);
}

// The strip between two rings of any side counts, walked by angle: one
// triangle a side of either ring. Returns the indices written.
fn zipper(at: u32, a: u32, na: u32, b: u32, nb: u32) -> u32 {
    var i = 0u;
    var j = 0u;
    var n = 0u;
    loop {
        if (i >= na && j >= nb) { break; }
        if (j >= nb || (i < na && (i + 1u) * nb <= (j + 1u) * na)) {
            triangle(at + n, a + i % na, a + (i + 1u) % na, b + j % nb);
            i += 1u;
        } else {
            triangle(at + n, a + i % na, b + (j + 1u) % nb, b + j % nb);
            j += 1u;
        }
        n += 3u;
    }
    return n;
}

fn cap(at: u32, centre: u32, p: Point, start: u32, n: u32, away: f32) {
    put(centre, p.centre, tangent(p) * away, p.along, 0.0, p.radius);
    for (var k = 0u; k < n; k++) {
        triangle(at + 3u * k, centre, start + k, start + (k + 1u) % n);
    }
}

fn emit_ring(r: u32) {
    let flags = record(r, 13u);
    if (flags == 0u) { return; }
    var f: array<f32, 12>;
    for (var k = 0u; k < 12u; k++) { f[k] = bitcast<f32>(record(r, k)); }
    var p: Point;
    p.centre = vec3(f[0], f[1], f[2]);
    p.radius = f[3];
    p.along = f[4];
    p.normal = vec3(f[5], f[6], f[7]);
    p.binormal = vec3(f[8], f[9], f[10]);
    let slope = f[11];
    let sides = record(r, 12u);
    let section = record(r, 14u);
    let index = record(r, 15u) & 0xFFFFu;
    let samples = record(r, 15u) >> 16u;
    var v = record(r, 16u);
    var t = record(r, 17u);
    let q = record(r, 18u);
    let previous = vec3(record(r, 19u), record(r, 20u), record(r, 21u));
    let polygon = v;
    if ((flags & POLYGON) != 0u) {
        for (var k = 0u; k < sides; k++) {
            let angle = TAU * f32(k) / f32(sides);
            var at: array<vec3<f32>, 2>;
            if (section != 0xFFFFFFFFu) {
                at = cell_vertex(section, index, samples, angle);
            } else {
                at = tube_vertex(p, angle, slope);
            }
            put(v + k, at[0], at[1], p.along, angle, p.radius);
        }
        v += sides;
        if ((flags & STRIP) != 0u) { t += zipper(t, previous.x, previous.y, polygon, sides); }
    }
    if ((flags & RIBBON) != 0u) {
        var view = cfg.forward.xyz;
        if (cfg.orthographic == 0u && dot(p.centre - cfg.eye.xyz, p.centre - cfg.eye.xyz) > 0.0) {
            view = normalize(p.centre - cfg.eye.xyz);
        }
        let tg = tangent(p);
        let across = cross(tg, view);
        var side = p.binormal;
        if (dot(across, across) > 1e-12) { side = normalize(across); }
        var facing = -view - tg * dot(-view, tg);
        if (dot(facing, facing) > 1e-12) { facing = normalize(facing); } else { facing = p.normal; }
        let normals = array<vec3<f32>, 3>(-side, facing, side);
        for (var k = 0u; k < 3u; k++) {
            let n = normals[k];
            let angle = atan2(dot(n, p.binormal), dot(n, p.normal));
            put(v + k, p.centre + side * (p.radius * (f32(k) - 1.0)), n, p.along, angle, p.radius);
        }
        if ((flags & QUAD) != 0u) {
            let a = previous.z;
            let c = v;
            let order = array<u32, 12>(a, a + 1u, c, c, a + 1u, c + 1u,
                a + 1u, a + 2u, c + 1u, c + 1u, a + 2u, c + 2u);
            for (var k = 0u; k < 12u; k++) { put_index(q + k, order[k]); }
        }
        v += 3u;
    }
    if ((flags & CAP_START) != 0u) {
        cap(t, v, p, polygon, sides, -1.0);
        v += 1u;
        t += 3u * sides;
    }
    if ((flags & CAP_END) != 0u) {
        cap(t, v, p, polygon, sides, 1.0);
    }
}
