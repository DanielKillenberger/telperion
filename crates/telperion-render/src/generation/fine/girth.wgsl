// A chain's girth under contract A (host decision, 2026-10-08): the pipe
// model of `girth::thicken_given` in its own fixed point, op for op, so
// a root's base radius is the CPU reference's from the same scales.
// Sections are W (128-bit, `feedback_wide.wgsl`), pipes and radii Q32;
// `log2_w`, `exp2_w`, `prod_shr`, `mul_shr` and `ripe` are the feedback
// contract's, `log2q`, `exp2q` and `mulq` the decision contract's. A
// section's W words borrow scratch the lever pass fills later: a
// phytomer's `P_NODE` and `P_LOAD`, and an axis's `S_TIP` and `S_LOAD`.
// A chain's record carries its first axis's base radius as
// a float and, in its next two words, as Q32 (`records[.. + 1, 2]`,
// which the lever pass overwrites with the load). Part of the geometry
// kernel's shader.

// The four words a section lives in.
const P_SECTION: u32 = 1u;
const S_SECTION: u32 = 5u;

// A PA's girth values, Q32 (`laid::table`).
const G_PIPE: u32 = 25u;
const G_EXPONENT: u32 = 27u;
const G_INVERSE: u32 = 29u;
const G_RIPENING: u32 = 31u;
const G_SECONDARY: u32 = 33u;
const G_RELAY_AT: u32 = 35u;

fn gq(pa: u32, f: u32) -> vec2<u32> {
    let b = 4u + GG * pa + f;
    return vec2<u32>(bitcast<u32>(gtab[b]), bitcast<u32>(gtab[b + 1u]));
}

fn sw(i: u32, f: u32) -> vec4<u32> {
    return vec4<u32>(s(i, f), s(i, f + 1u), s(i, f + 2u), s(i, f + 3u));
}

fn ssw(i: u32, f: u32, v: vec4<u32>) {
    for (var k = 0u; k < 4u; k++) {
        ss(i, f + k, v[k]);
    }
}

fn pw(q: u32) -> vec4<u32> {
    let b = sp + q * SP + P_SECTION;
    return vec4<u32>(scratch[b], scratch[b + 1u], scratch[b + 2u], scratch[b + 3u]);
}

fn spw(q: u32, v: vec4<u32>) {
    let b = sp + q * SP + P_SECTION;
    for (var k = 0u; k < 4u; k++) {
        scratch[b + k] = v[k];
    }
}

// `girth::section`: a radius's section at exponent `e`, W; none for no
// radius.
fn section_q(r: vec2<u32>, e: vec2<u32>) -> vec4<u32> {
    if !slt64(vec2<u32>(0u), r) {
        return vec4<u32>(0u);
    }
    return exp2_w(mulq(e, sub64(log2q(r), w(0u, 32u)))).v;
}

// `girth::radius`: a section's radius at the reciprocal exponent
// `inverse`, Q32; none for no section.
fn radius_q(section: vec4<u32>, inverse: vec2<u32>) -> vec2<u32> {
    if !slt4(vec4<u32>(0u), section) {
        return vec2<u32>(0u);
    }
    return exp2q(mulq(inverse, log2_w(section)));
}

// `split`: a section's share `s` (Q32), floored, and the rest.
fn split_q(x: vec4<u32>, s: vec2<u32>) -> array<vec4<u32>, 2> {
    let part = mul_shr(x, ext4(s), 32u).v;
    return array<vec4<u32>, 2>(part, sub4(x, part));
}

// A Q32 as a float.
fn real_q(q: vec2<u32>) -> f32 {
    let neg = negative(q);
    let a = select(q, neg64(q), neg);
    let v = f32(a.y) + f32(a.x) * (1.0 / 4294967296.0);
    return select(v, -v, neg);
}

// Adds section `v` borne at own axis `j`'s node `node`, or at its tip
// beyond its nodes.
fn borne_w(j: u32, node: u32, v: vec4<u32>) {
    if node < s(j, S_COUNT) {
        let q = s(j, S_START) + node;
        spw(q, add4(pw(q), v));
    } else {
        ssw(j, S_SECTION, add4(sw(j, S_SECTION), v));
    }
}

// The chain's radii by the pipe model over its own phytomers and its
// borne buds' base radii from their chains in the next wave
// (`girth::thicken_given`); its first axis's base radius into its record.
fn girth_realized(ch: Chain) {
    for (var i = 0u; i < ch.na; i++) {
        ssw(i, S_SECTION, vec4<u32>(0u));
        ssw(i, S_BASE, vec4<u32>(0u));
    }
    for (var p = 0u; p < ch.np; p++) {
        spw(p, vec4<u32>(0u));
    }
    var k = ch.k0;
    for (var r = 2u; r < ch.na; r++) {
        if kind_of(r) != LATERAL {
            continue;
        }
        let at = cfg.prior + CR * k;
        let radius = vec2<u32>(bitcast<u32>(records[at + 1u]), bitcast<u32>(records[at + 2u]));
        k += 1u;
        let j = ax(r, PARENT);
        if s(j, S_OWN) == NONE {
            continue;
        }
        borne_w(j, ax(r, NODE), section_q(radius, gq(ax(j, PA), G_EXPONENT)));
    }
    for (var i = ch.na; i > 1u; i--) {
        let j = i - 1u;
        if s(j, S_OWN) == NONE {
            continue;
        }
        let pa = ax(j, PA);
        let e = gq(pa, G_EXPONENT);
        let pipe = gq(pa, G_PIPE);
        let ripening = gq(pa, G_RIPENING);
        let secondary = gq(pa, G_SECONDARY);
        let inverse = gq(pa, G_INVERSE);
        let first_ripe = ripe(one32(), ripening);
        var section = sw(j, S_SECTION);
        for (var n = s(j, S_COUNT); n > 0u; n--) {
            let at = lo(j, n - 1u);
            let p = at_k(j, n - 1u);
            let scale = q32_f(out[at + 19u]);
            let years = cfg.age - min(ph(p, 2u), cfg.age) + 1u;
            let own = prod_shr(pipe, scale, ripe(w(0u, years), ripening), one32(), 96u).v.xy;
            section = add4(add4(section, section_q(own, e)), pw(s(j, S_START) + n - 1u));
            // Laid down at the apex, the phytomer carried nothing but itself.
            let established = prod_shr(pipe, scale, first_ripe, w(1u, 0u), 64u).v.xy;
            let root = radius_q(section, inverse);
            let weighed = add64(mulq(secondary, root), mulq(sub64(one32(), secondary), established));
            out[at + 18u] = real_q(weighed);
            // The section it carries, for its relays' joins (`joins`).
            spw(s(j, S_START) + n - 1u, section);
        }
        let radius = radius_q(add4(section, sw(j, S_BASE)), inverse);
        if j == 1u {
            let at = cfg.mine + CR * ch.wi;
            records[at] = real_q(radius);
            records[at + 1u] = bitcast<f32>(radius.x);
            records[at + 2u] = bitcast<f32>(radius.y);
            continue;
        }
        let p = ax(j, PARENT);
        let carried = section_q(radius, gq(ax(p, PA), G_EXPONENT));
        let node = ax(j, NODE);
        if kind_of(j) != RELAYED {
            // A lateral's comes up through its record; a continuation's
            // enters at the tip.
            ssw(p, S_SECTION, add4(sw(p, S_SECTION), carried));
            continue;
        }
        // A relay's enters where it joins (`girth::thicken`, fn-244): its
        // share at the node whose span it stands in, the rest at the node
        // before, or below the parent's first node; past the parent's
        // nodes, at its tip. Its section is kept for its join.
        ssw(j, S_SECTION, carried);
        let count = s(p, S_COUNT);
        if count == 0u {
            ssw(p, S_SECTION, add4(sw(p, S_SECTION), carried));
            continue;
        }
        let kt = attached(j);
        let parts = split_q(carried, kt.yz);
        borne_w(p, kt.x, parts[0]);
        if kt.x == 0u {
            ssw(p, S_BASE, add4(sw(p, S_BASE), parts[1]));
        } else {
            borne_w(p, kt.x - 1u, parts[1]);
        }
    }
    joins_realized(ch);
}

// Each relay's join (`telperion_space::Join`, fn-244): its parent's node
// and share, and the parent's radius there with the relay whole (below
// the join) and without it (above), from the section the parent's node
// carries, weighed as that node is; the node `NONE` where nothing joins.
fn joins_realized(ch: Chain) {
    for (var j = 2u; j < ch.na; j++) {
        ss(j, S_JOIN, NONE);
        let p = ax(j, PARENT);
        let n = s(p, S_COUNT);
        if kind_of(j) != RELAYED || s(j, S_OWN) == NONE || s(p, S_OWN) == NONE || n == 0u || s(j, S_COUNT) == 0u {
            continue;
        }
        let kt = attached(j);
        let parts = split_q(sw(j, S_SECTION), kt.yz);
        let section = pw(s(p, S_START) + kt.x);
        let pa = ax(p, PA);
        let inverse = gq(pa, G_INVERSE);
        let secondary = gq(pa, G_SECONDARY);
        let scale = q32_f(out[lo(p, kt.x) + 19u]);
        let first_ripe = ripe(one32(), gq(pa, G_RIPENING));
        let established = prod_shr(gq(pa, G_PIPE), scale, first_ripe, w(1u, 0u), 64u).v.xy;
        let below = radius_q(add4(section, parts[1]), inverse);
        let above = radius_q(sub4(section, parts[0]), inverse);
        let weigh = sub64(one32(), secondary);
        ss(j, S_JOIN, kt.x);
        ssf(j, S_JOIN + 1u, real_q(vec2<u32>(kt.y, kt.z)));
        ssf(j, S_JOIN + 2u, real_q(add64(mulq(secondary, below), mulq(weigh, established))));
        ssf(j, S_JOIN + 3u, real_q(add64(mulq(secondary, above), mulq(weigh, established))));
        // `lineage::join_key`: under the relay's first phytomer's key.
        let first = lo(j, 0u);
        let key = child(vec2<u32>(bitcast<u32>(out[first + 24u]), bitcast<u32>(out[first + 25u])), KEY_JOIN);
        ss(j, S_JOIN + 4u, key.x);
        ss(j, S_JOIN + 5u, key.y);
    }
}
