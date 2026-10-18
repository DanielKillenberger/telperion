---
satisfies: [R2]
---
# fn-291-the-cpu-entry-builds-every-shipped.7 R2f: the wood refused by name before allocation when it cannot fit 32-bit Wasm (host decision 2026-10-18)

## Description
R2f, the host's decision on R2's refusal budget (spec, "Host decision on R2's refusal budget (2026-10-18)"). The completion review found that Telperion seed 7 at a valid `surface.radialSegments` of 64 reaches 4 GiB of Wasm memory and throws the bare `Tree core: surface allocation`.

**Size:** M
**Files:** `crates/telperion-wasm/src/lib.rs` (the counting allocator), `crates/telperion-core/src/lib.rs` (a named error carrying bytes), `crates/telperion-core/src/pipeline/surface.rs`, `surface/rings.rs`, `surface/chunks.rs`, a new focused test file under `surface/`, `scripts/test-dist.mjs` (the refusal case), `src/browser/*.ts` only if the error text crosses the binding differently, `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/PROBE.md`
**Touches:** [crates/telperion-core/src/**, crates/telperion-wasm/src/**, scripts/test-dist.mjs, src/browser/**, .flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/**]

### Approach
- **Live bytes (Wasm only).** Wrap the global allocator in `telperion-wasm` in a counting wrapper. It adds on `alloc`/`realloc` growth and subtracts on `dealloc`/shrink. It counts live bytes, not grown memory, so freed heap is available. The core reads the count through one narrow hook. For example, the binding installs a `fn() -> usize` live-bytes source in the core once, or something equally small; pick the smallest shape that keeps the core free of `cfg(wasm32)` allocator code. No Jev, nothing in presets.
- **The need, exact, from the layout.** Right after `Layout::new` in `rings_mode` (`surface/rings.rs:165`), before any chunk is reserved, sum the bytes every buffer the surface stage then reserves for the drawn wood. That is positions, normals (oct, 4 B), coords (2 × u16), distances where asked, `u16` indices, and the per-chunk and per-run tables, from the layout's own counts (`Layout::extent`, `Piece::indices`). It is not an estimate. Add the headroom of one full chunk's largest buffer (768 KiB).
- **The check.** The available bytes are 4 GiB (the wasm32 address space) minus live bytes. If need > available, return a named error before allocating. It states the cause (the wood needs more memory than a 32-bit Wasm build has left), the bytes needed against the bytes available, and the remedy (build it through the device entry, or with fewer `radialSegments`). It needs an error variant that carries numbers, because `ResourceLimit` is `&'static str`. The binding maps it to the same "Tree core:" error class as `ResourceLimit`.
- **Native.** The same arithmetic runs against the host's limit only where one exists (for example a cgroup or `RLIMIT_AS` the process can read cheaply). Where none exists, the check is skipped. It is never a fallback path. If no cheap, reliable source exists on the native targets the workspace builds for, skip it natively and say so in the summary; do not invent one.
- **Named raw failures.** Every remaining `try_reserve` failure in the surface stage (`surface.rs::reserved`, `chunks.rs` `"surface vertices"`, others under `surface/`) surfaces as a named error. That error says what was being allocated and that memory ran out, never the bare "surface allocation".
- **Tests.**
  - A core unit test drives the check's arithmetic: a need over the available bytes is refused with the named cause and both numbers, and one under them passes.
  - `scripts/test-dist.mjs` gains one case. Telperion seed 7 with `surface.radialSegments` 64 through `dist/telperion.js` must fail with the named refusal (match the cause text and that the numbers are present). It must not reach the bare allocation error, and its `memoryBytes` stays well under 4 GiB, which proves the refusal came before the wood was allocated.
  - All 24 shipped cases still build, with high-water marks recorded.
- Heavy runs go through `telperion-heavy` only (`npm run build` at 8 GB, `test:dist` at 6 GB, gate `telperion-heavy 12 --threads 2`). The gate runs once at the end.

### Investigation targets
**Required**:
- `crates/telperion-core/src/pipeline/surface/rings.rs` (`rings_mode`, `written`)
- `crates/telperion-core/src/pipeline/surface/chunks.rs` (`Layout::new`, `Layout::extent`)
- `crates/telperion-core/src/pipeline/surface/build.rs` (what the mesh step reserves per chunk)
- `crates/telperion-wasm/src/lib.rs` (error mapping at line 53, the binding's allocations)
- `scripts/test-dist.mjs`

## Acceptance
- [ ] The Wasm build counts live heap bytes through a counting wrapper around its global allocator. The surface check reads that count, not grown memory.
- [ ] Before the wood reserves any chunk, the exact byte need from the layout plus 768 KiB of headroom is checked against 4 GiB minus live bytes. A need that does not fit is refused by name with the cause, bytes needed, bytes available and the remedy (the device entry, or fewer `radialSegments`).
- [ ] `test:dist`: Telperion seed 7 at `radialSegments` 64 is refused by that name, with `memoryBytes` well under 4 GiB. All 24 shipped cases still build, each at or under 3,584 MiB (high-water marks in PROBE.md).
- [ ] A core unit test covers the check's arithmetic both ways.
- [ ] Native builds check against the host's limit only where one exists, and skip the check otherwise. There is no fallback path.
- [ ] No surface-stage allocation failure surfaces as a bare "surface allocation". Each is named.
- [ ] Gate (`telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`), `npm test`, and `npm run build && npm run test:dist` are green, with results in the done summary.


## Done summary
R2f, the host's refusal budget. The Wasm binding counts its live heap at a counting global allocator (`telperion-wasm/src/live.rs`) and installs `4 GiB - __heap_base - live` as the core's bytes-left source (`telperion_core::memory`). Right after `Layout::new`, before any chunk is reserved, `surface::budget` sums the drawn wood's exact need from the layout: chunk buffers, lists, tables and the mesh step's scratch, including a split run's whole points and normals. It adds 768 KiB of headroom. A need over what is left is refused as `Error::WoodMemory` with both numbers and the remedy. Natively no source is installed and the check is skipped; there is no fallback. Raw surface allocation failures are named with the same remedy.

- test:dist: all 24 shipped cases build with high-water marks unchanged (highest 2,789 MiB). Telperion seed 7 at radialSegments 64 is refused by name at 2,653 MiB: "the wood needs 7411185660 bytes and 3679383462 are left".
- Gate: 1,417 passed, 0 failed, 109 ignored. npm test: 170/170. npm run build: green.

stage: impl-review - ran (codex, 3 lenses NEEDS_WORK on split-run scratch; fixed in ea3b6e0ab; re-review SHIP) (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 9a915b0e4b14b58b8bf4382355a9aab05e25e064, ea3b6e0ab62b4a5c863df18794b23a439880479b
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, npm test, npm run build && npm run test:dist
- PRs: