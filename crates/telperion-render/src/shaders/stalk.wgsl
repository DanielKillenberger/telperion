// A flower's stalks (fn-253, `bloom/stalk.rs`): a stalk vertex's surface
// coordinate is its kind, then its ring times `STALK_SIDES` plus its side.
const STALK_SIDES: u32 = 3u;
const STALK_SEGMENTS: f32 = 8.0;
const STALK_TAU: f32 = 6.283185307179586;

// Where a stalk vertex stands and which way its surface faces.
struct Stalk {
    at: vec3<f32>,
    normal: vec3<f32>,
}

// A rachis vertex at `ring`, `side` of the rachis that stands at `seat`
// along `heading`, `radius` thick there and the stamen's half width at its
// end: the axis's points in the plane its heading turns in. A ribbon's two
// sides are its edges, turned to face the eye.
fn rachis_vertex(seat: vec3<f32>, heading: vec3<f32>, radius: f32, ring: u32, side: u32, ribbon: bool) -> Stalk {
    let toward = rachis_toward(u.stalk.y);
    let at = rachis_point(seat, heading, u.stalk.x, u.stalk.y, ring);
    let before = rachis_tangent(heading, u.stalk.y, max(ring, 1u) - 1u);
    let after = rachis_tangent(heading, u.stalk.y, min(ring, AXIS_SEGMENTS - 1u));
    let tangent = stalk_or(before + after, after);
    // The axis turns in one plane, so its binormal holds along it and no
    // ring twists against the next.
    let binormal = stalk_or(cross(heading, toward), stalk_perpendicular(tangent));
    let across = normalize(cross(tangent, binormal));
    let turn = STALK_TAU * f32(side) / f32(STALK_SIDES);
    var out = binormal * cos(turn) + across * sin(turn);
    var normal = out;
    if ribbon {
        let eye = u.eye.xyz - at;
        out = stalk_or(cross(tangent, eye), binormal) * select(-1.0, 1.0, side == 0u);
        normal = stalk_or(eye, out);
    }
    let along = f32(ring) / STALK_SEGMENTS;
    return Stalk(at + out * mix(radius, u.stalk.z, along), normal);
}

// A pedicel vertex at `ring`, `side` under the floret whose base and frame
// are `base` and `frame`: from its seat on the rachis, a pedicel's length
// back along the floret's axis, to the base, the stamen's width round.
fn pedicel_vertex(base: vec3<f32>, frame: mat3x3<f32>, ring: u32, side: u32) -> Stalk {
    let turn = STALK_TAU * f32(side) / f32(STALK_SIDES);
    let out = frame[0] * cos(turn) + frame[2] * sin(turn);
    let along = f32(ring) / STALK_SEGMENTS;
    let at = base - frame[1] * (u.stalk.w * (1.0 - along));
    return Stalk(at + out * u.stalk.z, out);
}
