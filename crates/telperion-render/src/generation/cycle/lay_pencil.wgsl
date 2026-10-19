// fn-256 task 2: a new axis's pencil begun on its frame
// (`grow::draw::pencil`): the pulls it takes from the axis it carries
// on, its base scale and its walk. Part of the layout's shader.

// Axis `i` framed on its parent as `fr`, and its pencil begun.
fn frame(i: u32, fr: Framed) {
    let kind = kind_of(i);
    let made = axf(i, A_B0) * axf(i, A_B1);
    var inherited = 1.0;
    var vigour = made;
    var trunk = true;
    var share = 1.0;
    var reach = 0.0;
    if kind == LATERAL {
        inherited = phf(prow(ax(i, A_PARENT), ax(i, A_NODE)), PH_SCALE);
        trunk = false;
        share = dominance_of(i);
    } else if kind != SEED {
        let p = ax(i, A_PARENT);
        inherited = pnf(p, P_BASE_SCALE);
        vigour = pnf(p, P_GOING) * made;
        trunk = pn(p, P_TRUNK) != 0u;
        // The parent's own pull at its age when `i` was made, over the
        // length it had run.
        let made_at = i32(ax(i, A_BIRTH)) * i32(TICKS) + i32(ax(i, A_SLEEP));
        let born_at = i32(ax(p, A_BIRTH)) * i32(TICKS) + i32(ax(p, A_SLEEP));
        let bent = bend_of(p, years_of(max(made_at - born_at, 0)));
        reach = pnf(p, P_RUN);
        hand_lifts(i, p, blend_of(i), vec2<f32>(bent, reach));
    }
    set_ax3(i, A_BASE, fr.base);
    set_ax3(i, A_HEADING, fr.heading);
    set_ax3(i, A_SIDE, fr.side);
    let wk = Walk(fr.heading, fr.side, fr.base, 0.0, (1.0 + fr.heading.z) * 0.5, fr.curve, tropism_of(i));
    keep_walk(i, wk);
    take_over(i, reach);
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
