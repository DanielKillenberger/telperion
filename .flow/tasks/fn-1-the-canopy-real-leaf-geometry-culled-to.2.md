---
satisfies: [R1, R4]
---
# fn-1-the-canopy-real-leaf-geometry-culled-to.2 The leaf element: real parameterized geometry, fitted to its silhouette

## Description
The leaf element as real parameterized geometry in `src/canopy/`, fitted to its silhouette, authored in the local frame (petiole at origin, +Y axis, +Z normal).

## Acceptance
- Named shape parameters with defaults; non-finite/out-of-range values fall back through the `held` rail.
- Silhouette-fitted geometry, card mode reachable and off by default.

## Done summary
`buildElement` builds the blade as a height field (no degenerate triangles, all faces +Z); the outline covers 0.58 of its bounding quad. Mutation checks confirmed the fit, frame and winding tests bite.

## Evidence
- Commits: d047d4c, b1325e5
- Tests: npx vitest run (225 passed), npx tsc --noEmit
- PRs:
