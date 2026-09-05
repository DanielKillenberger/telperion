---
satisfies: [R1, R2]
---

# fn-8-lean-rust-tree-generation-core.2 Port crown colonization and structural generation state

## Description
Port the crown-generation stage onto the foundation without changing the growth model.

**Size:** M
**Files:** Rust colonization/spatial query modules and stage tests
**Touches:** [crates/telperion-core/src/colonization*, crates/telperion-core/src/spatial*, crates/telperion-core/tests/colonization*, crates/telperion-core/tests/crown_reference*]

## Approach
- Port crown fill/attractor placement, nearest-branch queries, step/kill behaviour and shared-field bending with the same deterministic iteration order.
- Emit the canonical parent-before-child arrays and diagnostic metadata established in task 1.
- Preserve inside-parent crown exit guards and outside-start approach behaviour; test candidate edges against vertical and radial constraints.
- Extend the task-1 equivalence runner to compare the colonization stage independently at ordinary and giant scales, including capped/empty cases.

## Investigation targets
**Required:**
- `src/skeleton/colonize.ts:103`
- `src/skeleton/colonize.test.ts`
- `src/envelope.ts`
- `src/skeleton/fill.ts`
- `src/skeleton/grow.ts:279`
**Optional:**
- `.flow/memory/bug/runtime-errors/zero-width-is-not-no-constraint-the-2026-09-04.md`

## Approved capture alignment
The rewritten parent capture is authoritative. Baselines diagnose drift; exact old topology or bytes are not a compatibility requirement, and known structural defects need not be reproduced. Preserve meaningful botanical and geometric invariants and report visual/numeric differences. Keep the core lean and simple.


## Acceptance
- [ ] Stage equivalence passes for ordinary and giant preset crown fixtures.
- [ ] Deterministic ordering, cap reports and valid empty output are preserved.
- [ ] Crown crossings and outside-start approach regressions pass without non-finite structure.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
