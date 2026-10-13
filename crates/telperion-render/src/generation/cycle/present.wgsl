// fn-270 R5 and R5a (`telperion_space::unfold`): the settled tree drawn a
// fraction into its last year, on the device. Every axis of this year's
// node table (`now`) is matched to last year's (`prev`) by its key (its
// lineage, kind, PA and depth); the presented table (`out`) holds, depth
// by depth, this year's axes and then last year's that are gone. Each
// organ of both years moves from last year's drawing by the fraction;
// this year's shoot extends from its base along its PA's elongation
// course and each leaf expands along its expansion course from its
// node's unfolding; a gone subtree fades by the fraction, carried with
// the wood it stood on. A depth is drawn after the one it stands on.

struct Cfg {
    // The dispatch's row of invocations, its first item and items.
    row: u32,
    base: u32,
    count: u32,
    // This year's table: its axis and phytomer strides, where its
    // phytomers start, its axes and phytomers.
    sk: u32,
    skp: u32,
    at_np: u32,
    kept: u32,
    phyts: u32,
    // Last year's, and the presented one's strides.
    psk: u32,
    pskp: u32,
    pat_np: u32,
    pkept: u32,
    osk: u32,
    oskp: u32,
    oat_np: u32,
    // The hash's slots less one, and where the work's columns start:
    // last year's keys, each axis's match either way (one past its index,
    // 0 for none), the gone flags and phytomers and their scans, last
    // year's depths' starts, what is gathered at them, and each presented
    // axis's shift.
    mask: u32,
    w_keys: u32,
    w_old: u32,
    w_now_of: u32,
    w_gone: u32,
    w_gphyt: u32,
    w_starts: u32,
    w_gather: u32,
    w_shift: u32,
    // Each axis's place in the presented table, this year's and last
    // year's gone, and the scan of a depth's children counts; then in
    // `counts` each presented axis's children, this year's children and
    // its first, and each of last year's axes' first child (`NONE` less
    // the index, so a maximum keeps the least).
    w_pos_now: u32,
    w_pos_gone: u32,
    w_scan: u32,
    c_all: u32,
    c_now: u32,
    c_first_now: u32,
    c_first_prev: u32,
    // Where the presented depth drawn starts, and the one above it.
    start_here: u32,
    start_up: u32,
    // How far into the year, in days from bud burst and as a share.
    tau: f32,
    f: f32,
    // Which table's axes a dispatch takes: 0 this year's, 1 last
    // year's; and the depth.
    which: u32,
    depth: u32,
    pad0: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> nowt: array<u32>;
@group(0) @binding(2) var<storage, read> prevt: array<u32>;
@group(0) @binding(3) var<storage, read_write> out: array<u32>;
@group(0) @binding(4) var<storage, read_write> work: array<u32>;
// The keys' hash while the years are matched, then the children counts
// (`counts`) while the presented table is laid out.
@group(0) @binding(5) var<storage, read_write> hash: array<atomic<u32>>;
@group(0) @binding(6) var<storage, read_write> leaf: array<f32>;
// Each PA's elongation and expansion in days and its internode's spread.
@group(0) @binding(7) var<storage, read> unfold: array<f32>;

const NONE: u32 = 0xffffffffu;
const SEED: u32 = 0u;
const CONTINUED: u32 = 2u;
const NA_COLS: u32 = 41u;
const NP_COLS: u32 = 22u;
const NA_KIND: u32 = 0u;
const NA_PARENT: u32 = 1u;
const NA_NODE: u32 = 2u;
const NA_LINEAGE: u32 = 5u;
const NA_PA: u32 = 7u;
const NA_PHYTS: u32 = 21u;
const NA_PSTART: u32 = 22u;
const NA_BASE: u32 = 24u;
const NA_HEADING: u32 = 27u;
const NA_SIDE: u32 = 30u;
const NA_JOINED: u32 = 33u;
const NP_KEY: u32 = 1u;
const NP_SCALE: u32 = 10u;
const NP_RADIUS: u32 = 11u;
const NP_TIP: u32 = 12u;
const NP_HEADING: u32 = 15u;
const NP_SIDE: u32 = 18u;
// A key's words: the lineage, the kind and PA, the depth.
const KEY: u32 = 4u;
// The work's fault word, set where two axes of a year share a key.
const FAULT: u32 = 0u;
const YEAR: f32 = 365.0;
const STEEP: f32 = 10.0;

fn item(id: vec3<u32>) -> u32 {
    return cfg.base + id.x + id.y * cfg.row;
}

fn na_now(c: u32, a: u32) -> u32 {
    return nowt[c * cfg.sk + a];
}

fn np_now(c: u32, p: u32) -> u32 {
    return nowt[cfg.at_np + c * cfg.skp + p];
}

fn na_prev(c: u32, a: u32) -> u32 {
    return prevt[c * cfg.psk + a];
}

fn np_prev(c: u32, p: u32) -> u32 {
    return prevt[cfg.pat_np + c * cfg.pskp + p];
}

fn na_out(c: u32, a: u32) -> u32 {
    return out[c * cfg.osk + a];
}

fn set_na(c: u32, a: u32, v: u32) {
    out[c * cfg.osk + a] = v;
}

fn np_out(c: u32, p: u32) -> u32 {
    return out[cfg.oat_np + c * cfg.oskp + p];
}

fn set_np(c: u32, p: u32, v: u32) {
    out[cfg.oat_np + c * cfg.oskp + p] = v;
}

fn f(v: u32) -> f32 {
    return bitcast<f32>(v);
}

fn b(v: f32) -> u32 {
    return bitcast<u32>(v);
}

fn now3(c: u32, p: u32) -> vec3<f32> {
    return vec3<f32>(f(np_now(c, p)), f(np_now(c + 1u, p)), f(np_now(c + 2u, p)));
}

fn prev3(c: u32, p: u32) -> vec3<f32> {
    return vec3<f32>(f(np_prev(c, p)), f(np_prev(c + 1u, p)), f(np_prev(c + 2u, p)));
}

fn out3(c: u32, p: u32) -> vec3<f32> {
    return vec3<f32>(f(np_out(c, p)), f(np_out(c + 1u, p)), f(np_out(c + 2u, p)));
}

fn set3(c: u32, p: u32, v: vec3<f32>) {
    set_np(c, p, b(v.x));
    set_np(c + 1u, p, b(v.y));
    set_np(c + 2u, p, b(v.z));
}

fn na_now3(c: u32, a: u32) -> vec3<f32> {
    return vec3<f32>(f(na_now(c, a)), f(na_now(c + 1u, a)), f(na_now(c + 2u, a)));
}

fn na_prev3(c: u32, a: u32) -> vec3<f32> {
    return vec3<f32>(f(na_prev(c, a)), f(na_prev(c + 1u, a)), f(na_prev(c + 2u, a)));
}

fn na_out3(c: u32, a: u32) -> vec3<f32> {
    return vec3<f32>(f(na_out(c, a)), f(na_out(c + 1u, a)), f(na_out(c + 2u, a)));
}

fn set_na3(c: u32, a: u32, v: vec3<f32>) {
    set_na(c, a, b(v.x));
    set_na(c + 1u, a, b(v.y));
    set_na(c + 2u, a, b(v.z));
}

// `a` moved a share `t` of the way to `z`: `a` at 0 and `z` at 1.
fn mixed(a: f32, z: f32, t: f32) -> f32 {
    if t <= 0.0 {
        return a;
    }
    if t >= 1.0 {
        return z;
    }
    return a + (z - a) * t;
}

fn mixed3(a: vec3<f32>, z: vec3<f32>, t: f32) -> vec3<f32> {
    return vec3<f32>(mixed(a.x, z.x, t), mixed(a.y, z.y, t), mixed(a.z, z.z, t));
}

// The logistic course over a span and its inverse (`unfold::course`,
// `unfold::reached`).
fn sigmoid(t: f32) -> f32 {
    return 1.0 / (1.0 + exp(-t));
}

fn course(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let lo = sigmoid(-STEEP / 2.0);
    let hi = sigmoid(STEEP / 2.0);
    return (sigmoid(STEEP * (x - 0.5)) - lo) / (hi - lo);
}

fn reached(y: f32) -> f32 {
    if y <= 0.0 {
        return 0.0;
    }
    if y >= 1.0 {
        return 1.0;
    }
    let lo = sigmoid(-STEEP / 2.0);
    let hi = sigmoid(STEEP / 2.0);
    let z = lo + y * (hi - lo);
    return clamp(0.5 + log(z / (1.0 - z)) / STEEP, 0.0, 1.0);
}

// How far a course `days` long has run `t` days after it began.
fn run(t: f32, days: f32) -> f32 {
    if days > 0.0 {
        return course(t / days);
    }
    return select(0.0, 1.0, t > 0.0);
}

fn row(pa: u32, r: u32) -> f32 {
    return unfold[pa * 3u + r];
}

// An axis's key in this year's table or last year's (`which`).
fn key_of(which: u32, a: u32) -> vec4<u32> {
    if which == 0u {
        let kind = (na_now(NA_KIND, a) & 0xffu) | (na_now(NA_PA, a) << 8u);
        return vec4<u32>(na_now(NA_LINEAGE, a), na_now(NA_LINEAGE + 1u, a), kind, cfg.depth);
    }
    let kind = (na_prev(NA_KIND, a) & 0xffu) | (na_prev(NA_PA, a) << 8u);
    return vec4<u32>(na_prev(NA_LINEAGE, a), na_prev(NA_LINEAGE + 1u, a), kind, cfg.depth);
}

fn mix32(v: u32) -> u32 {
    var z = v;
    z = (z ^ (z >> 16u)) * 0x21f0aaadu;
    z = (z ^ (z >> 15u)) * 0x735a2d97u;
    return z ^ (z >> 15u);
}

fn slot(k: vec4<u32>) -> u32 {
    return mix32(k.x ^ mix32(k.y ^ mix32(k.z ^ mix32(k.w)))) & cfg.mask;
}

// Last year's axes of a depth, each its key kept.
@compute @workgroup_size(64)
fn keys_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = item(id);
    if g >= cfg.base + cfg.count {
        return;
    }
    let k = key_of(1u, g);
    for (var i = 0u; i < 4u; i++) {
        work[cfg.w_keys + KEY * g + i] = k[i];
    }
}

fn kept_key(g: u32) -> vec4<u32> {
    let at = cfg.w_keys + KEY * g;
    return vec4<u32>(work[at], work[at + 1u], work[at + 2u], work[at + 3u]);
}

// Last year's axes placed in the hash, one past their index; two of one
// key are a fault.
@compute @workgroup_size(64)
fn insert_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = item(id);
    if g >= cfg.base + cfg.count {
        return;
    }
    let k = kept_key(g);
    var s = slot(k);
    // cap: the hash holds twice the axes, so a probe meets an empty slot.
    for (var n = 0u; n <= cfg.mask; n++) {
        let won = atomicCompareExchangeWeak(&hash[s], 0u, g + 1u);
        if won.exchanged {
            return;
        }
        if won.old_value != 0u {
            if all(kept_key(won.old_value - 1u) == k) {
                work[FAULT] = 1u;
                return;
            }
            s = (s + 1u) & cfg.mask;
        }
    }
}

// This year's axes of a depth, each matched to last year's of its key:
// `old` its axis last year, `now_of` the other way.
@compute @workgroup_size(64)
fn match_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let a = item(id);
    if a >= cfg.base + cfg.count {
        return;
    }
    let k = key_of(0u, a);
    var s = slot(k);
    var found = 0u;
    // cap: the probe ends at an empty slot, as `insert_main`'s.
    for (var n = 0u; n <= cfg.mask; n++) {
        let g = atomicLoad(&hash[s]);
        if g == 0u {
            break;
        }
        if all(kept_key(g - 1u) == k) {
            found = g;
            break;
        }
        s = (s + 1u) & cfg.mask;
    }
    work[cfg.w_old + a] = found;
    if found != 0u {
        work[cfg.w_now_of + found - 1u] = a + 1u;
    }
}

// Last year's axes: whether each is gone, and the phytomers it takes
// with it, for their scans.
@compute @workgroup_size(64)
fn gone_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let g = item(id);
    if g > cfg.pkept {
        return;
    }
    var gone = 0u;
    var phyts = 0u;
    if g < cfg.pkept && work[cfg.w_now_of + g] == 0u {
        gone = 1u;
        phyts = na_prev(NA_PHYTS, g);
    }
    work[cfg.w_gone + g] = gone;
    work[cfg.w_gphyt + g] = phyts;
}

// The presented place of last year's gone axis `g`'s bearer.
fn gone_parent(g: u32) -> u32 {
    let p = na_prev(NA_PARENT, g);
    let mine = work[cfg.w_now_of + p];
    if mine != 0u {
        return work[cfg.w_pos_now + mine - 1u];
    }
    return work[cfg.w_pos_gone + p];
}

// The axes of a depth counted onto their presented bearers: every child,
// this year's children and the first of them; and each of last year's
// axes' first child (`which` 1, every one of last year's axes).
@compute @workgroup_size(64)
fn count_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let a = item(id);
    if a >= cfg.base + cfg.count {
        return;
    }
    if cfg.which == 0u {
        let q = work[cfg.w_pos_now + na_now(NA_PARENT, a)];
        atomicAdd(&hash[cfg.c_all + q], 1u);
        atomicAdd(&hash[cfg.c_now + q], 1u);
        atomicMax(&hash[cfg.c_first_now + q], NONE - a);
        return;
    }
    atomicMax(&hash[cfg.c_first_prev + na_prev(NA_PARENT, a)], NONE - a);
    if work[cfg.w_now_of + a] == 0u {
        atomicAdd(&hash[cfg.c_all + gone_parent(a)], 1u);
    }
}

// Each axis of a depth placed: after the children of the bearers before
// its own, this year's children of a bearer before its gone ones, each
// in its year's order (`w_scan` holds the depth above's children counts,
// scanned), so a bearer's children stand together as the chains read
// them.
@compute @workgroup_size(64)
fn place_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let a = item(id);
    if a >= cfg.base + cfg.count {
        return;
    }
    if cfg.which == 0u {
        let q = work[cfg.w_pos_now + na_now(NA_PARENT, a)];
        let first = NONE - atomicLoad(&hash[cfg.c_first_now + q]);
        let before = work[cfg.w_scan + q - cfg.start_up];
        work[cfg.w_pos_now + a] = cfg.start_here + before + a - first;
        return;
    }
    if work[cfg.w_now_of + a] != 0u {
        return;
    }
    let q = gone_parent(a);
    let first = NONE - atomicLoad(&hash[cfg.c_first_prev + na_prev(NA_PARENT, a)]);
    let before = work[cfg.w_scan + q - cfg.start_up] + atomicLoad(&hash[cfg.c_now + q]);
    work[cfg.w_pos_gone + a] = cfg.start_here + before + work[cfg.w_gone + a] - work[cfg.w_gone + first];
}

// Each axis of a depth as presented, at its place (`place_main`): its
// bearer renumbered and a gone axis's phytomers after this year's.
@compute @workgroup_size(64)
fn axes_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let a = item(id);
    if a >= cfg.base + cfg.count {
        return;
    }
    if cfg.which == 0u {
        let to = work[cfg.w_pos_now + a];
        for (var c = 0u; c < NA_COLS; c++) {
            set_na(c, to, na_now(c, a));
        }
        if (na_now(NA_KIND, a) & 0xffu) != SEED {
            set_na(NA_PARENT, to, work[cfg.w_pos_now + na_now(NA_PARENT, a)]);
        }
        return;
    }
    if work[cfg.w_now_of + a] != 0u {
        return;
    }
    let to = work[cfg.w_pos_gone + a];
    for (var c = 0u; c < NA_COLS; c++) {
        set_na(c, to, na_prev(c, a));
    }
    set_na(NA_PSTART, to, cfg.phyts + work[cfg.w_gphyt + a]);
    if (na_prev(NA_KIND, a) & 0xffu) != SEED {
        set_na(NA_PARENT, to, gone_parent(a));
    }
}

// How far presented axis `p`, this year's axis `q`, has moved at its
// phytomer `k`, or its last where it holds fewer; at its base where it
// holds none.
fn shift_now(p: u32, q: u32, k: u32) -> vec3<f32> {
    let n = na_now(NA_PHYTS, q);
    if n == 0u {
        return na_out3(NA_BASE, p) - na_now3(NA_BASE, q);
    }
    let r = na_now(NA_PSTART, q) + min(k, n - 1u);
    return out3(NP_TIP, r) - now3(NP_TIP, r);
}

// The phytomer of its bearer an axis stands at: its node, or for a
// continuation its bearer's last.
fn stands_at(kind: u32, node: u32) -> u32 {
    if kind == CONTINUED {
        return NONE;
    }
    return node;
}

// This year's axis `a` of the depth drawn: moved from last year's, its
// shoot unfolded and its leaves' shares written.
fn present_now(a: u32) {
    let t = cfg.f;
    let to = work[cfg.w_pos_now + a];
    let kind = na_now(NA_KIND, a) & 0xffu;
    var moved = vec3<f32>(0.0);
    if kind != SEED {
        let q = na_now(NA_PARENT, a);
        moved = shift_now(work[cfg.w_pos_now + q], q, stands_at(kind, na_now(NA_NODE, a)));
    }
    let found = work[cfg.w_old + a];
    let old = found - 1u;
    let n = na_now(NA_PHYTS, a);
    let first = na_now(NA_PSTART, a);
    let base = na_now3(NA_BASE, a);
    var olds = 0u;
    if found != 0u {
        olds = min(n, na_prev(NA_PHYTS, old));
        set_na3(NA_BASE, to, mixed3(na_prev3(NA_BASE, old), base, t));
        set_na3(NA_HEADING, to, mixed3(na_prev3(NA_HEADING, old), na_now3(NA_HEADING, a), t));
        set_na3(NA_SIDE, to, mixed3(na_prev3(NA_SIDE, old), na_now3(NA_SIDE, a), t));
        let joined = na_now(NA_JOINED, a) != 0u && na_prev(NA_JOINED, old) != 0u;
        if joined && na_now(NA_JOINED + 1u, a) == na_prev(NA_JOINED + 1u, old) {
            for (var c = NA_JOINED + 2u; c < NA_JOINED + 5u; c++) {
                set_na(c, to, b(mixed(f(na_prev(c, old)), f(na_now(c, a)), t)));
            }
        }
        let origin = na_prev(NA_PSTART, old);
        for (var k = 0u; k < olds; k++) {
            let p = first + k;
            let q = origin + k;
            set_np(NP_SCALE, p, b(mixed(f(np_prev(NP_SCALE, q)), f(np_now(NP_SCALE, p)), t)));
            set_np(NP_RADIUS, p, b(mixed(f(np_prev(NP_RADIUS, q)), f(np_now(NP_RADIUS, p)), t)));
            set3(NP_TIP, p, mixed3(prev3(NP_TIP, q), now3(NP_TIP, p), t));
            set3(NP_HEADING, p, mixed3(prev3(NP_HEADING, q), now3(NP_HEADING, p), t));
            set3(NP_SIDE, p, mixed3(prev3(NP_SIDE, q), now3(NP_SIDE, p), t));
            leaf[p] = 1.0;
        }
    } else {
        set_na3(NA_BASE, to, base + moved);
    }
    // This year's shoot, measured by the wood each phytomer grows.
    let pa = na_now(NA_PA, a);
    let elongation = row(pa, 0u);
    let expansion = row(pa, 1u);
    let spread = row(pa, 2u);
    var total = 0.0;
    for (var k = olds; k < n; k++) {
        let p = first + k;
        total += f(np_now(NP_SCALE, p)) * internode_factor(spread, vec2<u32>(np_now(NP_KEY, p), np_now(NP_KEY + 1u, p)));
    }
    let reach = run(cfg.tau, elongation) * total;
    var origin = base;
    var drawn = na_out3(NA_BASE, to);
    if olds > 0u {
        origin = now3(NP_TIP, first + olds - 1u);
        drawn = out3(NP_TIP, first + olds - 1u);
    }
    var along = 0.0;
    for (var k = olds; k < n; k++) {
        let p = first + k;
        let scale = f(np_now(NP_SCALE, p));
        let piece = scale * internode_factor(spread, vec2<u32>(np_now(NP_KEY, p), np_now(NP_KEY + 1u, p)));
        var r = select(0.0, 1.0, reach >= along && cfg.tau > 0.0);
        if piece > 0.0 {
            r = clamp((reach - along) / piece, 0.0, 1.0);
        }
        let tip = now3(NP_TIP, p);
        let placed = drawn + (tip - origin) * r;
        set3(NP_TIP, p, placed);
        set_np(NP_SCALE, p, b(scale * r));
        along += piece;
        var share = 0.0;
        if total > 0.0 {
            share = along / total;
        }
        leaf[p] = run(cfg.tau - elongation * reached(share), expansion);
        origin = tip;
        drawn = placed;
    }
}

// Last year's gone axis `g` of the depth drawn: faded by the fraction,
// carried with what it stood on.
fn present_gone(g: u32) {
    if work[cfg.w_now_of + g] != 0u {
        return;
    }
    let to = work[cfg.w_pos_gone + g];
    let kind = na_prev(NA_KIND, g) & 0xffu;
    var moved = vec3<f32>(0.0);
    if kind != SEED {
        let p = na_out(NA_PARENT, to);
        let q = na_prev(NA_PARENT, g);
        let mine = work[cfg.w_now_of + q];
        if mine == 0u {
            moved = vec3<f32>(f(work[cfg.w_shift + 3u * p]), f(work[cfg.w_shift + 3u * p + 1u]), f(work[cfg.w_shift + 3u * p + 2u]));
        } else {
            moved = shift_gone(p, q, stands_at(kind, na_prev(NA_NODE, g)));
        }
    }
    work[cfg.w_shift + 3u * to] = b(moved.x);
    work[cfg.w_shift + 3u * to + 1u] = b(moved.y);
    work[cfg.w_shift + 3u * to + 2u] = b(moved.z);
    set_na3(NA_BASE, to, na_prev3(NA_BASE, g) + moved);
    let n = na_prev(NA_PHYTS, g);
    let origin = na_prev(NA_PSTART, g);
    let first = na_out(NA_PSTART, to);
    for (var k = 0u; k < n; k++) {
        let p = first + k;
        let q = origin + k;
        for (var c = 0u; c < NP_COLS; c++) {
            set_np(c, p, np_prev(c, q));
        }
        set_np(NP_SCALE, p, b(mixed(f(np_prev(NP_SCALE, q)), 0.0, cfg.f)));
        set_np(NP_RADIUS, p, b(mixed(f(np_prev(NP_RADIUS, q)), 0.0, cfg.f)));
        set3(NP_TIP, p, prev3(NP_TIP, q) + moved);
        leaf[p] = 1.0;
    }
}

// How far the place a gone axis stood on has moved: presented axis `p`,
// last year's `q`, at phytomer `k` (its last for a continuation), where
// both years hold it; else at its base.
fn shift_gone(p: u32, q: u32, k: u32) -> vec3<f32> {
    let held = na_prev(NA_PHYTS, q);
    let now = na_out(NA_PHYTS, p);
    var on = k;
    if k == NONE && held > 0u {
        on = held - 1u;
    }
    if on < held && on < now {
        return out3(NP_TIP, na_out(NA_PSTART, p) + on) - prev3(NP_TIP, na_prev(NA_PSTART, q) + on);
    }
    return na_out3(NA_BASE, p) - na_prev3(NA_BASE, q);
}

// The axes of the depth drawn: this year's (`which` 0) or last year's gone.
@compute @workgroup_size(64)
fn present_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let a = item(id);
    if a >= cfg.base + cfg.count {
        return;
    }
    if cfg.which == 0u {
        present_now(a);
    } else {
        present_gone(a);
    }
}

// What the scans hold at each of last year's depths' first axes: the
// gone axes and their phytomers before it.
@compute @workgroup_size(64)
fn gather_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let d = item(id);
    if d >= cfg.base + cfg.count {
        return;
    }
    let first = work[cfg.w_starts + d];
    work[cfg.w_gather + 2u * d] = work[cfg.w_gone + first];
    work[cfg.w_gather + 2u * d + 1u] = work[cfg.w_gphyt + first];
}
