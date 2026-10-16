---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-87-the-specimen-says-what-it-will-cost.1 Implement The specimen says what it will cost

## Description
Predict each specimen's resident bytes from its tree and admit seeds against one binary-wide budget.

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
`footprint::predict` and `footprint::measure` agree exactly on every seed of the four species; counting is shared with the builders; `Budget::of_host` (charge limit two thirds of a ceiling from `MemAvailable`) replaces `SEEDS_IN_FLIGHT`. Desk: 26.46 s at 12,539 MB under a 15,304 MB ceiling, narrowing to 35.47 s as free memory fell. All 49 digests hold.

## Evidence
- Commits: 33429197db67efdee2059f2b724b335c0ab70380, 75153b12f0b31b5124ccf9cbd3330c7a095d9e6d
- Tests: cargo test --release --workspace (685 passed), cargo test --profile ci --workspace, cargo clippy --profile ci --workspace --all-targets
- PRs:
