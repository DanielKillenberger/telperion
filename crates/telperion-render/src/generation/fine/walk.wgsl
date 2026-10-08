// A chain laid (`geometry.wgsl`): each own axis framed on its bearer and
// walked phytomer by phytomer (`geometry::single::lay_one`), before it
// bends and under its levers; and the kernel. Part of its shader.

// The fields of a laid phytomer this pass writes: as laid (0) or before
// bending (9).
fn field(bent: bool) -> u32 {
    return select(9u, 0u, bent);
}

// Region axis `r`'s insertion angle, of PA values `f`: its PA's, offset by
// its lineage's draw (`lineage::insertion`, fn-224).
fn insertion_of(r: u32, f: Pa) -> f32 {
    if f.insertion_spread <= 0.0 {
        return f.insertion;
    }
    return f.insertion + f.insertion_spread * (2.0 * real_of(insertion_draw(lineage_of(r))) - 1.0);
}

// The base, heading and side of a bud on `bearer`'s phytomer `at` (a
// laid index) of rank `rank` on an axis of phase `phase` (turns), below
// it `below`, for region axis `r`; turned up by
// `epitony` (`geometry::frame`'s lateral, `bud_frame`).
fn lateral_frame(r: u32, bearer_pa: u32, at: array<vec3<f32>, 3>, rank: vec2<u32>, phase: f32, below: vec3<f32>) -> array<vec3<f32>, 3> {
    let f = pa_of(ax(r, PA));
    let fp = pa_of(bearer_pa);
    let roll = f.roll * (2.0 * unit_of(child(lineage_of(r), KEY_ROLL)) - 1.0);
    let slot = f32((ax(r, KIND) >> 8u) & 0xffu);
    // Its bearer's phyllotaxis runs on from its axis's phase (fn-199).
    let azimuth = TAU * (phase + turns(rank, divergence_q(bearer_pa))) + TAU * slot / axf(r, WHORL) + roll;
    let t = turned(at[1], at[2], azimuth, insertion_of(r, f), fp.plane, 0.0);
    var base = at[0];
    if (ax(r, KIND) & IS_WOKEN) != 0u {
        base = (below + at[0]) * 0.5;
    }
    return array<vec3<f32>, 3>(base, t[0], t[1]);
}

// Where own axis `p` ends in this pass: its last node, or its base.
fn end_of(p: u32, o: u32) -> vec3<f32> {
    let n = s(p, S_COUNT);
    if n == 0u {
        return s3(p, S_FRAME);
    }
    return o3(lo(p, n - 1u) + o);
}

// How far along node `k_in`'s span of own axis `p` its relay at `share`
// of the last growth unit stands, by the unit's scaled lengths
// (`geometry::relay_span`); 1 on an axis of no phytomers.
fn relay_span(p: u32, share: f32, k_in: u32) -> f32 {
    let n = s(p, S_COUNT);
    if n == 0u {
        return 1.0;
    }
    let k = min(k_in, n - 1u);
    let last = ph(at_k(p, n - 1u), 2u);
    var first = 0u;
    for (var j = 0u; j < n; j++) {
        if ph(at_k(p, j), 2u) == last {
            first = j;
            break;
        }
    }
    var total = 0.0;
    for (var j = first; j < n; j++) {
        total += out[lo(p, j) + 19u];
    }
    let scale = out[lo(p, k) + 19u];
    if total > 0.0 && scale > 0.0 {
        var before = 0.0;
        for (var j = min(first, k); j < k; j++) {
            before += out[lo(p, j) + 19u];
        }
        return clamp((share * total - before) / scale, 0.0, 1.0);
    } else if total <= 0.0 {
        let fr = f32(first) + share * f32(n - first);
        return clamp(fr - f32(k), 0.0, 1.0);
    }
    return 1.0;
}

// Own axis `i`'s base, heading and side (`geometry::frame`); the first
// from its bearer's phytomer at `bearer` in the wave before, below it
// `below`, of PA `bearer_pa`.
fn frame_of(i: u32, bent: bool, bearer: u32, below: vec3<f32>, bearer_pa: u32) -> array<vec3<f32>, 3> {
    let o = field(bent);
    if i == 1u {
        let at = array<vec3<f32>, 3>(p3(bearer + o), p3(bearer + o + 3u), p3(bearer + o + 6u));
        let rank = vec2<u32>(bitcast<u32>(prev[bearer + 20u]), bitcast<u32>(prev[bearer + 21u]));
        return lateral_frame(i, bearer_pa, at, rank, prev[bearer + 22u], below);
    }
    // An axis carried on, a continuation at blend 0 (fn-199, host
    // 2026-10-07): from the line's stand (`stand.wgsl`) by its blend, the
    // parent's running direction and side carried on unturned.
    let p = ax(i, PARENT);
    let fp = pa_of(ax(p, PA));
    let along = s3(p, S_WALK);
    let across = s3(p, S_WALK + 3u);
    var b = 0.0;
    var node = 0u;
    if kind_of(i) == RELAYED {
        b = axf(i, BLEND);
        node = ax(i, NODE);
    }
    let at = stand(p, node, o);
    let t = turned(at[1].xyz, at[2].xyz, TAU * at[0].w, insertion_of(i, pa_of(ax(i, PA))), fp.plane, fp.epitony);
    let tip = end_of(p, o);
    let heading = unit_or(along * (1.0 - b) + t[0] * b, t[0]);
    let sd = across * (1.0 - b) + t[1] * b;
    let side = unit_or(sd - heading * dot(sd, heading), across);
    return array<vec3<f32>, 3>(tip + (at[0].xyz - tip) * b, heading, side);
}

// The pulls relay `i` carries on from its parent `p`'s walk
// (`Layer::handed`), into `i`'s scratch.
fn hand_lifts(i: u32, p: u32) {
    // A continuation is a relay at blend 0 (fn-199).
    var w_on = 1.0 - axf(i, BLEND);
    if kind_of(i) == CONTINUED {
        w_on = 1.0;
    }
    var n = 0u;
    let to = sb + i * SA + S_LIFTS + 1u;
    if w_on > 0.0 && (kind_of(i) == RELAYED || kind_of(i) == CONTINUED) {
        let run = sf(p, S_WALK + 9u);
        let src = sb + p * SA + S_LIFTS + 1u;
        let count = s(p, S_LIFTS);
        for (var k = 0u; k <= count; k++) {
            var l = vec4<f32>(bend_of(p, pa_of(ax(p, PA))), sf(p, S_WALK + 10u), sf(p, S_REACH), 0.0);
            if k > 0u {
                l = lift_at(src, k - 1u);
            }
            l.x *= w_on;
            l.w += run;
            if l.x > 0.0 && l.w < l.z {
                if n == LIFTS {
                    status |= 1u;
                    break;
                }
                for (var c = 0u; c < 4u; c++) {
                    scratch[to + 4u * n + c] = bitcast<u32>(l[c]);
                }
                n += 1u;
            }
        }
    }
    ss(i, S_LIFTS, n);
}

// Lays every own axis of the chain, before bending or under its levers;
// `first`: each phytomer's lever turned from its first lay's frame, as
// whole-tree growth's second lay does (FRAME-CORRECTION.md).
fn lay_all(na: u32, bent: bool, bearer: u32, below: vec3<f32>, bearer_pa: u32, ground: f32, first: bool) {
    let o = field(bent);
    for (var i = 1u; i < na; i++) {
        if s(i, S_OWN) == NONE {
            continue;
        }
        let fr = frame_of(i, bent, bearer, below, bearer_pa);
        if fr[0].z + ground < -1e-5 {
            status |= 2u;
        }
        var curve = vec3<f32>(0.0);
        if i > 1u {
            curve = s3(ax(i, PARENT), S_WALK + 6u);
            hand_lifts(i, ax(i, PARENT));
        } else {
            ss(i, S_LIFTS, 0u);
        }
        var f = pa_of(ax(i, PA));
        let own = vec2<f32>(bend_of(i, f), sf(i, S_REACH));
        // The axis's own tropism (`lineage::tropism`, fn-224), mean its PA's.
        if f.tropism_spread > 0.0 {
            f.tropism = f.tropism * (1.0 + f.tropism_spread * (2.0 * real_of(tropism_draw(lineage_of(i))) - 1.0));
        }
        var walk = Walk(fr[1], fr[2], fr[0], 0.0, (1.0 + fr[1].z) / 2.0, curve - fr[1] * dot(curve, fr[1]));
        let lifts = sb + i * SA + S_LIFTS + 1u;
        let n = s(i, S_LIFTS);
        let phase = axis_phase(i);
        for (var k = 0u; k < s(i, S_COUNT); k++) {
            let at = lo(i, k);
            let p = at_k(i, k);
            var st = Step(out[at + 19u], out[at + 18u], vec2<u32>(ph(p, 0u), ph(p, 1u)), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0));
            if first {
                st.was_heading = o3(at + 12u);
                st.was_side = o3(at + 15u);
            }
            let q = s(i, S_START) + k;
            let moment = vec3<f32>(pf(q, P_LEVER), pf(q, P_LEVER + 1u), pf(q, P_LEVER + 2u));
            let chord = vec3<f32>(pf(q, P_LEVER + 3u), pf(q, P_LEVER + 4u), pf(q, P_LEVER + 5u));
            step(&walk, &st, f, own, lifts, n, bent, moment, chord, ground);
            wo3(at + o, st.tip);
            wo3(at + o + 3u, st.heading);
            wo3(at + o + 6u, st.side);
            out[at + 22u] = fract(phase);
            // Its internode's factor, which its grown wood is counted by
            // (`dress.wgsl`).
            out[at + 23u] = internode_factor(f, st.key);
        }
        ss3(i, S_WALK, walk.running);
        ss3(i, S_WALK + 3u, walk.across);
        ss3(i, S_WALK + 6u, walk.curve);
        ssf(i, S_WALK + 9u, walk.run);
        ssf(i, S_WALK + 10u, walk.pull);
        ss3(i, S_FRAME, fr[0]);
        ss3(i, S_FRAME + 3u, fr[1]);
        ss3(i, S_FRAME + 6u, fr[2]);
        wo3((out_ax + s(i, S_OWN)) * AL + select(0u, 3u, bent), fr[0]);
    }
}
