---
satisfies: [R1, R2, R3, R4]
---
# fn-18-faster-field-generation.2 Optimize measured growth work with exact ordered output

## Description
Implement the highest-value exact-output growth candidate identified by task 1.

**Size:** M
**Files:** crates/telperion-core/src/colonization.rs, crates/telperion-core/src/colonization/grid.rs, crates/telperion-core/src/branching/local.rs, crates/telperion-core/tests/growth.rs, crates/telperion-core/tests/species.rs.
**Touches:** [crates/telperion-core/src/colonization.rs, crates/telperion-core/src/colonization/grid.rs, crates/telperion-core/src/branching/local.rs, crates/telperion-core/src/radius.rs, crates/telperion-core/tests/growth.rs, crates/telperion-core/tests/species.rs, .flow/evidence/fn18/**]

## Approach
- Select one measured dominant kernel, not a wholesale grower rewrite. Candidates include redundant full-attractor passes or repeated local-shoot planning/station work; current attraction lookup already uses a grid.
- Preserve strict distance tie selection, original attractor-index f64 summation, round snapshots, parent/frontier append order, IDs, RNG streams and cap decisions. Removing work requires proof it cannot affect a currently eligible attractor or shoot.
- Use task 1's frozen reference replay to compare full GrowthReport/tree arrays, diagnostics, downstream foliage matrices, field bounds and queries. Add equal-distance, sparse/empty, boundary-envelope and tight-budget fixtures.
- Measure in the complete browser build against original and immediate predecessor binaries. Retain smaller useful gains only under parent guardrails; explicitly keep R1 unresolved if 3× misses. A rejected kernel keeps the reference implementation and recorded evidence.
- Hand immutable planning/ordered commit opportunities to task 5; no parallel execution or new seed scheme in this task.

## Investigation targets
**Required:**
- crates/telperion-core/src/colonization.rs:121 — settle and nearest ties.
- crates/telperion-core/src/colonization.rs:213 — ordered pull/close/append.
- crates/telperion-core/src/branching/local.rs:51 — shoot planner.
- crates/telperion-core/src/branching/local.rs:259 — frontier commits.
- crates/telperion-core/src/radius.rs:46 — ordered accumulation.
- crates/telperion-core/tests/growth.rs and tests/species.rs — existing pins.
- scripts/benchmarks/field-generation.md — dependency-created protocol.

## Acceptance
- [ ] Selected kernel is supported by the measured stage budget and compares exact source/report/field output against the frozen original.
- [ ] Equal-distance ties, accumulation order, append IDs, RNG, caps and authored boundaries remain pinned.
- [ ] Complete browser/native results and failures are recorded separately; adoption or no-change is justified without calling a substage gain 3×.
- [ ] Focused growth/species tests and downstream differential oracle pass for adopted changes.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:

