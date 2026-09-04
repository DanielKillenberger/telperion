---
satisfies: [R1, R7]
---
# fn-5-branch-until-the-tips-bear-leaves-one.2 Branching depth as a parameter, from the panel to both presets

## Description
Exposes branching depth as one named parameter that scales the growth distances together (R1), carried on the panel and stated in both presets, with the default reproducing today's trees byte-identically (R7). Split from the local-rule work because this alone reaches 20 cm wood and 2,473 tips, and it is independently judgeable.

**Size:** M
**Files:** `src/skeleton/grow.ts`, `src/presets/preset.ts`, `src/presets/two-trees.ts`, `harness/params.ts`, `harness/skeleton-view.ts`
**Touches:** [src/skeleton/grow.ts, src/presets/preset.ts, src/presets/two-trees.ts, harness/params.ts, harness/skeleton-view.ts]

### Approach

- One dial, not three. Step and kill distance move together and their ratio is art direction the code already states; the search radius is now spacing-aware from task .1 and composes with the scaled step rather than being scaled itself.
- Decide where the parameter lives and say why: a sibling field on `SkeletonParams` beside `attractors`, or inside `PresetSkeleton.growth`. If it goes in `growth`, the doc comment at `src/presets/preset.ts:41-44` claiming "`growth` carries only bending stiffness" becomes false and is rewritten in the same change.
- Panel: one more `SliderSpec` in `harness/params.ts`, grouped under the existing `"skeleton"` group. `SLIDERS` runs in pipeline order and a dial's position states where in the pipeline it acts, so it belongs near `density` and `turn limit`, not at the end.
- `toSkeletonParams` at `harness/skeleton-view.ts:65-98` translates dials to params field by field with no spread; thread the new field through explicitly.
- The rail's ends are the measured range, not round numbers: 3.26 m down to somewhere near 0.44 m of step, expressed as whatever the parameter's own units turn out to be.
- `maxNodes` sits at 8,000 and the range passes it early. Raise it and report reaching it rather than truncating silently.

### Investigation targets

**Required** (read before coding):
- `src/skeleton/grow.ts:40-91` - `SkeletonParams`, `GrowthConfig` assembly, `defaultGrowth` as task .1 left it
- `harness/params.ts:180-289` - `SliderSpec` including the `group` field, and the pipeline-ordered `SLIDERS` array
- `harness/skeleton-view.ts:65-98` - `toSkeletonParams`, the explicit field-by-field translator
- `src/presets/preset.ts:38-68` - `PresetSkeleton` and `TreePreset`, and the "only bending stiffness" claim
- `src/presets/two-trees.ts` - both preset bodies, every term stated

**Optional** (reference as needed):
- `harness/GrowerDev.tsx:43-46` - `format` infers decimals from `step`, so choose the slider step deliberately

### Key context

- The measured curve is in the spec. Use it to set the rail: 1.63 m gives 844 tips and 33.8 cm wood; 0.89 m gives 2,473 and 20.6 cm at 1.08 M triangles and a 413 ms build.
- Project memory, an AC that enumerates the default state is the test: the preset convention is every term stated and none inherited, so both trees name the depth term explicitly rather than inheriting a default.
- R7 is why the default matters. Depth at its default must reproduce today's trees byte-identically so any later visual change is attributable to a dial the owner moved.

### Acceptance
## Acceptance
- [ ] Branching depth is one named parameter with a documented rail whose ends are the measured range (R1)
- [ ] It scales the growth distances together, preserving the ratios the defaults encode
- [ ] Non-finite values fall back to the documented default through the `held` rail idiom (R1 error case)
- [ ] The panel carries it under the `skeleton` group, in pipeline order
- [ ] Both presets state it in full
- [ ] The default reproduces today's trees byte-identically (R7)
- [ ] `maxNodes` accommodates the rail, and reaching it is reported rather than silently truncating
- [ ] Any doc comment the parameter's placement falsifies is rewritten in this change
- [ ] `npx vitest run`, `npx tsc --noEmit` and `npm run build` green
## Done summary
Branching depth is one parameter, `SkeletonParams.step` - the growth step as a fraction of envelope height, a sibling of `attractors` (not inside `growth`, which stays the metres-denominated override bag). Kill distance follows at two steps, the search radius composes through `influenceRadiusFor`, and `maxNodes` scales as 8000 x (0.022 / step) so the rail's bottom (42,414 nodes on the panel's widest crown at 0.003 h) fits under it; `resolveGrowth` exposes the config a tree grew under and `TreeStats.capped` reports growth that stopped at the ceiling. Non-finite steps are held to `DEFAULT_STEP` = 0.022. Both presets state `step: 0.022`; the panel carries `growth step` right after `density` under the skeleton group, rail 0.003..0.022 h, notch 0.0005. The doc comment the placement falsified - preset.ts "the two optional members stated in full" - is rewritten (the "`growth` carries only bending stiffness" claim stays true because the step is a sibling). R7: 12 pinned skeleton signatures (both presets, every R7 fixture, panel default and panel-dense) byte-identical before and after.

baseline: green (npx vitest run 18 files / 272 tests; npx tsc --noEmit; npm run build) at 24aa80d8
verify: green (npx vitest run 18 files / 280 tests; npx tsc --noEmit; npm run build) at a5f2a319; gate classify: FULL (harness/params.test.ts); receipts unittest/typecheck/build written
tests added: grow.test.ts "at its default ... is the tree it was before the dial existed" (R7 + R1 non-finite error case), "moves step and kill distance together, and derives the rest", "fits the whole rail under the node ceiling"; skeleton-view.test.ts "hands the growth step over", "the node ceiling is reported when the growth stops at it"; params.test.ts "puts the growth step beside density"; two-trees.test.ts key list gains "step"
outside Touches (tests only): src/presets/two-trees.test.ts, src/skeleton/grow.test.ts, harness/skeleton-view.test.ts, harness/params.test.ts
follow-ups (not built): GrowerDev.tsx:417 stats line does not render `capped`; src/index.ts does not re-export DEFAULT_STEP / resolveGrowth
run-note: /home/daniel/Projects/telperion/.git/flow-notes/fn-5-branch-until-the-tips-bear-leaves-one-20260904T203339Z-685936/fn5-t2-step-dial.md

stage: impl-review - skipped(policy: parallel-wave - conductor reviews after integration; REVIEW_MODE=none)
## Evidence
- Commits: a5f2a31996f7f91c7a17603715c5029e2f01bbce
- Tests: npx vitest run (18 files / 280 tests on the integrated target), npx tsc --noEmit clean, npm run build clean, byte-identity at the dial's default verified by the conductor with an independent node hash: Telperion 808 nodes / Laurelin 2442 nodes, hashes unchanged from the pre-task tree
- PRs: