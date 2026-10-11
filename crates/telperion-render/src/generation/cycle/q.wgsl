// fn-256 task 2: the layout's f32 vectors and angles on the device
// (`telperion_space::geometry::q`), each operation the CPU's in its order:
// no built-in dot, cross, length or normalize, whose order WGSL leaves
// free. An angle is an amount in turns. A product an add takes is
// written `m`, a quotient `qdiv`, a root `qsqrt`, and a sum or product
// the driver was seen to reorder or simplify `apart`: the sites where the
// driver may fuse, reorder or round otherwise than the CPU (LAYOUT.md).
// Each is the plain operation here; LAYOUT.md's experiment holds them to
// the CPU's.

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
    return a * b;
}

fn apart(x: f32) -> f32 {
    return x;
}

fn qdiv(a: f32, b: f32) -> f32 {
    return a / b;
}

fn qsqrt(x: f32) -> f32 {
    return sqrt(x);
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

// The cosine and sine of an amount in turns (`q::sincos`).
fn qsincos(turns: f32) -> vec2<f32> {
    let sc = sincosf(qwrap(turns) * QTAU);
    return vec2<f32>(sc.y, sc.x);
}

fn qatan2(y: f32, x: f32) -> f32 {
    return qdiv(atan2f(y, x), QTAU);
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

// libm's `atanf` (FreeBSD's), in its f32 operations.
fn atanf(x_in: f32) -> f32 {
    var x = x_in;
    let bits = bitcast<u32>(x);
    let negative = (bits >> 31u) != 0u;
    let ix = bits & 0x7fffffffu;
    if ix >= 0x4c800000u {
        let h = bitcast<f32>(0x3fc90fdau);
        return select(h, -h, negative);
    }
    var id = -1;
    if ix < 0x3ee00000u {
        if ix < 0x39800000u {
            return x;
        }
    } else {
        x = abs(x);
        if ix < 0x3f980000u {
            if ix < 0x3f300000u {
                x = qdiv(m(2.0, x) - 1.0, 2.0 + x);
                id = 0;
            } else {
                x = qdiv(x - 1.0, x + 1.0);
                id = 1;
            }
        } else if ix < 0x401c0000u {
            x = qdiv(x - 1.5, 1.0 + m(1.5, x));
            id = 2;
        } else {
            x = qdiv(-1.0, x);
            id = 3;
        }
    }
    let z = x * x;
    let w = z * z;
    let a0 = bitcast<f32>(0x3eaaaaa9u);
    let a1 = bitcast<f32>(0xbe4cca98u);
    let a2 = bitcast<f32>(0x3e11f50du);
    let a3 = bitcast<f32>(0xbdda1247u);
    let a4 = bitcast<f32>(0x3d7cac25u);
    let s1 = m(z, a0 + m(w, a2 + m(w, a4)));
    let s2 = m(w, a1 + m(w, a3));
    if id < 0 {
        return x - m(x, apart(s1 + s2));
    }
    let hi = array<f32, 4>(bitcast<f32>(0x3eed6338u), bitcast<f32>(0x3f490fdau), bitcast<f32>(0x3f7b985eu), bitcast<f32>(0x3fc90fdau));
    let lo = array<f32, 4>(bitcast<f32>(0x31ac3769u), bitcast<f32>(0x33222168u), bitcast<f32>(0x33140fb4u), bitcast<f32>(0x33a22168u));
    let r = hi[id] - apart(apart(m(x, apart(s1 + s2)) - lo[id]) - x);
    return select(r, -r, negative);
}

// libm's `atan2f` over finite values.
fn atan2f(y: f32, x: f32) -> f32 {
    let bx = bitcast<u32>(x);
    let by = bitcast<u32>(y);
    if bx == 0x3f800000u {
        return atanf(y);
    }
    let mm = ((by >> 31u) & 1u) | ((bx >> 30u) & 2u);
    let ix = bx & 0x7fffffffu;
    let iy = by & 0x7fffffffu;
    if iy == 0u {
        return select(select(-QPI, QPI, mm == 2u), y, mm < 2u);
    }
    if ix == 0u || ix + (26u << 23u) < iy {
        return select(QPI * 0.5, -QPI * 0.5, (mm & 1u) != 0u);
    }
    var z = 0.0;
    if (mm & 2u) == 0u || iy + (26u << 23u) >= ix {
        z = atanf(abs(qdiv(y, x)));
    }
    switch mm {
        case 0u: { return z; }
        case 1u: { return -z; }
        case 2u: { return QPI - apart(z - QPI_LO); }
        default: { return apart(z - QPI_LO) - QPI; }
    }
}
