# The site grows its trees on the device

## Goal & Context

fn-245's site stage, re-scoped by the owner (2026-10-11) after COUNCIL.md's early tests 4 and 5. killenberger.com stays on 0.1.4 until the new engine meets its speed (fn-237 R0).

The coarse-record route failed on measurement: `couple::Aggregate` reaches acceptable fidelity only with nearly full expansion, and is then slower than growing everything (fn-257 SPIKE.md); what the site draws from it costs 1.7-3.4× 0.1.4 for the oak at a 0.32 m cut and 2.2-3.8× for the birch at every cut (TEST5.md). Growing the full tree on the CPU in single-thread Wasm is about ten times the budget (oak 2.3-3.7 s). The owner's answers (2026-10-11):

- "Yes, WebGPU on the site": the site grows on the device like the harness, the only route on which it draws the same tree.
- The oak "Must match 0.1.4": no slowdown on the site.
- The ordinary tree "Yes, it must ship".

0.1.4's cold end-to-end times (TEST5.md, seeds 1 / 1234 / 5678): oak 290 / 273 / 332 ms, birch 292 / 352 / 354 ms; every other species' from PROFILE.md and the site's own measurement.

## Requirements

- **R1 (one tree, one pipeline):** *(superseded by R8, 2026-10-16)* the site's field is computed from the tree fn-256's device executor grows at full resolution, through the one pipeline, then read back once as the geometric records today's field query reads; the same seed gives the harness's tree. No stand-ins, no closed-form tables and no expected-count cut on the site path; `couple::Aggregate`, the stands, `Rows` construction for them and the predictor are deleted where no other consumer remains (named per deletion).
- **R1a (query contract, unchanged):** *(superseded by R8, 2026-10-16)* the field answers queries exactly as today (`crates/telperion-core/src/pipeline/field.rs`: arbitrary cubes evaluated against the tree's geometric records, the limb with the largest accumulated contribution owning a region); only the tree it reads now comes from the device, read back once as those records. No new grid, grid origin or aggregation semantics. The `cut` argument is removed from TypeScript and Wasm with a migration note; nothing replaces it.
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

- **R1' (the field stays on the device):** *(superseded by R8, 2026-10-16)* `growField` grows on the device and keeps the field's primitives there; `query(cells)` runs on the device for any cells and reads back only the answers; `release()` frees them. The query contract (R1a's cubes, answers and limb ownership) is unchanged; `cut` goes with a migration note.
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

## Host decisions after task 8 (2026-10-16)

Reusing growth's buffers across builds (about 33 ms on the spruce) is not taken: it raises the build's peak from 1,587 MB to about 2,250 MB and holds 1.6 GB between builds, against the owner's memory direction. The compaction stays as it is (removing its read or sizing it in was no clear gain). The foliage's level lists (80.8 MB, about 15 MB laid end to end) are fn-292. With these, the spruce's remaining non-kernel costs are about 26 ms of dispatch launches (folding measured slower, DISPATCH.md) and about 33 ms of buffer creation (the reuse above); the rest is kernel arithmetic, fn-285's.

## The owner's decisions on reuse and the site field's time (2026-10-16)

- **Buffer reuse across builds, declined:** "keeping buffers for 33ms costing 663mb seems not worth it". A reuse limited in time (freed after an idle spell) is a later idea for fn-285.
- **"Why is the build faster than the site field?"** The spruce's site field is the build (about 381 ms), the field's primitives (about 56 ms) and the 36³ query (about 65 ms), about 0.50 s. Before fn-257 closes, as part of its completion pass (through the flow, codex review): (a) measure whether `growField` runs the drawing build's dress and drawn stages it does not read for the field; if so, the field path stops after what it reads (one pipeline, no second chain, only not asking for outputs it does not use); (b) measure where the query's 65 ms goes (dispatches, readbacks, index traversal against arithmetic). Each is taken with a general remedy, measured paired warm over 16 runs with the field's answers identical, and the spruce's site-field time reported before and after.

## The field is a read-out of the build, not a second format (owner, 2026-10-16)

The owner, on why the site field costs more than the build: "so what's the output of the build phase? if not the field? why couldn't we evaluate the 36^3 query directly from that product?" and "making another format just for the fun of it doesn't make sense to me if the build output isn't that field..". So R1' and task 5's compact primitives are replaced by:

The owner (2026-10-16): "we defined we wanted a clean pipeline with consumers that take the output from previous stages". So the field is a consumer of the drawn stage's output through the one pipeline (docs/pipeline.md: stages private, no second chain), as the renderer is: no intermediate stage or format stands between the drawn records and the grid read-out, and docs/pipeline.md's field section is rewritten to say so.

- **R8 (one form of the tree):** the field reads the build's drawn records directly; the field-primitive format, its index, the compact primitive layout and the per-cube query are deleted. A field request names its grid (resolution over the tree's bounds); one device pass splats every drawn segment and leaf-bearing segment into the cells it touches (wood flag, thickest radius, leaf estimate, owning limb), and only the grid is read back.
- **R8a (same answers):** every preset at 24, 36 and 48 cells, seeds 1 and 5678, within the tolerances the field holds today against the CPU reference; the CPU reference itself reads the CPU's drawn tree the same way.
- **R8b (contract):** the package's field entry takes the grid and returns it; arbitrary-cube queries go, with a migration note beside `cut`'s; the TypeScript types and README change with it.
- **The grid (host, 2026-10-16, on the plan review):** the grid the site's voxelizer already draws (`src/field/voxelize.ts` `grid`): `n` cells a side, cubes of side `cell = longest bounds axis / (n - 2)`, centred on the bounds in x and z with one empty cell below the lowest point in y; cell `i + n * (j + n * k)` at `origin + (i + 0.5, j + 0.5, k + 0.5) * cell`. The answer returns `n`, `cell`, `origin` and one answer per cell. `n` is a whole number from 3 to 256; outside that, or a grid past the device's buffer limits, is refused by name before any work. The CPU field, the device and the voxelizer use this one definition.
- **Both executors (host, 2026-10-16, on the pipeline audit, fn-293 PIPELINE-AUDIT.md finding 1):** R8 covers the CPU field too. The CPU field reads the CPU's drawn tree as the device grid reads the device's: its wood and its leaf-bearing records as the drawn stage emits them, to the same grid. The field's own forms go: `Foliage::Planned` and `Foliage::Placed`, the wood capsules rebuilt from `tree.nodes` (`Field::wood`), `FieldSnapshot`, and `planned.rs`'s route to the field (its other consumers, if any, stay). The main package's `tree.field` (`query(cells)`, `snapshot()`) takes the grid as the field package does, with a migration note. The rest of the audit is fn-293's, after fn-257.
- **R8c (measured):** the spruce's site field, warm, paired against today's, and the held memory per tree after the read-out.
