// fn-256 task 2: where a relay stands on the line it carries on
// (`geometry::stand`), where it joins its parent (`geometry::attach`),
// the phase a line's phyllotaxis runs on from (`geometry::phase`) and a
// relay's node (`presence::relay_node`), in the CPU's f32 operations.
// Part of the layout's shader.

// The longest line of axes carried on, and the most units a stand blends.
const LINE: u32 = 128u;
const STAND: u32 = 16u;
const UPRIGHT: f32 = 0x1.99999ap-3f;

// A point along an axis, its frame, and the azimuth a bud there turns by.
struct Point {
    base: vec3<f32>,
    along: vec3<f32>,
    side: vec3<f32>,
    rank: f32,
}

// The line traced from an axis (`trace_line`): its axes, then each one's
// phase.
var<private> line_ax: array<u32, LINE>;
var<private> line_ph: array<f32, LINE>;
var<private> line_n: u32;
var<private> held_pt: array<Point, STAND>;
var<private> held_x: array<vec2<f32>, STAND>;

// A rank's turn of a PA's phyllotaxis, an amount (`by_rank`).
fn by_rank(pa: u32, rank: f32) -> f32 {
    return m(fm(pa, F_DIVERGENCE), rank);
}

fn carried(a: u32) -> bool {
    let k = kind_of(a);
    return k == CONTINUED || k == RELAYED;
}

// How far axis `a` has left the line it carries on (`carried_on`).
fn blend_of(a: u32) -> f32 {
    return select(0.0, axf(a, A_BLEND), kind_of(a) == RELAYED);
}

// The line from `p` down the axes it carries on, and each one's phase,
// from the line's first axis up (`phase_by`).
fn trace_line(p: u32) {
    var a = p;
    var n = 0u;
    // cap: `LINE` axes.
    loop {
        if n == LINE {
            status |= FAULT_LINE;
            break;
        }
        line_ax[n] = a;
        n += 1u;
        if !carried(a) {
            break;
        }
        a = ax(a, A_PARENT);
    }
    line_n = n;
    var phase = axf(line_ax[n - 1u], A_PHASE);
    line_ph[n - 1u] = phase;
    for (var k = n - 1u; k > 0u; k--) {
        let parent = line_ax[k];
        let b = blend_of(line_ax[k - 1u]);
        phase = m(1.0 - b, phase + by_rank(ax(parent, A_PA), axf(parent, A_RANK)));
        line_ph[k - 1u] = phase;
    }
}

// The first of the run of phytomers of one cycle that ends at `end`.
fn run_start(a: u32, end: u32) -> u32 {
    var first = end - 1u;
    let c = ph(prow(a, first), PH_CYCLE);
    // cap: the axis's phytomers.
    while first > 0u && ph(prow(a, first - 1u), PH_CYCLE) == c {
        first -= 1u;
    }
    return first;
}

// A column summed over phytomers `first..end` of `a`, as Rust sums.
fn summed(a: u32, first: u32, end: u32, c: u32) -> f32 {
    var s = neg0();
    for (var j = first; j < end; j++) {
        s = s + phf(prow(a, j), c);
    }
    return s;
}

fn zone_node(a: u32, first: u32, end: u32, share: f32) -> u32 {
    let total = summed(a, first, end, PH_SCALE);
    if total <= 0.0 {
        let k = u32(ceil(share * f32(end - first)));
        return first + clamp(k, 1u, end - first) - 1u;
    }
    let want = share * total;
    var before = 0.0;
    for (var j = first; j < end; j++) {
        let scale = phf(prow(a, j), PH_SCALE);
        if scale > 0.0 && before + scale >= want {
            return j;
        }
        before = before + scale;
    }
    return end - 1u;
}

fn span_in(a: u32, first: u32, end: u32, share: f32, k: u32) -> f32 {
    let total = summed(a, first, end, PH_SCALE);
    let scale = phf(prow(a, k), PH_SCALE);
    if total > 0.0 && scale > 0.0 {
        let before = summed(a, min(first, k), k, PH_SCALE);
        return clamp(qdiv(m(share, total) - before, scale), 0.0, 1.0);
    }
    if total > 0.0 {
        return 1.0;
    }
    return clamp(m(share, f32(end - first)) - f32(k - first), 0.0, 1.0);
}

fn square(side: vec3<f32>, along: vec3<f32>) -> vec3<f32> {
    return qunit_or(side - mv(along, qdot(side, along)), side);
}

// `t` of the way from `x` to `y`, made a unit: `or` where none.
fn blended(x: vec3<f32>, y: vec3<f32>, t: f32, or: vec3<f32>) -> vec3<f32> {
    return qunit_or(mv(x, 1.0 - t) + mv(y, t), or);
}

fn point_in(a: u32, first: u32, end: u32, share: f32, k: u32) -> Point {
    let t = span_in(a, first, end, share, k);
    let r = prow(a, k);
    var start = ax3(a, A_BASE);
    if k > 0u {
        start = ph3(prow(a, k - 1u), PH_TIP);
    }
    let heading = ph3(r, PH_HEADING);
    let side = ph3(r, PH_SIDE);
    var next_heading = heading;
    var next_side = side;
    var next_rank = axf(a, A_RANK);
    if k + 1u < nphyt(a) {
        let q = prow(a, k + 1u);
        next_heading = ph3(q, PH_HEADING);
        next_side = ph3(q, PH_SIDE);
        next_rank = phf(q, PH_RANK);
    }
    let along = blended(heading, next_heading, t, heading);
    let rank = phf(r, PH_RANK);
    let base = start + mv(ph3(r, PH_TIP) - start, t);
    let s = square(blended(side, next_side, t, side), along);
    return Point(base, along, s, rank + m(t, next_rank - rank));
}

fn toward(p: Point, to: Point, t: f32) -> Point {
    let along = blended(p.along, to.along, t, to.along);
    let base = p.base + mv(to.base - p.base, t);
    let side = square(blended(p.side, to.side, t, to.side), along);
    return Point(base, along, side, p.rank + m(t, to.rank - p.rank));
}

// Where a relay of `p`, at its node `node`, stands: the line's last unit
// at its PA's `relay_at`, blended back by each unit's extent to a whole
// unit or the line's base. `trace_line(p)` first.
fn stand(p: u32, node: u32) -> Point {
    var count = 0u;
    var li = 0u;
    var whole = false;
    var base = Point(vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), 0.0);
    // cap: the line's `LINE` axes.
    loop {
        let a = line_ax[li];
        let pa = ax(a, A_PA);
        let share = fm(pa, F_RELAY_AT);
        let n = nphyt(a);
        var end = n;
        // cap: the axis's runs of phytomers, at most `STAND`.
        while end > 0u {
            let first = run_start(a, end);
            var k = clamp(node, first, end - 1u);
            if a != p || end != n {
                k = zone_node(a, first, end, share);
            }
            var at = point_in(a, first, end, share, k);
            at.rank = line_ph[li] + by_rank(pa, at.rank);
            let e = summed(a, first, end, PH_GROWN);
            if count == STAND {
                status |= FAULT_LINE;
                return at;
            }
            held_pt[count] = at;
            held_x[count] = vec2<f32>(0.0, min(e, 1.0));
            count += 1u;
            if e >= 1.0 {
                whole = true;
                break;
            }
            end = first;
        }
        if whole {
            count -= 1u;
            base = held_pt[count];
            break;
        }
        if li + 1u >= line_n {
            let r = line_ph[li] + by_rank(pa, 0.0);
            base = Point(ax3(a, A_BASE), ax3(a, A_HEADING), ax3(a, A_SIDE), r);
            break;
        }
        li += 1u;
    }
    var point = base;
    // cap: `STAND` held points.
    while count > 0u {
        count -= 1u;
        point = toward(point, held_pt[count], held_x[count].y);
    }
    return point;
}

// How far along `p` a relay at its node `node` stands, in `p`'s scaled
// length (`stand::position`).
fn position(p: u32, node: u32) -> f32 {
    let share = fm(ax(p, A_PA), F_RELAY_AT);
    let n = nphyt(p);
    var count = 0u;
    var end = n;
    var whole = false;
    // cap: the axis's runs of phytomers, at most `STAND`.
    while end > 0u && count < STAND {
        let first = run_start(p, end);
        var k = clamp(node, first, end - 1u);
        if end != n {
            k = zone_node(p, first, end, share);
        }
        let t = span_in(p, first, end, share, k);
        let x = summed(p, 0u, k, PH_SCALE) + m(t, phf(prow(p, k), PH_SCALE));
        let e = summed(p, first, end, PH_GROWN);
        held_x[count] = vec2<f32>(x, min(e, 1.0));
        count += 1u;
        if e >= 1.0 {
            whole = true;
            break;
        }
        end = first;
    }
    if end > 0u && !whole {
        status |= FAULT_LINE;
    }
    var x = 0.0;
    if whole {
        count -= 1u;
        x = held_x[count].x;
    }
    // cap: `STAND` held places.
    while count > 0u {
        count -= 1u;
        x = x + m(held_x[count].x - x, held_x[count].y);
    }
    return x;
}

// Where relay `i`, of blend `b`, joins its parent `p` (`attach`): its
// node and the share of its span.
fn attach(i: u32, p: u32, b: f32) -> vec2<f32> {
    let n = nphyt(p);
    let tip = summed(p, 0u, n, PH_SCALE);
    let at = position(p, ax(i, A_NODE));
    let length = tip + m(at - tip, b);
    var before = 0.0;
    for (var k = 0u; k < n; k++) {
        let scale = phf(prow(p, k), PH_SCALE);
        if scale > 0.0 && before + scale >= length {
            return vec2<f32>(bitcast<f32>(k), clamp(qdiv(length - before, scale), 0.0, 1.0));
        }
        before = before + scale;
    }
    return vec2<f32>(bitcast<f32>(n), 0.0);
}

// The point at node and share `x` along `p` (`stand::at`).
fn stand_at(p: u32, x: vec2<f32>) -> vec3<f32> {
    let n = nphyt(p);
    if n == 0u {
        return ax3(p, A_BASE);
    }
    var k = bitcast<u32>(x.x);
    var t = clamp(x.y, 0.0, 1.0);
    if k > n - 1u {
        k = n - 1u;
        t = 1.0;
    }
    var start = ax3(p, A_BASE);
    if k > 0u {
        start = ph3(prow(p, k - 1u), PH_TIP);
    }
    return start + mv(ph3(prow(p, k), PH_TIP) - start, t);
}

// The node of `p`'s last unit a relay of it stands at (D18).
fn relay_node(p: u32) -> u32 {
    let n = nphyt(p);
    if n == 0u {
        return 0u;
    }
    let first = run_start(p, n);
    let share = fm(ax(p, A_PA), F_RELAY_NODE);
    let total = summed(p, first, n, PH_NODE);
    if total > 0.0 {
        let aim = share * total;
        var before = 0.0;
        for (var j = first; j < n; j++) {
            let q = phf(prow(p, j), PH_NODE);
            if q > 0.0 && before + q >= aim {
                return j;
            }
            before = before + q;
        }
        return n - 1u;
    }
    let k = first + u32(ceil(share * f32(n - first)));
    return clamp(k, first + 1u, n) - 1u;
}
