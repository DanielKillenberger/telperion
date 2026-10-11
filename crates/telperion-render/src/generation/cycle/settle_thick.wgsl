// fn-256 task 2 (SETTLE-PLAN.md S1 and S2): the settled tree scaled
// (`geometry::scale`), a depth at a time from the seed, then each
// relay's attachment on its parent (`girth::attached`), then thickened
// (`girth::thicken`) with each axis's reach (`geometry::reaches`), a
// depth at a time from the deepest, each axis taking its children as the
// CPU's tree does, by index, the last first.

fn listed(id: vec3<u32>) -> u32 {
    let k = id.x + id.y * cfg.row;
    return select(NONE, cfg.base + k, k < cfg.count);
}

// A lateral's share of its vigour among its siblings (`dominance`).
fn dominance_at(i: u32) -> f32 {
    let d = fm(ax(i, A_PA), F_DOMINANCE);
    if d <= 0.0 {
        return 1.0;
    }
    let draw_bits = shr64(draw(child(lineage_of(i), KEY_DOMINANCE)), 29u).x;
    let u = f32(draw_bits) * (1.0 / 16777216.0);
    let held = m(apart(apart(u * u) * u), u);
    return 1.0 - m(d, 1.0 - held);
}

// Each axis's base scale from its parent's, its phytomers scaled by it
// and then by their own size.
@compute @workgroup_size(64)
fn scale_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = listed(id);
    if i == NONE {
        return;
    }
    let kind = kind_of(i);
    let p = ax(i, A_PARENT);
    var inherited = 1.0;
    var share = 1.0;
    if kind == LATERAL {
        inherited = apart(phf(prow(p, ax(i, A_NODE)), PH_GROWN) * pyf(p, Y_BASE));
        share = dominance_at(i);
    } else if kind != SEED {
        inherited = pyf(p, Y_BASE);
    }
    let vigour = single(vec2<u32>(ax(i, A_VIGOUR), ax(i, A_VIGOUR + 1u)));
    let base = apart(apart(inherited * vigour) * share);
    set_pyf(i, Y_BASE, base);
    for (var j = 0u; j < nphyt(i); j++) {
        let r = prow(i, j);
        let scaled = apart(phf(r, PH_SCALE) * base);
        set_ph(r, PH_SCALE, apart(scaled * single(ph64(r, PH_SIZE))));
    }
}

// Each relay's attachment on its parent as the parent's node and the
// share of its span (`attached`): none for any other axis, or a parent
// without nodes.
@compute @workgroup_size(64)
fn attach_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = item(id);
    if i >= cfg.kept {
        return;
    }
    set_py(i, Y_ATTK, NONE);
    if kind_of(i) != RELAYED {
        return;
    }
    let p = ax(i, A_PARENT);
    let n = nphyt(p);
    if n == 0u {
        return;
    }
    status = 0u;
    let x = attach(i, p, axf(i, A_BLEND));
    var k = bitcast<u32>(x.x);
    var t = clamp(x.y, 0.0, 1.0);
    if k > n - 1u {
        k = n - 1u;
        t = 1.0;
    }
    set_py(i, Y_ATTK, k);
    set_pyf(i, Y_ATTT, t);
    if status != 0u {
        atomicOr(&atom[T_PLACE], status);
    }
}

fn ripened(years: f32, ripening: f32) -> f32 {
    if ripening <= 0.0 {
        return 1.0;
    }
    return min(exact_div(years, ripening), 1.0);
}

fn lit(light: f32, chi: f32) -> f32 {
    if chi == 0.0 {
        return 1.0;
    }
    if light <= 0.0 {
        return 0.0;
    }
    return exp2c(m(chi, log2c(light)));
}

fn section_of(r: f32, e: f32) -> f32 {
    if r <= 0.0 {
        return 0.0;
    }
    return exp2c(m(e, log2c(r)));
}

fn radius_of(s: f32, inv: f32) -> f32 {
    if s <= 0.0 {
        return 0.0;
    }
    return exp2c(m(inv, log2c(s)));
}

// `s root + (1 - s) established`: secondary growth's share.
fn weighed(pa: u32, s: f32, established: f32) -> f32 {
    let share = kf(pa, K_SECONDARY);
    return m(share, radius_of(s, kf(pa, K_INV_E))) + m(1.0 - share, established);
}

// The sections axis `f`'s disused pipes keep in its nodes, in grown
// order; their tip's share.
fn disused_in(f: u32, n: u32, e: f32) -> f32 {
    let g = ax(f, A_GROWN);
    var at_tip = 0.0;
    let first = atomicLoad(&atom[T_KCOUNT + g]);
    let end = atomicLoad(&atom[T_KCOUNT + g + 1u]);
    for (var k = first; k < end; k++) {
        let c = wa(W_KIDS, k);
        let on = wa(W_DON, c);
        if on == 0u {
            continue;
        }
        let s = apart(section_of(bitcast<f32>(wa(W_DR, c)), e) * bitcast<f32>(wa(W_DS, c)));
        let node = wa(W_NODE, c);
        if on == 1u && node < n {
            set_pq(prow(f, node), pq(prow(f, node)) + s);
        } else {
            at_tip = at_tip + s;
        }
    }
    return at_tip;
}

// Axis `f`'s phytomers' radii from the pipes they carry, its relays'
// joins, its base radius and its reach.
@compute @workgroup_size(64)
fn thick_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let f = listed(id);
    if f == NONE {
        return;
    }
    let pa = ax(f, A_PA);
    let e = kf(pa, K_EXPONENT);
    let n = nphyt(f);
    for (var k = 0u; k < n; k++) {
        set_pq(prow(f, k), 0.0);
    }
    var at_tip = disused_in(f, n, e);
    var at_base = 0.0;
    var reach = bitcast<f32>(0x80000000u);
    for (var k = 0u; k < n; k++) {
        reach = reach + phf(prow(f, k), PH_SCALE);
    }
    var relays = array<u32, 4>(NONE, NONE, NONE, NONE);
    var held = 0u;
    let first = wf(WF_KSTART, f);
    let kids = wf(WF_KC, f);
    for (var r = 0u; r < kids; r++) {
        let c = wf(WF_KBI, first + r);
        let s = section_of(pyf(c, Y_RADIUS), e);
        let kind = kind_of(c);
        let k = py(c, Y_ATTK);
        if kind == LATERAL && ax(c, A_NODE) < n {
            let at = prow(f, ax(c, A_NODE));
            set_pq(at, pq(at) + s);
        } else if kind == RELAYED && k != NONE {
            let at = apart(s * pyf(c, Y_ATTT));
            set_pq(prow(f, k), pq(prow(f, k)) + at);
            if k > 0u {
                set_pq(prow(f, k - 1u), pq(prow(f, k - 1u)) + (s - at));
            } else {
                at_base = at_base + (s - at);
            }
            if held == 4u {
                atomicOr(&atom[T_PLACE], FAULT_LINE);
            } else {
                relays[held] = c;
                held += 1u;
            }
        } else {
            at_tip = at_tip + s;
        }
        if kind == CONTINUED || kind == RELAYED {
            let b = select(0.0, axf(c, A_BLEND), kind == RELAYED);
            reach = reach + m(1.0 - b, pyf(c, Y_REACH));
        }
    }
    let leaves = (cfg.flags & LEAVES) != 0u;
    let mean = bitcast<f32>(atomicLoad(&atom[T_LEAF + 2u]));
    let chi = kf(pa, K_CHI);
    let ripe = ripened(1.0, kf(pa, K_RIPENING));
    var section = at_tip;
    for (var k = n; k > 0u; k--) {
        let r = prow(f, k - 1u);
        var leaf = 1.0;
        if leaves {
            leaf = apart(lit(single(ph64(r, PH_LIGHT)), chi) * mean);
        }
        let scale = phf(r, PH_SCALE);
        let years = f32(max(cfg.age, ph(r, PH_CYCLE)) - ph(r, PH_CYCLE) + 1u);
        let own = apart(apart(apart(kf(pa, K_PIPE) * scale) * ripened(years, kf(pa, K_RIPENING))) * leaf);
        section = section + (section_of(own, e) + pq(r));
        let established = apart(apart(kf(pa, K_PIPE) * scale) * ripe);
        set_ph(r, PH_RADIUS, weighed(pa, section, established));
        for (var h = 0u; h < held; h++) {
            let c = relays[h];
            if py(c, Y_ATTK) != k - 1u {
                continue;
            }
            let t = pyf(c, Y_ATTT);
            let s = section_of(pyf(c, Y_RADIUS), e);
            let at = apart(s * t);
            set_ax(c, A_JOINED, 1u);
            set_ax(c, A_JOINED + 1u, k - 1u);
            set_ax(c, A_JOINED + 2u, bitcast<u32>(t));
            set_ax(c, A_JOINED + 3u, bitcast<u32>(weighed(pa, section + (s - at), established)));
            set_ax(c, A_JOINED + 4u, bitcast<u32>(weighed(pa, section - at, established)));
        }
    }
    set_pyf(f, Y_RADIUS, radius_of(section + at_base, kf(pa, K_INV_E)));
    set_pyf(f, Y_REACH, reach);
}
