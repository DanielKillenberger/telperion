// fn-256 task 2 (SETTLE-PLAN.md S4 and S5): sag over the node table.
// Each phytomer's lever gathered on the tree laid unbent (`sag::gather`),
// a depth at a time from the deepest; and its turn as the tree is laid
// again (`sagged::sag_angle`), in the CPU's f32 operations.

// The sag of phytomer `r` of axis `i`, `len` long, laid at `at` (its
// running direction, side and tip): where its PA sags, it has girth and
// the place takes its load (`sagged::sag_angle`).
fn sag_of(i: u32, r: u32, len: f32, at: mat3x3<f32>, trunk: bool) -> Sag {
    let none = Sag(false, vec3<f32>(0.0), 0.0);
    let pa = ax(i, A_PA);
    let radius = phf(r, PH_RADIUS);
    if cfg.sagging == 0u || (kpa(pa, K_FLAGS) & KF_SAGS) == 0u || !(radius > 0.0) {
        return none;
    }
    let moment = vec3<f32>(pq_at(r, Q_LEVER), pq_at(r, Q_LEVER + 1u), pq_at(r, Q_LEVER + 2u));
    let chord0 = vec3<f32>(pq_at(r, Q_LEVER + 3u), pq_at(r, Q_LEVER + 4u), pq_at(r, Q_LEVER + 5u));
    let laid = sag_frame(ph3(r, PH_HEADING), ph3(r, PH_SIDE));
    let here = sag_frame(at[0], at[1]);
    var now = moment;
    var chord = chord0;
    if laid.ok && here.ok {
        now = frame_carry(laid, here, moment);
        chord = frame_carry(laid, here, chord0);
    }
    var share = 1.0;
    if !trunk {
        share = held(at[2].z, chord);
    }
    now = sub_scaled(now, share);
    let pivot = unit_w(torque(now));
    if pivot.l <= -INF {
        return none;
    }
    let running = at[0];
    let square = running - mv(pivot.u, qdot(running, pivot.u));
    let down = vec3<f32>(0.0, 0.0, -1.0);
    var room = qbetween(0.0, qatan2(qdot(qcross(square, down), pivot.u), qdot(square, down)));
    if room < -0.25 {
        room = room + 1.0;
    }
    if room > 0.5 {
        room = m(2.0, 0.75 - room);
    }
    let cap = unit_w(torque(moment)).l + log2_q(share);
    var angle = sag_turn(now, cap, kf(pa, K_SAGLOG), radius, len);
    angle = min(angle, max(room - HANG_AMOUNT, 0.0));
    if !trunk {
        angle = m(angle, support(at[2].z, radius, len));
    }
    return Sag(true, pivot.u, angle);
}

// A load (mass, then moment) `s` of it (`Load::share`).
fn scaled4(v: vec4<f32>, s: f32) -> vec4<f32> {
    return vec4<f32>(m(v.x, s), m(v.y, s), m(v.z, s), m(v.w, s));
}

// Phytomer `r`'s node bears `load` too.
fn add_load(r: u32, load: vec4<f32>) {
    for (var c = 0u; c < 4u; c++) {
        set_pq_at(r, Q_LOAD + c, pq_at(r, Q_LOAD + c) + load[c]);
    }
}

// Axis `i`'s phytomers' levers on the tree as laid (`sag::gather`): what
// each bears at its node and beyond, its children's loads, the last
// first, a relay's split where it joins; its far tip and its load at its
// base for its parent.
@compute @workgroup_size(64)
fn lever_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = listed(id);
    if i == NONE {
        return;
    }
    let pa = ax(i, A_PA);
    let n = nphyt(i);
    let trunk = py(i, Y_TRUNK) != 0u;
    var far = ax3(i, A_BASE);
    if n > 0u {
        far = ph3(prow(i, n - 1u), PH_TIP);
    }
    for (var k = 0u; k < n; k++) {
        for (var c = 0u; c < 4u; c++) {
            set_pq_at(prow(i, k), Q_LOAD + c, 0.0);
        }
    }
    var at_tip = vec4<f32>(0.0);
    var at_base = vec4<f32>(0.0);
    let first = wf(WF_KSTART, i);
    for (var x = 0u; x < wf(WF_KC, i); x++) {
        let c = wf(WF_KBI, first + x);
        let load = vec4<f32>(pyf(c, Y_LOAD), pyf(c, Y_LOAD + 1u), pyf(c, Y_LOAD + 2u), pyf(c, Y_LOAD + 3u));
        let kind = kind_of(c);
        let k = py(c, Y_ATTK);
        if kind == CONTINUED {
            far = vec3<f32>(pyf(c, Y_FAR), pyf(c, Y_FAR + 1u), pyf(c, Y_FAR + 2u));
        }
        if kind == LATERAL && ax(c, A_NODE) < n {
            add_load(prow(i, ax(c, A_NODE)), load);
        } else if kind == RELAYED && k != NONE {
            let part = scaled4(load, pyf(c, Y_ATTT));
            add_load(prow(i, k), part);
            if k > 0u {
                add_load(prow(i, k - 1u), load - part);
            } else {
                at_base = at_base + (load - part);
            }
        } else {
            at_tip = at_tip + load;
        }
    }
    var load = at_tip;
    for (var k = n; k > 0u; k--) {
        let r = prow(i, k - 1u);
        var base = ax3(i, A_BASE);
        if k > 1u {
            base = ph3(prow(i, k - 2u), PH_TIP);
        }
        let tip = ph3(r, PH_TIP);
        let length = qlength(tip - base);
        let leaves = own_pipe(pa, r, 1.0);
        let radius = phf(r, PH_RADIUS);
        let mass = apart((m(radius, radius) + m(leaves, leaves)) * length);
        let middle = (base + tip) * 0.5;
        load = load + vec4<f32>(pq_at(r, Q_LOAD), pq_at(r, Q_LOAD + 1u), pq_at(r, Q_LOAD + 2u), pq_at(r, Q_LOAD + 3u));
        let moment = load.yzw - mv(tip, load.x);
        let chord = far - base;
        for (var c = 0u; c < 3u; c++) {
            set_pq_at(r, Q_LEVER + c, moment[c]);
            set_pq_at(r, Q_LEVER + 3u + c, chord[c]);
        }
        load = load + vec4<f32>(mass, mv(middle, mass));
        if !trunk {
            load = scaled4(load, support(min(base.z, tip.z), radius, length));
        }
    }
    load = load + at_base;
    for (var c = 0u; c < 4u; c++) {
        set_pyf(i, Y_LOAD + c, load[c]);
    }
    for (var c = 0u; c < 3u; c++) {
        set_pyf(i, Y_FAR + c, far[c]);
    }
}
