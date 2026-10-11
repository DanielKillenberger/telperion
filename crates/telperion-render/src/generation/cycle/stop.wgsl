// An apex's stop, its move to its next age and the relay either may
// leave (`grow/stop.rs`), the buds they sprout and carry on in the same
// cycle (`grow/shoot.rs`), and the loop that runs an apex's advances and
// everything they carry on, in the CPU's order.

// Why an apex stopped: which relay threshold takes it over.
const ABORTED: u32 = 0u;
const ENDS: u32 = 1u;
const FAILED: u32 = 2u;

// How an axis stands on its parent.
const LATERAL: u32 = 1u;
const CONTINUED: u32 = 2u;
const RELAYED: u32 = 3u;
const WOKEN: u32 = 0x10000u;

fn onto(key: vec2<u32>, pa: u32) -> vec2<u32> {
    return child(child(key, KEY_CONTINUATION), w(pa, 0u));
}

// A new axis's horizon (`Shoot::horizon_of`): its parent's, or its own
// shedding delay and a cycle where that is later and it sheds.
fn horizon_of(pa: u32, kind: u32, parent: u32) -> vec2<u32> {
    let h = axis_word64(parent, A_HORIZON);
    if (kind == LATERAL || kind == RELAYED) && (prow(pa, R_FLAGS).x & 4u) != 0u {
        return max64(h, prow(pa, R_DELAY));
    }
    return h;
}

// A new bud that grows from the next cycle (`Shoot::sprout`), made by
// draws of presences `made`: its frame, or the scratch frame, written at
// once, for a lateral its bearer made.
fn sprout(key: vec2<u32>, pa: u32, cycle: u32, origin: u32, at: vec2<u32>, whorl: vec2<u32>, made: array<vec2<u32>, 2>) -> u32 {
    let parent = at.x;
    let node = at.y;
    let kind = origin & 0xffu;
    var spent = 0u;
    var root = key;
    var lineage = onto(key, pa);
    if kind == RELAYED {
        spent = fr(frame_of(parent), A_SPENT);
    }
    if kind == CONTINUED || kind == RELAYED {
        root = axis_word64(parent, A_ROOT);
        lineage = key;
    }
    let i = next_index();
    n_axes += 1u;
    push_next(i, spent);
    let borne = kind == LATERAL && (origin & WOKEN) == 0u;
    var s = SCRATCH;
    if borne {
        clear(s, i);
    } else {
        s = open_frame(i);
    }
    set_fr(s, A_PA, pa);
    set_fr(s, A_BIRTH, cycle);
    set_fr(s, A_ORIGIN, origin);
    set_fr(s, A_PARENT, parent);
    set_fr(s, A_NODE, node);
    set_fr64(s, A_LINEAGE, lineage);
    set_fr64(s, A_ROOT, root);
    set_fr64(s, A_HORIZON, horizon_of(pa, kind, parent));
    set_fr64(s, A_WHORL, whorl);
    set_fr64(s, A_BLEND, ONE64);
    set_fr(s, A_SPENT, spent);
    set_fr64(s, A_B0, made[0]);
    set_fr64(s, A_B1, made[1]);
    if borne {
        store(s);
    }
    return s;
}

// The bud last sprouted grows on in this cycle at the rest of it, where
// `used` of it is gone (`Shoot::carry_on`).
fn carry_on(cycle: u32, used: u32) -> Op {
    if used >= TICKS {
        return op(OP_NONE, Apex(0u, 0u));
    }
    let a = pop_next();
    let s = frame_of(a.axis);
    set_fr(s, A_BIRTH, cycle - 1u);
    set_fr(s, A_SLEEP, used);
    set_fr(s, A_FLAGS, fr(s, A_FLAGS) | F_CARRIED);
    return op(OP_ADVANCE, a);
}

// How a relay is made: its stop's blend, why, whether its first unit is
// whole, and what that unit holds below its ticks.
struct Made {
    blend: f32,
    why: u32,
    whole: bool,
    below: u32,
}

// The relay of the apex that stopped having spent `spent`, made and not
// yet grown (`Shoot::make_relay`).
fn make_relay(apex: Apex, cycle: u32, used: u32, spent: u32, made: array<vec2<u32>, 2>, m: Made) {
    let s = frame_of(apex.axis);
    let grown = fr(s, A_AGED) + fr(s, A_GROWN) + spent - apex.spent;
    let ended = fr(s, A_ENDED) + select(0u, 1u, m.why == ENDS);
    add_successor(s, next_index());
    let r = sprout(fr64(s, A_LINEAGE), fr(s, A_PA), cycle, RELAYED, vec2<u32>(apex.axis, 0u), vec2<u32>(0u), made);
    set_fr64(r, A_BLEND, real64(m.blend));
    set_fr(r, A_AGED, grown);
    set_fr(r, A_ENDED, ended);
    let rest = m.why != ENDS && used < TICKS;
    set_fr(r, A_FLAGS, select(0u, F_ENDED, m.whole) | select(0u, F_REST, rest));
    set_fr(r, A_BELOW, select(0u, m.below, rest));
}

// A relay operation (`Shoot::relay_bud`): the relay made, then carried
// on in the cycle, or, where the lived part of a failed unit aborted,
// stopped at once (`Shoot::fail`).
fn relay(o: Op, cycle: u32) -> Op {
    make_relay(o.apex, cycle, o.used, o.spent, array<vec2<u32>, 2>(o.made0, o.made1), o.m);
    if !o.then_stop {
        return carry_on(cycle, o.used);
    }
    let r = pop_next();
    set_fr(frame_of(r.axis), A_END, cycle + 1u);
    return op_stop(r, o.used, o.stopped, o.key, r.spent, Made(0.0, ABORTED, o.m.whole, o.m.below));
}

