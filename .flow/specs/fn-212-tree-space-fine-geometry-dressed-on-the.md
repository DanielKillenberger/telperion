# Tree space: fine geometry dressed on the device

## Goal

Owner, 2026-10-07: "Is realtime perf feasible? are we stuck in any way?" and "yes" to starting this spec. Engine trees carry 7 to 48 times the nodes of the shipped presets (spruce at 80: 4.6M against 96k); every CPU preparation pass after stage 2 (rings, stations, curve points) costs about 0.2 to 0.26 µs a node, about 3.9 s for the spruce at 80, and its 3.5 GB of CPU memory broke the browser's 4 GB ceiling. fn-209 grows and lays the fine roots (95 to 99% of nodes) on the GPU, but nothing after stage 2 read those buffers. This spec dresses the fine roots on the device (ring words, leaf stations, curve points) into the buffers the existing GPU stages read, while the structure above the cut keeps the CPU path; the CPU reference executor still defines correct (fn-125's principle). Depends on fn-209 and fn-198; the oak waited on fn-211 task 5.

## Decisions

- Astra's review of the draft (2026-10-07): accept with changes, all taken; R1 to R4 replaced by contract, delete-first audit, a native vertical probe gated on 3x at both spruce-80 seeds, and a browser pass criterion; R5a added (build latency and steady frame reported apart).
- Relations: fn-212 stays separate; fn-125 and fn-171 are reconciled after F4's acceptance; fn-171 does not land first and its obligations move here; one ring implementation.
- Host decisions during the build: D1 withdrawn, D1' delivered by fn-211's contract A; D3 amended to (c), landed roots masked on the device by the LANDED flag; station counts from grown length in whole micrometres on both executors; SHORTEST and the slender bound decided in whole micrometres; option 3, a bearer reads its fine roots through their attachments; frame pacing (b); one destroy-on-drop buffer type; a stated browser budget (8.5 GiB, calibrated on the 10 GB card) refusing builds by name; lobed ring vertices on the device (option (a)); shellDepth's removal goes on fn-198's R0 list.
- The oak and palm keep the CPU executor (`executor::apart` names why).
- The render wasm ceiling raised from 2,200,000 to 2,800,000 bytes with the fine module in production (owner decision, recorded in `scripts/artifact-budgets.json`).

## Acceptance Criteria

- **R1:** A checked input/output contract (identities, partition boundary with ground-routed roots, contacts across the seam, device-reduced envelope, buffer ownership) — met: final routing puts 91 to 96% of kept phytomers on the device; device folds within 4.6 µm of the CPU reference.
- **R2:** A delete-first audit with measured costs — met: foliage cull rejection removed on both executors (GPU peak -258 MiB), beech rings without a reader dropped (2.3 to 1.2 s), spruce at 80 takes the GPU ring path (15.5-18.0 to 7.3-9.2 s, peak 5.6 to 4.2 GB).
- **R3:** A native vertical probe, 3x at both spruce-80 seeds, final-output parity — met: 7.7x (seed 1) and 7.1x (seed 7) on the real path, 9.3x after the parallel bases scan; every run matched, leaves within fn-125's bound or a named class.
- **R4:** The browser path with a measured pass criterion for the spruce at 80 — met: draws in 3.4 s (refused before), largest process 4.9 to 0.8 GB, large-to-small rebuilds succeed, a second 80 beside an 80 refused by name.
- **R5a:** Build latency and steady frame reported apart — met: spruce 80 native change-to-drawn 9 to 11 s to 1.4 to 2.5 s, peak RSS 4.8 to 0.8 GB; steady frame 50 against 47 ms, the gap to fn-176.
- **R5:** Workspace gate, npm test, Codex review — met: Astra's nine implementation findings each reproduced red and fixed (one not reproducible on the RTX 3080, fixed as directed); gate render 176 + CPU 988, npm test green.

## Result

Browser (headless Chromium, before / after): beech 40 to 80 5,718 / 1,442 ms; spruce 40 opening 4,698 / 1,550-1,808 ms; spruce 80 opening refused / 3,248 ms (17.4M leaves); steady frame spruce 80 38.9 ms vegetation, 57.6 ms total. The owner's Chrome lost WebGPU on the dev origins, so browser figures come from fn-198's headless driver. Stills at 80 before and after SHORTEST: indistinguishable.

## Open

- The 6% steady-frame gap and realtime drawing of mature trees: fn-176 (level of detail).
- `telperion-field.wasm` was already over its ceiling before this spec.

## Friction

- Branch tip did not compile (`fine/expansion.rs:405`) -> measured with an uncommitted one-line fix; host to fix on the base.
- Timing passes under loads of 20 to 100 -> a desk-wide quiet window or timing lock.
- GPU lock queued behind whole workspace gates -> a lock with a visible queue, or gates that take it only for GPU tests.
- /tmp full, then out of memory -> build outputs inside each worktree, per-worker memory ceiling, default job limits.
- Split gate failed jev's replay tests; limit guard missed the shoulder sites -> run replays in the render half; classify sites on fn-198.
- `cargo fmt` rewrote untouched files (twice) -> one fmt pass on the base and `cargo fmt --check` in CI.
- Limit guard did not see a `pub const` -> match `pub(crate)`/`pub(super)` too.
- Two GPU probes in one process ran out of device memory; spruce at 80 overflowed the 10 GB card -> one tree per process (`PROBE=` filter, built), device semaphore for whole-tree tests.
- Shell guard refused variable redirects and Rust in heredocs -> `tee`/allowlisted evidence dir; hook could skip quoted heredocs.
- Classifying a residual run took a full probe pass -> env-selected tree and a residual dump.
- No production entry to route the device path -> tasks name the production path in their acceptance.
- Owner's Chrome lost WebGPU; builds hung silently on aliasing and NaN errors -> `Pass::bind` asserts aliasing natively and `tint.mjs` compiles every shader with Tint (built); a CI Tint step proposed.
- `npm test` stopped at a stale catalogue page -> regenerate pages in the same step that re-pins `pins.json`.
- A GPU race this card does not show -> none on this card; a software adapter could test it in CI.

## Evidence

- `.flow/evidence/fn-212-tree-space-fine-geometry-dressed-on-the/task10-browser/tint.mjs` (compiles every generation shader with Tint in headless Chromium)
