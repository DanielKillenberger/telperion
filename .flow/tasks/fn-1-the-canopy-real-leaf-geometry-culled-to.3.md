---
satisfies: [R2, R7]
---
# fn-1-the-canopy-real-leaf-geometry-culled-to.3 Shoots and placement: phyllotaxis, clumping, orientation bias, and determinism

## Description
Derive shoots from terminal runs and place elements on them with phyllotaxis, a tip clump and orientation bias, deterministically from its own sub-stream.

## Acceptance
- Shoots taken from `branchPaths` terminal runs; each placement term named with a default.
- Tiny or run-less skeletons yield an empty canopy; non-finite params fall back; array-backed iteration only.
- Two builds from one seed are byte-identical; existing streams untouched.

## Done summary
`shoots()` and `buildCanopy()` emit one column-major 4x4 per element from an XOR-derived sub-stream. Both presets state a full canopy block. 21 tests cover placement, fallbacks and determinism.

## Evidence
- Commits: 412c47d8b6182bd26d8c53deefbb61504c41095e, 368ad5a
- Tests: npx vitest run (214 passed), npx tsc --noEmit
- PRs:
