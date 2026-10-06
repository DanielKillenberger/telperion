// A chain grown alone on the GPU (fn-209 F1, task 2): P2's chain grower
// (`telperion_space::grow::alone::grow`), one invocation per chain, from
// its bud and the species point's tables, every decision on the contract
// (`contract.wgsl`). It writes the chain's axes (its own, and the
// laterals it bears, made but not grown) and phytomers into the chain's
// region; a chain that outgrows its region stops and is reported, to be
// grown again in a larger one (the overflow rule: a chain is grown from
// its own bud alone, so a larger region grows the same chain). Its
// sleeping buds wake as buds it bears, and a woken bud's chain begins
// with the stages it slept through (`dormant.rs`, `wake.rs`).

struct Cfg {
    n: u32,
    row: u32,
    axes_cap: u32,
    phyt_cap: u32,
    live_cap: u32,
    rows: u32,
    age: u32,
    span_lo: u32,
    span_hi: u32,
    sleep_cap: u32,
    pend_cap: u32,
    pad: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> buds: array<vec2<u32>>;
@group(0) @binding(2) var<storage, read> table: array<u32>;
@group(0) @binding(3) var<storage, read> words: array<vec2<u32>>;
@group(0) @binding(4) var<storage, read_write> axes: array<u32>;
@group(0) @binding(5) var<storage, read_write> phyts: array<u32>;
@group(0) @binding(6) var<storage, read_write> live: array<vec4<u32>>;
@group(0) @binding(7) var<storage, read_write> status: array<vec4<u32>>;
@group(0) @binding(8) var<storage, read_write> sleepers: array<u32>;

const AXW: u32 = 28u;
const PW: u32 = 6u;
const ROW: u32 = 17u;
const TICKS: u32 = 0x100000u;
const STACK: u32 = 16u;
const SW: u32 = 10u;
const QW: u32 = 8u;
const BW: u32 = 10u;
const DORMANT = vec2<u32>(0xfffffff9u, 0xffffffffu);

// The draws' parts (`lineage.rs`).
const VIABILITY: u32 = 0u;
const ABORTION: u32 = 1u;
const MOVE: u32 = 2u;
const RELAY = vec2<u32>(0xfffffffdu, 0xffffffffu);
const ENDED = vec2<u32>(0xfffffff8u, 0xffffffffu);
const END = vec2<u32>(0xfffffff7u, 0xffffffffu);
const CONTINUATION = vec2<u32>(0xffffffffu, 0xffffffffu);

// How an axis stands on its parent.
const SEED: u32 = 0u;
const LATERAL: u32 = 1u;
const CONTINUED: u32 = 2u;
const RELAYED: u32 = 3u;
const STUB: u32 = 4u;

// Why an apex stopped: the relay share that takes it over.
const ABORTED: u32 = 0u;
const ENDS: u32 = 1u;
const FAILED: u32 = 2u;

// The fields of an axis.
const LINEAGE: u32 = 0u;
const PA: u32 = 2u;
const BIRTH: u32 = 3u;
const KIND: u32 = 4u;
const PARENT: u32 = 5u;
const NODE: u32 = 6u;
const APEX_END: u32 = 7u;
const ROOT: u32 = 8u;
const HORIZON: u32 = 10u;
const UNITS: u32 = 12u;
const AGED: u32 = 14u;
const SHARES: u32 = 16u;
const NUNITS: u32 = 18u;
const SLEEP: u32 = 19u;
const ENDED_RELAYS: u32 = 20u;
const OUTLIVED: u32 = 21u;
const SUCC: u32 = 22u;
const NPHYT: u32 = 24u;

// The KIND word's flags.
const IS_ENDED: u32 = 0x10000u;
const IS_FAILED: u32 = 0x20000u;
const HAS_OUTLIVED: u32 = 0x40000u;
const IS_WOKEN: u32 = 0x80000u;

var<private> axis_base: u32;
var<private> phyt_base: u32;
var<private> live_base: u32;
var<private> n_axes: u32;
var<private> n_phyts: u32;
var<private> n_next: u32;
var<private> next_half: u32;
var<private> overflow: u32;
var<private> sleep_base: u32;
var<private> pend_base: u32;
var<private> n_sleepers: u32;
var<private> n_pends: u32;

struct Apex {
    axis: u32,
    spent: vec2<u32>,
}

// A growth unit's advance still to run: the tail of a stop or a move.
struct Next {
    go: bool,
    apex: Apex,
}

// What a move on leaves to do once its continuation's same-cycle growth
// is done: the relay of an apex that all but stopped (host decision 11).
struct After {
    apex: Apex,
    used: vec2<u32>,
    made: vec2<u32>,
    key: vec2<u32>,
}

var<private> stack: array<After, STACK>;
var<private> depth: u32;

fn none() -> Next {
    return Next(false, Apex(0u, vec2<u32>(0u)));
}

fn at(i: u32, f: u32) -> u32 {
    return axis_base + i * AXW + f;
}

fn rd(i: u32, f: u32) -> u32 {
    return axes[at(i, f)];
}

fn wr(i: u32, f: u32, v: u32) {
    axes[at(i, f)] = v;
}

fn rd64(i: u32, f: u32) -> vec2<u32> {
    return vec2<u32>(axes[at(i, f)], axes[at(i, f + 1u)]);
}

fn wr64(i: u32, f: u32, v: vec2<u32>) {
    axes[at(i, f)] = v.x;
    axes[at(i, f + 1u)] = v.y;
}

// PA `pa`'s row: its flags word, and its words from `k`.
fn flags(pa: u32) -> u32 {
    return table[cfg.rows + 4u * pa];
}

fn lived(pa: u32) -> u32 {
    return table[cfg.rows + 4u * pa + 1u];
}

fn successor(pa: u32) -> u32 {
    return table[cfg.rows + 4u * pa + 2u];
}

fn row(pa: u32, k: u32) -> vec2<u32> {
    return words[table[cfg.rows + 4u * pa + 3u] + k];
}

fn log_wood(pa: u32, cycle: u32) -> vec2<u32> {
    return words[row(pa, 15u).x + cycle];
}

fn q32_whole(n: u32) -> vec2<u32> {
    return w(0u, n);
}

fn ticks_whole() -> vec2<u32> {
    return w(TICKS, 0u);
}

// A new axis; its index, or none where the region is full.
fn push_axis(lineage: vec2<u32>, pa: u32, birth: u32, kind: u32, parent: u32, node: u32, root: vec2<u32>, horizon: vec2<u32>, units: vec2<u32>) -> u32 {
    if n_axes >= cfg.axes_cap {
        overflow |= 1u;
        return NONE;
    }
    let i = n_axes;
    n_axes += 1u;
    for (var f = 0u; f < AXW; f++) {
        wr(i, f, 0u);
    }
    wr64(i, LINEAGE, lineage);
    wr(i, PA, pa);
    wr(i, BIRTH, birth);
    wr(i, KIND, kind);
    wr(i, PARENT, parent);
    wr(i, NODE, node);
    wr64(i, ROOT, root);
    wr64(i, HORIZON, horizon);
    wr64(i, UNITS, units);
    wr(i, SUCC, NONE);
    wr(i, SUCC + 1u, NONE);
    return i;
}

fn add_successor(i: u32, s: u32) {
    if rd(i, SUCC) == NONE {
        wr(i, SUCC, s);
    } else {
        wr(i, SUCC + 1u, s);
    }
}

fn push_next(axis: u32, spent: vec2<u32>) {
    if n_next >= cfg.live_cap {
        overflow |= 4u;
        return;
    }
    live[live_base + next_half * cfg.live_cap + n_next] = vec4<u32>(axis, spent.x, spent.y, 0u);
    n_next += 1u;
}

// The horizon of a new axis of `pa` standing on `parent` as `kind`.
fn horizon_of(pa: u32, kind: u32, parent: u32) -> vec2<u32> {
    let h = rd64(parent, HORIZON);
    if (kind == LATERAL || kind == RELAYED) && (flags(pa) & 16u) != 0u {
        return smax64(h, row(pa, 13u));
    }
    return h;
}

// D14: a lateral of `pa` made in `cycle` on `parent` certain to be shed
// by the tree's age.
fn shed_certain(pa: u32, cycle: u32, parent: u32) -> bool {
    if (flags(pa) & 48u) != 48u {
        return false;
    }
    let h = smax64(rd64(parent, HORIZON), row(pa, 13u));
    let bound = sub64(q32_whole(cfg.age), add64(q32_whole(cycle), row(pa, 14u)));
    return !slt64(bound, h);
}

// Unit `i` (1 or 2) of a bud carrying `spent` ticks, `gone` of its first
// cycle, as the relay of an ended age or failed unit (`relay`), in an age
// of `life` ticks: its share (signed ticks), whether the age ends with it,
// and the share of its cycle gone when it ends.
struct Step {
    share: vec2<u32>,
    ends: bool,
    used: vec2<u32>,
}

fn step(spent: vec2<u32>, gone: u32, relay: bool, life: vec2<u32>, i: u32) -> Step {
    var before = spent;
    var room = w(TICKS - gone, 0u);
    var g = gone;
    if i == 2u {
        before = add64(spent, w(TICKS - gone, 0u));
        room = ticks_whole();
        g = 0u;
    }
    let left = sub64(life, before);
    var share = smin64(room, left);
    if i == 1u && relay {
        share = room;
    }
    return Step(share, !slt64(room, left), add64(w(g, 0u), share));
}

// A signed tick count as the decisions read it: none below none.
fn unsigned_ticks(t: vec2<u32>) -> vec2<u32> {
    return select(t, vec2<u32>(0u), negative(t));
}

fn onto(key: vec2<u32>, pa: u32) -> vec2<u32> {
    return child(child(key, CONTINUATION), w(pa, 0u));
}

// A growth unit's phytomers and the laterals they bear (`grow_unit`).
fn grow_unit(axis: u32, pa: u32, unit: vec2<u32>, cycle: u32) {
    let first = table[2u * pa];
    let zones = table[2u * pa + 1u];
    for (var k = 0u; k < zones; k++) {
        let zat = first + ZW * k;
        let zk = zone_key(unit, zat);
        let d = draw(zk);
        let n = zone_count(zat, d);
        if n > MOST {
            overflow |= 8u;
            return;
        }
        var drawn: array<u32, MOST>;
        sorted(zk, n, &drawn);
        let window = words[table[zat + 7u] + cycle];
        for (var m = 0u; m < n; m++) {
            let j = drawn[m];
            let node_key = child(zk, w(j, 0u));
            if n_phyts >= cfg.phyt_cap {
                overflow |= 2u;
                return;
            }
            let p = phyt_base + n_phyts * PW;
            let presence = presence(node_lead(zat, d, j), window);
            phyts[p] = node_key.x;
            phyts[p + 1u] = node_key.y;
            phyts[p + 2u] = cycle;
            phyts[p + 3u] = axis;
            phyts[p + 4u] = presence.x;
            phyts[p + 5u] = presence.y;
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
                push_axis(onto(place, lpa), lpa, cycle, LATERAL | (s << 8u), axis, node, place, h, vec2<u32>(0u));
            }
            if words[table[zat + 11u] + cycle].x != 0u {
                sleep(zat, node_key, axis, node, cycle);
            }
        }
    }
}

// The sleeping buds of a node's places that wake within the tree's age
// (`dormant::draw`, D3 and D11), by the cycle they wake in.
fn sleep(zat: u32, node_key: vec2<u32>, axis: u32, node: u32, cycle: u32) {
    let waking = table[zat + 10u];
    if waking == NONE {
        return;
    }
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
        n_sleepers += 1u;
    }
}

// The stage a bud of `pa` that slept `slept` ticks wakes in, and its time
// there; none where it slept past its last (`dormant::aged`).
fn aged(pa_in: u32, slept_in: vec2<u32>) -> vec4<u32> {
    var pa = lived(pa_in);
    var slept = slept_in;
    // An age moves on only to an older one, so the stages run out.
    while pa != NONE {
        let life = row(pa, 0u);
        if lt64(slept, life) {
            return vec4<u32>(pa, slept.x, slept.y, 0u);
        }
        slept = sub64(slept, life);
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
        var top = 1u;
        open[0] = sleepers[q + 1u];
        while top > 0u {
            top -= 1u;
            let a = open[top];
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
                    n_pends += 1u;
                }
                continue;
            }
            if top + 2u > 32u {
                overflow |= 128u;
                return;
            }
            if rd(a, SUCC + 1u) != NONE {
                open[top] = rd(a, SUCC + 1u);
                top += 1u;
            }
            open[top] = rd(a, SUCC);
            top += 1u;
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

// The relay of the apex that stopped having spent `spent` (`relay_bud`).
fn relay_bud(apex: Apex, cycle: u32, used: vec2<u32>, spent: vec2<u32>, why: u32) -> Next {
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
    wr64(r, AGED, grown);
    wr(r, KIND, RELAYED | select(0u, IS_ENDED, why != ABORTED));
    wr(r, ENDED_RELAYS, ended_relays);
    push_next(r, units);
    return carry_on(cycle, used);
}

// The apex has stopped in the unit keyed `key` (`stop`).
fn stop(apex: Apex, cycle: u32, used: vec2<u32>, key: vec2<u32>, spent: vec2<u32>, why: u32) -> Next {
    let a = apex.axis;
    wr64(a, UNITS, spent);
    if why == ENDS {
        wr(a, KIND, rd(a, KIND) | HAS_OUTLIVED);
        wr(a, OUTLIVED, used.x);
    }
    let t = row(rd(a, PA), 6u + why);
    if is_zero(t) {
        return none();
    }
    if lt64(draw(child(key, RELAY)), t) {
        return relay_bud(apex, cycle, used, spent, why);
    }
    return none();
}

// The apex has spent its age (`move_on`).
fn move_on(apex: Apex, key: vec2<u32>, cycle: u32, used: vec2<u32>) -> Next {
    let a = apex.axis;
    let pa = rd(a, PA);
    let next = successor(pa);
    if next == NONE {
        return stop(apex, cycle, used, key, apex.spent, ENDS);
    }
    let d = draw(child(key, w(MOVE, 0u)));
    if !lt64(d, row(pa, 9u)) {
        return stop(apex, cycle, used, key, apex.spent, ENDS);
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
    let t = row(rd(f.apex.axis, PA), 7u);
    if lt64(f.made, one32()) && !is_zero(t) && lt64(draw(child(f.key, RELAY)), t) {
        wr64(f.apex.axis, UNITS, f.apex.spent);
        return relay_bud(f.apex, cycle, f.used, f.apex.spent, ENDS);
    }
    return none();
}

// One living apex's growth unit in `cycle` (`advance`).
fn advance(apex_in: Apex, cycle: u32) -> Next {
    var apex = apex_in;
    let a = apex.axis;
    let pa = rd(a, PA);
    let life = row(pa, 0u);
    let lineage = rd64(a, LINEAGE);
    let unit = child(lineage, w(cycle, 0u));
    let n = rd(a, ENDED_RELAYS);
    var decide = unit;
    if n != 0u {
        decide = child(child(unit, ENDED), w(n, 0u));
    }
    let first = rd(a, NUNITS) == 0u;
    let relay = (rd(a, KIND) & IS_ENDED) != 0u;
    let gone = select(0u, rd(a, SLEEP), first);
    let s1 = step(apex.spent, gone, first && relay, life, 1u);
    // D5.
    var survives = row(pa, 1u);
    if lt64(unsigned_ticks(s1.share), ticks_whole()) {
        survives = survival_q(row(pa, 2u), unsigned_ticks(s1.share));
    }
    if !lt64(draw(child(decide, w(VIABILITY, 0u))), survives) {
        wr(a, APEX_END, cycle);
        wr(a, KIND, rd(a, KIND) | IS_FAILED);
        return stop(apex, cycle, ticks_whole(), decide, add64(apex.spent, s1.share), FAILED);
    }
    wr(a, NUNITS, rd(a, NUNITS) + 1u);
    wr64(a, SHARES, add64(rd64(a, SHARES), s1.share));
    grow_unit(a, pa, unit, cycle);
    apex.spent = add64(apex.spent, s1.share);
    // D6.
    let grown = add64(rd64(a, AGED), rd64(a, SHARES));
    let exposure = unsigned_ticks(smin64(s1.share, step(apex_in.spent, gone, first && relay, life, 2u).share));
    var aborts = vec2<u32>(0u);
    let f = flags(pa);
    if (f & 2u) != 0u {
        if (f & 4u) == 0u && !lt64(exposure, ticks_whole()) {
            aborts = row(pa, 3u);
        } else {
            aborts = abortion_q(row(pa, 4u), (f & 4u) != 0u, row(pa, 5u), unsigned_ticks(grown), exposure);
        }
    }
    if !is_zero(aborts) && !s1.ends {
        if lt64(draw(child(decide, w(ABORTION, 0u))), aborts) {
            wr(a, APEX_END, cycle + 1u);
            return stop(apex, cycle, ticks_whole(), decide, apex.spent, ABORTED);
        }
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

// A woken bud's chain begins (`Shoot::woke`): the stages it slept
// through, axes of no length, then its own first unit in its waking
// cycle, which its sleep shortens.
fn woke(b: u32) -> u32 {
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

@compute @workgroup_size(64)
fn grow(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x + id.y * cfg.row;
    if c >= cfg.n {
        return;
    }
    axis_base = c * cfg.axes_cap * AXW;
    phyt_base = c * cfg.phyt_cap * PW;
    live_base = c * cfg.live_cap * 2u;
    sleep_base = c * (cfg.sleep_cap * SW + cfg.pend_cap * QW);
    pend_base = sleep_base + cfg.sleep_cap * SW;
    n_axes = 0u;
    n_phyts = 0u;
    n_sleepers = 0u;
    n_pends = 0u;
    overflow = 0u;
    depth = 0u;
    next_half = 0u;
    n_next = 0u;
    let b = c * BW;
    let lineage = buds[b];
    let pa = buds[b + 1u].x;
    let birth = buds[b + 1u].y;
    let seed = buds[b + 2u].x == 1u;
    let node = buds[b + 2u].y;
    let woken = buds[b + 5u].x == 1u;
    let slot = buds[b + 5u].y;
    let root = buds[b + 3u];
    let horizon = buds[b + 4u];
    var start = 1u;
    if seed {
        push_axis(lineage, pa, 0u, SEED, NONE, 0u, root, horizon, vec2<u32>(0u));
        push_next(0u, vec2<u32>(0u));
    } else if woken {
        // The bearer's stub stands at the bud's horizon.
        push_axis(vec2<u32>(0u), pa, 0u, STUB, NONE, 0u, vec2<u32>(0u), horizon, vec2<u32>(0u));
        start = woke(b);
    } else {
        push_axis(vec2<u32>(0u), pa, 0u, STUB, NONE, 0u, vec2<u32>(0u), neg64(inf()), vec2<u32>(0u));
        push_axis(lineage, pa, birth, LATERAL | (slot << 8u), 0u, node, root, horizon, vec2<u32>(0u));
        push_next(1u, vec2<u32>(0u));
        start = birth + 1u;
    }
    var cycles = 0u;
    for (var cycle = start; cycle <= cfg.age; cycle++) {
        if overflow != 0u || (n_next == 0u && !asleep_from(cycle)) {
            break;
        }
        // This cycle's living apexes, in order; the next cycle's are made
        // in the other half.
        let half = next_half;
        let count = n_next;
        next_half = 1u - half;
        n_next = 0u;
        for (var k = 0u; k < count; k++) {
            let e = live[live_base + half * cfg.live_cap + k];
            run(Apex(e.x, e.yz), cycle);
        }
        wakes(cycle);
        cycles += 1u;
    }
    for (var i = 0u; i < n_axes; i++) {
        if (rd(i, KIND) & 0xffu) == RELAYED {
            relay_node(i);
        }
    }
    // The buds that woke on it, after its axes, as it bears them.
    for (var k = 0u; k < n_pends; k++) {
        let p = pend_base + k * QW;
        let key = vec2<u32>(sleepers[p], sleepers[p + 1u]);
        let first = lived(sleepers[p + 2u]);
        let kind = LATERAL | IS_WOKEN | (sleepers[p + 5u] << 8u);
        let at = push_axis(onto(key, first), first, sleepers[p + 6u] - 1u, kind, sleepers[p + 3u], sleepers[p + 4u], key, vec2<u32>(0u), vec2<u32>(0u));
        if at == NONE {
            break;
        }
    }
    status[c] = vec4<u32>(n_axes, n_phyts, overflow, n_axes - n_pends);
}
