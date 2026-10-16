// fn-256 task 6 (COUNCIL.md round 4, step 4): a sweep's narrow block
// levels folded into one workgroup, which loops them in the sweep's order
// with barriers between: the few levels at the hierarchy's top and the
// deepest, whose blocks a dispatch each would leave the device idle for.
// The workgroup's lanes take up to `FOLD` blocks of a level at once, a
// group of `FOLD_LANES` lanes a block, each block's members by depth as a
// block's own workgroup takes them. Each axis takes its children in
// fn-255's order, so the words are those of a dispatch a level.
//
// The module that includes this gives the level's blocks
// (`fold_level`), a block's run for the sweep (`fold_run`, empty where
// the sweep enters it not), its members at a depth
// (`fold_lower`), a member (`fold_member`), the sweep's work at an axis
// (`fold_apply`), and the levels `[cfg.fold_lo, cfg.fold_hi)` folded,
// none past `cfg.levels`.

const FOLD: u32 = 8u;
const FOLD_LANES: u32 = 32u;
const FOLDED: u32 = 256u;

var<workgroup> w_fold_level: vec2<u32>;
var<workgroup> w_fold_runs: array<vec4<u32>, FOLD>;
const FOLD_AT: u32 = FOLD * (FOLD_LANES + 1u);
var<workgroup> w_fold_at: array<u32, FOLD_AT>;
var<workgroup> w_fold_steps: u32;

// The `i`th of a block's depth bounds in the sweep's order, `FOLD_LANES`
// of them taken at once by a block's lanes: where depth `run.z + i`
// begins, or where `up`, where the depth `i` above the deepest ends.
fn fold_start(run: vec4<u32>, i: u32, up: bool) -> u32 {
    let depths = run.w - run.z + 1u;
    if run.y <= run.x || i >= depths {
        return select(run.y, run.x, up);
    }
    if i == 0u {
        return select(run.x, run.y, up);
    }
    return fold_lower(run, select(run.z + i, run.w + 1u - i, up));
}

// The folded levels, deepest first where `up`, each level's blocks
// `FOLD` at a time, their depths deepest first where `up`.
fn folded(op: u32, up: bool, lid: u32) {
    let g = lid / FOLD_LANES;
    let lane = lid % FOLD_LANES;
    let hi = min(cfg.fold_hi, cfg.levels);
    let n = hi - min(cfg.fold_lo, hi);
    for (var t = 0u; t < n; t++) {
        let l = select(cfg.fold_lo + t, hi - 1u - t, up);
        if lid == 0u {
            w_fold_level = fold_level(l);
        }
        let lv = workgroupUniformLoad(&w_fold_level);
        let batches = (lv.y - min(lv.x, lv.y) + FOLD - 1u) / FOLD;
        for (var bt = 0u; bt < batches; bt++) {
            if lane == 0u {
                let k = lv.x + bt * FOLD + g;
                var run = vec4<u32>(0u);
                if k < lv.y {
                    run = fold_run(op, k);
                }
                w_fold_runs[g] = run;
            }
            workgroupBarrier();
            if lid == 0u {
                var most = 0u;
                for (var j = 0u; j < FOLD; j++) {
                    let r = w_fold_runs[j];
                    if r.y > r.x {
                        most = max(most, r.w - r.z + 1u);
                    }
                }
                w_fold_steps = most;
            }
            let steps = workgroupUniformLoad(&w_fold_steps);
            let run = w_fold_runs[g];
            let at = g * (FOLD_LANES + 1u);
            for (var h = 0u; h < steps; h++) {
                let q = h % FOLD_LANES;
                if q == 0u {
                    for (var j = lane; j <= FOLD_LANES; j += FOLD_LANES) {
                        w_fold_at[at + j] = fold_start(run, h + j, up);
                    }
                    workgroupBarrier();
                }
                var span = vec2<u32>(0u);
                if run.y > run.x && h <= run.w - run.z {
                    let a = w_fold_at[at + q];
                    let b = w_fold_at[at + q + 1u];
                    span = select(vec2<u32>(a, b), vec2<u32>(b, a), up);
                }
                for (var k = span.x + lane; k < span.y; k += FOLD_LANES) {
                    fold_apply(op, fold_member(k));
                }
                storageBarrier();
                workgroupBarrier();
            }
            workgroupBarrier();
        }
    }
}
