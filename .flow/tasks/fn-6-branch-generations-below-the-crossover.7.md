---
satisfies: [R6]
---
# fn-6-branch-generations-below-the-crossover.7 The cost on the named machine, and the docs that still say orders

## Description
Both presets are measured on the RTX 3080 with the rig against the strategy's 2 ms hero budget and the build timer, the results go into the spec, and every module header, the README, the barrel comment and the harness comments that still describe orders of twigs are rewritten to the branch-generation model. Finalisation folds into one task by convention.

**Size:** M
**Files:** `.flow/specs/fn-6-branch-generations-below-the-crossover.md` (§Measured), `harness/stage.ts` (sweep reaches the native ratio), `README.md`, `src/index.ts`, `src/skeleton/twigs.ts` (header), `src/skeleton/grow.ts` (header, field docs), `src/radius.ts` (header), `src/presets/preset.ts`, `harness/GrowerDev.tsx` (perf comments), `harness/skeleton-view.ts` (comments)
**Touches:** [harness/stage.ts, README.md, src/index.ts, src/skeleton/twigs.ts, src/skeleton/grow.ts, src/radius.ts, src/presets/preset.ts, harness/GrowerDev.tsx, harness/skeleton-view.ts, .flow/specs/fn-6-branch-generations-below-the-crossover.md]

### Approach
- Measurement: with the harness at `devicePixelRatio` 2 and vsync off, extend `SWEEP_RATIOS` (`harness/stage.ts:138`) so the sweep reaches the display's own ratio (applied 2.00 on the named machine), then sweep both presets at rest with the GPU timer rig (`describeSweep`, `:198`); R6 is judged on the GPU figure at the native applied ratio, and the lower ratios are kept as diagnostics; record nodes, twigs, leaves, triangles, CPU build, GPU per frame, on both presets and in the comparison view. Hero budget 2 ms; if a preset misses it, report the number as the reason the rendering spec is needed sooner, do not clamp. Record the timer-query-unavailable path prints "no gpu timing available".
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
- [ ] Hero tree inside 2 ms of GPU time at the display's native pixel ratio (applied 2.00) on both presets, or the miss is reported as the reason for the rendering spec; lower ratios recorded as diagnostics
- [ ] `grep -rn 'orders\|twig pass\|eight orders' src harness README.md` returns no stale description; module headers, README, barrel comment rewritten
- [ ] fn-5 R8 annotated as superseded
- [ ] `npx vitest run` and `npx tsc --noEmit` green
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:

