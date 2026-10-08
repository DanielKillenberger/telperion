// Fine roots settled under realized mechanics on the GPU (fn-209 F1 task
// 6; REALIZED.md, host decision 8, step 4): each root, as phase 1 shed
// it, scaled from the wave before (`scales`), thickened from the wave
// after (`girths`, its base radius up), laid before it bends (`lays`),
// loaded from the wave after (`levers`, its load up) and laid under its
// levers from its first lay's frames (`bends`), the structure above the
// cut settling between the phases. Each pass is the whole tree's own on
// the chain's axes (`geometry::scale`, `girth::thicken_given`,
// `geometry::place_from`, `sag::gather`), in f32 but the girth, which is
// contract A's fixed point (`girth.wgsl`). Part of the geometry kernel's
// shader.

// A chain's record in `records`: its first axis's base radius, then its
// load at its base (mass and first moment, positions the root's own).
const CR: u32 = 5u;
// A root's record, before the chains': its origins' heights, then its
// first axis's vigour, Q32, as its seed hands it (task 9: a woken root's
// as its bearer kept it, known once the structure above has grown).
const RR: u32 = 4u;
// The grower's Q32 fields (`assign.wgsl`, `fades.wgsl`).
const QVIG: u32 = 48u;
const FADE: u32 = 66u;
const QSCALE: u32 = 9u;

// A chain as a pass takes it: its counts, its bearer's phytomer in the
// wave before and the points below it, its bearer's PA, its origins'
// heights, its place in its wave and its first borne bud's in the next.
struct Chain {
    na: u32,
    np: u32,
    bearer: u32,
    below_u: vec3<f32>,
    below_b: vec3<f32>,
    bearer_pa: u32,
    ground_u: f32,
    ground_b: f32,
    wi: u32,
    k0: u32,
    root_vigour: f32,
}

fn begin(c: u32) -> Chain {
    let it = c * IW;
    ax0 = items[it];
    let na = items[it + 1u];
    ph0 = items[it + 2u];
    let np = items[it + 3u];
    out_ax = items[it + 5u];
    out_ph = items[it + 6u];
    sb = items[it + 7u];
    sp = sb + na * SA;
    let stub = (items[it + 8u] & 1u) != 0u;
    let root = items[it + 9u];
    let rec = items[it + 4u] * AL;
    var node = 0u;
    if !stub && na > 1u {
        node = ax(1u, NODE);
    }
    let bearer = cfg.prev_phyts + (bitcast<u32>(prev[rec + 7u]) + node) * PL;
    var below_u = p3(rec);
    var below_b = p3(rec + 3u);
    if node > 0u {
        below_u = p3(bearer - PL + 9u);
        below_b = p3(bearer - PL);
    }
    let pa = bitcast<u32>(prev[rec + 6u]);
    let ground = vec2<f32>(records[RR * root], records[RR * root + 1u]);
    // A first wave's chain is its root's: its first axis's vigour comes
    // down from its seed.
    var root_vigour = -1.0;
    if stub {
        let made = vec2<u32>(bitcast<u32>(records[RR * root + 2u]), bitcast<u32>(records[RR * root + 3u]));
        root_vigour = qf(made.x, made.y);
    }
    status = 0u;
    if na > 0u {
        status = scratch[sp + np * SP];
    }
    return Chain(na, np, bearer, below_u, below_b, pa, ground.x, ground.y, items[it + 10u], items[it + 11u], root_vigour);
}

fn finish(ch: Chain) {
    scratch[sp + ch.np * SP] = status;
}

fn chain_id(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

// A Q32 presence as a real.
fn qf(lo: u32, hi: u32) -> f32 {
    return f32(hi) + f32(lo) * (1.0 / 4294967296.0);
}

// An axis's vigour as shed sizes it: its presences' times its fade.
fn vigour_of(i: u32) -> f32 {
    return qf(ax(i, QVIG), ax(i, QVIG + 1u)) * qf(ax(i, FADE), ax(i, FADE + 1u));
}

// A root's first axis's vigour as shed sizes it: its seed's times its
// fade.
fn root_vigour_of(made: f32) -> f32 {
    return made * qf(ax(1u, FADE), ax(1u, FADE + 1u));
}

// Each own axis's base scale, from its bearer's (`geometry::scale`), and
// its phytomers' scales and ranks.
fn scale_realized(na: u32, bearer: u32, root_vigour: f32) {
    for (var i = 1u; i < na; i++) {
        if s(i, S_OWN) == NONE {
            continue;
        }
        var base = 0.0;
        if i == 1u {
            let f = pa_of(ax(i, PA));
            var vigour = vigour_of(1u);
            if root_vigour >= 0.0 {
                vigour = root_vigour_of(root_vigour);
            }
            base = prev[bearer + 19u] * vigour * dominance(f.dominance, lineage_of(i));
        } else {
            base = sf(ax(i, PARENT), S_SCALE) * vigour_of(i);
        }
        ssf(i, S_SCALE, base);
        var reach = 0.0;
        for (var k = 0u; k < s(i, S_COUNT); k++) {
            let p = at_k(i, k);
            let at = lo(i, k);
            let scale = qf(ph(p, QSCALE), ph(p, QSCALE + 1u)) * base;
            out[at + 19u] = scale;
            out[at + 20u] = bitcast<f32>(ph(p, 7u));
            out[at + 21u] = bitcast<f32>(ph(p, 8u));
            reach += scale;
        }
        ssf(i, S_REACH, reach);
        out[(out_ax + s(i, S_OWN)) * AL + 6u] = bitcast<f32>(ax(i, PA));
        out[(out_ax + s(i, S_OWN)) * AL + 7u] = bitcast<f32>(out_ph + s(i, S_START));
    }
    for (var i = na; i > 1u; i--) {
        let j = i - 1u;
        if s(j, S_OWN) != NONE && kind_of(j) == RELAYED {
            let p = ax(j, PARENT);
            ssf(p, S_REACH, sf(p, S_REACH) + (1.0 - axf(j, BLEND)) * sf(j, S_REACH));
        }
        // A continuation is a relay at blend 0 (fn-199).
        if s(j, S_OWN) != NONE && kind_of(j) == CONTINUED {
            let p = ax(j, PARENT);
            ssf(p, S_REACH, sf(p, S_REACH) + sf(j, S_REACH));
        }
    }
}

// Adds `v` borne at own axis `j`'s node `node`, or at its tip beyond its
// nodes, into the scratch field at `fp` (a phytomer's) or `fa` (an
// axis's).
fn borne_at(j: u32, node: u32, fp: u32, fa: u32, v: f32) {
    if node < s(j, S_COUNT) {
        let q = s(j, S_START) + node;
        spf(q, fp, pf(q, fp) + v);
    } else {
        ssf(j, fa, sf(j, fa) + v);
    }
}

// Adds `share` of a load (`mass`, `moment`) at region phytomer `q`.
fn load_at(q: u32, mass: f32, moment: vec3<f32>, share: f32) {
    spf(q, P_LOAD, pf(q, P_LOAD) + mass * share);
    for (var c = 0u; c < 3u; c++) {
        spf(q, P_LOAD + 1u + c, pf(q, P_LOAD + 1u + c) + moment[c] * share);
    }
}

// The chain's levers from its own wood and its borne buds' loads from
// their chains in the next wave, on the chain as laid before it bends
// (`sag::gather`); its first axis's load into its record.
fn levers_realized(ch: Chain) {
    for (var i = 0u; i < ch.na; i++) {
        for (var f = 0u; f < 4u; f++) {
            ssf(i, S_LOAD + f, 0.0);
            ssf(i, S_BASE + f, 0.0);
        }
    }
    for (var p = 0u; p < ch.np; p++) {
        for (var f = 0u; f < 4u; f++) {
            spf(p, P_LOAD + f, 0.0);
        }
    }
    let age = f32(cfg.age);
    for (var i = 1u; i < ch.na; i++) {
        if s(i, S_OWN) == NONE {
            continue;
        }
        let n = s(i, S_COUNT);
        var far = s3(i, S_FRAME);
        if n > 0u {
            far = o3(lo(i, n - 1u) + 9u);
        }
        ss3(i, S_FAR, far);
    }
    for (var i = ch.na; i > 2u; i--) {
        let j = i - 1u;
        if s(j, S_OWN) != NONE && kind_of(j) == CONTINUED {
            ss3(ax(j, PARENT), S_FAR, s3(j, S_FAR));
        }
    }
    var k = ch.k0;
    for (var r = 2u; r < ch.na; r++) {
        if kind_of(r) != LATERAL {
            continue;
        }
        let at = cfg.prior + CR * k;
        k += 1u;
        let j = ax(r, PARENT);
        if s(j, S_OWN) == NONE {
            continue;
        }
        for (var f = 0u; f < 4u; f++) {
            borne_at(j, ax(r, NODE), P_LOAD + f, S_LOAD + f, records[at + 1u + f]);
        }
    }
    for (var i = ch.na; i > 1u; i--) {
        let j = i - 1u;
        if s(j, S_OWN) == NONE {
            continue;
        }
        let n = s(j, S_COUNT);
        let base0 = s3(j, S_FRAME);
        let f = pa_of(ax(j, PA));
        var mass = sf(j, S_LOAD);
        var moment = s3(j, S_LOAD + 1u);
        let far = s3(j, S_FAR);
        for (var m = n; m > 0u; m--) {
            let at = lo(j, m - 1u);
            let tip = o3(at + 9u);
            var base = base0;
            if m > 1u {
                base = o3(lo(j, m - 2u) + 9u);
            }
            let len = length(tip - base);
            let p = at_k(j, m - 1u);
            let years = age - min(f32(ph(p, 2u)), age) + 1.0;
            let leaves = f.pipe * out[at + 19u] * ripe_f(years, f.ripening);
            let radius = out[at + 18u];
            let own = len * (radius * radius + leaves * leaves);
            let q = s(j, S_START) + m - 1u;
            mass += pf(q, P_LOAD);
            moment += vec3<f32>(pf(q, P_LOAD + 1u), pf(q, P_LOAD + 2u), pf(q, P_LOAD + 3u));
            let lever = moment - tip * mass;
            let chord = far - base;
            for (var c = 0u; c < 3u; c++) {
                spf(q, P_LEVER + c, lever[c]);
                spf(q, P_LEVER + 3u + c, chord[c]);
            }
            mass += own;
            moment += (base + tip) * 0.5 * own;
            let held_share = support(min(base.z, tip.z) + ch.ground_u, radius, len);
            mass *= held_share;
            moment *= held_share;
        }
        mass += sf(j, S_BASE);
        moment += s3(j, S_BASE + 1u);
        if j == 1u {
            let at = cfg.mine + CR * ch.wi;
            records[at + 1u] = mass;
            records[at + 2u] = moment.x;
            records[at + 3u] = moment.y;
            records[at + 4u] = moment.z;
            continue;
        }
        let p = ax(j, PARENT);
        var w_node = 0.0;
        let node = ax(j, NODE);
        if kind_of(j) == RELAYED && node < s(p, S_COUNT) {
            w_node = axf(j, BLEND);
        }
        if w_node > 0.0 {
            // Spread over the span it stands in (`girth::relay_shares`).
            let t = relay_span(p, pa_of(ax(p, PA)).relay_at, node);
            let q = s(p, S_START) + node;
            load_at(q, mass, moment, w_node * t);
            if node > 0u {
                load_at(q - 1u, mass, moment, w_node * (1.0 - t));
            } else {
                let w = w_node * (1.0 - t);
                ssf(p, S_BASE, sf(p, S_BASE) + mass * w);
                ss3(p, S_BASE + 1u, s3(p, S_BASE + 1u) + moment * w);
            }
        }
        ssf(p, S_LOAD, sf(p, S_LOAD) + mass * (1.0 - w_node));
        ss3(p, S_LOAD + 1u, s3(p, S_LOAD + 1u) + moment * (1.0 - w_node));
    }
}

@compute @workgroup_size(64)
fn scales(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = chain_id(id);
    if c >= cfg.n {
        return;
    }
    let ch = begin(c);
    if ch.na == 0u {
        return;
    }
    own_axes(ch.na, ch.np);
    scale_realized(ch.na, ch.bearer, ch.root_vigour);
}

@compute @workgroup_size(64)
fn girths(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = chain_id(id);
    if c >= cfg.n {
        return;
    }
    let ch = begin(c);
    for (var f = 0u; f < 3u; f++) {
        records[cfg.mine + CR * ch.wi + f] = 0.0;
    }
    if ch.na == 0u {
        return;
    }
    girth_realized(ch);
}

@compute @workgroup_size(64)
fn lays(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = chain_id(id);
    if c >= cfg.n {
        return;
    }
    let ch = begin(c);
    if ch.na == 0u {
        return;
    }
    for (var p = 0u; p < ch.np; p++) {
        for (var f = 0u; f < 6u; f++) {
            spf(p, P_LEVER + f, 0.0);
        }
    }
    lay_all(ch.na, false, ch.bearer, ch.below_u, ch.bearer_pa, ch.ground_u, false);
    if cfg.sags == 0u {
        for (var i = 1u; i < ch.na; i++) {
            if s(i, S_OWN) == NONE {
                continue;
            }
            for (var k = 0u; k < s(i, S_COUNT); k++) {
                let at = lo(i, k);
                for (var f = 0u; f < 9u; f++) {
                    out[at + f] = out[at + 9u + f];
                }
            }
            let a = (out_ax + s(i, S_OWN)) * AL;
            wo3(a + 3u, o3(a));
        }
    }
    finish(ch);
}

@compute @workgroup_size(64)
fn levers(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = chain_id(id);
    if c >= cfg.n {
        return;
    }
    let ch = begin(c);
    for (var f = 1u; f < CR; f++) {
        records[cfg.mine + CR * ch.wi + f] = 0.0;
    }
    if ch.na == 0u {
        return;
    }
    levers_realized(ch);
}

@compute @workgroup_size(64)
fn bends(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = chain_id(id);
    if c >= cfg.n {
        return;
    }
    let ch = begin(c);
    if ch.na == 0u {
        return;
    }
    lay_all(ch.na, true, ch.bearer, ch.below_b, ch.bearer_pa, ch.ground_b, true);
    finish(ch);
}
