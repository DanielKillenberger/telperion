// fn-256 task 2: the walk along an axis (`geometry::Layer::step`), in the
// CPU's f32 operations, without sag, as the rough layout takes it. Part
// of the layout's shader.

const CONTACT: f32 = 4.0;
const LANDING: f32 = 6.0;
const REACH: f32 = 4096.0;

// Where a walk stands (`Layer`); its pulls carried on stay in its
// pencil's columns.
struct Walk {
    running: vec3<f32>,
    across: vec3<f32>,
    tip: vec3<f32>,
    run: f32,
    pull: f32,
    curve: vec3<f32>,
    turning: f32,
}

fn walk_of(i: u32) -> Walk {
    return Walk(pn3(i, P_RUNNING), pn3(i, P_ACROSS), pn3(i, P_TIP), pnf(i, P_RUN), pnf(i, P_PULL), pn3(i, P_CURVE), pnf(i, P_TURNING));
}

fn keep_walk(i: u32, w: Walk) {
    set_pn3(i, P_RUNNING, w.running);
    set_pn3(i, P_ACROSS, w.across);
    set_pn3(i, P_TIP, w.tip);
    set_pnf(i, P_RUN, w.run);
    set_pnf(i, P_PULL, w.pull);
    set_pn3(i, P_CURVE, w.curve);
    set_pnf(i, P_TURNING, w.turning);
}

fn lift_at(i: u32, k: u32) -> vec4<f32> {
    let c = P_LIFTS + 4u * k;
    return vec4<f32>(pnf(i, c), pnf(i, c + 1u), pnf(i, c + 2u), pnf(i, c + 3u));
}

// A draw's uniform in f32 (`lineage::unit`) and its signed one.
fn uniform24(d: vec2<u32>) -> f32 {
    return f32(shr64(d, 29u).x) * (1.0 / 16777216.0);
}

fn signed24(d: vec2<u32>) -> f32 {
    return uniform24(d) * 2.0 - 1.0;
}

fn support(z: f32, radius: f32, len: f32) -> f32 {
    let zone = m(CONTACT, radius) + m(LANDING, len);
    if z >= zone {
        return 1.0;
    }
    return qsqrt(qdiv(max(z, 0.0), zone));
}

fn rest(k_in: f32) -> f32 {
    let k = clamp(k_in, 0.0, 1.0);
    return qsqrt(1.0 - m(k, k));
}

// `toward_elevation`: `d` turned in its vertical plane by `share` of its
// gap to `elevation`, weakened within the cone about straight down.
fn toward_elevation(d: vec3<f32>, side: vec3<f32>, elevation: f32, share_in: f32) -> vec3<f32> {
    if share_in <= 0.0 {
        return d;
    }
    let lean = qlength(vec3<f32>(d.x, d.y, 0.0));
    let cone = form[0];
    var share = share_in;
    if d.z < 0.0 && lean < cone {
        share = qdiv(share * lean, cone);
    }
    var out = qunit(vec3<f32>(d.x, d.y, 0.0));
    if out.w == 0.0 {
        out = qunit(vec3<f32>(side.x, side.y, 0.0));
        if out.w == 0.0 {
            return d;
        }
    }
    let now = qbetween(0.0, qatan2(d.z, lean));
    let to = now + m(elevation - now, share);
    let cs = qsincos(to);
    return mv(out.xyz, cs.x) + mv(UP, cs.y);
}

// `Layer::step` for axis `i`'s phytomer at row `r`, laid at `scale`,
// its own pull `bent` over `total`; a fault where it would stop.
fn step(wk: ptr<function, Walk>, i: u32, r: u32, scale: f32, bent: f32, total: f32, trunk: bool) {
    let pa = ax(i, A_PA);
    var running = (*wk).running;
    var across = (*wk).across;
    var curve = (*wk).curve;
    let tip = (*wk).tip;
    var run = (*wk).run;
    let key = vec2<u32>(ph(r, PH_KEY), ph(r, PH_KEY + 1u));
    var varied = 1.0;
    if fm(pa, F_INTERNODE_SPREAD) > 0.0 {
        varied = 1.0 + m(fm(pa, F_INTERNODE_SPREAD), signed24(internode_draw(key)));
    }
    let len = apart(fm(pa, F_INTERNODE) * scale) * varied;
    let tropic = apart(fm(pa, F_TROPISM) * (*wk).turning) * len;
    let bent_to = toward_elevation(running, across, fm(pa, F_ELEVATION), fails(apart(tropic * QLOG2E)));
    var ok = false;
    let t = qcarry(running, bent_to, &ok);
    if ok {
        across = qturned_by(across, t);
        curve = qturned_by(curve, t);
    }
    running = bent_to;
    let wander = fm(pa, F_WANDER);
    let kicked = wander > 0.0;
    var sgn = 0.0;
    var kick = 0.0;
    var pivot = vec3<f32>(0.0);
    if kicked {
        let k = child(key, KEY_WANDER);
        sgn = signed24(draw(child(k, w(0u, 0u))));
        kick = wander * sgn;
        let cs = qsincos(uniform24(draw(child(k, w(1u, 0u)))));
        pivot = mv(across, cs.x) + mv(qcross(running, across), cs.y);
    }
    let bend_length = fm(pa, F_BEND_LENGTH);
    var turn_about = vec3<f32>(0.0);
    var angle = 0.0;
    var turns = false;
    if bend_length > 0.0 {
        let keep = exp2c(apart(-qdiv(len, bend_length) * QLOG2E));
        curve = mv(curve, keep);
        if kicked {
            curve = curve + mv(pivot, kick * rest(keep));
        }
        curve = curve - mv(running, qdot(curve, running));
        angle = qturns(qlength(curve) * len);
        let u = qunit(curve);
        turns = u.w > 0.0;
        turn_about = u.xyz;
    } else if kicked {
        angle = qturns(apart(wander * len) * sgn);
        turn_about = pivot;
        turns = true;
        curve = pivot * kick;
    } else {
        curve = vec3<f32>(0.0);
    }
    if turns {
        let t2 = qturn(turn_about, angle);
        running = qturned_by(running, t2);
        across = qturned_by(across, t2);
    }
    // Its own pull, and those it carries on, each fading over its reach.
    var lift = 0.0;
    var up = 0.0;
    let n = pn(i, P_LIFTS_N);
    for (var k = 0u; k <= n; k++) {
        var l = vec4<f32>(bent, (*wk).pull, total, 0.0);
        if k > 0u {
            l = lift_at(i, k - 1u);
        }
        var along = 1.0;
        if l.z > 0.0 {
            along = qdiv(l.w + run + m(scale, 0.5), l.z);
        }
        let here = m(l.x, max(1.0 - along, 0.0));
        lift = lift + here;
        up = up + m(here, l.y);
    }
    run = run + scale;
    let blend = mv(running, 1.0 - lift) + vec3<f32>(0.0, 0.0, up);
    var direction = running;
    let ub = qunit(blend);
    if ub.w > 0.0 && lift > 0.0 {
        direction = ub.xyz;
    }
    if !trunk && direction.z < 0.0 {
        let ease = support(tip.z, 0.0, len);
        if ease < 1.0 {
            direction = qunit_or(vec3<f32>(direction.x, direction.y, direction.z * ease), direction);
        }
    }
    let moved = tip + mv(direction, len);
    var next = vec3<f32>(apart(moved.x), apart(moved.y), apart(moved.z));
    if any(abs(next) >= vec3<f32>(REACH)) {
        status |= FAULT_BEYOND;
    }
    if trunk && next.z <= form[1] {
        status |= FAULT_BELOW;
    }
    if !trunk && next.z < 0.0 {
        next.z = 0.0;
        direction = qunit_or(next - tip, direction);
    }
    set_ph3(r, PH_TIP, next);
    set_ph3(r, PH_HEADING, direction);
    set_ph3(r, PH_SIDE, qunit_or(across - mv(direction, qdot(across, direction)), across));
    (*wk).running = running;
    (*wk).across = across;
    (*wk).tip = next;
    (*wk).run = run;
    (*wk).curve = curve;
}
