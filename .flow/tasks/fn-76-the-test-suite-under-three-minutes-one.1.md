---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-76-the-test-suite-under-three-minutes-one.1 Implement the test suite under three minutes

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The species suite generates each fixed seed once and proves determinism against a committed digest per preset and seed (49 values in `crates/telperion-core/tests/species/digests.json`, mismatches reported together naming preset, seed, expected and actual); fifteen core test files read shipped skeletons from a per-run cache under `target/tmp/specimens` keyed on the wire, the seed and the generator sources (`tests/specimens/mod.rs`, with `mesh::build` split into `generate` + `mesh::assemble` so mesh consumers assemble on the cached skeleton); the workflow runs the Rust suites as one cargo-nextest pool in four `count` partitions on standard runners, with the crate receipts saved by a `rust-receipts` job only when every partition is green. Local numbers, the count diff (five added, none lost) and the partition loads are in `.flow/evidence/fn76/LOCAL.md`; two friction entries in `.flow/evidence/fn76/FRICTION.md`.

What the conductor must know: the spec's cost model was wrong (the skeleton is a twentieth of a specimen; foliage placement is the mass), so the cache holds skeletons, not meshes, and the measured gain is smaller than the spec's halving. The digests were generated on the desk; the runner's first run is R1's cross-machine check and may move them (glibc libm on both, 17 std transcendental calls in the generator). R5 on the runner is bounded by the spruce fixed-seed test (148 s on the loaded desk, 13 seeds on one core); if the three-minute bound is missed, the next software lever is running a species' seeds on threads inside `fixed_species`, which keeps every seed and assertion but is not named by the spec, so it is a follow-up, not built. The workflow's shard count (4) is a starting value to be re-chosen from the runner's per-partition times (R3 records N); the `rm` of `target/tmp/specimens` before rust-cache saves keeps the per-run cache out of the build cache. Local commands keep `cargo test` (`npm run rust:test` unchanged).

baseline: none (the spec lists no Quick commands; pre-edit core suite under nextest recorded green, 332 passed)

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: b86af1a3e5f95848afa9263372183c0ec816379a
- Tests: cargo nextest run --cargo-profile ci -j 6 -p telperion-core --no-fail-fast (337 passed, 11 skipped), cargo nextest run --cargo-profile ci -j 6 -p telperion-render -p telperion-wasm -p telperion-jev --no-fail-fast (256 passed), cargo test --profile ci -p <crate> -- --list (599 before, 604 after, five added, none lost), cargo fmt --all -- --check, node scripts/ci-key.mjs rust-core
- PRs: