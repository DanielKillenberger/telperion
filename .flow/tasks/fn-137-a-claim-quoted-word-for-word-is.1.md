---
satisfies: [R1, R2, R3]
---
# fn-137-a-claim-quoted-word-for-word-is.1 Implement fn-137-a-claim-quoted-word-for-word-is

## Description
Exempt a verbatim-quoted `supports` claim from the confidence cut.

## Acceptance
Spec R1-R3.

## Done summary
`cite.rs` `list_reason` skips the confidence rule for a verbatim span (`is_verbatim()` on `extract::candidate_spans` and `collapse_ws`); other rules unchanged; reason reads `verbatim`. Red-then-green test on the fn-80 A1 scenario plus R2 tests. Gate green (1049 passed).

## Evidence
- Commits: f8fdcb24f982a13826639e9a6a4ce99542612aee
- Tests: cargo test -p telperion-jev --test cite_triage; cargo test --profile ci --workspace --no-fail-fast (1049 passed)
- PRs:
