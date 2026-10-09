// A growth unit's zones (fn-209 F1, task 2): `grow_unit`'s draws from the
// unit tables (`telperion_space::Units`), shared by the unit kernel and the
// chain grower. Each shader declares `table` (each PA's first zone and
// zones, then each zone's twelve words: zone, poisson, nodes, law at,
// places, laterals at, PAs, windows at, sleeping buds at, PAs, waking at,
// sleeps at) and `words` (each node's threshold and bound, then each
// node's bound's `logit` or `log2` (task 8), the laterals'
// and sleeping buds' thresholds, all ones for none, each cycle's window,
// Q32, the waking delay and years a nat, and each cycle's gate on
// sleeping buds).

const MOST: u32 = 64u;
const ZW: u32 = 14u;
const NONE: u32 = 0xffffffffu;

// The zone at `at`'s node count for its draw `d` (D2).
fn zone_count(at: u32, d: vec2<u32>) -> u32 {
    let n = table[at + 2u];
    let law = table[at + 3u];
    for (var j = 0u; j < n; j++) {
        if lt64(d, words[law + 2u * j]) {
            return j;
        }
    }
    return n;
}

fn zone_key(unit: vec2<u32>, at: u32) -> vec2<u32> {
    return child(unit, w(KEY_ZONE.x + table[at], 0u));
}

// The zone's `n` nodes in their order along the unit (D4): by their first
// place's draw, descending, equal draws by the node's index.
fn sorted(zk: vec2<u32>, n: u32, drawn: ptr<function, array<u32, MOST>>) {
    var order: array<vec2<u32>, MOST>;
    for (var j = 0u; j < n; j++) {
        let o = draw(child(child(zk, w(j, 0u)), w(0u, 0u)));
        var m = j;
        // cap: m counts down from j.
        while m > 0u && lt64(order[m - 1u], o) {
            order[m] = order[m - 1u];
            (*drawn)[m] = (*drawn)[m - 1u];
            m -= 1u;
        }
        order[m] = o;
        (*drawn)[m] = j;
    }
}

// The zone's draw's side of every node's lead, taken once a zone (task
// 8): its log-odds where the bounds are upper (`above_q`), its `log2`
// where they are lower (`past_q`); none for a draw of none or a zone
// that drew no node.
fn zone_side(at: u32, d: vec2<u32>, n: u32) -> vec2<u32> {
    if n == 0u {
        return vec2<u32>(0u);
    }
    if table[at + 1u] != 0u {
        return logit_draw(d);
    }
    if is_zero(d) {
        return vec2<u32>(0u);
    }
    return log2q(shl64(d, 9u));
}

// Node `j`'s lead past its bound, for the zone's draw `d` and its side
// (D18's input): `above_q(d, bound)` or `past_q(d, bound)` to the word.
fn node_lead(at: u32, d: vec2<u32>, side: vec2<u32>, j: u32) -> vec2<u32> {
    let bound = words[table[at + 3u] + 2u * j + 1u];
    if is_zero(bound) {
        return inf();
    }
    // The bound's own side, from the CPU (`units.rs`): its logit or log2.
    let own = words[table[at + 3u] + 2u * table[at + 2u] + j];
    if table[at + 1u] != 0u {
        return sub64(side, own);
    }
    if is_zero(d) {
        return neg64(inf());
    }
    return smul_shr(sub64(side, own), LN2, 62u, false);
}

// The PA a draw keyed `key` gives under the thresholds at `start`, `n`
// of them (D3), 0xff for none.
fn drawn_pa(start: u32, n: u32, key: vec2<u32>) -> u32 {
    let d = draw(key);
    for (var j = 0u; j < n; j++) {
        let t = words[start + j];
        if !(t.x == NONE && t.y == NONE) && lt64(d, t) {
            return j;
        }
    }
    return 0xffu;
}

// Whether bud place `s` of the zone at `at`, keyed `place`, stands: a
// whole one always, the last a fraction makes under its threshold, by its
// `PLACED` draw (`lineage::placed`, fn-231).
fn stands(at: u32, s: u32, place: vec2<u32>) -> bool {
    let t = words[table[at + 12u] + s];
    return !lt64(t, vec2<u32>(0u, 0x200000u)) || lt64(draw(child(place, KEY_PLACED)), t);
}

// The presence, Q32, over `window` of the draw that made that place stand:
// its lead under its threshold; a whole place's is whole
// (`lineage::place_presence`, fn-231).
fn place_q(at: u32, s: u32, place: vec2<u32>, window: vec2<u32>) -> vec2<u32> {
    let t = words[table[at + 12u] + s];
    if !lt64(t, vec2<u32>(0u, 0x200000u)) {
        return one32();
    }
    let d = draw(child(place, KEY_PLACED));
    return presence(lead_q(vec2<u32>(0u), t, d, vec4<u32>(0u), 0u), window);
}

// The PA bud place `place` drew, 0xff for a bare one.
fn bud(at: u32, place: vec2<u32>) -> u32 {
    return drawn_pa(table[at + 5u], table[at + 6u], place);
}
