---
satisfies: [R2, R7]
---
# fn-1-the-canopy-real-leaf-geometry-culled-to.3 Shoots and placement: phyllotaxis, clumping, orientation bias, and determinism

## Description
Derives shoots from the skeleton and places elements along them (R2), and pins the whole canopy to its seed (R7). Emits per-instance transforms only; it never inspects element geometry, which is what lets it run alongside the element task.

**Size:** M
**Files:** `src/canopy/shoots.ts`, `src/canopy/place.ts`, `src/canopy/place.test.ts`, `src/presets/two-trees.ts`
**Touches:** [src/canopy/shoots.ts, src/canopy/place.ts, src/canopy/place.test.ts, src/presets/two-trees.ts, src/presets/preset.ts]

### Approach

- Reuse `branchPaths` (`src/mesh/paths.ts:69`) for run decomposition, including its `stands[]` alias resolution at `src/mesh/paths.ts:77-105`. Do not re-walk the raw skeleton and do not re-derive zero-length-edge folding.
- Reuse `transportFrames` (`src/mesh/frames.ts:77`) for shoot-local orientation instead of building a tangent frame.
- A shoot is a terminal run, per the spec's contract. Trunk-only trees have exactly one.
- RNG sub-stream: `createRng((seed ^ CONST) >>> 0)`, the convention at `src/torsion.ts:173`. That site is the only XOR'd stream in the repo and uses `0x5b_f0_3d_11`; the skeleton (`src/skeleton/grow.ts:83`) and the noise field (`src/noise.ts:107`) take the caller's seed raw. Pick any constant not in that set.
- Scatter shape follows `sampleEnvelope` at `src/envelope.ts:127-150`: attempts-per-point loop driven by one `Rng`.
- Both presets state a canopy block. The convention at `src/presets/preset.ts:22-26` is every term stated and none inherited.

### Investigation targets

**Required** (read before coding):
- `src/mesh/paths.ts:69-137` - run decomposition and `stands[]`
- `src/mesh/frames.ts:38-135` - the `Frame` type and `transportFrames`
- `src/torsion.ts:165-180` - sub-stream derivation, the pattern to copy
- `src/envelope.ts:127-150` - the scatter loop shape
- `src/presets/preset.ts:22-40` - the every-term-stated preset rule

**Optional** (reference as needed):
- `src/skeleton/colonize.ts:93-104` - `SkeletonNode` and `Skeleton`

### Key context

- Project memory, trunk-region guard: a guard on a region must test both ends of every edge crossing it, not just the parent node. Phyllotaxis walking a run crosses region boundaries repeatedly.
- Determinism is array-backed iteration only. A `Map` or `Set` anywhere in the candidate path is how R7 fails intermittently and is painful to reproduce.
- Two trees stand on stage at once and each carries its own seed, so their canopies must not correlate.
- No repeated-run determinism test exists anywhere in the repo today. Seeds appear only in fixtures, so this is a new pattern rather than one to copy.

### Acceptance

- [ ] Shoots derived as terminal runs through `branchPaths`, reusing `stands[]` rather than re-deriving it
- [ ] Phyllotaxis along the shoot, clumping at its end, and outward and upward orientation bias, each a named parameter with a default (R2)
- [ ] A skeleton with fewer than two nodes, or with no terminal runs, yields an empty canopy without throwing (R2 error case)
- [ ] Non-finite placement parameters fall back through the same rail (R2 error case)
- [ ] Candidate iteration is array-backed; no `Map` or `Set` ordering in the placement path (R2 error case)
- [ ] Determinism test: two builds from one seed produce identical typed arrays (R7)
- [ ] Existing skeleton and surface tests still pass byte-identical, since no earlier stage calls the canopy (R7 error case)
- [ ] Telperion and Laurelin each state a canopy parameter block
- [ ] `npx vitest run` and `npx tsc --noEmit` green

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
