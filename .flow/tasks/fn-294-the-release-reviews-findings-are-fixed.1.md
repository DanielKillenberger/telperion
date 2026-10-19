---
satisfies: [R1, R7]
---
# fn-294-the-release-reviews-findings-are-fixed.1 Flowers cross the Wasm binding into the package output

## Description
Carry `Outputs::organ` through the C-ABI binding and `TreeEngine.build` as `TreeOutput.organ`, shaped like `TreeOutput.foliage` per the spec's Decisions. Red test first: a Laurelin build with `foliage: true` through the binding carries no organ today.

**Size:** M
**Files:** `crates/telperion-wasm/src/generate.rs`, `crates/telperion-wasm/src/lib.rs`, `crates/telperion-wasm/src/generate/tests.rs`, `src/browser/core.ts`, `tests/browser/bindings.mjs`, `README.md`, `docs/migration-0.2-flowers.md`
**Touches:** [crates/telperion-wasm/**, src/browser/core.ts, src/index.ts, tests/browser/bindings.mjs, README.md, docs/**]

### Approach
- `Output` (`generate.rs:10-19`) gains the organ's element positions, indices and instances; fill them from `o.organ` beside the leaf copy (`generate.rs:~143-160`), only under `wants("foliage")`.
- Metadata gains `organReference` (null without an organ) in the `foliageReference` form (`generate.rs:~185`).
- New slots after 25, skipping retired 26, with a comment in the style of `lib.rs:140-186` (`buffer_ptr`, `buffer_len`); the instance slot's length is `leaves.len() * foliage::WORDS` like slot 5.
- `core.ts`: `TreeOutput.organ?` and `Diagnostics.organReference`; set `result.organ` only when `outputs.foliage` and `diagnostics.organReference` is non-null (`core.ts:148` pattern).
- Docs: README package example names `tree.organ`; a short migration note in the style of `docs/migration-0.2-wood-chunks.md` (additive: what the field is, how to decode it with `leafTransform` against `organReference`).

### Investigation targets
**Required**:
- `crates/telperion-wasm/src/generate.rs` — the leaf copy to mirror
- `crates/telperion-core/src/pipeline/drawn.rs:286-310` — how the organ is produced (`request.leaves`, `tree.blooms`)
- `crates/telperion-render/src/lib.rs:266-274` — the form the renderer reads
- `crates/telperion-wasm/src/generate/tests.rs:126` — the every-shipped-preset test
- `tests/browser/bindings.mjs:119-153` — package foliage checks

**Optional**:
- `crates/telperion-core/src/suite/flowers.rs:39-60` — reading organ counts from `built.outputs.organ`

### Key context
- Laurelin (catalogue id 2) and Telperion (`/supernatural/flowering/age14 = 0.3`) both flower; confirm each yields florets at its default age and season. The natural presets (oak, birch, spruce, beech, palm, ordinary) carry no flowering row; read the flowering set from the pipeline rather than hard-coding it, so a preset that flowers is compared against its own pipeline output and one that does not is checked for absence.
- The organ's instances carry their own reference (packed against the leaves' box via `bloom_box`); hand that one, not the leaves'.
- Existing outputs stay byte-identical.

### Acceptance
- [ ] A Rust test through the binding over every shipped preset: where `pipeline::build` produces an organ (Laurelin, Telperion), the binding returns organ instance words equal to `organ.instances.leaves.len() * WORDS`, a non-empty element and a non-null `organReference` equal to the pipeline's; where it produces none, empty organ buffers and a null `organReference`; `foliage: false` returns none.
- [ ] `tests/browser/bindings.mjs` (run by `npm run test:bindings`, which `npm test` does not run; vitest excludes it) checks `tree.organ` on Laurelin (count matches `diagnostics`/the binding, owned copy survives `release`) and its absence on a non-flowering preset.
- [ ] Types, README and migration note name the organ output.
- [ ] Gate, `npm test` and `npm run test:bindings` green; codex impl-review SHIP.

## Acceptance
- [ ] TBD

## Done summary
The flowers cross the Wasm binding: `Output` carries the organ element's positions and indices and its florets (slots 27, 28, 29), the metadata carries `organReference`, and `TreeEngine.build` returns `TreeOutput.organ` beside `foliage` where foliage was asked for and the tree flowers (Telperion, Laurelin). Absent, never empty, otherwise, including a flowering window of no length. One slot lookup serves both accessors. README, docs/pipeline.md and a new docs/migration-0.2-flowers.md say so; test-dist checks the organ is well formed.

Tests: `flowers_cross_the_binding_where_the_pipeline_grows_them` (red, then green; counts equal the pipeline's for every shipped preset), `a_window_of_no_length_crosses_no_organ` (red, then green), bindings.mjs organ checks.
Gate: 1402 passed, 0 failed, 111 ignored. npm test: 173 passed. npm run test:bindings: green.

stage: impl-review - ran (codex, 3-lens panel: correctness NEEDS_WORK, contracts SHIP, integration SHIP; one fix; re-review SHIP) (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 03b1f990336889381e82000e937cc6614d634062, 750e221ded99bfe84c61680797aa8bc94ba00b94, e84616a785ea095a7bf15b1431c5347ebb338cd3
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, npm test, npm run test:bindings
- PRs: