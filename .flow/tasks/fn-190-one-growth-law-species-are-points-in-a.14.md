---
satisfies: [R6]
---
# fn-190-one-growth-law-species-are-points-in-a.14 Production design (host, Astra review)

## Description
Stage G design, recorded in the spec before production code. Host-owned.

**Size:** M
**Files:** the spec, evidence `DESIGN.md`
**Touches:** [.flow/specs/fn-190-one-growth-law-species-are-points-in-a.md, .flow/evidence/fn-190-one-growth-law-species-are-points-in-a/DESIGN.md]

### Approach
- Where the rule lives (`crates/telperion-core/src/pipeline/executor.rs:34`, stage 2), the vocabulary as catalogue rows, the deletion list (twig layer `pipeline/branching/local*`, `twigs.rs`, girth rows `pipeline/radius*`, cull `pipeline/foliage.rs:255` and `place.wgsl`, clumping `foliage/clumping.rs`), a migration map for retired rows with an error for unknown rows, the fn-125 contract, every preset as coordinates.
- Astra reviews against `docs/principles.md`.
- Split the port into tasks.

### Investigation targets
**Required:**
- `docs/pipeline.md`
- `crates/telperion-core/src/pipeline/executor.rs`
- `crates/telperion-core/tests/catalogue_identity.rs`

## Acceptance
- [ ] Design recorded in the spec with Astra's review answered
- [ ] Port split into tasks

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
