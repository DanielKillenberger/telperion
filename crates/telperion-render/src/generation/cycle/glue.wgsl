// fn-256 task 2: what moves between the cycle's phases over the one
// resident state (`state.rs`), each a view or a copy, none a decision:
// - `tree_main`: the hierarchy's tree columns from the axes and pencils;
// - `scatter_main`: the balance's and allotment's memory, kept share and
//   size into the pencils, and each apex it shed ended;
// - `inputs_main`: the units' living apexes and the pencils they read;
// - `born_main`: a new axis's layout columns, its reals as the f32s the
//   layout reads (`as f32`, to nearest);
// - `grew_mark_main`, `grew_gather_main`: the axes that grew, by index;
// - `points_main`: the cycle's leaves and the living apexes' tips;
// - `lit_main`: each living apex's light into its pencil;
// - `relive_main`: the living apexes a cycle leaves as the next one's;
// - `fresh_main`: a new phytomer's scale and size whole, as it grows.
// The software f64 (`soft64.wgsl`) widens and narrows reals exactly.

struct Cfg {
    n: u32,
    row: u32,
    cycle: u32,
    // Axes laid (pencils), and the strides: axes, units, phytomers.
    laid: u32,
    axes: u32,
    units: u32,
    phyts: u32,
    phyt_at: u32,
    // The hierarchy's living columns' stride, and the units' list of
    // the living a cycle leaves.
    live_stride: u32,
    next_at: u32,
    next_cap: u32,
    // The first item (a new axis, a leaf's phytomer), the leaves, whether
    // the units' living come before the balance (none laid), and the
    // living's count.
    first: u32,
    leaves: u32,
    from_in: u32,
    live: u32,
    pad0: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read_write> axes: array<u32>;
@group(0) @binding(2) var<storage, read_write> pencils: array<u32>;
@group(0) @binding(3) var<storage, read_write> units: array<u32>;
// The phytomers' columns the layout writes (`state.rs`'s `laid`).
@group(0) @binding(4) var<storage, read_write> laid: array<u32>;
@group(0) @binding(5) var<storage, read_write> lists: array<u32>;
@group(0) @binding(6) var<storage, read_write> live: array<u32>;
@group(0) @binding(7) var<storage, read_write> aux: array<u32>;
@group(0) @binding(8) var<storage, read_write> aux2: array<u32>;

const NONE: u32 = 0xffffffffu;
// The axes' columns read here (`state.rs`).
const A_PA: u32 = 1u;
const A_ORIGIN: u32 = 3u;
const A_PARENT: u32 = 4u;
const A_NODE: u32 = 5u;
const A_LINEAGE: u32 = 6u;
const A_WHORL: u32 = 12u;
const A_BLEND: u32 = 14u;
const A_END: u32 = 16u;
const A_PHYTS: u32 = 17u;
const A_B0: u32 = 19u;
const A_B1: u32 = 21u;
const A_TAIL: u32 = 33u;
const A_LAYOUT: u32 = 35u;
const A_BASE: u32 = 41u;
const LAYOUT_COLUMNS: u32 = 16u;
// A unit's links and phytomers; a phytomer's tip and leaf area.
const U_PREV: u32 = 9u;
const U_FIRST: u32 = 11u;
const U_COUNT: u32 = 12u;
const PH_TIP: u32 = 0u;
const PH_SCALE: u32 = 9u;
const PH_SIZE: u32 = 11u;
const PH_AREA: u32 = 13u;
// A pencil's scalars the balance and the units read.
const P_BASE_SCALE: u32 = 48u;
const P_GOING: u32 = 49u;
const P_WOOD: u32 = 54u;
const P_LIGHT: u32 = 55u;
const P_SIZE: u32 = 56u;
const P_MEMORY: u32 = 57u;
const P_KEPT: u32 = 58u;
// The hierarchy's tree columns (`hier_cols.wgsl`) and the sums it leaves
// (`hier_cycle.wgsl`), and its living's columns.
const T_PARENT: u32 = 0u;
const T_NODE: u32 = 1u;
const T_PA: u32 = 2u;
const T_LINEAGE: u32 = 3u;
const T_WOOD: u32 = 5u;
const T_LIGHT: u32 = 7u;
const T_BASE: u32 = 9u;
const T_RUNNING: u32 = 10u;
const T_KEPT: u32 = 11u;
const T_MEMORY: u32 = 12u;
const T_SIZE: u32 = 13u;
const S_MEMORY: u32 = 6u;
const S_KEPT: u32 = 7u;
const S_SIZE: u32 = 8u;
const S_GONE: u32 = 18u;
const V_AXIS: u32 = 0u;
const V_SPENT: u32 = 1u;
const V_OUT_AXIS: u32 = 4u;
const V_OUT_SPENT: u32 = 5u;
// The layout's tally, before its list of the axes that grew.
const LAY_TALLY: u32 = 32u;

fn index(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

fn ax(i: u32, c: u32) -> u32 {
    return axes[c * cfg.axes + i];
}

fn set_ax(i: u32, c: u32, v: u32) {
    axes[c * cfg.axes + i] = v;
}

fn pn(i: u32, c: u32) -> u32 {
    return pencils[c * cfg.axes + i];
}

fn ax64(i: u32, c: u32) -> vec2<u32> {
    return vec2<u32>(ax(i, c), ax(i, c + 1u));
}

fn lv(c: u32, k: u32) -> u32 {
    return c * cfg.live_stride + k;
}

// Each laid axis's tree columns: its bearer, its node (none at the tip),
// PA and lineage, its wood and light as f64s, and its pencil's f32s.
@compute @workgroup_size(64)
fn tree_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.laid {
        return;
    }
    let s = cfg.axes;
    let kind = ax(i, A_ORIGIN) & 0xffu;
    aux[T_PARENT * s + i] = ax(i, A_PARENT);
    aux[T_NODE * s + i] = select(NONE, ax(i, A_NODE), kind == 1u);
    aux[T_PA * s + i] = ax(i, A_PA);
    aux[T_LINEAGE * s + i] = ax(i, A_LINEAGE);
    aux[(T_LINEAGE + 1u) * s + i] = ax(i, A_LINEAGE + 1u);
    let wood = f64_from_f32(bitcast<f32>(pn(i, P_WOOD)));
    let light = f64_from_f32(bitcast<f32>(pn(i, P_LIGHT)));
    aux[T_WOOD * s + i] = wood.x;
    aux[(T_WOOD + 1u) * s + i] = wood.y;
    aux[T_LIGHT * s + i] = light.x;
    aux[(T_LIGHT + 1u) * s + i] = light.y;
    aux[T_BASE * s + i] = pn(i, P_BASE_SCALE);
    aux[T_RUNNING * s + i] = pn(i, P_GOING);
    aux[T_KEPT * s + i] = pn(i, P_KEPT);
    aux[T_MEMORY * s + i] = pn(i, P_MEMORY);
    aux[T_SIZE * s + i] = pn(i, P_SIZE);
}

// The balance's and allotment's words into each laid axis's pencil (the
// sums start from the pencil's, so an axis they leave keeps its own),
// and each apex the walk shed ended a cycle back.
@compute @workgroup_size(64)
fn scatter_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.laid {
        return;
    }
    let s = cfg.axes;
    pencils[P_MEMORY * s + i] = aux[S_MEMORY * s + i];
    pencils[P_KEPT * s + i] = aux[S_KEPT * s + i];
    pencils[P_SIZE * s + i] = aux[S_SIZE * s + i];
    if aux[S_GONE * s + i] != 0u {
        set_ax(i, A_END, cfg.cycle);
    }
}

