// Sleeping buds on the GPU chain grower (fn-209 F1, task 2): a node's
// sleeping buds, their waking as buds the chain bears, and a woken bud's
// chain's beginning (`dormant.rs`, `wake.rs`). Part of `chain.wgsl`'s
// shader.

// The sleeping buds of a node's places that wake within the tree's age
// (`dormant::draw`, D3 and D11), by the cycle they wake in.
fn sleep(zat: u32, node_key: vec2<u32>, axis: u32, node: u32, cycle: u32) {
    let waking = table[zat + 10u];
    if waking == NONE {
        return;
    }
    let zg = zone_g(zat);
    let buds_f = gf(zg);
    let npa = table[zat + 9u];
    for (var s = 0u; s < table[zat + 4u]; s++) {
        let key = child(child(node_key, w(s, 0u)), DORMANT);
        let pa = drawn_pa(table[zat + 8u], table[zat + 9u], key);
        if pa == 0xffu {
            continue;
        }
        let years = wake_q(words[waking], words[waking + 1u], draw(child(key, DORMANT)));
        if !lt64(years, shl64(w(cfg.age - cycle, 0u), 20u)) {
            continue;
        }
        if n_sleepers >= cfg.sleep_cap {
            overflow |= 32u;
            return;
        }
        let q = sleep_base + n_sleepers * SW;
        let whole = shr64(years, 20u).x;
        sleepers[q] = cycle + 1u + whole;
        sleepers[q + 1u] = axis;
        sleepers[q + 2u] = node;
        sleepers[q + 3u] = s;
        sleepers[q + 4u] = pa;
        sleepers[q + 5u] = key.x;
        sleepers[q + 6u] = key.y;
        sleepers[q + 7u] = years.x & (TICKS - 1u);
        sleepers[q + 8u] = whole;
        // Its unit, its draw's presence by its place's share, and its
        // node's buds a whorl (`dormant::draw`, `Shoot::woke`).
        sleepers[q + 9u] = rd(axis, NUNITS) - 1u;
        let lead = run_lead(gu(zg + 2u), pa, draw(key));
        let wood_s = gf(gu(zg + 3u) + cycle * npa + pa);
        let placed = min(buds_f - f32(s), 1.0);
        sleepers[q + 10u] = bitcast<u32>(decided(lead, wood_s, 1.0) * placed);
        sleepers[q + 11u] = bitcast<u32>(buds_f);
        n_sleepers += 1u;
    }
}

// The stage a bud of `pa` that slept `slept` ticks wakes in, and its time
// there; none where it slept past its last (`dormant::aged`).
fn aged(pa_in: u32, slept_in: vec2<u32>) -> vec4<u32> {
    var pa = lived(pa_in);
    var slept = slept_in;
    // The share of it reaching each stage (f32).
    var reach = pa_f(pa_in, 0u);
    // An age moves on only to an older one, so the stages run out.
    while pa != NONE {
        let life = row(pa, 0u);
        if lt64(slept, life) {
            return vec4<u32>(pa, slept.x, slept.y, bitcast<u32>(reach));
        }
        slept = sub64(slept, life);
        reach *= pa_f(pa, 1u);
        pa = successor(pa);
    }
    return vec4<u32>(NONE);
}

// The sleeping buds that wake in `cycle`: a bud on every axis that
// carries their bearer on (`borne_wakes`, `dormant::carried`).
fn wakes(cycle: u32) {
    for (var k = 0u; k < n_sleepers; k++) {
        let q = sleep_base + k * SW;
        if sleepers[q] != cycle {
            continue;
        }
        let slept = add64(shl64(w(sleepers[q + 8u], 0u), 20u), w(sleepers[q + 7u], 0u));
        let stage = aged(sleepers[q + 4u], slept);
        if stage.x == NONE {
            continue;
        }
        var open: array<u32, 32>;
        var carried: array<f32, 32>;
        var since: array<u32, 32>;
        var top = 1u;
        open[0] = sleepers[q + 1u];
        // What carried its bearer from its node's unit on
        // (`dormant::carried`).
        let u0 = unit_at(open[0], sleepers[q + 9u]);
        carried[0] = select(1.0, unit_f(u0, 3u), u0 != NONE);
        since[0] = sleepers[q + 9u] + 1u;
        while top > 0u {
            top -= 1u;
            let a = open[top];
            let here = carried[top] * units_from(a, since[top]);
            if rd(a, SUCC) == NONE {
                let end = rd(a, APEX_END);
                let ended_here = end == cycle + 1u && (rd(a, KIND) & HAS_OUTLIVED) != 0u;
                if end == 0u || ended_here {
                    if n_pends >= cfg.pend_cap {
                        overflow |= 64u;
                        return;
                    }
                    let p = pend_base + n_pends * QW;
                    sleepers[p] = sleepers[q + 5u];
                    sleepers[p + 1u] = sleepers[q + 6u];
                    sleepers[p + 2u] = sleepers[q + 4u];
                    sleepers[p + 3u] = sleepers[q + 1u];
                    sleepers[p + 4u] = sleepers[q + 2u];
                    sleepers[p + 5u] = sleepers[q + 3u];
                    sleepers[p + 6u] = cycle;
                    sleepers[p + 7u] = a;
                    // What its own chain's waking reads.
                    sleepers[p + 8u] = sleepers[q + 7u];
                    sleepers[p + 9u] = stage.x;
                    sleepers[p + 10u] = stage.y;
                    sleepers[p + 11u] = stage.z;
                    sleepers[p + 12u] = bitcast<u32>(here);
                    sleepers[p + 13u] = stage.w;
                    sleepers[p + 14u] = rd(a, NUNITS);
                    sleepers[p + 15u] = sleepers[q + 10u];
                    sleepers[p + 16u] = sleepers[q + 11u];
                    n_pends += 1u;
                }
                continue;
            }
            if top + 2u > 32u {
                overflow |= 128u;
                return;
            }
            if rd(a, SUCC + 1u) != NONE {
                let n = rd(a, SUCC + 1u);
                open[top] = n;
                carried[top] = here * rdf(n, MADE);
                since[top] = 0u;
                top += 1u;
            }
            let n = rd(a, SUCC);
            open[top] = n;
            carried[top] = here * rdf(n, MADE);
            since[top] = 0u;
            top += 1u;
        }
    }
}

// A woken bud's chain begins (`Shoot::woke`): the stages it slept
// through, axes of no length, then its own first unit in its waking
// cycle, which its sleep shortens.
fn woke(b: u32, vigour: f32, whorl: f32) -> u32 {
    let key = buds[b + 6u];
    let spa = buds[b + 7u].x;
    let pa = buds[b + 7u].y;
    let spent = buds[b + 8u];
    let sleep = buds[b + 9u].x;
    let cycle = buds[b + 9u].y;
    let node = buds[b + 2u].y;
    let slot = buds[b + 5u].y;
    var kind = LATERAL | IS_WOKEN | (slot << 8u);
    var parent = 0u;
    var lineage = key;
    var stage = lived(spa);
    while stage != pa && stage != NONE {
        if kind != CONTINUED {
            lineage = onto(key, stage);
        }
        let h = horizon_of(stage, kind & 0xffu, parent);
        let at = push_axis(lineage, stage, cycle - 1u, kind, parent, select(0u, node, kind != CONTINUED), key, h, row(stage, 0u));
        if at == NONE {
            return cycle + 1u;
        }
        if kind != CONTINUED {
            wrf(at, MADE, vigour);
            wrf(at, WHORL, whorl);
        }
        wr(at, APEX_END, cycle);
        stage = successor(stage);
        lineage = onto(key, stage);
        kind = CONTINUED;
        parent = at;
        add_successor(at, at + 1u);
    }
    if kind != CONTINUED {
        lineage = onto(key, pa);
    }
    let h = horizon_of(pa, kind & 0xffu, parent);
    let a = push_axis(lineage, pa, cycle - 1u, kind, parent, select(0u, node, kind != CONTINUED), key, h, vec2<u32>(0u));
    if a == NONE {
        return cycle + 1u;
    }
    if kind != CONTINUED {
        wrf(a, MADE, vigour);
        wrf(a, WHORL, whorl);
    }
    wr(a, SLEEP, sleep);
    wr64(a, AGED, spent);
    wr64(a, UNITS, spent);
    run(Apex(a, spent), cycle);
    return cycle + 1u;
}

// Whether a sleeping bud still wakes in `cycle` or after.
fn asleep_from(cycle: u32) -> bool {
    for (var k = 0u; k < n_sleepers; k++) {
        if sleepers[sleep_base + k * SW] >= cycle {
            return true;
        }
    }
    return false;
}

// How much of its size a bud that woke keeps (`Woken::kept`), from its
// pending record at `p`.
fn woken_kept(p: u32) -> f32 {
    let a = sleepers[p + 7u];
    let cycle = sleepers[p + 6u];
    let share = 1.0 - f32(sleepers[p + 8u]) * (1.0 / 1048576.0);
    let end = rd(a, APEX_END);
    let succ = rd(a, SUCC) != NONE;
    if (rd(a, KIND) & HAS_OUTLIVED) != 0u && end == cycle + 1u && !succ {
        let used = f32(rd(a, OUTLIVED)) * (1.0 / 1048576.0);
        return max(used + share - 1.0, 0.0);
    }
    if cycle >= cfg.age {
        let l = rdf(a, LIVING);
        let on = select(1.0, l, l >= 0.0);
        return share + (1.0 - share) * on;
    }
    var on = 0.0;
    if succ {
        on += rdf(rd(a, SUCC), MADE);
        if rd(a, SUCC + 1u) != NONE {
            on += rdf(rd(a, SUCC + 1u), MADE);
        }
    }
    let u = unit_at(a, sleepers[p + 14u]);
    if u == NONE {
        return share + (1.0 - share) * on;
    }
    let stopped = end == cycle + 2u && (rd(a, KIND) & IS_FAILED) == 0u;
    let q = unit_f(u, 2u) * unit_f(u, 3u) * select(1.0, on, stopped);
    return share + (1.0 - share) * q;
}

// How much the apex of `axis`, still living at the age having spent
// `spent` of its PA, lives on (`Grower::living`).
fn living(axis: u32, spent: vec2<u32>) {
    let pa = rd(axis, PA);
    let left = clamp(pa_f(pa, 5u) - cycles_of(spent), 0.0, 1.0);
    let go = pa_f(pa, 1u);
    wrf(axis, LIVING, left + (1.0 - left) * (go + (1.0 - go) * pa_f(pa, 3u)));
}
