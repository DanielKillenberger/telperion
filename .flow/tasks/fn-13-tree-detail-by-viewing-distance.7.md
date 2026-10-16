---
satisfies: [R2, R3, R4, R6]
---
# fn-13-tree-detail-by-viewing-distance.7 Correct GPU measurement and minimal batched submission

## Description
Correct GPU measurement and add minimal batched submission with functional shared depth.

## Acceptance
- Measurement, batching and shared depth pass hardware checks in all depth paths.

## Done summary
Measurement, batching and shared depth complete; the live selector preserves mode, seed, geometry identity and camera across all three depth paths, and the reversed path passes hardware stage/depth/material checks.

## Evidence
- Commits: d8491716ce84ce7c80949a387578b16f6adf95ff
- Tests: npm run typecheck; vitest run; browser integration
