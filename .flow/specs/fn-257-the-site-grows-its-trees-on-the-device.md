# The site grows its trees on the device

## Goal & Context

fn-245's site stage, re-scoped by the owner (2026-10-11) after COUNCIL.md's early tests 4 and 5. killenberger.com stays on 0.1.4 until the new engine meets its speed (fn-237 R0).

The coarse-record route failed on measurement: `couple::Aggregate` reaches acceptable fidelity only with nearly full expansion, and is then slower than growing everything (fn-257 SPIKE.md); what the site draws from it costs 1.7-3.4× 0.1.4 for the oak at a 0.32 m cut and 2.2-3.8× for the birch at every cut (TEST5.md). Growing the full tree on the CPU in single-thread Wasm is about ten times the budget (oak 2.3-3.7 s). The owner's answers (2026-10-11):

- "Yes, WebGPU on the site": the site grows on the device like the harness, the only route on which it draws the same tree.
- The oak "Must match 0.1.4": no slowdown on the site.
- The ordinary tree "Yes, it must ship".

0.1.4's cold end-to-end times (TEST5.md, seeds 1 / 1234 / 5678): oak 290 / 273 / 332 ms, birch 292 / 352 / 354 ms; every other species' from PROFILE.md and the site's own measurement.

## Requirements

- **R1 (one tree, one pipeline):** the site's field is computed from the tree fn-256's device executor grows at full resolution, through the one pipeline, then voxelised at the field's cell (on the device or after one readback, measured); the same seed gives the harness's tree. No stand-ins, no closed-form tables and no expected-count cut on the site path; `couple::Aggregate`, the stands, `Rows` construction for them and the predictor are deleted where no other consumer remains (named per deletion).
- **R1a (query contract):** the existing query API is kept (`src/field/index.ts`), with the field's resolution stated in world units; the `cut` argument is removed and its migration noted; point and region queries as fn-257's earlier R1a.
- **R1b (no device):** a browser or worker without WebGPU, or whose device is lost, gets a named error from the field call, never a CPU fallback; what the site shows then is the site's decision (killenberger repo), reported to the owner.
- **R2 (fidelity):** the device-grown field against the CPU reference cycle's at the same seed: wood occupancy, maximum radius, foliage density and limb ownership per cell at 24³, 36³ and 48³, within tolerances measured from real runs; viewed stills of the voxel grove beside the harness's tree and beside 0.1.4's.
- **R3 (speed, the owner's bar):** cold end-to-end per tree at or under 0.1.4's for every shipped species, the ordinary included, measured in Chrome on the owner's machine: adapter and device creation, pipeline compilation, growth, voxelisation, readback and query. First visit (empty shader cache) and repeat visit reported apart; a grove of several trees reported per tree after the first. Any species over 0.1.4 is named with its gap for the owner.
- **R4 (proved):** gate and npm test green; Astra's agreement; the owner's verdict on the grove.

## Early test (before R1 is built)

A probe page the owner opens in Chrome (workers never run WebGPU in a browser): adapter and device creation, compilation of fn-256's cycle kernels cold and cached, and one empty cycle chain, timed. Stop: device creation plus cold compilation alone exceeds 0.1.4's oak time (273 ms), so a first visit cannot match; the owner decides whether first and repeat visits are judged apart.

## Boundaries

The site's field. Depends on fn-256 (the device executor). Release item for fn-237.
