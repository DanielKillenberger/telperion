I’ll review the release diff and trace the field API through the killenberger.com consumer, using source inspection and lightweight checks only. I’ll separate bounded release fixes from architectural follow-ups and distinguish existing performance evidence from new observations.
The consumer uses one worker and the prerelease field API. The release now grows fields on WebGPU, so the older CPU timings don’t establish its current browser performance. I’m tracing the field path for unnecessary preparation and readback, then checking the public entry points for integration defects.
Two concrete sources of extra field work are visible: field startup compiles wood-rendering programs it never dispatches, and flowering fields read every floret back to the CPU before counting them into the grid. I haven’t measured their browser cost. The consumer also records hangs with live legends, but its evidence doesn’t isolate the cause.
I confirmed a CI integration gap with a lightweight check: changing either parameter reference leaves the core suite’s receipt key unchanged, although new core tests read both files. I also traced a renderer path that builds a CPU wood surface and then uploads only the curve. I’m treating these as P2 findings; the evidence doesn’t establish a P0/P1 defect or the cause of the browser hangs.
No confirmed introduced P0/P1 findings. I found four P2 issues: two bounded release fixes and two larger simplifications suitable for after release.

This was read-only. I ran no builds, captures or full suites, and made no new browser timing measurements.

## Bounded pre-0.2.0 fixes

1. **Severity:** P2  
   **Confidence:** 100  
   **Classification:** introduced  
   **File:Line:** `crates/telperion-render/src/generation/resident.rs:51`  
   **R-IDs:** []  
   **Problem:** A worker calling `prepareField()` or its first `growField()` compiles wood-rendering programs it never uses. `WebField` calls `Generator::prepare_programs`, which explicitly prepares `curve::passes`: eleven pipelines in `curve.rs:36`. `Tools::new` also constructs the sweeper and ringer, although the field path deliberately bypasses the wood sweep (`resident/field/make.rs:75`). The first field waits for these unused programs to finish compiling. This is confirmed unnecessary work; its browser cost remains unmeasured.  
   **Suggestion:** Remove eager curve, sweep and ring preparation from the field path. Keep their preparation with the consumers that dispatch them.

2. **Severity:** P2  
   **Confidence:** 100  
   **Classification:** introduced  
   **File:Line:** `scripts/ci-key.mjs:38`  
   **R-IDs:** []  
   **Problem:** The new core reference tests read `docs/parameters.md` and `src/browser/parameters.generated.ts` (`tests/parameter_reference.rs:25,30`), but neither file participates in the core receipt key. After a successful core run, changing generated browser metadata can reuse that receipt and skip the test intended to reject the change. A parameters-document-only change also starts no workflow because of the path exclusions. A lightweight key check confirmed that changing either input leaves the core key unchanged.  
   **Suggestion:** Include both files in the core receipt inputs and allow `docs/parameters.md` through both workflow path filters.

## Post-release simplifications

3. **Severity:** P2  
   **Confidence:** 100  
   **Classification:** introduced  
   **File:Line:** `crates/telperion-core/src/pipeline.rs:78`  
   **R-IDs:** []  
   **Problem:** The synchronous renderer now builds two wood representations. `WebRenderer::set_tree` calls `mesh::build`, whose request produces both the indexed CPU surface and the curve. The changed renderer uploads only `mesh.curve` (`telperion-render/src/lib.rs:246`); the CPU surface supplies counts and bounds. Consequently, a normal `renderer.setTree(...)` still tessellates and allocates a complete surface that it never draws. This does not affect the site’s `growField` path.  
   **Suggestion:** Remove indexed-surface generation from the renderer’s CPU preparation path, preserving it for callers requesting CPU surface output.

4. **Severity:** P2  
   **Confidence:** 100  
   **Classification:** introduced  
   **File:Line:** `crates/telperion-render/src/generation/resident/florets.rs:145`  
   **R-IDs:** []  
   **Problem:** Flowering fields read every packed floret back, copy them into `Instances`, then decode them on the CPU for bounds and grid counting (`resident/field/make.rs:178,229`). Thus `growField(LAURELIN, 24)` transfers work proportional to every floret although its consumer needs only grid counts. Using the existing fn-316 seed-1 count, this is approximately 7.9 MB of floret data before the grid result. That is arithmetic from recorded counts, not a new measurement. The existing native timing evidence reports only about 1% additional total time, so this does not establish the browser regression’s cause.  
   **Suggestion:** Remove the full floret snapshot and CPU recounting path; retain the flower reduction on the device and read back its grid result.

## Performance evidence and limits

The current killenberger.com consumer uses one worker, passes `Family` parameters, and handles the new error codes. I found no stale field-query caller in that path.

The older fn-249 CPU figures describe an implementation since replaced by WebGPU. Fn-257’s timing table is explicitly native. The consumer’s fn-46.4 records both successful isolated growth and live-page hangs on prerelease `af1d320`; it does not isolate their cause. None establishes a paired browser comparison of **b2261e472 versus v0.1.4**. Attributing those hangs to either finding above would exceed the evidence.

## Requirements coverage

Integration epic **fn-198**, applying fn-237’s recorded amendments and deferrals:

| R-ID | Status | Evidence |
|---|---|---|
| R0 | met | Production skeleton uses the engine; remaining scaffold retirement is explicitly deferred. |
| R1 | met | Engine integration in `pipeline/space`; CPU leaf output no longer runs the old cull. |
| R2 | deferred | Original relative performance bar superseded by recorded release budgets and exceptions; broader optimisation follows after release. |
| R3 | met | Existing fn-314 and fn-317 evidence records successful checks; not rerun during this audit. |
| R4 | met | Species sheets and owner decisions are recorded; merge authority remains with the host. |
| R5 | met | fn-125 and pipeline documentation describe the replacement engine contract. |

Unaddressed R-IDs: []

Suppressed findings: 1 at confidence 50 — causal attribution of the consumer’s browser hangs.

Classification counts: 4 introduced, 0 pre_existing.

```json
{"suppressed_count":{"50":1},"classification_counts":{"introduced":4,"pre_existing":0},"unaddressed":[]}
```

<verdict>SHIP</verdict>