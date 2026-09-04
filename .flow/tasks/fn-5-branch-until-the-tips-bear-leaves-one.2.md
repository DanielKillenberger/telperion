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

- [ ] Branching depth is one named parameter with a documented rail whose ends are the measured range (R1)
- [ ] It scales the growth distances together, preserving the ratios the defaults encode
- [ ] Non-finite values fall back to the documented default through the `held` rail idiom (R1 error case)
- [ ] The panel carries it under the `skeleton` group, in pipeline order
- [ ] Both presets state it in full
- [ ] The default reproduces today's trees byte-identically (R7)
- [ ] `maxNodes` accommodates the rail, and reaching it is reported rather than silently truncating
- [ ] Any doc comment the parameter's placement falsifies is rewritten in this change
- [ ] `npx vitest run`, `npx tsc --noEmit` and `npm run build` green

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
