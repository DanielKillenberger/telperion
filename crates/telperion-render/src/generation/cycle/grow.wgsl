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

// One growth unit of the axis of frame `s`, of `pa`, keyed `unit_key`,
// in `cycle`: `share` ticks of its cycle, cut from `cut` ticks where a
// lifespan's fraction cut it (none: 0); its record `rec`.
fn grow_unit(s: u32, pa: u32, unit_key: vec2<u32>, cycle: u32, share: u32, cut: u32, rec: u32) {
    let axis = fr(s, 0u);
    let first = table[2u * pa];
    for (var k = 0u; k < table[2u * pa + 1u]; k++) {
        let zat = first + ZW * k;
        let zk = zone_key(unit_key, zat);
        let d = draw(zk);
        let window = fw(table[zat + 7u] + cycle);
        let lay = laid(zat, zk, zone_count(zat, d), window, share, cut);
        if lay.n > MOST {
            fault |= 4u;
            return;
        }
        var drawn: array<u32, MOST>;
        sorted(zk, lay.n, &drawn);
        let whorl = words[cfg.whorls + (zat - table[0]) / ZW];
        for (var m = 0u; m < lay.n; m++) {
            let j = drawn[m];
            let node_key = child(zk, w(j, 0u));
            var p = node_presence(zat, d, j, window);
            if lay.cut && j + 1u == lay.n {
                p = lay.last;
            }
            let node = fr(s, A_PHYTS);
            set_fr(s, A_PHYTS, node + 1u);
            push_phyt(axis, node_key, cycle, p);
            for (var slot = 0u; slot < table[zat + 4u]; slot++) {
                let place = child(node_key, w(slot, 0u));
                if !stands(zat, slot, place) {
                    continue;
                }
                let drew = bud(zat, place);
                if drew == 0xffu || lived_pa(drew) == NONE {
                    continue;
                }
                let lpa = lived_pa(drew);
                let window_l = window_none(lpa, cycle + 1u, 0u);
                let lead_p = drawn_presence(table[zat + 5u], drew, place, window_l);
                let made = exact_kept(lead_p * place_presence(zat, slot, place, window_l));
                let made2 = array<vec2<u32>, 2>(real64(made), real64(reach(drew, 0u)));
                sprout(place, lpa, cycle, LATERAL | (slot << 8u), vec2<u32>(axis, node), whorl, made2);
            }
            if words[table[zat + 11u] + cycle].x != 0u {
                sleep(zat, node_key, s, node, rec, cycle, whorl);
            }
        }
    }
    unit_grown(rec);
}

// The sleeping buds of the node keyed `node_key` that wake within the
// tree's age (`Shoot::sleep`, `dormant::draw`).
fn sleep(zat: u32, node_key: vec2<u32>, s: u32, node: u32, rec: u32, cycle: u32, whorl: vec2<u32>) {
    let waking = table[zat + 10u];
    if waking == NONE {
        return;
    }
    let start = table[zat + 8u];
    let n = table[zat + 9u];
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
        let whole = shr64(years, 20u).x;
        let window = fw(table[zat + 13u] + cycle * n + pa);
        let own = exact_kept(drawn_presence(start, pa, key, window) * place_presence(zat, slot, place, window));
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
            rec,
        ));
    }
}
