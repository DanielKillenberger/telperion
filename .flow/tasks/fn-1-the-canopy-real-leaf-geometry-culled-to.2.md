---
satisfies: [R1, R4]
---
# fn-1-the-canopy-real-leaf-geometry-culled-to.2 The leaf element: real parameterized geometry, fitted to its silhouette

## Description
Builds the foliage element itself (R1) and the silhouette fit that R4 requires of the geometry. Split from placement because the spec states the local-frame contract between them, so neither waits on the other.

**Size:** M
**Files:** `src/canopy/element.ts`, `src/canopy/element.test.ts`
**Touches:** [src/canopy/element.ts, src/canopy/element.test.ts]

### Approach

- New stage directory `src/canopy/`. Follow the params convention exactly: an `ElementParams` interface with a `DEFAULT_ELEMENT` const beside it and inline JSDoc on each field giving units, range and why, as `SurfaceParams` / `DEFAULT_SURFACE` do at `src/mesh/surface.ts:56-147`.
- Clamp at point of use with the local `held(value, fallback)` plus `Math.min/max` rail block. Copy the idiom from `src/mesh/surface.ts:215`; it is deliberately duplicated per call site (`src/radius.ts:172` is the other), so do not extract a shared helper.
- Return the library's canonical geometry-out shape, the same field set as `SurfaceMesh` at `src/mesh/surface.ts:195-203`, so the harness wraps it exactly as it wraps the trunk at `harness/skeleton-view.ts:209-218`.
- Author in the local frame the spec fixes: petiole at the origin, axis along +Y, face normal along +Z.
- The silhouette fit is the point of R4. The outline follows the leaf; a bounding quad is what this avoids. Card mode is a named parameter and is off by default.

### Investigation targets

**Required** (read before coding):
- `src/mesh/surface.ts:56-147` - params interface plus `DEFAULT_` const, the JSDoc register to match
- `src/mesh/surface.ts:195-215` - the mesh-out shape and the `held` idiom
- `src/radius.ts:168-191` - the same rail block, second instance, with the comment on why NaN is named separately

**Optional** (reference as needed):
- `src/mesh/surface.test.ts` - assertion style for generated geometry
- `harness/skeleton-view.ts:209-218` - how a mesh becomes a `THREE.BufferGeometry`

### Key context

- Project memory, interpenetrating junctions: assert the geometric claim on every vertex, never on its centre, and measure against the drawn geometry rather than an analytic approximation. An element test that checks a bounding box is not checking a silhouette.
- Project memory, zero width is not no constraint: a degenerate parameter set must not produce a vacuously valid mesh. The prevention note on that entry names foliage as one of the surfaces still at risk.

### Acceptance

- [ ] Element geometry is generated procedurally from named parameters with a `DEFAULT_` const and per-field JSDoc (R1)
- [ ] The outline fits the leaf silhouette rather than a bounding quad (R4)
- [ ] A bounding-quad card is reachable by parameter and off by default (R1)
- [ ] Geometry is authored in the spec's local frame: petiole at origin, axis +Y, face normal +Z
- [ ] Non-finite and out-of-range parameters fall back to documented defaults through `held`, with a test (R1 error case)
- [ ] Assertions are vertex-level, not centroid or bounding box
- [ ] `npx vitest run` and `npx tsc --noEmit` green

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
