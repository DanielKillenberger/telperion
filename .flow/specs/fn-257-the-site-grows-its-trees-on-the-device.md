# The site grows its trees on the device

## Goal & Context

fn-245's site stage, re-scoped by the owner (2026-10-11) after COUNCIL.md's early tests 4 and 5. killenberger.com stays on 0.1.4 until the new engine meets its speed (fn-237 R0).

The coarse-record route failed on measurement: `couple::Aggregate` reaches acceptable fidelity only with nearly full expansion, and is then slower than growing everything (fn-257 SPIKE.md); what the site draws from it costs 1.7-3.4× 0.1.4 for the oak at a 0.32 m cut and 2.2-3.8× for the birch at every cut (TEST5.md). Growing the full tree on the CPU in single-thread Wasm is about ten times the budget (oak 2.3-3.7 s). The owner's answers (2026-10-11):

- "Yes, WebGPU on the site": the site grows on the device like the harness, the only route on which it draws the same tree.
- The oak "Must match 0.1.4": no slowdown on the site.
- The ordinary tree "Yes, it must ship".

0.1.4's cold end-to-end times (TEST5.md, seeds 1 / 1234 / 5678): oak 290 / 273 / 332 ms, birch 292 / 352 / 354 ms; every other species' from PROFILE.md and the site's own measurement.

## Requirements

- **R1 (one tree, one pipeline):** the site's field is computed from the tree fn-256's device executor grows at full resolution, through the one pipeline, then read back once as the geometric records today's field query reads; the same seed gives the harness's tree. No stand-ins, no closed-form tables and no expected-count cut on the site path; `couple::Aggregate`, the stands, `Rows` construction for them and the predictor are deleted where no other consumer remains (named per deletion).
- **R1a (query contract, unchanged):** the field answers queries exactly as today (`crates/telperion-core/src/pipeline/field.rs`: arbitrary cubes evaluated against the tree's geometric records, the limb with the largest accumulated contribution owning a region); only the tree it reads now comes from the device, read back once as those records. No new grid, grid origin or aggregation semantics. The `cut` argument is removed from TypeScript and Wasm with a migration note; nothing replaces it.
- **R1b (no device):** a browser or worker without WebGPU, or whose device is lost, gets a named error from the field call, never a CPU fallback; what the site shows then is the site's decision (killenberger repo), reported to the owner.
- **R2 (fidelity):** the device-grown field against the CPU reference cycle's at the same seed: wood occupancy, maximum radius, foliage density and limb ownership per cell at 24³, 36³ and 48³, within tolerances measured from real runs; viewed stills of the voxel grove beside the harness's tree and beside 0.1.4's.
- **R3-0 (the owner, 2026-10-11, on the probe's 501 ms of device creation and cold compile):** "i think the shader compile on first visit used to run for all trees taking 500ms is probably fine? in that time noone will have had the opportunity to see a tree. As long as generation after that is fast that seems acceptable". So device creation and pipeline compilation are a once-per-page cost, started at page load, reported apart (first and repeat visit) and not charged to a tree; R3's per-tree bar is everything after them.
- **R3 (speed, the owner's bar):** cold end-to-end per tree at or under 0.1.4's for every shipped species, the ordinary included, measured in Chrome on the owner's machine: adapter and device creation, pipeline compilation, growth, voxelisation, readback and query. First visit (empty shader cache) and repeat visit reported apart; a grove of several trees reported per tree after the first. Any species over 0.1.4 is named with its gap for the owner.
- **R3a (tests migrate, not vanish):** the Node field tests (`src/field/field.test.ts`; Vitest runs without WebGPU) assert the named no-device error and the input contract, and their `cut` assertions are removed; positive coverage moves to native Rust tests in the render crate that grow every shipped preset on the device and compare its field answers against the CPU reference cycle's at the same seed within R2's tolerances, including a forced device loss rejected by name.
- **R4 (proved):** gate and npm test green; Astra's agreement; the owner's verdict on the grove.

## Tasks

Tasks 1 and 2 are the closed spikes (early tests 4 and 5, SPIKE.md, TEST5.md, TABLES.md): prior evidence for this re-scope, not its implementation. The implementation tasks are planned after fn-256's executor lands, each with its touched paths.

## Early test (before R1 is built)

A probe page the owner opens in Chrome (workers never run WebGPU in a browser): adapter and device creation, compilation of fn-256's cycle kernels cold and cached, and one empty cycle chain, timed. Stop: device creation plus cold compilation alone exceeds 0.1.4's oak time (273 ms), so a first visit cannot match; the owner decides whether first and repeat visits are judged apart.

## Boundaries

The engine's field query only. The owner (2026-10-15): "that really shouldn't be here but on killenberger.com": the site's page, its measurement in Chrome as a page, what a visitor without WebGPU sees (R1b's "the site's decision") and the grove's verdict belong to the killenberger.com repository; here R3 is measured through the package entry the site calls. Depends on fn-256. Not a release item (fn-237, 2026-10-15): killenberger.com stays on 0.1.4 until it moves.
