// The frond curve (`leaflet::rachis`), the one every module that draws a
// frond calls: the point `t` of the way along the rachis that leaves
// `point` on `heading`, `length` long, bent out of that line by its `arch`
// toward `lift`, and the rachis's direction there. A rachis whose bark,
// `girth` where it leaves the stem and tapering by its station, would pass
// below the ground meets it at its first contact and lies along it from
// there (fn-301 R2): its centreline at its radius, in the horizontal
// direction it met the ground on (its heading's where it met it straight
// down), its share mapped to the parabola's own arc length; a base at or
// under its radius rests from the base. One that never reaches the ground
// is the parabola. One that meets it straight down on a
// vertical heading has no way to lie: its point is not finite, which the
// placement refuses (`pack.wgsl`'s `packed`).
struct Rachis { at: vec3<f32>, run: vec3<f32> }

// fn-283 R1: the thinnest wood the engine draws (`space.rs`' `FINEST`).
const FINEST: f32 = 2e-4;

fn rachis(point: vec3<f32>, heading: vec3<f32>, lift: vec3<f32>, length: f32, arch: f32, girth: f32, t: f32) -> Rachis {
    let c = rachis_contact(point.y, heading.y * length, lift.y * arch * length, girth);
    if c < 0.0 || t < c {
        let at = point + heading * (length * t) + lift * (arch * length * t * t);
        let run = normalize(heading + lift * (2.0 * arch * t));
        return Rachis(at, run);
    }
    let met = heading + lift * (2.0 * arch * c);
    var along = vec3<f32>(met.x, 0.0, met.z);
    if dot(along, along) <= 1e-12 {
        along = vec3<f32>(heading.x, 0.0, heading.z);
    }
    if dot(along, along) <= 1e-12 {
        var bits = 0x7fc00000u;
        let none = vec3<f32>(bitcast<f32>(bits));
        return Rachis(none, none);
    }
    along = normalize(along);
    let at = point + heading * (length * c) + lift * (arch * length * c * c);
    let lies = at + along * rachis_along(length, arch, girth, c, t);
    return Rachis(vec3<f32>(lies.x, rachis_radius(girth, t), lies.z), along);
}

// The rachis's radius at share `t`: its `girth` tapering to its tip, and
// never below the finest wood.
fn rachis_radius(girth: f32, t: f32) -> f32 {
    return max(girth * (1.0 - t), FINEST);
}

// The least share at which a centreline standing `base` high, rising by
// `rise` a share and bending by `bend` a share squared, comes down to its
// radius; -1 where it never does.
fn rachis_contact(base: f32, rise: f32, bend: f32, girth: f32) -> f32 {
    if base <= rachis_radius(girth, 0.0) {
        return 0.0;
    }
    let thin = rachis_thin(girth);
    let tapered = rachis_root(bend, rise + girth, base - girth, 0.0, thin);
    if tapered >= 0.0 {
        return tapered;
    }
    return rachis_root(bend, rise, base - FINEST, thin, 1.0);
}

// The least root of `a t² + b t + c` within `[lo, hi]`; -1 where none.
fn rachis_root(a: f32, b: f32, c: f32, lo: f32, hi: f32) -> f32 {
    if a == 0.0 {
        if b == 0.0 {
            return -1.0;
        }
        return rachis_within(-c / b, lo, hi);
    }
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    var root = sqrt(discriminant);
    if b < 0.0 {
        root = -root;
    }
    let q = -0.5 * (b + root);
    var y = q / a;
    if q != 0.0 {
        y = c / q;
    }
    let x = q / a;
    let least = rachis_within(min(x, y), lo, hi);
    if least >= 0.0 {
        return least;
    }
    return rachis_within(max(x, y), lo, hi);
}

fn rachis_within(t: f32, lo: f32, hi: f32) -> f32 {
    return select(-1.0, t, t >= lo && t <= hi);
}

// The share where a rachis's taper meets the finest wood.
fn rachis_thin(girth: f32) -> f32 {
    return select(0.0, 1.0 - FINEST / girth, girth > FINEST);
}

// How far a resting rachis runs along the ground from share `c` to `t`:
// the parabola's arc length between them less what its centreline falls
// as its radius tapers, so it keeps the parabola's length.
fn rachis_along(length: f32, arch: f32, girth: f32, c: f32, t: f32) -> f32 {
    let thin = rachis_thin(girth);
    var tapered = 0.0;
    if c < thin {
        tapered = rachis_ground(length, arch, girth, min(t, thin)) - rachis_ground(length, arch, girth, c);
    }
    return tapered + rachis_ground(length, arch, 0.0, max(t, thin)) - rachis_ground(length, arch, 0.0, max(c, thin));
}

// The run along the ground to share `t` of a rachis whose centreline
// falls by `fall` a share, in closed form: the parabola's speed less the
// fall. Not finite where it falls faster than it runs, which the
// placement refuses as the CPU refuses it.
fn rachis_ground(length: f32, arch: f32, fall: f32, t: f32) -> f32 {
    let square = length * length - fall * fall;
    let b = abs(2.0 * arch * length);
    let speed = sqrt(square + b * b * t * t);
    if b == 0.0 {
        return speed * t;
    }
    var tail = 0.0;
    if square > 0.0 {
        tail = asinh(b * t / sqrt(square));
    } else if square < 0.0 {
        tail = acosh(max(b * t / sqrt(-square), 1.0));
    }
    return t / 2.0 * speed + square / (2.0 * b) * tail;
}
