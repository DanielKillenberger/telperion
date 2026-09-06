---
satisfies: [R1, R2, R3]
---
# fn-18-faster-field-generation.4 Qualify the remaining bounds and spatial-index improvement

## Description
Optimize the remaining measured bounds/index cost after growth and foliage changes.

**Size:** M
**Files:** crates/telperion-core/src/field.rs, crates/telperion-core/tests/field.rs, scripts/benchmarks/field-generation.mjs.
**Touches:** [crates/telperion-core/src/field.rs, crates/telperion-core/tests/field.rs, scripts/benchmarks/field-generation.mjs, .flow/evidence/fn18/**]

## Approach
- Compare at most two focused candidates selected from task 1 and updated measurements: redundant scans, partition metadata or a cheaper exact hierarchy. No new library is mandatory.
- Preserve every exact primitive bound and closed predicate. Do not replace eight-corner transforms with min/max-only transforms or differently rounded algebra.
- If topology changes, retain schema-1 layout, exact exported live CPU topology and deterministic same-implementation repeat arrays. Validate equal-centroid/axis ties, empty/degenerate partitions and extreme finite coordinates.
- Compare against both the original and post-task-3 build, with the same query corpus and allocation domains. Account for extra cached metadata and query degradation.
- Retain only useful exact-output gains within guardrails. Record no-change/rejected candidates explicitly; continue task 5's bounded assessment without silently lowering R1.

## Investigation targets
**Required:**
- crates/telperion-core/src/field.rs:282 — median builder and allocation.
- crates/telperion-core/src/field.rs:215 — inclusive overlap.
- crates/telperion-core/tests/field.rs — boundary queries.
- scripts/benchmarks/generation.md:65 — snapshot schema.
- scripts/benchmarks/field-generation.mjs — dependency-created oracle/runner.

## Acceptance
- [ ] Separated stage costs justify the bounded candidate set and selected/no-change decision.
- [ ] Source multiplicity, exact bounds/query flags, schema traversal and deterministic repeats pass all tie/degenerate cases.
- [ ] Original and predecessor comparisons account for retained/temporary memory, build and query tradeoffs.
- [ ] Report identifies achieved and remaining R1 savings; no topology-only or substage-only claim substitutes for complete-build gain.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:

