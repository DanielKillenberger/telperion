---
satisfies: [R6]
---
# fn-6-branch-generations-below-the-crossover.7 The cost on the named machine, and the docs that still say orders

## Description
Measure the cost on the RTX 3080 at native and high DPR, and update every doc that still says orders.

## Acceptance
- Both presets measured with GPU timer queries against the 2 ms hero budget and the build timer; no numeric fallback without the extension; docs updated.

## Done summary
Native DPR 1 GPU medians: Telperion 5.847 ms, Laurelin 3.425 ms (miss 2 ms); CPU builds 4.923 s and 2.421 s. Sweep shares one native-aware ratio helper; docs describe radius-derived generations and the fixed twig; fn-5 R8 annotated superseded.

## Evidence
- Commits: 4027b078a5ca840ca62b69e0a2641ce972de1c81
- Tests: npx vitest run (363 passed), npx tsc --noEmit, RTX 3080 browser measurement
- PRs:
