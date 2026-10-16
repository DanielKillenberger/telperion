---
satisfies: [R4, R6, R8]
---
# fn-1-the-canopy-real-leaf-geometry-culled-to.5 The instanced draw, the clay judgment, and framing that includes the canopy

## Description
Draw the canopy as one instanced mesh in clay, alpha-tested, and make framing include the canopy.

## Acceptance
- One instanced draw per element type; alpha test, no blending; clay defaults unchanged; `src/` emits no material, colour or light.
- Canopy contributes to subject bounds; zero elements leaves framing identical; uncomputed bounds never contribute zero extent.

## Done summary
Each tree is a group of trunk plus one `InstancedMesh` crown (2 draws per tree). `measureSubject` recomputes instanced bounds before framing; `disposeSubject` frees instance transforms. Ten canopy dials on the panel. Cull re-measured: Telperion 14.8/15.0/15.2%, Laurelin 6.5/6.2/5.9% at 1x/2x/4x density.

## Evidence
- Commits: 922749e, 9e8d7ff
- Tests: npx vitest run (266 passed), npx tsc --noEmit, npm run build
- PRs:
