// A growth unit's nodes and the buds of each (`grow/unit.rs`): its zones
// in their order along it, each zone's nodes in their draws' order, the
// laterals each node's places sprout and the sleeping buds they put to
// sleep (`dormant::draw`), on the unit tables (`zones.wgsl`).

// The nodes a unit a lifespan's fraction cut lays (`lineage::laid`): how
// many of the zone's, in index order, and the last one's presence where
// its `LAID` draw made it.
struct Laid {
    n: u32,
    cut: bool,
    last: f32,
}

fn node_presence(zat: u32, d: vec2<u32>, j: u32, window: f32) -> f32 {
    return presence(node_lead(zat, d, j), window);
}

fn laid(zat: u32, zk: vec2<u32>, n: u32, window: f32, share: u32, span: u32) -> Laid {
    if span == 0u {
        return Laid(n, false, 0.0);
    }
    let d = draw(zk);
    var sum = 0.0;
    for (var j = 0u; j < n; j++) {
        sum += node_presence(zat, d, j, window);
    }
    let quota = f32(share) / f32(span) * sum;
    var held = 0.0;
    for (var j = 0u; j < n; j++) {
        let p = node_presence(zat, d, j, window);
        if held + p <= quota {
            held += p;
            continue;
        }
        let left = quota - held;
        if left > 0.0 {
            let dl = draw(child(child(zk, w(j, 0u)), KEY_LAID));
            let t = threshold(left / p);
            if lt64(dl, t) {
                let lead_p = presence(lead(vec2<u32>(0u), t, dl), window);
                return Laid(j + 1u, true, exact_kept(p * lead_p));
            }
        }
        return Laid(j, false, 0.0);
    }
    return Laid(n, false, 0.0);
}

// The lower bound of PA `j`'s run of draws among the thresholds from
// `start` (`lineage::bud_presence`): the last made PA's before it, none
// where none is.
fn low_bound(start: u32, j: u32) -> vec2<u32> {
    var low = vec2<u32>(0u);
    for (var k = 0u; k < j; k++) {
        let t = words[start + k];
        if !(t.x == NONE && t.y == NONE) {
            low = t;
        }
    }
    return low;
}

// The presence of the draw keyed `key` that made PA `j` under the
// thresholds from `start`, over `window`.
fn drawn_presence(start: u32, j: u32, key: vec2<u32>, window: f32) -> f32 {
    return presence(lead(low_bound(start, j), words[start + j], draw(key)), window);
}

// The presence of the draw that made bud place `slot`, keyed `place`,
// stand (`lineage::place_presence`): whole for a whole place.
fn place_presence(zat: u32, slot: u32, place: vec2<u32>, window: f32) -> f32 {
    let t = words[table[zat + 12u] + slot];
    if !lt64(t, ONE53) {
        return 1.0;
    }
    return presence(lead(vec2<u32>(0u), t, draw(child(place, KEY_PLACED))), window);
}

// A unit's zone `k` (DESIGN.md section 10), at `zat`, its columns in
// `counts` from `cfg.zone_at`, `cfg.job_cap * cfg.zones` apart: its nodes
// laid, whether the last is cut and that one's presence, its laterals and
// sleeping buds, and a fault.
const Z_N: u32 = 0u;
const Z_CUT: u32 = 1u;
const Z_LAST: u32 = 2u;
const Z_LAT: u32 = 3u;
const Z_SLEEP: u32 = 4u;
const Z_FAULT: u32 = 5u;
// Then each zone's nodes in their order along the unit (`sorted`), `MOST`
// words a zone.
const Z_ORDER: u32 = 6u;

fn zw(z: u32, f: u32) -> u32 {
    return counts[cfg.zone_at + f * cfg.job_cap * cfg.zones + job * cfg.zones + z];
}

fn set_zw(z: u32, f: u32, v: u32) {
    counts[cfg.zone_at + f * cfg.job_cap * cfg.zones + job * cfg.zones + z] = v;
}

// The zone at `zat` of a unit keyed `unit_key` (`zone_key`): its nodes
// laid (`laid`), kept for the zone's nodes and `after`.
fn zone_laid(z: u32, zat: u32, unit_key: vec2<u32>, cycle: u32, share: u32, cut: u32) {
    let zk = zone_key(unit_key, zat);
    let d = draw(zk);
    let lay = laid(zat, zk, zone_count(zat, d), fw(table[zat + 7u] + cycle), share, cut);
    set_zw(z, Z_N, min(lay.n, MOST));
    set_zw(z, Z_CUT, u32(lay.cut));
    set_zw(z, Z_LAST, bitcast<u32>(lay.last));
    set_zw(z, Z_FAULT, select(0u, 4u, lay.n > MOST));
}

// Zone `z`'s nodes along the unit (`grow_unit`'s inner loops): `what`
// counts their laterals (0) or their sleeping buds (7), or writes their laterals
// (1), their sleeping buds (2), their phytomers (3) or their laterals'
// presences (the PA's draw's, 4, then the place's, 5), in the nodes'
// order along the unit from the places the zones before it left.
fn zone_nodes(z: u32, zat: u32, s: u32, cycle: u32, what: u32) {
    let axis = fr(s, 0u);
    let zk = zone_key(jw64(J_UKEY), zat);
    let d = draw(zk);
    let window = fw(table[zat + 7u] + cycle);
    let n = zw(z, Z_N);
    let whorl = words[cfg.whorls + (zat - table[0]) / ZW];
    var node = fr(s, A_PHYTS);
    let counting = what == 0u || what == 7u;
    if !counting {
        for (var k = 0u; k < z; k++) {
            n_phyts += zw(k, Z_N);
            n_axes += zw(k, Z_LAT);
            n_next += zw(k, Z_LAT);
            n_sleep += zw(k, Z_SLEEP);
            node += zw(k, Z_N);
        }
    }
    var laterals = 0u;
    var asleep = 0u;
    for (var m = 0u; m < n; m++) {
        let j = select(ordered(z, m), m, counting);
        let node_key = child(zk, w(j, 0u));
        if what == 3u {
            var p = node_presence(zat, d, j, window);
            if zw(z, Z_CUT) != 0u && j + 1u == n {
                p = bitcast<f32>(zw(z, Z_LAST));
            }
            push_phyt(axis, node_key, cycle, p);
        }
        if what < 2u || what == 4u || what == 5u {
            laterals += node_laterals(zat, node_key, vec2<u32>(axis, node), cycle, whorl, what);
        }
        if (what == 7u || what == 2u) && words[table[zat + 11u] + cycle].x != 0u {
            asleep += sleep(zat, node_key, s, node, cycle, whorl, what == 2u);
        }
        node += 1u;
    }
    if what == 1u {
        flush_next();
    } else if what == 0u {
        set_zw(z, Z_LAT, laterals);
    } else if what == 7u {
        set_zw(z, Z_SLEEP, asleep);
    }
}

// Zone `z`'s sleeping buds, as `sleep` wrote them, their presence taken:
// their PA's draw's times their place's.
fn sleepers_placed(z: u32, zat: u32, s: u32, cycle: u32) {
    var node0 = fr(s, A_PHYTS);
    for (var k = 0u; k < z; k++) {
        n_sleep += zw(k, Z_SLEEP);
        node0 += zw(k, Z_N);
    }
    if !emitting {
        return;
    }
    let zk = zone_key(jw64(J_UKEY), zat);
    let n = table[zat + 9u];
    for (var q = 0u; q < zw(z, Z_SLEEP); q++) {
        let row = base_sleep + n_sleep + q;
        if row >= cfg.sleep_cap {
            return;
        }
        let slot = sleeper(row, S_SLOT);
        let pa = sleeper(row, S_PA);
        let place = child(child(zk, w(ordered(z, sleeper(row, S_NODE) - node0), 0u)), w(slot, 0u));
        let window = fw(table[zat + 13u] + cycle * n + pa);
        let key = vec2<u32>(sleeper(row, S_KEY), sleeper(row, S_KEY + 1u));
        let own = exact_kept(drawn_presence(table[zat + 8u], pa, key, window) * place_presence(zat, slot, place, window));
        set_sleeper(row, S_OWN, bitcast<u32>(own));
    }
}

// Zone `z`'s node `m`'th along the unit, as `order_zone` kept it.
fn ordered(z: u32, m: u32) -> u32 {
    return counts[cfg.zone_at + Z_ORDER * cfg.job_cap * cfg.zones + (job * cfg.zones + z) * MOST + m];
}

// Zone `z`'s nodes in their order along the unit (D4), kept.
fn order_zone(z: u32, zat: u32) {
    let zk = zone_key(jw64(J_UKEY), zat);
    let n = zw(z, Z_N);
    var drawn: array<u32, MOST>;
    sorted(zk, n, &drawn);
    let at = cfg.zone_at + Z_ORDER * cfg.job_cap * cfg.zones + (job * cfg.zones + z) * MOST;
    for (var m = 0u; m < n; m++) {
        counts[at + m] = drawn[m];
    }
}

// The laterals the places of the node keyed `node_key`, on `at` (its
// axis and node), sprout: counted (`what` 0), sprouted (1), or the
// presence their draws made written to them once sprouted: the PA's
// draw's held in the low word (4), then times the place's (5).
fn node_laterals(zat: u32, node_key: vec2<u32>, at: vec2<u32>, cycle: u32, whorl: vec2<u32>, what: u32) -> u32 {
    var laterals = 0u;
    for (var slot = 0u; slot < table[zat + 4u]; slot++) {
        let place = child(node_key, w(slot, 0u));
        if !stands(zat, slot, place) {
            continue;
        }
        let drew = bud(zat, place);
        if drew == 0xffu || lived_pa(drew) == NONE {
            continue;
        }
        laterals += 1u;
        let lpa = lived_pa(drew);
        if what == 1u {
            let made2 = array<vec2<u32>, 2>(vec2<u32>(0u), real64(reach(drew, 0u)));
            sprout(place, lpa, cycle, LATERAL | (slot << 8u), at, whorl, made2);
        } else {
            let i = next_index();
            n_axes += 1u;
            let window_l = window_none(lpa, cycle + 1u, 0u);
            if !emitting || i >= cfg.axes_cap {
                continue;
            }
            if what == 4u {
                set_ax(i, A_B0, bitcast<u32>(drawn_presence(table[zat + 5u], drew, place, window_l)));
            } else {
                let made = real64(exact_kept(bitcast<f32>(ax(i, A_B0)) * place_presence(zat, slot, place, window_l)));
                set_ax(i, A_B0, made.x);
                set_ax(i, A_B0 + 1u, made.y);
            }
        }
    }
    return laterals;
}

// The sleeping buds of the node keyed `node_key` that wake within the
// tree's age (`Shoot::sleep`, `dormant::draw`): counted, or, where
// `write`, appended.
fn sleep(zat: u32, node_key: vec2<u32>, s: u32, node: u32, cycle: u32, whorl: vec2<u32>, write: bool) -> u32 {
    let waking = table[zat + 10u];
    if waking == NONE {
        return 0u;
    }
    let start = table[zat + 8u];
    let n = table[zat + 9u];
    var asleep = 0u;
    for (var slot = 0u; slot < table[zat + 4u]; slot++) {
        let place = child(node_key, w(slot, 0u));
        if !stands(zat, slot, place) {
            continue;
        }
        let key = child(place, KEY_DORMANT);
        let pa = drawn_pa(start, n, key);
        if pa == 0xffu {
            continue;
        }
        let years = wake_ticks_held(words[waking], fw(waking + 1u), draw(child(key, KEY_DORMANT)));
        if !lt64(years, shl64(w(cfg.age - cycle, 0u), 20u)) {
            continue;
        }
        asleep += 1u;
        if !write {
            continue;
        }
        let whole = shr64(years, 20u).x;
        // Its presence is taken by `sleepers_placed`.
        let own = 0.0;
        push_sleeper(array<u32, 14>(
            cycle + 1u + whole,
            fr(s, 0u),
            node,
            fr(s, A_UNITS) - 1u,
            slot,
            pa,
            bitcast<u32>(own),
            years.x & (TICKS - 1u),
            whole,
            whorl.x,
            whorl.y,
            key.x,
            key.y,
            jw(J_REC),
        ));
    }
    return asleep;
}
