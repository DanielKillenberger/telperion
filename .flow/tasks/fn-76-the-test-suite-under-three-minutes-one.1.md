---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-76-the-test-suite-under-three-minutes-one.1 Implement the test suite under three minutes

## Description
Digest determinism, a per-run specimen cache, nextest and named shards.

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
49 committed species digests (mismatches reported together); species seeds four at a time on scoped threads with outcomes read back in seed order; fifteen core test files read skeletons from `target/tmp/specimens` (`mesh::build` split into `generate` + `mesh::assemble`); `.github/actions/nextest` runs `core 1/4`..`4/4`, `render`, `wasm and jev`; `rust-receipts` saves per crate. Counts 599 -> 604, none lost. Runner: 2 min 55 s warm.

## Evidence
- Commits: b86af1a3e5f95848afa9263372183c0ec816379a, c345818910c58edbd59e213d267548c616a2100c, 38908fb5da16fa44a9913ed0e642d3802cf5131b, 28cd231d7025952986238f95a8d508c7c123a6bd
- Tests: cargo nextest run --cargo-profile ci -p telperion-core --no-fail-fast, cargo nextest run --cargo-profile ci -p telperion-render -p telperion-wasm -p telperion-jev, cargo test --profile ci -p <crate> -- --list
- PRs: #37
