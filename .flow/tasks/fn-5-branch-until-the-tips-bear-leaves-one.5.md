---
satisfies: [R5, R6]
---
# fn-5-branch-until-the-tips-bear-leaves-one.5 What it costs: the curve, the counts, and a build you can still drag

## Description
Measures what the recursion costs and whether the counts reach the botanical order (R5, R6). Separate from the branching work because the measurement is most of the job and because the answer decides whether the spec's Boundary against an impostor survives.

**Size:** M
**Files:** `src/skeleton/colonize.ts`, `src/skeleton/grow.ts` (the node ceiling must scale with twig orders as well as step), `src/skeleton/shed.ts` (interior shedding as its own pass after the twigs, so it never collides with .4's continuity work in twigs.ts), `harness/skeleton-view.ts`, `harness/GrowerDev.tsx`
**Touches:** [src/skeleton/colonize.ts, src/skeleton/grow.ts, src/skeleton/shed.ts, src/skeleton/shed.test.ts, harness/skeleton-view.ts, harness/GrowerDev.tsx]

### Approach

- **Attractor association is the first cost.** `settle()` at `src/skeleton/colonize.ts:243-257` is an O(new nodes x attractors) linear scan with no spatial index. Every serious implementation uses a grid or kd-tree; at the node counts this spec reaches it stops being optional. A uniform grid is the smaller change and the envelope gives natural bounds.
- **The rig from the canopy work already measures GPU frame cost** with timer queries at vsync off across a four-point resolution sweep. Use it; do not build a second one.
- **Record the curve across the depth rail on both presets**: nodes, tips, finest wood, triangles, build time and GPU frame cost. This is the artifact R6 asks for and it belongs in the spec, not in a scratch script.
- **Interactive build is the user-facing cost.** A full build was 194 ms at today's depth and 413 ms at a 0.89 m step, before the local pass exists. Measure first, then decide between debouncing, a coarse preview while dragging, or nothing.
- **Report reaching `maxNodes`** in the panel rather than truncating silently.
- **Count leaves and state the reachable ceiling.** R5 wants the order compared against the botanical 10^5 to 10^7, and the count reachable at the top of the rail reported rather than clamped.

### Investigation targets

**Required** (read before coding):
- `src/skeleton/colonize.ts:229-260` - the attractor arrays and `settle()`, the scan to index
- `harness/stage.ts:205-225` - the renderer, the log-depth toggle and the pixel-ratio pinning
- `harness/skeleton-view.ts:132-152` - `TreeStats`, already carrying draw calls, instances and both pixel ratios
- `harness/GrowerDev.tsx:280-300` - the panel line and the sweep control

**Optional** (reference as needed):
- `src/mesh/surface.ts:319-320` - `positions`/`indices` as plain arrays pushed one number at a time before a final typed-array conversion, a likely allocation hotspot at a million triangles

### Key context

- A 4x4 per-instance transform is 64 bytes, so eight million elements is about 512 MB before geometry. A packed format is the difference between the top of R5's range being reachable and being arithmetic.
- The spec's Boundaries admit a far impostor only if the measurement demands one, and the research says it will somewhere inside R5's own range. Report the number that settles it; do not relax the Boundary by assertion and do not discover it at 400 MB.
- WebGL2 has no compute and no writable storage buffers, so instance transforms are built on the CPU and uploaded. Changing renderer is out of scope; the constraint is recorded because it bounds what the top of the rail can mean.
- A geometry crossing 65,535 vertices silently doubles its index memory when three promotes the index to `Uint32`. Put it in the table.
- Vitest runs in Node with no GPU, so anything about draw counts or frame cost is asserted on CPU-side data and the frame budget is a harness procedure the owner runs.

### Acceptance
## Acceptance
- [ ] Attractor association uses a spatial index rather than a linear scan, with the speedup measured at the top of the rail
- [ ] The cost curve is recorded across the depth range on both presets: nodes, tips, finest wood, triangles, build time, GPU frame cost (R6)
- [ ] Leaf count is measured on both presets and stated against the 10^5 to 10^7 range, with the reachable ceiling reported (R5)
- [ ] A count the frame budget cannot carry is reported as the measured ceiling, never quietly clamped (R5 error case)
- [ ] Reaching `maxNodes` is reported rather than silently truncating (R6)
- [ ] Building at depth stays usable while a dial is dragged, by whatever the measurement shows is needed (R6)
- [ ] When the timer-query extension is unavailable the panel says so and reports no timing number (R6 error case)
- [ ] The done summary states whether the measurement demands an impostor, with the number behind the answer
- [ ] `npx vitest run`, `npx tsc --noEmit` and `npm run build` green
- [ ] Interior twigs are shed by the shell rule the leaf culler already uses, and the leaf count is reported after shedding (R5)
- [ ] A shell rule that sheds everything or nothing fails, by the same floor-and-ceiling guard the leaf culler carries (R5 error case)
- [ ] The depth both presets ship at renders inside 16.7 ms at the display's native pixel ratio on the RTX 3080, measured with the rig at vsync off (R6)
- [ ] The reachable ceiling in R5 is the count inside that frame, not the count before the tab dies (R5)
- [ ] A shipping depth that cannot meet the frame is reported as the reason an impostor is demanded (R6 error case)
## Done summary
Attractor association in `colonize.ts` now folds each new node through a uniform grid over the attractor cloud (cell = the wider of the search radius and the kill distance, capped at 32 cells per axis) instead of scanning every attractor; 19 skeleton signatures are byte-identical before and after, and at the bottom of the rail colonization runs 1.5-1.6x faster (Telperion 0.003h 175 -> 108 ms, Laurelin 352 -> 231 ms, median of 5), with the association's own cost halved and the bias field's noise now the first cost. `src/skeleton/shed.ts` applies the leaf culler's shell rule to the twig pass's nodes - a twig survives on the first node of its subtree not definitely interior, the predicate fails safe, `DEFAULT_SHED` IS `DEFAULT_CULL` - hooked into `growSkeleton` after the twigs. The node ceiling scales with the twig orders and children (`twigHeadroom`: 1 + 0.25 x sum of children^order, exact at zero orders so R7 holds bit for bit) under a 250,000 absolute stop; `growReport` returns `capped` and `shed` beside the skeleton because a shed tree is smaller than the ceiling it hit. The panel says "node ceiling reached" in words, labels instances as leaves, and settles a build dearer than 80 ms until 250 ms after the last notch while a cheap one still builds per notch (measurement: 1.0 s Telperion / 2.2 s Laurelin per build at eight orders - deferral, not coarsening, is what the numbers asked for).

R5, measured after shedding and culling at the default step: Telperion 2,904 / 14,310 / 40,523 / 136,686 / 519,001 leaves at 0/4/6/8/10 orders; Laurelin 8,115 / 59,220 / 204,321 / 766,390 at 0/4/6/8. Both reach 10^5 at eight orders and Laurelin 10^6 at ten; the reachable ceiling before the node stop is about 0.8M (Telperion, 12 orders) and 1.1M (Laurelin, 10) - scaled from the 1.25M / 1.76M measured at a 400k ceiling before it was set to 250k. The ceiling INSIDE THE FRAME is not known: GPU frame cost was not measured (no GPU-backed browser was driven inside the timebox; the fn-1.1 CDP route and the panel's sweep button are the procedure), so the shipping depth and the impostor verdict await the owner's sweep. What the CPU side says: at eight orders the scene is 5.3M triangles (surface 3.1M + 137k leaves x 16) and 8.3 MB of instance transforms on Telperion, 19.6M triangles and 47 MB on Laurelin; fn-1.1 measured 59k triangles at 0.21 ms, so six orders (1.5M / 5.2M triangles) is likely inside 16.7 ms and eight on Laurelin is the row the sweep has to settle. Memory never demands an impostor inside the ceiling - 107 MB at the 400k cap, not the 512 MB the spec feared, because shedding and the stop hold the count near 1.7M. Shedding removes 8-11% of twig nodes on both presets at every depth and the leaf cull then finds under 1% more; it buys realism more than budget here.

Tests: src/skeleton/shed.test.ts (5: sheds-a-part-and-leaves-the-rest on both presets - the culler's two-sided guard, R5 error case; colonization nodes untouched and parents re-indexed; interior base with a line to the shell kept while a buried line is shed; shell of 1 and NaN shell; unplaceable twig and no-twig rails); src/skeleton/grow.test.ts +2 (ceiling exact at zero orders, scales with orders/children, stops at 250k; growReport capped/shed after shedding). Timer-unavailable panel text (R6 error case) is fn-1.1's existing describeSweep test, unchanged.

baseline: green (npx vitest run 19 files / 302 tests; npx tsc --noEmit; npm run build) at 04e027738a145cf8118d5d49f976a92a858bf47c
verify: npx vitest run 20 files: 19 passed, 309/310 tests - the one red is src/skeleton/twigs.test.ts:205 (sibling-owned, asserts growSkeleton returns exactly maxNodes nodes, which shedding makes false by design; move to growReport: nodes + shed === ceiling - see the run note); npx tsc --noEmit clean; npm run build clean (dist/telperion.js 46.74 kB); gate classify: FULL. No receipt written (suite not green).

outside Touches (tests only): src/skeleton/grow.test.ts (+2 focused tests). Nothing else.
follow-ups (not built): lift envelopeProfile/distanceToProfile from cull.ts onto src/envelope and import in shed.ts; shed depth is not a dial (DEFAULT_CULL's 0.45, one rule one number); src/index.ts does not re-export growReport / shedTwigs / DEFAULT_SHED; the GPU sweep at depth; the half-reach grid cell and the bias-field cost are the next colonize wins.
run-note: /home/daniel/Projects/telperion/.git/flow-notes/fn-5-branch-until-the-tips-bear-leaves-one-20260904T203339Z-685936/fn5-t5-cost.md ; measurements: /tmp/claude-1000/-home-daniel-Projects-telperion/1941ff7f-8640-487d-bf47-b4616d50c1eb/scratchpad/fn5-t5-curve.tsv, fn5-t5-bench.txt, fn5-t5-pre-signatures.txt / fn5-t5-post2-signatures.txt

stage: impl-review - skipped(policy: parallel-wave - conductor reviews after integration; REVIEW_MODE=none)
## Evidence
- Commits: 2421f9d8c7f4c8508eae207f8b74b711cfc4abfd, 776a011
- Tests: npx vitest run (21 files / 325 tests on the joined target), npx tsc --noEmit clean, npm run build clean, byte-identity under the spatial index by the conductor's node hash: unchanged, GPU frame cost not measured by the worker; the conductor runs the sweep at quiesce
- PRs: