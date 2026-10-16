---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-19-comparative-botanical-geometry-benchmark.4 Publish the baseline assessment and verified revision replay

## Description
Run the frozen population, publish the comparative assessment and independent-review packet, and document reproducible reruns.

## Acceptance
- Every frozen case and view has a terminal record; inspection, collection success and missing expert feedback stay distinct.
- The report ranks actionable discrepancies for fn-20/fn-21 and qualifies every comparator claim.

## Done summary
All 12 mature cases and 84 views completed; an independent small replay reproduced 42 views and six native cases with zero metric, image, geometry or gap drift. A capture-validator key-order defect was fixed (semantic object comparison). Mature tool identity bda8f89c... and corrected replay identity ee62dad3... stay separate; frozen generator revision b5249e16. Costs inconclusive; independent assessment unassessed. Report: `.flow/evidence/fn19/REPORT.md`.

## Evidence
- Commits: e2fe534c42195ddf2876f670ef2a4d627637ccee
- Tests: cargo test --release -p telperion-core --test geometry_benchmark --test species_metrics; node --test tests/browser/geometry-benchmark.test.mjs tests/browser/geometry-compare.test.mjs; npm run typecheck
