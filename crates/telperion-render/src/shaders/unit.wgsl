// An organ's inflorescence as one unit (fn-253): its florets, then its
// rachis, stand in one run of the placements on both executors, so a
// floret finds the rachis it hangs from as the first instance after it
// that carries the rachis's mark. The unit is thinned and enlarged as one,
// about the rachis's seat, so no floret is kept without its rachis. Read
// by selection and by the organ's draw, which bind `placements`.

// The most instances a unit's run holds: `bloom.rs`'s 64 florets, then
// its rachis.
const UNIT_RUN: u32 = 65u;

// The rachis instance `instance` belongs to: itself on a rachis, the first
// marked after it on a floret, or itself where its run holds none.
fn unit_rachis(instance: u32) -> u32 {
    let n = arrayLength(&placements) / LEAF_WORDS;
    let end = min(instance + UNIT_RUN, n);
    for (var j = instance; j < end; j++) {
        if (placements[j * LEAF_WORDS + 2u] & LEAF_WITHERED) != 0u {
            return j;
        }
    }
    return instance;
}

// The words of instance `i`.
fn unit_words(i: u32) -> vec3<u32> {
    let base = i * LEAF_WORDS;
    return vec3<u32>(placements[base], placements[base + 1u], placements[base + 2u]);
}

// Where the unit whose rachis is instance `r` is seated, against the box.
fn unit_seat(r: u32, box_min: vec3<f32>, box_extent: vec3<f32>) -> vec3<f32> {
    return leaf_position(unit_words(r), box_min, box_extent);
}

// A rachis's axis (`bloom.rs`'s `Axis::laid`): its eight segments.
const AXIS_SEGMENTS: u32 = 8u;

// `Vec3::perpendicular`.
fn stalk_perpendicular(v: vec3<f32>) -> vec3<f32> {
    let a = abs(v);
    var reference = vec3<f32>(0.0, 0.0, 1.0);
    if a.x <= a.y && a.x <= a.z {
        reference = vec3<f32>(1.0, 0.0, 0.0);
    } else if a.y <= a.z {
        reference = vec3<f32>(0.0, 1.0, 0.0);
    }
    return normalize(cross(reference, v));
}

// A unit vector, or `fallback` where `v` has no length (`bloom.rs`'s `or`).
fn stalk_or(v: vec3<f32>, fallback: vec3<f32>) -> vec3<f32> {
    if dot(v, v) > 1e-24 {
        return normalize(v);
    }
    return fallback;
}

// Where the axis turns: toward hanging at a droop of zero or more, toward
// standing below it.
fn rachis_toward(droop: f32) -> vec3<f32> {
    if droop >= 0.0 {
        return vec3<f32>(0.0, -1.0, 0.0);
    }
    return vec3<f32>(0.0, 1.0, 0.0);
}

// The axis's tangent along segment `k`: the shoot's heading turned toward
// hanging or standing by the droop, most of it near the base.
fn rachis_tangent(heading: vec3<f32>, droop: f32, k: u32) -> vec3<f32> {
    let toward = rachis_toward(droop);
    let rest = 1.0 - (f32(k) + 0.5) / f32(AXIS_SEGMENTS);
    let turned = abs(droop) * (1.0 - rest * rest * rest);
    return stalk_or(heading + (toward - heading) * turned, toward);
}

// The axis's point at `ring` of a rachis `long` metres seated at `seat`.
fn rachis_point(seat: vec3<f32>, heading: vec3<f32>, long: f32, droop: f32, ring: u32) -> vec3<f32> {
    var at = seat;
    for (var k = 0u; k < ring; k++) {
        at += rachis_tangent(heading, droop, k) * (long / f32(AXIS_SEGMENTS));
    }
    return at;
}

// How far a ribbon whose chords join every `stride`th ring stands off the
// axis at its farthest: the error of its level, in metres.
fn rachis_error(seat: vec3<f32>, heading: vec3<f32>, long: f32, droop: f32, stride: u32) -> f32 {
    var points: array<vec3<f32>, 9>;
    points[0] = seat;
    for (var k = 0u; k < AXIS_SEGMENTS; k++) {
        points[k + 1u] = points[k] + rachis_tangent(heading, droop, k) * (long / f32(AXIS_SEGMENTS));
    }
    var error = 0.0;
    for (var j = 1u; j < AXIS_SEGMENTS; j++) {
        let a = points[(j / stride) * stride];
        let d = points[min((j / stride + 1u) * stride, AXIS_SEGMENTS)] - a;
        let t = clamp(dot(points[j] - a, d) / max(dot(d, d), 1e-30), 0.0, 1.0);
        error = max(error, length(points[j] - (a + d * t)));
    }
    return error;
}
