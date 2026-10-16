# The CPU entry builds every shipped preset or refuses it by name

## Goal & Context

`scripts/test-dist.mjs` (the package as Node loads it) fails on F4: `TreeEngine.build(TELPERION, seed 7, { surface: true, foliage: true })` in `dist/telperion.js` throws "Tree core: surface allocation" (reproduced at 6a6d5abcb, after fn-257, 2026-10-16). The package's main entry is the README's first example, and docs/principles.md asks that every shipped preset's every artifact builds through each package entry. The script is already a step of CI's `package` job, so that job fails on this branch until the entry builds the tree or refuses it by name (checked in the workflow at plan time; the capture's "not in CI" was stale).

The refusal comes from the wood's buffer reservation (`try_reserve_exact`, mapped to `ResourceLimit("surface allocation")`). The 32-bit Wasm binding runs with the default 4 GiB memory. The existing wasm32 ceiling caps kept phytomers (4M, predicted before growth and checked after) and does not bound the wood's ring, normal and index buffers, so a tree under the phytomer ceiling can still exhaust memory in the surface stage (fn-198 BROWSER.md records the same message for the beech at 80 and the spruce at 60 and 70). The native entry test (`every_shipped_preset_builds_through_the_main_entry`) runs on the host allocator and cannot see the limit. Unknown until R1 runs: which presets and seeds fail, at what memory high-water mark, and which allocation is the one that fails.

## Acceptance Criteria

- **R1 (probe first):** every shipped preset at its preset age, seeds 1, 7, 42, through the built `dist/` CPU entry in Node with surface and foliage: pass or the error, with the Wasm memory high-water mark (`memoryBytes` from a fresh engine per case) and the node count where it built; the cause of each failure traced to code (the allocation, its size, what sized it) and written down before any fix. Errors: a refusal by name ("no engine species", the phytomer ceiling) is recorded as that refusal, not as a failure of this spec; a Wasm trap or `RangeError` is recorded with its message, distinct from a `Tree core:` error.
- **R2 (owner, 2026-10-16; see "The owner's decision on R2"):** every shipped preset at every seed builds through the CPU entry by a memory fix, with no refusal by name for a shipped preset: R2a frees what the surface stage does not read before it allocates and writes the wood's buffers once at final size; R2b stores the wood's per-vertex data compactly with each field's precision bounded by a test; R2c proves all 24 PROBE.md cases build at or under 3.5 GiB of Wasm memory, Telperion and Laurelin unchanged to the eye, other presets byte-identical or within the bounded precision, and the probe in `test:dist`. Errors: a family that truly cannot be built is refused before allocation with a named cause, never a bare "surface allocation"; fewer sides on thin wood is not a remedy; if R2a and R2b cannot reach the bar, the run stops and reports the measured gap.
- **R3:** `test:dist` runs in CI's package job and passes, covering the R1 matrix or the part of it R2 settles; gate (`cargo test --profile ci --workspace --no-fail-fast`) and `npm test` green. Errors: a preset the entry neither builds nor refuses by name fails the script naming the entry, preset and seed.

## Early proof point

Task fn-291-the-cpu-entry-builds-every-shipped.1 settled R1 (PROBE.md). Task fn-291-the-cpu-entry-builds-every-shipped.2 proves the memory approach: R2a measured on the four failing cases, and the floor that R2b's per-vertex compaction can reach computed from the measured buffers. If that floor plus the measured Wasm slack is over 3.5 GiB, the run stops there and reports the gap; changing the index layout or the side count is not this plan's call.

## Boundaries

Release item (fn-237). Found by the host on 2026-10-16 while checking fn-257's report. Out of scope: memory64 or a second binding, the device (WebGPU) entry and the field entry (fn-257 owns them), the growth path, and charting memory against age (fn-285). The field entry's Node refusal ("WebGPU is not available") stays as it is.

## The owner's decision on R2 (2026-10-16)

"we shouldn't refuse by name. If we have params we can't build we should refuse those cleanly, but ideally we don't have that." Every shipped preset at every seed builds through the CPU entry; a refusal is reserved for parameters that truly cannot be built, predicted before allocation and named. So R2 is a memory fix, chosen by the host from PROBE.md:

- **R2a (the build's peak):** what the surface stage does not read is freed before it allocates (the grown tree's working state, the foliage plan's scratch), and the wood's intermediate buffers are written once at their final size, not grown or copied.
- **R2b (compact wood):** the wood's per-vertex data in the smallest layout the drawing allows (for example oct-encoded normals, bark coordinates in 16-bit), each field's precision bounded by a test at the drawn tolerance; if the package's output format changes, the TypeScript types, README and a migration note change with it.
- **R2c (proof):** all 24 cases of PROBE.md build with the Wasm high-water mark at or under 3.5 GiB (headroom), stills of Telperion and Laurelin unchanged to the eye, every other preset's surface byte-identical or within the bounded precision; the probe joins `test:dist`.
- Fewer sides on thin wood (PROBE.md option 3) changes the tree's look and is not taken here; if R2a-R2b cannot reach the bar, the run stops and reports the measured gap.

## Host decision after task 2's floor (2026-10-16)

PROBE.md: growth leaves 2.6-2.7 GiB of Wasm memory in fragments, and the wood's index buffer is one 1,651 MiB run, so R2a and R2b cannot fit Telperion at seed 7. Following the owner's direction on fn-257 ("if you define a different format to write in the beginning and not convert at the end that should get the savings throughout?"; "if we can compress it further even better"):

- **R2d (wood in pieces, compact from the start):** the surface is written as chunks of at most 65,536 vertices, each with 16-bit indices, allocated chunk by chunk, so no buffer needs one large free run and the index bytes halve; the package returns the chunks (TypeScript types, README and a migration note change with it; the GPU path and the harness unchanged).
- **R2e (growth compact from the start, lossless):** the CPU growth's tables narrowed where they are created (counts, indices, flags and ids in the narrowest width their range allows, padding removed), trees byte-identical, the post-growth Wasm footprint measured per preset; the same rule as fn-257's.
- The plan changed, so the review rounds restart with this re-plan; task 2's round-2 finding (the floor after R2a) is re-reviewed in the new round.

## Strategy Alignment

Active tracks served by this plan:
- **The core and integration** — the Wasm binding is the browser and Node face of the one core; every shipped preset building through it, or refusing by name, is the contract integrations rely on.

## Decision Context

R1 showed the failing wood needs more than 32-bit Wasm holds as drawn (PROBE.md). The owner chose a memory fix over refusal (section above). Measured facts the tasks build on: the CPU entry hands JS positions, normals and indices only (bark coords stay in Wasm); the native stills draw from the curve, not from the CPU entry's mesh, so "unchanged to the eye" is judged on a still of the CPU entry's own mesh; indices (1,651 MiB for Telperion seed 7) are not per-vertex data, so compacting them is outside R2b as decided and goes back to the host if the floor needs it.

## Quick commands

```bash
CARGO_TARGET_DIR=$PWD/target telperion-heavy 8 --threads 8 -- npm run build
E=.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped
node $E/probe-dist.mjs dist $E/raw/probe-after.jsonl 1,7,42 telperion,laurelin
telperion-heavy 6 --threads 2 -- npm run test:dist
```

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
| --- | --- | --- | --- |
| R1 | (probe first): every shipped preset at its preset age, seeds 1, 7, 42, through the built `dist/` CPU entry in Node with surface and foliage: pass or the error, with the Wasm memory high-water mark (`memoryBytes` from a fresh engine per case) and the node count where it built; the cause of each failure traced to code (the allocation, its size, what sized it) and written down before any fix. Errors: a refusal by name ("no engine species", the phytomer ceiling) is recorded as that refusal, not as a failure of this spec; a Wasm trap or `RangeError` is recorded with its message, distinct from a `Tree core:` error. | fn-291-the-cpu-entry-builds-every-shipped.1 | — |
| R2 | (owner, 2026-10-16; see "The owner's decision on R2"): every shipped preset at every seed builds through the CPU entry by a memory fix, with no refusal by name for a shipped preset: R2a frees what the surface stage does not read before it allocates and writes the wood's buffers once at final size; R2b stores the wood's per-vertex data compactly with each field's precision bounded by a test; R2c proves all 24 PROBE.md cases build at or under 3.5 GiB of Wasm memory, Telperion and Laurelin unchanged to the eye, other presets byte-identical or within the bounded precision, and the probe in `test:dist`. Errors: a family that truly cannot be built is refused before allocation with a named cause, never a bare "surface allocation"; fewer sides on thin wood is not a remedy; if R2a and R2b cannot reach the bar, the run stops and reports the measured gap. | fn-291-the-cpu-entry-builds-every-shipped.2, fn-291-the-cpu-entry-builds-every-shipped.3, fn-291-the-cpu-entry-builds-every-shipped.4 | — |
| R3 | `test:dist` runs in CI's package job and passes, covering the R1 matrix or the part of it R2 settles; gate (`cargo test --profile ci --workspace --no-fail-fast`) and `npm test` green. Errors: a preset the entry neither builds nor refuses by name fails the script naming the entry, preset and seed. | fn-291-the-cpu-entry-builds-every-shipped.4 | — |
