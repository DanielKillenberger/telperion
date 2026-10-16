---
satisfies: [R1]
---
# fn-291-the-cpu-entry-builds-every-shipped.1 Probe every shipped preset through the dist CPU entry in Node and trace each failure

## Description
Measure before fixing (spec R1). A probe drives the built `dist/telperion.js` in Node over every catalogue preset at its preset age, seeds 1, 7, 42, with `{ surface: true, foliage: true }`, one child process per case so one case's grown memory never colours the next. Each row: preset, seed, outcome (built / named refusal / `Tree core:` error / trap or `RangeError`), node count, `engine.memoryBytes` after the build (or after the throw), wall time. For every failing row, re-run surface-only and foliage-only to separate the stages, then trace the failing allocation to code: which buffer, its requested size, and the arithmetic that sized it (`WoodExtent`, ring counts, radial segments). No fix in this task.

**Size:** M
**Files:** `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/probe-dist.mjs`, `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/PROBE.md`
**Touches:** [.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/**]

### Approach
- Build once: `CARGO_TARGET_DIR=<worktree>/target telperion-heavy 8 --threads 8 -- npm run build`; the probe builds nothing.
- Load the entry the way `scripts/test-dist.mjs:1-18` does (file URL beside `dist/`); `PRESETS`/`CATALOGUE` from the same module gives the ids.
- `engine.memoryBytes` is `memory.buffer.byteLength` (`src/browser/core.ts:144`); memory only grows, so it is the high-water mark of that process.
- Run each child under `telperion-heavy 6 --threads 2`, sequentially (one heavy run at a time, `.flow/memory` convention).
- Raw per-case output to `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/raw/`; PROBE.md holds the table, the traced cause per failure with `file:line`, and the arithmetic of the failing request against 4 GiB.

### Investigation targets
**Required**:
- `crates/telperion-core/src/pipeline/surface.rs:64-90` — `reserved`, `WoodExtent::bytes`
- `crates/telperion-core/src/pipeline/surface/build.rs:21-160` — `extent_of`, `faces` reservations on the serial (wasm) path
- `crates/telperion-core/src/pipeline/space.rs:27-240` — `WASM_KEPT`, `foreseen`, `fits`, `OVER`
- `src/browser/core.ts:120-180` — `create`, `build`, `check`, output copies
**Optional**:
- `.flow/evidence/fn-198-tree-space-f-the-engine-in-production/BROWSER.md:25-45` — earlier wasm memory measurements
- `scripts/benchmarks/mature-generation.mjs` — existing `memoryBytes` reading

### Key context
The native entry test cannot reproduce this; only the wasm32 binding in Node can. A refusal by name ("no engine species", the phytomer ceiling) is a recorded outcome, not a failure.

## Acceptance
- [ ] PROBE.md tables all 8 presets x seeds 1, 7, 42 with outcome class, nodes, memoryBytes and time, from a fresh process per case
- [ ] Each failing case is split into surface-only and foliage-only runs
- [ ] Each failure's cause is traced to the allocation and the sizing code (`file:line`) with the requested bytes against the 4 GiB limit
- [ ] PROBE.md ends with the R2 options the table leaves open, for the host; no generator or entry code changed

## Done summary
R1 probe done; no generator or entry code changed. PROBE.md tables all 8 shipped presets at their preset ages x seeds 1, 7, 42 through the built dist CPU entry in Node, one fresh process per case, with outcome class, nodes, Wasm memory and time. 4 of 24 cases fail, all `core-error: surface allocation`: telperion (82) and laurelin (80) at seeds 7 and 42. Foliage-only builds the same trees. A traced throwaway wasm build names each failing reservation: normals in faces() for three cases, coords in rings() for telperion seed 42. Each is an exact request over what the 4 GiB memory had left. The native counting-allocator trace puts the wood buffers at 3.5 to 3.8 GiB and the build's live heap at 4.2 to 4.5 GiB, so these trees as drawn exceed 32-bit Wasm. R2 is a host decision; the options are listed at the end of PROBE.md.

stage: impl-review - ran [round 1 NEEDS_WORK (3 findings, all fixed in a73fbb802) .. re-review SHIP] (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 6697f7d255e73be3334d9d8c63978268a2d0319d, a73fbb802d3077d81eff4c8aa8a97be0c326aadd
- Tests: node .flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/probe-dist.mjs dist .flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/raw/probe.jsonl, node scripts/test-dist.mjs (fails: Tree core: surface allocation, reproduced)
- PRs: