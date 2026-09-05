---
satisfies: [R4, R2]
---
# fn-6-branch-generations-below-the-crossover.5 Fill, shed, and the leaf on the twig

## Description
The volume-fill criterion becomes a measurement with thresholds taken first, the shell rule is checked as the load-bearing pruner it now is, and the canopy places its shoots on twig nodes so the leaf hangs on the twig rather than on any wood under a fraction of the trunk.

**Size:** M
**Files:** `src/skeleton/fill.test.ts` (measurement on the new trees), `src/skeleton/shed.ts`, `src/skeleton/shed.test.ts`, `src/canopy/place.ts`, `src/canopy/place.test.ts`, `.flow/specs/fn-6-branch-generations-below-the-crossover.md` (§Measured)
**Touches:** [.flow/specs/fn-6-branch-generations-below-the-crossover.md, src/skeleton/fill.test.ts, harness/skeleton-view.ts, harness/skeleton-view.test.ts, src/skeleton/shed.ts, src/skeleton/shed.test.ts, src/canopy/place.ts, src/canopy/place.test.ts, src/index.ts]

### Approach
- The metrics exist from task 1 (`src/skeleton/fill.ts`), in shedding's shell units, with the twig mark as the terminal classification now. Threshold procedure, as fn-5's R2 did: run both presets at rest on the new pass, record both numbers, set them against the tuft baseline task 1 recorded (the shipped eight-order trees), and choose the shipped thresholds at the value a clay render distinguishes; write the new numbers and the chosen thresholds into §Measured beside the baseline.
- Shed: `shedTwigs` (`src/skeleton/shed.ts:66-120`) keeps its per-node predicate and subtree removal; assert the floor-and-ceiling guard — sheds neither everything nor nothing on both presets — and that the metrics are measured after shedding.
- Canopy: `buildCanopy` (`src/canopy/place.ts:203-413`) accepts the twig mark as the shoot gate when present, falling back to `shootRadius` for skeletons without one; leaf stations follow the twig's `internode` and `stations`. Assert the leaf is a botanical multiple of the twig diameter at rest on both presets (the R3 test in `src/presets/two-trees.test.ts` already measures the ratio; retarget it to twig nodes).

### Investigation targets
**Required:**
- `src/envelope.ts:169-209` — profile and distance, the one shape description
- `src/skeleton/shed.ts:66-120`; `src/canopy/cull.ts:66-201` — the shell predicate and its guard
- `src/canopy/place.ts:203-260` — shoot gate and station walk

**Optional:**
- `src/canopy/cull.test.ts:56-70` — the filled-crown fixture pattern

### Key context
- Two thinning passes share one shell criterion (shed on wood, cull on leaves); keep the depth number identical at both call sites or wood and leaves visibly disagree.
- If the shell rule cannot meet the threshold, record it; a light term is a new spec (spec §Parked unknowns).
### Conductor amendment (2026-09-05, after the first dispatch)
- Path chosen for the twig's leaf stations: `buildCanopy` gains an optional anatomy argument (the `TwigAnatomy` from the skeleton params) implemented in `src/canopy/place.ts`; the harness build (`harness/skeleton-view.ts`, the `buildCanopy` call at ~435) passes `skeletonParams.twigs.twig`, and a harness test proves the stations parameter reaches actual placement (changing `stationsPerInternode` changes the element count). Without the argument the placement falls back to the shoot rule, so plain skeletons keep working. The alternative, carrying anatomy on `TwiggedSkeleton`, is rejected as widening the skeleton contract for a canopy concern.
- `harness/skeleton-view.ts` and `harness/skeleton-view.test.ts` join the Touches for that one call site and its test.
## Acceptance
- [ ] Both presets measured at rest on the new pass with the twig mark; thresholds chosen against task 1's tuft baseline by the stated procedure and recorded in §Measured; the shipped trees hold them
- [ ] Shell rule sheds neither everything nor nothing on both presets
- [ ] Canopy shoots key on twig nodes; the leaf is a botanical multiple of the twig diameter on both presets
- [ ] `npx vitest run src/skeleton src/canopy src/presets` green; `npx tsc --noEmit` green
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
