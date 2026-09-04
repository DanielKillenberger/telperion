---
satisfies: [R4, R6, R8]
---
# fn-1-the-canopy-real-leaf-geometry-culled-to.5 The instanced draw, the clay judgment, and framing that includes the canopy

## Description
Puts the canopy on screen: one instanced draw per element type with alpha test (R4), inside the clay room unchanged (R6), framed so the stage accounts for foliage spread (R8). Carries R7's manual sign-off gate, which cannot be automated.

**Size:** M
**Files:** `harness/skeleton-view.ts`, `harness/stage.ts`, `harness/params.ts`
**Touches:** [harness/skeleton-view.ts, harness/stage.ts, harness/params.ts]

### Approach

- Build one `THREE.InstancedMesh` per element type from the culled instances. This is the repo's first instanced mesh; a repo-wide grep for `InstancedMesh`, `alphaTest` and alpha-to-coverage returns zero hits, so there is no local pattern to mirror and no existing code to conflict with.
- Alpha test or alpha-to-coverage, never blending.
- The placeholder material is the harness's, not the library's. Do not touch the judging-mode defaults: `CLAY` at `harness/stage.ts:30`, the single `HemisphereLight` at `:238`, `NoToneMapping` at `:219`, and `setLightingCheck` staying opt-in and off at `:454-465`.
- R8 is the integration risk. Compute the element geometry's bounding volume before the instanced mesh joins the tree group, or `box.setFromObject` contributes zero extent for it and framing silently ignores the canopy. Then verify room fit, orbit pivot and the near and far planes against a canopy-widened subject.
- Confirm the dispose path releases instance attribute buffers on regenerate. `InstancedMesh` extends `Mesh`, so an existing `instanceof THREE.Mesh` check catches it, but confirm rather than assume.
- Draw-call and instance numbers land in the panel fields the measurement task added.

### Investigation targets

**Required** (read before coding):
- `harness/skeleton-view.ts:193-231` - the build path the canopy joins
- `harness/stage.ts:150-200` - room fit and pivot maths tuned on branch-only spans
- `harness/stage.ts:317-333` - `subjectBox` and `box.setFromObject(tree)`
- `harness/stage.ts:205-225` - renderer settings
- `harness/stage.ts:440-470` - lighting check and dispose

**Optional** (reference as needed):
- `harness/params.ts` - dials for canopy density if judging needs them

### Key context

- Project memory, slack band around an orbit pivot: a tolerance sized by the thing being protected rather than the failure being corrected left the pivot off the subject. R8 is the same surface.
- Project memory, a second subject on stage makes every "what is on screen" read a call site. Two trees each grow a canopy, so grep every read of the tree group rather than patching the one that looks wrong.
- Project memory, an AC that enumerates the default state is the test and a comment is not a licence. R6 enumerates what the clay room emits. A helpful extra such as ambient occlusion or a quiet LOD is not licensed by a comment explaining it.
- Vitest cannot observe WebGL state. Assert instance counts and draw counts on the CPU-side data.

### Acceptance

- [ ] One instanced draw per element type, confirmed by the panel's draw-call count (R4)
- [ ] Cutout transparency by alpha test or alpha-to-coverage, never blending (R4)
- [ ] Clay judging-mode defaults unchanged and the lighting check still opt-in and off (R6)
- [ ] Nothing in `src/` emits a material, a colour or a light (R6)
- [ ] The canopy contributes to subject bounds, so room fit, orbit pivot and near and far account for foliage spread (R8)
- [ ] A canopy with zero elements leaves framing identical to the branch-only framing (R8 error case)
- [ ] An instanced mesh with an uncomputed bounding volume cannot silently contribute zero extent (R8 error case)
- [ ] Regenerating the tree releases instance attribute buffers
- [ ] Owner sign-off recorded: the canopy reads as worth building a scene on, judged in clay (R7 manual gate)
- [ ] `npx vitest run` and `npx tsc --noEmit` green

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
