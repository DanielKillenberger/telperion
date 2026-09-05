---
satisfies: [R1, R8]
---
# fn-6-branch-generations-below-the-crossover.4 Laterals along the limbs, the budget that survives them, and the prefix proof

## Description
Colonization nodes whose wood is under a stated radius bear laterals under the same law, from their own arrival direction and their own phase, so a node bears the same laterals whether it is currently a tip or a limb. The node budget is re-derived for a pass that no longer branches only from tips. R8's prefix test is written here, because it only means something once interior nodes bear wood.

**Size:** M
**Files:** `src/skeleton/twigs.ts`, `src/skeleton/twigs.test.ts`, `src/skeleton/grow.ts` (`defaultGrowth`, `twigHeadroom`, `GrowthReport`), `src/skeleton/grow.test.ts`, `src/presets/two-trees.ts` (the new `limbRadius` term), `src/presets/two-trees.test.ts`
**Touches:** [src/skeleton/twigs.ts, src/skeleton/twigs.test.ts, src/skeleton/grow.ts, src/skeleton/grow.test.ts, src/presets/two-trees.ts, src/presets/two-trees.test.ts, harness/skeleton-view.ts, harness/params.ts]

### Approach
- New param `limbRadius` (fraction of trunk radius, rail 0..1, rest measured): every colonization node past the root with `field.radius[i]` under it seeds a lateral whorl of `laterals` at the divergence, phase seeded per node from a deterministic function of the node index, frame from the node's arrival direction (`arrival` in `src/skeleton/colonize.ts:~316` shows the derivation). No leader for interior nodes. Colliding against the arrival direction is already the rule from task 2.
- Bare-trunk guard applies to interior origins (memory: `a-trunk-region-guard-on-the-parent-node`, test both endpoints).
- Budget: replace `twigHeadroom` (`src/skeleton/grow.ts:153-159`) with an estimate from the law: handoff count × nodes-per-branch summed over the derived generations, using task 1's `generationsUntilTwig` on the field's median handoff radius; keep `NODE_CEILING`. `GrowthReport` gains a `levelCapped` count distinct from `capped` (node ceiling), both reported.
- R8 prefix test: grow a preset's colonization; take a prefix at a round boundary (add a `rounds` read-out to `colonize`'s result, or reproduce the boundary by counting nodes per round in the test with the same config — pick the one that does not change colonization's output); run the pass on the prefix under the MATURE tree's field values for the nodes present; assert every node present in both bears byte-identical laterals, tips differing only by the leader continuation.
- Measure and record: nodes, tips, handoffs, level-capped handoffs on both presets at rest; both finish uncapped.

### Investigation targets
**Required:**
- `src/skeleton/twigs.ts` (as of task 2) — frontier seeding, collision rule
- `src/skeleton/colonize.ts:439-575` — the round loop; nodes are pushed inside it, so the array at a round boundary is a prefix
- `src/skeleton/grow.ts:140-160, 259-283, 315-345` — ceiling, headroom, report

**Optional:**
- `src/canopy/place.ts:221-224` — `shootRadius` as a fraction of trunk radius; the same idiom for `limbRadius`

### Key context
- The memory entry `zero-width-is-not-no-constraint` applies: below the crown base the envelope has no width, a lateral there is outside the silhouette.
- Build time is the constraint; if Laurelin's interior laterals push the build past what task 7 can measure interactively, the rest value of `limbRadius` is where the trade is made, and it is recorded.

## Acceptance
- [ ] Colonization nodes under `limbRadius` bear laterals from their own arrival direction and per-node phase; no lateral below the bare-trunk line
- [ ] The node budget derives from the law; both presets finish uncapped at rest; `GrowthReport` reports level-capped handoffs separately from the node ceiling
- [ ] R8: on a round-boundary prefix under the mature field, every node present in both bears byte-identical laterals; a field of the wrong length is reported
- [ ] Presets state `limbRadius`; structural-key test follows; harness round-trip compiles
- [ ] `npx vitest run src/skeleton src/presets harness` green; `npx tsc --noEmit` green

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
