---
satisfies: [R1, R2, R3]
---
# fn-186-the-no-geometry-job-tests-the-build.1 Implement The no-geometry job tests the build without geometry

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The core's self dev-dependency sets `default-features = false`, so `--no-default-features` now drops `geometry` from the core's test build (package feature tree: only `json`). Tests and test-only imports that need geometry are gated on it: the `pipeline::tests` module (every test asks for wood or leaves, which the build without geometry refuses by name), `radii_tests`, the folds test module, `surface::extent`, `contract::surface::*`, `foliage::leaflets`, and single tests or statements in params, packed foliage, radius holds and the growth year.

- R1 (measured): no-default lib list 461 (256 `suite::`) -> 135 (0 `suite::`); `cargo test --profile ci -p telperion-core --no-default-features --lib` passes, 132 passed, 3 ignored, 211 s locally at two threads.
- R2 (measured): default lib list identical to the base, 461 tests; workspace gate `cargo test --profile ci --workspace --no-fail-fast` (telperion-heavy 12, two threads) green, 1406 passed, 0 failed, 103 ignored.
- R3: OPEN_ITEM, proven only by a PR's CI run. `scripts/ci-key.mjs` fails at this base (`FIELD is not defined`), so the receipts job every Rust job needs cannot run until that is repaired; the 3-minute target is unproven (211 s locally, led by a 60 s+ repeat test).

stage: impl-review - ran (codex correctness draw, SHIP, rid 5b66100533f54635bd7d3d18d7f4fc1c) (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 18657d5fad7b25e1489acc0b5dcd1f1ae553bc32, 729f60eb1b5e1aa542de9a778b5520694d21f2ae
- Tests: cargo test --profile ci -p telperion-core --no-default-features --lib, cargo test --profile ci --workspace --no-fail-fast
- PRs: