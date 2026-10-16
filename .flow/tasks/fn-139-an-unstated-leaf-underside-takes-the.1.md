---
satisfies: [R1, R2, R3, R4]
---
# fn-139-an-unstated-leaf-underside-takes-the.1 Implement fn-139-an-unstated-leaf-underside-takes-the

## Description
Default an unstated or dropped leaf underside colour to the sourced front colour.

## Acceptance
Spec R1-R4.

## Done summary
`appearance.rs` gathers chosen sentences order-independently; `default_to_front` records the back at the front's level when the back is unstated or was dropped without replacement and the front is sourced; everything else unchanged. Four tests in `judged_select.rs`, each red first. Gate green (1052 passed).

## Evidence
- Commits: 03e1cc9a16b962b0a867abf0822d12ba4e02a21c, 1fb7986200522845c35a63e31ed4aab5f5b77bb7
- Tests: cargo test -p telperion-jev --profile ci --test judged_select; cargo test --profile ci --workspace --no-fail-fast (1052 passed)
- PRs:
