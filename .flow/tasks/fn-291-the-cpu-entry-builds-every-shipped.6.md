---
satisfies: [R2, R4]
---
# fn-291-the-cpu-entry-builds-every-shipped.6 R4 package with R2b: the entry returns the chunks, oct normals and 16-bit bark coords, types, README and migration note

## Description
R4 in the package, with R2b. The CPU entry returns the wood as chunks. In each chunk the normals are oct-encoded and the bark coords are 16-bit, with each field's precision bounded by a test. `TreeOutput.surface`'s TypeScript types, the README and a new migration note state the layout and how to decode it. The GPU path and the harness are unchanged.

**Size:** M
**Files:** `crates/telperion-wasm/src/lib.rs`, `crates/telperion-wasm/src/generate.rs`, `crates/telperion-core/src/pipeline/surface*.rs` (the chunk's compact fields), `crates/telperion-core/src/pipeline/surface/curve/pack.rs` (oct encoding shared, not copied), `src/browser/core.ts`, `tests/browser/bindings.mjs`, `scripts/test-dist.mjs`, `README.md`, `docs/pipeline.md`, `docs/migration-0.2-wood-chunks.md` (new; no convention exists), `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/PROBE.md`
**Touches:** [crates/telperion-wasm/**, crates/telperion-core/src/pipeline/surface/**, crates/telperion-core/src/pipeline/surface.rs, src/browser/**, tests/browser/**, scripts/test-dist.mjs, README.md, docs/**, .flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/**]

### Approach
- ABI. The metadata gains `surfaceChunks`. One accessor pair returns a chunk's field pointer and length, by chunk and by field. It follows the slot tables at `telperion-wasm/src/lib.rs:146-260`: no existing variable-count pattern exists, so add one, small. Slots 0, 1, 2 and 14 for the flat surface go away. Any read of them returns empty, which `bindings.mjs:248` already covers for unknown slots.
- TS. `surface?: { chunks: { positions: Float32Array; normals: Int16Array /* oct */; indices: Uint16Array }[]; bounds }`, copied chunk by chunk.
  - The old flat fields are absent from the type, so an old reader gets a type error.
  - If a consumer needs flat normals, a decode helper is exported, typed.
- R2b. Oct normals use `octahedral`/`unoctahedral` (`surface/curve/pack.rs:161-186`), moved to where both can read them, never copied.
  - Coords are quantised over a reference range, as `foliage/packed.rs` does.
  - One test per field bounds the decoded error: the normal's angle, and the coord against the bark texel.
  - The unit-normal asserts in `suite/surface_collapse.rs` and `suite/surface_reference.rs` are restated on decoded normals.
- Docs. `README.md:12` and `:82` change. The migration note covers the old shape, the new one, and the decode in five lines of TS.
- Re-run task 1's probe on all 24 cases and record the result in PROBE.md.

### Investigation targets
**Required**:
- `crates/telperion-wasm/src/lib.rs:140-260`, `crates/telperion-wasm/src/generate.rs:107-200`
- `src/browser/core.ts:85-165`
- `tests/browser/bindings.mjs:40-250`
- `crates/telperion-core/src/pipeline/surface/curve/pack.rs:140-190`
- `README.md:5-20,75-95`

## Acceptance
- [ ] The entry returns chunks: the metadata's chunk count plus per-chunk positions, oct normals and `u16` indices; the old flat slots are gone
- [ ] A test per compact field bounds its decoded error and fails past the bound
- [ ] `TreeOutput` types, README, `docs/pipeline.md` and a migration note state the chunked layout and its decode
- [ ] `tests/browser/bindings.mjs` and `scripts/test-dist.mjs` read the chunks; `npm test` covers them
- [ ] PROBE.md records the 24-case probe after this task


## Done summary
The CPU entry returns the wood as chunks. Metadata carries `surfaceChunks`, and `chunk_ptr`/`chunk_len` read each chunk's positions, oct-encoded normals (2×i16) and u16 indices. The core writes the normals oct-encoded and the bark coords as 2×u16 as each chunk is written, so the saving is in Wasm memory. One precision test per field: normals within 1e-4 rad, coords within half a code.

The flat slots are gone, so an old reader of `surface.positions` gets a type error. The package exports `decodeNormals` and `SurfaceChunk`. Updated: README, `docs/pipeline.md`, `bindings.mjs` and `test-dist.mjs`; new migration note `docs/migration-0.2-wood-chunks.md`.

Probe (PROBE.md "Task 6"):
- telperion: 1,640 MiB at seed 1; 2,777 MiB at seeds 7 and 42
- laurelin: 1,639 MiB at seed 1; 2,783 MiB at seed 7; 2,755 MiB at seed 42
- all under the 3,584 MiB bar

Checks: `npm test` (170 tests), `npm run test:dist`, tsc, and the core, render and wasm crate tests that read the wood.

stage: impl-review - ran [fan-out correctness/contracts/integration: SHIP; 2 P2 fixed after in 6d374b93c] (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: e0a0c70d5394546195dfbe5c67dd8e5b6d0f9412, 6d374b93c8a9e2fc97ecbe3118498a4b697d5711
- Tests: npm test, npm run test:dist, npx tsc --noEmit
- PRs: