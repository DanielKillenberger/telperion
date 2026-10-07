// A living apex on the GPU chain grower (fn-209 F1, task 2): its growth
// unit (`grow_unit`), its advance, stop, move and relay (`advance.rs`,
// `stop.rs`), and each relay's node (D18). Part of `chain.wgsl`'s shader.

// A growth unit's phytomers and the laterals they bear (`grow_unit`).
fn grow_unit(axis: u32, pa: u32, unit: vec2<u32>, cycle: u32, ticks: u32) {
    let first = table[2u * pa];
    let zones = table[2u * pa + 1u];
    for (var k = 0u; k < zones; k++) {
        let zat = first + ZW * k;
        let zk = zone_key(unit, zat);
        let d = draw(zk);
        let n = zone_count(zat, d);
        let side = zone_side(zat, d, n);
        if n > MOST {
            overflow |= 8u;
            return;
        }
        var drawn: array<u32, MOST>;
        sorted(zk, n, &drawn);
        let window = words[table[zat + 7u] + cycle];
        let zg = zone_g(zat);
        let buds_f = gf(zg);
        for (var m = 0u; m < n; m++) {
            let j = drawn[m];
            let node_key = child(zk, w(j, 0u));
            if n_phyts >= cfg.phyt_cap {
                overflow |= 2u;
                return;
            }
            let p = phyt_base + n_phyts * PW;
            let node_p = presence(node_lead(zat, d, side, j), window);
            phyts[p] = node_key.x;
            phyts[p + 1u] = node_key.y;
            phyts[p + 2u] = cycle;
            phyts[p + 3u] = axis;
            phyts[p + 4u] = node_p.x;
            phyts[p + 5u] = node_p.y;
            // Its rank by presence (`presence::assign_one`).
            let rank = vec2<u32>(rd(axis, RANKF), rd(axis, RANK_HI));
            phyts[p + PRANK] = rank.x;
            phyts[p + PRANK + 1u] = rank.y;
            let next = add64(rank, mul64(node_p, w(ticks, 0u)));
            wr(axis, RANKF, next.x);
            wr(axis, RANK_HI, next.y);
            n_phyts += 1u;
            let node = rd(axis, NPHYT);
            wr(axis, NPHYT, node + 1u);
            for (var s = 0u; s < table[zat + 4u]; s++) {
                let place = child(node_key, w(s, 0u));
                let drew = bud(zat, place);
                if drew == 0xffu || lived(drew) == NONE {
                    continue;
                }
                let lpa = lived(drew);
                if shed_certain(lpa, cycle, axis) {
                    continue;
                }
                let h = horizon_of(lpa, LATERAL, axis);
                let at = push_axis(onto(place, lpa), lpa, cycle, LATERAL | (s << 8u), axis, node, place, h, vec2<u32>(0u));
                if at != NONE {
                    // Its node's buds a whorl, and its draws' presences as
                    // made, Q32, at the chain's end (`assign.wgsl`).
                    wrf(at, WHORL, buds_f);
                    ask(at, vec4<u32>(LATERAL_ASKS, zat, drew, place.x), vec4<u32>(place.y, 0u, 0u, 0u));
                }
            }
            if words[table[zat + 11u] + cycle].x != 0u {
                sleep(zat, node_key, axis, node, cycle);
            }
        }
    }
}

fn carry_on(cycle: u32, used: vec2<u32>) -> Next {
    if !lt64(used, ticks_whole()) || n_next == 0u {
        return none();
    }
    n_next -= 1u;
    let e = live[live_base + next_half * cfg.live_cap + n_next];
    wr(e.x, BIRTH, cycle - 1u);
    wr(e.x, SLEEP, used.x);
    return Next(true, Apex(e.x, e.yz));
}

// The relay of the apex that stopped having spent `spent` (`relay_bud`);
// one made `whole` grows a whole first unit.
fn relay_bud(apex: Apex, cycle: u32, used: vec2<u32>, spent: vec2<u32>, why: u32, whole: bool, below: u32, blend: f32, asks: array<vec4<u32>, 2>) -> Next {
    let a = apex.axis;
    let pa = rd(a, PA);
    let grown = add64(add64(rd64(a, AGED), rd64(a, SHARES)), sub64(spent, apex.spent));
    let ended_relays = rd(a, ENDED_RELAYS) + select(0u, 1u, why == ENDS);
    let units = rd64(a, UNITS);
    let h = horizon_of(pa, RELAYED, a);
    let r = push_axis(rd64(a, LINEAGE), pa, cycle, RELAYED, a, 0u, rd64(a, ROOT), h, units);
    if r == NONE {
        return none();
    }
    add_successor(a, r);
    made_axis(r, blend);
    ask(r, asks[0], asks[1]);
    wr64(r, AGED, grown);
    // A failed unit's relay grows its rest where it grows in the same
    // cycle, and on from the next as the apex would have where the unit
    // lived it whole.
    let rest = why == FAILED && lt64(used, ticks_whole());
    wr(r, KIND, RELAYED | select(0u, IS_ENDED, whole) | select(0u, IS_REST, rest));
    wr(r, ENDED_RELAYS, ended_relays);
    wr(r, BELOW, select(0u, below, rest));
    push_next(r, units);
    return carry_on(cycle, used);
}

// The apex has stopped in the unit keyed `key` (`stop`).
fn stop(apex: Apex, cycle: u32, used: vec2<u32>, key: vec2<u32>, spent: vec2<u32>, why: u32, stopped: f32, bound: vec2<u32>) -> Next {
    let a = apex.axis;
    wr64(a, UNITS, spent);
    if why == ENDS {
        wr(a, KIND, rd(a, KIND) | HAS_OUTLIVED);
        wr(a, OUTLIVED, used.x);
    }
    let pa = rd(a, PA);
    let t = row(pa, 6u + why);
    if is_zero(t) {
        return none();
    }
    let d = draw(child(key, RELAY));
    if lt64(d, t) {
        // It stands between the axis's line and its own bud by the
        // stop's lead over the widest window (`stop`); its presences,
        // Q32, at the chain's end, a failed unit's past its survival's
        // bound.
        let blend = clamp(stopped / SPAN_F, 0.0, 1.0);
        let asks = array<vec4<u32>, 2>(vec4<u32>(RELAY_ASKS, why, key.x, key.y), vec4<u32>(cycle, used.x, bound.x, bound.y));
        return relay_bud(apex, cycle, used, spent, why, why == ENDS, 0u, blend, asks);
    }
    return none();
}

// The apex has spent its age (`move_on`).
fn move_on(apex: Apex, key: vec2<u32>, cycle: u32, used: vec2<u32>) -> Next {
    let a = apex.axis;
    let pa = rd(a, PA);
    let next = successor(pa);
    if next == NONE {
        return stop(apex, cycle, used, key, apex.spent, ENDS, HUGE, vec2<u32>(0u));
    }
    let d = draw(child(key, w(MOVE, 0u)));
    if !lt64(d, row(pa, 9u)) {
        let stopped = above_t(d, row(pa, 9u));
        return stop(apex, cycle, used, key, apex.spent, ENDS, stopped, vec2<u32>(0u));
    }
    let woods = array<vec2<u32>, 2>(log_wood(next, cycle), log_wood(next, cycle + 1u));
    let span = w(cfg.span_lo, cfg.span_hi);
    let made = moved_q(row(pa, 10u), d, woods[0], woods[1], used, row(pa, 11u), row(pa, 12u), span);
    let h = horizon_of(next, CONTINUED, a);
    let c = push_axis(onto(rd64(a, ROOT), next), next, cycle, CONTINUED, a, 0u, rd64(a, ROOT), h, vec2<u32>(0u));
    if c == NONE {
        return none();
    }
    add_successor(a, c);
    made_axis(c, 1.0);
    wq(c, B0, made);
    push_next(c, vec2<u32>(0u));
    let tail = carry_on(cycle, used);
    let after = After(apex, used, made, key);
    if tail.go {
        if depth >= STACK {
            overflow |= 16u;
            return none();
        }
        stack[depth] = after;
        depth += 1u;
        return tail;
    }
    return relay_after(after, cycle);
}

// An apex that all but stopped also grows the relay it would have had,
// where its relay draw makes one (D9, D10).
fn relay_after(f: After, cycle: u32) -> Next {
    let pa = rd(f.apex.axis, PA);
    let t = row(pa, 7u);
    if lt64(f.made, one32()) && !is_zero(t) && lt64(draw(child(f.key, RELAY)), t) {
        wr64(f.apex.axis, UNITS, f.apex.spent);
        let asks = array<vec4<u32>, 2>(vec4<u32>(AFTER_ASKS, f.key.x, f.key.y, cycle), vec4<u32>(f.used.x, f.made.x, f.made.y, 0u));
        return relay_bud(f.apex, cycle, f.used, f.apex.spent, ENDS, true, 0u, 0.0, asks);
    }
    return none();
}

// The apex's unit keyed `keys.y` failed on draw `dv` under `survives`
// and its relay draw makes one (`fail`, fn-199 R1): the unit grows the
// share of its cycle the apex lived, at the relay's presence, and the
// relay grows the rest of the cycle, whole.
fn fail(apex_in: Apex, cycle: u32, s1: Step, gone: u32, whole: bool, keys: array<vec2<u32>, 2>, dv: vec2<u32>, survives: vec2<u32>) -> Next {
    var apex = apex_in;
    let a = apex.axis;
    let pa = rd(a, PA);
    let lived = lived_q(row(pa, 2u), unsigned_ticks(s1.share), dv);
    let below = sub64(shl64(lived, 12u), lived_fine_q(row(pa, 2u), unsigned_ticks(s1.share), dv)).x;
    let u = push_unit(a, rd(a, NUNITS));
    unit_asks(u, cycle, lived.x, keys[0], survives);
    unit_fails(u, 2u, s1.used.x);
    unit_grows(u, a, true);
    wr(a, NUNITS, rd(a, NUNITS) + 1u);
    wr64(a, SHARES, add64(rd64(a, SHARES), lived));
    grow_unit(a, pa, keys[1], cycle, lived.x);
    unit_grows(u, a, false);
    apex.spent = add64(apex.spent, lived);
    wr(a, APEX_END, cycle + 1u);
    wr(a, KIND, rd(a, KIND) | IS_FAILED);
    wr64(a, UNITS, apex.spent);
    let blend = clamp(above_t(dv, survives) / SPAN_F, 0.0, 1.0);
    let used = add64(w(gone, 0u), lived);
    let asks = array<vec4<u32>, 2>(vec4<u32>(RELAY_ASKS, FAILED, keys[0].x, keys[0].y), vec4<u32>(cycle, used.x, 0u, 0u));
    return relay_bud(apex, cycle, used, apex.spent, FAILED, whole, below, blend, asks);
}

// One living apex's growth unit in `cycle` (`advance`).
fn advance(apex_in: Apex, cycle: u32) -> Next {
    var apex = apex_in;
    let a = apex.axis;
    let pa = rd(a, PA);
    let life = row(pa, 0u);
    let lineage = rd64(a, LINEAGE);
    let first = rd(a, NUNITS) == 0u;
    var unit = child(lineage, w(cycle, 0u));
    // A failed unit's relay's first unit is keyed apart (fn-199 R1).
    if first && (rd(a, KIND) & IS_REST) != 0u {
        unit = child(unit, FAILED_KEY);
    }
    let n = rd(a, ENDED_RELAYS);
    var decide = unit;
    if n != 0u {
        decide = child(child(unit, ENDED), w(n, 0u));
    }
    let relay = (rd(a, KIND) & IS_ENDED) != 0u;
    let gone = select(0u, rd(a, SLEEP), first);
    let s1 = step(apex.spent, gone, first && relay, life, 1u);
    // D5.
    // A failed unit's relay's first unit holds a part of a tick more than
    // its ticks, which its thresholds read (fn-199 B2).
    let below = w(select(0u, rd(a, BELOW), first), 0u);
    var survives = row(pa, 1u);
    if lt64(unsigned_ticks(s1.share), ticks_whole()) {
        survives = survival_fine_q(row(pa, 2u), add64(shl64(unsigned_ticks(s1.share), 12u), below));
    }
    let dv = draw(child(decide, w(VIABILITY, 0u)));
    // A failure's relay, where its draw makes one (D9).
    let tf = row(pa, 6u + FAILED);
    let relays = !is_zero(tf) && lt64(draw(child(decide, RELAY)), tf);
    // A failed unit's relay's first unit carries on where its own failure
    // relays (fn-199, host decision A3).
    let nested = first && (rd(a, KIND) & IS_REST) != 0u && relays;
    if !lt64(dv, survives) && relays && !nested {
        return fail(apex, cycle, s1, gone, first && relay, array<vec2<u32>, 2>(decide, unit), dv, survives);
    }
    if !lt64(dv, survives) && !nested {
        wr(a, APEX_END, cycle);
        wr(a, KIND, rd(a, KIND) | IS_FAILED);
        let stopped = above_t(dv, survives);
        let spent = add64(apex.spent, s1.share);
        return stop(apex, cycle, ticks_whole(), decide, spent, FAILED, stopped, survives);
    }
    // Its survival's presence, Q32, at the chain's end (`assign.wgsl`).
    let u = push_unit(a, rd(a, NUNITS));
    unit_asks(u, cycle, unsigned_ticks(s1.share).x, decide, survives);
    if relays {
        unit_fails(u, 1u, s1.used.x);
    }
    unit_grows(u, a, true);
    wr(a, NUNITS, rd(a, NUNITS) + 1u);
    wr64(a, SHARES, add64(rd64(a, SHARES), s1.share));
    grow_unit(a, pa, unit, cycle, unsigned_ticks(s1.share).x);
    unit_grows(u, a, false);
    apex.spent = add64(apex.spent, s1.share);
    // D6.
    let grown = add64(rd64(a, AGED), rd64(a, SHARES));
    let next = shl64(unsigned_ticks(step(apex_in.spent, gone, first && relay, life, 2u).share), 12u);
    let exposure = min64(add64(shl64(unsigned_ticks(s1.share), 12u), below), next);
    var aborts = vec2<u32>(0u);
    let f = flags(pa);
    if (f & 2u) != 0u {
        if (f & 4u) == 0u && !lt64(exposure, shl64(ticks_whole(), 12u)) {
            aborts = row(pa, 3u);
        } else {
            aborts = abortion_fine_q(row(pa, 4u), (f & 4u) != 0u, row(pa, 5u), unsigned_ticks(grown), exposure);
        }
    }
    if !is_zero(aborts) && !s1.ends {
        let da = draw(child(decide, w(ABORTION, 0u)));
        if lt64(da, aborts) {
            wr(a, APEX_END, cycle + 1u);
            let stopped = below_t(da, aborts);
            return stop(apex, cycle, ticks_whole(), decide, apex.spent, ABORTED, stopped, aborts);
        }
        // An apex that all but aborted carries on as itself and as the
        // relay it would have had (host decision 11): its persistence,
        // Q32, at the chain's end.
        unit_persists(u, aborts);
    }
    if !s1.ends {
        push_next(a, apex.spent);
        return none();
    }
    wr(a, APEX_END, cycle + 1u);
    var used = ticks_whole();
    if lt64(s1.used, ticks_whole()) {
        used = s1.used;
    }
    let ends = child(child(lineage, END), w(n, 0u));
    return move_on(apex, ends, cycle, used);
}

// A relay's node (D18): on its parent's last unit's node presences.
fn relay_node(r: u32) {
    let p = rd(r, PARENT);
    var last = 0u;
    var count = 0u;
    for (var k = 0u; k < n_phyts; k++) {
        if phyts[phyt_base + k * PW + 3u] == p {
            last = phyts[phyt_base + k * PW + 2u];
            count += 1u;
        }
    }
    if count == 0u {
        wr(r, NODE, 0u);
        return;
    }
    // The unit's presences, the index of its first node on the axis.
    var total = vec4<u32>(0u);
    var first = 0u;
    var seen = 0u;
    var started = false;
    for (var k = 0u; k < n_phyts; k++) {
        let q = phyt_base + k * PW;
        if phyts[q + 3u] != p {
            continue;
        }
        if phyts[q + 2u] == last {
            if !started {
                first = seen;
                started = true;
            }
            total = add128(total, vec2<u32>(phyts[q + 4u], phyts[q + 5u]));
        }
        seen += 1u;
    }
    let share = row(rd(p, PA), 16u);
    let len = count - first;
    if is_zero(total.xy) && total.z == 0u {
        let f = mul64(share, w(len, 0u));
        let k = f.y + select(1u, 0u, f.x == 0u);
        wr(r, NODE, first + clamp(k, 1u, len) - 1u);
        return;
    }
    let goal = mul128(share, total.xy);
    var before = vec2<u32>(0u);
    var j = 0u;
    for (var k = 0u; k < n_phyts; k++) {
        let q = phyt_base + k * PW;
        if phyts[q + 3u] != p || phyts[q + 2u] != last {
            continue;
        }
        let pr = vec2<u32>(phyts[q + 4u], phyts[q + 5u]);
        let reach = add64(before, pr);
        let r4 = vec4<u32>(0u, reach.x, reach.y, 0u);
        let ge = r4.w > goal.w || (r4.w == goal.w && (r4.z > goal.z
            || (r4.z == goal.z && (r4.y > goal.y || (r4.y == goal.y && r4.x >= goal.x)))));
        if !is_zero(pr) && ge {
            wr(r, NODE, first + j);
            return;
        }
        before = reach;
        j += 1u;
    }
    wr(r, NODE, count - 1u);
}

// Runs the advances of the living apex `apex` in `cycle`, and every
// same-cycle growth they carry on, in order.
fn run(apex: Apex, cycle: u32) {
    var cur = Next(true, apex);
    loop {
        if overflow != 0u {
            break;
        }
        if cur.go {
            cur = advance(cur.apex, cycle);
            continue;
        }
        if depth == 0u {
            break;
        }
        depth -= 1u;
        cur = relay_after(stack[depth], cycle);
    }
}
