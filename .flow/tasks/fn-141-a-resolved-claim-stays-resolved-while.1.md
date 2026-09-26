---
satisfies: [R1, R2]
---
# fn-141-a-resolved-claim-stays-resolved-while.1 Implement fn-141-a-resolved-claim-stays-resolved-while

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Verify keys each claim decision on the claim (pointer, value, source id, sentence) instead of the whole select.json checksum, so a resolution stays bound while its claim is unchanged. Red test first; gate 1053 passed, 0 failed.
## Evidence
- Commits: db9364fb
- Tests: cargo test --profile ci --workspace --no-fail-fast (1053 passed, 0 failed)
- PRs: