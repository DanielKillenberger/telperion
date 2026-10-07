// Laying a fine root's axes on the GPU (fn-209 F1, task 4): the walk of
// `telperion_space::geometry::lay` (`Layer::step`), the frames of
// `geometry::frame` and `turned`, and sag's turn (`sag.rs`), in f32.
// Positions are the root's own: metres from its bearer's node, so a
// walk's rounding is the size of the root's reach, never the tree's; the
// ground is at `-ground` in them. Part of `geometry.wgsl`'s shader.

const UP = vec3<f32>(0.0, 0.0, 1.0);
const DOWN = vec3<f32>(0.0, 0.0, -1.0);
const PI: f32 = 3.14159265358979;
const TAU: f32 = 6.28318530717959;
const CONTACT: f32 = 4.0;
const DOWNWARD: f32 = 0.05;
const HANG: f32 = 0.15;
const LANDING: f32 = 6.0;
const UPRIGHT: f32 = 0.2;
const LIFTS: u32 = 8u;

// `Vec3::unit`: none (a zero vector) for a vector of no length.
fn unit_or(v: vec3<f32>, or: vec3<f32>) -> vec3<f32> {
    let l = length(v);
    if l > 1e-12 {
        return v / l;
    }
    return or;
}

fn has_unit(v: vec3<f32>) -> bool {
    return length(v) > 1e-12;
}

// Rodrigues.
fn rotated(v: vec3<f32>, pivot: vec3<f32>, angle: f32) -> vec3<f32> {
    let t = turned_by(angle);
    return v * (1.0 - t.z) + cross(pivot, v) * t.x + pivot * (dot(pivot, v) * t.z);
}

// The sine, cosine and versine (1 - cos) of `a`: a short turn's by their
// series, since the GPU's own sine is good to only about 5e-7 whatever
// its argument, a large share of a short turn, and a walk of many short
// turns gathers it into a drift of its heading (fn-209 task 6).
fn turned_by(a: f32) -> vec3<f32> {
    if abs(a) < 0.1 {
        let t2 = a * a;
        let s = a * (1.0 - t2 * (1.0 / 6.0 - t2 / 120.0));
        let vers = t2 * (0.5 - t2 * (1.0 / 24.0 - t2 / 720.0));
        return vec3<f32>(s, 1.0 - vers, vers);
    }
    let c = cos(a);
    return vec3<f32>(sin(a), c, 1.0 - c);
}

// `v` by the least rotation taking unit `from` to unit `to`.
fn carried_by(v: vec3<f32>, was: vec3<f32>, now: vec3<f32>) -> vec3<f32> {
    let axis = cross(was, now);
    if !has_unit(axis) {
        return v;
    }
    return rotated(v, normalize(axis), atan2(length(axis), dot(was, now)));
}

// `v` by the rotation taking frame (h, s) to (h2, s2); unchanged where
// either has no side square to its heading.
fn framed(v: vec3<f32>, h: vec3<f32>, s: vec3<f32>, h2: vec3<f32>, s2: vec3<f32>) -> vec3<f32> {
    let a = s - h * dot(s, h);
    let b = s2 - h2 * dot(s2, h2);
    if !has_unit(a) || !has_unit(b) {
        return v;
    }
    let side = normalize(a);
    let side2 = normalize(b);
    return h2 * dot(v, h) + side2 * dot(v, side) + cross(h2, side2) * dot(v, cross(h, side));
}

// The heading and side of a bud at `azimuth` about a parent going `along`
// with `side`, turned `epitony` of the way to its upper side, inserted at
// `angle`, its plane turned by `plane` (`geometry::turned`).
fn turned(along: vec3<f32>, side: vec3<f32>, azimuth: f32, angle: f32, plane: f32, epitony: f32) -> array<vec3<f32>, 2> {
    var toward = side * cos(azimuth) + cross(along, side) * sin(azimuth);
    let level = UP - along * dot(UP, along);
    if has_unit(level) {
        let upper = normalize(level);
        let facing = dot(toward, upper);
        let turn = atan2(dot(along, cross(toward, upper)), facing);
        let sided = min(length(level) / UPRIGHT, 1.0);
        toward = rotated(toward, along, epitony * sided * turn * (1.0 + facing) / 2.0);
    }
    let heading = along * cos(angle) + toward * sin(angle);
    let s0 = toward * cos(angle) - along * sin(angle);
    let s1 = s0 * cos(plane) + cross(heading, s0) * sin(plane);
    return array<vec3<f32>, 2>(heading, s1);
}

// `toward_elevation`.
fn toward_elevation(direction: vec3<f32>, side: vec3<f32>, elevation: f32, share_in: f32) -> vec3<f32> {
    if share_in <= 0.0 {
        return direction;
    }
    let lean = length(direction.xy);
    let cone = sin(DOWNWARD);
    var share = share_in;
    if direction.z < 0.0 && lean < cone {
        share = share * lean / cone;
    }
    var out = vec3<f32>(0.0);
    if has_unit(vec3<f32>(direction.xy, 0.0)) {
        out = normalize(vec3<f32>(direction.xy, 0.0));
    } else if has_unit(vec3<f32>(side.xy, 0.0)) {
        out = normalize(vec3<f32>(side.xy, 0.0));
    } else {
        return direction;
    }
    // Its elevation, from its lean: asin of a near-one loses its digits.
    // Turned by the step towards the elevation, as a rotation in its
    // vertical plane rather than the sine and cosine of the elevation
    // reached: the elevation's pull holds each step's error, so the
    // GPU's coarse sine of a whole angle would settle as a lean of its
    // error over the share (fn-209 task 6).
    let now = atan2(direction.z, lean);
    let turn = (elevation - now) * share;
    let at = out * lean + UP * direction.z;
    let up = UP * lean - out * direction.z;
    let t = turned_by(turn);
    return at * t.y + up * t.x;
}

// `geometry::support`: how far wood at height `z` stands free of the
// ground.
fn support(z: f32, radius: f32, len: f32) -> f32 {
    let zone = CONTACT * radius + LANDING * len;
    if z >= zone {
        return 1.0;
    }
    return sqrt(max(z, 0.0) / zone);
}

// `sag::held`.
fn held(height: f32, chord: vec3<f32>) -> f32 {
    let low = height + chord.z;
    if low >= 0.0 {
        return 1.0;
    }
    if height <= 0.0 {
        return 0.0;
    }
    let free = height / (height - low);
    return free * free;
}

fn torque(lever: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(-lever.y, lever.x, 0.0);
}

// `sag::turn`, the moment's curvature integrated over the phytomer.
fn sag_turn(lever: vec3<f32>, cap: f32, sag: f32, radius: f32, len: f32) -> f32 {
    let reach = length(lever);
    if reach <= 0.0 || cap <= 0.0 {
        return 0.0;
    }
    let r2 = radius * radius;
    let c = sag / (r2 * r2);
    let start = acos(clamp(-lever.z / reach, -1.0, 1.0));
    let low = asin(min(cap / reach, 1.0));
    let high = PI - low;
    var fall = start;
    var left = len;
    if fall > high {
        let s = log(tan(fall / 2.0) / tan(high / 2.0)) / (c * reach);
        if s >= left {
            return start - 2.0 * atan(tan(fall / 2.0) * exp(-c * reach * left));
        }
        fall = high;
        left = left - s;
    }
    if fall > low {
        let rate = c * cap;
        let s = (fall - low) / rate;
        if s >= left {
            return start - (fall - rate * left);
        }
        fall = low;
        left = left - s;
    }
    return start - 2.0 * atan(tan(fall / 2.0) * exp(-c * reach * left));
}

// A walk along an axis (`Layer`), its pulls carried on from the axis it
// relays held in the scratch at `lifts`.
struct Walk {
    running: vec3<f32>,
    across: vec3<f32>,
    tip: vec3<f32>,
    run: f32,
    pull: f32,
    curve: vec3<f32>,
}

// A PA's continuous values (`telperion_space::PaGeometry`), from the
// geometry table.
struct Pa {
    internode: f32,
    insertion: f32,
    divergence: f32,
    relay_at: f32,
    epitony: f32,
    straightening: f32,
    erection: f32,
    tropism: f32,
    elevation: f32,
    wander: f32,
    plane: f32,
    pipe: f32,
    exponent: f32,
    ripening: f32,
    dominance: f32,
    roll: f32,
    sag: f32,
    secondary: f32,
    bend_length: f32,
    retained: f32,
    internode_spread: f32,
}

// A phytomer as the walk lays it: in, its scale, radius and key, and its
// first lay's heading and side (none: unlaid); out, its tip and frame.
struct Step {
    scale: f32,
    radius: f32,
    key: vec2<u32>,
    was_heading: vec3<f32>,
    was_side: vec3<f32>,
    tip: vec3<f32>,
    heading: vec3<f32>,
    side: vec3<f32>,
}

// 1 - e^-x, without the cancellation of f32 near none: a short step's
// share of a pull is a small difference of two near-ones, whose error a
// long axis turns into a drift of its heading (fn-209 task 6).
fn gone(x: f32) -> f32 {
    if abs(x) < 0.05 {
        return x * (1.0 - x * (0.5 - x * (1.0 / 6.0 - x * (1.0 / 24.0 - x / 120.0))));
    }
    return 1.0 - exp(-x);
}

// A key's draw as a real in [0, 1).
fn unit_of(key: vec2<u32>) -> f32 {
    return real_of(draw(key));
}

// A 53-bit draw as a real in [0, 1).
fn real_of(d: vec2<u32>) -> f32 {
    return (f32(d.y) * 4294967296.0 + f32(d.x)) * (1.0 / 9007199254740992.0);
}

// `Layer::step`: lays the next phytomer from `walk`; its own pull `bend`
// over `total`, the pulls it carries on at scratch `lifts` (`n` of
// them), the lever `moment` and `chord` where it sags (`sags`), and the
// ground at `-ground`.
fn step(walk: ptr<function, Walk>, p: ptr<function, Step>, f: Pa, own: vec2<f32>, lifts: u32, n: u32, sags: bool, moment: vec3<f32>, chord: vec3<f32>, ground: f32) {
    var running = (*walk).running;
    var across = (*walk).across;
    var curve = (*walk).curve;
    let tip = (*walk).tip;
    var run = (*walk).run;
    var len = f.internode * (*p).scale;
    // The node's internode draw (fn-222), mean one.
    if f.internode_spread > 0.0 {
        len = len * (1.0 + f.internode_spread * (2.0 * real_of(internode_draw((*p).key)) - 1.0));
    }
    let share = gone(f.tropism * len);
    let bent = toward_elevation(running, across, f.elevation, share);
    across = carried_by(across, running, bent);
    curve = carried_by(curve, running, bent);
    running = bent;
    var kicked = false;
    var draw0 = 0.0;
    var pivot = vec3<f32>(0.0);
    var u0 = 0.0;
    if f.wander > 0.0 {
        let key = child((*p).key, w(1u, 0u));
        u0 = unit_of(child(key, w(0u, 0u)));
        draw0 = f.wander * (2.0 * u0 - 1.0);
        let azimuth = TAU * unit_of(child(key, w(1u, 0u)));
        pivot = across * cos(azimuth) + cross(running, across) * sin(azimuth);
        kicked = true;
    }
    if f.bend_length > 0.0 {
        let keep = exp(-len / f.bend_length);
        curve = curve * keep;
        if kicked {
            curve = curve + pivot * (draw0 * sqrt(gone(2.0 * len / f.bend_length)));
        }
        curve = curve - running * dot(curve, running);
        let angle = length(curve) * len;
        if has_unit(curve) {
            let axis = normalize(curve);
            running = rotated(running, axis, angle);
            across = rotated(across, axis, angle);
        }
    } else if kicked {
        let angle = f.wander * len * (2.0 * u0 - 1.0);
        running = rotated(running, pivot, angle);
        across = rotated(across, pivot, angle);
        curve = pivot * draw0;
    } else {
        curve = vec3<f32>(0.0);
    }
    if sags && f.sag > 0.0 && (*p).radius > 0.0 {
        let now0 = framed(moment, (*p).was_heading, (*p).was_side, running, across);
        let carried = held(tip.z + ground, framed(chord, (*p).was_heading, (*p).was_side, running, across));
        let now = now0 * carried;
        let turning = torque(now);
        let m = length(turning);
        if m > 0.0 {
            let pv = turning / m;
            let square = running - pv * dot(running, pv);
            var room = atan2(dot(cross(square, DOWN), pv), dot(square, DOWN));
            if room < -PI / 2.0 {
                room += TAU;
            }
            if room > PI {
                room = 2.0 * (1.5 * PI - room);
            }
            let cap = length(torque(moment)) * carried;
            var angle = min(sag_turn(now, cap, f.sag, (*p).radius, len), max(room - HANG, 0.0));
            angle = angle * support(tip.z + ground, (*p).radius, len);
            running = rotated(running, pv, angle);
            across = rotated(across, pv, angle);
            curve = rotated(curve, pv, angle);
        }
    }
    // Its own pull and those it carries on, each fading over its reach.
    var lift = 0.0;
    var up = 0.0;
    for (var k = 0u; k <= n; k++) {
        var l = vec4<f32>(own.x, (*walk).pull, own.y, 0.0);
        if k > 0u {
            l = lift_at(lifts, k - 1u);
        }
        var along = 1.0;
        if l.z > 0.0 {
            along = (l.w + run + (*p).scale / 2.0) / l.z;
        }
        let here = l.x * max(1.0 - along, 0.0);
        lift += here;
        up += here * l.y;
    }
    run += (*p).scale;
    let blend = running * (1.0 - lift) + UP * up;
    var direction = running;
    if lift > 0.0 && has_unit(blend) {
        direction = normalize(blend);
    }
    if direction.z < 0.0 {
        let ease = support(tip.z + ground, (*p).radius, len);
        if ease < 1.0 {
            let eased = vec3<f32>(direction.xy, direction.z * ease);
            if has_unit(eased) {
                direction = normalize(eased);
            }
        }
    }
    var next = tip + direction * len;
    if next.z + ground < 0.0 {
        next.z = -ground;
        if has_unit(next - tip) {
            direction = normalize(next - tip);
        }
    }
    (*p).tip = next;
    (*p).heading = direction;
    (*p).side = unit_or(across - direction * dot(across, direction), across);
    (*walk).running = running;
    (*walk).across = across;
    (*walk).tip = next;
    (*walk).run = run;
    (*walk).curve = curve;
}
