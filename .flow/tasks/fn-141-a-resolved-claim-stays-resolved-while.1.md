---
satisfies: [R1, R2]
---
# fn-141-a-resolved-claim-stays-resolved-while.1 Implement fn-141-a-resolved-claim-stays-resolved-while

## Description
Key verify's claim decisions on the claim instead of the whole `select.json`.

## Acceptance
Spec R1-R2.

## Done summary
Claim decisions keyed on pointer, value, source id and sentence; a resolution stays bound while its claim is unchanged. Red test first; gate 1053 passed.

## Evidence
- Commits: db9364fb
- Tests: cargo test --profile ci --workspace --no-fail-fast (1053 passed)
- PRs:
