// A chain's shedding on the GPU (fn-209 F1, task 2, D15): which of its own
// axes it keeps, from its own axes and its borne buds' extinction curves
// alone (`alone/shed.rs`), and each borne bud's context, the chance its
// bearer's other buds had all died, on which the bud's own chain's
// keeping is conditioned. Chances of death are `-log2`, Q32, summed in any
// order. Part of `chain.wgsl`'s shader.

const SHED = vec2<u32>(0xfffffffeu, 0xffffffffu);
const UNTIL: u32 = 25u;
const DEAD: u32 = 26u;
const SHED_FLAGS: u32 = 28u;
const CONTEXT: u32 = 29u;
const KEPT: u32 = 1u;
const HAS_CONTEXT: u32 = 2u;
const NEVER: i32 = -2147483647 - 1;
const ALWAYS: i32 = 2147483647;

fn kind_of(i: u32) -> u32 {
    return rd(i, KIND) & 0xffu;
}

// Whether axis `i` is one of the chain's own: its first, or a
// continuation or relay of one.
fn own(i: u32, root: u32) -> bool {
    return i == root || (i > root && kind_of(i) != LATERAL);
}

// An own axis's parent among the chain's own, or none.
fn own_parent(i: u32, root: u32) -> u32 {
    let p = rd(i, PARENT);
    if p == NONE || p < root || !own(i, root) {
        return NONE;
    }
    return p;
}

// Whether own axis `i` sheds at all; its delay's last dead cycle.
fn sheds(i: u32, root: u32) -> bool {
    let k = kind_of(i);
    return own(i, root) && (k == LATERAL || k == RELAYED) && (flags(rd(i, PA)) & 64u) != 0u;
}

fn shed_at(i: u32) -> i32 {
    return bitcast<i32>(row(rd(i, PA), 17u).x);
}

fn until(i: u32) -> i32 {
    return bitcast<i32>(rd(i, UNTIL));
}

// `-log2` of the chance that a bud of `pa` made in `birth` has no living
// apex past cycle `t` (`Rows::dead_by`).
fn dead_by(pa: u32, birth: u32, t: i32) -> vec2<u32> {
    let at = t - i32(birth);
    if at < 0 {
        return inf();
    }
    let m = cfg.age - min(birth, cfg.age);
    let k = cfg.curves + 2u * (pa * (cfg.age + 1u) + m);
    let len = table[k + 1u];
    return words[table[k] + min(u32(at), len - 1u)];
}

// Whether own axis `j` is `at` or stands on it.
fn bears(j0: u32, at: u32, root: u32) -> bool {
    var j = j0;
    // A parent stands before its axis, so the walk ends.
    while j != NONE {
        if j == at {
            return true;
        }
        if j < root || !own(j, root) {
            return false;
        }
        let p = rd(j, PARENT);
        if p == NONE || p < root {
            return false;
        }
        j = p;
    }
    return false;
}

fn shed(root: u32, conditioned: bool, context: vec2<u32>) {
    let n = n_axes;
    for (var i = 0u; i < n; i++) {
        wr(i, UNTIL, bitcast<u32>(NEVER));
        wr64(i, DEAD, vec2<u32>(0u));
        wr(i, SHED_FLAGS, 0u);
    }
    // The last cycle its own axes lived, through each own axis's subtree.
    for (var k = n; k > root; k--) {
        let i = k - 1u;
        if !own(i, root) {
            continue;
        }
        let end = rd(i, APEX_END);
        let mine = select(i32(end) - 1, ALWAYS, end == 0u);
        let u = max(until(i), mine);
        wr(i, UNTIL, bitcast<u32>(u));
        let p = own_parent(i, root);
        if p != NONE {
            wr(p, UNTIL, bitcast<u32>(max(until(p), u)));
        }
    }
    // The chance every bud borne in each own axis's subtree had died by
    // that axis's threshold.
    for (var b = root + 1u; b < n; b++) {
        if kind_of(b) != LATERAL {
            continue;
        }
        var j = rd(b, PARENT);
        if j == NONE || j < root {
            continue;
        }
        while j != NONE {
            if sheds(j, root) {
                let d = dead_by(rd(b, PA), rd(b, BIRTH), shed_at(j));
                wr64(j, DEAD, and_q(rd64(j, DEAD), d));
            }
            j = own_parent(j, root);
        }
    }
    // Each own axis kept, its parent first.
    for (var i = root; i < n; i++) {
        if !own(i, root) {
            continue;
        }
        let p = own_parent(i, root);
        let above = p == NONE || (rd(p, SHED_FLAGS) & KEPT) != 0u;
        var lives = true;
        if sheds(i, root) && until(i) <= shed_at(i) {
            let d = draw(child(child(rd64(i, LINEAGE), SHED), CONTINUATION));
            lives = kept_q(rd64(i, DEAD), conditioned && i == root, context, d);
        }
        wr(i, SHED_FLAGS, select(0u, KEPT, above && lives));
    }
    // Each borne bud's context.
    for (var b = root + 1u; b < n; b++) {
        if kind_of(b) != LATERAL {
            continue;
        }
        var j = rd(b, PARENT);
        if j < root {
            continue;
        }
        while j != NONE {
            if sheds(j, root) {
                let t = shed_at(j);
                if until(j) > t {
                    break;
                }
                var others = vec2<u32>(0u);
                for (var c = root + 1u; c < n; c++) {
                    if c == b || kind_of(c) != LATERAL || !bears(rd(c, PARENT), j, root) {
                        continue;
                    }
                    others = and_q(others, dead_by(rd(c, PA), rd(c, BIRTH), t));
                }
                wr(b, SHED_FLAGS, rd(b, SHED_FLAGS) | HAS_CONTEXT);
                wr64(b, CONTEXT, others);
                break;
            }
            j = own_parent(j, root);
        }
    }
}
