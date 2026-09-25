# fn-151-the-design-principles-are-checked.1 The design principles are checked before a push

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The design principles are checked before a push. Three deterministic guards block in cargo test and in .githooks/pre-push: the production boundary, entry coverage of every shipped preset, and the artifact budgets. The advisory Jev reviewer (jev principles) runs in shadow for every principle; it is calibrated on 39 owner-labelled PRs and replayed offline (.flow/evidence/fn-151-the-design-principles-are-checked/EVALUATION.md).

R1, R2, R3, R6 and R8 are met. R4 holds for every guard-owned positive and every clean PR; of the Jev-owned positives, #3 and #13 are caught, nine are missed and two are unsupported (#52, #94). R5: Jev recall 2/3 (calibration) and 0/11 (holdout), 0 of 22 clean pushes flagged; the targets are not met, so every principle stays in shadow. R7: fn-150 yields no finding and a proposal without context abstains, but the labelled duplicate-path spec is not flagged (shown 0.84, covered 0.64 over the 0.50 cut).

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 9734cb0c29f7b529bd9d12ddcb7b55bf74071e35, 70c0dd230119916337d1e2c11c7b81205e37fdea
- Tests: cargo test --profile ci --workspace --no-fail-fast (969 passed, 0 failed, 21 ignored, 129 suites), npm test (128 passed, 11 files), npm run build && jev principles budget (all six artifacts at their 0.1.4 sizes), cargo test -p telperion-field every_shipped_preset on 39348def (red: date-palm, slim entry), jev principles report (offline replay of 39 PRs), baseline: none run (AGENTS.md: the gate runs once, at the end)
- PRs: