---
satisfies: [R2, R3, R4, R7]
---
# fn-294-the-release-reviews-findings-are-fixed.2 Tree memory counts flowers; CI keys see the space crate; field test builds the renderer

## Description
Three small confirmed fixes, grouped so the gate runs once.

**Size:** M
**Files:** `crates/telperion-render/src/generation.rs`, `crates/telperion-render/src/generation/resident/room_tests.rs` (or a sibling), `scripts/ci-key.mjs`, `scripts/ci-key.test.mjs`, `scripts/test-wasm.mjs`, `package.json`
**Touches:** [crates/telperion-render/src/generation.rs, crates/telperion-render/src/generation/**, scripts/ci-key.mjs, scripts/ci-key.test.mjs, scripts/test-wasm.mjs, package.json]

### Approach
- R2: add `renderer.organ.allocated_bytes()` to `tree_buffer_bytes` (`generation.rs:140-142`). Test (GPU, skips without `test_gpu()` like its neighbours): resident Laurelin, `tree_buffer_bytes` equals the ledger's tree buffers (`drawn_memory_tests.rs` uses `ledger.census_now()`); and an admission case after `room_tests.rs:55` with a budget that admits only when the organ bytes count as releasable. Red before the fix.
- R3: add `SPACE` to `rust-core`, `rust-render`, `rust-jev`, `node` in `scripts/ci-key.mjs`; fix the stale comment on `SPACE`. Make the suites importable (export `SUITES` and the key function; run the CLI only when invoked directly) and add `scripts/ci-key.test.mjs` that reads `cargo metadata --no-deps --format-version 1`, computes each workspace crate's transitive path dependencies, and asserts every suite listing a crate also lists each crate it depends on. Wire it into `npm test` the way `scripts/artifact-budgets.test.mjs` is.
- R4: run `scripts/build-render.mjs` after `scripts/build-wasm.mjs` in `scripts/test-wasm.mjs` with the same `execFileSync` pattern; update its header comment. Prove by reading `tests/browser/field.mjs` → `src/field/index.ts` imports and running `node scripts/build-render.mjs` alone. Never run the browser test.

### Investigation targets
**Required**:
- `crates/telperion-render/src/generation.rs:140-142`, `generation/submit.rs:74-82`, `generation/resident.rs:40`
- `crates/telperion-render/src/generation/resident/room_tests.rs:39-80`, `crates/telperion-render/src/generation/drawn_memory_tests.rs:78`
- `scripts/ci-key.mjs`, `.github/workflows/tests.yml:86`
- `scripts/test-wasm.mjs`, `scripts/build-render.mjs`, `src/field/index.ts:18,53,68`

### Key context
- The ledger also holds non-tree allocations; compare tree bytes as the existing drawn-memory test does, not against all of `live()`.
- `ci-key.mjs` reads committed objects (`git rev-parse HEAD:<path>`); the test checks inputs against the dependency graph, it does not need commits.

### Acceptance
- [ ] R2 test red before, green after (or skipped without a GPU, stated).
- [ ] `scripts/ci-key.test.mjs` fails on the old suite lists and passes on the new; it runs under `npm test`.
- [ ] `scripts/test-wasm.mjs` builds the renderer; `node scripts/build-render.mjs` run alone produces the files `src/field/index.ts` imports.
- [ ] Gate and `npm test` green; codex impl-review SHIP.

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
