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
- Laurelin (catalogue id 2) is the shipped flowering preset; confirm it flowers at its default age and season before relying on it. Every other shipped preset flowers nowhere.
- The organ's instances carry their own reference (packed against the leaves' box via `bloom_box`); hand that one, not the leaves'.
- Existing outputs stay byte-identical.

### Acceptance
- [ ] A Rust test through the binding: Laurelin with `foliage: true` returns organ instance words equal to `pipeline::build`'s `organ.instances.leaves.len() * WORDS`, a non-empty element and a non-null `organReference`; every non-flowering shipped preset returns empty organ buffers and a null `organReference`; `foliage: false` returns none.
- [ ] `tests/browser/bindings.mjs` (run by `npm test`) checks `tree.organ` on Laurelin (count matches `diagnostics`/the binding, owned copy survives `release`) and its absence on a non-flowering preset.
- [ ] Types, README and migration note name the organ output.
- [ ] Gate and `npm test` green; codex impl-review SHIP.

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
