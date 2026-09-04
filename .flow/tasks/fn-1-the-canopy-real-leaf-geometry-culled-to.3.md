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
Derives shoots as the young-wood end of every terminal `branchPaths` run and
places elements along them - phyllotaxis, a clump at the tip, outward and
upward orientation bias - emitting one column-major 4x4 per element against
the element's stated local frame, from an XOR-derived sub-stream that leaves
every existing stream untouched. Telperion and Laurelin each state a canopy
block in full.

stage: impl-review - skipped(policy: parallel wave - conductor reviews after integration)

### What landed

- `src/canopy/shoots.ts` - `shoots(skeleton, field, maxRadius)`. A shoot is the
  distal stretch of a run where the wood is thin enough to bear foliage. Every
  run `branchPaths` emits already ends at a childless tip, so the terminal-run
  rule is satisfied by taking its output - which is also what inherits its
  `stands[]` zero-length fold instead of re-deriving it.
- `src/canopy/place.ts` - `buildCanopy(skeleton, field, envelope, seed, params)`
  returning `{ matrices: Float32Array, count }`, 16 floats per element in
  `THREE.Matrix4.elements` order. Ten named parameters, each with a default in
  `DEFAULT_CANOPY`, each pinned on the same local `held` rail `surface.ts` and
  `radius.ts` use.
- `src/canopy/place.test.ts` - 21 tests.
- `src/presets/preset.ts` - `TreePreset` gained a required `canopy`.
- `src/presets/two-trees.ts` - both canopy blocks, every term stated.

### Acceptance

| AC | Where |
|---|---|
| Shoots as terminal runs through `branchPaths`, reusing `stands[]` | `shoots.ts`; tests "takes the young-wood end of a terminal run" and "inherits the zero-length fold rather than re-deriving it" |
| Phyllotaxis, clumping, orientation bias, each named with a default | `CanopyParams` / `DEFAULT_CANOPY`; tests "turns each element by the divergence angle", "gathers a clump into the last stretch", "turns elements outward/upward when the bias asks" |
| < 2 nodes, or no terminal runs, yields an empty canopy without throwing | test "yields an empty canopy for %s, without throwing" (3 cases) plus "when no wood is thin enough" |
| Non-finite placement parameters fall back through the same rail | test "falls back to the defaults for every non-finite parameter" - every dial NaN/Infinity, plus a NaN envelope height, byte-equal to the stated build |
| Candidate iteration array-backed, no `Map`/`Set` | test "iterates arrays, never a Map or a Set" (source guard; verified red by inserting a `new Set` before it went green) |
| Determinism: two builds from one seed, identical typed arrays | tests "builds the same canopy twice from one seed" and "gives two seeds two canopies" |
| Existing skeleton and surface tests byte-identical | all 193 pre-existing tests untouched and green; plus test "leaves the stages before it byte-identical" (skeleton positions, radius field and `buildSurface` output unchanged across a canopy build) |
| Both presets state a canopy block | tests "%s states its canopy in full" and "%s carries foliage, and none of it below its crown" |
| `npx vitest run` and `npx tsc --noEmit` green | 16 files / 214 tests; tsc clean |

### Judgement calls the conductor should see

- **Three parameters beyond the four the AC names.** `shootRadius` bounds a
  shoot to young wood: without it the trunk run is a shoot along its whole
  length and a 7 m-radius column grows leaves, which shell culling in .4 would
  not remove because they are on the silhouette. `size` and `sizeVariation`
  carry the element's scale, because placement is the stage that knows how big
  the tree is and the spec's own words are "per-instance transform plus
  variation". `scatter` is orientation disorder - a canopy with none reads as a
  diagram of a canopy.
- **The element is assumed to be authored at unit scale**, sized by the
  transform. If task .2 sizes its geometry to the tree, one of the two has to
  give at integration.
- **Laurelin diverges at the Lucas angle (99.502) where Telperion takes the
  golden one.** A real botanical alternative, one number in the same mechanism,
  and it is the massed reading against Telperion's open one.
- **`TreePreset.canopy` is required, not optional.** Nothing outside
  `two-trees.ts` constructs a preset literal today, so nothing else broke - but
  .5 and .6 should know.
- `src/index.ts` is deliberately untouched; the barrel export is task .6's.

Measured: Telperion 133 shoots / 3407 elements, Laurelin 376 / 8679, both
entirely above their own crown base. `spacing` is the dial if .5's clay
judgement wants more.

Sibling note written to the run-notes surface as `t3-placement-surface.md`.
## Evidence
- Commits: 412c47d8b6182bd26d8c53deefbb61504c41095e, 368ad5a
- Tests: npx vitest run (16 files / 214 tests passed on the integrated target; baseline 15 / 193), npx tsc --noEmit (clean on the integrated target)
- PRs: