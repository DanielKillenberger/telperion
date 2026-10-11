// One living apex's growth unit in a cycle (`grow/advance.rs`): its
// survival on the one cycle's contract (`decide::shaded`, `survival32`),
// the unit, then its abortion or its move to its next age; a failure its
// relay takes over grows the share the apex lived (`fail`, `stop.wgsl`).

// Unit `i` (1 or 2) of a bud carrying `spent` ticks into an age of `life`
// ticks, `gone` of its first cycle gone, a relay's whole first unit where
// `relay` (`schedule::Carried::unit`): its share (signed ticks), whether
// the age ends with it, and the share of its cycle gone when it ends.
struct Step {
    share: i32,
    ends: bool,
    used: i32,
}

fn step(spent: u32, gone: u32, relay: bool, life_ticks: u32, i: u32) -> Step {
    var g = i32(gone);
    var before = i32(spent);
    if i == 2u {
        before = i32(spent) + i32(TICKS) - i32(gone);
        g = 0;
    }
    let room = i32(TICKS) - g;
    let left = i32(life_ticks) - before;
    var share = min(room, left);
    if i == 1u && relay {
        share = room;
    }
    return Step(share, left <= room, g + share);
}

// A tick count as the contract's `ticks` takes it: none below none.
fn ticks_of(t: i32) -> u32 {
    return u32(max(t, 0));
}

// The one cycle's survival (`decide::shaded`): `v^(light^-phi)` in f32.
fn shaded(v: f32, phi: f32, light: f32) -> f32 {
    if phi > 0.0 && light <= 0.0 {
        return select(0.0, v, v >= 1.0);
    }
    if phi > 0.0 && light < 1.0 {
        return exp2c(log2c(v) * exp2c(-phi * log2c(light)));
    }
    return v;
}

// D5 of an f32 survival (`decide::survival32`) over `share` ticks and
// `rest` more in 2^-32 of a cycle.
fn survival32(p: f32, share: i32, rest: u32) -> vec2<u32> {
    if share >= i32(TICKS) {
        return threshold(p);
    }
    return survival_fine(neg_log2(p), add64(shl64(w(ticks_of(share), 0u), 12u), w(rest, 0u)));
}

// D6 (`decide::abortion`): after a unit that ends `grown` ticks of its
// axis's growth, exposed `exposure` in 2^-32 of a cycle.
fn abortion(pa: u32, grown: u32, exposure: vec2<u32>) -> vec2<u32> {
    let f = prow(pa, R_FLAGS).x;
    if (f & 1u) == 0u {
        return vec2<u32>(0u);
    }
    let rises = (f & 2u) != 0u;
    if !rises && !lt64(exposure, w(0u, 1u)) {
        return prow(pa, R_ABORT);
    }
    return abortion_fine(prow_f(pa, R_ABORT_L), rises, prow_f(pa, R_RISE), w(grown, 0u), exposure);
}

// The presence of the relay of a stop decided by `key`, under threshold
// `t`, over `window` (`Shoot::relayed`): none where it makes none.
fn relayed(t: vec2<u32>, key: vec2<u32>, window: f32) -> f32 {
    if is_zero(t) {
        return 0.0;
    }
    let d = draw(child(key, KEY_RELAY));
    if lt64(d, t) {
        return presence(lead(vec2<u32>(0u), t, d), window);
    }
    return 0.0;
}

// The abortion draw keyed `key` after a unit of `pa` in `cycle` under
// `aborts` (`Shoot::persists`): `ok` and the persistence, or the lead
// past it where the apex aborts.
struct Persists {
    ok: bool,
    value: f32,
}

fn persists(key: vec2<u32>, pa: u32, cycle: u32, aborts: vec2<u32>) -> Persists {
    if is_zero(aborts) {
        return Persists(true, 1.0);
    }
    let d = draw(child(key, KEY_ABORTION));
    if lt64(d, aborts) {
        return Persists(false, lead(vec2<u32>(0u), aborts, d));
    }
    let faded = presence(lead(aborts, ONE53, d), window_none(pa, cycle + 1u, 2u));
    let r = relayed(prow(pa, R_RELAY), key, window_none(pa, cycle + 1u, 0u));
    return Persists(true, r + kept_apart((1.0 - r) * faded));
}

