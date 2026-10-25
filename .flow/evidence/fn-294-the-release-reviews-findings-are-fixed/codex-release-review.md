You are reviewing the Telperion 0.2.0 release before it merges to master. Find real defects. Do not fix anything and do not edit files; this is a read-only review.

## What to review

- Repository: /home/daniel/Projects/telperion/.worktrees/f4, branch `fn-198-tree-space-f-the-engine-in-production` (PR #159).
- Diff: `git diff 428009eb8..HEAD -- . ':!.flow' ':!vendor' ':!Cargo.lock' ':!package-lock.json'`. That is 836 files, about +112k / −15k lines.
- Ignore `.flow/` (plans and evidence) and `vendor/wgpu-30.0.1-async`, which is wgpu 30.0.1 plus one browser-only method, `create_compute_pipeline_async`, explained in `vendor/README.md`.
- Read `AGENTS.md`, `STRATEGY.md` ("Our approach"), `docs/principles.md` and `docs/pipeline.md` first. They state the rules the code must keep.

## What changed in this release

0.1.4 drew a mature tree directly, mostly on the CPU. 0.2.0 replaces it with a tree-space engine that grows every tree year by year on the GPU (WGSL compute in `crates/telperion-render/src/generation`).

The pipeline is grow → settle → dress → drawn records (152 B a segment, 48 B an axis) → consumers. The consumers are the renderer, and the field (`growField(n)`, which splats the drawn records into an n³ grid and reads back only the grid).

`crates/telperion-space` holds the tree space and a CPU reference of the growth cycle. `crates/telperion-core` holds the pipeline, the presets (value tables, never species branches) and the CPU executor. `crates/telperion-wasm`, `src/browser` and `src/field` hold the Wasm and the browser package.

The release was built as about 40 specs. Each task was reviewed on its own diff, so **nobody has reviewed the release as a whole**. Problems that cross specs are what this review exists for.

## Work through these areas one at a time

For each area, read the diff and the code around it.

1. `crates/telperion-render/src/generation/cycle/**`: the GPU growth cycle (grow, settle, dress, present, compaction, sizing in `given.rs`).
2. `crates/telperion-render/src/generation/resident/**`: the sweep, stations, rings, boots and field on the device; the drawn-record buffer and the renderer's packed curve.
3. The rest of `crates/telperion-render/src/**`: scene, shaders, selection, the generator API, the program cache (`Program::make_many`).
4. `crates/telperion-core/src/pipeline/**`: the stages, drawn records, surface/curve, foliage, the CPU executor, error types.
5. The rest of `crates/telperion-core`: presets, catalogue, supernatural, validation (`Family::validate`), the suite.
6. `crates/telperion-space/**`.
7. `crates/telperion-wasm`, `src/**`, `scripts/**`, `.github/**`, `harness/**`, `docs/**`, plus the root `Cargo.toml` and `package.json`.
8. **Cross-cutting, the most important.** Check these across areas:
   - every layout shared between WGSL, Rust and TypeScript agrees, field by field: strides, offsets, packing, signs, and the units of the drawn records, the packed curve and the field grid;
   - every GPU buffer's size matches what is written into it, with no overflow at large trees, so the 2 GiB device limit and the 20M-phytomer budget refuse by name instead of corrupting memory;
   - the CPU and GPU executors agree where they are meant to;
   - every error path refuses by name, with no silent fallback (a principle);
   - no second copy of a pipeline stage exists outside the one pipeline (a principle);
   - resources are released (the field read-out should leave 0 bytes held on the device);
   - the package's public API in `src/index.ts` and `src/field` matches its docs and its migration notes;
   - no code is unreachable from production but still compiled into it.

## What counts as a finding

A finding is a concrete defect with a scenario: these inputs or this state lead to this wrong output, crash, leak or corruption. Severity is P0 (wrong trees, crash, memory corruption, security), P1 (wrong under a reachable condition) or P2 (latent, or a maintainability risk with a concrete cost).

**Not** findings:
- style or naming;
- "could be cleaner";
- missing tests, unless the gap hides a defect you can name;
- anything the specs record as accepted. Search `.flow/specs` before reporting something that looks deliberate. Examples: speed against 0.1.4, the CPU/GPU leaf difference (fn-289), and refusals at age 100.

Before reporting a finding, try to refute it. Read the callers, the tests and the code on the other side of the boundary. Keep only what survives. Mark each finding CONFIRMED (you traced it end to end) or PLAUSIBLE (strong evidence, not fully traced).

## Output

Write one markdown table per area, then a short ranked list of the five findings you would fix first. Each row has these columns:

| Column | Content |
|---|---|
| id | the area number and finding number, e.g. 8.3 |
| severity | P0, P1 or P2 |
| verdict | CONFIRMED or PLAUSIBLE |
| file:line | where the defect is |
| defect | one sentence |
| failure scenario | the inputs or state, and what goes wrong |
| suggested fix | one line |

Say "none found" for an area with nothing. At the end, list what you could not check and why.
