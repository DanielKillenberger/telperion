---
satisfies: [R3]
---
# fn-1-the-canopy-real-leaf-geometry-culled-to.4 Culling to a shell, and the silhouette test that proves it

## Description
Cull the canopy to a shell under the envelope and ship the screen-space silhouette helper that proves the outline is unchanged.

## Acceptance
- Count falls substantially and stays above zero on a fixture dense enough that a no-op fails.
- Classification by every vertex, never the centroid; silhouette unchanged within the raster's pixel floor from six views.

## Done summary
`cullCanopy` measures depth to the envelope profile per vertex; `silhouette.ts` rasterizes the outline in Node. The default shell removes ~15% of the preset canopies and 16% of an envelope-filling one; silhouette held across six views within ~1.8% at 256 px.

## Evidence
- Commits: 23909d3
- Tests: npx vitest run (251 passed), npx tsc --noEmit
- PRs:
