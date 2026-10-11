// fn-256 task 2 (SETTLE-PLAN.md S0): the kept tree compacted into the
// node table in its final order, each kept axis's units' and phytomers'
// rows contiguous at their scanned starts: its scalars, its parent
// renumbered, its vigour by its fade (`shed_past`, exact in the f64), its
// units' presences, and each phytomer's cycle, key, rank (summed in the
// f64 in its order, `assign_one`), shade, grown and scale.

// The node table's axis columns, `cfg.sk` rows apart from word 0.
const NA_KIND: u32 = 0u;
const NA_PARENT: u32 = 1u;
const NA_NODE: u32 = 2u;
const NA_WHORL: u32 = 3u;
const NA_LINEAGE: u32 = 5u;
const NA_PA: u32 = 7u;
const NA_BIRTH: u32 = 8u;
const NA_END: u32 = 9u;
const NA_VIGOUR: u32 = 10u;
const NA_BLEND: u32 = 12u;
const NA_ALIVE: u32 = 14u;
const NA_RANK: u32 = 15u;
const NA_SLEEP: u32 = 17u;
const NA_PHASE: u32 = 18u;
const NA_UNITS: u32 = 19u;
const NA_USTART: u32 = 20u;
const NA_PHYTS: u32 = 21u;
const NA_PSTART: u32 = 22u;
const NA_GROWN: u32 = 23u;
// The phytomers' columns, `cfg.skp` rows apart from `cfg.at_np`.
const NP_CYCLE: u32 = 0u;
const NP_KEY: u32 = 1u;
const NP_RANK: u32 = 3u;
const NP_SIZE: u32 = 5u;
const NP_LIGHT: u32 = 7u;
const NP_GROWN: u32 = 9u;
const NP_SCALE: u32 = 10u;

fn set_na(c: u32, f: u32, v: u32) {
    nt[c * cfg.sk + f] = v;
}

fn set_na64(c: u32, f: u32, v: vec2<u32>) {
    set_na(c, f, v.x);
    set_na(c + 1u, f, v.y);
}

fn set_np(c: u32, r: u32, v: u32) {
    nt[cfg.at_np + c * cfg.skp + r] = v;
}

fn set_np64(c: u32, r: u32, v: vec2<u32>) {
    set_np(c, r, v.x);
    set_np(c + 1u, r, v.y);
}

// Each kept axis's phytomers and units, for their scans.
@compute @workgroup_size(64)
fn sizes_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let f = item(id);
    if f > cfg.kept {
        return;
    }
    var phyts = 0u;
    var units_n = 0u;
    if f < cfg.kept {
        let g = wf(WF_GROWN, f);
        phyts = ax(g, A_PHYTS);
        units_n = ax(g, A_UNITS);
    }
    set_wf(WF_PSTART, f, phyts);
    set_wf(WF_USTART, f, units_n);
}

// One kept axis's row, units and phytomers in the node table.
@compute @workgroup_size(64)
fn compact_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let f = item(id);
    if f >= cfg.kept {
        return;
    }
    let g = wf(WF_GROWN, f);
    let kind = kind_of(g);
    let p = ax(g, A_PARENT);
    set_na(NA_KIND, f, ax(g, A_ORIGIN));
    set_na(NA_PARENT, f, select(wa(W_FINAL, p), NONE, p == NONE));
    set_na(NA_NODE, f, wa(W_NODE, g));
    set_na64(NA_WHORL, f, select(vec2<u32>(0u), ax64(g, A_WHORL), kind == LATERAL));
    set_na64(NA_LINEAGE, f, ax64(g, A_LINEAGE));
    set_na(NA_PA, f, ax(g, A_PA));
    set_na(NA_BIRTH, f, ax(g, A_BIRTH));
    set_na(NA_END, f, ax(g, A_END));
    set_na64(NA_VIGOUR, f, f64_mul(real64(waf(W_VIG, g)), real64(waf(W_FADE, g))));
    set_na64(NA_BLEND, f, ax64(g, A_BLEND));
    set_na(NA_ALIVE, f, wa(W_ALIVEQ, g));
    set_na(NA_SLEEP, f, ax(g, A_SLEEP));
    set_na(NA_PHASE, f, ax(g, A_PHASE));
    let nu = ax(g, A_UNITS);
    let np = ax(g, A_PHYTS);
    let fu = wf(WF_USTART, f);
    let fp = wf(WF_PSTART, f);
    set_na(NA_UNITS, f, nu);
    set_na(NA_USTART, f, fu);
    set_na(NA_PHYTS, f, np);
    set_na(NA_PSTART, f, fp);
    set_na(NA_GROWN, f, g);
    let us = wa(W_USTART, g);
    for (var k = 0u; k < nu; k++) {
        nt[cfg.at_nu + fu + k] = wu(WU_PRES, us + k);
    }
    let ps = wa(W_PSTART, g);
    var rank = vec2<u32>(0u);
    for (var j = 0u; j < np; j++) {
        let row = wp(WP_ROW, ps + j);
        let r = wp(WP_UNIT, ps + j);
        let at = fp + j;
        set_np(NP_CYCLE, at, phyt(row, P_CYCLE));
        set_np64(NP_KEY, at, vec2<u32>(phyt(row, P_KEY), phyt(row, P_KEY + 1u)));
        set_np64(NP_RANK, at, rank);
        set_np64(NP_SIZE, at, vec2<u32>(unit(r, U_SHADE), unit(r, U_SHADE + 1u)));
        set_np64(NP_LIGHT, at, vec2<u32>(unit(r, U_SHADE + 2u), unit(r, U_SHADE + 3u)));
        let scale = wp(WP_SCALE, ps + j);
        set_np(NP_GROWN, at, scale);
        set_np(NP_SCALE, at, scale);
        let node = real64(bitcast<f32>(phyt(row, P_NODE)));
        rank = f64_add(rank, f64_mul(node, real64(tick_q(unit(r, U_SIZE)))));
    }
    set_na64(NA_RANK, f, rank);
}
