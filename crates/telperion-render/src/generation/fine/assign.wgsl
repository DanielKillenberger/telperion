// Presences as Q32 integers on the GPU chain grower (fn-209 F1 task 6;
// host decision 8): every presence a fade reads, bit for bit the CPU's
// (`fixed::presence`, `presence::assign_one`, `dormant.rs`). No decision
// reads one, so growth records only what each is taken from (its
// draw's key, its bounds, its wood), and the chain's end takes them all
// in one loop through one call of the contract's leads and windows: the
// driver's compiler inlines a 128-bit `log2` and `exp2` once, not at
// every site (FRICTION.md). The asks are taken by their own pass, a few
// threads a chain, and the products then run in the grower's order, a
// thread a chain (task 8).
// Part of `chain.wgsl`'s shader; the f32 presences (`presence.wgsl`)
// still size geometry alongside.

// An axis's Q32 fields, two words each: the draws that made it, its
// running survival, how much its apex lives on (where `IS_ALIVE`), its
// vigour, its end's presence, its living apex's, its growth units'
// presences summed and its last one's, its parent's time to its birth
// (Q32.32) and the whole cycles past it, and, for a lateral, its node's
// scale; then what its making draws' presences are taken from.
const B0: u32 = 40u;
const B1: u32 = 42u;
const QRUN: u32 = 44u;
const QLIVING: u32 = 46u;
const QVIG: u32 = 48u;
const QEND: u32 = 50u;
const QALIVE: u32 = 52u;
const QGROWN: u32 = 54u;
const QLAST: u32 = 56u;
const BEFORE: u32 = 58u;
const GAP: u32 = 60u;
const NSCALE: u32 = 68u;
const ASKS: u32 = 72u;
const IS_ALIVE: u32 = 0x100000u;
// A phytomer's scale by presence, Q32.
const QSCALE: u32 = 9u;
// What an axis's making draws' presences are taken from: none (given
// as made), a lateral's PA draw, a relay's stop and relay, the relay of
// an apex that all but stopped, a woken bud's own draw.
const LATERAL_ASKS: u32 = 1u;
const RELAY_ASKS: u32 = 2u;
const AFTER_ASKS: u32 = 3u;
const WOKEN_ASKS: u32 = 4u;
// A unit record's fields from word 4: its cycle, share (ticks), whether
// its apex all but aborted, its draws' key, its survival's and
// abortion's bounds, the phytomers and axes its unit made and its axis's
// phytomers before it; its Q32 survival, persistence and presence.
const U_CYCLE: u32 = 4u;
const U_SHARE: u32 = 5u;
const U_ABORTS: u32 = 6u;
const U_KEY: u32 = 7u;
const U_SURVIVES: u32 = 9u;
const U_BOUND: u32 = 11u;
const U_PHYTS: u32 = 13u;
const U_AXES: u32 = 15u;
const U_BEFORE: u32 = 17u;
const QSURV: u32 = 18u;
const QPERS: u32 = 20u;
const QUNIT: u32 = 22u;
// The windows' stakes (`PaRow::stakes`): whole, a failed unit's, an
// abortion's.
const WHOLE_STAKE: u32 = 0u;
const FAIL_STAKE: u32 = 1u;
const ABORT_STAKE: u32 = 2u;
const NEVER53 = vec2<u32>(0u, 0x200000u);

fn rq(i: u32, f: u32) -> vec2<u32> {
    return rd64(i, f);
}

fn wq(i: u32, f: u32, v: vec2<u32>) {
    wr64(i, f, v);
}

fn unit_q(u: u32, f: u32) -> vec2<u32> {
    return vec2<u32>(sleepers[u + f], sleepers[u + f + 1u]);
}

fn set_unit_q(u: u32, f: u32, v: vec2<u32>) {
    sleepers[u + f] = v.x;
    sleepers[u + f + 1u] = v.y;
}

// A share of a cycle in ticks, as a Q32 presence (`q` of a grid time).
fn ticks_q(t: vec2<u32>) -> vec2<u32> {
    return shl64(t, 12u);
}

// Records what axis `i`'s making draws' presences are taken from.
fn ask(i: u32, a: vec4<u32>, b: vec4<u32>) {
    for (var f = 0u; f < 4u; f++) {
        wr(i, ASKS + f, a[f]);
        wr(i, ASKS + 4u + f, b[f]);
    }
}

// Records unit `u`'s survival: its cycle, share, draws' key and bound.
fn unit_asks(u: u32, cycle: u32, share: u32, key: vec2<u32>, survives: vec2<u32>) {
    if u == NONE {
        return;
    }
    sleepers[u + U_CYCLE] = cycle;
    sleepers[u + U_SHARE] = share;
    set_unit_q(u, U_KEY, key);
    set_unit_q(u, U_SURVIVES, survives);
}

// Records where unit `u` of `a` begins (`start`) or ends: the phytomers
// and axes made, and `a`'s phytomers before it.
fn unit_grows(u: u32, a: u32, start: bool) {
    if u == NONE {
        return;
    }
    let at = select(1u, 0u, start);
    sleepers[u + U_PHYTS + at] = n_phyts;
    sleepers[u + U_AXES + at] = n_axes;
    if start {
        sleepers[u + U_BEFORE] = rd(a, NPHYT);
    }
}

// Records that unit `u`'s apex all but aborted, under `aborts`.
fn unit_persists(u: u32, aborts: vec2<u32>) {
    if u == NONE {
        return;
    }
    sleepers[u + U_ABORTS] = 1u;
    set_unit_q(u, U_BOUND, aborts);
}

// A presence to take: its draw `d` past the lower bound `low` and under
// the upper `high` (none: 0 and 2^53), over the window of PA `pa`'s wood
// from `cycle` (`used` ticks through it) at `stake`, or over `window`.
struct Ask {
    go: bool,
    low: vec2<u32>,
    high: vec2<u32>,
    d: vec2<u32>,
    pa: u32,
    cycle: u32,
    used: u32,
    stake: u32,
    window: vec2<u32>,
    wood: bool,
}

fn no_ask() -> Ask {
    return Ask(false, vec2<u32>(0u), NEVER53, vec2<u32>(0u), 0u, 0u, 0u, 0u, vec2<u32>(0u), false);
}

fn wood_ask(low: vec2<u32>, high: vec2<u32>, d: vec2<u32>, pa: u32, cycle: u32, used: u32, stake: u32) -> Ask {
    return Ask(true, low, high, d, pa, cycle, used, stake, vec2<u32>(0u), true);
}

// Unit `u`'s ask `j`: its survival (`advance`), then, where its apex all
// but aborted, its fade and its relay's presence.
fn unit_ask(u: u32, j: u32) -> Ask {
    let pa = rd(sleepers[u], PA);
    let cycle = sleepers[u + U_CYCLE];
    let key = unit_q(u, U_KEY);
    if j == 0u {
        let d = draw(child(key, w(VIABILITY, 0u)));
        return wood_ask(vec2<u32>(0u), unit_q(u, U_SURVIVES), d, pa, cycle, 0u, FAIL_STAKE);
    }
    if sleepers[u + U_ABORTS] == 0u {
        return no_ask();
    }
    if j == 1u {
        let d = draw(child(key, w(ABORTION, 0u)));
        return wood_ask(unit_q(u, U_BOUND), NEVER53, d, pa, cycle + 1u, 0u, ABORT_STAKE);
    }
    let t = row(pa, 6u + ABORTED);
    let d = draw(child(key, RELAY));
    if is_zero(t) || !lt64(d, t) {
        return no_ask();
    }
    return wood_ask(vec2<u32>(0u), t, d, pa, cycle + 1u, 0u, WHOLE_STAKE);
}

// Axis `i`'s ask `j`: the presence of its first (0) or second (1)
// making draw, where one is taken.
fn axis_ask(i: u32, j: u32) -> Ask {
    let kind = rd(i, ASKS);
    let pa = rd(i, PA);
    let p = ASKS + 1u;
    if kind == LATERAL_ASKS && j == 0u {
        let zat = rd(i, p);
        let drew = rd(i, p + 1u);
        let at = cfg.geo + gu(zone_g(zat) + 1u) + 2u * drew;
        let d = draw(vec2<u32>(rd(i, p + 2u), rd(i, p + 3u)));
        return wood_ask(words[at], words[at + 1u], d, pa, rd(i, BIRTH) + 1u, 0u, WHOLE_STAKE);
    }
    if kind == RELAY_ASKS {
        let why = rd(i, p);
        let key = vec2<u32>(rd(i, p + 1u), rd(i, p + 2u));
        let cycle = rd(i, p + 3u);
        let used = rd(i, p + 4u);
        if j == 0u {
            if why != FAILED {
                return no_ask();
            }
            let d = draw(child(key, w(VIABILITY, 0u)));
            let bound = vec2<u32>(rd(i, p + 5u), rd(i, p + 6u));
            return wood_ask(bound, NEVER53, d, pa, cycle, used, FAIL_STAKE);
        }
        let d = draw(child(key, RELAY));
        return wood_ask(vec2<u32>(0u), row(pa, 6u + why), d, pa, cycle, used, WHOLE_STAKE);
    }
    if kind == AFTER_ASKS && j == 1u {
        let key = vec2<u32>(rd(i, p), rd(i, p + 1u));
        let d = draw(child(key, RELAY));
        return wood_ask(vec2<u32>(0u), row(pa, 6u + ENDS), d, pa, rd(i, p + 2u), rd(i, p + 3u), WHOLE_STAKE);
    }
    if kind == WOKEN_ASKS && j == 0u {
        let zat = rd(i, p);
        let spa = rd(i, p + 2u);
        let at = cfg.geo + gu(zone_g(zat) + 2u) + 2u * spa;
        let d = draw(vec2<u32>(rd(i, p + 3u), rd(i, p + 4u)));
        let npa = table[zat + 9u];
        let window = words[table[zat + 13u] + rd(i, p + 1u) * npa + spa];
        return Ask(true, words[at], words[at + 1u], d, 0u, 0u, 0u, 0u, window, false);
    }
    return no_ask();
}

// The presence the chain's growth recorded as its ask `r`: unit records'
// first, three a unit, then the axes', two an axis, each into the unit's
// fields or the axis's B0 and B1. No ask reads what another writes.
fn take_ask(r: u32) {
    let units = n_units * 3u;
    {
        var a = no_ask();
        if r < units {
            a = unit_ask(unit_base + (r / 3u) * UW, r % 3u);
        } else {
            a = axis_ask((r - units) / 2u, (r - units) % 2u);
        }
        var p = vec2<u32>(0u);
        if a.go {
            var window = a.window;
            if a.wood {
                let wood = wood_at_q(log_wood(a.pa, a.cycle), log_wood(a.pa, a.cycle + 1u), w(a.used, 0u));
                let span = w(cfg.span_lo, cfg.span_hi);
                let rate = row(a.pa, 18u + 2u * a.stake);
                window = window_q(wood, rate, row(a.pa, 19u + 2u * a.stake), span);
            }
            p = presence(lead_q(a.low, a.high, a.d), window);
        }
        if r < units {
            set_unit_q(unit_base + (r / 3u) * UW, QSURV + 2u * (r % 3u), p);
        } else if a.go {
            wq((r - units) / 2u, B0 + 2u * ((r - units) % 2u), p);
        }
    }
}

// Lanes a chain's asks are shared among (task 8: a thread a chain left
// the beech's longest chains a third of its growth).
const LANES: u32 = 8u;

// Chain `c` as its growth left it: its regions, its counts from its
// status, and its units' count from its living apexes' room, spent once
// the chain has grown. False where it outgrew its region.
fn grown_chain(c: u32) -> bool {
    axis_base = c * cfg.axes_cap * AXW;
    phyt_base = c * cfg.phyt_cap * PW;
    live_base = c * cfg.live_cap * 2u;
    sleep_base = c * (cfg.sleep_cap * SW + cfg.pend_cap * QW + cfg.unit_cap * UW);
    pend_base = sleep_base + cfg.sleep_cap * SW;
    unit_base = pend_base + cfg.pend_cap * QW;
    let s = status[c];
    n_axes = s.x;
    n_phyts = s.y;
    overflow = s.z;
    n_units = live[live_base].x;
    return overflow == 0u;
}

// Every chain's asks, `LANES` threads a chain.
@compute @workgroup_size(64)
fn asks(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = id.x + id.y * cfg.ask_row;
    let c = g / LANES;
    if c >= cfg.n || !grown_chain(c) {
        return;
    }
    for (var r = g % LANES; r < n_units * 3u + n_axes * 2u; r += LANES) {
        take_ask(r);
    }
}

// Every chain's presences from its asks, a thread a chain; then every
// axis kept (realized mechanics).
@compute @workgroup_size(64)
fn assign(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x + id.y * cfg.row;
    if c >= cfg.n || !grown_chain(c) {
        return;
    }
    assign_q();
    keep_all();
}

// Stage `k` a bud of `pa` reaches, its reach (`PaRow::reaches`).
fn reach_q(pa: u32, k: u32) -> vec2<u32> {
    return words[row(pa, 27u).x + k];
}

