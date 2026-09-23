---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-132-a-palms-size-is-its-growth-rate-and.1 Implement fn-132-a-palms-size-is-its-growth-rate-and

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The palm's row now asks a height growth rate (new `rate` field kind, growth-rate question set with labelled cases on A1/P5 and a no-match gap, filled in m/yr) and mature height and trunk diameter; a no-value gap beside a partial or sufficient level becomes the level's own, so the palm's sufficient leaflets pass; the conductor reruns stages whose artifact records another build before searching or pausing for the owner, once per build. The two unknown code locations are written into the spec (quality.rs `run`; plan.rs `next`).

Tests: R1, R2 and R3 in `crates/telperion-jev/tests/palm_size.rs` (fixture `tests/fixtures/palm/fn132-screen-rows.json`, live screen rows); R4 in `tests/stale_first.rs`; the rate set scored in `tests/sets.rs`. Gate `cargo test --profile ci --workspace --no-fail-fast`: 139 suites, 1023 passed, 0 failed, 21 ignored.

Host to confirm: the palm's `dbh_m` bar is `proxy_only`. The mature-size set labels A1's "up to ½ m" `proxy_only`/`bound_only`, and at `partial` R2's fill could not happen. The quantity grammar reads no "½", so the trunk fills from the same sentence's "18 inches" (0.4572 m). Follow-up: vulgar fractions in `quantity::unit_re`. The palm manifest in fn-80 now needs `height_growth_m_per_year` (Boundaries: the host rewrites it).

stage: impl-review - skipped(config: REVIEW_MODE=none)
Tier: session (owner and host settled design)
## Evidence
- Commits: 068ba16217722c51e3b389d896e0d9238c48d445
- Tests: baseline: none (spec lists no Quick commands; the owner's gate runs once at the end), cargo test --profile ci -p telperion-jev --test palm_size --test stale_first (red first: R1 passed=false on no_mature_size, R2 row lacked the rate, R4 next was SearchAgain), cargo test --profile ci --workspace --no-fail-fast (139 suites, 1023 passed, 0 failed, 21 ignored; log .flow/evidence/fn-132-a-palms-size-is-its-growth-rate-and/raw/gate.log), cargo fmt -p telperion-jev -- --check, cargo clippy --profile ci -p telperion-jev --all-targets (no new warnings)
- PRs: