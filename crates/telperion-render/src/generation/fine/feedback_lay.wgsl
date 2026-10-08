// The fine roots' rough layout on the GPU (fn-211 task 10; CONTRACT-A.md
// 2.4): one axis's units a thread, `grow::draw`'s `draw_bent` and
// `geometry::Layer::step` without a load, word for word, over the
// feedback contract (`feedback*.wgsl`). The CPU frames each round's axes
// and hands their walks over (`grow::stepped::LayJob`).

struct Cfg {
    n: u32,
    row: u32,
    pad0: u32,
    pad1: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
// Per PA: internode, tropism, elevation, wander, bend length, flags (bit
// 0 wanders, bit 1 bends), the cone's sine, the internode spread (fn-222).
@group(0) @binding(1) var<storage, read> pas: array<vec2<u32>>;
// Per job: (PA, trunk), bend, the walk's 14 words (running, across, tip,
// run, pull, curve), base scale, presence, rank, wood, (units' first word,
// count), (nodes' first, count), (lifts' first word, count), the axis's
// tropism over its PA's (fn-224).
@group(0) @binding(2) var<storage, read> jobs: array<vec2<u32>>;
// Per unit: share, survive, persist, size, phytomers; per lift: bend,
// pull, total, offset.
@group(0) @binding(3) var<storage, read> tables: array<vec2<u32>>;
// Per phytomer: key, node draw, radius.
@group(0) @binding(4) var<storage, read> nodes: array<vec2<u32>>;
// Per job: (status, 0), z, the walk's 14 words, rank, presence, wood.
@group(0) @binding(5) var<storage, read_write> out: array<vec2<u32>>;
// Per phytomer: scale, rank, tip, heading, side.
@group(0) @binding(6) var<storage, read_write> laid: array<vec2<u32>>;

const PA: u32 = 8u;
const JOB: u32 = 24u;
const OUT: u32 = 19u;
const PHYTOMER: u32 = 11u;
const REACH_HI: u32 = 0x1000u;
const GROUND: u32 = 4u;

struct Walk {
    running: V3,
    across: V3,
    tip: V3,
    run: vec2<u32>,
    pull: vec2<u32>,
    curve: V3,
}

fn v_add(a0: V3, b0: V3) -> V3 {
    var a = a0;
    var b = b0;
    return V3(add64(a[0], b[0]), add64(a[1], b[1]), add64(a[2], b[2]));
}

fn v_sub(a0: V3, b0: V3) -> V3 {
    var a = a0;
    var b = b0;
    return V3(sub64(a[0], b[0]), sub64(a[1], b[1]), sub64(a[2], b[2]));
}

fn v_scaled(v0: V3, s: vec2<u32>) -> V3 {
    var v = v0;
    return V3(mulq(v[0], s), mulq(v[1], s), mulq(v[2], s));
}

// `lay::falloff`: 1 - e^-x, Q32, floored.
fn falloff(x: vec2<u32>) -> vec2<u32> {
    return shr64(sub64(p_one(), exp2_neg(smul_shr(x, LOG2E, 62u, false))), 30u);
}

// `lay::falloff_keep`: e^-x, Q32, floored.
fn falloff_keep(x: vec2<u32>) -> vec2<u32> {
    return shr64(exp2_neg(smul_shr(x, LOG2E, 62u, false)), 30u);
}

fn turn_of(pivot: V3, angle: vec2<u32>) -> Turn {
    let cs = sincos(angle.x);
    return Turn(pivot, cs.xy, cs.zw, true);
}

// `lay::toward_elevation`.
fn toward_elevation(d0: V3, side0: V3, elevation: vec2<u32>, share0: vec2<u32>, cone: vec2<u32>) -> V3 {
    var d = d0;
    var side = side0;
    if !slt64(vec2<u32>(0u), share0) {
        return d;
    }
    let lean = length3(d[0], d[1], vec2<u32>(0u));
    var share = share0;
    if negative(d[2]) && slt64(lean, cone) {
        let q = xquotient(xof(pair(share0, lean)), ext4(cone)).v;
        share = vec2<u32>(q[0], q[1]);
    }
    var out = unit3(V3(d[0], d[1], vec2<u32>(0u)));
    if !out.some {
        out = unit3(V3(side[0], side[1], vec2<u32>(0u)));
        if !out.some {
            return d;
        }
    }
    let now = between(0u, atan2_t(d[2], lean));
    let to = add64(now, smul_shr(sub64(elevation, now), share, 32u, false));
    let cs = sincos(to.x);
    var o = out.c;
    return V3(mulq(o[0], cs.xy), mulq(o[1], cs.xy), add64(mulq(o[2], cs.xy), cs.zw));
}

// `lay::support_q`: how far wood at height z stands free of the ground.
fn support(z: vec2<u32>, radius: vec2<u32>, length: vec2<u32>) -> vec2<u32> {
    let zone = add64(shl64(radius, 2u), add64(shl64(length, 2u), shl64(length, 1u)));
    if !slt64(z, zone) {
        return one32();
    }
    return isqrt(shift4(u4(over(smax64(z, vec2<u32>(0u)), zone)), 32u, true, false));
}

// The step's phytomer: its key, sized scale and radius in, its tip,
// heading and side out.
struct Node {
    key: vec2<u32>,
    scale: vec2<u32>,
    radius: vec2<u32>,
    tip: V3,
    heading: V3,
    side: V3,
}

// `Layer::step` without a load: 0 laid, 1 beyond the domain, 2 the trunk
// below the ground (its height in `node.tip[2]`).
fn step(walk: ptr<function, Walk>, node: ptr<function, Node>, pa: u32, job: u32, bend: vec2<u32>, total: vec2<u32>, trunk: bool) -> u32 {
    let p = pa * PA;
    let flags = pas[p + 5u].x;
    var running = (*walk).running;
    var across = (*walk).across;
    let tip = (*walk).tip;
    var run = (*walk).run;
    var curve = (*walk).curve;
    let scale = (*node).scale;
    // `lineage::internode` and `lineage::tropism`: each one product, one
    // floor, the plain product's bits without spread (fn-222, fn-224).
    var varied = one32();
    if slt64(vec2<u32>(0u), pas[p + 7u]) {
        let u = shr64(draw(child((*node).key, KEY_INTERNODE)), 21u).x;
        varied = add64(one32(), mulq(pas[p + 7u], sub64(w(u << 1u, u >> 31u), one32())));
    }
    let length = prod_shr(pas[p], scale, varied, w(1u, 0u), 64u).v.xy;
    let share = falloff(prod_shr(pas[p + 1u], jobs[job * JOB + 23u], length, w(1u, 0u), 64u).v.xy);
    let bent = toward_elevation(running, across, pas[p + 2u], share, pas[p + 6u]);
    let t = carry(running, bent);
    if t.some {
        across = turned_by(across, t);
        curve = turned_by(curve, t);
    }
    running = bent;
    let wanders = (flags & 1u) != 0u;
    var signed = vec2<u32>(0u);
    var drawn = vec2<u32>(0u);
    var pivot: V3;
    if wanders {
        let key = child((*node).key, KEY_WANDER);
        let u = shr64(draw(child(key, w(0u, 0u))), 21u).x;
        signed = sub64(w(u << 1u, u >> 31u), one32());
        drawn = mulq(pas[p + 3u], signed);
        let cs = sincos(shr64(draw(child(key, w(1u, 0u))), 21u).x);
        pivot = v_add(v_scaled(across, cs.xy), v_scaled(cross3(running, across), cs.zw));
    }
    if (flags & 2u) != 0u {
        let keep = falloff_keep(over(length, pas[p + 4u]));
        curve = v_scaled(curve, keep);
        if wanders {
            curve = v_add(curve, v_scaled(pivot, mulq(drawn, rest(keep))));
        }
        curve = v_sub(curve, v_scaled(running, dot3(curve, running)));
        var c = curve;
        let angle = turns_of(mulq(length3(c[0], c[1], c[2]), length));
        let axis = unit3(curve);
        if axis.some {
            let r = turn_of(axis.c, angle);
            running = turned_by(running, r);
            across = turned_by(across, r);
        }
    } else if wanders {
        let angle = turns_of(mulq(mulq(pas[p + 3u], length), signed));
        let r = turn_of(pivot, angle);
        running = turned_by(running, r);
        across = turned_by(across, r);
        curve = v_scaled(pivot, drawn);
    } else {
        curve = V3(vec2<u32>(0u), vec2<u32>(0u), vec2<u32>(0u));
    }
    // Its own pull and those it carries on, each fading over its reach.
    var lift = vec2<u32>(0u);
    var up = vec2<u32>(0u);
    let j = job * JOB;
    let lifts = jobs[j + 22u];
    for (var k = 0u; k <= lifts.y; k++) {
        var l_bend = bend;
        var l_pull = (*walk).pull;
        var l_total = total;
        var l_offset = vec2<u32>(0u);
        if k > 0u {
            let at = lifts.x + 4u * (k - 1u);
            l_bend = tables[at];
            l_pull = tables[at + 1u];
            l_total = tables[at + 2u];
            l_offset = tables[at + 3u];
        }
        var along = one32();
        if slt64(vec2<u32>(0u), l_total) {
            along = over(add64(add64(l_offset, run), shr64(scale, 1u)), l_total);
        }
        let here = mulq(l_bend, smax64(sub64(one32(), along), vec2<u32>(0u)));
        lift = add64(lift, here);
        up = add64(up, mulq(here, l_pull));
    }
    run = add64(run, scale);
    var blend = v_scaled(running, sub64(one32(), lift));
    blend[2] = add64(blend[2], up);
    var direction = running;
    let u = unit3(blend);
    if u.some && slt64(vec2<u32>(0u), lift) {
        direction = u.c;
    }
    if !trunk && negative(direction[2]) {
        var t3 = tip;
        let ease = support(t3[2], (*node).radius, length);
        if slt64(ease, one32()) {
            let eased = unit3(V3(direction[0], direction[1], mulq(direction[2], ease)));
            if eased.some {
                direction = eased.c;
            }
        }
    }
    var next = v_add(tip, v_scaled(direction, length));
    for (var k = 0u; k < 3u; k++) {
        if !lt64(abs64(next[k]), w(0u, REACH_HI)) {
            return 1u;
        }
    }
    if trunk && slt64(next[2], neg64(w(GROUND, 0u))) {
        (*node).tip = next;
        return 2u;
    }
    if !trunk && negative(next[2]) {
        next[2] = vec2<u32>(0u);
        let rested = unit3(v_sub(next, tip));
        if rested.some {
            direction = rested.c;
        }
    }
    (*node).tip = next;
    (*node).heading = direction;
    let side = unit3(v_sub(across, v_scaled(direction, dot3(across, direction))));
    (*node).side = across;
    if side.some {
        (*node).side = side.c;
    }
    (*walk).running = running;
    (*walk).across = across;
    (*walk).tip = next;
    (*walk).run = run;
    (*walk).curve = curve;
    return 0u;
}

fn put(o: u32, v0: V3) {
    var v = v0;
    laid[o] = v[0];
    laid[o + 1u] = v[1];
    laid[o + 2u] = v[2];
}

// One axis's units laid: their phytomers sized and ranked, then each
// stepped on from the walk.
@compute @workgroup_size(64)
fn lay(@builtin(global_invocation_id) id: vec3<u32>) {
    let job = id.x + id.y * cfg.row;
    if job >= cfg.n {
        return;
    }
    let j = job * JOB;
    let pa = jobs[j].x;
    let trunk = jobs[j].y != 0u;
    let bend = jobs[j + 1u];
    var walk = Walk(
        V3(jobs[j + 2u], jobs[j + 3u], jobs[j + 4u]),
        V3(jobs[j + 5u], jobs[j + 6u], jobs[j + 7u]),
        V3(jobs[j + 8u], jobs[j + 9u], jobs[j + 10u]),
        jobs[j + 11u],
        jobs[j + 12u],
        V3(jobs[j + 13u], jobs[j + 14u], jobs[j + 15u]),
    );
    let base = jobs[j + 16u];
    var running = jobs[j + 17u];
    var rank = jobs[j + 18u];
    var wood = jobs[j + 19u];
    let units = jobs[j + 20u];
    var at = jobs[j + 21u].x;
    var status = 0u;
    var z = vec2<u32>(0u);
    for (var k = 0u; k < units.y && status == 0u; k++) {
        let u = units.x + 5u * k;
        let grown = tables[u];
        let present = mulq(running, tables[u + 1u]);
        let size = tables[u + 3u];
        let count = tables[u + 4u].x;
        // Unsized: what it bears inherits its scale without its size.
        var total = walk.run;
        for (var n = 0u; n < count; n++) {
            let node = nodes[(at + n) * 3u + 1u];
            let scale = prod_shr(base, present, node, grown, 96u);
            if scale.status != 0u {
                status = 3u;
            }
            let o = (at + n) * PHYTOMER;
            laid[o] = scale.v.xy;
            laid[o + 1u] = rank;
            rank = add64(rank, mulq(node, grown));
            total = add64(total, mulq(scale.v.xy, size));
        }
        // Laid at its size, then left unsized for what it bears.
        for (var n = 0u; n < count && status == 0u; n++) {
            let o = (at + n) * PHYTOMER;
            let sized = mulq(laid[o], size);
            var node = Node(nodes[(at + n) * 3u], sized, nodes[(at + n) * 3u + 2u], V3(), V3(), V3());
            status = step(&walk, &node, pa, job, bend, total, trunk);
            if status == 2u {
                var t = node.tip;
                z = t[2];
            }
            if status == 0u {
                put(o + 2u, node.tip);
                put(o + 5u, node.heading);
                put(o + 8u, node.side);
                wood = add64(wood, mulq(pas[pa * PA], sized));
            }
        }
        at += count;
        running = mulq(present, tables[u + 2u]);
    }
    let r = job * OUT;
    out[r] = vec2<u32>(status, 0u);
    out[r + 1u] = z;
    out[r + 2u] = walk.running[0];
    out[r + 3u] = walk.running[1];
    out[r + 4u] = walk.running[2];
    out[r + 5u] = walk.across[0];
    out[r + 6u] = walk.across[1];
    out[r + 7u] = walk.across[2];
    out[r + 8u] = walk.tip[0];
    out[r + 9u] = walk.tip[1];
    out[r + 10u] = walk.tip[2];
    out[r + 11u] = walk.run;
    out[r + 12u] = walk.pull;
    out[r + 13u] = walk.curve[0];
    out[r + 14u] = walk.curve[1];
    out[r + 15u] = walk.curve[2];
    out[r + 16u] = rank;
    out[r + 17u] = running;
    out[r + 18u] = wood;
}
