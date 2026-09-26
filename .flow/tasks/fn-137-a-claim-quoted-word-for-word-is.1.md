---
satisfies: [R1, R2, R3]
---
# fn-137-a-claim-quoted-word-for-word-is.1 Implement fn-137-a-claim-quoted-word-for-word-is

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
A verbatim numeric span now exempts a `supports` citation claim from the confidence-below-cut listing rule in `crates/telperion-jev/src/cite.rs`'s `list_reason`; the other listing rules (site-quality criterion, height-at-age kind mismatch) still apply, and the row's reason reads `verbatim`. Red-then-green integration test reproduces the fn-80 A1 scenario (palm height "50 - 100 feet", supports at 0.65, cut 0.8): listed before the fix (reason `confidence 0.65 below 0.8`), not listed after (reason `verbatim`). Two further tests cover R2 (a non-verbatim span stays listed below the cut; a verbatim span on a site-quality criterion is still listed) and the whitespace-only normalisation contract of the new `is_verbatim()` helper (built on `extract::candidate_spans` and the now-`pub` `extract::collapse_ws`).

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: f8fdcb24f982a13826639e9a6a4ce99542612aee
- Tests: cargo test -p telperion-jev --test cite_triage (focused, red-then-green), cargo test --profile ci --workspace --no-fail-fast (144 test result lines, 1049 passed, 0 failed)
- PRs: