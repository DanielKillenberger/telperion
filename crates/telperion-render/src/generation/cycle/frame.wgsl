// One invocation's work (DESIGN.md section 2.1): the axes it advances or
// makes, held in private frames, and the records it appends, counted in
// both modes and written at their scanned places in emit mode alone. A
// lateral is written as it is made and never read again; every other
// axis it touches stays in a frame until the invocation ends.

const FRAMES: u32 = 16u;
// The frame a lateral is made in, written at once.
const SCRATCH: u32 = FRAMES;
// The axis columns past a frame's words: its first unit's record.
const A_HEAD: u32 = AW;
const AXES_COLUMNS: u32 = AW + 1u;

var<private> frame: array<u32, 578>;
var<private> n_frames: u32;
// What went wrong, by bit: a frame too many (1), an axis no frame holds
// (2), a zone of more nodes than a unit sorts (4), more buds woken on
// one sleeper than it holds (8), a stack too deep (16), a loop past its
// bound (32).
var<private> fault: u32;
var<private> emitting: bool;

// The records appended so far and where the first of each goes.
var<private> n_axes: u32;
var<private> n_phyts: u32;
var<private> n_units: u32;
var<private> n_sleep: u32;
var<private> n_next: u32;
var<private> n_woken: u32;
var<private> base_axes: u32;
var<private> base_phyts: u32;
var<private> base_units: u32;
var<private> base_sleep: u32;
var<private> base_next: u32;
var<private> base_woken: u32;
// The living apex last appended, which `carry_on` may take back, so it
// is written only once another follows it or the invocation ends: taken
// back, its slot is past the invocation's count, the next one's first.
var<private> last_axis: u32;
var<private> last_spent: u32;
var<private> pending: bool;

struct Apex {
    axis: u32,
    spent: u32,
}

fn begin(emit: bool) {
    emitting = emit;
    n_frames = 0u;
    fault = 0u;
    n_axes = 0u;
    n_phyts = 0u;
    n_units = 0u;
    n_sleep = 0u;
    n_next = 0u;
    n_woken = 0u;
    pending = false;
}

fn fr(s: u32, f: u32) -> u32 {
    return frame[s * AW + f];
}

fn set_fr(s: u32, f: u32, v: u32) {
    frame[s * AW + f] = v;
}

fn fr64(s: u32, f: u32) -> vec2<u32> {
    return vec2<u32>(fr(s, f), fr(s, f + 1u));
}

fn set_fr64(s: u32, f: u32, v: vec2<u32>) {
    set_fr(s, f, v.x);
    set_fr(s, f + 1u, v.y);
}

// The frame holding axis `i`.
fn frame_of(i: u32) -> u32 {
    for (var s = 0u; s < n_frames; s++) {
        if fr(s, 0u) == i {
            return s;
        }
    }
    fault |= 2u;
    return SCRATCH;
}

fn held(i: u32) -> bool {
    for (var s = 0u; s < n_frames; s++) {
        if fr(s, 0u) == i {
            return true;
        }
    }
    return false;
}

// Axis `i`'s word `f`, from its frame where one holds it.
fn axis_word(i: u32, f: u32) -> u32 {
    if held(i) {
        return fr(frame_of(i), f);
    }
    return ax(i, f);
}

fn axis_word64(i: u32, f: u32) -> vec2<u32> {
    return vec2<u32>(axis_word(i, f), axis_word(i, f + 1u));
}

// The frame of axis `i` as the tree holds it, its first unit in `head`.
fn load(i: u32) -> u32 {
    let s = open_frame(i);
    for (var f = 1u; f < AW; f++) {
        set_fr(s, f, ax(i, f));
    }
    heads[s] = ax(i, A_HEAD);
    return s;
}

var<private> heads: array<u32, 17>;

// A new frame for axis `i`, every word none but its index.
fn open_frame(i: u32) -> u32 {
    if n_frames >= FRAMES {
        fault |= 1u;
        return SCRATCH;
    }
    let s = n_frames;
    n_frames += 1u;
    clear(s, i);
    return s;
}

fn clear(s: u32, i: u32) {
    for (var f = 0u; f < AW; f++) {
        set_fr(s, f, 0u);
    }
    set_fr(s, 0u, i);
    set_fr(s, A_SUCC0, NONE);
    set_fr(s, A_SUCC1, NONE);
    set_fr(s, A_TAIL, NONE);
    heads[s] = NONE;
}

// Frame `s` written to its axis's columns.
fn store(s: u32) {
    let i = fr(s, 0u);
    if !emitting || i >= cfg.axes_cap {
        return;
    }
    for (var f = 1u; f < AW; f++) {
        set_ax(i, f, fr(s, f));
    }
    set_ax(i, A_HEAD, heads[s]);
}

fn store_all() {
    for (var s = 0u; s < n_frames; s++) {
        store(s);
    }
}

// The index the next axis made takes.
fn next_index() -> u32 {
    return base_axes + n_axes;
}

fn push_next(axis: u32, spent: u32) {
    flush_next();
    n_next += 1u;
    last_axis = axis;
    last_spent = spent;
    pending = true;
}

// The living apex last appended written at its place.
fn flush_next() {
    let k = base_next + n_next - 1u;
    if pending && emitting && k < cfg.next_cap {
        set_next(k, last_axis, last_spent);
    }
    pending = false;
}

// The apex last appended taken back (`carry_on`, `fail`, `woke`): the
// bud just sprouted, never written.
fn pop_next() -> Apex {
    n_next -= 1u;
    pending = false;
    return Apex(last_axis, last_spent);
}

fn add_successor(s: u32, at: u32) {
    if fr(s, A_SUCC0) == NONE {
        set_fr(s, A_SUCC0, at);
    } else {
        set_fr(s, A_SUCC1, at);
    }
}

// A unit appended to the axis of frame `s`: its record.
fn push_unit(s: u32, survive: f32, share: u32, size: u32, shade: array<vec2<u32>, 2>) -> u32 {
    let r = base_units + n_units;
    n_units += 1u;
    let tail = fr(s, A_TAIL);
    if emitting && r < cfg.units_cap {
        set_unit(r, U_AXIS, fr(s, 0u));
        set_unit(r, U_SURVIVE, bitcast<u32>(survive));
        set_unit(r, U_PERSIST, bitcast<u32>(1.0));
        set_unit(r, U_SHARE, share);
        set_unit(r, U_SIZE, size);
        for (var k = 0u; k < 2u; k++) {
            set_unit(r, U_SHADE + 2u * k, shade[k].x);
            set_unit(r, U_SHADE + 2u * k + 1u, shade[k].y);
        }
        set_unit(r, U_PREV, tail);
        set_unit(r, U_NEXT, NONE);
        set_unit(r, U_FIRST, base_phyts + n_phyts);
        set_unit(r, U_COUNT, 0u);
        if tail != NONE {
            set_unit(tail, U_NEXT, r);
        }
    }
    if tail == NONE {
        heads[s] = r;
    }
    set_fr(s, A_TAIL, r);
    set_fr(s, A_UNITS, fr(s, A_UNITS) + 1u);
    set_fr(s, A_GROWN, fr(s, A_GROWN) + share);
    return r;
}

// Unit `r`'s phytomers, once its unit has grown, and its persistence.
fn unit_grown(r: u32) {
    if emitting && r < cfg.units_cap {
        set_unit(r, U_COUNT, base_phyts + n_phyts - unit(r, U_FIRST));
    }
}

fn unit_persist(r: u32, persist: f32) {
    if emitting && r < cfg.units_cap {
        set_unit(r, U_PERSIST, bitcast<u32>(persist));
    }
}

fn push_phyt(axis: u32, key: vec2<u32>, cycle: u32, node: f32) {
    let p = base_phyts + n_phyts;
    n_phyts += 1u;
    if emitting && p < cfg.phyts_cap {
        set_phyt(p, P_KEY, key.x);
        set_phyt(p, P_KEY + 1u, key.y);
        set_phyt(p, P_AXIS, axis);
        set_phyt(p, P_CYCLE, cycle);
        set_phyt(p, P_NODE, bitcast<u32>(node));
    }
}

// A sleeping bud's row: its words from S_WAKE to S_REC.
fn push_sleeper(row: array<u32, 14>) {
    let k = base_sleep + n_sleep;
    n_sleep += 1u;
    if emitting && k < cfg.sleep_cap {
        for (var f = 0u; f < 14u; f++) {
            set_sleeper(k, f, row[f]);
        }
    }
}

fn push_woken(row: array<u32, 5>) {
    let k = base_woken + n_woken;
    n_woken += 1u;
    if emitting && k < cfg.woken_cap {
        for (var f = 0u; f < 5u; f++) {
            set_woken(k, f, row[f]);
        }
    }
}