// The units' inputs (`arena.wgsl`): the living apexes (axis, time), then
// each laid axis's light and kept share as f32s, and its size and light
// as f64 words.
@compute @workgroup_size(64)
fn inputs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k < cfg.live {
        let a = select(V_OUT_AXIS, V_AXIS, cfg.from_in != 0u);
        let t = select(V_OUT_SPENT, V_SPENT, cfg.from_in != 0u);
        aux[2u * k] = live[lv(a, k)];
        aux[2u * k + 1u] = live[lv(t, k)];
    }
    if k < cfg.laid {
        let at = 2u * cfg.live;
        let n = cfg.laid;
        let light = pn(k, P_LIGHT);
        aux[at + k] = light;
        aux[at + n + k] = pn(k, P_KEPT);
        let size = f64_from_f32(bitcast<f32>(pn(k, P_SIZE)));
        let wide = f64_from_f32(bitcast<f32>(light));
        aux[at + 2u * n + k] = size.x;
        aux[at + 3u * n + k] = size.y;
        aux[at + 4u * n + k] = wide.x;
        aux[at + 5u * n + k] = wide.y;
    }
}

// A new axis's layout columns: its whorl, blend and birth draws to the
// nearest f32, and its frame, rank, phase and faults none.
@compute @workgroup_size(64)
fn born_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.n {
        return;
    }
    let i = cfg.first + k;
    for (var c = 0u; c < LAYOUT_COLUMNS; c++) {
        set_ax(i, A_LAYOUT + c, 0u);
    }
    set_ax(i, A_LAYOUT, bitcast<u32>(f64_to_f32(ax64(i, A_WHORL))));
    set_ax(i, A_LAYOUT + 1u, bitcast<u32>(f64_to_f32(ax64(i, A_BLEND))));
    set_ax(i, A_LAYOUT + 2u, bitcast<u32>(f64_to_f32(ax64(i, A_B0))));
    set_ax(i, A_LAYOUT + 3u, bitcast<u32>(f64_to_f32(ax64(i, A_B1))));
}

// Each living apex at the cycle's start that was laid, flagged by its
// axis (`aux`: the flags, then their copy the scan takes in place).
@compute @workgroup_size(64)
fn grew_mark_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.live {
        return;
    }
    let a = live[lv(V_AXIS, k)];
    if a < cfg.laid {
        aux[a] = 1u;
        aux[cfg.axes + a] = 1u;
    }
}

// The flagged axes listed in index order for the layout's first draw.
@compute @workgroup_size(64)
fn grew_gather_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = index(id);
    if i >= cfg.laid || aux[i] == 0u {
        return;
    }
    aux2[LAY_TALLY + aux[cfg.axes + i]] = i;
}

// Axis `a`'s tip (`Grower::tip`): its last phytomer's, or its base.
fn tip_of(a: u32) -> vec3<f32> {
    var r = ax(a, A_TAIL);
    // cap: the axis's units.
    while r != NONE && units[U_COUNT * cfg.units + r] == 0u {
        r = units[U_PREV * cfg.units + r];
    }
    if r == NONE {
        let b = A_BASE;
        return vec3<f32>(bitcast<f32>(ax(a, b)), bitcast<f32>(ax(a, b + 1u)), bitcast<f32>(ax(a, b + 2u)));
    }
    let p = units[U_FIRST * cfg.units + r] + units[U_COUNT * cfg.units + r] - 1u;
    let at = PH_TIP * cfg.phyts + p;
    let s = cfg.phyts;
    return vec3<f32>(bitcast<f32>(laid[at]), bitcast<f32>(laid[at + s]), bitcast<f32>(laid[at + 2u * s]));
}

// The light's points (`light.wgsl`): the cycle's leaves (x, y, z, area),
// the phytomers laid in it, then the living apexes' tips.
@compute @workgroup_size(64)
fn points_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.n {
        return;
    }
    var p = vec4<f32>(0.0);
    if k < cfg.leaves {
        let r = cfg.first + k;
        let at = PH_TIP * cfg.phyts + r;
        let s = cfg.phyts;
        let area = laid[PH_AREA * cfg.phyts + r];
        p = vec4<f32>(bitcast<f32>(laid[at]), bitcast<f32>(laid[at + s]), bitcast<f32>(laid[at + 2u * s]), bitcast<f32>(area));
    } else {
        p = vec4<f32>(tip_of(lists[cfg.next_at + k - cfg.leaves]), 0.0);
    }
    for (var c = 0u; c < 4u; c++) {
        aux[4u * k + c] = bitcast<u32>(p[c]);
    }
}

// Each living apex's read into its pencil's light.
@compute @workgroup_size(64)
fn lit_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.n {
        return;
    }
    pencils[P_LIGHT * cfg.axes + lists[cfg.next_at + k]] = aux[k];
}

// The living apexes the cycle leaves as the next one's, before its
// balance; a time's high word none (it holds ticks).
@compute @workgroup_size(64)
fn relive_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.n {
        return;
    }
    live[lv(V_AXIS, k)] = lists[cfg.next_at + k];
    live[lv(V_SPENT, k)] = lists[cfg.next_at + cfg.next_cap + k];
    live[lv(V_SPENT + 1u, k)] = 0u;
}

// Each phytomer the cycle grew as `Grower` grows it: scale and size
// whole, its frame, grown and rank none.
@compute @workgroup_size(64)
fn fresh_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = index(id);
    if k >= cfg.n {
        return;
    }
    let r = cfg.first + k;
    laid[PH_SCALE * cfg.phyts + r] = bitcast<u32>(1.0);
    laid[PH_SIZE * cfg.phyts + r] = bitcast<u32>(1.0);
}
