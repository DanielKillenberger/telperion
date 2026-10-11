// A chain's girth (host decision, 2026-10-08; in f32 since fn-254): the
// pipe model of `girth::thicken_given` by the same operations, so a root's
// base radius is the CPU reference's from the same scales to f32's
// rounding. A section borrows the first word of scratch the lever pass
// fills later: a phytomer's `P_NODE` and an axis's `S_TIP`. A chain's
// record carries its first axis's base radius in its first two words
// (`records[.. + 1]`, which the lever pass overwrites with the load).
// Part of the geometry kernel's shader.

// The word a section lives in.
const P_SECTION: u32 = 1u;
const S_SECTION: u32 = 5u;

// A PA's girth values (`laid::table`).
const G_PIPE: u32 = 25u;
const G_EXPONENT: u32 = 27u;
const G_INVERSE: u32 = 29u;
const G_RIPENING: u32 = 31u;
const G_SECONDARY: u32 = 33u;
const G_RELAY_AT: u32 = 35u;

fn gq(pa: u32, f: u32) -> f32 {
    return gtab[4u + GG * pa + f];
}

fn sw(i: u32, f: u32) -> f32 {
    return bitcast<f32>(s(i, f));
}

fn ssw(i: u32, f: u32, v: f32) {
    ss(i, f, bitcast<u32>(v));
}

fn pw(q: u32) -> f32 {
    return bitcast<f32>(scratch[sp + q * SP + P_SECTION]);
}

fn spw(q: u32, v: f32) {
    scratch[sp + q * SP + P_SECTION] = bitcast<u32>(v);
}

// `girth::section`: a radius's section at exponent `e`; none for no
// radius.
fn section_q(r: f32, e: f32) -> f32 {
    if !(r > 0.0) {
        return 0.0;
    }
    return exp2c(e * log2c(r));
}

// `girth::radius`: a section's radius at the reciprocal exponent
// `inverse`; none for no section.
fn radius_q(section: f32, inverse: f32) -> f32 {
    if !(section > 0.0) {
        return 0.0;
    }
    return exp2c(inverse * log2c(section));
}

// `girth::weighed`: secondary growth's share.
fn weighed(secondary: f32, root: f32, established: f32) -> f32 {
    return secondary * root + (1.0 - secondary) * established;
}

// Adds section `v` borne at own axis `j`'s node `node`, or at its tip
// beyond its nodes.
fn borne_w(j: u32, node: u32, v: f32) {
    if node < s(j, S_COUNT) {
        let q = s(j, S_START) + node;
        spw(q, pw(q) + v);
    } else {
        ssw(j, S_SECTION, sw(j, S_SECTION) + v);
    }
}

// The chain's radii by the pipe model over its own phytomers and its
// borne buds' base radii from their chains in the next wave
// (`girth::thicken_given`); its first axis's base radius into its record.
fn girth_realized(ch: Chain) {
    for (var i = 0u; i < ch.na; i++) {
        ssw(i, S_SECTION, 0.0);
        ssw(i, S_BASE, 0.0);
    }
    for (var p = 0u; p < ch.np; p++) {
        spw(p, 0.0);
    }
    var k = ch.k0;
    for (var r = 2u; r < ch.na; r++) {
        if kind_of(r) != LATERAL {
            continue;
        }
        let at = cfg.prior + CR * k;
        let radius = records[at + 1u];
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
        let first_ripe = ripe_f(1.0, ripening);
        var section = sw(j, S_SECTION);
        for (var n = s(j, S_COUNT); n > 0u; n--) {
            let at = lo(j, n - 1u);
            let p = at_k(j, n - 1u);
            let scale = out[at + 19u];
            let years = f32(cfg.age - min(ph(p, 2u), cfg.age) + 1u);
            let own = pipe * scale * ripe_f(years, ripening) * 1.0;
            section += section_q(own, e) + pw(s(j, S_START) + n - 1u);
            // Laid down at the apex, the phytomer carried nothing but itself.
            let established = pipe * scale * first_ripe;
            out[at + 18u] = weighed(secondary, radius_q(section, inverse), established);
            // The section it carries, for its relays' joins (`joins`).
            spw(s(j, S_START) + n - 1u, section);
        }
        let radius = radius_q(section + sw(j, S_BASE), inverse);
        if j == 1u {
            let at = cfg.mine + CR * ch.wi;
            records[at] = radius;
            records[at + 1u] = radius;
            continue;
        }
        let p = ax(j, PARENT);
        let carried = section_q(radius, gq(ax(p, PA), G_EXPONENT));
        let node = ax(j, NODE);
        if kind_of(j) != RELAYED {
            // A lateral's comes up through its record; a continuation's
            // enters at the tip.
            ssw(p, S_SECTION, sw(p, S_SECTION) + carried);
            continue;
        }
        // A relay's enters where it joins (`girth::thicken`, fn-244): its
        // share at the node whose span it stands in, the rest at the node
        // before, or below the parent's first node; past the parent's
        // nodes, at its tip. Its section is kept for its join.
        ssw(j, S_SECTION, carried);
        let count = s(p, S_COUNT);
        if count == 0u {
            ssw(p, S_SECTION, sw(p, S_SECTION) + carried);
            continue;
        }
        let kt = attached(j);
        let at = carried * kt.t;
        borne_w(p, kt.k, at);
        if kt.k == 0u {
            ssw(p, S_BASE, sw(p, S_BASE) + (carried - at));
        } else {
            borne_w(p, kt.k - 1u, carried - at);
        }
    }
    joins_realized(ch);
}

// Each relay's join (`telperion_space::Join`, fn-244): its parent's node
// (its share stays in `S_ATTACH`), and the parent's radius there with the
// relay whole (below the join) and without it (above), from the section
// the parent's node carries, weighed as that node is; the node `NONE`
// where nothing joins.
fn joins_realized(ch: Chain) {
    for (var j = 2u; j < ch.na; j++) {
        ss(j, S_JOIN, NONE);
        let p = ax(j, PARENT);
        let n = s(p, S_COUNT);
        if kind_of(j) != RELAYED || s(j, S_OWN) == NONE || s(p, S_OWN) == NONE || n == 0u || s(j, S_COUNT) == 0u {
            continue;
        }
        let kt = attached(j);
        let carried = sw(j, S_SECTION);
        let at = carried * kt.t;
        let section = pw(s(p, S_START) + kt.k);
        let pa = ax(p, PA);
        let inverse = gq(pa, G_INVERSE);
        let secondary = gq(pa, G_SECONDARY);
        let scale = out[lo(p, kt.k) + 19u];
        let established = gq(pa, G_PIPE) * scale * ripe_f(1.0, gq(pa, G_RIPENING));
        let below = radius_q(section + (carried - at), inverse);
        let above = radius_q(section - at, inverse);
        ss(j, S_JOIN, kt.k);
        ssf(j, S_JOIN + 2u, weighed(secondary, below, established));
        ssf(j, S_JOIN + 3u, weighed(secondary, above, established));
        // `lineage::join_key`: under the relay's first phytomer's key.
        let first = lo(j, 0u);
        let key = child(vec2<u32>(bitcast<u32>(out[first + 24u]), bitcast<u32>(out[first + 25u])), KEY_JOIN);
        ss(j, S_JOIN + 4u, key.x);
        ss(j, S_JOIN + 5u, key.y);
    }
}
