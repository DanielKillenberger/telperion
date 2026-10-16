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

## Re-scope after the probes (host and owner, 2026-10-16)

PROBE.md: reading the drawn tree back (13-358 MB) and rebuilding the field on the CPU misses 0.1.4 on every site species. GROWTH-PROFILE.md: device growth is not bound by data movement (at most 28 ms on the spruce); the birch loses 252-266 ms to 23 mid-build buffer resizes with the GPU idle, every tree 60-74 ms to 12-15 thousand dispatches under 20 µs, and the drawn records are 100 % full at 152 B a segment. The owner's decisions: the spruce at about 0.35 s is acceptable; compression ships in this release; the kernels' own speed is after it. So R1 and R1a are replaced by:

- **R1' (the field stays on the device):** `growField` grows on the device and keeps the field's primitives there; `query(cells)` runs on the device for any cells and reads back only the answers; `release()` frees them. The query contract (R1a's cubes, answers and limb ownership) is unchanged; `cut` goes with a migration note.
- **R5 (sized ahead):** the device's working buffers are sized from the family's parameters before growth (fn-270 R4's parameter-derived estimates, calibrated on the presets, no species branch), so a build never resizes mid-growth; a build that would outgrow its estimate is still caught by name, never silently.
- **R6 (fewer dispatches):** passes of tiny dispatches fused or batched, the saving measured.
- **R7 (compact records):** the drawn node records (152 B a segment, 48 B an axis) in a compact layout: positions quantized within the tree's bounds, the start shared with the parent's end, radii and orientation quantized, each field's precision bounded by a test at the drawn tolerance; read by the renderer, the field and every consumer of the records; memory before and after per preset; stills unchanged to the eye.
- **R3' (speed):** per tree, warm, through the package entry: oak and birch at or under 0.1.4; spruce about 0.35 s or better (owner); the harness's build times not worse.

## Boundaries

The engine's field query only. The owner (2026-10-15): "that really shouldn't be here but on killenberger.com": the site's page, its measurement in Chrome as a page, what a visitor without WebGPU sees (R1b's "the site's decision") and the grove's verdict belong to the killenberger.com repository; here R3 is measured through the package entry the site calls. Depends on fn-256. Release item (owner, 2026-10-15: "no let's do a proper 0.2.0 release with this shipped"): 0.2.0's `telperion/field` grows on the device; killenberger.com moves to it in its own repository.

## The owner's decisions on tasks 4 and 5 and the spruce (2026-10-16)

- **R7, compression:** "if we can compress it further even better". The drawn records are not any build's peak (LAYOUT.md), so R7 is re-scoped to the data a consumer holds: the field's resident primitives (the spruce's 109 MB, about 60 MB estimated) in a compact layout, each field's precision bounded by a test against the CPU reference's answers at 24, 36 and 48 cells. The growth tables, the actual peak, are measured for the same treatment and, if their compaction changes no kernel arithmetic, taken here; otherwise reported for fn-285.
- **R6, fewer dispatches:** "go for it if we're sure that it does the saving". A probe first: fold the narrow hierarchy levels on one preset and measure warm build time paired against the base in a quiet window; kept only if the saving is measured (DISPATCH.md's 22-27 ms per tree is the ceiling), otherwise dropped with the measurement.
- **The spruce at about 0.51 s:** accepted "only if that is the clear best performance we can do without improving base calculation speed". After R6 and R7, the spruce's warm build is reprofiled; every remaining cost that is not kernel arithmetic (waits, resizes, setup, copies, dispatch overhead) is listed with its share, and each one with a general remedy is taken. What remains is kernel arithmetic (fn-285), shown by the profile.

## Compact from the start (owner, 2026-10-16)

"if you define a different format to write in the beginning and not convert at the end that should get the savings throughout?" So R7 is: every table is laid out compact where it is created, never converted at the end. During growth only lossless narrowing (counts, indices, flags and ids in the narrowest width their range allows, packed bit-fields, padding removed), so the trees stay byte-identical, each preset's peak and warm time measured before and after; lossy layouts only for data growth never reads back (the drawn records, the field's resident primitives, wood normals and bark coordinates), their precision bounded by tests. What quantising growth's own floats would save, and its risk to the approved trees, is measured and reported for fn-285, not taken.

## Decision Context

- R5's "a build never resizes mid-growth" is held for every shipped preset (task 3: no preset moves, SIZED.md). Moves stay the general mechanism for parameters off the presets (walks between species, the harness's dials), growing tables by observed counts, and a count past a table is still refused by name (`build_size::refusal`). Refusing every build whose estimate falls short would refuse trees the owner wants built (fn-291: every shipped preset and walk builds, no refusal by name). Host, 2026-10-16, on the plan review's finding.
