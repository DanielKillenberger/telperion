---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-92-two-devices-asked-for-at-once-do-not.1 Implement Two devices asked for at once do not crash the process

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
`Gpu::request` now makes its `wgpu::Instance` under a process-wide lock on native targets, held around `Instance::new` alone and released before any await; `bark_plates` went from 5 crashes in 12 runs on the base to 0 in 36 as shipped. R1 to R5 hold; counts, the core's two frames and what stays unknown are in `.flow/evidence/fn-92-two-devices-asked-for-at-once-do-not/REPRO.md`, wall times in `TIMING.md`, three friction entries in `FRICTION.md`.

One batch of five render-crate runs failed and was discarded as contaminated by a target directory shared with a base checkout; REPRO.md says so and says the cause is inferred. `concurrent_request.rs` is a smoke test that has never been red. `.flow/evidence/fn71/resolution/smooth_bark.json` was rewritten by the runs and kept out of every commit.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 2587b61b1f0d0bc5615a772e5d3bcd1a39981b77, 430b80e3fc2392a44f7edcd975f9bc79eb988f9e, 3b9eba06c5ecca17e6947ad96dfae76fc98db55b, b84c9aedc4ddd5548a3cdd71f48324b9e29aae5e
- Tests: baseline: none (spec defines no Quick commands), target/ci/deps/bark_plates x24 at 3b9eba06: 0 crashes (base: 5 of 12), cargo test --profile ci -p telperion-render x5 at 3b9eba06: 5 of 5 green, cargo check -p telperion-wasm --target wasm32-unknown-unknown, cargo clippy --profile ci -p telperion-render --all-targets: no warnings, cargo test --profile ci --workspace --no-fail-fast at 3b9eba06: exit 0, 375 s, 101 suites ok
- PRs: