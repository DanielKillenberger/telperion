---
satisfies: [R7]
---
# fn-190-one-growth-law-species-are-points-in-a.15 Port the rule to production (rewrite with tests; host splits)

## Description
Stage G port placeholder; the design task splits it.

**Size:** M
**Files:** `crates/telperion-core/src/pipeline/**`, `crates/telperion-render/src/generation/place.wgsl`, `crates/telperion-core/tests/**`, `docs/**`
**Touches:** [crates/telperion-core/**, crates/telperion-render/src/generation/**, docs/**, README.md, STRATEGY.md]

### Approach
- Rewrite, never copy, the probe (AGENTS.md "Code rules"); the deletions made; a test walks every setting; off-rail values refused by name.
- Docs: `docs/pipeline.md`, `docs/parameters.md` (generated), `README.md:90`, `docs/generation-limits*`.

### Investigation targets
**Required:**
- `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/DESIGN.md`

## Acceptance
- [ ] The rule replaces stage 2 for every preset; deletions made
- [ ] Setting-walk test and refusal tests pass
- [ ] Docs updated

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
