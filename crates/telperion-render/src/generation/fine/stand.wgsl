// Where a relay stands on the own axis it carries on (`geometry::stand`,
// fn-199 rule 2'): at its PA's `relay_at` along the last growth unit of
// the line of own axes carried on, read as one axis, blended back towards
// the unit before by the last unit's extent, down to a whole unit or the
// line's base. Part of the laid shader.

// A unit of the line: its own axis and its phytomers `first..end`; none
// past the line's base, whose axis it names.
struct UnitRef {
    axis: u32,
    first: u32,
    end: u32,
    ok: bool,
}

// A PA's divergence in turns, as a float.
fn divergence_f(pa: u32) -> f32 {
    let d = divergence_q(pa);
    return (f32(d.y) * 4294967296.0 + f32(d.x)) * (1.0 / 1152921504606846976.0);
}

// Whether own axis `i` carries its parent on: a relay or a continuation.
fn carries_on(i: u32) -> bool {
    let k = kind_of(i);
    return i > 1u && (k == RELAYED || k == CONTINUED);
}

// How far own axis `i` has left the line of the axis it carries on: a
// relay's blend, a continuation's none.
fn blend_of(i: u32) -> f32 {
    if kind_of(i) == RELAYED {
        return axf(i, BLEND);
    }
    return 0.0;
}

// The phase of own axis `i`'s phyllotaxis, in turns, unwrapped
// (`geometry::phase`): each axis it carries on adds its divergence times
// its rank, as far as it is still that axis's continuation. Taken from
// the line's top down.
fn axis_phase(i: u32) -> f32 {
    var depth = 0u;
    var top = i;
    // cap: the region's axes; a line no longer than them.
    let cap = arrayLength(&axes) / AXW;
    while carries_on(top) && depth < cap {
        top = ax(top, PARENT);
        depth += 1u;
    }
    var phase = 0.0;
    for (var level = depth; level > 0u; level--) {
        // The axis `level - 1` links down from the top, and its parent.
        var c = i;
        for (var k = 1u; k < level; k++) {
            c = ax(c, PARENT);
        }
        let p = ax(c, PARENT);
        phase = (1.0 - blend_of(c)) * (phase + rank_f(axis_rank(p)) * divergence_f(ax(p, PA)));
    }
    return phase;
}

// The unit `j` back from the newest of the line ending in own axis `p`.
fn unit_back(p: u32, j: u32) -> UnitRef {
    var a = p;
    var count = j;
    var found = UnitRef(p, 0u, 0u, false);
    // cap: the region's axes; a line no longer than them.
    var cap = arrayLength(&axes) / AXW;
    loop {
        if cap == 0u {
            break;
        }
        cap -= 1u;
        var end = s(a, S_COUNT);
        var hit = false;
        // cap: end counts down from the axis's phytomers.
        while end > 0u {
            let c = ph(at_k(a, end - 1u), 2u);
            var first = end - 1u;
            // cap: first counts down from end.
            while first > 0u && ph(at_k(a, first - 1u), 2u) == c {
                first -= 1u;
            }
            if count == 0u {
                found = UnitRef(a, first, end, true);
                hit = true;
                break;
            }
            count -= 1u;
            end = first;
        }
        if hit {
            break;
        }
        if !carries_on(a) {
            found = UnitRef(a, 0u, 0u, false);
            break;
        }
        a = ax(a, PARENT);
    }
    return found;
}

// A unit's extent: its nodes' presences times its lived share, summed,
// at most one.
fn extent(u: UnitRef) -> f32 {
    var e = 0.0;
    for (var k = u.first; k < u.end; k++) {
        let p = at_k(u.axis, k);
        e += qf(ph(p, QSCALE), ph(p, QSCALE + 1u));
    }
    return min(e, 1.0);
}

// How far along node `k`'s span of the unit a relay at `share` of it
// stands, by its phytomers' scaled lengths (`span_in`).
fn span_in(u: UnitRef, share: f32, k: u32) -> f32 {
    var total = 0.0;
    for (var j = u.first; j < u.end; j++) {
        total += out[lo(u.axis, j) + 19u];
    }
    let scale = out[lo(u.axis, k) + 19u];
    if total > 0.0 && scale > 0.0 {
        var before = 0.0;
        for (var j = min(u.first, k); j < k; j++) {
            before += out[lo(u.axis, j) + 19u];
        }
        return clamp((share * total - before) / scale, 0.0, 1.0);
    } else if total <= 0.0 {
        let fr = f32(u.first) + share * f32(u.end - u.first);
        return clamp(fr - f32(k), 0.0, 1.0);
    }
    return 1.0;
}

// The node of the unit a relay at `share` of it stands in, by its
// phytomers' scaled lengths (`zone_node`).
fn zone_node(u: UnitRef, share: f32) -> u32 {
    var total = 0.0;
    for (var j = u.first; j < u.end; j++) {
        total += out[lo(u.axis, j) + 19u];
    }
    if total <= 0.0 {
        let k = u32(ceil(share * f32(u.end - u.first)));
        return u.first + clamp(k, 1u, u.end - u.first) - 1u;
    }
    var before = 0.0;
    for (var j = u.first; j < u.end; j++) {
        let q = out[lo(u.axis, j) + 19u];
        if q > 0.0 && before + q >= share * total {
            return j;
        }
        before += q;
    }
    return u.end - 1u;
}

// The point `share` along the unit in node `k`'s span and its frame:
// base and azimuth offset, along, side (`point_in`).
fn point_in(u: UnitRef, share: f32, k: u32, o: u32) -> array<vec4<f32>, 3> {
    let p = u.axis;
    let n = s(p, S_COUNT);
    let t = span_in(u, share, k);
    let at = lo(p, k);
    var start = s3(p, S_FRAME);
    if k > 0u {
        start = o3(lo(p, k - 1u) + o);
    }
    var nh = o3(at + o + 3u);
    var ns = o3(at + o + 6u);
    var nr = axis_rank(p);
    if k + 1u < n {
        let nx = lo(p, k + 1u);
        nh = o3(nx + o + 3u);
        ns = o3(nx + o + 6u);
        nr = rank_of(nx);
    }
    let h = o3(at + o + 3u);
    let sd = o3(at + o + 6u);
    let r = rank_f(rank_of(at)) + t * rank_f(sub64(nr, rank_of(at)));
    let base = start + (o3(at + o) - start) * t;
    let along = unit_or(h * (1.0 - t) + nh * t, h);
    let side = square(unit_or(sd * (1.0 - t) + ns * t, sd), along);
    let w = axis_phase(p) + r * divergence_f(ax(p, PA));
    return array<vec4<f32>, 3>(vec4<f32>(base, w), vec4<f32>(along, 0.0), vec4<f32>(side, 0.0));
}

// The `j`th unit back's point; the newest on `p` stands in the drawn
// node `node` (D18), the others in theirs by length.
fn unit_point(u: UnitRef, j: u32, p: u32, node: u32, o: u32) -> array<vec4<f32>, 3> {
    let share = pa_of(ax(u.axis, PA)).relay_at;
    var k = zone_node(u, share);
    if j == 0u && u.axis == p {
        k = clamp(node, u.first, u.end - 1u);
    }
    return point_in(u, share, k, o);
}

// `side` made square to `along` (`stand::square`).
fn square(side: vec3<f32>, along: vec3<f32>) -> vec3<f32> {
    return unit_or(side - along * dot(side, along), side);
}

// `t` of the way from `a` to `b`, the frame turned as far.
fn point_toward(a: array<vec4<f32>, 3>, b: array<vec4<f32>, 3>, t: f32) -> array<vec4<f32>, 3> {
    let base = a[0].xyz + (b[0].xyz - a[0].xyz) * t;
    let w = a[0].w + t * (b[0].w - a[0].w);
    let along = unit_or(a[1].xyz * (1.0 - t) + b[1].xyz * t, b[1].xyz);
    let side = square(unit_or(a[2].xyz * (1.0 - t) + b[2].xyz * t, b[2].xyz), along);
    return array<vec4<f32>, 3>(vec4<f32>(base, w), vec4<f32>(along, 0.0), vec4<f32>(side, 0.0));
}

// Where a relay of own axis `p` stands, its node `node` on `p`'s last
// unit: base and azimuth in turns, along, side.
fn stand(p: u32, node: u32, o: u32) -> array<vec4<f32>, 3> {
    var n = 0u;
    var whole = false;
    // cap: a line holds no more units than the region's phytomers.
    let cap = arrayLength(&phyts) / PW;
    loop {
        if n >= cap {
            break;
        }
        let u = unit_back(p, n);
        if !u.ok {
            break;
        }
        n += 1u;
        if extent(u) >= 1.0 {
            whole = true;
            break;
        }
    }
    var pt: array<vec4<f32>, 3>;
    if whole {
        n -= 1u;
        pt = unit_point(unit_back(p, n), n, p, node, o);
    } else {
        let top = unit_back(p, n);
        let a = top.axis;
        pt = array<vec4<f32>, 3>(vec4<f32>(s3(a, S_FRAME), axis_phase(a)), vec4<f32>(s3(a, S_FRAME + 3u), 0.0), vec4<f32>(s3(a, S_FRAME + 6u), 0.0));
    }
    var j = n;
    // cap: j counts down from n.
    while j > 0u {
        j -= 1u;
        let u = unit_back(p, j);
        pt = point_toward(pt, unit_point(u, j, p, node, o), extent(u));
    }
    return pt;
}
