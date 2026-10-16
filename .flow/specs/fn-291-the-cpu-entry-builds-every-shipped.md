# The CPU entry builds every shipped preset or refuses it by name

## Goal & Context

`scripts/test-dist.mjs` (the package as Node loads it) fails on F4: `TreeEngine.build(TELPERION, seed 7, { surface: true, foliage: true })` in `dist/telperion.js` throws "Tree core: resource limit: surface allocation" (reproduced at 6a6d5abcb, after fn-257, 2026-10-16). The package's main entry is the README's first example, and docs/principles.md asks that every shipped preset's every artifact builds through each package entry. The script is already a step of CI's `package` job, so that job fails on this branch until the entry builds the tree or refuses it by name (checked in the workflow at plan time; the capture's "not in CI" was stale).

The refusal comes from the wood's buffer reservation (`try_reserve_exact`, mapped to `ResourceLimit("surface allocation")`). The 32-bit Wasm binding runs with the default 4 GiB memory. The existing wasm32 ceiling caps kept phytomers (4M, predicted before growth and checked after) and does not bound the wood's ring, normal and index buffers, so a tree under the phytomer ceiling can still exhaust memory in the surface stage (fn-198 BROWSER.md records the same message for the beech at 80 and the spruce at 60 and 70). The native entry test (`every_shipped_preset_builds_through_the_main_entry`) runs on the host allocator and cannot see the limit. Unknown until R1 runs: which presets and seeds fail, at what memory high-water mark, and which allocation is the one that fails.

## Acceptance Criteria

- **R1 (probe first):** every shipped preset at its preset age, seeds 1, 7, 42, through the built `dist/` CPU entry in Node with surface and foliage: pass or the error, with the Wasm memory high-water mark (`memoryBytes` from a fresh engine per case) and the node count where it built; the cause of each failure traced to code (the allocation, its size, what sized it) and written down before any fix. Errors: a refusal by name ("no engine species", the phytomer ceiling) is recorded as that refusal, not as a failure of this spec; a Wasm trap or `RangeError` is recorded with its message, distinct from a `Tree core:` error.
- **R2 (host decides from R1):** either the entry builds them (a sizing or memory fix, measured in the same probe), or the limit is a named, documented refusal that the package's README and types state, with the owner's word if a shipped preset at its preset age is refused. Errors: a refusal names the preset's cause and its remedy, never the bare "surface allocation"; no fallback path that draws a different tree.
- **R3:** `test:dist` runs in CI's package job and passes, covering the R1 matrix or the part of it R2 settles; gate (`cargo test --profile ci --workspace --no-fail-fast`) and `npm test` green. Errors: a preset the entry neither builds nor refuses by name fails the script naming the entry, preset and seed.

## Early proof point

Task fn-291-the-cpu-entry-builds-every-shipped.1 validates the core approach (the probe table and traced cause). If it shows the choice is between a memory or sizing fix and refusing a shipped preset by name, the host decides R2 before fn-291-the-cpu-entry-builds-every-shipped.2 starts; task 2's shape follows that decision.

## Boundaries

Release item (fn-237). Found by the host on 2026-10-16 while checking fn-257's report. Out of scope: memory64 or a second binding, the device (WebGPU) entry and the field entry (fn-257 owns them), the growth path, and charting memory against age (fn-285). The field entry's Node refusal ("WebGPU is not available") stays as it is.

## Strategy Alignment

Active tracks served by this plan:
- **The core and integration** — the Wasm binding is the browser and Node face of the one core; every shipped preset building through it, or refusing by name, is the contract integrations rely on.

## Decision Context

R2 is deliberately left to the host after R1 (owner instruction in the dispatch): a sizing fix keeps every preset drawable; a refusal by name changes what the package promises and needs the owner's word for a shipped preset.

## Quick commands

```bash
CARGO_TARGET_DIR=$PWD/target telperion-heavy 8 --threads 8 -- npm run build
telperion-heavy 6 --threads 2 -- npm run test:dist
```

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
| --- | --- | --- | --- |
| R1 | (probe first): every shipped preset at its preset age, seeds 1, 7, 42, through the built `dist/` CPU entry in Node with surface and foliage: pass or the error, with the Wasm memory high-water mark (`memoryBytes` from a fresh engine per case) and the node count where it built; the cause of each failure traced to code (the allocation, its size, what sized it) and written down before any fix. Errors: a refusal by name ("no engine species", the phytomer ceiling) is recorded as that refusal, not as a failure of this spec; a Wasm trap or `RangeError` is recorded with its message, distinct from a `Tree core:` error. | fn-291-the-cpu-entry-builds-every-shipped.1 | — |
| R2 | (host decides from R1): either the entry builds them (a sizing or memory fix, measured in the same probe), or the limit is a named, documented refusal that the package's README and types state, with the owner's word if a shipped preset at its preset age is refused. Errors: a refusal names the preset's cause and its remedy, never the bare "surface allocation"; no fallback path that draws a different tree. | fn-291-the-cpu-entry-builds-every-shipped.2 | — |
| R3 | `test:dist` runs in CI's package job and passes, covering the R1 matrix or the part of it R2 settles; gate (`cargo test --profile ci --workspace --no-fail-fast`) and `npm test` green. Errors: a preset the entry neither builds nor refuses by name fails the script naming the entry, preset and seed. | fn-291-the-cpu-entry-builds-every-shipped.3 | — |
