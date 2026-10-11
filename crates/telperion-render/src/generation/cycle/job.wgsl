// fn-256 task 2 (DESIGN.md section 10): a job's state between the phase
// kernels. A job is one living apex's growth in the cycle, or one waking
// bud's: an invocation of `units.wgsl`'s phases, its place in its range
// `cfg.base` on. Its operation, the stack of what a move leaves to do,
// its counts of the records it appended, the buds it wakes on, the unit
// it grows and its frames live in `counts` from `cfg.job_at`, a column a
// word, `cfg.job_cap` jobs apart; each phase loads what it reads into the
// private words below and keeps what it changed.

const J_OP: u32 = 0u;
const J_AXIS: u32 = 1u;
const J_SPENT: u32 = 2u;
const J_USED: u32 = 3u;
const J_OSPENT: u32 = 4u;
const J_STOPPED: u32 = 5u;
const J_KEY: u32 = 6u;
const J_BLEND: u32 = 8u;
const J_WHY: u32 = 9u;
const J_WHOLE: u32 = 10u;
const J_MBELOW: u32 = 11u;
const J_MADE0: u32 = 12u;
const J_MADE1: u32 = 14u;
const J_THEN: u32 = 16u;
const J_DEPTH: u32 = 17u;
// The records appended: axes, phytomers, units, sleeping buds, living
// apexes, woken buds; the fault, the frames open, the living apex last
// appended and whether it is written yet.
const J_COUNTS: u32 = 18u;
const J_FAULT: u32 = 24u;
const J_FRAMES: u32 = 25u;
const J_LAST_AXIS: u32 = 26u;
const J_LAST_SPENT: u32 = 27u;
const J_PENDING: u32 = 28u;
// A waking bud's: the next of the axes it wakes on, how many, its row,
// and the stage it wakes in, its time there and the stages it passed.
const J_WOKE: u32 = 29u;
const J_CARRIED_N: u32 = 30u;
const J_ROW: u32 = 31u;
const J_G: u32 = 32u;
// The unit `decide` laid out for the zones and `after`: its record, its
// frame, whether it dies, its step (share, ends, used), its cut, the
// share its zones grow, its keys, what came before it (gone, whole,
// first unit's ticks below), a failure's lived ticks, below, relay and
// stop, its persistence, and the apex's spent before it.
const J_REC: u32 = 35u;
const J_FRAME: u32 = 36u;
const J_DIES: u32 = 37u;
const J_ST: u32 = 38u;
const J_CUT: u32 = 41u;
const J_GROW: u32 = 42u;
const J_UKEY: u32 = 43u;
const J_DECIDE: u32 = 45u;
const J_GONE: u32 = 47u;
const J_WHOLEF: u32 = 48u;
const J_BELOW: u32 = 49u;
const J_FAILED: u32 = 50u;
const J_AFTER: u32 = 54u;
const J_IN_SPENT: u32 = 56u;
// What the unit's survival decided: its survival, threshold and draw,
// whether a relay takes a failure over, and the key a move draws by.
const J_V: u32 = 57u;
const J_SURVIVES: u32 = 58u;
const J_D: u32 = 60u;
const J_RELAYS: u32 = 62u;
const J_ENDKEY: u32 = 112u;
// What a move leaves to do (`After`), six words an entry.
const J_STACK: u32 = 64u;
const STACK: u32 = 8u;
// The axes a waking bud's bearer carries it to, two words each.
const J_CARRIED: u32 = 114u;
const CARRIED: u32 = 32u;
// The frames, `FW` words each: an axis's columns, then its first unit.
const J_FRAME0: u32 = 178u;
const FW: u32 = 35u;
const FRAMES: u32 = 6u;
// The frame a lateral is made in, private to its invocation.
const SCRATCH: u32 = FRAMES;
const JW: u32 = J_FRAME0 + FRAMES * FW;

// An operation's kind; a job past its last: done, or a waking bud's next
// axis to wake on.
const OP_NONE: u32 = 0u;
const OP_ADVANCE: u32 = 1u;
const OP_STOP: u32 = 2u;
const OP_RELAY: u32 = 3u;
const OP_MOVE: u32 = 4u;
const OP_GROWN: u32 = 5u;
const OP_WOKE: u32 = 6u;
const OP_DONE: u32 = 7u;
const OP_UNIT: u32 = 8u;
const OP_FAILS: u32 = 9u;
const OP_DIES: u32 = 10u;
const OP_LIVED: u32 = 11u;
const OP_MOVED: u32 = 12u;

var<private> job: u32;

fn jat(f: u32) -> u32 {
    return cfg.job_at + f * cfg.job_cap + job;
}

fn jw(f: u32) -> u32 {
    return counts[jat(f)];
}

fn set_jw(f: u32, v: u32) {
    counts[jat(f)] = v;
}

fn jw64(f: u32) -> vec2<u32> {
    return vec2<u32>(jw(f), jw(f + 1u));
}

fn set_jw64(f: u32, v: vec2<u32>) {
    set_jw(f, v.x);
    set_jw(f + 1u, v.y);
}

fn jwf(f: u32) -> f32 {
    return bitcast<f32>(jw(f));
}

fn set_jwf(f: u32, v: f32) {
    set_jw(f, bitcast<u32>(v));
}

// Job `k` of the range taken up: its counts, its frames and its bases.
fn take_job(k: u32) {
    job = k;
    emitting = cfg.emit != 0u;
    n_axes = jw(J_COUNTS);
    n_phyts = jw(J_COUNTS + 1u);
    n_units = jw(J_COUNTS + 2u);
    n_sleep = jw(J_COUNTS + 3u);
    n_next = jw(J_COUNTS + 4u);
    n_woken = jw(J_COUNTS + 5u);
    fault = jw(J_FAULT);
    n_frames = jw(J_FRAMES);
    last_axis = jw(J_LAST_AXIS);
    last_spent = jw(J_LAST_SPENT);
    pending = jw(J_PENDING) != 0u;
    bases(cfg.base + k, emitting, cfg.wake != 0u);
}

// The job's counts and frames kept for the next phase.
fn keep_job() {
    set_jw(J_COUNTS, n_axes);
    set_jw(J_COUNTS + 1u, n_phyts);
    set_jw(J_COUNTS + 2u, n_units);
    set_jw(J_COUNTS + 3u, n_sleep);
    set_jw(J_COUNTS + 4u, n_next);
    set_jw(J_COUNTS + 5u, n_woken);
    set_jw(J_FAULT, fault);
    set_jw(J_FRAMES, n_frames);
    set_jw(J_LAST_AXIS, last_axis);
    set_jw(J_LAST_SPENT, last_spent);
    set_jw(J_PENDING, u32(pending));
}

// A fresh job: nothing appended, no frame open.
fn open_job(k: u32) {
    job = k;
    for (var f = J_COUNTS; f <= J_PENDING; f++) {
        set_jw(f, 0u);
    }
    set_jw(J_DEPTH, 0u);
    set_jw(J_WOKE, 0u);
    set_jw(J_CARRIED_N, 0u);
}

// The operation a job runs next, and the one it keeps.
fn op_of() -> Op {
    var o = op(jw(J_OP), Apex(jw(J_AXIS), jw(J_SPENT)));
    o.used = jw(J_USED);
    o.spent = jw(J_OSPENT);
    o.stopped = jwf(J_STOPPED);
    o.key = jw64(J_KEY);
    o.m = Made(jwf(J_BLEND), jw(J_WHY), jw(J_WHOLE) != 0u, jw(J_MBELOW));
    o.made0 = jw64(J_MADE0);
    o.made1 = jw64(J_MADE1);
    o.then_stop = jw(J_THEN) != 0u;
    return o;
}

fn keep_op(o: Op) {
    set_jw(J_OP, o.kind);
    set_jw(J_AXIS, o.apex.axis);
    set_jw(J_SPENT, o.apex.spent);
    set_jw(J_USED, o.used);
    set_jw(J_OSPENT, o.spent);
    set_jwf(J_STOPPED, o.stopped);
    set_jw64(J_KEY, o.key);
    set_jwf(J_BLEND, o.m.blend);
    set_jw(J_WHY, o.m.why);
    set_jw(J_WHOLE, u32(o.m.whole));
    set_jw(J_MBELOW, o.m.below);
    set_jw64(J_MADE0, o.made0);
    set_jw64(J_MADE1, o.made1);
    set_jw(J_THEN, u32(o.then_stop));
}
