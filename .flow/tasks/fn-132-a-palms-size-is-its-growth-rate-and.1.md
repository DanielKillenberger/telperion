---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-132-a-palms-size-is-its-growth-rate-and.1 Implement fn-132-a-palms-size-is-its-growth-rate-and

## Description
Give palms a growth-rate plus mature-range requirement and fix the rerun's two defects.

## Acceptance
Spec R1-R5.

## Done summary
New `rate` field kind and growth-rate question set; palm row asks rate plus mature height and DBH; a no-value gap beside a partial/sufficient level takes the level's own; the conductor reruns build-stale stages before searching or pausing. Tests `tests/palm_size.rs` (live screen rows fixture), `tests/stale_first.rs`, `tests/sets.rs`, each red first. Gate green (1023 passed).

## Evidence
- Commits: 068ba16217722c51e3b389d896e0d9238c48d445
- Tests: cargo test --profile ci -p telperion-jev --test palm_size --test stale_first; cargo test --profile ci --workspace --no-fail-fast (1023 passed)
- PRs:
