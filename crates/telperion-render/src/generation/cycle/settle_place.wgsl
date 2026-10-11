// fn-256 task 2 (SETTLE-PLAN.md S3, S5 and S6): the settled tree laid
// (`geometry::place`), a depth at a time from the seed: each axis framed
// on its laid parent (`framed`), its pull and the pulls it carries on,
// then walked phytomer by phytomer (`Layer::step`), a bounded run of
// them a dispatch, its girth read where it meets the ground and, on the
// sag's lay, its levers. An axis that
// cannot be laid gives its grown index to the least of those that
// failed, as laying them in turn gives the CPU's first.

fn laid_radius(r: u32) -> f32 {
    return phf(r, PH_RADIUS);
}

fn failed(i: u32, code: u32) {
    atomicMin(&atom[T_ERR], ax(i, A_GROWN) * 4u + code);
}

// Each axis of the depth framed on its laid parent, its pull and the
// pulls it carries on, and its walk begun (`Layer::bent`).
@compute @workgroup_size(64)
fn frame_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = listed(id);
    if i == NONE {
        return;
    }
    status = 0u;
    let kind = kind_of(i);
    let fr = framed(i);
    let p = ax(i, A_PARENT);
    let birth = ax(i, A_BIRTH);
    let ticks = i32(max(cfg.age, birth) - birth) * i32(TICKS) - i32(ax(i, A_SLEEP));
    let bent = bend_of(i, years_of(ticks));
    var trunk = kind == SEED;
    if kind == CONTINUED || kind == RELAYED {
        trunk = py(p, Y_TRUNK) != 0u;
        hand_lifts(i, p, blend_of(i), vec2<f32>(pyf(p, Y_BEND), pyf(p, Y_REACH)));
    } else {
        set_pn(i, P_LIFTS_N, 0u);
    }
    set_pyf(i, Y_BEND, bent);
    set_py(i, Y_TRUNK, u32(trunk));
    set_ax3(i, A_BASE, fr.base);
    set_ax3(i, A_HEADING, fr.heading);
    set_ax3(i, A_SIDE, fr.side);
    keep_walk(i, Walk(fr.heading, fr.side, fr.base, 0.0, (1.0 + fr.heading.z) * 0.5, fr.curve, tropism_of(i)));
    let below = fr.base.z <= form[1];
    set_py(i, Y_FAIL, u32(below));
    if below {
        failed(i, 2u);
    }
    if status != 0u {
        atomicOr(&atom[T_PLACE], status);
    }
}

// Each axis of the depth walked on by its phytomers `cfg.walk_at` to
// `cfg.walk_at + cfg.walk_n` (`Layer::step`), its walk carried in its
// columns from the dispatch before: a dispatch's work bounded however
// long the axis. An axis past its end, or stopped, takes none.
@compute @workgroup_size(64)
fn walk_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = listed(id);
    if i == NONE || cfg.walk_at >= nphyt(i) || py(i, Y_FAIL) != 0u {
        return;
    }
    status = 0u;
    var wk = walk_of(i);
    let bent = pyf(i, Y_BEND);
    let reach = pyf(i, Y_REACH);
    let trunk = py(i, Y_TRUNK) != 0u;
    let end = min(cfg.walk_at + cfg.walk_n, nphyt(i));
    for (var j = cfg.walk_at; j < end; j++) {
        let r = prow(i, j);
        step(&wk, i, r, phf(r, PH_SCALE), bent, reach, trunk);
        if (status & (FAULT_BEYOND | FAULT_BELOW)) != 0u {
            failed(i, select(2u, 3u, (status & FAULT_BEYOND) != 0u));
            set_py(i, Y_FAIL, 1u);
            break;
        }
    }
    keep_walk(i, wk);
    let faults = status & ~(FAULT_BEYOND | FAULT_BELOW);
    if faults != 0u {
        atomicOr(&atom[T_PLACE], faults);
    }
}

// Whether any phytomer of the laid tree has size (`Error::Collapsed`
// where none has).
@compute @workgroup_size(64)
fn sized_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let r = item(id);
    if r < cfg.count && phf(r, PH_SCALE) != 0.0 {
        atomicOr(&atom[T_SCALED], 1u);
    }
}
