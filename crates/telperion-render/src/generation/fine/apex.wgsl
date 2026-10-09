// A living apex on the GPU chain grower (fn-209 F1, task 2): its growth
// unit (`grow_unit`), its advance, stop, move and relay (`advance.rs`,
// `stop.rs`), and each relay's node (D18). Part of `chain.wgsl`'s shader.

// The nodes of a zone a unit cut to `cut` ticks of `span_of` lays
// (`lineage::laid`): the index of the node its quota runs out in (`n`
// where it does not), and the threshold its `LAID` draw takes, none where
// that node is not laid.
struct Laid {
    last: u32,
    t: vec2<u32>,
}

fn laid_nodes(zat: u32, d: vec2<u32>, side: vec2<u32>, n: u32, window: vec2<u32>, cut: u32, span_of: u32) -> Laid {
    if span_of == 0u {
        return Laid(n, vec2<u32>(0u));
    }
    var sum = vec2<u32>(0u);
    for (var j = 0u; j < n; j++) {
        sum = add64(sum, presence(node_lead(zat, d, side, j), window));
    }
    let quota = mul64(w(cut, 0u), sum);
    let span = w(span_of, 0u);
    var held = vec2<u32>(0u);
    for (var j = 0u; j < n; j++) {
        let p = presence(node_lead(zat, d, side, j), window);
        if !lt64(quota, mul64(add64(held, p), span)) {
            held = add64(held, p);
            continue;
        }
        let left = sub64(quota, mul64(held, span));
        if is_zero(left) {
            return Laid(j, vec2<u32>(0u));
        }
        return Laid(j, fraction_up(left, mul64(p, span), 53u));
    }
    return Laid(n, vec2<u32>(0u));
}

// `lead_q(0, t, d, ..)`, to the word: the log-odds of the bound `t`
// less the draw `d`'s, its four `log2q` from one call site, so `grow`
// inlines the logarithm once (fn-235's cold make; MERGE-235.md).
fn under_lead(t: vec2<u32>, d: vec2<u32>) -> vec2<u32> {
    if !lt64(t, w(0u, 0x200000u)) || is_zero(d) {
        return inf();
    }
    // `logit(t << 9)`, then `logit_draw(d)`: each `ln_ratio`.
    var r = array<vec2<u32>, 2>();
    for (var k = 0u; k < 2u; k++) {
        let top = select(p_one(), w(0u, 0x200000u), k == 1u);
        let x = select(shl64(t, 9u), d, k == 1u);
        let y = sub64(top, min64(x, top));
        if is_zero(x) {
            r[k] = neg64(inf());
            continue;
        }
        if is_zero(y) {
            r[k] = inf();
            continue;
        }
        var lg = array<vec2<u32>, 2>();
        for (var m = 0u; m < 2u; m++) {
            lg[m] = log2q(select(x, y, m == 1u));
        }
        r[k] = smul_shr(sub64(lg[0], lg[1]), LN2, 62u, false);
    }
    return sub64(r[0], r[1]);
}

// A growth unit's phytomers and the laterals they bear (`grow_unit`), its
// nodes `ticks` of a whole one's length; where a lifespan's fraction cut it
// to `cut` ticks of `span_of` (none: 0), `cut / span_of` of each zone's
// present nodes, the next one by its `LAID` draw (`lineage::laid`, fn-231).
fn grow_unit(axis: u32, pa: u32, unit: vec2<u32>, cycle: u32, ticks: u32, cut: u32, span_of: u32) {
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
        let laid = laid_nodes(zat, d, side, n, window, cut, span_of);
        for (var m = 0u; m < n; m++) {
            let j = drawn[m];
            let node_key = child(zk, w(j, 0u));
            if j > laid.last || (j == laid.last && is_zero(laid.t)) {
                continue;
            }
            var node_p = presence(node_lead(zat, d, side, j), window);
            if j == laid.last {
                let t = laid.t;
                let dl = draw(child(node_key, KEY_LAID));
                if !lt64(dl, t) {
                    continue;
                }
                node_p = times_q(node_p, presence(under_lead(t, dl), window));
            }
            if n_phyts >= cfg.phyt_cap {
                overflow |= 2u;
                return;
            }
            let p = phyt_base + n_phyts * PW;
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
                if !stands(zat, s, place) {
                    continue;
                }
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
    wr(e.x, KIND, rd(e.x, KIND) | IS_CARRIED);
    return Next(true, Apex(e.x, e.yz));
}

// The relay of the apex that stopped having spent `spent` (`relay_bud`);
// one made `whole` grows a whole first unit.
fn relay_bud(apex: Apex, cycle: u32, used: vec2<u32>, spent: vec2<u32>, why: u32, whole: bool, below: u32, blend: f32, asks: array<vec4<u32>, 2>) -> Next {
    let r = make_relay(apex, cycle, used, spent, why, whole, below, blend, asks);
    if r == NONE {
        return none();
    }
    push_next(r, rd64(apex.axis, UNITS));
    return carry_on(cycle, used);
}

// `relay_bud`'s relay, made and not yet grown (`make_relay`).
fn make_relay(apex: Apex, cycle: u32, used: vec2<u32>, spent: vec2<u32>, why: u32, whole: bool, below: u32, blend: f32, asks: array<vec4<u32>, 2>) -> u32 {
    let a = apex.axis;
    let pa = rd(a, PA);
    let grown = add64(add64(rd64(a, AGED), rd64(a, SHARES)), sub64(spent, apex.spent));
    let ended_relays = rd(a, ENDED_RELAYS) + select(0u, 1u, why == ENDS);
    let units = rd64(a, UNITS);
    let h = horizon_of(pa, RELAYED, a);
    let r = push_axis(rd64(a, LINEAGE), pa, cycle, RELAYED, a, 0u, rd64(a, ROOT), h, units);
    if r == NONE {
        return NONE;
    }
    add_successor(a, r);
    made_axis(r, blend);
    ask(r, asks[0], asks[1]);
    wr64(r, AGED, grown);
    // A relay that grows in the cycle its axis stopped in, other than an
    // ended age's, grows the rest of a failed unit (fn-199 R1, C1); one
    // whose unit lived it whole grows on from the next, as the apex would
    // have.
    let rest = why != ENDS && lt64(used, ticks_whole());
    wr(r, KIND, RELAYED | select(0u, IS_ENDED, whole) | select(0u, IS_REST, rest));
    wr(r, ENDED_RELAYS, ended_relays);
    wr(r, BELOW, select(0u, below, rest));
    return r;
}

// The apex has stopped in the unit keyed `key` (`stop`).
fn stop(apex: Apex, cycle: u32, used: vec2<u32>, key: vec2<u32>, spent: vec2<u32>, why: u32, whole: bool, below: u32, stopped: f32, bound: vec2<u32>) -> Next {
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
    let d = draw(child(key, KEY_RELAY));
    if lt64(d, t) {
        // It stands between the axis's line and its own bud by the
        // stop's lead over the widest window (`stop`); its presences,
        // Q32, at the chain's end, a failed unit's past its survival's
        // bound.
        let blend = clamp(stopped / SPAN_F, 0.0, 1.0);
        let asks = array<vec4<u32>, 2>(vec4<u32>(RELAY_ASKS, why, key.x, key.y), vec4<u32>(cycle, used.x, bound.x, bound.y));
        return relay_bud(apex, cycle, used, spent, why, whole || why == ENDS, below, blend, asks);
    }
    return none();
}

// The apex has spent its age (`move_on`).
fn move_on(apex: Apex, key: vec2<u32>, cycle: u32, used: vec2<u32>) -> Next {
    let a = apex.axis;
    let pa = rd(a, PA);
    let next = successor(pa);
    if next == NONE {
        return stop(apex, cycle, used, key, apex.spent, ENDS, true, 0u, HUGE, vec2<u32>(0u));
    }
    let d = draw(child(key, KEY_MOVE));
    if !lt64(d, row(pa, 9u)) {
        let stopped = above_t(d, row(pa, 9u));
        return stop(apex, cycle, used, key, apex.spent, ENDS, true, 0u, stopped, vec2<u32>(0u));
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
    if lt64(f.made, one32()) && !is_zero(t) && lt64(draw(child(f.key, KEY_RELAY)), t) {
        wr64(f.apex.axis, UNITS, f.apex.spent);
        let asks = array<vec4<u32>, 2>(vec4<u32>(AFTER_ASKS, f.key.x, f.key.y, cycle), vec4<u32>(f.used.x, f.made.x, f.made.y, 0u));
        return relay_bud(f.apex, cycle, f.used, f.apex.spent, ENDS, true, 0u, 0.0, asks);
    }
    return none();
}

// The apex's unit failed on draw `dv` under `survives` and its relay
// draw makes one (`fail`, fn-199 R1): the unit, decided by `key`, grew
// the share `lived` of its cycle the apex lived, at the relay's presence
// (`advance`, which grows it, so `grow_unit` has one call site and the
// driver compiles it once, fn-235), and the relay grows the rest of the
// cycle, whole. `aborts` is the lived part's abortion threshold over the
// share it lived and the next unit's (fn-199, host decision C1), zero
// where the cycle ends the step.
fn fail(apex_in: Apex, cycle: u32, gone: u32, whole: bool, key: vec2<u32>, dv: vec2<u32>, survives: vec2<u32>, lived: vec2<u32>, below: u32, u: u32, aborts: vec2<u32>) -> Next {
    var apex = apex_in;
    let a = apex.axis;
    apex.spent = add64(apex.spent, lived);
    wr(a, APEX_END, cycle + 1u);
    wr(a, KIND, rd(a, KIND) | IS_FAILED);
    wr64(a, UNITS, apex.spent);
    let blend = clamp(above_t(dv, survives) / SPAN_F, 0.0, 1.0);
    let used = add64(w(gone, 0u), lived);
    let asks = array<vec4<u32>, 2>(vec4<u32>(RELAY_ASKS, FAILED, key.x, key.y), vec4<u32>(cycle, used.x, 0u, 0u));
    if !is_zero(aborts) {
        let da = draw(child(key, KEY_ABORTION));
        if lt64(da, aborts) {
            // Its relay stops at once, as an apex that aborted, and the
            // abortion's relay stands on it, so on the failed axis.
            let r = make_relay(apex, cycle, used, apex.spent, FAILED, whole, below, blend, asks);
            if r == NONE {
                return none();
            }
            wr(r, APEX_END, cycle + 1u);
            let stopped = below_t(da, aborts);
            return stop(Apex(r, apex.spent), cycle, used, key, apex.spent, ABORTED, whole, below, stopped, aborts);
        }
        unit_persists(u, aborts);
    }
    return relay_bud(apex, cycle, used, apex.spent, FAILED, whole, below, blend, asks);
}

// D6: the abortion threshold after a unit of PA `pa`, its axis having
// grown `grown` ticks, exposed `exposure` in 2^-32 of a cycle.
fn abortion_after(pa: u32, grown: vec2<u32>, exposure: vec2<u32>) -> vec2<u32> {
    let f = flags(pa);
    if (f & 2u) == 0u {
        return vec2<u32>(0u);
    }
    if (f & 4u) == 0u && !lt64(exposure, shl64(ticks_whole(), 12u)) {
        return row(pa, 3u);
    }
    return abortion_fine_q(row(pa, 4u), (f & 4u) != 0u, row(pa, 5u), unsigned_ticks(grown), exposure);
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
        unit = child(unit, KEY_FAILED);
    }
    let n = rd(a, ENDED_RELAYS);
    var decide = unit;
    if n != 0u {
        decide = child(child(unit, KEY_ENDED), w(n, 0u));
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
    let dv = draw(child(decide, KEY_VIABILITY));
    // A failure's relay, where its draw makes one (D9).
    let tf = row(pa, 6u + FAILED);
    let relays = !is_zero(tf) && lt64(draw(child(decide, KEY_RELAY)), tf);
    // A failed unit's relay's first unit carries on where its own failure
    // relays (fn-199, host decision A3).
    let nested = first && (rd(a, KIND) & IS_REST) != 0u && relays;
    let failing = !lt64(dv, survives) && relays && !nested;
    if !lt64(dv, survives) && !failing && !nested {
        wr(a, APEX_END, cycle);
        wr(a, KIND, rd(a, KIND) | IS_FAILED);
        let stopped = above_t(dv, survives);
        let spent = add64(apex.spent, s1.share);
        return stop(apex, cycle, ticks_whole(), decide, spent, FAILED, false, 0u, stopped, survives);
    }
    // A failing unit grows the share of its cycle the apex lived (`fail`).
    var share = s1.share;
    var lived = vec2<u32>(0u);
    var lived_below = 0u;
    if failing {
        lived = lived_q(row(pa, 2u), unsigned_ticks(s1.share), dv);
        lived_below = sub64(shl64(lived, 12u), lived_fine_q(row(pa, 2u), unsigned_ticks(s1.share), dv)).x;
        share = lived;
    }
    let grows = select(unsigned_ticks(s1.share).x, lived.x, failing);
    // Its survival's presence, Q32, at the chain's end (`assign.wgsl`).
    let u = push_unit(a, rd(a, NUNITS));
    unit_asks(u, cycle, grows, decide, survives);
    // A unit a lifespan's fraction cuts, the age's last or a successor's
    // rest of the cycle, lays that share of each zone's nodes, whole
    // (`Unit::laid`, fn-231); a woken bud's first unit and a failing
    // unit's lived part do not.
    let kind = rd(a, KIND);
    let successor = first && (kind & IS_CARRIED) != 0u && (kind & IS_REST) == 0u;
    let woken = first && (kind & IS_WOKE) != 0u;
    // A successor's rest is cut from a whole unit; the age's last from the
    // room its cycle leaves (`Unit::laid`).
    var span_of = 0u;
    if !failing && !woken && successor && gone > 0u {
        span_of = TICKS;
    } else if !failing && !woken && s1.ends && grows < TICKS - gone {
        span_of = TICKS - gone;
    }
    if u != NONE {
        sleepers[u + U_LAID] = span_of;
    }
    if failing {
        unit_fails(u, 2u, s1.used.x);
    } else if relays {
        unit_fails(u, 1u, s1.used.x);
    }
    unit_grows(u, a, true);
    wr(a, NUNITS, rd(a, NUNITS) + 1u);
    wr64(a, SHARES, add64(rd64(a, SHARES), share));
    grow_unit(a, pa, unit, cycle, select(grows, span_of, span_of != 0u), grows, span_of);
    unit_grows(u, a, false);
    // D6.
    let grown = add64(rd64(a, AGED), rd64(a, SHARES));
    let next = shl64(unsigned_ticks(step(apex_in.spent, gone, first && relay, life, 2u).share), 12u);
    var exposure = min64(add64(shl64(unsigned_ticks(s1.share), 12u), below), next);
    if failing {
        exposure = min64(shl64(lived, 12u), next);
    }
    let aborts = abortion_after(pa, grown, exposure);
    if failing {
        let lived_aborts = select(aborts, vec2<u32>(0u), s1.ends);
        return fail(apex, cycle, gone, first && relay, decide, dv, survives, lived, lived_below, u, lived_aborts);
    }
    apex.spent = add64(apex.spent, s1.share);
    if !is_zero(aborts) && !s1.ends {
        let da = draw(child(decide, KEY_ABORTION));
        if lt64(da, aborts) {
            wr(a, APEX_END, cycle + 1u);
            let stopped = below_t(da, aborts);
            return stop(apex, cycle, ticks_whole(), decide, apex.spent, ABORTED, false, 0u, stopped, aborts);
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
    let ends = child(child(lineage, KEY_END), w(n, 0u));
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
    // cap: every step grows a unit or makes, stops or pops an axis, so the
    // chain's regions bound them; past it the chain overflows, by name.
    var cap = 2u * (cfg.unit_cap + cfg.axes_cap) + 2u;
    loop {
        if cap == 0u {
            overflow |= 512u;
        }
        cap -= min(cap, 1u);
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
