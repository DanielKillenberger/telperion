// The instance rest (fn-301 R2, `foliage/rest.rs`), in the same order: an
// instance whose eight corners `v`, offsets from its attachment, would
// reach more than `t` below it, each its own, is turned about the
// attachment by the rotation that carries the up direction nearest world
// up with `v·d >= -t` onto up, or, where none clears, the one that lifts
// the corners' box's lowest point highest.

// The 10-bit rotation's worst angle and the half float's worst relative
// rounding (`rest::ANGLE`, `rest::HALF`).
const REST_ANGLE: f32 = 0.0029325513196480938;
const REST_HALF: f32 = 0.00048828125;

// What packing may move a vertex `reach` from its attachment, in a box
// whose position code steps `step` in y (`rest::margin`).
fn rest_margin(step: f32, reach: f32) -> f32 {
    return 0.5 * step + (REST_HALF + REST_ANGLE) * reach;
}

// Corner `k` of a wedge (`rest::Wedge`): its side from bit 0, its end from
// bit 1, its face from bit 2; `a` the sides and ends, `b` the top, the
// base's bottom and the tip's.
fn rest_corner(k: u32, a: vec4<f32>, b: vec4<f32>) -> vec3<f32> {
    let x = select(a.x, a.y, (k & 1u) != 0u);
    let tip = (k & 2u) != 0u;
    let bottom = select(b.y, b.z, tip);
    return vec3<f32>(x, select(a.z, a.w, tip), select(bottom, b.x, (k & 4u) != 0u));
}

// The direction nearest up with `a·d = -t`; `w` 1 where it exists.
fn rest_one(a: vec3<f32>, t: f32) -> vec4<f32> {
    let l = length(a);
    let c = -t / l;
    if !(l > 0.0 && abs(c) <= 1.0) {
        return vec4<f32>(0.0);
    }
    let n = a / l;
    var w = vec3<f32>(0.0, 1.0, 0.0) - n * n.y;
    if dot(w, w) > 1e-24 {
        w = normalize(w);
    } else {
        w = vec3<f32>(1.0, 0.0, 0.0);
    }
    return vec4<f32>(n * c + w * sqrt(1.0 - c * c), 1.0);
}

// One of the two directions with `a·d = -ta` and `b·d = -tb` (`hand`
// picks which).
fn rest_two(a: vec3<f32>, b: vec3<f32>, ta: f32, tb: f32, hand: f32) -> vec4<f32> {
    let la = length(a);
    let lb = length(b);
    if !(la > 0.0 && lb > 0.0) {
        return vec4<f32>(0.0);
    }
    let na = a / la;
    let nb = b / lb;
    let ca = -ta / la;
    let cb = -tb / lb;
    let g = dot(na, nb);
    let det = 1.0 - g * g;
    if det <= 1e-12 {
        return vec4<f32>(0.0);
    }
    let alpha = (ca - g * cb) / det;
    let beta = (cb - g * ca) / det;
    let rest = 1.0 - (alpha * ca + beta * cb);
    if rest < 0.0 {
        return vec4<f32>(0.0);
    }
    return vec4<f32>(na * alpha + nb * beta + cross(na, nb) * (sqrt(rest / det) * hand), 1.0);
}

fn rest_fits(v: array<vec3<f32>, 8>, t: array<f32, 8>, d: vec4<f32>, slack: f32) -> bool {
    var c = v;
    var below = t;
    if d.w == 0.0 || any(abs(d.xyz) > vec3<f32>(3.4e38)) {
        return false;
    }
    for (var k = 0u; k < 8u; k++) {
        if dot(c[k], d.xyz) < -below[k] - slack {
            return false;
        }
    }
    return true;
}

// The up direction nearest world up for which every corner `k` stands at
// least `-t[k]` along it (`rest::upright`); `w` 1 where one does.
fn rest_upright(v: array<vec3<f32>, 8>, t: array<f32, 8>) -> vec4<f32> {
    var c = v;
    var below = t;
    var reach = 0.0;
    for (var k = 0u; k < 8u; k++) {
        reach = max(reach, max(length(c[k]), abs(below[k])));
    }
    // A float32 solve's equalities hold to its rounding.
    let slack = 1e-5 * reach;
    var found = vec4<f32>(0.0);
    var offer = vec4<f32>(0.0, 1.0, 0.0, 1.0);
    if rest_fits(c, t, offer, slack) {
        found = offer;
    }
    for (var i = 0u; i < 8u; i++) {
        offer = rest_one(c[i], below[i]);
        if rest_fits(c, t, offer, slack) && (found.w == 0.0 || offer.y > found.y) {
            found = offer;
        }
    }
    for (var i = 0u; i < 8u; i++) {
        for (var j = i + 1u; j < 8u; j++) {
            for (var s = 0u; s < 2u; s++) {
                offer = rest_two(c[i], c[j], below[i], below[j], select(1.0, -1.0, s == 1u));
                if rest_fits(c, t, offer, slack) && (found.w == 0.0 || offer.y > found.y) {
                    found = offer;
                }
            }
        }
    }
    return found;
}

// Where no direction clears (`rest::best`): toward the corners' box's
// nearest point outside it, away from its nearest face inside it.
fn rest_best(v: array<vec3<f32>, 8>) -> vec3<f32> {
    var c = v;
    var lo = c[0];
    var hi = c[0];
    for (var k = 1u; k < 8u; k++) {
        lo = min(lo, c[k]);
        hi = max(hi, c[k]);
    }
    let near = clamp(vec3<f32>(0.0), lo, hi);
    if dot(near, near) > 0.0 {
        return normalize(near);
    }
    let gaps = array<f32, 6>(-lo.x, -lo.y, -lo.z, hi.x, hi.y, hi.z);
    var least = 0u;
    for (var k = 1u; k < 6u; k++) {
        if gaps[k] < gaps[least] {
            least = k;
        }
    }
    var away = vec3<f32>(0.0);
    away[least % 3u] = select(1.0, -1.0, least >= 3u);
    return away;
}

// The direction the corners `v` rest by, below `t`: up where they clear.
fn rest_direction(v: array<vec3<f32>, 8>, t: array<f32, 8>) -> vec3<f32> {
    let d = rest_upright(v, t);
    if d.w != 0.0 {
        return d.xyz;
    }
    return rest_best(v);
}

// `u` turned by the rotation carrying `d` onto up (`rest::turned`).
fn rest_turned(u: vec3<f32>, d: vec3<f32>) -> vec3<f32> {
    let c = d.y;
    if c <= -1.0 + 1e-12 {
        return vec3<f32>(u.x, -u.y, -u.z);
    }
    let a = vec3<f32>(-d.z, 0.0, d.x);
    return u * c + cross(a, u) + a * (dot(a, u) / (1.0 + c));
}

// A leaf's frame at `scale` from `point`, rested on its wedge `a`, `b`
// (`rest::leaf`): its columns turned where its corners would cross the
// ground, else as given; its base corners, its stalk, may reach down to its
// base depth, the rest clear the ground by the margin.
fn rest_leaf(frame: mat3x3<f32>, scale: f32, point: vec3<f32>, a: vec4<f32>, b: vec4<f32>, step: f32) -> mat3x3<f32> {
    var v: array<vec3<f32>, 8>;
    var reach = 0.0;
    for (var k = 0u; k < 8u; k++) {
        let c = rest_corner(k, a, b);
        v[k] = (frame[0] * c.x + frame[1] * c.y + frame[2] * c.z) * scale;
        reach = max(reach, length(v[k]));
    }
    let clear = max(point.y, 0.0) - rest_margin(step, reach);
    let stalk = max(clear, b.w * scale);
    var t: array<f32, 8>;
    var fits = true;
    for (var k = 0u; k < 8u; k++) {
        t[k] = select(stalk, clear, (k & 2u) != 0u);
        fits = fits && v[k].y >= -t[k];
    }
    if fits {
        return frame;
    }
    let d = rest_direction(v, t);
    return mat3x3<f32>(rest_turned(frame[0], d), rest_turned(frame[1], d), rest_turned(frame[2], d));
}
