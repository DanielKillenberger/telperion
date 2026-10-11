// The sleeping buds that wake in the cycle (DESIGN.md section 2.2,
// `grow/wake.rs`): `asleep[cycle]` as the stable compaction of the rows
// waking now (`flag`, a scan, `gather`), then one invocation per bud in
// that order (`units.wgsl`'s entry), which wakes it on every axis that
// carries its bearer on, each a job with its first unit. A bud reads only
// what the units left.

@compute @workgroup_size(64)
fn flag(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n {
        return;
    }
    counts_a[4u * i] = select(0u, 1u, sleeper(i, S_WAKE) == cfg.cycle);
    counts_a[4u * i + 1u] = 0u;
    counts_a[4u * i + 2u] = 0u;
    counts_a[4u * i + 3u] = 0u;
}

@compute @workgroup_size(64)
fn gather(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.n || sleeper(i, S_WAKE) != cfg.cycle {
        return;
    }
    lists[cfg.list_at + counts_a[4u * i]] = i;
}

// The stage a bud of `pa` that slept `slept` ticks wakes in, its time
// there, and the stages it moved on (`dormant::aged`); none past its last.
struct Aged {
    pa: u32,
    spent: u32,
    steps: u32,
}

fn aged(pa_in: u32, slept_in: u32) -> Aged {
    var pa = lived_pa(pa_in);
    var slept = slept_in;
    // cap: an age moves on only to an older one.
    for (var steps = 0u; steps < 64u; steps++) {
        if pa == NONE {
            break;
        }
        if slept < life(pa) {
            return Aged(pa, slept, steps);
        }
        slept -= life(pa);
        pa = successor_pa(pa);
    }
    return Aged(NONE, 0u, 0u);
}

fn present(x: f32) -> f32 {
    return select(0.0, x, x > 0.0);
}

// Each axis that carries the bearer of sleeping bud `row` on to `cycle`,
// with the presence that carries it there (`dormant::carried`), in its
// order.
const CARRIED: u32 = 32u;
var<private> carried_n: u32;
var<private> carried_at: array<vec2<u32>, CARRIED>;

fn carried(row: u32, cycle: u32) {
    carried_n = 0u;
    var open: array<vec3<u32>, CARRIED>;
    let rec = sleeper(row, S_REC);
    open[0] = vec3<u32>(bitcast<u32>(present(bitcast<f32>(unit(rec, U_PERSIST)))), sleeper(row, S_PARENT), unit(rec, U_NEXT));
    var top = 1u;
    // cap: every axis of the bearer's chain taken once.
    for (var turns = 0u; turns < 4096u && top > 0u; turns++) {
        top -= 1u;
        var p = bitcast<f32>(open[top].x);
        let a = open[top].y;
        var r = open[top].z;
        for (var guard = 0u; r != NONE && guard < 65536u; guard++) {
            p = p * present(bitcast<f32>(unit(r, U_SURVIVE)));
            p = p * present(bitcast<f32>(unit(r, U_PERSIST)));
            r = unit(r, U_NEXT);
        }
        if ax(a, A_SUCC0) == NONE {
            let end = ax(a, A_END);
            let here = end == cycle + 1u && (ax(a, A_FLAGS) & F_OUTLIVED) != 0u;
            if end == 0u || here {
                if carried_n >= CARRIED {
                    fault |= 8u;
                    return;
                }
                carried_at[carried_n] = vec2<u32>(bitcast<u32>(p), a);
                carried_n += 1u;
            }
            continue;
        }
        if top + 2u > CARRIED {
            fault |= 8u;
            return;
        }
        for (var k = 0u; k < 2u; k++) {
            let next = ax(a, A_SUCC1 - k);
            if next == NONE {
                continue;
            }
            let on = p * q64(vec2<u32>(ax(next, A_B0), ax(next, A_B0 + 1u)));
            let on2 = on * q64(vec2<u32>(ax(next, A_B1), ax(next, A_B1 + 1u)));
            open[top] = vec3<u32>(bitcast<u32>(on2), next, ax(next, A_HEAD));
            top += 1u;
        }
    }
}

// A stage a sleeping bud slept through: an axis of no length (`slept`).
fn slept_axis(key: vec2<u32>, root: vec2<u32>, pa: u32, cycle: u32, origin: u32, at: vec2<u32>, whorl: vec2<u32>, made: array<vec2<u32>, 2>) -> u32 {
    let i = next_index();
    n_axes += 1u;
    let s = open_frame(i);
    set_fr(s, A_PA, pa);
    set_fr(s, A_BIRTH, cycle);
    set_fr(s, A_ORIGIN, origin);
    set_fr(s, A_PARENT, at.x);
    set_fr(s, A_NODE, at.y);
    set_fr64(s, A_LINEAGE, key);
    set_fr64(s, A_ROOT, root);
    set_fr64(s, A_HORIZON, horizon_of(pa, origin & 0xffu, at.x));
    set_fr64(s, A_WHORL, whorl);
    set_fr64(s, A_BLEND, ONE64);
    set_fr(s, A_SPENT, life(pa));
    set_fr64(s, A_B0, made[0]);
    set_fr64(s, A_B1, made[1]);
    set_fr(s, A_END, cycle + 1u);
    return s;
}

// Sleeping bud `row` wakes on the axis `by` names, aged to `g`, carried
// there by the presence `by` holds (`Shoot::woke`): the stages it slept
// through, then its own axis, whose apex grows its first unit in the
// cycle.
fn woke(row: u32, g: Aged, by: vec2<u32>, cycle: u32) -> Apex {
    let bearer = by.y;
    let spa = sleeper(row, S_PA);
    let sleep_ticks = sleeper(row, S_SLEEP);
    let root = vec2<u32>(sleeper(row, S_KEY), sleeper(row, S_KEY + 1u));
    push_woken(array<u32, 5>(next_index(), bearer, ax(bearer, A_UNITS), cycle, TICKS - sleep_ticks));
    let own = bitcast<f32>(sleeper(row, S_OWN));
    var made = array<vec2<u32>, 2>(real64(own), real64(bitcast<f32>(by.x) * reach(spa, g.steps)));
    var origin = LATERAL | (sleeper(row, S_SLOT) << 8u) | WOKEN;
    var at = vec2<u32>(sleeper(row, S_PARENT), sleeper(row, S_NODE));
    var whorl = vec2<u32>(sleeper(row, S_WHORL), sleeper(row, S_WHORL + 1u));
    var key = root;
    var stage = lived_pa(spa);
    // cap: an age moves on only to an older one.
    for (var turns = 0u; stage != g.pa && turns < 64u; turns++) {
        let i = next_index();
        var lineage = key;
        if (origin & 0xffu) == LATERAL {
            lineage = onto(root, stage);
        }
        let s = slept_axis(lineage, root, stage, cycle - 1u, origin, at, whorl, made);
        add_successor(s, i + 1u);
        stage = successor_pa(stage);
        origin = CONTINUED;
        at = vec2<u32>(i, 0u);
        whorl = vec2<u32>(0u);
        key = onto(root, stage);
        made = array<vec2<u32>, 2>(ONE64, ONE64);
    }
    let s = sprout(key, g.pa, cycle - 1u, origin, at, whorl, made);
    set_fr(s, A_SLEEP, sleep_ticks);
    set_fr(s, A_AGED, g.spent);
    set_fr(s, A_FLAGS, F_WOKE);
    set_fr(s, A_SPENT, g.spent);
    return Apex(pop_next().axis, g.spent);
}