// A stop operation (`Shoot::stop`): the apex has stopped `stopped` past
// its last draw; a relay may take over.
fn stop(o: Op, cycle: u32) -> Op {
    let s = frame_of(o.apex.axis);
    let why = o.m.why;
    set_fr(s, A_SPENT, o.spent);
    if why == ENDS {
        set_fr(s, A_FLAGS, fr(s, A_FLAGS) | F_OUTLIVED);
        set_fr(s, A_OUTLIVED, o.used);
    }
    let pa = fr(s, A_PA);
    let t = prow(pa, R_RELAY + why);
    let d = draw(child(o.key, KEY_RELAY));
    if is_zero(t) || !lt64(d, t) {
        return op(OP_NONE, o.apex);
    }
    let made = presence(lead(vec2<u32>(0u), t, d), window_whole(pa, cycle, o.used));
    let m = Made(presence(o.stopped, 2.0), why, o.m.whole || why == ENDS, o.m.below);
    return op_relay(o.apex, o.used, o.spent, array<vec2<u32>, 2>(ONE64, real64(made)), m);
}

// What a move leaves to do once its continuation's growth in the cycle
// is done: the relay of an apex that all but stopped (host decision 11).
struct After {
    apex: Apex,
    used: u32,
    made: f32,
    key: vec2<u32>,
}

// The job's stack of `After`s (`job.wgsl`), six words an entry.
fn push_after(f: After) {
    let depth = jw(J_DEPTH);
    if depth >= STACK {
        fault |= 16u;
        return;
    }
    let at = J_STACK + 6u * depth;
    set_jw(at, f.apex.axis);
    set_jw(at + 1u, f.apex.spent);
    set_jw(at + 2u, f.used);
    set_jwf(at + 3u, f.made);
    set_jw64(at + 4u, f.key);
    set_jw(J_DEPTH, depth + 1u);
}

fn pop_after() -> After {
    let depth = jw(J_DEPTH) - 1u;
    set_jw(J_DEPTH, depth);
    let at = J_STACK + 6u * depth;
    return After(Apex(jw(at), jw(at + 1u)), jw(at + 2u), jwf(at + 3u), jw64(at + 4u));
}

// A move operation (`Shoot::move_on`): the apex has spent its age and
// moves on by its continuation, or stops.
fn move_on(o: Op, cycle: u32) -> Op {
    let apex = o.apex;
    let s = frame_of(apex.axis);
    let pa = fr(s, A_PA);
    let next = successor_pa(pa);
    let d = draw(child(o.key, KEY_MOVE));
    let go = prow(pa, R_GO);
    if next == NONE || !lt64(d, go) {
        let stopped = select(lead(go, ONE53, d), INF, next == NONE);
        return op_stop(apex, o.used, stopped, o.key, apex.spent, Made(0.0, ENDS, true, 0u));
    }
    let m = prow(pa, R_MOVED);
    let woods = vec2<f32>(log_wood_at(next, cycle), log_wood_at(next, cycle + 1u));
    let made = moved(prow_f(pa, R_GO_LOGIT), d, woods.x, woods.y, w(o.used, 0u), bitcast<f32>(m.x), bitcast<f32>(m.y), 2.0);
    add_successor(s, next_index());
    let carried = array<vec2<u32>, 2>(real64(made), ONE64);
    sprout(onto(fr64(s, A_ROOT), next), next, cycle, CONTINUED, vec2<u32>(apex.axis, 0u), vec2<u32>(0u), carried);
    let tail = carry_on(cycle, o.used);
    let after = After(apex, o.used, made, o.key);
    if tail.kind == OP_NONE {
        return relay_after(after, cycle);
    }
    push_after(after);
    return tail;
}

fn relay_after(f: After, cycle: u32) -> Op {
    let s = frame_of(f.apex.axis);
    let pa = fr(s, A_PA);
    let t = prow(pa, R_RELAY + ENDS);
    if !(f.made < 1.0) || is_zero(t) || !lt64(draw(child(f.key, KEY_RELAY)), t) {
        return op(OP_NONE, f.apex);
    }
    let r = relayed(t, f.key, window_whole(pa, cycle, f.used));
    set_fr(s, A_SPENT, f.apex.spent);
    let made = array<vec2<u32>, 2>(one_less(f.made), real64(r));
    return op_relay(f.apex, f.used, f.apex.spent, made, Made(0.0, ENDS, true, 0u));
}

// What a failed unit's relay takes over (fn-199 R1): the ticks the apex
// lived and what they hold below them, the relay's presence, and the
// failure's lead past its bound.
struct Failed {
    lived: u32,
    below: u32,
    relayed: f32,
    stopped: f32,
}

// The rest of a failed unit its relay takes over (`Shoot::fail`), once
// the unit has grown the share the apex lived: the relay grows the rest
// of the cycle, or stops at once where the lived part aborted.
fn failed_relay(apex_in: Apex, cycle: u32, gone: u32, whole: bool, f: Failed, decide: vec2<u32>, after: Persists) -> Op {
    var apex = apex_in;
    let s = frame_of(apex.axis);
    apex.spent += f.lived;
    set_fr(s, A_END, cycle + 1u);
    set_fr(s, A_SPENT, apex.spent);
    let m = Made(presence(f.stopped, 2.0), FAILED, whole, f.below);
    var o = op_relay(apex, gone + f.lived, apex.spent, array<vec2<u32>, 2>(ONE64, ONE64), m);
    if !after.ok {
        o.then_stop = true;
        o.stopped = after.value;
        o.key = decide;
    }
    return o;
}
