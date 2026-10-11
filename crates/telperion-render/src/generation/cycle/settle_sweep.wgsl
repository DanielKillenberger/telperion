// fn-256 task 2 (SETTLE-PLAN.md G3 to G5): the grown tree's sweeps, a
// dispatch a depth over its list. Down: each axis's identity
// (`canonical::identity`) and, where any PA keeps the tree as grown, its
// base scale (`bases`). Up, deepest first: its base radius as grown
// (`attachments_with`) and its subtree's living carried up into its fade
// (`shed::fades`). Down again: whether it is kept (`shed_past`), the
// share its bearers' fades leave it (`above`), the pipe it leaves its
// bearer (`disused`), and the kept tree's counts.

// The axis this invocation takes from its depth's list, none past it.
fn listed(id: vec3<u32>) -> u32 {
    let k = id.x + id.y * cfg.row;
    if k >= cfg.count {
        return NONE;
    }
    return wa(W_DLIST, cfg.base + k);
}

fn id_of(i: u32) -> vec2<u32> {
    return vec2<u32>(wa(W_ID, i), wa(W_ID + 1u, i));
}

fn identity(i: u32) -> vec2<u32> {
    let kind = kind_of(i);
    let pa = vec2<u32>(ax(i, A_PA), 0u);
    if kind == SEED {
        return child(mix(vec2<u32>(0x7f4a7c15u, 0x9e3779b9u)), pa);
    }
    let up = id_of(ax(i, A_PARENT));
    if kind == LATERAL {
        let origin = ax(i, A_ORIGIN);
        let at = child(child(up, w(1u, 0u)), w(ax(i, A_NODE), 0u));
        return child(child(at, w((origin >> 8u) & 0xffu, 0u)), w((origin >> 16u) & 1u, 0u));
    }
    return child(child(up, w(kind, 0u)), pa);
}

// A lateral's share of its vigour among its siblings (`dominance`).
fn dominance(i: u32) -> f32 {
    let pa = ax(i, A_PA);
    if (kpa(pa, K_FLAGS) & KF_DOMINANCE) == 0u {
        return 1.0;
    }
    let d = shr64(draw(child(ax64(i, A_LINEAGE), KEY_DOMINANCE)), 29u).x;
    let u = f32(d) * (1.0 / 16777216.0);
    let held = m(apart(apart(u * u) * u), u);
    return 1.0 - m(kpaf(pa, K_DOMINANCE), 1.0 - held);
}

// The assigned scale of axis `a`'s phytomer `j`.
fn scale_at(a: u32, j: u32) -> f32 {
    return bitcast<f32>(wp(WP_SCALE, wa(W_PSTART, a) + j));
}

@compute @workgroup_size(64)
fn down_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = listed(id);
    if i == NONE {
        return;
    }
    let key = identity(i);
    set_wa(W_ID, i, key.x);
    set_wa(W_ID + 1u, i, key.y);
    if (cfg.flags & KEEPS) == 0u {
        return;
    }
    let kind = kind_of(i);
    let p = ax(i, A_PARENT);
    var inherited = 1.0;
    var share = 1.0;
    if kind == LATERAL {
        inherited = apart(scale_at(p, ax(i, A_NODE)) * waf(W_BASE, p));
        share = dominance(i);
    } else if kind != SEED {
        inherited = waf(W_BASE, p);
    }
    set_waf(W_BASE, i, apart(apart(inherited * waf(W_VIG, i)) * share));
}

// A phytomer's shade, size (0) or light (1), as the f32 the girth takes
// it at, from its unit's record.
fn shade_of(r: u32, k: u32) -> f32 {
    return single(vec2<u32>(unit(r, U_SHADE + 2u * k), unit(r, U_SHADE + 2u * k + 1u)));
}

// `ripened`: years over the ripening, at most one; one where none ripens.
fn ripened(years: f32, ripening: f32) -> f32 {
    if ripening <= 0.0 {
        return 1.0;
    }
    return min(exact_div(years, ripening), 1.0);
}

// A phytomer's own pipe grown in `cycle` (`own_at`).
fn own_pipe(pa: u32, cycle: u32, scale: f32, leaf: f32) -> f32 {
    let years = f32(max(cfg.age, cycle) - cycle + 1u);
    let ripe = ripened(years, kpaf(pa, K_RIPENING));
    return apart(apart(apart(kpaf(pa, K_PIPE) * scale) * ripe) * leaf);
}

// A phytomer's light term, light^χ (`lit`).
fn lit(light: f32, chi: f32) -> f32 {
    if chi == 0.0 {
        return 1.0;
    }
    if light <= 0.0 {
        return 0.0;
    }
    return exp2c(m(chi, log2c(light)));
}

// A radius's section at exponent `e`, and back by `1/e` (`section`,
// `radius`).
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

// A grown phytomer's own pipe at its axis's base scale `base`, by `leaf`.
fn grown_pipe(pa: u32, g: u32, base: f32, leaf: f32) -> f32 {
    let r = wp(WP_UNIT, g);
    let scale = apart(apart(bitcast<f32>(wp(WP_SCALE, g)) * base) * shade_of(r, 0u));
    return own_pipe(pa, phyt(wp(WP_ROW, g), P_CYCLE), scale, leaf);
}

// Axis `i`'s base radius as grown: its own pipes and those its children
// carry into its nodes and tip, the last child first.
fn radius_grown(i: u32) {
    let pa = ax(i, A_PA);
    let e = kpaf(pa, K_EXPONENT);
    let n = ax(i, A_PHYTS);
    let ps = wa(W_PSTART, i);
    for (var k = 0u; k < n; k++) {
        set_wp(WP_AT, ps + k, 0u);
    }
    var at_tip = 0.0;
    let kids = kids_of(i);
    for (var k = kids.y; k > kids.x; k--) {
        let c = kid(k - 1u);
        let s = section_of(waf(W_RADIUS, c), e);
        let node = ax(c, A_NODE);
        if kind_of(c) == LATERAL && node < n {
            set_wp(WP_AT, ps + node, bitcast<u32>(bitcast<f32>(wp(WP_AT, ps + node)) + s));
        } else {
            at_tip = at_tip + s;
        }
    }
    let leaves = (cfg.flags & LEAVES) != 0u;
    let mean = bitcast<f32>(atomicLoad(&atom[T_LEAF + 2u]));
    let chi = kpaf(pa, K_CHI);
    let base = waf(W_BASE, i);
    var section = at_tip;
    for (var k = n; k > 0u; k--) {
        let g = ps + k - 1u;
        var leaf = 1.0;
        if leaves {
            leaf = apart(lit(shade_of(wp(WP_UNIT, g), 1u), chi) * mean);
        }
        let pipe = grown_pipe(pa, g, base, leaf);
        section = section + (section_of(pipe, e) + bitcast<f32>(wp(WP_AT, g)));
    }
    set_waf(W_RADIUS, i, radius_of(section, kpaf(pa, K_INV_E)));
}

// Axis `i`'s subtree's living, from its own and what each child carries
// up (`carry`), and its fade where it sheds (`faded`).
fn fade_grown(i: u32) {
    var lived = waf(W_LIVED, i);
    var alive = waf(W_SALIVE, i);
    let birth = ax(i, A_BIRTH);
    let units_n = ax(i, A_UNITS);
    let kids = kids_of(i);
    for (var k = kids.x; k < kids.y; k++) {
        let c = kid(k);
        var weight = qf(waf(W_VIG, c));
        if kind_of(c) == LATERAL {
            weight = m(weight, qf(scale_at(i, ax(c, A_NODE))));
        }
        let span = ax(c, A_BIRTH) - min(birth, ax(c, A_BIRTH));
        let units = min(span, units_n);
        var before = 0.0;
        if units != 0u {
            before = tick_q(ax(i, A_SLEEP)) + bitcast<f32>(wu(WU_PSUM, wa(W_USTART, i) + units - 1u));
        }
        let carried = before + m(weight, f32(span - units) + waf(W_LIVED, c));
        lived = max(lived, carried);
        alive = max(alive, m(waf(W_SALIVE, c), weight));
    }
    set_waf(W_LIVED, i, lived);
    set_waf(W_SALIVE, i, alive);
    let kind = kind_of(i);
    var faded = 1.0;
    let pa = ax(i, A_PA);
    if (kind == LATERAL || kind == RELAYED) && (kpa(pa, K_FLAGS) & KF_SHEDS) != 0u {
        let idle = f32(i32(cfg.age) - i32(birth)) - lived;
        faded = clamp(kpaf(pa, K_DELAY) - idle, min(alive, 1.0), 1.0);
    }
    set_waf(W_FADE, i, faded);
}

@compute @workgroup_size(64)
fn up_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = listed(id);
    if i == NONE {
        return;
    }
    if (cfg.flags & KEEPS) != 0u {
        radius_grown(i);
    }
    if (cfg.flags & SHEDS) != 0u {
        fade_grown(i);
    } else {
        set_waf(W_FADE, i, 1.0);
    }
}

@compute @workgroup_size(64)
fn keep_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = listed(id);
    if i == NONE {
        return;
    }
    let p = ax(i, A_PARENT);
    let kind = kind_of(i);
    let fade = waf(W_FADE, i);
    var kept = true;
    var above = 1.0;
    if p != NONE {
        kept = wa(W_KEPT, p) != 0u;
        if kind == LATERAL || kind == RELAYED {
            kept = kept && fade > 0.0;
        }
        above = apart(waf(W_ABOVE, p) * waf(W_FADE, p));
    }
    set_wa(W_KEPT, i, u32(kept));
    set_waf(W_ABOVE, i, above);
    if kept {
        atomicAdd(&atom[T_KDCOUNT + cfg.depth], 1u);
        atomicAdd(&atom[T_KEPT], 1u);
        atomicAdd(&atom[T_KEPT_PHYTS], ax(i, A_PHYTS));
        atomicAdd(&atom[T_KEPT_UNITS], ax(i, A_UNITS));
    }
    let pa = ax(i, A_PA);
    let sheddable = kind == LATERAL || kind == RELAYED;
    let retains = (kpa(pa, K_FLAGS) & KF_RETAINS) != 0u;
    if (cfg.flags & KEEPS) == 0u || p == NONE || !sheddable || !retains || wa(W_KEPT, p) == 0u {
        return;
    }
    var taken = 1.0;
    if kept {
        taken = 1.0 - fade;
    }
    if taken > 0.0 {
        // 1 at a lateral's node, 2 at the bearer's tip.
        set_wa(W_DON, i, select(2u, 1u, kind == LATERAL));
        set_waf(W_DR, i, apart(waf(W_RADIUS, i) * above));
        set_waf(W_DS, i, apart(kpaf(pa, K_RETAINED) * taken));
    }
}

// The leaf term's two sums over the tree as grown (`leaf_partials`):
// each phytomer's pipe and its pipe by its light term, in the order the
// axes and their phytomers stand, summed in turn from the pair the last
// dispatch left. The lanes take the terms of a tile of 256 at once and
// one lane adds them; then the mean (`grown_mean`).
var<workgroup> w_pipe: array<f32, 256>;
var<workgroup> w_term: array<f32, 256>;

@compute @workgroup_size(256)
fn leaf_main(@builtin(local_invocation_index) lane: u32) {
    var total = bitcast<f32>(atomicLoad(&atom[T_LEAF]));
    var weighted = bitcast<f32>(atomicLoad(&atom[T_LEAF + 1u]));
    for (var t = 0u; t < cfg.count; t += 256u) {
        var pipe = 0.0;
        var term = 0.0;
        if t + lane < cfg.count {
            let g = cfg.base + t + lane;
            let i = phyt(wp(WP_ROW, g), P_AXIS);
            let pa = ax(i, A_PA);
            pipe = grown_pipe(pa, g, waf(W_BASE, i), 1.0);
            term = apart(pipe * lit(shade_of(wp(WP_UNIT, g), 1u), kpaf(pa, K_CHI)));
        }
        w_pipe[lane] = pipe;
        w_term[lane] = term;
        workgroupBarrier();
        if lane == 0u {
            for (var k = 0u; k < min(256u, cfg.count - t); k++) {
                total = apart(total + w_pipe[k]);
                weighted = apart(weighted + w_term[k]);
            }
        }
        workgroupBarrier();
    }
    if lane == 0u {
        atomicStore(&atom[T_LEAF], bitcast<u32>(total));
        atomicStore(&atom[T_LEAF + 1u], bitcast<u32>(weighted));
        var mean = 1.0;
        if weighted > 0.0 {
            mean = exact_div(total, weighted);
        }
        atomicStore(&atom[T_LEAF + 2u], bitcast<u32>(mean));
    }
}