// Each axis's making draws' presences from what was taken: a lateral's
// by its place's share, and its reach; a relay's stop, whole but for a
// failed unit's; the share a continuation that all but stopped lacks.
fn made_q() {
    for (var i = 0u; i < n_axes; i++) {
        let kind = rd(i, ASKS);
        let p = ASKS + 1u;
        if kind == LATERAL_ASKS || kind == WOKEN_ASKS {
            let zat = rd(i, p);
            let placed = words[table[zat + 12u] + ((rd(i, KIND) >> 8u) & 0xffu)];
            wq(i, B0, times_q(rq(i, B0), placed));
        }
        if kind == LATERAL_ASKS {
            wq(i, B1, reach_q(rd(i, p + 1u), 0u));
        }
        if kind == RELAY_ASKS && rd(i, p) != FAILED {
            wq(i, B0, one32());
        }
        if kind == AFTER_ASKS {
            wq(i, B0, sub64(one32(), vec2<u32>(rd(i, p + 4u), rd(i, p + 5u))));
        }
    }
}

// Each unit's persistence, then, in the grower's order, each axis's
// running survival, its units' presences summed, its phytomers' scales
// and its laterals' time and node (`presence::assign_one`, `shed.rs`).
fn units_q() {
    for (var k = 0u; k < n_units; k++) {
        let u = unit_base + k * UW;
        if sleepers[u + U_ABORTS] != 0u {
            let relayed = unit_q(u, QUNIT);
            let faded = unit_q(u, QPERS);
            set_unit_q(u, QPERS, add64(relayed, times_q(sub64(one32(), relayed), faded)));
        } else {
            set_unit_q(u, QPERS, one32());
        }
        let a = sleepers[u];
        let run = times_q(rq(a, QRUN), unit_q(u, QSURV));
        let share = ticks_q(w(sleepers[u + U_SHARE], 0u));
        let unit = times_q(run, share);
        set_unit_q(u, QUNIT, unit);
        wq(a, QGROWN, add64(rq(a, QGROWN), unit));
        wq(a, QLAST, unit);
        let first = sleepers[u + U_PHYTS];
        for (var q = first; q < sleepers[u + U_PHYTS + 1u]; q++) {
            let at = phyt_base + q * PW;
            let node = vec2<u32>(phyts[at + 4u], phyts[at + 5u]);
            let scale = times_q(times_q(run, node), share);
            phyts[at + QSCALE] = scale.x;
            phyts[at + QSCALE + 1u] = scale.y;
        }
        let before = add64(ticks_q(w(rd(a, SLEEP), 0u)), rq(a, QGROWN));
        for (var l = sleepers[u + U_AXES]; l < sleepers[u + U_AXES + 1u]; l++) {
            wq(l, BEFORE, before);
            let at = phyt_base + (first + rd(l, NODE) - sleepers[u + U_BEFORE]) * PW;
            wq(l, NSCALE, vec2<u32>(phyts[at + QSCALE], phyts[at + QSCALE + 1u]));
        }
        wq(a, QRUN, times_q(run, unit_q(u, QPERS)));
    }
}

// Every Q32 presence of the chain, once it has grown and its asks are
// taken (`asks`): the products in the grower's order, each woken bud's carrying, and
// each axis's vigour, end and living apex's presence and its parent's
// time to its birth (`presence::assign`).
fn assign_q() {
    made_q();
    units_q();
    for (var i = 0u; i < n_axes; i++) {
        if rd(i, ASKS) == WOKEN_ASKS {
            let p = rd(i, ASKS + 6u);
            let reach = reach_q(sleepers[p + 2u], sleepers[p + 18u]);
            wq(i, B1, times_q(times_q(carried_q(p), reach), woken_kept_q(p)));
        }
    }
    for (var i = 0u; i < n_axes; i++) {
        let made = times_q(rq(i, B0), rq(i, B1));
        let run = rq(i, QRUN);
        wq(i, QEND, run);
        let kind = rd(i, KIND) & 0xffu;
        if kind == CONTINUED || kind == RELAYED {
            wq(i, QVIG, times_q(rq(rd(i, PARENT), QEND), made));
        } else {
            wq(i, QVIG, made);
        }
        var alive = vec2<u32>(0u);
        if (rd(i, KIND) & IS_ALIVE) != 0u {
            alive = times_q(run, rq(i, QLIVING));
        }
        wq(i, QALIVE, alive);
        // A lateral the chain grew was given its time and its node's
        // scale with its unit.
        let woken = (rd(i, KIND) & IS_WOKEN) != 0u;
        if kind != LATERAL || woken {
            before_q(i);
        }
        if kind == LATERAL && woken {
            wq(i, NSCALE, node_scale_q(rd(i, PARENT), rd(i, NODE)));
        }
    }
}
