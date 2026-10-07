// A chain's resolution passes (`geometry.wgsl`): its kept axes and their
// phytomers in order, scale, girth, lay, levers. Part of its shader.

// The chain's own kept axes, numbered in order, and their phytomers in
// order along each, by the region's indices; with `all`, numbered among
// every own axis, kept or not, as realized growth placed them.
fn own_axes(na: u32, np: u32, all: bool) -> u32 {
    var n = 0u;
    var phytomers = 0u;
    for (var i = 0u; i < na; i++) {
        let own = i == 1u || (i > 1u && kind_of(i) != LATERAL);
        let kept = own && (ax(i, SHED_FLAGS) & 1u) != 0u;
        ss(i, S_OWN, NONE);
        if kept {
            ss(i, S_OWN, n);
            ss(i, S_START, phytomers);
            ss(i, S_COUNT, 0u);
        }
        if kept || (own && all) {
            phytomers += ax(i, NPHYT);
            n += 1u;
        }
    }
    for (var p = 0u; p < np; p++) {
        let i = ph(p, 3u);
        if s(i, S_OWN) == NONE {
            continue;
        }
        let k = s(i, S_COUNT);
        scratch[sp + (s(i, S_START) + k) * SP + P_ORDER] = p;
        ss(i, S_COUNT, k + 1u);
    }
    return n;
}

// Each own axis's base scale, from its bearer's (`geometry::scale`), and
// its phytomers' scales and ranks; the bearer's phytomer at `bearer`.
fn scale_all(na: u32, bearer: u32) {
    for (var i = 1u; i < na; i++) {
        if s(i, S_OWN) == NONE {
            continue;
        }
        let made = axf(i, MADE);
        var base = 0.0;
        if i == 1u {
            let f = pa_of(ax(i, PA));
            base = prev[bearer + 19u] * made * dominance(f.dominance, lineage_of(i));
        } else {
            let p = ax(i, PARENT);
            base = sf(p, S_SCALE) * axf(p, RUN) * made;
        }
        ssf(i, S_SCALE, base);
        var reach = 0.0;
        for (var k = 0u; k < s(i, S_COUNT); k++) {
            let p = at_k(i, k);
            let at = lo(i, k);
            let scale = bitcast<f32>(ph(p, 6u)) * base;
            out[at + 19u] = scale;
            out[at + 20u] = bitcast<f32>(ph(p, 7u));
            out[at + 21u] = bitcast<f32>(ph(p, 8u));
            reach += scale;
        }
        ssf(i, S_REACH, reach);
        out[(out_ax + s(i, S_OWN)) * AL + 6u] = bitcast<f32>(ax(i, PA));
        out[(out_ax + s(i, S_OWN)) * AL + 7u] = bitcast<f32>(out_ph + s(i, S_START));
    }
    // A relay's reach adds to its parent's as far as it is still the
    // continuation it replaces (`geometry::reaches`).
    for (var i = na; i > 1u; i--) {
        let j = i - 1u;
        if s(j, S_OWN) != NONE && kind_of(j) == RELAYED {
            let p = ax(j, PARENT);
            ssf(p, S_REACH, sf(p, S_REACH) + (1.0 - axf(j, BLEND)) * sf(j, S_REACH));
        }
    }
}

// The chain's radii by the pipe model over its own phytomers and its
// borne buds' committed sections, fitted to its committed base
// (`Bounded::radii`).
fn girth_all(na: u32, np: u32) {
    for (var i = 0u; i < na; i++) {
        ssf(i, S_TIP, 0.0);
    }
    for (var p = 0u; p < np; p++) {
        spf(p, P_NODE, 0.0);
    }
    for (var r = 2u; r < na; r++) {
        if !is_borne(r) {
            continue;
        }
        let j = ax(r, PARENT);
        let node = ax(r, NODE);
        if node < s(j, S_COUNT) {
            let e = pa_of(ax(j, PA)).exponent;
            let q = s(j, S_START) + node;
            spf(q, P_NODE, pf(q, P_NODE) + handed(r, e));
        }
    }
    let age = f32(cfg.age);
    for (var i = na; i > 1u; i--) {
        let j = i - 1u;
        if s(j, S_OWN) == NONE {
            continue;
        }
        let f = pa_of(ax(j, PA));
        var section = sf(j, S_TIP);
        for (var k = s(j, S_COUNT); k > 0u; k--) {
            let at = lo(j, k - 1u);
            let p = at_k(j, k - 1u);
            let scale = out[at + 19u];
            let years = age - min(f32(ph(p, 2u)), age) + 1.0;
            let own = f.pipe * scale * ripe(years, f.ripening);
            section += pow(own, f.exponent) + pf(s(j, S_START) + k - 1u, P_NODE);
            let established = f.pipe * scale * ripe(1.0, f.ripening);
            out[at + 18u] = f.secondary * pow(section, 1.0 / f.exponent) + (1.0 - f.secondary) * established;
        }
        if j == 1u {
            continue;
        }
        let p = ax(j, PARENT);
        let carried = pow(pow(section, 1.0 / f.exponent), pa_of(ax(p, PA)).exponent);
        let node = ax(j, NODE);
        if kind_of(j) == RELAYED && node < s(p, S_COUNT) {
            let b = axf(j, BLEND);
            let q = s(p, S_START) + node;
            spf(q, P_NODE, pf(q, P_NODE) + carried * b);
            ssf(p, S_TIP, sf(p, S_TIP) + carried * (1.0 - b));
        } else {
            ssf(p, S_TIP, sf(p, S_TIP) + carried);
        }
    }
    // The fit to the chain's committed base radius.
    var r0 = 0.0;
    var found = false;
    for (var i = 1u; i < na && !found; i++) {
        if s(i, S_OWN) != NONE && s(i, S_COUNT) > 0u {
            r0 = out[lo(i, 0u) + 18u];
            found = true;
        }
    }
    if !found {
        return;
    }
    let pa = ax(1u, PA);
    let f = pa_of(pa);
    let birth = ax(1u, BIRTH);
    let e = row_at(pa, birth, 7u);
    let k = row_at(pa, birth, 0u);
    let s0 = sf(1u, S_SCALE);
    var pipe = 0.0;
    if k > 0.0 {
        pipe = delta(k, row_at(pa, birth, 1u) * pow(s0, e), row_at(pa, birth, 2u) * pow(s0, 2.0 * e), 1.0 / e) / k;
    }
    let base = f.secondary * pipe + (1.0 - f.secondary) * f.pipe * s0 * ripe(1.0, f.ripening);
    let zero = r0 <= 0.0 || base <= 0.0;
    let l = log(base / r0);
    var beta = 1.0;
    if abs(l) > 1.0 {
        beta = 1.0 / abs(l);
    }
    for (var i = 1u; i < na; i++) {
        if s(i, S_OWN) == NONE {
            continue;
        }
        for (var k = 0u; k < s(i, S_COUNT); k++) {
            let at = lo(i, k) + 18u;
            if zero {
                out[at] = 0.0;
            } else {
                let q = min(out[at] / r0, 1.0);
                out[at] = out[at] * exp(l * pow(q, beta));
            }
        }
    }
}

// How far an axis is pulled towards the vertical at its base
// (`geometry::bend`).
fn bend_of(i: u32, f: Pa) -> f32 {
    let years = f32(cfg.age - min(ax(i, BIRTH), cfg.age)) - f32(ax(i, SLEEP)) * (1.0 / 1048576.0);
    let k = kind_of(i);
    let b = axf(i, BLEND);
    let erected = 1.0 - exp(-f.erection * years);
    if k == CONTINUED {
        return 0.0;
    }
    if k == RELAYED {
        return 1.0 - (1.0 - f.straightening * b) * (1.0 - b * erected);
    }
    return 1.0 - (1.0 - f.straightening) * (1.0 - erected);
}

// The borne buds' levers and the chain's own wood, on the chain as laid
// before it bends (`Bounded::levers`, `sag::gather`).
fn levers_all(na: u32, np: u32, ground: f32) {
    for (var i = 0u; i < na; i++) {
        for (var f = 0u; f < 4u; f++) {
            ssf(i, S_LOAD + f, 0.0);
        }
    }
    for (var p = 0u; p < np; p++) {
        for (var f = 0u; f < 4u; f++) {
            spf(p, P_LOAD + f, 0.0);
        }
    }
    let age = f32(cfg.age);
    // Each axis's far tip: its own, or that of the continuation carrying
    // it on.
    for (var i = 1u; i < na; i++) {
        if s(i, S_OWN) == NONE {
            continue;
        }
        let n = s(i, S_COUNT);
        var far = s3(i, S_FRAME);
        if n > 0u {
            far = o3(lo(i, n - 1u) + 9u);
        }
        ss3(i, S_FAR, far);
    }
    for (var i = na; i > 2u; i--) {
        let j = i - 1u;
        if s(j, S_OWN) != NONE && kind_of(j) == CONTINUED {
            ss3(ax(j, PARENT), S_FAR, s3(j, S_FAR));
        }
    }
    for (var r = 2u; r < na; r++) {
        if !is_borne(r) {
            continue;
        }
        let j = ax(r, PARENT);
        let node = ax(r, NODE);
        if node >= s(j, S_COUNT) {
            continue;
        }
        let frame = bud_frame(r, j, node);
        let s0 = borne_scale(r);
        let pa = ax(r, PA);
        let birth = ax(r, BIRTH);
        let mass = row_at(pa, birth, 5u) * s0 * s0 * s0;
        let along = row_at(pa, birth, 6u) * s0 * s0 * s0 * s0;
        let m = frame[0] * mass + frame[1] * along;
        let q = s(j, S_START) + node;
        spf(q, P_LOAD, pf(q, P_LOAD) + mass);
        for (var c = 0u; c < 3u; c++) {
            spf(q, P_LOAD + 1u + c, pf(q, P_LOAD + 1u + c) + m[c]);
        }
    }
    for (var i = na; i > 1u; i--) {
        let j = i - 1u;
        if s(j, S_OWN) == NONE {
            continue;
        }
        let n = s(j, S_COUNT);
        let base0 = s3(j, S_FRAME);
        let f = pa_of(ax(j, PA));
        var mass = sf(j, S_LOAD);
        var moment = s3(j, S_LOAD + 1u);
        let far = s3(j, S_FAR);
        for (var k = n; k > 0u; k--) {
            let at = lo(j, k - 1u);
            let tip = o3(at + 9u);
            var base = base0;
            if k > 1u {
                base = o3(lo(j, k - 2u) + 9u);
            }
            let len = length(tip - base);
            let p = at_k(j, k - 1u);
            let years = age - min(f32(ph(p, 2u)), age) + 1.0;
            let leaves = f.pipe * out[at + 19u] * ripe(years, f.ripening);
            let radius = out[at + 18u];
            let own = len * (radius * radius + leaves * leaves);
            let q = s(j, S_START) + k - 1u;
            mass += pf(q, P_LOAD);
            moment += vec3<f32>(pf(q, P_LOAD + 1u), pf(q, P_LOAD + 2u), pf(q, P_LOAD + 3u));
            let lever = moment - tip * mass;
            let chord = far - base;
            for (var c = 0u; c < 3u; c++) {
                spf(q, P_LEVER + c, lever[c]);
                spf(q, P_LEVER + 3u + c, chord[c]);
            }
            mass += own;
            moment += (base + tip) * 0.5 * own;
            let held_share = support(min(base.z, tip.z) + ground, radius, len);
            mass *= held_share;
            moment *= held_share;
        }
        if j == 1u {
            continue;
        }
        let p = ax(j, PARENT);
        var w_node = 0.0;
        var w_tip = 1.0;
        let node = ax(j, NODE);
        if kind_of(j) == RELAYED && node < s(p, S_COUNT) {
            w_node = axf(j, BLEND);
            w_tip = 1.0 - w_node;
        }
        if w_node > 0.0 {
            let q = s(p, S_START) + node;
            spf(q, P_LOAD, pf(q, P_LOAD) + mass * w_node);
            for (var c = 0u; c < 3u; c++) {
                spf(q, P_LOAD + 1u + c, pf(q, P_LOAD + 1u + c) + moment[c] * w_node);
            }
        }
        ssf(p, S_LOAD, sf(p, S_LOAD) + mass * w_tip);
        ss3(p, S_LOAD + 1u, s3(p, S_LOAD + 1u) + moment * w_tip);
    }
}