// What a unit a lifespan's fraction cuts lays (`schedule::Unit::laid`):
// the ticks it is cut from, none where it is not cut.
fn cut_of(s: Step, gone: u32, successor: bool, woken: bool) -> u32 {
    let room = i32(TICKS) - i32(gone);
    if woken {
        return 0u;
    }
    if successor && gone > 0u {
        return TICKS;
    }
    if s.ends && s.share < room {
        return u32(room);
    }
    return 0u;
}

// A unit's shade as its axis's pencil gives it (`Draws::shade`).
fn shade_of(axis: u32) -> array<vec2<u32>, 2> {
    var light = ONE64;
    if cfg.lights != 0u {
        light = pen64(axis, 1u);
    }
    return array<vec2<u32>, 2>(pen64(axis, 0u), light);
}

// What runs next in an apex's cycle (`stop.wgsl`'s `run`): its
// advance; a stop, on `used` of the cycle, its lead `stopped`, its key
// and `spent` of its age (why, whole and below in `m`); the relay a stop
// makes, of presences `made0` and `made1`, which where `then_stop`
// stops at once on `stopped` and `key`; or a move to its next age.
const OP_NONE: u32 = 0u;
const OP_ADVANCE: u32 = 1u;
const OP_STOP: u32 = 2u;
const OP_RELAY: u32 = 3u;
const OP_MOVE: u32 = 4u;

struct Op {
    kind: u32,
    apex: Apex,
    used: u32,
    spent: u32,
    stopped: f32,
    key: vec2<u32>,
    m: Made,
    made0: vec2<u32>,
    made1: vec2<u32>,
    then_stop: bool,
}

fn op(kind: u32, apex: Apex) -> Op {
    let z = vec2<u32>(0u);
    return Op(kind, apex, 0u, 0u, 0.0, z, Made(0.0, 0u, false, 0u), z, z, false);
}

fn op_stop(apex: Apex, used: u32, stopped: f32, key: vec2<u32>, spent: u32, m: Made) -> Op {
    var o = op(OP_STOP, apex);
    o.used = used;
    o.stopped = stopped;
    o.key = key;
    o.spent = spent;
    o.m = m;
    return o;
}

fn op_relay(apex: Apex, used: u32, spent: u32, made: array<vec2<u32>, 2>, m: Made) -> Op {
    var o = op(OP_RELAY, apex);
    o.used = used;
    o.spent = spent;
    o.made0 = made[0];
    o.made1 = made[1];
    o.m = m;
    return o;
}

// One living apex's growth unit in `cycle` (`Shoot::advance`), the
// failed unit's that its relay takes over (`Shoot::fail`) included: what
// runs next.
fn advance(apex_in: Apex, cycle: u32) -> Op {
    var apex = apex_in;
    let s = frame_of(apex.axis);
    let pa = fr(s, A_PA);
    let lineage = fr64(s, A_LINEAGE);
    let flags = fr(s, A_FLAGS);
    let first = fr(s, A_UNITS) == 0u;
    var unit_key = child(lineage, w(cycle, 0u));
    if first && (flags & F_REST) != 0u {
        unit_key = child(unit_key, KEY_FAILED);
    }
    let n = fr(s, A_ENDED);
    var decide = unit_key;
    if n != 0u {
        decide = child(child(unit_key, KEY_ENDED), w(n, 0u));
    }
    let gone = select(0u, fr(s, A_SLEEP), first);
    let whole = first && (flags & F_ENDED) != 0u;
    let st = step(apex.spent, gone, whole, life(pa), 1u);
    let successor = (flags & F_CARRIED) != 0u && (flags & F_REST) == 0u;
    let cut = cut_of(st, gone, first && successor, first && (flags & F_WOKE) != 0u);
    let v = shaded(prow_f(pa, R_VIABILITY), prow_f(pa, R_SHADE), light32(apex.axis));
    let below = select(0u, fr(s, A_BELOW), first);
    let survives = survival32(v, st.share, below);
    let d = draw(child(decide, KEY_VIABILITY));
    let t_failed = prow(pa, R_RELAY + 2u);
    let relays = !is_zero(t_failed) && lt64(draw(child(decide, KEY_RELAY)), t_failed);
    let nested = first && (flags & F_REST) != 0u && relays;
    let dies = !lt64(d, survives) && !nested;
    if dies && !relays {
        set_fr(s, A_END, cycle);
        set_fr(s, A_FLAGS, flags | F_FAILED);
        let spent = u32(i32(apex.spent) + st.share);
        return op_stop(apex, TICKS, lead(survives, ONE53, d), decide, spent, Made(0.0, FAILED, false, 0u));
    }
    var share = ticks_of(st.share);
    var size = select(share, cut, cut != 0u);
    var failed = Failed(0u, 0u, 0.0, 0.0);
    var after = Persists(true, 1.0);
    var survive = 0.0;
    if dies {
        let r = relayed(t_failed, decide, window_whole(pa, cycle, u32(st.used)));
        let l = neg_log2(v);
        let unit_share = min64(w(share, 0u), w(TICKS, 0u));
        let lived = lived_ticks(l, unit_share, d).x;
        let lived_below = sub64(shl64(w(lived, 0u), 12u), lived_fine(l, unit_share, d)).x;
        let grown = fr(s, A_AGED) + fr(s, A_GROWN) + lived;
        let next = step(apex.spent, gone, whole, life(pa), 2u).share;
        var aborts = vec2<u32>(0u);
        if !st.ends {
            aborts = abortion(pa, grown, shl64(w(min(lived, ticks_of(next)), 0u), 12u));
        }
        after = persists(decide, pa, cycle, aborts);
        failed = Failed(lived, lived_below, r, lead(survives, ONE53, d));
        survive = kept_apart(r * kept32(apex.axis));
        share = lived;
        size = lived;
    } else {
        survive = presence(lead(vec2<u32>(0u), survives, d), window_none(pa, cycle, 1u));
        if relays {
            let r = relayed(t_failed, decide, window_whole(pa, cycle, u32(st.used)));
            survive = r + kept_apart((1.0 - r) * survive);
        }
        survive = kept_apart(survive * kept32(apex.axis));
    }
    let rec = push_unit(s, survive, share, size, shade_of(apex.axis));
    if dies {
        unit_persist(rec, select(1.0, after.value, after.ok));
        set_fr(s, A_FLAGS, fr(s, A_FLAGS) | F_FAILED);
    }
    grow_unit(s, pa, unit_key, cycle, share, select(cut, 0u, dies), rec);
    if dies {
        return failed_relay(apex, cycle, gone, whole, failed, decide, after);
    }
    apex.spent = u32(i32(apex.spent) + st.share);
    if !st.ends {
        let grown = fr(s, A_AGED) + fr(s, A_GROWN);
        let next = ticks_of(step(apex_in.spent, gone, whole, life(pa), 2u).share);
        var exposure = shl64(w(next, 0u), 12u);
        let own = add64(shl64(w(share, 0u), 12u), w(below, 0u));
        if !lt64(exposure, own) {
            exposure = own;
        }
        let p = persists(decide, pa, cycle, abortion(pa, grown, exposure));
        if !p.ok {
            set_fr(s, A_END, cycle + 1u);
            return op_stop(apex, TICKS, p.value, decide, apex.spent, Made(0.0, ABORTED, false, 0u));
        }
        unit_persist(rec, p.value);
        push_next(apex.axis, apex.spent);
        return op(OP_NONE, apex);
    }
    set_fr(s, A_END, cycle + 1u);
    var o = op(OP_MOVE, apex);
    o.key = child(child(lineage, KEY_END), w(n, 0u));
    o.used = TICKS;
    if st.used < i32(TICKS) {
        o.used = u32(st.used);
    }
    return o;
}
