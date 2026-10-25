// fn-256 task 2: the layout's f32 vectors and angles on the device
// (`telperion_space::geometry::q`), each operation the CPU's in its order:
// no built-in dot, cross, length or normalize, whose order WGSL leaves
// free. An angle is an amount in turns. A product an add takes is
// written `m`, a quotient `qdiv`, a root `qsqrt`, and a sum or product
// the driver was seen to reorder or simplify `apart`: the sites where the
// driver may fuse, reorder or round otherwise than the CPU (LAYOUT.md).
// Each is the plain operation, or held to the CPU's where the kernels are
// built exact (`Arith`).

const UP = vec3<f32>(0.0, 0.0, 1.0);
// The f32 turn, pi, and log2 e, by their bits.
const QTAU: f32 = 0x1.921fb6p+2f;
const QPI: f32 = 0x1.921fb6p+1f;
const QPI_LO: f32 = -0x1.777a5cp-24f;
const QLOG2E: f32 = 0x1.715476p+0f;
// -0.0, the start of Rust's f32 sum, by its bits at run time.
fn neg0() -> f32 {
    return bitcast<f32>(0x80000000u);
}

fn m(a: f32, b: f32) -> f32 {
    return exact_kept(a * b);
}

fn apart(x: f32) -> f32 {
    return exact_kept(x);
}

fn qdiv(a: f32, b: f32) -> f32 {
    return exact_div(a, b);
}

fn qsqrt(x: f32) -> f32 {
    return exact_sqrt(x);
}

fn mv(v: vec3<f32>, s: f32) -> vec3<f32> {
    return vec3<f32>(m(v.x, s), m(v.y, s), m(v.z, s));
}

fn qdot(a: vec3<f32>, b: vec3<f32>) -> f32 {
    return m(a.x, b.x) + m(a.y, b.y) + m(a.z, b.z);
}

fn qcross(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(m(a.y, b.z) - m(a.z, b.y), m(a.z, b.x) - m(a.x, b.z), m(a.x, b.y) - m(a.y, b.x));
}

fn qlength(v: vec3<f32>) -> f32 {
    return qsqrt(qdot(v, v));
}

// `unit`: the unit vector in xyz and 1 in w; w 0 for the zero vector.
fn qunit(v: vec3<f32>) -> vec4<f32> {
    let l = qlength(v);
    if l > 0.0 {
        return vec4<f32>(mv(v, qdiv(1.0, l)), 1.0);
    }
    return vec4<f32>(v, 0.0);
}

fn qunit_or(v: vec3<f32>, or: vec3<f32>) -> vec3<f32> {
    let u = qunit(v);
    return select(or, u.xyz, u.w > 0.0);
}

// A rotation: its unit pivot, cosine and sine (`Turn`).
struct Turn {
    pivot: vec3<f32>,
    cos: f32,
    sin: f32,
}

// Rodrigues: `v cos + (p × v) sin + p (p · v)(1 - cos)`.
fn qturned_by(v: vec3<f32>, t: Turn) -> vec3<f32> {
    let across = qcross(t.pivot, v);
    let fall = m(qdot(t.pivot, v), 1.0 - t.cos);
    return mv(v, t.cos) + mv(across, t.sin) + mv(t.pivot, fall);
}

// The least rotation from unit `was` to unit `to`; `ok` none where they
// are parallel or opposed exactly.
fn qcarry(was: vec3<f32>, to: vec3<f32>, ok: ptr<function, bool>) -> Turn {
    let axis = qcross(was, to);
    let p = qunit(axis);
    *ok = p.w > 0.0;
    let c = qdot(was, to);
    let s = qlength(axis);
    let norm = qsqrt(m(c, c) + m(s, s));
    return Turn(p.xyz, qdiv(c, norm), qdiv(s, norm));
}

fn qwrap(turns: f32) -> f32 {
    return turns - floor(turns + 0.5);
}

fn qbetween(was: f32, to: f32) -> f32 {
    return qwrap(to - was);
}

// The cosine and sine of an amount in turns (`q::sincos`), the device's
// own: geometry floors into no integer, so it takes plain arithmetic
// within the ruled tolerance (DESIGN.md 9.1, 11).
fn qsincos(turns: f32) -> vec2<f32> {
    let x = qwrap(turns) * QTAU;
    return vec2<f32>(cos(x), sin(x));
}

fn qatan2(y: f32, x: f32) -> f32 {
    return qdiv(atan2(y, x), QTAU);
}

fn qturns(radians: f32) -> f32 {
    return qdiv(radians, QTAU);
}

fn qturn(pivot: vec3<f32>, angle: f32) -> Turn {
    let cs = qsincos(angle);
    return Turn(pivot, cs.x, cs.y);
}

fn qrotated(v: vec3<f32>, pivot: vec3<f32>, angle: f32) -> vec3<f32> {
    return qturned_by(v, qturn(pivot, angle));
}
