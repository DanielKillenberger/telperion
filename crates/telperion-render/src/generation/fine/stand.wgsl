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

// Where a relay joins its parent (`geometry::attach`, fn-244 host
// decision 4), Q32 as the CPU resolves it: one rule for its base, girth,
// sag and the join the drawing inserts. Positions are in the parent's
// nodes, a 64-bit Q32 (`w(lo, hi)`, the node in the high word).

// `stand::span_in`, Q32, on own axis `p`'s phytomers `first..end`.
fn span_q(p: u32, first: u32, end: u32, share: vec2<u32>, k: u32) -> vec2<u32> {
    var total = vec2<u32>(0u);
    for (var j = first; j < end; j++) {
        total = add64(total, q32_f(out[lo(p, j) + 19u]));
    }
    let scale = q32_f(out[lo(p, k) + 19u]);
    if slt64(vec2<u32>(0u), total) && slt64(vec2<u32>(0u), scale) {
        var before = vec2<u32>(0u);
        for (var j = min(first, k); j < k; j++) {
            before = add64(before, q32_f(out[lo(p, j) + 19u]));
        }
        let t = over(sub64(mulq(share, total), before), scale);
        return smin64(smax64(t, vec2<u32>(0u)), one32());
    } else if slt64(vec2<u32>(0u), total) {
        return one32();
    }
    let f = add64(w(0u, first), mulq(share, w(0u, end - first)));
    return smin64(smax64(sub64(f, w(0u, k)), vec2<u32>(0u)), one32());
}

// `stand::zone_node`, Q32.
fn zone_q(p: u32, first: u32, end: u32, share: vec2<u32>) -> u32 {
    var total = vec2<u32>(0u);
    for (var j = first; j < end; j++) {
        total = add64(total, q32_f(out[lo(p, j) + 19u]));
    }
    if !slt64(vec2<u32>(0u), total) {
        // ceil(share n), the share Q32 and n whole: exact.
        let k = add64(mulq(share, w(0u, end - first)), w(0xffffffffu, 0u)).y;
        return first + clamp(k, 1u, end - first) - 1u;
    }
    let want = mulq(share, total);
    var before = vec2<u32>(0u);
    for (var j = first; j < end; j++) {
        let q = q32_f(out[lo(p, j) + 19u]);
        if slt64(vec2<u32>(0u), q) && !slt64(add64(before, q), want) {
            return j;
        }
        before = add64(before, q);
    }
    return end - 1u;
}

// A unit's extent, Q32, at most one (`stand::position`).
fn extent_q(p: u32, first: u32, end: u32) -> vec2<u32> {
    var e = vec2<u32>(0u);
    for (var k = first; k < end; k++) {
        let q = at_k(p, k);
        e = add64(e, vec2<u32>(ph(q, QSCALE), ph(q, QSCALE + 1u)));
    }
    return smin64(e, one32());
}

// The first phytomer of the unit of own axis `p` ending at `end`.
fn unit_first(p: u32, end: u32) -> u32 {
    let c = ph(at_k(p, end - 1u), 2u);
    var first = end - 1u;
    // cap: first counts down from end.
    while first > 0u && ph(at_k(p, first - 1u), 2u) == c {
        first -= 1u;
    }
    return first;
}

// The position in `p`'s unit `first..end`, its newest when `node` is
// given (D18), the others by length.
fn unit_x(p: u32, first: u32, end: u32, newest: bool, node: u32) -> vec2<u32> {
    let share = gq(ax(p, PA), G_RELAY_AT);
    var k = zone_q(p, first, end, share);
    if newest {
        k = clamp(node, first, end - 1u);
    }
    return add64(w(0u, k), span_q(p, first, end, share, k));
}

// `stand::position`: how far along own axis `p` a relay at its node
// `node` stands, in `p`'s nodes, Q32: `p`'s own units only, newest first
// down to a whole one or `p`'s start.
fn position_q(p: u32, node: u32) -> vec2<u32> {
    // The oldest unit read: the first whole one back, or none.
    var end = s(p, S_COUNT);
    var stop = 0u;
    var base = vec2<u32>(0u);
    // cap: end counts down from the axis's phytomers.
    while end > 0u {
        let first = unit_first(p, end);
        if !slt64(extent_q(p, first, end), one32()) {
            stop = end;
            base = unit_x(p, first, end, end == s(p, S_COUNT), node);
            break;
        }
        end = first;
    }
    // Then each younger unit, oldest first, blended in by its extent.
    var x = base;
    var at = stop;
    if stop == 0u {
        at = 0u;
    }
    // cap: at counts up to the axis's phytomers.
    while at < s(p, S_COUNT) {
        var last = at + 1u;
        let c = ph(at_k(p, at), 2u);
        while last < s(p, S_COUNT) && ph(at_k(p, last), 2u) == c {
            last += 1u;
        }
        let to = unit_x(p, at, last, last == s(p, S_COUNT), node);
        x = add64(x, mulq(sub64(to, x), extent_q(p, at, last)));
        at = last;
    }
    return x;
}

// `geometry::attach`: own relay `i`'s position on its parent, blended
// towards the parent's tip by its blend, Q32.
fn attach_q(i: u32) -> vec2<u32> {
    let p = ax(i, PARENT);
    let tip = w(0u, s(p, S_COUNT));
    let b = q32_f(axf(i, BLEND));
    if !slt64(vec2<u32>(0u), b) {
        return tip;
    }
    return add64(tip, mulq(sub64(position_q(p, ax(i, NODE)), tip), b));
}

// `stand::split`: a position as its node and the share of its span, Q32;
// the last node's whole span at the tip and beyond.
fn split_x(x: vec2<u32>, last: u32) -> vec3<u32> {
    var k = 0u;
    if !negative(x) {
        k = min(x.y, last);
    }
    let t = smin64(smax64(sub64(x, w(0u, k)), vec2<u32>(0u)), one32());
    return vec3<u32>(k, t.x, t.y);
}
