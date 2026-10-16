---
satisfies: [R1, R2, R5]
---
# fn-19-comparative-botanical-geometry-benchmark.1 Freeze reference coverage and the comparative benchmark protocol

## Description
Freeze the versioned reference inventory, specimen/view manifest, metric definitions and review rubric the tooling tasks consume, designed for more species than the initial oak/spruce cohort.

## Acceptance
- Every oak/spruce anatomical scale has attributed evidence or a specific missing-reference record; competitor availability recorded with no superiority inferred from absence.
- Versioned protocol and seed manifest fix the 12-specimen population, views, metrics and rubric before tuning.

## Done summary
Frozen fn19-v1: twelve mature specimens with full parameters, six new seeds audited against 2,637 seed-bearing files, hashed source identity, shared JSON Schema receipt/admission contracts, and fixed metric and camera definitions. All six attributed OSU reference images opened; both species admit bounded qualitative comparison; base/fork detail and bare spruce remain reference gaps. No production files changed.

## Evidence
- Commits: 554720e77abd495f249bf2c5c7f9ff828ad21582
- Tests: cargo test --release -p telperion-core --test species_metrics; npm run typecheck
