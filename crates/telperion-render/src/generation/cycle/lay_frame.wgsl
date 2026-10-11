// fn-256 task 2: a new axis framed from its parent alone and its pencil
// begun (`grow::draw::pencil`, `geometry::frame`), in the CPU's f32
// operations. Part of the layout's shader.

// `side` turned about `heading` by the amount `turn`.
fn spun(heading: vec3<f32>, side: vec3<f32>, turn: f32) -> vec3<f32> {
    let cs = qsincos(turn);
    return mv(side, cs.x) + mv(qcross(heading, side), cs.y);
}

// The heading and side of a bud at `azimuth` about a parent going
// `along` with `side`, turned `epitony` of the way to its upper side,
// inserted at `angle`, its plane turned by `plane` (`geometry::turned`).
fn turned(along: vec3<f32>, side: vec3<f32>, azimuth: f32, angle: f32, plane: f32, epitony: f32) -> array<vec3<f32>, 2> {
    var toward = spun(along, side, azimuth);
    let level = UP - mv(along, qdot(UP, along));
    let upper = qunit(level);
    if upper.w > 0.0 {
        let facing = qdot(toward, upper.xyz);
        let turn = qbetween(0.0, qatan2(qdot(along, qcross(toward, upper.xyz)), facing));
        let sided = min(qdiv(qlength(level), UPRIGHT), 1.0);
        let epi = apart(apart(apart(apart(epitony * sided) * turn) * (1.0 + facing)) * 0.5);
        toward = qrotated(toward, along, epi);
    }
    let cs = qsincos(angle);
    let heading = mv(along, cs.x) + mv(toward, cs.y);
    let s = mv(toward, cs.x) - mv(along, cs.y);
    return array<vec3<f32>, 2>(heading, spun(heading, s, plane));
}

// An axis's insertion angle (`lineage::insertion`), an amount.
fn insertion_of(i: u32) -> f32 {
    let pa = ax(i, A_PA);
    let angle = fm(pa, F_INSERTION);
    let spread = fm(pa, F_INSERTION_SPREAD);
    if spread <= 0.0 {
        return angle;
    }
    return angle + m(spread, signed24(insertion_draw(lineage_of(i))));
}

// A lateral's share of its vigour among its siblings (`dominance`).
fn dominance_of(i: u32) -> f32 {
    let d = fm(ax(i, A_PA), F_DOMINANCE);
    if d <= 0.0 {
        return 1.0;
    }
    let u = uniform24(draw(child(lineage_of(i), KEY_DOMINANCE)));
    let held = m(apart(apart(u * u) * u), u);
    return 1.0 - m(d, 1.0 - held);
}

// The axis's own tropism over its PA's (`lineage::tropism`).
fn tropism_of(i: u32) -> f32 {
    let spread = fm(ax(i, A_PA), F_TROPISM_SPREAD);
    if spread <= 0.0 {
        return 1.0;
    }
    return 1.0 + m(spread, signed24(tropism_draw(lineage_of(i))));
}

// Where axis `p` ends: its last node, or its base.
fn end_of(p: u32) -> vec3<f32> {
    let n = nphyt(p);
    if n == 0u {
        return ax3(p, A_BASE);
    }
    return ph3(prow(p, n - 1u), PH_TIP);
}

// The pulls axis `i`, carried on at blend `b`, takes from its parent
// `p`'s walk (`Layer::handed`), into its pencil.
fn hand_lifts(i: u32, p: u32, b: f32) {
    let made = i32(ax(i, A_BIRTH)) * i32(TICKS) + i32(ax(i, A_SLEEP));
    let born = i32(ax(p, A_BIRTH)) * i32(TICKS) + i32(ax(p, A_SLEEP));
    let bent = bend_of(p, years_of(max(made - born, 0)));
    let keep = 1.0 - b;
    var n = 0u;
    if keep > 0.0 {
        let run = pnf(p, P_RUN);
        for (var k = 0u; k <= pn(p, P_LIFTS_N); k++) {
            var l = vec4<f32>(bent, pnf(p, P_PULL), run, 0.0);
            if k > 0u {
                l = lift_at(p, k - 1u);
            }
            l.x = l.x * keep;
            l.w = l.w + run;
            if l.x > 0.0 && l.w < l.z {
                if n == LIFTS {
                    status |= FAULT_LIFTS;
                    break;
                }
                for (var c = 0u; c < 4u; c++) {
                    set_pnf(i, P_LIFTS + 4u * n + c, l[c]);
                }
                n += 1u;
            }
        }
    }
    set_pn(i, P_LIFTS_N, n);
}

// Axis `i` framed on its parent and its pencil begun.
fn frame(i: u32) {
    let kind = kind_of(i);
    let made = axf(i, A_B0) * axf(i, A_B1);
    var base = vec3<f32>(0.0);
    var heading = UP;
    var side = vec3<f32>(1.0, 0.0, 0.0);
    var curve = vec3<f32>(0.0);
    var inherited = 1.0;
    var vigour = made;
    var trunk = true;
    var share = 1.0;
    if kind != SEED {
        let p = ax(i, A_PARENT);
        let ppa = ax(p, A_PA);
        trace_line(p);
        var along = vec3<f32>(0.0);
        var aside = vec3<f32>(0.0);
        var azimuth = 0.0;
        var epitony = 0.0;
        let b = blend_of(i);
        if kind == LATERAL {
            let node = ax(i, A_NODE);
            let r = prow(p, node);
            along = ph3(r, PH_HEADING);
            aside = ph3(r, PH_SIDE);
            let roll = m(fm(ax(i, A_PA), F_ROLL), signed24(draw(child(lineage_of(i), KEY_ROLL))));
            let slot = qdiv(f32((ax(i, A_ORIGIN) >> 8u) & 0xffu), axf(i, A_WHORL));
            azimuth = line_ph[0] + by_rank(ppa, phf(r, PH_RANK)) + slot + roll;
            let tip = ph3(r, PH_TIP);
            base = tip;
            if (ax(i, A_ORIGIN) & 0x10000u) != 0u {
                var below = ax3(p, A_BASE);
                if node > 0u {
                    below = ph3(prow(p, node - 1u), PH_TIP);
                }
                base = (below + tip) * 0.5;
            }
            inherited = phf(r, PH_SCALE);
            trunk = false;
            share = dominance_of(i);
        } else {
            curve = pn3(p, P_CURVE);
            inherited = pnf(p, P_BASE_SCALE);
            vigour = pnf(p, P_GOING) * made;
            trunk = pn(p, P_TRUNK) != 0u;
            if b != 0.0 {
                let at = stand(p, ax(i, A_NODE));
                along = at.along;
                aside = at.side;
                azimuth = at.rank;
                epitony = fm(ppa, F_EPITONY);
            }
        }
        var h = along;
        var s = aside;
        if kind == LATERAL || b != 0.0 {
            let t = turned(along, aside, azimuth, insertion_of(i), fm(ppa, F_PLANE), epitony);
            h = t[0];
            s = t[1];
        }
        if kind == LATERAL {
            heading = h;
            side = s;
        } else {
            let running = pn3(p, P_RUNNING);
            let across = pn3(p, P_ACROSS);
            if b == 0.0 {
                heading = qunit_or(running, running);
                side = qunit_or(across - mv(heading, qdot(across, heading)), across);
                base = end_of(p);
            } else {
                heading = qunit_or(mv(running, 1.0 - b) + mv(h, b), h);
                let sd = mv(across, 1.0 - b) + mv(s, b);
                side = qunit_or(sd - mv(heading, qdot(sd, heading)), across);
                base = stand_at(p, attach(i, p, b));
            }
            hand_lifts(i, p, b);
        }
    }
    set_ax3(i, A_BASE, base);
    set_ax3(i, A_HEADING, heading);
    set_ax3(i, A_SIDE, side);
    let bent_curve = curve - mv(heading, qdot(curve, heading));
    let wk = Walk(heading, side, base, 0.0, (1.0 + heading.z) * 0.5, bent_curve, tropism_of(i));
    keep_walk(i, wk);
    set_pnf(i, P_BASE_SCALE, apart(inherited * vigour) * share);
    set_pnf(i, P_GOING, 1.0);
    set_pnf(i, P_RANKED, 0.0);
    set_pn(i, P_UNITS, 0u);
    set_pn(i, P_LAID, 0u);
    set_pn(i, P_TRUNK, u32(trunk));
    set_pnf(i, P_WOOD, 0.0);
    for (var k = 0u; k < 4u; k++) {
        set_pnf(i, P_FRESH + k, 1.0);
    }
}
