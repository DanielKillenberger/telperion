---
satisfies: [R6]
---
# fn-6-branch-generations-below-the-crossover.7 The cost on the named machine, and the docs that still say orders

## Description
Both presets are measured on the RTX 3080 with the rig against the strategy's 2 ms hero budget and the build timer, the results go into the spec, and every module header, the README, the barrel comment and the harness comments that still describe orders of twigs are rewritten to the branch-generation model. Finalisation folds into one task by convention.

**Size:** M
**Files:** `.flow/specs/fn-6-branch-generations-below-the-crossover.md` (§Measured), `harness/stage.ts` (sweep reaches the native ratio), `README.md`, `src/index.ts`, `src/skeleton/twigs.ts` (header), `src/skeleton/grow.ts` (header, field docs), `src/radius.ts` (header), `src/presets/preset.ts`, `harness/GrowerDev.tsx` (perf comments), `harness/skeleton-view.ts` (comments)
**Touches:** [.flow/evidence/fn6-task7/**,src/skeleton/shed.ts, src/envelope.ts, src/radius.test.ts, src/skeleton/continuity.test.ts, harness/stage.ts, harness/stage.test.ts, .flow/specs/fn-5-branch-until-the-tips-bear-leaves-one.md, README.md, src/index.ts, src/skeleton/twigs.ts, src/skeleton/grow.ts, src/radius.ts, src/presets/preset.ts, harness/GrowerDev.tsx, harness/skeleton-view.ts, .flow/specs/fn-6-branch-generations-below-the-crossover.md]

### Approach
- Measurement: with vsync off at the observed native browser pixel ratio and explicit applied 2.00 high-DPI diagnostic, extend `SWEEP_RATIOS` (`harness/stage.ts:138`) so the sweep reaches the display's observed ratio (monitor scale 1 on 2026-09-05), retaining explicit applied 2.00 as a high-DPI diagnostic, then sweep both presets at rest with the GPU timer rig (`describeSweep`, `:198`); R6 is judged on the GPU figure at the native applied ratio, and the lower ratios are kept as diagnostics; record nodes, twigs, leaves, triangles, CPU build, GPU per frame, on both presets and in the comparison view. Hero budget 2 ms; if a preset misses it, report the number as the reason the rendering spec is needed sooner, do not clamp. Record the timer-query-unavailable path prints "no gpu timing available".
- Docs, per the docs-gap scan: `README.md:40, 52`; `src/index.ts:8-11, 71-82`; `src/skeleton/twigs.ts` header (all of `:11-108` as rewritten by tasks 2 and 4, checked for leftover "orders"); `src/skeleton/grow.ts:46-57, ~91, ~129-151`; `src/radius.ts:57-82, 105-132, 214-216`; `src/presets/preset.ts:40-52`; `harness/GrowerDev.tsx:51-53, 94-99`; `harness/skeleton-view.ts:85-95`. Grep for `orders`, `levels`, `twig pass`, `eight orders` across `src harness README.md` and resolve every hit.
- Note in fn-5's spec, under R8, one line: superseded by fn-6 R3 and R7. Do not close fn-5 here; report it.

### Investigation targets
**Required:**
- `harness/stage.ts:138-211` — the rig and its honesty rule
- The docs-gap list above, file by file

**Optional:**
- `.flow/specs/fn-5-branch-until-the-tips-bear-leaves-one.md` §Measured — the table format the new numbers extend

### Build-cost handoff from accepted task 8 (2026-09-05)
Task 8 is visually accepted. Its historical approximately-three-second build criterion remains unmet: diagnostic full CPU pipeline samples were 3.808 s baseline and 5.014 s corrected Telperion on a shared machine, excluding GPU upload/render. These are single samples, not a precise performance regression estimate. This task owns final representative CPU/GPU measurements and explicit reporting of the miss; do not claim the old target passed or lower it. Carry any required performance follow-up into the migration/rendering decision.

### Key context
- The strategy's Frame metric is 2 ms of GPU time for a hero tree, not the 16.7 ms fn-5 measured against.
## Acceptance
- [ ] Final CPU measurement explicitly assesses task 8's unresolved ~3 s full-build target; report any miss and follow-up without claiming a pass.
- [ ] §Measured carries nodes, twigs, leaves, triangles, CPU build and GPU per frame for both presets at rest on the RTX 3080, with the machine and settings named
- [ ] Hero tree inside 2 ms of GPU time at the display's observed native pixel ratio, with applied 2.00 also recorded as a high-DPI diagnostic on both presets, or the miss is reported as the reason for the rendering spec; lower ratios recorded as diagnostics
- [ ] `grep -rn 'orders\|twig pass\|eight orders' src harness README.md` returns no stale description; module headers, README, barrel comment rewritten
- [ ] fn-5 R8 annotated as superseded
- [ ] `npx vitest run` and `npx tsc --noEmit` green
## Done summary
The default sweep now includes capped native resolution, DPR 2 and the lower diagnostics through one helper shared with the controls and running label. GPU points retain acquisition-order query samples without changing the eight-frame warmup, twenty-sample median or no-fallback rule. README, API and module/harness comments describe radius-derived branch generations, branch-local endRadius taper and fixed terminal twig anatomy; the README canopy example passes anatomy. FN-5 R8 is annotated as superseded, and FN-5 remains open.

The conductor measured actual RTX 3080 GPU queries at native DPR 1 and explicit high-DPI DPR 2, then supplied the checked-in raw results, reproduction runner and final FN-6 Measured section. Native GPU medians of 5.847 ms Telperion and 3.425 ms Laurelin miss the 2 ms hero budget. CPU builder medians are 4.923 s and 2.421 s; Telperion misses the historical approximately-three-second target. Comparison is recorded separately. Submitted scene triangles include foliage and room, and are distinguished from wood triangles. The misses motivate rendering work and CPU migration without reducing geometry or relaxing a target. The unsupported timer path returned no timing number on a separate real WebGL context with the extension hidden. Scope included no generator behavior or preset geometry changes.

Investigation reused the existing sweep, median and panel controls, read the required documentation targets and relevant comparison-state memory, and swept sibling terminology. The new ratio cases failed first because the helper did not exist. Final npx vitest run passed 363 tests in 23 files (84.76 s), and npx tsc --noEmit passed. Existing unsupported/incomplete reporting tests passed unchanged. Baseline was green via predecessor handoff and a honored full-suite receipt. No test suite ran during the conductor's browser measurements. The final report and raw artifact were added after verification without executable source edits.

GATE_SKIPPED:unittest:green-receipt 0e0711e2 - baseline reused from prior post-gate pass

stage: impl-review - skipped(config: REVIEW_MODE=none; user requested no impl-review)
## Evidence
- Commits: 4027b078a5ca840ca62b69e0a2641ce972de1c81
- Tests: baseline: green via handoff (363-test change begins from 360 tests and typecheck green at 0e0711e2; subsequent predecessor changes docs only), GATE_SKIPPED:unittest:green-receipt 0e0711e2 - baseline reused from prior post-gate pass, npx vitest run harness/stage.test.ts: red before implementation, 4 cases fail because sweepRatios is not a function; /tmp/fn6-task7-red.log, npx vitest run: exit 0, 363 tests in 23 files, 84.76 s; /tmp/fn6-task7-full.log, npx tsc --noEmit: exit 0; /tmp/fn6-task7-types.log, git diff --check: exit 0, Actual RTX 3080 browser CPU/GPU measurements and simulated timer-unavailable WebGL context: .flow/evidence/fn6-task7/results.json; reproduction .flow/evidence/fn6-task7/measure.mjs, Terminology sweep: remaining orders matches are mathematical magnitudes; remaining levels matches assert retired key absence
- PRs: