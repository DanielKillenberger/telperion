---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-134-the-bark-rings-are-a-plan-artifact.1 Implement The bark rings are a plan artifact

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The wood is now two steps. The ring step (`surface/rings.rs`) sweeps positions, coords, contact edges and the run table. The mesh step (`surface::faces`) adds indices, normals and bounds around those rings, and `Rings::into_mesh` moves the buffers into the wood without copying. The pipeline has one flow for every family: the rings sit in a `OnceLock` that the wood and seated leaves share, and the wood mesh runs beside the leaves. The seated fork is gone, along with its tests.

- **R1:** `every_family_builds_the_same_bytes_under_either_schedule` covers the 6 catalogue and 2 in-work families. It asserts they run side by side on native.
- **R2:** output bytes match base in 16 of 16 `mesh::build` builds and 240 of 240 binding builds.
- **R3:** Wasm peak memory equals base in all 240 builds.
- **R4:** every whole-build median is within noise of base (−2.0% to +0.6%, in two batches). The spruce is 0.9 to 2.0% faster than fn-102's merged base, where fn-102 had accepted +4.6%.
- **R6:** the parallel wood no longer zero-fills a buffer before writing it (`parallel/unfilled.rs`). This brings in the crate's first `unsafe`: one `set_len`, taken only after the parts are checked to tile the buffer and be written in full. The wood build is 10 to 19 ms faster for the oak, the spruce and the birch.
- **R5:** production lines are net +341, tests net +30.
- **Limits inventory:** `docs/generation-limits-inventory.json` was re-keyed because limit sites moved between files.

The full numbers are in `.flow/evidence/fn-134-the-bark-rings-are-a-plan-artifact/RESULTS.md`, and one friction entry is in `FRICTION.md` there.

Tier: session (actual model: claude-opus-5-5)

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 7d51f3abb686a600d7ddedb5487fd9ed377268cd
- Tests: baseline: none (spec defines no Quick commands), cargo test --profile ci --workspace --no-fail-fast (932 passed, 0 failed, 21 ignored), cargo test --profile ci -p telperion-core --lib -- pipeline:: surface::, npm run rust:test:wasm, meshhash base vs candidate: 16/16 identical, binding.mjs 8 families x 2 seeds x 15 combos: 240/240 identical bytes, 240/240 equal Wasm peak memory
- PRs: