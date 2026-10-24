// Florets on the device (fn-229): `pipeline/foliage/bloom.rs` in f32, draw
// for draw. A bloom grows in past its draw by its age's share; each of its
// florets is a pure function of (bloom, index, seed), keyed by the bearing
// node's phytomer key (`NodeIdentity::birth_order`) under the lineage
// registry's bloom keys. The stations count each node's florets by it, and
// the placement places them by it, so the two never disagree on a floret.
// It reads `contract.wgsl`'s `child` and `draw` and `lineage.wgsl`'s keys.

// A bloom's rows: florets, taper, scatter and the time since the window
// opened (a share of the year); the window's length and stagger, and the
// droop; the rachis, the pedicel, and the stalk's spread as its sine and
// cosine.
struct Flower {
    a: vec4<f32>,
    b: vec4<f32>,
    c: vec4<f32>,
}

// `bloom.rs`'s constants.
const BLOOM_WINDOW: f32 = 0.1;
const BLOOM_GROW: f32 = 0.25;
const BLOOM_SEGMENTS: u32 = 8u;
const BLOOM_LEAST: f32 = 1e-3;
const BLOOM_TAU: f32 = 6.283185307179586;
// The golden angle as a share of a turn, (3 - sqrt 5) / 2.
const BLOOM_GOLDEN: f32 = 0.3819660112501051;

// A draw keyed by `key` under `part`, uniform on [0, 1).
fn bloom_unit(key: vec2<u32>, part: vec2<u32>) -> f32 {
    let d = draw(child(key, part));
    return f32(d.y) * 4.76837158203125e-7 + f32(d.x) * 1.1102230246251565e-16;
}

// How grown a bloom of `share` is (`presence`).
fn bloom_grown(share: f32, key: vec2<u32>) -> f32 {
    let u = bloom_unit(key, KEY_BLOOM);
    return clamp((share - u * (1.0 - BLOOM_WINDOW)) / BLOOM_WINDOW, 0.0, 1.0);
}

fn bloom_smooth(x: f32) -> f32 {
    let y = clamp(x, 0.0, 1.0);
    return y * y * (3.0 - 2.0 * y);
}

// The size the season gives a floret whose opening was drawn `u`.
fn bloom_season(f: Flower, u: f32) -> f32 {
    let life = f.b.x * (1.0 - 0.5 * f.b.y);
    if life <= 0.0 {
        return 0.0;
    }
    let offset = u * (f.b.x - life);
    let v = f.a.w - offset;
    let x = (v - floor(v)) / life;
    if x >= 1.0 {
        return 0.0;
    }
    return min(bloom_smooth(x / BLOOM_GROW), bloom_smooth((1.0 - x) / BLOOM_GROW));
}

fn floret_key(key: vec2<u32>, i: u32) -> vec2<u32> {
    return child(child(key, KEY_FLORET), vec2<u32>(i, 0u));
}

// Floret `i`'s size, before the least size drops it.
fn floret_size(f: Flower, grown: f32, key: vec2<u32>, i: u32) -> f32 {
    let fraction = min(f.a.x - f32(i), 1.0);
    let t = min((f32(i) + 0.5) / f.a.x, 1.0);
    let k = floret_key(key, i);
    let scatter = 1.0 + f.a.z * (2.0 * bloom_unit(k, KEY_FLORET_SIZE) - 1.0);
    let season = bloom_season(f, bloom_unit(k, KEY_FLORET_OPENING));
    return grown * fraction * (1.0 - f.a.y * t) * scatter * season;
}

fn floret_kept(size: f32) -> bool {
    return !(size < BLOOM_LEAST);
}

// The florets a bloom keeps.
fn floret_count(f: Flower, grown: f32, key: vec2<u32>) -> u32 {
    if grown <= 0.0 {
        return 0u;
    }
    var n = 0u;
    for (var i = 0u; i < u32(ceil(f.a.x)); i++) {
        if floret_kept(floret_size(f, grown, key, i)) {
            n += 1u;
        }
    }
    return n;
}

// `v` as a unit vector, or `fallback` where it has no length.
fn bloom_or(v: vec3<f32>, fallback: vec3<f32>) -> vec3<f32> {
    if dot(v, v) > 1e-24 {
        return normalize(v);
    }
    return fallback;
}

// `Vec3::perpendicular`.
fn bloom_perpendicular(v: vec3<f32>) -> vec3<f32> {
    let a = abs(v);
    var reference = vec3<f32>(0.0, 0.0, 1.0);
    if a.x <= a.y && a.x <= a.z {
        reference = vec3<f32>(1.0, 0.0, 0.0);
    } else if a.y <= a.z {
        reference = vec3<f32>(0.0, 1.0, 0.0);
    }
    return normalize(cross(reference, v));
}

// `Vec3::lerp` (the contract's `mix` is its hash).
fn bloom_lerp(a: vec3<f32>, b: vec3<f32>, t: f32) -> vec3<f32> {
    return a + (b - a) * t;
}

fn bloom_rotate(v: vec3<f32>, axis: vec3<f32>, angle: f32) -> vec3<f32> {
    let s = sin(angle);
    let c = cos(angle);
    return v * c + cross(axis, v) * s + axis * (dot(axis, v) * (1.0 - c));
}

// A bloom's axis, laid in segments as it turns (`Axis::laid`): its
// points from the seat and each segment's tangent.
struct BloomAxis {
    points: array<vec3<f32>, 9>,
    tangents: array<vec3<f32>, 8>,
}

// The axis of a bloom on the shoot tip at `node`, its parent at `held`:
// it continues the shoot (fn-253) and droops toward world down or up.
fn bloom_axis(f: Flower, node: vec3<f32>, held: vec3<f32>) -> BloomAxis {
    let heading = bloom_or(node - held, vec3<f32>(0.0, 1.0, 0.0));
    var toward = vec3<f32>(0.0, 1.0, 0.0);
    if f.b.z >= 0.0 {
        toward = vec3<f32>(0.0, -1.0, 0.0);
    }
    var axis: BloomAxis;
    axis.points[0] = node;
    let step = f.c.x / f32(BLOOM_SEGMENTS);
    for (var k = 0u; k < BLOOM_SEGMENTS; k++) {
        let along = (f32(k) + 0.5) / f32(BLOOM_SEGMENTS);
        let rest = 1.0 - along;
        let turned = abs(f.b.z) * (1.0 - rest * rest * rest);
        axis.tangents[k] = bloom_or(bloom_lerp(heading, toward, turned), toward);
        axis.points[k + 1u] = axis.points[k] + axis.tangents[k] * step;
    }
    return axis;
}

// One floret: where it stands, its stalk and its first petal's face.
struct Floret {
    point: vec3<f32>,
    stalk: vec3<f32>,
    face: vec3<f32>,
}

// Floret `i` of the bloom on the shoot tip at `node`, its parent at
// `held` (`inflorescence`, `Axis`): the axis continues the shoot (fn-253).
fn floret(f: Flower, node: vec3<f32>, held: vec3<f32>, key: vec2<u32>, i: u32) -> Floret {
    var axis = bloom_axis(f, node, held);
    // Its place along the axis and the axis's tangent there (`Axis::at`).
    let t = min((f32(i) + 0.5) / f.a.x, 1.0);
    let x = clamp(t, 0.0, 1.0) * f32(BLOOM_SEGMENTS);
    let k = min(u32(x), BLOOM_SEGMENTS - 1u);
    let at = bloom_lerp(axis.points[k], axis.points[k + 1u], x - f32(k));
    let mid = x - 0.5;
    let j = min(u32(max(mid, 0.0)), BLOOM_SEGMENTS - 2u);
    let share = clamp(mid - f32(j), 0.0, 1.0);
    let tangent = bloom_or(bloom_lerp(axis.tangents[j], axis.tangents[j + 1u], share), axis.tangents[k]);
    let normal = bloom_perpendicular(tangent);
    // The golden angle about the axis, from the bloom's own roll, reduced
    // to a turn before its sine is taken.
    let turns = bloom_unit(key, KEY_BLOOM_ROLL) + f32(i) * BLOOM_GOLDEN;
    let radial = bloom_rotate(normal, tangent, BLOOM_TAU * fract(turns));
    let stalk = bloom_or(tangent * f.c.w + radial * f.c.z, tangent);
    let face = bloom_or(cross(radial, stalk), bloom_perpendicular(stalk));
    return Floret(at + stalk * f.c.y, stalk, face);
}
