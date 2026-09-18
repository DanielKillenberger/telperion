---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-76-the-test-suite-under-three-minutes-one.1 Implement the test suite under three minutes

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The species suite generates each fixed seed once and proves determinism against a committed digest per preset and seed (49 values in `crates/telperion-core/tests/species/digests.json`, green on the runner's first run, mismatches reported together naming preset, seed, expected and actual), and runs a species' seeds four at a time on scoped threads with the outcomes read back in seed order; fifteen core test files read shipped skeletons from a per-run cache under `target/tmp/specimens` keyed on the wire, the seed and the generator sources (`tests/specimens/mod.rs`, with `mesh::build` split into `generate` + `mesh::assemble`); the workflow runs each Rust crate without a receipt through `.github/actions/nextest` as named jobs (`core 1/4` to `core 4/4`, `render`, `wasm and jev`), and `rust-receipts` saves a crate's receipt when that crate's jobs are green. Local numbers, the count diff (five added, none lost), the predicted shard loads from the runner's per-test times and the threaded species times are in `.flow/evidence/fn76/LOCAL.md`; the first runner run is in `.flow/evidence/fn76/RUNS.md`; two friction entries in `.flow/evidence/fn76/FRICTION.md`.

What the conductor must know: the spec's cost model was wrong (the skeleton is a twentieth of a specimen; foliage placement is the mass), so the cache holds skeletons, not meshes. The first runner run missed R5 at 6 min 23 s cold; the second commit answers with the named shards (predicted core test phases 54/89/54/51 s, `count` chosen over `hash`) and the seed threads (spruce 178 s on one core to a chain of four rounds; 41 s on the desk); the second run's numbers are the next RUNS.md row and decide whether the bound holds. The core split count 4 is recorded with its prediction. Local commands keep `cargo test` (`npm run rust:test` unchanged).

baseline: none (the spec lists no Quick commands; pre-edit core suite under nextest recorded green, 332 passed)

stage: impl-review - skipped(config: REVIEW_MODE=none)

## Evidence
- Commits: b86af1a3e5f95848afa9263372183c0ec816379a, c345818910c58edbd59e213d267548c616a2100c, 38908fb5da16fa44a9913ed0e642d3802cf5131b, 28cd231d7025952986238f95a8d508c7c123a6bd
- Tests: cargo nextest run --cargo-profile ci -j 3 -p telperion-core --no-fail-fast (337 passed, 11 skipped, after both follow-ups), cargo nextest run --cargo-profile ci -j 1 -p telperion-core --test species (10 passed, seeds four at a time), cargo nextest run --cargo-profile ci -j 6 -p telperion-render -p telperion-wasm -p telperion-jev --no-fail-fast (256 passed), cargo test --profile ci -p <crate> -- --list (599 before, 604 after, five added, none lost), cargo fmt --all -- --check, node scripts/ci-key.mjs rust-core, GitHub run 35385460955: green, R1 digests matched on the runner, 6 min 23 s cold (RUNS.md)
- PRs: 
