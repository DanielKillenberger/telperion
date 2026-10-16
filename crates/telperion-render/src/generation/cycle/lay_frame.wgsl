// fn-256 task 2: an axis framed from its laid parent alone
// (`geometry::framed`) and how far its base is pulled towards the
// vertical (`geometry::bend`), in the CPU's f32 operations. Part of the
// layout's shader and of the settle's place.

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

// How far axis `a`, `years` old, is pulled towards the vertical at its
// base (`geometry::bend`).
fn bend_of(a: u32, years: f32) -> f32 {
    let pa = ax(a, A_PA);
    let x = fm(pa, F_ERECTION) * years;
    let grown = fails(apart(x * QLOG2E));
    let kind = kind_of(a);
    var straightening = 0.0;
    var erected = grown;
    if kind == CONTINUED || kind == RELAYED {
        let b = select(0.0, axf(a, A_BLEND), kind == RELAYED);
        straightening = m(fm(pa, F_STRAIGHTENING), b);
        erected = m(b, grown);
    } else if kind == LATERAL {
        straightening = fm(pa, F_STRAIGHTENING);
    }
    return 1.0 - m(1.0 - straightening, 1.0 - erected);
}

// An axis's frame on its laid parent: its base, heading and side, and
// the curvature it carries on, square to its heading.
struct Framed {
    base: vec3<f32>,
    heading: vec3<f32>,
    side: vec3<f32>,
    curve: vec3<f32>,
}

// Axis `i` framed on its laid parent alone (`geometry::framed`): the
// seed at the ground; a lateral at its node, turned about its parent by
// its azimuth and inserted; an axis carried on between its parent's line
// at its tip and its own stand, by its blend.
fn framed(i: u32) -> Framed {
    let kind = kind_of(i);
    if kind == CONTINUED || kind == RELAYED {
        return framed_on(i, stood(i));
    }
    return framed_lateral(i);
}

// Where an axis carried on stands on its parent's line, where its blend
// is not none (`stand`); none elsewhere. The layout takes it in a dispatch
// before the rest of its frame: `stand` is most of the frame's cold
// compile (fn-256 task 6).
fn stood(i: u32) -> Point {
    let p = ax(i, A_PARENT);
    if blend_of(i) == 0.0 {
        return Point(vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), 0.0);
    }
    trace_line(p);
    return stand(p, ax(i, A_NODE));
}

// The seed at the ground, or a lateral at its node: `framed` for all but
// an axis carried on. Its own entry's in the layout, whose cold compile
// the two paths together took 1.3 s of (fn-256 task 6).
fn framed_lateral(i: u32) -> Framed {
    if kind_of(i) == SEED {
        return Framed(vec3<f32>(0.0), UP, vec3<f32>(1.0, 0.0, 0.0), vec3<f32>(0.0));
    }
    let p = ax(i, A_PARENT);
    let ppa = ax(p, A_PA);
    trace_line(p);
    let node = ax(i, A_NODE);
    let r = prow(p, node);
    let along = ph3(r, PH_HEADING);
    let aside = ph3(r, PH_SIDE);
    let roll = m(fm(ax(i, A_PA), F_ROLL), signed24(draw(child(lineage_of(i), KEY_ROLL))));
    let slot = qdiv(f32((ax(i, A_ORIGIN) >> 8u) & 0xffu), axf(i, A_WHORL));
    let azimuth = line_ph[0] + by_rank(ppa, phf(r, PH_RANK)) + slot + roll;
    let tip = ph3(r, PH_TIP);
    var base = tip;
    if (ax(i, A_ORIGIN) & 0x10000u) != 0u {
        var below = ax3(p, A_BASE);
        if node > 0u {
            below = ph3(prow(p, node - 1u), PH_TIP);
        }
        base = (below + tip) * 0.5;
    }
    let t = turned(along, aside, azimuth, insertion_of(i), fm(ppa, F_PLANE), 0.0);
    let heading = t[0];
    return Framed(base, heading, t[1], vec3<f32>(0.0) - mv(heading, qdot(vec3<f32>(0.0), heading)));
}

// An axis carried on: between its parent's line at its tip and its own
// stand `at` (`stood`), by its blend.
fn framed_on(i: u32, at: Point) -> Framed {
    let p = ax(i, A_PARENT);
    let ppa = ax(p, A_PA);
    trace_line(p);
    let b = blend_of(i);
    let curve = pn3(p, P_CURVE);
    var h = vec3<f32>(0.0);
    var s = vec3<f32>(0.0);
    if b != 0.0 {
        let t = turned(at.along, at.side, at.rank, insertion_of(i), fm(ppa, F_PLANE), fm(ppa, F_EPITONY));
        h = t[0];
        s = t[1];
    }
    let running = pn3(p, P_RUNNING);
    let across = pn3(p, P_ACROSS);
    var heading = vec3<f32>(0.0);
    var side = vec3<f32>(0.0);
    var base = vec3<f32>(0.0);
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
    return Framed(base, heading, side, curve - mv(heading, qdot(curve, heading)));
}

// How far axis `i` runs back onto a line, and the line, into its
// pencil: its own take-over as a relay (`geometry::taken`), and what it
// carries on of its parent's, left where the parent's walk ended over its
// reach `total`, as far as `i` is still its continuation
// (`Layer::handed_take`), their lines weighted by how far each pulls
// (`geometry::joined`); none on a seed or lateral.
fn take_over(i: u32, total: f32) {
    var own = 0.0;
    var mine = vec3<f32>(0.0);
    var carried = 0.0;
    var line = vec3<f32>(0.0);
    let kind = kind_of(i);
    if kind == CONTINUED || kind == RELAYED {
        let p = ax(i, A_PARENT);
        let b = blend_of(i);
        if kind == RELAYED {
            let t = m(fm(ax(i, A_PA), F_TAKEOVER), b);
            let u = qunit(pn3(p, P_RUNNING));
            if u.w > 0.0 && t > 0.0 {
                own = t;
                mine = u.xyz;
            }
        }
        let keep = 1.0 - b;
        let had = pnf(p, P_TAKE);
        if keep > 0.0 && had > 0.0 && total > 0.0 {
            let left = max(1.0 - qdiv(pnf(p, P_RUN), total), 0.0);
            let c = m(m(had, left), keep);
            if c > 0.0 {
                carried = c;
                line = pn3(p, P_LINE);
            }
        }
    }
    var take = 0.0;
    var at = vec3<f32>(0.0);
    if own > 0.0 && carried > 0.0 {
        take = own + carried;
        for (var c = 0u; c < 3u; c++) {
            at[c] = qdiv(m(mine[c], own) + m(line[c], carried), take);
        }
    } else if own > 0.0 {
        take = own;
        at = mine;
    } else if carried > 0.0 {
        take = carried;
        at = line;
    }
    set_pnf(i, P_TAKE, take);
    for (var c = 0u; c < 3u; c++) {
        set_pnf(i, P_LINE + c, at[c]);
    }
}

// The pulls axis `i`, carried on at blend `b`, takes from its parent
// `p`'s walk (`Layer::handed`): `own`, the parent's own pull's bend and
// the length it fades over, then those the parent carries, each as far
// as `i` is still the parent's continuation, into `i`'s pencil.
fn hand_lifts(i: u32, p: u32, b: f32, own: vec2<f32>) {
    let keep = 1.0 - b;
    var n = 0u;
    if keep > 0.0 {
        let run = pnf(p, P_RUN);
        for (var k = 0u; k <= min(pn(p, P_LIFTS_N), LIFTS); k++) {
            var l = vec4<f32>(own.x, pnf(p, P_PULL), own.y, 0.0);
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
