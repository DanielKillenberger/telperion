# The whole grower runs on the GPU

## Goal

fn-245's third stage. The CPU grew structure above a fine-root cut and the GPU grew each chain alone in one invocation (`chain.wgsl`), which cannot carry feedback between branches. Astra's architecture review (fn-245 `ASTRA-ARCH.md` §1, §4) proposed every axis on the device grower, scheduled in bounded blocks with dispatch boundaries for dependencies, so the CPU/GPU cut and the expected-phytomer predictor that placed it can go. Release item for fn-237. Scope: execution placement only; the growth law is fn-255's.

## Decisions

- Council plan (host, 2026-10-11): the owner asked for a council of Fable and Astra; both signed `.flow/evidence/fn-245-a-breakthrough-in-generation-time/COUNCIL.md`, which governs this spec where it differs from the requirements. Early test 3 (dispatch probe, busy-cycle replay) before the full port decides the hierarchy schedule, deposit and sweep forms; then early test 5.
- The spike (SPIKE.md): dependency overhead at most ~14 % of the warm budget, every integer output agreed, no device loss; the design followed (bounded subtree blocks of 1,024 growing buds, one reduction order, integer deposit, inline light read, retirement with fn-255's three-cycle wait).
- The owner's two Chrome probes (2026-10-11): indirect dispatches cost ~430 µs each (Dawn's validation), direct ~5-7 µs, so every dispatch is direct; the owner accepts device creation and compile as a once-per-page cost (~0.5 s a page), outside the per-tree budget (Telperion 82: 1.2 s warm, ~340 dispatches a cycle).
- Host's rulings (2026-10-12, DESIGN.md §9): plain f32 arithmetic with the lineage tolerance (identity integers exact; exact mode deleted: 34 % more GPU time and 96 s cold compile); no subnormal emulation; cold compile under 2 s is a blocker; the settle's leaf mean is the canonical blocked reduction on both executors; the 2,048-cycle limit stays a named error.
- Host decision (ii), 2026-10-12: the palm's retained leaf bases are their own spec (fn-266), on which fn-256 depends; until then the resident palm is refused by that capability, never by species.
- Host's rulings (2026-10-12, task 5): a device budget guard (native 4 GiB default, at most 85 % of what other processes leave; reserve before create; charges held until completion); identity is the lineage key, never a slot; one fixed arena per build sized from the closed-form hint, compacted in place; restarts and table moves deleted; every overflow refused by name.
- Host's rulings (2026-10-12, task 12): retirement's inside marks only before compaction and the settle; a dormant lattice skips its deposit; units rounds sized from rounds used plus a stated slack, refused by name past it.
- Upward sums with dirty bits measured and deleted (marking cost what skipping saved); retirement reuse kept.

## Acceptance Criteria

- **R1:** one executor chosen per build context, never per species; every axis on the device grower; no per-cycle CPU botanical work; a capacity failure is a named error, never a CPU rebuild — met (CPU cycle stays the reference).
- **R2:** compact arrays with stable ranges; resident queues and totals; block size and scheduling never change the tree — met (node table byte-identical over 10 contexts and caps 64/128/1,024 on all 8 presets).
- **R2a:** the device's node table repeats run to run for every preset (fn-261's probe made a test) — met.
- **R3:** the fine-root execution cut deleted (and the predictor if no other consumer); capacity from checked counts and bounded allocation; the unbent-before-bent dependency kept — met (every fn-255 R3B.md deletion done or kept for a named site-path reader, DESIGN.md §12.5).
- **R3a:** CPU peak during preparation at or under master's — met in substance: 0.77-1.01 GB every preset (F4: 3.6-9.2 GB); the process floor alone is 0.77 GB, so not comparable with master's 0.35 GB.
- **R4:** CPU-GPU agreement on every preset; build time per preset beside master and the branch; cold compile; stills viewed; Astra's agreement; gate and npm test green — met except the warm-build targets, each miss measured (below).

## Result

- Agreement (`resident/agree_tests.rs`, every preset at 12): nodes equal in lineage, positions within 3e-5 m, stations, wood runs and section rows equal; the palm's leaflets within 3.2e-5 of height.
- Cold compile: 1.54 s native (cache off), then ~1.77-2.0 s after later tasks, against 67 s at the first port and fn-254's 11.7 s. Longest dispatch 1.24 ms.
- Warm build, seed 42 (task 11, pre-light): oak 261 ms, beech 190, birch 230, spruce 451, ordinary 458, palm 77, Laurelin 1,177, Telperion 1,276. With every species lit (task 13, native f32 light, skewed-tile sweep): lit Telperion's light phase 746.7 → 26.0 ms, warm build ~1,252 ms; Laurelin ~1,156 ms.
- Dispatches a cycle (task 12): Telperion 365, Laurelin 360, beech 295, birch 320, palm 230, spruce 378, oak 339, ordinary 408 (target ~340).
- Against 0.1.4 at its older ages: at or under on the beech and birch; over on the oak (1.5-1.7×), spruce (2.8×), Laurelin (2.1-4.2×), Telperion (1.5× seed 1, 15× seed 42), ordinary (7.5-7.9×), palm (13×). Against F4: slower on the Two Trees (1.9×), spruce and palm.
- Two races (`fault_out`, `node_laterals`) reproduced red and fixed; the lattice dormant where nothing shades; the roofline (ROOFLINE.md): every phase but compaction runs 6-270× above its roofline time (latency, not bandwidth); laterals living under 5 years carry 62-76 % of grown phytomers and 61-77 % of GPU time.
- Every preset's stills viewed beside F4's: all keep F4's look (Telperion at share 0.3 and Laurelin as approved; the palm with its boots). Astra AGREE (round 3). Codex SHIP on every task. Gate and npm test (162) green.
- Arena: all 24 builds (8 presets × seeds 1, 7, 42) grow with 0 moves; a 450 m tall narrow pole builds.

## Open

- Warm-build targets missed against F4 (Two Trees ~1.2 s against 0.67 s) and dispatches above ~340 on some presets: the remaining routes are fn-245's scale split (task 14, form (d)) and log-depth hierarchy sums (task 16, experiment).
- The oak at 80 exceeds 4 GiB on its own axes hint (the closed form is light-neutral); the oak at 100 does not build on this card.
- Telperion's production build peaks at 4,209 MB, 85 MB under the 4 GiB native default.
- The owner's Chrome probe of the whole tree is built and typechecked but was not run here.
- `the_roofline_is_measured` and `the_executor_is_measured` no longer run on lit presets.

## Friction

62 entries in FRICTION.md (kept; cited by `scripts/build-wasm.mjs`). Recurring: `flowctl` not on PATH for dispatched workers (local setup); the dcg hook and quota'd tmpfs scratch (local setup); GPU hangs and device out-of-memory during measurements -> the in-shader halt, the device budget guard and `timeout` on GPU runs (built); two workers sharing one worktree or index -> one worker per worktree (practice); WGSL reserved words and naga checks only on the GPU -> a standalone shader check (proposed); `scripts/build-wasm.mjs` ignored `CARGO_TARGET_DIR` -> fixed (fn-237 R15); cold compile not gated per task -> a gated measurement (proposed); stale timing harnesses on lit presets -> follow-up.

## Evidence

All of `.flow/evidence/fn-256-the-whole-grower-runs-on-the-gpu/` is kept: DESIGN.md (cited by section from the device cycle's source), SETTLE-PLAN.md, HIERARCHY.md, LAYOUT.md, LIGHT.md, UNITS.md, EXECUTOR.md, ROOFLINE.md (+ `roofline.py`), PROOF.md (per-task tables), SPIKE.md, CYCLE.md, GUARD.md, RETIRE.md, SETTLE.md, STEP2.md, `exact-layout.patch`, `exact-units.patch`, `full-card.sh` (loaded by `full_card_tests.rs`), FRICTION.md.
