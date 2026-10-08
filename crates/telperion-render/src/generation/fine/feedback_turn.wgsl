// Rotations built from vectors on the GPU (fn-211 task 9; host decision
// 2026-10-07, CONTRACT-A.md 2.3: trig only where an angle is a stated
// quantity): `fixed::feedback::vector`'s `dot`, `cross`, `carry` and
// `turned`, word for word, over `feedback.wgsl`.

alias V3 = array<vec2<u32>, 3>;

// The exact dot product, Q64.
fn dot_wide(a0: V3, b0: V3) -> vec4<u32> {
    var a = a0;
    var b = b0;
    var s = vec4<u32>(0u);
    for (var k = 0u; k < 3u; k++) {
        s = add4(s, pair(a[k], b[k]));
    }
    return s;
}

// a · b, one floor.
fn dot3(a: V3, b: V3) -> vec2<u32> {
    return shift4(dot_wide(a, b), 32u, false, true).xy;
}

// The exact cross product, Q64.
fn cross_wide(a0: V3, b0: V3) -> array<vec4<u32>, 3> {
    var a = a0;
    var b = b0;
    return array<vec4<u32>, 3>(
        sub4(pair(a[1], b[2]), pair(a[2], b[1])),
        sub4(pair(a[2], b[0]), pair(a[0], b[2])),
        sub4(pair(a[0], b[1]), pair(a[1], b[0])),
    );
}

// a × b, each component one floor.
fn cross3(a: V3, b: V3) -> V3 {
    var c = cross_wide(a, b);
    var out: V3;
    for (var k = 0u; k < 3u; k++) {
        out[k] = shift4(c[k], 32u, false, true).xy;
    }
    return out;
}

// A rotation: its unit pivot, cosine and sine; `some` false for none.
struct Turn {
    pivot: V3,
    cos: vec2<u32>,
    sin: vec2<u32>,
    some: bool,
}

// `vector::carry`: the least rotation from unit `from` to unit `to`, its
// cosine and sine made unit by one Newton step.
fn carry(from_v: V3, to_v: V3) -> Turn {
    var t: Turn;
    let axis = cross3(from_v, to_v);
    let p = unit3(axis);
    if !p.some {
        return t;
    }
    var ax = axis;
    let c = dot3(from_v, to_v);
    let s = length3(ax[0], ax[1], ax[2]);
    let norm = shift4(add4(pair(c, c), pair(s, s)), 32u, false, true).xy;
    let step = sar64(sub64(w(0u, 3u), norm), 1u);
    return Turn(p.c, mulq(c, step), mulq(s, step), true);
}

// `vector::turned`: Rodrigues, one floor a term, the terms added exactly.
fn turned_by(v0: V3, t: Turn) -> V3 {
    var v = v0;
    var pivot = t.pivot;
    var across = cross_wide(t.pivot, v0);
    let along = dot_wide(t.pivot, v0);
    var out: V3;
    for (var k = 0u; k < 3u; k++) {
        let a = mulq(v[k], t.cos);
        let b = xnarrow64(xfloor(xmul(across[k], ext4(t.sin)), 64u)).v.xy;
        let fall = pair(pivot[k], sub64(one32(), t.cos));
        let c = xnarrow64(xfloor(xmul(along, fall), 96u)).v.xy;
        out[k] = add64(add64(a, b), c);
    }
    return out;
}
