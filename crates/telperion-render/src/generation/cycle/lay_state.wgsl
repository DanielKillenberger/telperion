// fn-256 task 2: the layout's reads and writes of the one resident state
// (`state.rs`): an axis's columns, a phytomer's in the arena, a pencil's,
// a unit's record, and an axis's phytomer found through its units'
// records. Part of the layout's shader.

fn ax(i: u32, c: u32) -> u32 {
    return axes[c * cfg.axes + i];
}

fn axf(i: u32, c: u32) -> f32 {
    return bitcast<f32>(ax(i, c));
}

fn ax3(i: u32, c: u32) -> vec3<f32> {
    return vec3<f32>(axf(i, c), axf(i, c + 1u), axf(i, c + 2u));
}

fn set_ax(i: u32, c: u32, v: u32) {
    axes[c * cfg.axes + i] = v;
}

fn set_ax3(i: u32, c: u32, v: vec3<f32>) {
    for (var k = 0u; k < 3u; k++) {
        set_ax(i, c + k, bitcast<u32>(v[k]));
    }
}

fn kind_of(i: u32) -> u32 {
    return ax(i, A_ORIGIN) & 0xffu;
}

fn lineage_of(i: u32) -> vec2<u32> {
    return vec2<u32>(ax(i, A_LINEAGE), ax(i, A_LINEAGE + 1u));
}

fn ph(r: u32, c: u32) -> u32 {
    return arena[cfg.phyt_at + c * cfg.phyts + r];
}

fn phf(r: u32, c: u32) -> f32 {
    return bitcast<f32>(ph(r, c));
}

fn ph3(r: u32, c: u32) -> vec3<f32> {
    return vec3<f32>(phf(r, c), phf(r, c + 1u), phf(r, c + 2u));
}

fn set_ph(r: u32, c: u32, v: f32) {
    arena[cfg.phyt_at + c * cfg.phyts + r] = bitcast<u32>(v);
}

fn set_ph3(r: u32, c: u32, v: vec3<f32>) {
    for (var k = 0u; k < 3u; k++) {
        set_ph(r, c + k, v[k]);
    }
}

fn pn(i: u32, c: u32) -> u32 {
    return pencils[c * cfg.axes + i];
}

fn pnf(i: u32, c: u32) -> f32 {
    return bitcast<f32>(pn(i, c));
}

fn pn3(i: u32, c: u32) -> vec3<f32> {
    return vec3<f32>(pnf(i, c), pnf(i, c + 1u), pnf(i, c + 2u));
}

fn set_pn(i: u32, c: u32, v: u32) {
    pencils[c * cfg.axes + i] = v;
}

fn set_pnf(i: u32, c: u32, v: f32) {
    set_pn(i, c, bitcast<u32>(v));
}

fn set_pn3(i: u32, c: u32, v: vec3<f32>) {
    for (var k = 0u; k < 3u; k++) {
        set_pnf(i, c + k, v[k]);
    }
}

fn fm(pa: u32, k: u32) -> f32 {
    return form[2u + pa * FORM + k];
}

fn unit_word(r: u32, c: u32) -> u32 {
    return arena[c * cfg.units + r];
}

// A unit's survival and persistence, the share of a node's length its
// nodes grew at (ticks, at most a cycle, so exact in f32), and its
// phytomers' size from its shade, an f64 that holds an f32.
fn unit_survive(r: u32) -> f32 {
    return bitcast<f32>(unit_word(r, UN_SURVIVE));
}

fn unit_persist(r: u32) -> f32 {
    return bitcast<f32>(unit_word(r, UN_PERSIST));
}

fn unit_size(r: u32) -> f32 {
    return f32(unit_word(r, UN_SIZE)) * (1.0 / TICKS);
}

fn unit_shade(r: u32) -> f32 {
    return narrow(vec2<u32>(unit_word(r, UN_SHADE), unit_word(r, UN_SHADE + 1u)));
}

// An f64 that holds an f32 (zero or normal) as that f32; a fault where
// it holds none.
fn narrow(v: vec2<u32>) -> f32 {
    let e = (v.y >> 20u) & 0x7ffu;
    let sign = v.y & 0x80000000u;
    if e == 0u && (v.y & 0xfffffu) == 0u && v.x == 0u {
        return bitcast<f32>(sign);
    }
    let e32 = i32(e) - 1023 + 127;
    if (v.x & 0x1fffffffu) != 0u || e32 <= 0 || e32 >= 255 {
        status |= FAULT_NARROW;
    }
    let m = ((v.y & 0xfffffu) << 3u) | (v.x >> 29u);
    return bitcast<f32>(sign | (u32(e32) << 23u) | m);
}

// The record of axis `a`'s unit `k`: walked back from its last.
fn unit_of(a: u32, k: u32) -> u32 {
    var r = ax(a, A_TAIL);
    var j = ax(a, A_UNITS_N);
    // cap: the axis's units.
    while j > k + 1u && r != NONE {
        r = unit_word(r, UN_PREV);
        j -= 1u;
    }
    return r;
}

// The phytomers axis `a` holds.
fn nphyt(a: u32) -> u32 {
    return ax(a, A_PHYTS);
}

// The unit the last `prow` stood in: its axis, record, and the node of
// its first phytomer. Reset at each entry.
var<private> cur_axis: u32;
var<private> cur_unit: u32;
var<private> cur_ord: u32;

fn forget() {
    cur_axis = NONE;
}

// The arena row of axis `a`'s phytomer `j`: its unit found by stepping
// from the last one stood in along the axis's records, so a walk along
// the axis steps a unit at a time.
fn prow(a: u32, j: u32) -> u32 {
    if a != cur_axis {
        cur_unit = ax(a, A_TAIL);
        if cur_unit == NONE {
            status |= FAULT_UNREAD;
            return 0u;
        }
        cur_axis = a;
        cur_ord = ax(a, A_PHYTS) - unit_word(cur_unit, UN_COUNT);
    }
    // cap: the axis's units, back.
    while j < cur_ord {
        cur_unit = unit_word(cur_unit, UN_PREV);
        if cur_unit == NONE {
            status |= FAULT_UNREAD;
            forget();
            return 0u;
        }
        cur_ord -= unit_word(cur_unit, UN_COUNT);
    }
    // cap: the axis's units, on.
    while j >= cur_ord + unit_word(cur_unit, UN_COUNT) {
        cur_ord += unit_word(cur_unit, UN_COUNT);
        cur_unit = unit_word(cur_unit, UN_NEXT);
        if cur_unit == NONE {
            status |= FAULT_UNREAD;
            forget();
            return 0u;
        }
    }
    return unit_word(cur_unit, UN_FIRST) + (j - cur_ord);
}
