// A growth unit's zones (fn-209 F1, task 2): `grow_unit`'s draws from the
// unit tables (`telperion_space::Units`), shared by the unit kernel and the
// chain grower. Each shader declares `table` (each PA's first zone and
// zones, then each zone's twelve words: zone, poisson, nodes, law at,
// places, laterals at, PAs, windows at, sleeping buds at, PAs, waking at,
// sleeps at) and `words` (each node's threshold and bound, the laterals'
// and sleeping buds' thresholds, all ones for none, each cycle's window,
// Q32, the waking delay and years a nat, and each cycle's gate on
// sleeping buds).

const MOST: u32 = 64u;
const ZW: u32 = 14u;
const ZONE: u32 = 16u;
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
    return child(unit, w(ZONE + table[at], 0u));
}

// The zone's `n` nodes in their order along the unit (D4): by their first
// place's draw, descending, equal draws by the node's index.
fn sorted(zk: vec2<u32>, n: u32, drawn: ptr<function, array<u32, MOST>>) {
    var order: array<vec2<u32>, MOST>;
    for (var j = 0u; j < n; j++) {
        let o = draw(child(child(zk, w(j, 0u)), w(0u, 0u)));
        var m = j;
        while m > 0u && lt64(order[m - 1u], o) {
            order[m] = order[m - 1u];
            (*drawn)[m] = (*drawn)[m - 1u];
            m -= 1u;
        }
        order[m] = o;
        (*drawn)[m] = j;
    }
}

// Node `j`'s lead past its bound, for the zone's draw `d` (D18's input).
fn node_lead(at: u32, d: vec2<u32>, j: u32) -> vec2<u32> {
    let bound = words[table[at + 3u] + 2u * j + 1u];
    if table[at + 1u] != 0u {
        return above_q(d, bound);
    }
    return past_q(d, bound);
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

// The PA bud place `place` drew, 0xff for a bare one.
fn bud(at: u32, place: vec2<u32>) -> u32 {
    return drawn_pa(table[at + 5u], table[at + 6u], place);
}
